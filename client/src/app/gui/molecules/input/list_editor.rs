use std::{cell::RefCell, rc::Rc};

use rpg_helper::api::data::{equation::Equation, tag::Tag};
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T: Clone + PartialEq>
{
    /// The html panel is expected to fit within the scrollable base of this
    /// list editor
    pub data_to_html_panel: Callback<T, Html>,
    pub add_click_allowed: bool,
    pub on_add_clicked: Callback<()>,
    /// Assumes that all data in the vec is unique.
    pub data_ref: Vec<T>,

    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(ListEditor)]
pub fn list_editor<T: Clone + PartialEq>(props: &Props<T>) -> Html
{
    // TODO: Add the ability to add to the list
    // by clicking the add button at the top of the list editor
    // The logic for the add is then handled by the owner of this HTML element.
    // Todo this:
    // 1. Scrollable sub-element to hold list of html panels
    // 2. header above scrollable sub elements
    // 3. header toolbar (might be better as a separate html input?)
    //      - That way we have a simple list display that simply has a header element
    //      - Such header element can then handle change callbacks to the data
    html!
    {
        <div class={props.class.clone()} style={props.style.clone()}>
        {
            props.data_ref.iter().map(
                |t| { props.data_to_html_panel.emit(t.clone()) }
            ).collect::<Html>()
        }
        </div>
    }
}