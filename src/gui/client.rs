use stylist::{css, yew::Global};
use yew::prelude::*;
use yew_router::prelude::*;

mod display;

use crate::gui::{client::display::pages::home::*, style::theme::*};

use self::display::pages::{character_viewer::CharacterViewer, page_not_found::PageNotFound};

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
        Route::NotFound => html! { <PageNotFound/> },
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
    // TODO: Define a UserProvider for providing the user context to each element
    // This will make it much easier to know if a user is logged in or not
    html! {
        <ThemeProvider>
            <App />
        </ThemeProvider>
    }
}

