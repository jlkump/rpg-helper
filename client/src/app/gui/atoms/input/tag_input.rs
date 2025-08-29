use rpg_helper::api::data::tag::Tag;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props
{
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
    pub onchange: Callback<Tag>,
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
#[function_component(TagInput)]
pub fn tag_input(props: &Props) -> Html
{
    let error_message = use_state(|| None);
    let value = use_state(|| props.default_value.clone());

    let onchange = 
    {
        let callback = props.onchange.clone();
        let error_message = error_message.clone();
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
                        match Tag::from_str(&input_string)
                        {
                            Ok(tag) => 
                            {
                                callback.emit(tag);
                                error_message.set(None);
                            },
                            Err(e) =>
                            {
                                log::warn!("Failed to parse tag: {:?}", e);
                                error_message.set(Some(format!("{:?}", e)));
                            },
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