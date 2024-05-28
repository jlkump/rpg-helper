use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::client::{display::organisms::nav_bar::*, use_theme};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
}

#[styled_component(PageNotFound)]
pub fn page_not_found(_: &Props) -> Html {
    let style = css!(
        r#"
            display: flex;
            flex-direction: column;
            align-items: center;
        "#
    );
    
    html! {
        <>
        <NavBar/>
        <div class={style}>
            <img src="img/generic/Birb Wizard Transparent.png" />
            <h1>{"Could not find page"}</h1>
            <h3>{"404 Error"}</h3>
        </div>
        </>
    }
}