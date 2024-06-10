use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::display::organisms::nav_bar::NavBar;

#[derive(Properties, PartialEq)]
pub struct Props {

}

#[styled_component(SettingEditor)]
pub fn setting_editor(_: &Props) -> Html {
    html! {
        <NavBar>
            <h1>{"Setting Editor: TODO"}</h1>
        </NavBar>
    }
}