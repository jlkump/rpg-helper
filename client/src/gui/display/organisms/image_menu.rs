use std::{cell::RefCell, rc::Rc};

use gloo::file::File;
use validator::ValidationErrors;
use web_sys::{DragEvent, Event, FileList, HtmlInputElement};
use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::{contexts::theme::use_theme, display::atoms::{popup::Popup, form_input::FormInput, loading::SkeletonPane}};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub z_index: i32,
    pub active: UseStateHandle<bool>,
    #[prop_or_default]
    pub on_close_callback: Callback<()>,
    pub on_image_selected: Callback<String>, // Image is a src path
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[derive(PartialEq, Clone)]
enum ImageSrcUpdate {
    ExternalUrl(String),
    UpdateName(String),
    UploadFile(File),
}

fn handle_image_src_update() -> Callback<ImageSrcUpdate> {
    Callback::from(move |update: ImageSrcUpdate| {
        match update {
            ImageSrcUpdate::ExternalUrl(_) => todo!(),
            ImageSrcUpdate::UpdateName(_) => todo!(),
            ImageSrcUpdate::UploadFile(_) => todo!(),
        }
    })
}

#[styled_component(ImageMenu)]
pub fn image_menu(Props { z_index, active, on_close_callback, on_image_selected, class, style }: &Props) -> Html {
    let default_images: Vec<String> = vec![
        ""
    ].into_iter().map(|s| { format!("/img/defaults/{}", s) }).collect();
    let theme = use_theme();
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

            .upload {
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

                .upload {
                    overflow-y: unset;
                    max-height: unset;
                }
            }
        "#
    );

    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::default())));
    html! {
        <Popup class={menu_style} style={style.clone()} {z_index} active={active.clone()} {on_close_callback}>
            <div class="header">
                <h1 style="text-align: center;">{"Image Select"}</h1>
                <hr/>
            </div>
            <div class="container" 
                ondrop={Callback::from(|event: DragEvent| { 
                    event.prevent_default(); 
                    if let Some(files) = event.data_transfer() {
                        let file = files.files().into_iter().nth(0);
                    }
                })}
                ondragover={Callback::from(|event: DragEvent| {event.prevent_default(); })}
                ondragenter={Callback::from(|event: DragEvent| { event.prevent_default(); })}
            >
                <div style="flex: 70%;">
                    <h5>{"My Uploads"}<hr/></h5>
                    <div class="img-display">
                        <ImagePanel />
                        <ImagePanel />
                        <ImagePanel />
                        <ImagePanel />
                        <ImagePanel />
                        <ImagePanel />
                        <ImagePanel />
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
                <div class="upload" style="flex: 30%;">
                    <h5 style="text-align: center;">{"Selected"}</h5>
                    <hr style="width: 90%;" />
                    <div style="display: flex; flex-direction: column; align-items: center;">
                        <SkeletonPane style="width: 128px; height: 128px; margin: 5px;" />
                        <button>{"Use"}</button>
                    </div>

                    <h5 style="text-align: center;">{"Upload"}</h5>
                    <hr style="width: 90%;" />
                    <form style="display: flex; flex-direction: column; align-items: center; width: 90%;">
                        <FormInput<ImageSrcUpdate> 
                            input_type="text" placeholder="File Name" label="" name="file-name" input_ref={NodeRef::default()} 
                            to_type={Callback::from(|s| { ImageSrcUpdate::UpdateName(s) })}
                            onchange={Callback::from(|_| {})} 
                            onblur={Callback::from(|_| {})} 
                            errors={&*validation_errors} 
                        />
                        <input
                            style="width: 239px;"
                            id={"file-upload"}
                            name={"file-upload"}
                            type={"file"}
                            onchange={Callback::from(|_| {})}
                        />
                        <button type="submit">{"Upload"}</button>
                    </form>

                    <h5 style="text-align: center;">{"External Link"}</h5>
                    <hr style="width: 90%;" />
                    <form style="display: flex; flex-direction: column; align-items: center; width: 90%;">
                        <FormInput<ImageSrcUpdate> 
                            input_type="text" placeholder="https://image/link.com" label="" name="image-src" input_ref={NodeRef::default()} 
                            to_type={Callback::from(|s| ImageSrcUpdate::ExternalUrl(s))}
                            onchange={Callback::from(|_| {})} 
                            onblur={Callback::from(|_| {})} 
                            errors={&*validation_errors} 
                        />
                        <button type="submit">{"Submit"}</button>
                    </form>
                </div>
            </div>
        </Popup>
    }
}

#[styled_component(ImagePanel)]
fn image_panel() -> Html {
    let theme = use_theme();
    let style = css!(
        r#"
            margin: 5px;
            border: 3px solid transparent;
            align-self: center;

            &:hover {
                border: 3px solid ${highlight};
            }
        "#, highlight=theme.text_colored_highlight
    );
    html! {
        <SkeletonPane class={style} style="width: 128px; height: 128px;" />
    }
}