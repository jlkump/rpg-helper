use yew::prelude::*;
use stylist::yew::styled_component;

use crate::{gui::display::{atoms::collapsable_headers::{Header, HeaderType}, organisms::{image_menu::ImageMenu, nav_bar::NavBar}}, model::types::ImageData};

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
                    <Header header_type={HeaderType::H1} content={
                        html! {
                            <div>
                                if let Some(image) = &*image_selected {
                                    <img src={image.to_src().to_string()} style="width: 100%; height: 100%; object-fit: contain;" />
                                }
                                <button {onclick}>{"Open image menu"}</button>
                            </div>
                        }
                    }>{format!("Image Selected: {:?}", image_selected)}<hr/></Header>

                    <Header header_type={HeaderType::H2} content={html!{{"Test Content"}}}>{"Header 2"}</Header>
                    <Header header_type={HeaderType::H3} content={html!{{"Test Content"}}}>{"Header 3"}</Header>
                    <Header header_type={HeaderType::H4} content={html!{{"Test Content"}}}>{"Header 4"}</Header>
                    <Header header_type={HeaderType::H5} content={html!{{"Test Content"}}}>{"Header 5"}</Header>
                    <Header header_type={HeaderType::H6} content={html!{{"Test Content"}}}>{"Header 6"}</Header>
                </div>
            </div>
        </NavBar>
    }
}