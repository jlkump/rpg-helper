use std::rc::Rc;

use yew::prelude::*;
use stylist::yew::styled_component;

use crate::{gui::contexts::data_context::use_data_context, model::data_model::storage::types::TypeIndex};

#[derive(Properties, PartialEq, Clone)]
pub struct TypeEditorProps {
    pub type_data: UseStateHandle<Option<Rc<TypeIndex>>>,
}

#[styled_component(TypeEditor)]
pub fn type_editor(props: &TypeEditorProps) -> Html {
    html! {

    }
}