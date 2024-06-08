use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::display::organisms::nav_bar::NavBar;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
}

#[styled_component(RegisterUser)]
pub fn register_user(_: &Props) -> Html {
    // Display changes based on whether logged-in or not
    // TODO: Complete based on tutorial here: https://codevoweb.com/rust-yew-frontend-jwt-access-and-refresh-tokens/
    html! {
        <NavBar />
    }
}