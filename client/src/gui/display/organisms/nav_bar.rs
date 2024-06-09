use stylist::{style, yew::styled_component, Style};
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew::{html, Html};
use yew_router::components::Link;

use crate::{client::Route, gui::{contexts::style::theme::{use_theme, Theme}, display::atoms::logo::Logo}};


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[styled_component(NavBar)]
pub fn nav_bar(props: &Props) -> Html {
    let menu_open = use_state(|| false);
    let onclick = {
        let menu_open_clone = menu_open.clone();
        Callback::from(move |_| menu_open_clone.set(!*menu_open_clone))
    };
    let theme = use_theme();

    let page_style = css!(
        r#"
            display: flex;
            flex-direction: row;
            height: 100%;

            .sidebar {
                flex: 20%;
                transition: 0.5s;
                overflow-y: auto;
                overflow-x: hidden;
                border-right: 3px solid ${siderbar_line};
            }

            .content {
                flex: 100%;
                overflow-x: hidden;
            }

            .sidebar.closed {
                flex: 0;
                border-right: 0px;
            }

            .exit_sidebar {
                display: none;
            }

            @media screen and (max-width: 800px) {
                .sidebar {
                    border-right: 0px;
                }

                .sidebar, .content {
                    flex: 100%;
                }

                .content.closed, .sidebar.closed {
                    flex: 0;
                }

                .exit_sidebar {
                    display: flex;
                }
            }
        "#,
        siderbar_line=theme.border_colored
    );
    let body_style = css!(
        r#"
            display: flex;
            flex-direction: column;
        "#
    );
    let body_classes = if *menu_open {
        classes!("content", "closed", body_style)
    } else {
        classes!("content", body_style)
    };

    html! {
        <div class={page_style}>
            <SideBar sidebar_open={*menu_open} exit_callback={onclick.clone()}/>
            <div class={body_classes}>
                <span class={get_bar_style(&theme)}>
                    <div class={get_hamburger_style(&theme)}>
                        <Icon onclick={onclick} icon_id={IconId::LucideMenu} width={"2em".to_owned()} height={"2em".to_owned()}/>
                    </div>
                    <Logo />
                    <UserMenu />
                </span>
                <div style="flex: 90%;">
                    {props.children.clone()}
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct UserMenuProps;

#[styled_component(UserMenu)]
fn user_menu(_: &UserMenuProps) -> Html {
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

            cursor: pointer;
        "#
    );
    html! {
        <div class={style}>
            <Link<Route> to={Route::Register} classes={css!("text-decoration: none;")}><h3>{"Sign in"}</h3></Link<Route>>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct SideBarProps {
    sidebar_open: bool,
    exit_callback: Callback<MouseEvent>,
}

#[styled_component(SideBar)]
fn sider_bar(props: &SideBarProps) -> Html {
    let theme = use_theme();
    let sidebar_style = css!(
        r#"
            background: ${paper_dark};
            display: flex;
            flex-direction: column;
            align-items: center;
            width: 100%;

            ul {
                list-style-type: none;
                padding: 0;
            }
        "#,
        paper_dark=theme.paper_dark
    );

    let classes = if props.sidebar_open {
        classes!("sidebar", sidebar_style)
    } else {
        classes!("sidebar", "closed", sidebar_style)
    };

    let exit_icon_color = css!("color: ${color};", color=theme.h2);

    html! {
        <div class={classes}>
            <div style="display: flex; flex-direction: column; flex; width: 100%;">
                <h3 style="font-size: 2em;">{"Menu"}</h3>
                <div class={classes!("exit_sidebar", exit_icon_color)} onclick={props.exit_callback.clone()}>
                    <h2>{"Close"}</h2>
                    <Icon icon_id={IconId::FontAwesomeSolidXmark} width={"2em".to_owned()} height={"2em".to_owned()}/>
                </div>
            </div>

            <ul>
                <li>{"Item 1"}</li>
            </ul>
        </div>
    }
}

fn get_bar_style(theme: &Theme) -> Style {
    Style::new(format!(
        r#"
            position: sticky;
            position: -webkit-sticky;
            top: 0;
            flex: 10%;
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
    )).unwrap()
}

fn get_hamburger_style(theme: &Theme) -> Style {
    Style::new(format!(
        r#"
            cursor: pointer;
    
            border-radius: 20px;
            padding: 10px;
    
            align-self: center;
    
            color: {};
        "#,
        theme.hamburger_menu
    )).unwrap()
}