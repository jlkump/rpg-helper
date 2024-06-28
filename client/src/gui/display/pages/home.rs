use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::display::organisms::{image_menu::ImageMenu, nav_bar::NavBar};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
}

#[styled_component(Home)]
pub fn home(_: &Props) -> Html {
    // Display changes based on whether logged-in or not
    let active = use_state(|| true);
    let image_selected = use_state(|| None);
    let callback = {
        let image_selected = image_selected.clone();
        Callback::from(move |s: String| { image_selected.set(Some(s)) })
    };
    html! {
        <NavBar>
            <div style="display: flex; height: 100%; width: 100%; justify-content: center;">
                <h1>{"Welcome!"}</h1>
                <h4>{format!("Image Selected: {:?}", image_selected)}</h4>
                <ImageMenu {active} z_index=11 on_image_selected={callback} />
            </div>
        </NavBar>
    }
}