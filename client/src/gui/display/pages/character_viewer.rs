use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::display::{molecules::tabbed_pane::TabbedPane, organisms::nav_bar::NavBar, pages::sheets::{character_details::CharacterDetails, tabbed_sheet::TabbedSheet}};


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

    // TODO: Get Character ID from Game Context
    let temp_character_ids = vec!["CharacterID 1", "Character ID 2"].into_iter().map(|s| s.to_owned()).collect();

    html! {
        <>
            <NavBar/>
            <TabbedPane tabs={get_character_tabs(&temp_character_ids)} content={get_character_panes(temp_character_ids)} no_hard_border=true/>
        </>
    }
}

fn get_character_tabs(names: &Vec<String>) -> Vec<Html> {
    names.iter().map(|name| html! { <h4>{name}</h4>}).collect()
}

fn get_character_panes(ids: Vec<String>) -> Vec<Html> {
    ids.into_iter().map(|id| html! { <CharacterPane character_id={id}/> }).collect()
}

#[derive(Properties, PartialEq)]
struct CharacterPaneProps {
    character_id: AttrValue, // String for character to display
}

#[styled_component(CharacterPane)]
fn character_pane(props: &CharacterPaneProps) ->Html {
    let style = css!(
        r#"
            display: flex;
            justify-content: space-evenly;

            -webkit-user-select: none; /* Safari */
            -ms-user-select: none; /* IE 10 and IE 11 */
            user-select: none; /* Standard syntax */
        "#
    );

    // TODO: Use character context based on the Character ID
    html! {
        <div class={style}>
            <CharacterDetails />
            <TabbedSheet />
        </div>
    }
}