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
            <div class="fullpage-container" style="display: flex; justify-content: center;">
                <EditorBar<DisplayEditorState> buttons={vec![]} />
            </div>
        </BasePage>
    }
}