use stylist::{css, yew::Global};
use yew::prelude::*;
use yew_router::prelude::*;

pub mod molecules;
pub mod organisms;
pub mod pages;

use crate::gui::display::client::pages::home::*;

use self::pages::character_sheet::CharacterSheet;

pub fn run_app() {
    yew::Renderer::<App>::new().render();
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/CharacterSheet")]
    CharacterSheet,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::CharacterSheet => html! { <CharacterSheet/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            // TODO: Use Yew Contexts for theme coloring rather than hardcoding
            <Global css={css!(
                r#"
                    html, body {
                        background-color: ${bg};
                        margin: 0px;
                    }
                "#, bg = "#ece9e4"
            )} />
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </>
    }
}

