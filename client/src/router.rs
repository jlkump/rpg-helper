use yew::prelude::*;
use yew_router::prelude::*;

use crate::gui::display::pages::{character_creator::CharacterCreator, character_viewer::CharacterViewer, dashboard::Dashboard, error_pages::{ErrorPage, PageNotFound}, home::Home, ruleset_creator::RulesetCreator, setting_editor::SettingEditor, user::{LoginUser, RegisterUser, UserProfile, UserPrefernces}};

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum Route {
    #[at("/")]
    Home,
    #[at("/About")]
    About,
    #[at("/Login")]
    Login,
    #[at("/Register")]
    Register,
    #[at("/Dashboard")]
    Dashboard,
    #[at("/Profile/:id")]
    Profile { id: String },
    #[at("/Preferences")]
    Preferences,
    #[at("/Character-Creator")]
    CharacterCreator,
    #[at("/Ruleset-Creator")]
    RulesetCreator,
    #[at("/Setting-Editor")]
    SettingEditor,
    #[at("/Game-Select")]
    JoinGame,
    #[at("/Game-Host")]
    HostGame,
    #[at("/Character-Sheet")]
    CharacterSheet,
    #[at("/Error/:error")]
    Error { error: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    // TODO: Redirect most routes if not logged-in
    match routes {
        Route::Home => html! { <Home/> },
        Route::About => html! { <Redirect<Route> to={Route::Home} />},
        Route::Login => html! { <LoginUser/> },
        Route::Register => html! { <RegisterUser/> },
        Route::Dashboard => html! { <Dashboard /> },
        Route::Profile { id } => html! { if let Ok(id) = uuid::Uuid::parse_str(&id) { <UserProfile {id}/> } else { <Redirect<Route> to={Route::NotFound} /> } },
        Route::Error { error } => html! { <ErrorPage {error} />},
        Route::Preferences => html! { <UserPrefernces /> },
        Route::CharacterCreator => html! { <CharacterCreator /> },
        Route::RulesetCreator => html! { <RulesetCreator /> },
        Route::SettingEditor => html! { <SettingEditor /> },
        Route::JoinGame => html! { <Redirect<Route> to={Route::Home} /> },
        Route::HostGame => html! { <Redirect<Route> to={Route::Home} /> },
        Route::CharacterSheet => html! { <CharacterViewer/> },
        Route::NotFound => html! { <PageNotFound/> },
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