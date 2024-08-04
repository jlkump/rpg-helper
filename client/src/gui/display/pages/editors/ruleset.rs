use yew::{platform::spawn_local, prelude::*};
use stylist::yew::styled_component;

use crate::{api::data_api::fetch_ruleset_data, gui::display::{atoms::{loading::SkeletonPane, tooltip::Tooltip}, organisms::{nav_bar::NavBar, searchable_gallery}}, model::{data_model::storage::{intermediate_view::IntermediateView, ruleset::Ruleset}, schema::RulesetRequestSchema, types::RulesetId}};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub ruleset_id: RulesetId,
}

#[styled_component(RulesetCreator)]
pub fn ruleset_creator(_: &Props) -> Html {
    // TODO: Figure out how to do data stuff :P
    let i_view = ViewContext::IntermediateView(IntermediateView::from_ruleset(r));

    let ruleset_data = use_state(|| None);
    use_effect_with((), {
        let ruleset_data = ruleset_data.clone();
        move |_| {
            spawn_local(async move {
    
                let res = fetch_ruleset_data(RulesetRequestSchema { id: uuid::Uuid::new_v4() }).await;
                match res {
                    Ok(d) => {
                        let r: Ruleset = d.into();
                        ruleset_data.set(Some(r))
                    },
                    Err(_e) => {
                        todo!()
                    },
                }
            });
        }
    });

    let wiki_mut = use_mut_ref(|| ruleset_data.unwrap().);
    html! {
        <NavBar>
            // TODO: 
            // [ ]. Create a Skeleton of the Ruleset Creator 
            // [ ]. Implement functionality for the Ruleset Creator
            // [ ]. Implement functionality for the Ruleset Gallery viewer
            <div style="display: flex; flex-direction: column; align-items: center;">
                // Tab 1 - Type Editor
                // Tab 2 - Wiki Editor
                // Tab 3 - Location Editor
                // Tab 4 - Character Template Creator
            </div>
        </NavBar>
    }
}