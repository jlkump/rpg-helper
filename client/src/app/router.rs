use yew::prelude::*;
use yew_router::prelude::*;

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
        Route::Home => html! { <h1>{"Home Page WIP"}</h1>},
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