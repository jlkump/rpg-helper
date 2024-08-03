use yew::prelude::*;
use stylist::yew::styled_component;
use yew_router::components::Link;

use crate::{gui::display::{atoms::scroll_div::ScrollDiv, organisms::nav_bar::NavBar}, router::Route};

#[derive(Properties, Clone, PartialEq)]
pub struct PageNotFoundProps;

#[styled_component(PageNotFound)]
pub fn page_not_found(_: &PageNotFoundProps) -> Html {    
    html! {
        <NavBar content_class={css!("display: flex; flex-direction: column; align-items: center;")}>

            <h1>{"Could not find page"}</h1>
            <img src="/img/generic/Birb Wizard Transparent.png" />
            <Link<Route> to={Route::Dashboard}><h5>{"Return to Dashboard"}</h5></Link<Route>>
            <h3>{"404 Error"}</h3>
        </NavBar>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct ErrorPageProps {
    pub error: String,
}

#[styled_component(ErrorPage)]
pub fn error_page(props: &ErrorPageProps) -> Html {    
    html! {
        <NavBar content_class={css!("display: flex; flex-direction: column; align-items: center; margin-top: 50px;")}>
            <ScrollDiv class={css!("display: flex; flex-direction: column; align-items: center; width: 60%; text-align: center; word-wrap: break-word;")}>
                <h1>{"Error!"}<hr/></h1>
                <h3 style="font-size: 1.5em;">{"Recieved Error During Execution:"}</h3>
                <h4>{props.error.clone()}</h4>
            </ScrollDiv>
        </NavBar>
    }
}