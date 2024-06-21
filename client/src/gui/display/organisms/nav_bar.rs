use gloo::console::log;
use stylist::{yew::styled_component, Style};
use yew::{platform::spawn_local, prelude::*};
use yew_icons::{Icon, IconId};
use yew::{html, Html};
use yew_router::{components::Link, hooks::use_navigator, navigator::Navigator};
use yewdux::use_store;

use crate::{api::user_api::api_logout_user, gui::{contexts::theme::{use_theme, Theme}, display::atoms::{hamburger_menu::HamburgerMenu, loading::Loader, logo::Logo, profile::ProfilePortrait}}, router::Route, store::{set_auth_user, AuthUser}};


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Html,
    #[prop_or_default]
    pub content_class: Classes,
}

#[styled_component(NavBar)]
pub fn nav_bar(props: &Props) -> Html {
    let menu_open = use_state(|| false);
    let onclick = {
        let menu_open_clone = menu_open.clone();
        Callback::from(move |_| menu_open_clone.set(!*menu_open_clone))
    };
    let theme = use_theme();
    let (store, _) = use_store::<AuthUser>();
    let user = store.auth_user.clone();
    let logged_in = user.is_some();
    log!(format!("Navbar: user is {:?}", user));
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
            <SideBar sidebar_open={*menu_open} exit_callback={onclick.clone()} {logged_in}/>
            <div class={body_classes}>
                <span class={get_bar_style(&theme)}>
                    <div class={get_hamburger_style(&theme)} onclick={onclick}>
                        <HamburgerMenu color={theme.hamburger_menu.clone()} open={*menu_open}/>
                    </div>
                    <Logo />
                    <UserMenu {logged_in}/>
                </span>
                <div class={props.content_class.clone()} style="flex: 90%;">
                    {props.children.clone()}
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct UserMenuProps {
    logged_in: bool
}

#[styled_component(UserMenu)]
fn user_menu(props: &UserMenuProps) -> Html {
    // Display is dependant upon whether a User is logged in
    // If logged-in, display user profil picture and have user drop-down options
    // If logged-out, display user sign-in
    let theme = use_theme();
    let style = css!(
        r#"
            margin: 4px;
            margin-right: 15px;
            display: flex;
            align-items: center;
            justify-content: center;
        "#
    );

    let hover_color = css!(
        r#"
            cursor: pointer;
            &:hover {
                color: ${hover};
            }
        "#,
        hover = theme.text_colored_highlight,
    );

    let navigator = use_navigator().unwrap();
    let loading = use_state(|| false);
    let (store, dispatch) = use_store::<AuthUser>();

    let handle_logout = {
        let navigator = navigator.clone();
        let loading = loading.clone();
        let dispatch = dispatch.clone();
        Callback::from(move |_: MouseEvent| {
            let loading = loading.clone();
            let dispatch = dispatch.clone();
            let navigator = navigator.clone();
            spawn_local(async move {
                let res = api_logout_user().await;
                loading.set(true);
                match res {
                    Ok(_) => {
                        loading.set(false);
                        set_auth_user(None, dispatch);
                        navigator.push(&Route::Home);
                    },
                    Err(e) => {
                        loading.set(false);
                        e.route_based_on_err(&navigator);
                    },
                }

            });
        })
    };
    html! {
        if props.logged_in {
            <div class={style}>
                if *loading {
                    <Loader color={theme.text_colored.clone()} />
                } else {
                    <div onclick={handle_logout}>
                        <h3 class={hover_color.clone()}>{"Logout"}</h3>
                    </div>
                    if let Some(user) = &store.auth_user {
                        <Link<Route> to={Route::Profile { id: user.id.to_string() }}><div>
                            <ProfilePortrait style="margin-left: 10px;" hover=true loading={*loading} src={user.profile_photo.clone()} />
                        </div></Link<Route>>
                    }
                }
            </div>
        } else {            
            <div class={style}>
                <div style="margin-right: 10px">
                    <Link<Route> to={Route::Login} classes={css!("text-decoration: none;")}>
                        <h3 class={hover_color.clone()}>{"Login"}</h3>
                    </Link<Route>>
                </div>
                <div>
                    <Link<Route> to={Route::Register} classes={css!("text-decoration: none;")}>
                        <h3 class={hover_color.clone()}>{"Sign up"}</h3>
                    </Link<Route>>
                </div>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
struct SideBarProps {
    sidebar_open: bool,
    exit_callback: Callback<MouseEvent>,
    logged_in: bool,
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

    let navigator = use_navigator().unwrap();

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
            if props.logged_in {
                {get_signed_in_menu_options(&navigator)}
            } else {
                {get_signed_out_menu_options(&navigator)}
            }


            <div class={copyright_style} style="margin-top: auto; width: 100%;">
                {"Copyright (c) 2024 by J. Landon Kump"}
            </div>
        </div>
    }
}

fn get_route_callback(navigator: &Navigator, route: Route) -> Callback<MouseEvent> {
    let nav_clone = navigator.clone();
    Callback::from(move |_: MouseEvent| nav_clone.push(&route))
}

fn get_signed_in_menu_options(navigator: &Navigator) -> Html {
    html! {
        <>
            <div style="width: 100%;">
                <hr/>
            </div>

            <ul>
                <li onclick={get_route_callback(navigator, Route::Dashboard)}>
                    <Icon icon_id={IconId::LucideLayoutDashboard} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"Dashboard"}</div>
                </li>
                <li onclick={get_route_callback(navigator, Route::CharacterCreator)}>
                    <Icon icon_id={IconId::OcticonsPersonAdd16} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"Character Creator"}</div>
                </li>
                <li onclick={get_route_callback(navigator, Route::RulesetCreator)}>
                    <Icon icon_id={IconId::LucideHammer} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"Ruleset Creator"}</div>
                </li>
                <li onclick={get_route_callback(navigator, Route::SettingEditor)}>
                    <Icon icon_id={IconId::LucideMountainSnow} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"Setting Editor"}</div>
                </li>
            </ul>

            <div style="width: 100%;">
                <hr/>
            </div>

            <ul>
                <li onclick={get_route_callback(navigator, Route::HostGame)}>
                    <Icon icon_id={IconId::FeatherServer} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"Host Game"}</div>
                </li>
                <li onclick={get_route_callback(navigator, Route::JoinGame)}>
                    <Icon icon_id={IconId::BootstrapBoxArrowInLeft} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"Join Game"}</div>
                </li>
            </ul>
            
            <div style="width: 100%;">
                <hr/>
            </div>

            <ul>
                <li onclick={get_route_callback(navigator, Route::Preferences)}>
                    <Icon icon_id={IconId::BootstrapGear} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"Preferences"}</div>
                </li>
                <li onclick={get_route_callback(navigator, Route::About)}>
                    <Icon icon_id={IconId::OcticonsInfo24} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"About"}</div>
                </li>
            </ul>
        </>
    }
}

fn get_signed_out_menu_options(navigator: &Navigator) -> Html {
    html! {
        <>
            <div style="width: 100%;">
                <hr/>
            </div>

            <ul>
                <li onclick={get_route_callback(navigator, Route::Login)}>
                    <Icon icon_id={IconId::FeatherCornerDownRight} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"Login"}</div>
                </li>
                <li onclick={get_route_callback(navigator, Route::Register)}>
                    <Icon icon_id={IconId::HeroiconsOutlinePencilSquare} width={"1em".to_owned()} height={"1em".to_owned()}/>
                    <div style="margin-left: 10px;">{"Sign Up"}</div>
                </li>
                <li onclick={get_route_callback(navigator, Route::About)}>
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
            z-index: 100;

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