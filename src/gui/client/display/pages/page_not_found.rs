use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::client::{display::organisms::nav_bar::*, use_theme};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
}

#[styled_component(PageNotFound)]
pub fn page_not_found(_: &Props) -> Html {
    let theme = use_theme();
    let style = css!(
        r#"
            display: flex;
            flex-direction: column;
            align-items: center;

            h1 {
                color: ${h1};
            }

            h3 {
                color: ${h3};
            }
        "#,
        h1 = theme.h1,
        h3 = theme.text_faint
    );
    
    html! {
        <>
        <NavBar/>
        <div class={style}>
            <h1>{"Sorry, we couldn't find the given page"}</h1>
            <h3>{"404 Error"}</h3>
        </div>
        </>
    }
}