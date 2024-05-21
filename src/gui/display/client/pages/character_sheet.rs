use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::display::client::{molecules::character_select_dropdown::CharacterSelectDropdown, organisms::nav_bar::*};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
}

#[styled_component(CharacterSheet)]
pub fn home(_: &Props) -> Html {
    html! {
        <PageNavBar center_item={Some(html!{<CharacterSelectDropdown/>})}/>
    }
}