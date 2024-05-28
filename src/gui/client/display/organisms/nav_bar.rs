use std::borrow::Borrow;

use stylist::yew::styled_component;
use yew::{html::IntoPropValue, prelude::*};
use yew_icons::{Icon, IconId};
use yew::{html, use_context, Html};
use yew_router::components::Link;
use stylist::Style;

use crate::gui::client::{use_theme, Route, Theme};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
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
            <div class={get_hamburger_menu_style(&theme)}>
                <Icon onclick={onclick} icon_id={IconId::LucideMenu} width={"2em".to_owned()} height={"2em".to_owned()}/>
            </div>
            <div class={get_logo_style(&theme)}>
                <Link<Route> to={Route::Home} classes={css!("display: flex; flex-direction: row; align-items: center;")}><img src="img/generic/Dice RPG Icon.svg" width=30px height=30px/><h3>{"RPG Helper"}</h3></Link<Route>>
            </div>
            <UserMenu />
        </span>
    }
}

#[derive(Properties, PartialEq)]
struct UserMenuProps {

}

#[styled_component(UserMenu)]
fn user_menu(props: &UserMenuProps) -> Html {
    // Display is dependant upon whether a User is logged in
    // If logged-in, display user profil picture and have user drop-down options
    // If logged-out, display user sign-in
    let style = css!(
        r#"
            margin: 4px;
            margin-right: 15px;
            display: flex;
            align-items: center;
            justify-content: center;

            -webkit-user-select: none; /* Safari */
            -ms-user-select: none; /* IE 10 and IE 11 */
            user-select: none; /* Standard syntax */

            cursor: pointer;
        "#
    );
    html! {
        <div class={style}>
            <h3>{"Sign in"}</h3>
        </div>
    }
}

fn get_bar_span_style(theme: &Theme) -> Style {
    Style::new(
        format!(
            r#"
                position: sticky;
                position: -webkit-sticky;
                top: 0;
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