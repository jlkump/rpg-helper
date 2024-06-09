use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::display::organisms::nav_bar::NavBar;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
}

#[styled_component(Home)]
pub fn home(_: &Props) -> Html {
    // Display changes based on whether logged-in or not
    html! {
        <NavBar>
            <div style="display: flex; height: 100%; width: 100%; justify-content: center;">
                <h1>{"Welcome!"}</h1>
            </div>
        </NavBar>
    }
}