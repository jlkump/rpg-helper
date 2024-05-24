use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::client::display::{molecules::character_select_dropdown::CharacterSelectDropdown, organisms::nav_bar::*, pages::sheets::{character_details::CharacterDetails, tabbed_sheet::TabbedSheet}};

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
    let style = css!(
        r#"
            display: flex;
            justify-content: space-evenly;

            -webkit-user-select: none; /* Safari */
            -ms-user-select: none; /* IE 10 and IE 11 */
            user-select: none; /* Standard syntax */
        "#
    );
    html! {
        <>
            <NavBar center_item={Some(html!{<CharacterSelectDropdown/>})}/>
            <div class={style}>
                <CharacterDetails />
                <TabbedSheet />
            </div>
        </>
    }
}