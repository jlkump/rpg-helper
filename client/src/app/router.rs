use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::gui::pages::editor::display::DisplayEditor;

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum Route
{
    #[at("/")]
    Home,
}

fn switch(route: Route) -> Html
{
    match route
    {
        Route::Home => html! { <DisplayEditor/> },
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