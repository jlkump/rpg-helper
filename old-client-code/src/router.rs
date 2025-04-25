use yew::prelude::*;
use yew_router::prelude::*;

use crate::{gui::display::pages::{generic::{dashboard::Dashboard, error_pages::{ErrorPage, PageNotFound}, home::Home}, editors::ruleset::RulesetCreator, user::{LoginUser, RegisterUser, UserPrefernces, UserProfile}}, model::types::RulesetId};

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum Route {
    // Main Routes
    #[at("/")]
    Home,
    #[at("/About")]
    About,
    #[at("/Dashboard")]
    Dashboard,
    #[at("/Error/:error")]
    Error { error: String },
    #[not_found]
    #[at("/404")]
    NotFound,

    // User Routes
    #[at("/Login")]
    Login,
    #[at("/Register")]
    Register,
    #[at("/Preferences")]
    Preferences,
    #[at("/Profile/:id")]
    Profile { id: String },

    // Character Routes
    #[at("/Character/Viewer")]
    CharacterViewer,
    #[at("/Character/Editor/:id")]
    CharacterEditor { id: String },

    // Ruleset Routes
    #[at("/Ruleset/Viewer")]
    RulesetViewer,
    #[at("/Ruleset/Editor/:id")]
    RulesetEditor { id: String },

    // Setting Routes
    #[at("/Setting/Viewer")]
    SettingViewer,
    #[at("/Setting/Editor/:id")]
    SettingEditor { id: String },

    // Game Routes
    #[at("/Game/Viewer")]
    GameViewer,
    #[at("/Game/Editor/:id")]
    GameEditor { id: String },
    #[at("/Game/Host-Select")]
    GameHostSelect,
    #[at("/Game/Host/:id")]
    GameHost { id: String },
    #[at("/Game/Join/:id")]
    JoinGame { id: String },
    #[at("/Game/Play/:id")]
    PlayGame { id: String },
}

fn switch(routes: Route) -> Html {
    // TODO: Redirect most routes if not logged-in
    match routes {
        Route::Home => html! { <Home/> },
        Route::About => html! { <Redirect<Route> to={Route::Home} />},
        Route::Dashboard => html! { <Dashboard /> },
        Route::Error { error } => html! { <ErrorPage {error} />},
        Route::NotFound => html! { <PageNotFound/> },

        Route::Login => html! { <LoginUser/> },
        Route::Register => html! { <RegisterUser/> },
        Route::Profile { id } => html! { if let Ok(id) = uuid::Uuid::parse_str(&id) { <UserProfile {id}/> } else { <Redirect<Route> to={Route::NotFound} /> } },
        Route::Preferences => html! { <UserPrefernces /> },

        Route::CharacterViewer => todo!(),
        Route::CharacterEditor { id } => todo!(),

        Route::RulesetViewer => todo!(),
        Route::RulesetEditor { id } => html! { <RulesetCreator ruleset_id={RulesetId::parse_str(&id).unwrap()}/> },

        Route::SettingViewer => todo!(),
        Route::SettingEditor { id } => todo!(),

        Route::GameViewer => todo!(), // View active public games able to join
        Route::GameEditor { id } => todo!(),
        Route::GameHostSelect => todo!(),
        Route::GameHost {id } => todo!(),
        Route::JoinGame { id } => todo!(),
        Route::PlayGame { id } => todo!(),
    }
}

#[derive(Properties, PartialEq)]
pub struct Props;

#[function_component(Router)]
pub fn router(_: &Props) -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}