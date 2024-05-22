use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::client::display::organisms::nav_bar::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
}

#[styled_component(Home)]
pub fn home(_: &Props) -> Html {
    // Display changes based on whether logged-in or not
    html! {
        <NavBar/>
    }
}