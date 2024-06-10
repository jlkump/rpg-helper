use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::display::organisms::nav_bar::NavBar;

#[derive(Properties, PartialEq)]
pub struct Props {

}

#[styled_component(RulesetCreator)]
pub fn ruleset_creator(_: &Props) -> Html {
    html! {
        <NavBar>
            <h1>{"Ruleset Creator: TODO"}</h1>
        </NavBar>
    }
}