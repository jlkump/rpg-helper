use std::{cell::RefCell, rc::Rc};

use stylist::css;
use validator::ValidationErrors;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::gui::contexts::style::theme::use_theme;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or("text".to_string())]
    pub input_type: String,
    pub label: String,
    pub name: String,
    #[prop_or("".to_string())]
    pub placeholder: String,
    pub input_ref: NodeRef,
    pub onchange: Callback<String>,
    pub onblur: Callback<String>,
    pub errors: Rc<RefCell<ValidationErrors>>,
}

#[function_component(FormInput)]
pub fn form_input(props: &Props) -> Html {
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
    let onchange = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        handle_onchange.emit(value);
    });

    let handle_onblur = props.onblur.clone();
    let on_blur: Callback<FocusEvent> = {
        Callback::from(move |event: FocusEvent| {
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            handle_onblur.emit(value);
        })
    };

    let theme = use_theme();
    let label_style = css!(
        r#"
            margin: 5px;
        "#
    );

    html! {
    <div>
      <label html={props.name.clone()} class={label_style}>
        {props.label.clone()}
      </label>
      <input
        type={props.input_type.clone()}
        placeholder={props.placeholder.clone()}
        ref={props.input_ref.clone()}
        onchange={onchange}
        onblur={on_blur}
      />
    <span class="">
        {error_message}
    </span>
    </div>
    }
}