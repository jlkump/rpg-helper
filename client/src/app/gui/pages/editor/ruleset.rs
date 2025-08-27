use yew::prelude::*;

use crate::app::gui::pages::{editor::editor_bar::EditorBar, BasePage};

#[function_component(RulesetEditor)]
pub fn ruleset_editor() -> Html
{
    html!
    {
        <BasePage style="display: flex; align-items: center; flex-direction: column;">
            <h1>{"Ruleset Editor"}</h1>
            <form>
                <label for="test">{"Test Input"}<i style="margin-left: auto; font-size: 16px;" class="fa-solid fa-circle-question"></i></label>
                // <hr/>
                <input class="" type="text" name="test" placeholder="Tag.Name"/>
            </form>
            <div style="background-color: var(--paper-25); padding: 10px; margin: 4px; box-shadow: 0px 4px 4px var(--drop-shadow);">
                <form>
                    <h3>{"Date"}</h3>
                    <hr/>
                    <label for="test">{"Ordering Equation"}<i class="fa-solid fa-circle-question"></i></label>
                    <input class="" type="text" name="test" placeholder="(rhs.Year - lhs.Year) * 365"/>
                    <label for="test">{"Required Values"}<i class="fa-solid fa-circle-question"></i></label>
                    <input class="" type="text" name="test" placeholder="Year"/>
                </form>
            </div>
        
            <div style="background-color: var(--paper-25); padding: 10px; margin: 4px; max-width: 60vw; box-shadow: 0px 4px 4px var(--drop-shadow);">
                <h1>{"Header 1"}</h1>
                <hr/>
                <p>
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                </p>
                <h2>{"Header 2"}</h2>
                <hr/>
                <p>
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                </p>
                <h3>{"Header 3"}</h3>
                <hr/>
                <p>
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                </p>
            </div>
        </BasePage>
    }
}