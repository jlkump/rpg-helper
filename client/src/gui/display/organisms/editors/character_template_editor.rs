use std::rc::Rc;

use yew::prelude::*;
use stylist::yew::styled_component;

use crate::{gui::contexts::data_context::use_data_context, model::data_model::storage::character::CharacterTemplate};

#[derive(Properties, PartialEq, Clone)]
pub struct CharacterTemplateEditorProps {
    pub character_template_data: UseStateHandle<Option<Rc<Vec<CharacterTemplate>>>>,
}

#[styled_component(CharacterTemplateEditor)]
pub fn character_template_editor(props: &CharacterTemplateEditorProps) -> Html {
    html! {

    }
}