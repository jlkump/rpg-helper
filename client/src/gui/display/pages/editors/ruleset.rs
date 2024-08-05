use std::{cell::RefCell, rc::Rc};

use validator::ValidationErrors;
use yew::{platform::spawn_local, prelude::*};
use stylist::yew::styled_component;

use crate::{api::data_api::fetch_ruleset_data, gui::{contexts::theme::use_theme, display::{atoms::{form_input::FormInput, loading::SkeletonPane, tooltip::Tooltip}, molecules::tabbed_pane::TabbedPane, organisms::{editors::{character_template_editor::CharacterTemplateEditor, location_editor::LocationEditor, type_editor::TypeEditor, wiki_editor::WikiEditor}, nav_bar::NavBar, searchable_gallery}}}, model::{data_model::storage::{intermediate_view::IntermediateView, ruleset::Ruleset}, schema::RulesetRequestSchema, types::RulesetId}};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub ruleset_id: RulesetId,
}

#[styled_component(RulesetCreator)]
pub fn ruleset_creator(props: &Props) -> Html {

    let ruleset_data: UseStateHandle<Option<Rc<Ruleset>>> = use_state(|| None);
    let type_data = use_state(|| None);
    let wiki_data = use_state(|| None);
    let location_data = use_state(|| None);
    let character_template_data = use_state(|| None);
    use_effect_with((), {
        let ruleset_data = ruleset_data.clone();
        let id = props.ruleset_id.clone();
        move |_| {
            spawn_local(async move {
    
                let res = fetch_ruleset_data(RulesetRequestSchema { id }).await;
                // match res {
                //     Ok(d) => {
                //         let r: Ruleset = d.into();
                //         ruleset_data.set(Some(Rc::new(r)))
                //     },
                //     Err(_e) => {
                //         todo!()
                //     },
                // }
            });
        }
    });

    let theme = use_theme();
    let css = css!(
        r#"
            display: flex;
            flex-direction: column;
            height: 100%;

            .header {
                text-align: center;
                display: flex;
                justify-content: center;
            }

            h5 {
                margin-top: 15px; 
                margin-bottom: 15px;
                text-align: center;
            }

            textarea {
                width: 100%;
                resize: none;
                background: ${panel_primary};
                height: 20vh;
            }

            .img-wrapper {
                display: flex;
                justify-content: center;
                height: 20vh;
            }

            img {
                max-width: 100%;
                max-height: 100%;
                border: 3px solid ${img_border};
                border-radius: 2px;
            }

            .editor {
                display: flex;
                flex-direction: row;
                height: 90%;
            }

            .edit-header {
                position: absolute;
                display: flex;
                justify-content: center;
                top: -40px;
            }

            .ruleset-editor {
                position: relative;
                margin: 20px;
                flex: 30%;
                display: flex;
                flex-direction: column;
                align-items: center;
            }

            .ruleset-option {
                display: flex; 
                flex-direction: column;
                justify-content: space-around;
                height: 100%;
                margin: 3px;
            }

            .tabbed-editor {
                flex: 70%;
                height: 95%;
                margin: 20px;
            }

            @media screen and (max-width: 1000px) {
                .editor {
                    flex-direction: column;
                }

                .edit-header {
                    position: relative;
                    top: 0;
                }

                .tabbed-editor {
                    flex: none;
                    height: 80vh;
                    margin: 10px;
                }
            }

            @media screen and (max-width: 500px) {
                h5 {
                    font-size: 0.75em;
                }
            }
        "#,
        panel_primary=theme.panel_primary,
        img_border=theme.border_colored,
    );

    html! {
        <NavBar>
            // TODO: 
            // [ ]. Create a Skeleton of the Ruleset Creator 
            // [ ]. Implement functionality for the Ruleset Creator
            // [ ]. Implement functionality for the Ruleset Gallery viewer
            // Tab 1 - Type Editor
            // Tab 2 - Wiki Editor
            // Tab 3 - Location Editor
            // Tab 4 - Character Template Creator
            <div class={css}>
                <div class="header">
                    <h2>{"Ruleset Editor"}<hr/></h2>
                </div>
                <div class="editor">
                    <div class="ruleset-editor">
                        <div class="edit-header"><h3>{"General"}<hr/></h3></div>
                        <div style="display: flex; justify-content: space-evenly; width: 100%; align-items: center;">
                            <div class="ruleset-option">
                                <div class="img-wrapper">
                                    <img src="/img/generic/ars-magica-logo-icon.png" />
                                </div>
                                <button>{"Edit Image"}</button>
                            </div>
                            <div class="ruleset-option">
                                <FormInput<String>
                                    input_type="text"
                                    name="ruleset-name"
                                    placeholder="Ruleset Name"
                                    input_ref={NodeRef::default()}
                                    onchange={Callback::from(|_| {})}
                                    onblur={Callback::from(|_| {})}
                                    to_type={Callback::from(|s| {s})}
                                    errors={Rc::new(RefCell::new(ValidationErrors::new()))}
                                    style="width: 150px;"
                                />
                                <FormInput<String>
                                    input_type="text"
                                    name="ruleset-tags"
                                    placeholder="Ruleset Tags"
                                    input_ref={NodeRef::default()}
                                    onchange={Callback::from(|_| {})}
                                    onblur={Callback::from(|_| {})}
                                    to_type={Callback::from(|s| {s})}
                                    errors={Rc::new(RefCell::new(ValidationErrors::new()))}
                                    style="width: 150px;"
                                />
                            </div>
                        </div>
                        <textarea placeholder="Description ..."></textarea>
                        <button>{"Save Changes"}</button>
                    </div>
                    <div class="tabbed-editor">
                        <TabbedPane 
                            tabs={vec![
                                html! { <h5>{"Type Editor"}</h5> },
                                html! { <h5>{"Wiki Editor"}</h5> },
                                html! { <h5>{"Location Editor"}</h5> },
                                html! { <h5>{"Character Templates"}</h5> },
                            ]}
                            content={vec![
                                html! { <TypeEditor {type_data}/> },
                                html! { <WikiEditor wiki_data={(*wiki_data).clone()}/> },
                                html! { <LocationEditor {location_data} /> },
                                html! { <CharacterTemplateEditor {character_template_data} />},
                            ]}
                        />
                    </div>
                </div>
            </div>
        </NavBar>
    }
}