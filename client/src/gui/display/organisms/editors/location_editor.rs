use std::rc::Rc;

use yew::prelude::*;
use stylist::yew::styled_component;

use crate::{gui::contexts::data_context::use_data_context, model::data_model::storage::location::LocationIndex};

#[derive(Properties, PartialEq, Clone)]
pub struct LocationEditorProps {
    pub location_data: UseStateHandle<Option<Rc<LocationIndex>>>,
}

#[styled_component(LocationEditor)]
pub fn location_editor(props: &LocationEditorProps) -> Html {
    html! {

    }
}