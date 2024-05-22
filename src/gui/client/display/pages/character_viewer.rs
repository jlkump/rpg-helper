use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::client::display::{molecules::character_select_dropdown::CharacterSelectDropdown, organisms::nav_bar::*};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
}

#[styled_component(CharacterViewer)]
pub fn character_viewer(_: &Props) -> Html {
    html! {
        <NavBar center_item={Some(html!{<CharacterSelectDropdown/>})}/>
    }
}