use yew::prelude::*;
use yew_router::prelude::Link;

use crate::app::{context::focus::use_focus, router::{Route, ToolsRoute}};

#[derive(Properties, Clone, PartialEq)]
pub struct Props
{
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component(Navbar)]
pub fn navbar(props: &Props) -> Html
{
    let active = use_state(|| false);

    let fctx = use_focus();
    let onclick = 
    {
        let active = active.clone();
        Callback::from(
            move |_|
            {
                active.set(!*active);
                fctx.clear_focus();
            }
        )
    };


    html!
    {
        <nav class={classes!("navbar", props.class.clone())} style={props.style.clone()}>
            <span class={"content-container"}>
                <Link<Route> classes={"logo"} to={Route::Home}><img src="/assets/Dice RPG Icon.svg"/>{"RPG Helper"}</Link<Route>>
                <button class={if *active { "nav-toggle active" } else { "nav-toggle" }} {onclick}>
                    <span class="bar"></span>
                    <span class="bar"></span>
                    <span class="bar"></span>
                </button>
                <ul class={if *active { "menu active" } else { "menu" }}>
                    <li><a>{"Dashboard"}</a></li>
                    <NavbarDropdown dropdown_name={"Play"}>
                        <ul class={"dropdown-menu"}>
                            <li><a>{"Host Game"}</a></li>
                            <li><a>{"Join Game"}</a></li>
                        </ul>
                    </NavbarDropdown>
                    <NavbarDropdown dropdown_name={"Tools"}>
                        <ul class={"dropdown-menu"}>
                            <li><a>{"Character Creator"}</a></li>
                            <li><a>{"Ruleset Creator"}</a></li>
                            <li><a>{"Theme Editor"}</a></li>
                            <li><Link<ToolsRoute> to={ToolsRoute::DisplayEditor} >{"Display Editor"}</Link<ToolsRoute>></li>
                        </ul>
                    </NavbarDropdown>
                    <li><a>{"About"}</a></li>
                </ul>
            </span>
        </nav>
    }
}


#[derive(Properties, Clone, PartialEq)]
struct DropdownProps
{
    dropdown_name: AttrValue,
    children: Html,
    #[prop_or_default]
    class: Classes,
    #[prop_or_default]
    style: Option<AttrValue>,
}

#[function_component(NavbarDropdown)]
fn nav_dropdown_menu(props: &DropdownProps) -> Html
{
    let fctx = use_focus();

    let c = if fctx.get_focus() == Some(props.dropdown_name.as_str())
    {
        "dropdown focused"
    }
    else
    {
        "dropdown"
    };

    let onclick =
    {
        let name = props.dropdown_name.to_string();
        Callback::from(move |_| fctx.toggle_focus(&name))
    };

    html!
    {
        <li class={c}>
            <a {onclick}>{props.dropdown_name.clone()}</a>
            {props.children.clone()}
        </li>
    }
}