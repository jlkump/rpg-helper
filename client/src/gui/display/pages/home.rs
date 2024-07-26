use yew::prelude::*;
use stylist::yew::styled_component;

use crate::{gui::display::organisms::{image_menu::ImageMenu, nav_bar::NavBar}, model::types::ImageData};

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
        Callback::from(move |s: Option<ImageData>| { image_selected.set(s) })
    };

    let onclick = {
        let active = active.clone();
        Callback::from(move |_| { active.set(true); })
    };
    html! {
        <NavBar>
            <ImageMenu {active} z_index=11 on_image_selected={callback} />
            <div style="display: flex; height: 100%; width: 100%; justify-content: center;">
                <div>
                    <h4>{format!("Image Selected: {:?}", image_selected)}</h4>
                    <button {onclick}>{"Open image menu"}</button>
                </div>
                <div>
                    if let Some(image) = &*image_selected {
                        <img src={image.to_src().to_string()} style="width: 100%; height: 100%; object-fit: contain;" />
                    }
                </div>
            </div>
        </NavBar>
    }
}