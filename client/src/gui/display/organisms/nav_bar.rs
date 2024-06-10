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
            <SideBar sidebar_open={*menu_open} exit_callback={onclick.clone()} signed_in=true/>
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
    signed_in: bool,
}

#[styled_component(SideBar)]
fn sider_bar(props: &SideBarProps) -> Html {
    let theme = use_theme();
    let padding = if props.sidebar_open {
        15
    } else {
        0
    };
    let sidebar_style = css!(
        r#"
            background: ${paper_dark};
            display: flex;
            flex-direction: column;
            align-items: center;
            width: 100%;
            padding: ${padding}px;

            ul {
                list-style-type: none;
                width: 100%;
                padding: 0;
            }

            li {
                border-radius: 10px;
                cursor: pointer;
                display: flex;
                align-items: center;
                padding: 10px;
                font-size: 1.5em;
            }

            li:hover {
                background: ${list_hover};
                color: ${text_invert};
            }
        "#,
        paper_dark=theme.paper_dark,
        padding=padding,
        list_hover=theme.panel_color_primary,
        text_invert=theme.text_invert,
    );

    let classes = if props.sidebar_open {
        classes!("sidebar", sidebar_style)
    } else {
        classes!("sidebar", "closed", sidebar_style)
    };

    let copyright_style = css!("color: ${color}; font-size: 0.5em; display: flex; align-items: center; justify-content: center;", color=theme.text_faint);

    let exit_icon_color = css!("color: ${color}; position: absolute; margin: 7px; top: 0; right: 0;", color=theme.h3);

    html! {
        <div class={classes}>
            <div style="display: flex; flex-direction: column; flex; width: 100%;">
                <div class={classes!("exit_sidebar", exit_icon_color)} onclick={props.exit_callback.clone()}>
                    <Icon icon_id={IconId::FontAwesomeSolidXmark} width={"2.5em".to_owned()} height={"2.5em".to_owned()}/>
                </div>
            </div>
            <h3 style="font-size: 2em;">
                {"Menu"}
            </h3>
            if props.signed_in {
                {get_signed_in_menu_options()}
            } else {
                {get_signed_out_menu_options()}
            }


            <div class={copyright_style} style="margin-top: auto; width: 100%;">
                {"Copyright (c) 2024 by J. Landon Kump"}
            </div>
        </div>
    }
}

fn get_signed_in_menu_options() -> Html {
    html! {
        <>
            <div style="width: 100%;">
                <hr/>
            </div>

            <ul>
                <li>
                    <Icon icon_id={IconId::LucideLayoutDashboard} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"Dashboard"}</div>
                </li>
                <li>
                    <Icon icon_id={IconId::OcticonsPersonAdd16} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"Character Creator"}</div>
                </li>
                <li>
                    <Icon icon_id={IconId::LucideHammer} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"Ruleset Creator"}</div>
                </li>
                <li>
                    <Icon icon_id={IconId::LucideMountainSnow} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"Setting Editor"}</div>
                </li>
            </ul>

            <div style="width: 100%;">
                <hr/>
            </div>

            <ul>
                <li>
                    <Icon icon_id={IconId::FeatherServer} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"Host Game"}</div>
                </li>
                <li>
                    <Icon icon_id={IconId::BootstrapBoxArrowInLeft} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"Join Game"}</div>
                </li>
            </ul>
            
            <div style="width: 100%;">
                <hr/>
            </div>

            <ul>
                <li>
                    <Icon icon_id={IconId::BootstrapGear} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"Preferences"}</div>
                </li>
                <li>
                    <Icon icon_id={IconId::OcticonsInfo24} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"About"}</div>
                </li>
            </ul>
        </>
    }
}

fn get_signed_out_menu_options() -> Html {
    html! {
        <>
            <div style="width: 100%;">
                <hr/>
            </div>

            <ul>
                <li>
                    <Icon icon_id={IconId::HeroiconsOutlinePencilSquare} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"Sign Up"}</div>
                </li>
                <li>
                    <Icon icon_id={IconId::OcticonsInfo24} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"About"}</div>
                </li>
            </ul>
        </>
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