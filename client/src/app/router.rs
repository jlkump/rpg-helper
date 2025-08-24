use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::gui::pages::{editor::{display::DisplayEditor, ruleset::RulesetEditor}, general::home::HomePage};

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum Route
{
    #[at("/")]
    Home,
    #[at("/tools")]
    ToolsRoot,
    #[at("/tools/*")]
    Tools,
}

fn switch(route: Route) -> Html
{
    match route
    {
        Route::Home => html! { <HomePage/> },
        Route::ToolsRoot | Route::Tools => html! { <Switch<ToolsRoute> render={switch_tools} />},
    }
}

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum ToolsRoute
{
    #[at("/tools/RulesetEditor")]
    RulesetEditor,
    #[at("/tools/DisplayEditor")]
    DisplayEditor,
}

fn switch_tools(route: ToolsRoute) -> Html
{
    match route
    {
        ToolsRoute::RulesetEditor => html! { <RulesetEditor/> },
        ToolsRoute::DisplayEditor => html! { <DisplayEditor/> },
    }
}

#[derive(Properties, PartialEq)]
pub struct Props;

#[function_component(Router)]
pub fn router(_: &Props) -> Html
{
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}