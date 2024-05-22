use std::borrow::Borrow;

use stylist::yew::styled_component;
use yew::{html::IntoPropValue, prelude::*};
use yew_icons::{Icon, IconId};
use yew::{html, use_context, Html};
use yew_router::components::Link;
use stylist::Style;

use crate::gui::{client::Route, style::theme::{use_theme, Theme}};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub center_item: Option<Html>,
    #[prop_or_default]
    pub hamburger_menu_clicked: Callback<()>
}

#[styled_component(NavBar)]
pub fn nav_bar(props: &Props) -> Html {
    let onclick = props.hamburger_menu_clicked.reform(|_: MouseEvent| ());
    let theme = use_theme();

    // TODO: Stylize and format correctly
    html! {
        <span class={get_bar_span_style(&theme)}>
            <div class={get_logo_style(&theme)}>
            <Link<Route> to={Route::Home}><h3>{"RPG Helper"}</h3></Link<Route>>
            </div>
            if let Some(elem) = &props.center_item {
                {elem.clone()}
            }
            <div class={get_hamburger_menu_style(&theme)}>
                <Icon onclick={onclick} icon_id={IconId::LucideMenu} width={"2em".to_owned()} height={"2em".to_owned()}/>
            </div>
        </span>
    }
}

fn get_bar_span_style(theme: &Theme) -> Style {
    Style::new(
        format!(
            r#"
                display: flex;
                flex-direction: row;
                flex-wrap: nowrap;
                justify-content: space-between;
        
                background: {};
        
                border-width: 0px 0px 4px 0px;
                border-style: solid;
                border-color: {};
            "#,
            theme.paper_dark,
            theme.navbar_line
        )
    )
    .expect("Failed to create style")
}

fn get_logo_style(theme: &Theme) -> Style {
    Style::new(
        format!(
            r#"
                border-radius: 20px;
                padding: 10px;
    
                -webkit-user-select: none; /* Safari */
                -ms-user-select: none; /* IE 10 and IE 11 */
                user-select: none; /* Standard syntax */
    
                cursor: pointer;
                a {{
                    color: {0};
                    text-decoration: none;
                }}

            "#,
            theme.logo
        )
    ).expect("Failed to create logo style")
}

fn get_hamburger_menu_style(theme: &Theme) -> Style {
    Style::new(
        format!(
            r#"
                cursor: pointer;
    
                border-radius: 20px;
                padding: 10px;
    
                align-self: center;
    
                color: {};
            "#,
            theme.hamburger_menu
        )
    ).expect("Failed to create hamburger menu style")
}