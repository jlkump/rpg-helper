use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::client::display::{molecules::character_select_dropdown::CharacterSelectDropdown, organisms::nav_bar::*};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
}

#[styled_component(CharacterViewer)]
pub fn character_viewer(_: &Props) -> Html {
    // Displays nothing but navbar if not logged-in
    // Might be useful to have a context for the game currently active
    html! {
        <NavBar center_item={Some(html!{<CharacterSelectDropdown/>})}/>
    }
}