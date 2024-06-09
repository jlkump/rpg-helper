use stylist::{css, yew::Global};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::gui::display::pages::user::UserProfile;
use crate::gui::{display::pages::home::*, contexts::style::theme::*};
use crate::gui::display::pages::{character_viewer::CharacterViewer, page_not_found::PageNotFound, user::RegisterUser, user::LoginUser};


pub fn run_app() {
    yew::Renderer::<Root>::new().render();
}

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum Route {
    #[at("/")]
    Home,
    #[at("/Login")]
    Login,
    #[at("/Register")]
    Register,
    #[at("/Profile")]
    Profile,
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
        Route::Login => html! { <LoginUser/>},
        Route::Register => html! { <RegisterUser/> },
        Route::Profile => html! { <UserProfile/>},
    }
}

#[function_component(App)]
fn app() -> Html {
    let theme = use_theme();
    // TODO: Define text sizes

    html! {
        <>
            <Global css={css!(
                r#"
                    html, body {
                        background-color: ${bg};
                        margin: 0px;
                        height: 100%;
                    }

                    h1 {
                        color: ${h1};
                        font-size: 2.5em;
                        margin-top: 10px;
                        margin-bottom: 10px;
                    }

                    h2 {
                        color: ${h2};
                        font-size: 2em;
                        margin-top: 5px;
                        margin-bottom: 5px;
                    }

                    h3 {
                        color: ${h3};
                        font-size: 1em;
                        margin-top: 2px;
                        margin-bottom: 2px;
                    }

                    h4 {
                        color: ${h4};
                        font-size: 1em;
                        margin-top: 2px;
                        margin-bottom: 2px;
                    }

                    h5 {
                        color: ${h5};
                        font-size: 1em;
                        margin-top: 2px;
                        margin-bottom: 2px;
                    }

                    h6 {
                        color: ${h6};
                        font-size: 1em;
                        margin-top: 2px;
                        margin-bottom: 2px;
                    }

                    /* width */
                    ::-webkit-scrollbar {
                        width: 10px;
                    }

                    /* Track */
                    ::-webkit-scrollbar-track {
                        background: rgb(0, 0, 0, 0);
                    }

                    /* Handle */
                    ::-webkit-scrollbar-thumb {
                        background: ${scroll_bar};
                    }

                    /* Handle on hover */
                    ::-webkit-scrollbar-thumb:hover {
                        background: ${scroll_hover};
                    }

                    -webkit-user-select: none; /* Safari */
                    -ms-user-select: none; /* IE 10 and IE 11 */
                    user-select: none; /* Standard syntax */

                "#, bg = theme.paper, h1 = theme.h1, h2 = theme.h2, 
                h3 = theme.h3, h4 = theme.h4, h5 = theme.h5, h6 = theme.h6,
                scroll_bar = theme.scroll_bar, scroll_hover = theme.scroll_bar_hover
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
    // This will also probably replace the theme provider as the User defines the theme used
    html! {
        <ThemeProvider>
            <App />
        </ThemeProvider>
    }
}

