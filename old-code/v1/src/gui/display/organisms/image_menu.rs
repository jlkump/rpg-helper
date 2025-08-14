use std::{borrow::Cow, cell::RefCell, rc::Rc};

use gloo::file::File;
use validator::{ValidationError, ValidationErrors};
use web_sys::{DragEvent, Event, FileList, HtmlElement, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};
use stylist::yew::styled_component;
use yewdux::use_store;

use crate::{api::user_api::api_user_upload, gui::{contexts::theme::use_theme, display::atoms::{form_input::{FileFormInput, FormInput}, loading::SkeletonPane, popup::Popup}}, model::types::{ImageData, UploadImageData}, store::AuthUser};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub z_index: i32,
    pub active: UseStateHandle<bool>,
    #[prop_or_default]
    pub on_close_callback: Callback<()>,
    pub on_image_selected: Callback<Option<ImageData>>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}


#[styled_component(ImageMenu)]
pub fn image_menu(
    Props { 
        z_index, 
        active, 
        on_close_callback, 
        on_image_selected, 
        class, 
        style 
    }: &Props
) -> Html {

    let default_images: Vec<String> = vec![
        ""
    ].into_iter().map(|s| { format!("/img/defaults/{}", s) }).collect();

    let menu_style = css!(
        r#"
            width: 60vw;
            height: 80vh;

            .container {
                margin: 10px;
                display: flex;
                justify-content: space-evenly;
                max-height: 90%;
                overflow-y: scroll;
            }

            .img-display {
                padding: 10px;
                display: flex;
                justify-content: flex-start;
                flex-wrap: wrap;
                align-items: center;

                overflow-y: scroll;
                min-height: 128px;
                max-height: 25vh;
            }

            .right {
                display: flex;
                flex-direction: column;
                align-items: center;
                max-height: 68vh;
                min-width: 260px;
                overflow-y: scroll;
            }

            @media screen and (max-width: 800px) {
                width: 100vw;
                height: 90vh;
                .container {
                    flex-direction: column-reverse;
                }   
                .img-display {
                    flex-direction: column;
                }

                .right {
                    overflow-y: unset;
                    max-height: unset;
                }
            }
        "#
    );

    let selected_image = use_state(|| None);
    let image_selected_callback = {
        let selected_image = selected_image.clone();
        Callback::from(move |s: ImageData| {
            selected_image.set(Some(s));
        })
    };
    let image_submit_selected_callback = {
        let selected_image = selected_image.clone();
        let active = active.clone();
        let on_image_selected = on_image_selected.clone();
        Callback::from(move |_| {
            on_image_selected.emit((*selected_image).clone());
            selected_image.set(None); // For reseting value
            active.set(false);
        })
    };


    let url_selected_onsubmit = {
        let active = active.clone();
        let on_image_selected = on_image_selected.clone();
        let selected_image = selected_image.clone();
        Callback::from(move |data: ImageData| {
            on_image_selected.emit(Some(data));
            selected_image.set(None); // For reseting value
            active.set(false);
        })
    };

    let temp_img_data = ImageData::InternalUpload(UploadImageData { src: "/img/test/Antonio Tremis - AI Portrait.png".to_string(), name: "Antonio Tremis".to_string(), size: 1242, dimen: (124, 124) });
    
    html! {
        <Popup class={menu_style} style={style.clone()} {z_index} active={active.clone()} {on_close_callback}>
            <div class="header">
                <h3 style="text-align: center;">{"Image Select"}</h3>
                <hr/>
            </div>
            <div class="container"
                ondrop={Callback::from(|event: DragEvent| { 
                    event.prevent_default(); 
                    if let Some(files) = event.data_transfer() {
                        let file = files.files().into_iter().nth(0);
                        // TODO: allow drag drop?
                    }
                })}
                ondragover={Callback::from(|event: DragEvent| { event.prevent_default(); })}
                ondragenter={Callback::from(|event: DragEvent| { event.prevent_default(); })}
            >
                <div style="flex: 70%;">
                    <h5>{"My Uploads"}<hr/></h5>
                    <div class="img-display">
                        <ImagePanel data={temp_img_data.clone()} onclick={image_selected_callback.clone()}/>
                        <ImagePanel data={temp_img_data.clone()} onclick={image_selected_callback.clone()}/>
                        <ImagePanel data={temp_img_data.clone()} onclick={image_selected_callback.clone()}/>
                        <ImagePanel data={temp_img_data.clone()} onclick={image_selected_callback.clone()}/>
                        <ImagePanel data={temp_img_data.clone()} onclick={image_selected_callback.clone()}/>
                        <ImagePanel data={temp_img_data.clone()} onclick={image_selected_callback.clone()}/>
                        <ImagePanel data={temp_img_data.clone()} onclick={image_selected_callback.clone()}/>
                    </div>
                    <h5>{"Default"}<hr/></h5>
                    <div class="img-display">
                        <ImagePanel />
                        <ImagePanel />
                        <ImagePanel />
                        <ImagePanel />
                        <ImagePanel />
                        <ImagePanel />
                        <ImagePanel />
                    </div>
                </div>
                <div class="right" style="flex: 30%;">
                    <h5 style="text-align: center;">{"Selected"}</h5>
                    <hr style="width: 90%;" />
                    <div style="display: flex; flex-direction: column; align-items: center;">
                        // TODO: Allow re-naming images
                        <DetailedImagePanel data={(*selected_image).clone()} />
                        <button onclick={image_submit_selected_callback} >{"Use"}</button>
                    </div>

                    <h5 style="text-align: center;">{"External Link"}</h5>
                    <hr style="width: 90%;" />
                    <ImageUrlInput onsubmit={url_selected_onsubmit}/ >

                    <h5 style="text-align: center;">{"Upload"}</h5>
                    <hr style="width: 90%;" />
                    <ImageUpload onsubmit={Callback::from(move |_| {})} />
                </div>
            </div>
        </Popup>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct ImageUploadProps {
    onsubmit: Callback<ImageData> // Add to list of uploaded files
}

#[styled_component(ImageUpload)]
fn image_upload(props: &ImageUploadProps) -> Html {
    let file_input_ref = NodeRef::default();
    let uploaded_file = use_state(|| None);
    let upload_file_name = use_state(|| None);
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::default())));

    let (store, _) = use_store::<AuthUser>();
    

    let onupload = {
        let uploaded_file = uploaded_file.clone();
        Callback::from(move |files: FileList| {
            if let Some(file) = files.get(0) {
                uploaded_file.set(Some(file));
            }
        })
    };
    let onchange = {
        let upload_file_name = upload_file_name.clone();
        Callback::from(move |s: String| {
            upload_file_name.set(Some(s));
        })
    };
    let upload_submit = {
        let uploaded_file = uploaded_file.clone();
        let upload_file_name = upload_file_name.clone();
        let vald_errors = validation_errors.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let uploaded_file = uploaded_file.clone();
            let upload_file_name = upload_file_name.clone();
            let mut vald_errors = vald_errors.clone();
            let store = store.clone();

            spawn_local(async move {
                if let Some(name) = &*upload_file_name {
                    remove_vald_error(&mut vald_errors, "file-name", "file-name-missing");
                    if let Some(file) = &*uploaded_file {
                        remove_vald_error(&mut vald_errors, "file-upload", "file-missing");
                        if let Some(auth_token) = &store.auth_token {
                            let meta_data = name.clone();
                            let res = api_user_upload(meta_data, file, auth_token).await;
                            match res {
                                Ok(_) => {
                                    remove_vald_error(&mut vald_errors, "file-upload", "server-err");
                                },
                                Err(e) => {
                                    insert_vald_error(&mut vald_errors, "file-upload", "server-err", e.to_string());
                                },
                            }
                        } else {
                            insert_vald_error(&mut vald_errors, "file-upload", "file-missing", "File upload is missing".to_string());
                        }
                        }
                } else {
                    insert_vald_error(&mut vald_errors, "file-name", "file-name-missing", "File name is missing".to_string());
                }
            })
        })
    };

    html! {
        <form style="display: flex; flex-direction: column; align-items: center; width: 90%;" onsubmit={upload_submit}>
            <FormInput<String> 
                input_type="text" placeholder="File Name" name="file-name" input_ref={NodeRef::default()} 
                to_type={Callback::from(|s| { s })}
                {onchange} 
                onblur={Callback::from(|_| {})} 
                errors={&*validation_errors} 
            />
            <FileFormInput 
                name={"file-upload"}
                input_ref={file_input_ref.clone()}
                oninput={onupload}
                errors={&*validation_errors} 
            />
            <button type="submit">{"Upload"}</button>
        </form>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct ImageUrlInputProps {
    onsubmit: Callback<ImageData>
}

#[styled_component(ImageUrlInput)]
fn image_url_input(props: &ImageUrlInputProps) -> Html {
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::default())));
    let input_ref = NodeRef::default();
    let url = use_state(|| "".to_string());
    let onchange = {
        let url = url.clone();
        Callback::from(move |s: String| {
            url.set(s);
        })
    };
    let onsubmit = {
        let handler = props.onsubmit.clone();
        let url = url.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if !(*url).eq("") {
                handler.emit(ImageData::ExternalPath((*url).clone()))
            }
            // TODO: Validations
        })
    };
    html! {
        <form style="display: flex; flex-direction: column; align-items: center; width: 90%;" {onsubmit}>
            <FormInput<String> 
                input_type="text" placeholder="https://image/link.com" name="image-src" {input_ref} 
                to_type={Callback::from(|s| { s })}
                {onchange}
                onblur={Callback::from(|_| {})} 
                errors={&*validation_errors} 
            />
            <button type="submit">{"Use"}</button>
        </form>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct ImagePanelProps {
    #[prop_or_default]
    data: Option<ImageData>,
    #[prop_or_default]
    onclick: Callback<ImageData>,
}

#[styled_component(ImagePanel)]
fn image_panel(props: &ImagePanelProps) -> Html {
    let theme = use_theme();
    let style = css!(
        r#"
            width: 128px; 
            margin: 5px;
            align-self: center;
            text-align: center;
            text-wrap: wrap;
            position: relative;

            p {
                margin-block-start: 0em;
                margin-block-end: 0.5em;
            }

            .text-header {
                position: absolute;
                bottom: 1px;
                width: 100%;
                background: ${bg};
            }

            .wrapper {
                border: 3px solid transparent;
                cursor: pointer;
                width: 128px; 
                height: 128px;
            }

            .wrapper:hover {
                border: 3px solid ${highlight};
            }

            .wrapper:focus {
                border: 3px solid ${focus_highlight};
            }

            img {
                width: 100%;
                height: 100%;
                object-fit: cover;
            }

        "#, 
        bg=theme.paper_dark,
        focus_highlight=theme.text_colored,
        highlight=theme.text_colored_highlight
    );
    let onclick = {
        let click_handler = props.onclick.clone();
        let props = props.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(data) = props.data.clone() {
                click_handler.emit(data);
            }
        })
    };
    html! {
        if let Some(data) = props.data.clone() {
            if let Some(data) = data.as_upload_data() {
                <div class={style}>
                    <div class="wrapper" {onclick}>
                        <div class="text-header">
                            <p>{data.name.clone()}</p>
                        </div>
                        <img src={data.src.clone()} />
                    </div>
                </div>
            }
        } else {
            <p>{""}</p>
            <SkeletonPane style="width: 128px; height: 128px; margin: 5px;"/>
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
struct DetailedImagePanelProps {
    #[prop_or_default]
    data: Option<ImageData>,
}

#[styled_component(DetailedImagePanel)]
fn detailed_image_panel(props: &DetailedImagePanelProps) -> Html {
    let style = css!(
        r#"
            margin: 5px;
            align-self: center;
            width: 100%;
            height: 100%;
            text-align: center;

            p {
                margin-block-start: 0em;
                margin-block-end: 0em;
            }
            
            .wrapper {
                border: 3px solid transparent;
            }

            img {
                width: 100%;
                height: 100%;
                object-fit: contain;
            }
        "#
    );
    html! {
        if let Some(data) = props.data.clone() {
            if let Some(data) = data.as_upload_data() {
                <div class={style}>
                    <div class="wrapper">
                        <img src={data.src.clone()} />
                    </div>
                    <h6>{data.name.clone()}</h6>
                    <p><em>{format!("{}px, {}px", data.dimen.0, data.dimen.1)}</em></p>
                </div>
            }
        } else {
            <SkeletonPane style="min-width: 240px; min-height: 240px; margin: 5px;" />
        }
    }
}

fn insert_vald_error(vald_errors: &mut UseStateHandle<Rc<RefCell<ValidationErrors>>>, target: &'static str, code: &'static str, message: String) {
    let err = ValidationError::new(code).with_message(Cow::from(message));
    vald_errors
        .borrow_mut()
        .errors_mut()
        .insert(target, validator::ValidationErrorsKind::Field(vec![err]));
}

fn remove_vald_error(vald_errors: &mut UseStateHandle<Rc<RefCell<ValidationErrors>>>, target: &'static str, code: &'static str) {
    let prev = vald_errors
    .borrow_mut()
    .errors_mut()
    .remove(target);
    if let Some(prev) = prev {
        let new_errs: Vec<ValidationError>;
        match prev {
            validator::ValidationErrorsKind::Struct(_) => new_errs = vec![],
            validator::ValidationErrorsKind::List(_) => new_errs = vec![],
            validator::ValidationErrorsKind::Field(errs) => {
                new_errs = errs.into_iter().filter(|f| !f.code.eq(code)).collect();
            },
        }
        vald_errors
            .borrow_mut()
            .errors_mut()
            .insert(target, validator::ValidationErrorsKind::Field(new_errs));
    }
}

fn get_file_vald_errors(vald_errors: &Rc<RefCell<ValidationErrors>>) -> Vec<String> {
    let t = vald_errors.borrow();
    let errors = t.field_errors().clone();
    let empty_errors = vec![];
    let error = match errors.get("file") {
        Some(error) => error,
        None => &empty_errors,
    };
    error.into_iter().map(|m| match &m.message {
        Some(message) => message.to_string(),
        None => "".to_string(),
    }).collect()
}