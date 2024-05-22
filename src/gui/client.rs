use stylist::{css, yew::Global};
use yew::prelude::*;
use yew_router::prelude::*;

mod display;

use crate::gui::{client::display::pages::home::*, style::theme::*};

use self::display::pages::character_viewer::CharacterViewer;

pub fn run_app() {
    yew::Renderer::<Root>::new().render();
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
        Route::CharacterSheet => html! { <CharacterViewer/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    let theme = use_theme();

    html! {
        <>
            <Global css={css!(
                r#"
                    html, body {
                        background-color: ${bg};
                        margin: 0px;
                    }
                "#, bg = theme.paper
            )} />
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </>
    }
}

#[function_component(Root)]
fn root() -> Html {
    html! {
        <ThemeProvider>
            <App />
        </ThemeProvider>
    }
}

