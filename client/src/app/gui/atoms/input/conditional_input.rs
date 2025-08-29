use std::{cell::RefCell, rc::Rc};

use rpg_helper::api::data::{conditional::Conditional, tag::Tag};
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props
{
    pub conditional_id: Option<Rc<RefCell<Tag>>>,
    #[prop_or_default]
    pub default_value: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: AttrValue,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub onchange: Callback<Conditional>,
    #[prop_or_default]
    pub allowed_tag_values: Option<Rc<RefCell<Vec<Tag>>>>,
}

/// This component handles user input for creation of an equation.
/// It can be provided with optional autocomplete suggestions,
/// which can be used to autocomplete tag based input.
/// 
/// The equation input will also provide key-word
/// autocompletion. 
/// 
/// The input performs validation (error checks the input)
/// before the created equation is handed off in the callback.
/// If the input provided is not a valid equation, then the
/// equation callback will not fire.
#[function_component(ConditionalInput)]
pub fn conditional_input(props: &Props) -> Html
{
    let error_message = use_state(|| None);
    let value = use_state(|| props.default_value.clone());
    let onchange = 
    {
        let e_id = props.conditional_id.clone();
        let callback = props.onchange.clone();
        let error_message = error_message.clone();
        let allowed = props.allowed_tag_values.clone();
        let value = value.clone();
        Callback::from(move |e: Event|
            {
                if let Some(target) = e.target()
                {
                    let input_string = target.unchecked_into::<HtmlInputElement>().value();
                    if input_string.is_empty()
                    {
                        error_message.set(Some("Empty input not valid".to_string()));
                    }
                    else
                    {
                        if let Some(conditional_id) = e_id.as_ref()
                        {
                            match Conditional::new(conditional_id.borrow().clone(), &input_string)
                            {
                                Ok(conditional) => 
                                {
                                    if let Some(limited) = allowed.clone()
                                    {
                                        match conditional.check_only_allowed_tags(&*limited.borrow())
                                        {
                                            Ok(_) => 
                                            {
                                                // TODO: Correct input with the equation string we construct
                                                handle_conditional_complete(conditional, &callback, &error_message);
                                            },
                                            Err(e) =>
                                            {
                                                match e
                                                {
                                                    rpg_helper::api::data::template::Templated::Template(t) =>
                                                    {
                                                        error_message.set(Some(format!("Found invalid tag template value: {}", t)));
                                                    },
                                                    rpg_helper::api::data::template::Templated::Complete(t) =>
                                                    {
                                                        error_message.set(Some(format!("Found invalid tag value: {}", t)));
                                                    },
                                                }
                                            },
                                        }
                                    }
                                    else
                                    {
                                        handle_conditional_complete(conditional, &callback, &error_message);
                                    }
                                },
                                Err(e) =>
                                {
                                    log::warn!("Failed to parse conditional: {:?}", e);
                                    error_message.set(Some(format!("{:?}", e)));
                                },
                            }
                        }
                        else
                        {
                            log::warn!("Attempt input conditional without input.");
                            error_message.set(Some(format!("Equation input requires valid tag id.")));
                        }
                    }
                    value.set(AttrValue::from(input_string));
                }
            }
        )
    };

    html!
    {
        <>
            <input
                type="text"
                class={props.class.clone()}
                style={props.style.clone()}
                name={props.name.clone()}
                placeholder={props.placeholder.clone()}
                value={&*value}
                {onchange}
            />
            if let Some(message) = &*error_message
            {
                <span class="input-error">{message}</span>
            }
        </>
    }
}

fn handle_conditional_complete(conditional: Conditional, callback: &Callback<Conditional>, error_message: &UseStateHandle<Option<String>>)
{
    callback.emit(conditional);
    error_message.set(None);
}