use yew::prelude::*;
use stylist::yew::styled_component;

use crate::{gui::display::{atoms::collapsable_headers::{Header, HeaderType}, organisms::{image_menu::ImageMenu, nav_bar::NavBar}}, model::types::ImageData};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
}

#[styled_component(Home)]
pub fn home(_: &Props) -> Html {
    html! {
        <NavBar>
            {"TODO"}
        </NavBar>
    }
}