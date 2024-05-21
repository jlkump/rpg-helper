use stylist::yew::styled_component;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::components::Link;

use crate::gui::display::client::Route;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub hamburger_menu_clicked: Callback<()>
}

#[styled_component(PageNavBar)]
pub fn page_nav_bar(props: &Props) -> Html {
    let onclick = props.hamburger_menu_clicked.reform(|_: MouseEvent| ());

    // TODO: Stylize and format correctly
    html! {
        <span>
            <ul>
                <li>
                    <Link<Route> to={Route::Home}><img style="filter: invert(50%) opacity(0.5) drop-shadow(0 0 0 #32a852)" src="img/generic/Dice-Icon.png" alt="RPG dice logo"/></Link<Route>>
                </li>
                <li>
                    <h1>{"RPG Helper"}</h1>
                </li>
                <li>
                    <a style="cursor: pointer" {onclick}><Icon icon_id={IconId::LucideMenu}/></a>
                </li>
            </ul>
        </span>
    }
}