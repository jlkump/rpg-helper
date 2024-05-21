use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::display::client::organisms::nav_bar::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
}

#[styled_component(Home)]
pub fn home(_: &Props) -> Html {
    html! {
        <PageNavBar/>
    }
}