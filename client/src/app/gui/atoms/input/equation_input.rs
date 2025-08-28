use std::{cell::RefCell, rc::Rc};

use rpg_helper::api::data::{equation::Equation, tag::Tag};
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props
{
    pub equation_id: Rc<RefCell<Tag>>,
    #[prop_or_default]
    pub default_value: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: AttrValue,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub onchange: Callback<Equation>,
    #[prop_or_default]
    pub allowed_tag_values: Option<Rc<RefCell<Vec<Tag>>>>,
}

fn handle_equation_complete(equation: Equation, callback: &Callback<Equation>, error_message: &UseStateHandle<Option<String>>)
{
    callback.emit(equation);
    error_message.set(None);
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
#[function_component(EquationInput)]
pub fn equation_input(props: &Props) -> Html
{
    let error_message = use_state(|| None);
    let onchange = 
    {
        let e_id = props.equation_id.clone();
        let callback = props.onchange.clone();
        let error_message = error_message.clone();
        let allowed = props.allowed_tag_values.clone();
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
                        match Equation::new(e_id.as_ref().borrow().clone(), &input_string)
                        {
                            Ok(equation) => 
                            {
                                if let Some(limited) = allowed.clone()
                                {
                                    match equation.check_only_allowed_tags(&*limited.borrow())
                                    {
                                        Ok(_) => 
                                        {
                                            // TODO: Correct input with the equation string we construct
                                            handle_equation_complete(equation, &callback, &error_message);
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
                                    handle_equation_complete(equation, &callback, &error_message);
                                }
                            },
                            Err(e) =>
                            {
                                log::warn!("Failed to parse equation: {:?}", e);
                                error_message.set(Some(format!("{:?}", e)));
                            },
                        }
                    }
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
                {onchange}
            />
            if let Some(message) = &*error_message
            {
                <span class="input-error">{message}</span>
            }
        </>
    }
}