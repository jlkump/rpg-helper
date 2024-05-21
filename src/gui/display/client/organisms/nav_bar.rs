use stylist::yew::styled_component;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::components::Link;
use stylist::Style;

use crate::gui::display::client::Route;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub center_item: Option<Html>,
    #[prop_or_default]
    pub hamburger_menu_clicked: Callback<()>
}

#[styled_component(PageNavBar)]
pub fn page_nav_bar(props: &Props) -> Html {
    let onclick = props.hamburger_menu_clicked.reform(|_: MouseEvent| ());


    // TODO: Stylize and format correctly
    html! {
        <span class={get_bar_span_style()}>
            <div class={get_logo_style()}>
                <h3>{"RPG Helper"}</h3>
            </div>
            if let Some(elem) = &props.center_item {
                {elem.clone()}
            }
            <div class={get_hamburger_menu_style()}>
                <Icon icon_id={IconId::LucideMenu} width={"2em".to_owned()} height={"2em".to_owned()}/>
            </div>
        </span>
    }
}

fn get_bar_span_style() -> Style {
    Style::new(
        r#"
            display: flex;
            flex-direction: row;
            flex-wrap: nowrap;
            justify-content: space-between;

            background: #e2ded8;

            border-width: 0px 0px 4px 0px;
            border-style: solid;
            border-color: #7a0002;
        "#,
    )
    .expect("Failed to create style")
}

fn get_logo_style() -> Style {
    Style::new(
        r#"
            /* background: #ccb897; */
            color: #7a0002;
            border-radius: 20px;
            padding: 10px;

            -webkit-user-select: none; /* Safari */
            -ms-user-select: none; /* IE 10 and IE 11 */
            user-select: none; /* Standard syntax */

            cursor: pointer;
        "#
    ).expect("Failed to create logo style")
}

fn get_hamburger_menu_style() -> Style {
    Style::new(
        r#"
            cursor: pointer;

            border-radius: 20px;
            /* margin-left: auto; */
            /* margin-right: 10px; */
            padding: 10px;

            align-self: center;

            color: #7a0002;
            /* background: #ccb897; */
        "#
    ).expect("Failed to create hamburger menu style")
}