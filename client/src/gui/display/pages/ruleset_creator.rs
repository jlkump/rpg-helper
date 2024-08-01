use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::display::{atoms::{loading::SkeletonPane, tooltip::Tooltip}, organisms::{nav_bar::NavBar, searchable_gallery}};

#[derive(Properties, PartialEq)]
pub struct Props {

}

#[styled_component(RulesetCreator)]
pub fn ruleset_creator(_: &Props) -> Html {
    html! {
        <NavBar>
            <div style="display: flex; flex-direction: column; align-items: center;">
                <h2 style="text-align: center;">{"Rulesets"}<hr/></h2>
                {searchable_gallery::test_gallery()}
            </div>
        </NavBar>
    }
}