use yew::prelude::*;

use crate::app::gui::pages::{editor::editor_bar::EditorBar, BasePage};

#[function_component(RulesetEditor)]
pub fn ruleset_editor() -> Html
{
    html!
    {
        <BasePage>
            <div class="fullpage-container" style="display: flex; justify-content: center;">
                <h1>{"Ruleset Editor"}</h1>
            </div>
        </BasePage>
    }
}