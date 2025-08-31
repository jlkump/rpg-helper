use std::{cell::RefCell, rc::Rc};

use rpg_helper::api::data::tag::Tag;
use yew::prelude::*;

use crate::app::gui::{atoms::input::{equation_input::EquationInput, tag_input::TagInput}, pages::{editor::editor_bar::EditorBar, BasePage}};

#[function_component(RulesetEditor)]
pub fn ruleset_editor() -> Html
{
    let equation_id = Rc::new(RefCell::new(Tag::from_str("test.equation").unwrap()));
    let allowed = Rc::new(RefCell::new(vec![Tag::from_str("lhs.Year").unwrap(), Tag::from_str("rhs.Year").unwrap()]));
    
    
    html!
    {
        <BasePage style="display: flex; align-items: center; flex-direction: column;">
            <h1>{"Ruleset Editor"}
                <hr class="full"/>
            </h1>
            <div style="display: flex; justify-content: space-around;">
                <div style="display: flex; flex-direction: column; align-items: center;">
                    <div style="background-color: var(--paper-25); padding: 10px; margin: 4px; box-shadow: 0px 4px 4px var(--drop-shadow); width: 20vw;">
                        <form>
                            <h3>{"Date"}</h3>
                            <hr class="full"/>
                            <label>{"Ordering Equation"}<i class="fa-solid fa-circle-question"></i></label>
                            <EquationInput allowed_tag_values={allowed} equation_id={equation_id} default_value={"(rhs.Year - lhs.Year) * 365"} placeholder="Ex: (rhs.Year - lhs.Year) * 365"/>
                            <label>{"Required Values"}<i class="fa-solid fa-circle-question"></i></label>
                            <TagInput default_value={"Year"} placeholder="Tag.Value"/>

                        </form>
                    </div>
                </div>
                <div>
                    <div style="background-color: var(--paper-25); padding: 10px; margin: 4px; max-width: 60vw; box-shadow: 0px 4px 4px var(--drop-shadow);">
                        <h1>{"Header 1"}</h1>
                        <hr/>
                        <label>{"Equation"}<i class="fa-solid fa-circle-question"></i></label>
                        // <input class="temp-in" style="color: var(--text-primary); background-color: var(--primary);" type="text" placeholder="Alt Year"/>
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
                </div>
            </div>
            <form>
                <label for="test">{"Test Input"}<i style="margin-left: auto; font-size: 16px;" class="fa-solid fa-circle-question"></i></label>
                // <hr/>
                <input class="" type="text" name="test" placeholder="Tag.Name"/>
            </form>

        </BasePage>
    }
}