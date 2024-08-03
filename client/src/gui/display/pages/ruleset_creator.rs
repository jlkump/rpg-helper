use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::display::{atoms::{loading::SkeletonPane, tooltip::Tooltip}, organisms::{nav_bar::NavBar, searchable_gallery}};

#[derive(Properties, PartialEq)]
pub struct Props {

}

#[styled_component(RulesetCreator)]
pub fn ruleset_creator(_: &Props) -> Html {
    let create_new_clicked = use_state(|| false);
    let create_clicked = {
        let create_new_clicked = create_new_clicked.clone();
        Callback::from(move |_| {create_new_clicked.set(true)})
    };
    html! {
        <NavBar>
            <div style="display: flex; flex-direction: column; align-items: center;">
                <h2 style="text-align: center; position: relative;">
                    {"Rulesets"}
                    if *create_new_clicked {
                        {"Create clicked!"}
                    }
                <hr/></h2>
                {searchable_gallery::test_gallery(Some(create_clicked))}

            </div>
        </NavBar>
    }
}