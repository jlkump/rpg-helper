use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::display::{atoms::{loading::SkeletonPane, tooltip::Tooltip}, organisms::nav_bar::NavBar};

#[derive(Properties, PartialEq)]
pub struct Props {

}

#[styled_component(RulesetCreator)]
pub fn ruleset_creator(_: &Props) -> Html {
    html! {
        <NavBar>
            <div style="display: flex; flex-direction: column;">
                <h2 style="text-align: center;">{"Rulesets"}<hr/></h2>
                <div style="display: flex; flex-direction: column;">
                    <div style="display: flex; justify-content: space-between; flex: 20%;">
                        <div style="border: 3px solid black;">
                            {"Search"}
                        </div>
                        <div>
                            {"Sort ..."}
                        </div>
                        <div>
                            {"See Mine"}
                        </div>
                    </div>
                    <div style="flex: 80%;">
                        <RulesetBrief />
                    </div>
                </div>
            </div>
        </NavBar>
    }
}

#[styled_component(RulesetBrief)]
pub fn ruleset_brief() -> Html {

    html! {
        <Tooltip tooltip_content={html! {<div>{"Ruleset Description: TODO"}</div>}}>
            <SkeletonPane style="width: 128px; height: 128px;"/>
        </Tooltip>
    }
}