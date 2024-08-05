use std::{cell::RefCell, rc::Rc};

use gloo::file::{File};
use stylist::{css, Style};
use validator::ValidationErrors;
use wasm_bindgen::JsCast;
use web_sys::{FileList, HtmlInputElement};
use yew::prelude::*;

use crate::gui::contexts::theme::{use_theme, Theme};

#[derive(Properties, PartialEq)]
pub struct Props<T> 
where 
    T: PartialEq + 'static
{
    #[prop_or("text".to_string())]
    pub input_type: String,
    pub name: String,
    #[prop_or_default]
    pub autocomplete: Option<String>,
    #[prop_or("".to_string())]
    pub placeholder: String,
    pub input_ref: NodeRef,
    pub onchange: Callback<T>,
    pub onblur: Callback<(String, T)>,
    pub to_type: Callback<String, T>,
    pub errors: Rc<RefCell<ValidationErrors>>,
    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(FormInput)]
pub fn form_input<T>(props: &Props<T>) -> Html 
where
    T: PartialEq + Clone + 'static,
{
    let val_errors = props.errors.borrow();
    let errors = val_errors.field_errors().clone();
    let empty_errors = vec![];
    let error = match errors.get(&props.name.as_str()) {
        Some(error) => error,
        None => &empty_errors,
    };
    let error_message = match error.get(0) {
        Some(message) => message.to_string(),
        None => "".to_string(),
    };

    let handle_onchange = props.onchange.clone();
    let to_type = props.to_type.clone();
    let onchange = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        handle_onchange.emit(to_type.emit(value));
    });

    let handle_onblur = props.onblur.clone();
    let to_type = props.to_type.clone();
    let name = props.name.clone();
    let on_blur: Callback<FocusEvent> = {
        Callback::from(move |event: FocusEvent| {
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            handle_onblur.emit((name.clone(), to_type.emit(value)));
        })
    };

    let theme = use_theme();
    let div_style = css!(
        r#"
            display: flex;
            flex-direction: column;
            margin-top: 2px;
            margin-bottom: 2px;
            width: 250px;
        "#
    );

    let error_style = get_err_style(&theme);

    let autocomplete = props.autocomplete.clone().unwrap_or(props.name.clone());

    html! {
        <div class={div_style} style={props.style.clone()}>
            <label html={props.name.clone()} hidden=true for={props.name.clone()}></label>
            <input
                id={props.name.clone()}
                name={props.name.clone()}
                autocomplete={autocomplete}
                type={props.input_type.clone()}
                placeholder={props.placeholder.clone()}
                ref={props.input_ref.clone()}
                onchange={onchange}
                onblur={on_blur}
            />
            <span class={error_style}>
                {error_message}
            </span>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct FileInputProps {
    pub name: String,
    pub input_ref: NodeRef,
    pub oninput: Callback<FileList>,
    pub errors: Rc<RefCell<ValidationErrors>>,
}

#[function_component(FileFormInput)]
pub fn file_form_input(props: &FileInputProps) -> Html {
    let val_errors = props.errors.borrow();
    let errors = val_errors.field_errors().clone();
    let empty_errors = vec![];
    let error = match errors.get(&props.name.as_str()) {
        Some(error) => error,
        None => &empty_errors,
    };
    let error_message = match error.get(0) {
        Some(message) => message.to_string(),
        None => "".to_string(),
    };

    let onchange = {
        let handle_onchange = props.oninput.clone();
        let file_input_ref = props.input_ref.clone();
        Callback::from(move |event: Event| {
            event.prevent_default();
            if let Some(value) = file_input_ref.cast::<HtmlInputElement>() {
                if let Some(files) = value.files() {
                    handle_onchange.emit(files.into());
                }
            }
    
        })
    };

    let theme = use_theme();
    let div_style = css!(
        r#"
            display: flex;
            flex-direction: column;
            margin-top: 2px;
            margin-bottom: 2px;
            width: 250px;
        "#
    );

    let error_style = get_err_style(&theme);

    html! {
        <div class={div_style}>
            <label html={props.name.clone()} hidden=true for={props.name.clone()}></label>
            <input
                id={"file-upload"}
                name={"file-upload"}
                type={"file"}
                ref={props.input_ref.clone()}
                {onchange}
            />
            <span class={error_style}>
                {error_message}
            </span>
        </div>
    }
}


fn get_err_style(theme: &Theme) -> Style {
    Style::new(
        format!(r#"
            color: {color};
            word-wrap: break-word;
        "#,
        color = theme.text_colored
        )
    ).unwrap()
}