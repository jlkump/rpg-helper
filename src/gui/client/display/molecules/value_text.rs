use std::ops::Deref;

use gloo::{console::log, timers::callback::Timeout};
use web_sys::window;
use yew::prelude::*;
use stylist::{css, yew::styled_component};

use crate::gui::client::{contexts::character::use_character, display::atoms::loader::Loader, use_theme};

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub value: String,
}

#[styled_component(ValueText)]
pub fn value_text(props: &Props) -> Html {
    // let character_context = use_character().unwrap();

    html! {

    }
}