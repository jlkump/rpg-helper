use yew::prelude::*;

use crate::app::gui::pages::{editor::editor_bar::EditorBar, BasePage};

#[derive(Clone, PartialEq)]
struct DisplayEditorState
{

}

#[function_component(DisplayEditor)]
pub fn display_editor() -> Html
{
    html!
    {
        <BasePage>
            <h1>{"Display Editor"}</h1>
            // <EditorBar<DisplayEditorState> buttons={vec![]} />
        </BasePage>
    }
}