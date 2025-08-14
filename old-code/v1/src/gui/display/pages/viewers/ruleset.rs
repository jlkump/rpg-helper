use yew::prelude::*;
use stylist::yew::styled_component;
use yew_router::hooks::use_navigator;

use crate::{gui::display::organisms::{nav_bar::NavBar, searchable_gallery}, router::Route};

#[derive(Properties, PartialEq)]
pub struct Props {

}

#[styled_component(RulesetGallery)]
pub fn ruleset_creator(_: &Props) -> Html {
    let create_clicked = {
        if let Some(navigator) = use_navigator() {
            // TODO: do loading while waiting for response to push to new editor
            Some(Callback::from(move |_| {navigator.push(&Route::RulesetEditor { id: "TODO".to_string() })}))
        } else {
            None
        }
    };
    html! {
        <NavBar>
            // TODO: 
            // [ ]. Create a Skeleton of the Ruleset Creator 
            // [ ]. Implement functionality for the Ruleset Creator
            // [ ]. Implement functionality for the Ruleset Gallery viewer
            <div style="display: flex; flex-direction: column; align-items: center;">
                <h2 style="text-align: center; position: relative;">
                    {"Rulesets"}
                <hr/></h2>
                {searchable_gallery::test_gallery(create_clicked)}

            </div>
        </NavBar>
    }
}