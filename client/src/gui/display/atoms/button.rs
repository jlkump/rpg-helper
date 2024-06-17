use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::{contexts::style::theme::use_theme, display::atoms::loading::Loader};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    #[prop_or(false)]
    pub loading: bool,
}

#[styled_component(Button)]
pub fn button(_: &Props) -> Html {
    html! {

    }
}

#[styled_component(SubmitButton)]
pub fn submit_button(props: &Props) -> Html {
    let theme = use_theme();
    html! {
        <button type="submit" style="margin-top: 8px;">
            if props.loading {
                <div style="display: flex; flex-direction: row; align-items: center; justify-content: center;">
                    <Loader color={theme.text_invert.clone()}/ >
                    <span style="margin-left: 4px;">{"Loading..."}</span>
                </div>
            } else {
                {props.children.clone()}
            }
        </button>
    }
}