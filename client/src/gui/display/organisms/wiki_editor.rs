use yew::prelude::*;
use stylist::yew::styled_component;

use crate::{gui::contexts::data_context::use_data_context, model::data_model::storage::wiki::WikiIndex};

#[derive(Properties, PartialEq, Clone)]
pub struct WikiEditorProps {
}

#[styled_component(WikiEditor)]
pub fn wiki_editor(props: &WikiEditorProps) -> Html {
    html! {

    }
}