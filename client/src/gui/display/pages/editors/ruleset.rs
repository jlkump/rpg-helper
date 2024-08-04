use yew::prelude::*;
use stylist::yew::styled_component;

use crate::{gui::display::{atoms::{loading::SkeletonPane, tooltip::Tooltip}, organisms::{nav_bar::NavBar, searchable_gallery}}, model::data_model::storage::ruleset::RulesetId};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub ruleset_id: RulesetId,
}

#[styled_component(RulesetCreator)]
pub fn ruleset_creator(_: &Props) -> Html {
    html! {
        <NavBar>
            // TODO: 
            // [ ]. Create a Skeleton of the Ruleset Creator 
            // [ ]. Implement functionality for the Ruleset Creator
            // [ ]. Implement functionality for the Ruleset Gallery viewer
            <div style="display: flex; flex-direction: column; align-items: center;">
                // Tab 1 - Type Editor
                // Tab 2 - Wiki Editor
                // Tab 3 - Location Editor
                // Tab 4 - Character Template Creator
            </div>
        </NavBar>
    }
}