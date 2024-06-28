use std::{cell::RefCell, rc::Rc};

use stylist::css;
use validator::ValidationErrors;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::gui::contexts::theme::use_theme;

#[derive(Properties, PartialEq)]
pub struct Props<T> 
where 
    T: PartialEq + 'static
{
    #[prop_or("text".to_string())]
    pub input_type: String,
    pub label: String,
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
        "#,
        // button = theme.button_color,
        // button_hover = theme.button_color_hover,
        // button_press = theme.button_color_press,
        // invert = theme.text_invert,
        // bg = theme.panel_secondary,
        // focus = theme.h1
    );

    let label_style = if props.label.eq("") {
        css!("margin: 5px;")
    } else {
        css!("")
    };

    let error_style = css!(
        r#"
            color: ${color};
            word-wrap: break-word;
        "#,
        color = theme.text_colored
    );

    let autocomplete = props.autocomplete.clone().unwrap_or(props.name.clone());

    html! {
    <div class={div_style}>
        <label html={props.name.clone()} class={label_style} for={props.name.clone()}>
            {props.label.clone()}
        </label>
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