use yew::prelude::*;

use crate::app::gui::pages::BasePage;

#[function_component(HomePage)]
pub fn home_page() -> Html
{
    html!
    {
        <BasePage>
            <h1>{"Homepage!"}</h1>
        </BasePage>
    }
}