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
    // This would provide the information for all atom based elements, as they
    // could retrieve character data based on the active character.
    // TODO: Define game context and character context in client/contexts
    
    // A character viewer can be either rule-set specific or setting specific.
    // Any default character viewer sheet i
    html! {
        <NavBar center_item={Some(html!{<CharacterSelectDropdown/>})}/>
    }
}