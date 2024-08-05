use std::rc::Rc;

use yew::prelude::*;
use stylist::yew::styled_component;
use yew_icons::{Icon, IconId};

use crate::{gui::{contexts::{data_context::use_data_context, theme::use_theme}, display::atoms::loading::Loader}, model::data_model::{primatives::wiki::{WikiData, WikiFolder, WikiPage}, storage::wiki::WikiIndex}};

#[derive(Properties, PartialEq, Clone)]
pub struct WikiTreeItemProps {
    pub item_name: String,
    // Things are gonna have to be Rc<RefCell> :(
    pub data: Option<Rc<WikiData>>,
    // pub onselected: Callback<WikiPage>,
}

#[styled_component(WikiTreeItem)]
pub fn wiki_tree_item(props: &WikiTreeItemProps) -> Html {
    let theme = use_theme();
    let css = css!(
        r#"
            & > div {
                padding-top: 2.5px;
                padding-left: 2.4rem;
                padding-bottom: 2.5px;
                cursor: pointer;
            }

            & > div:hover {
                background: ${hover};
            }

            ul:before {
                border-left: 1px solid black;
            }

            .arrow {
                top: -5px;
                align-self: center;
                color: ${arrow_color};
                transition: transform 0.3s;
                transform: translate(0, 15%) rotate(-90deg);
            }

            .arrow:hover {
                color: ${arrow_hover};
            }

            .arrow.open {
                transform: translate(0, 15%) rotate(0deg);
            }

            .folder-icon {
                justify-self: center;
                align-self: center;
            }
        "#,
        hover=theme.panel_primary_hover,
        arrow_color=theme.border_hard,
        arrow_hover=theme.border_colored,
    );
    let open = use_state(|| false);
    let onclick = {
        let open = open.clone();
        Callback::from(move |_| open.set(!*open))
    };

    if let Some(data) = &props.data {
        match data.as_ref() {
            WikiData::Page(p) => {
                html! {

                }
            },
            WikiData::Folder(f) => {

                html! {
                    <li class={css}>
                        <Icon width={"1em"} height={"1em"} class={ if *open { "arrow open" } else { "arrow" } } icon_id={IconId::FeatherChevronDown} {onclick} />
                        <div style="display: flex;">
                            <Icon class="folder-icon" height={"1em"} class="folder-icon" icon_id={IconId::FeatherFolder} />
                            {props.item_name.clone()}
                        </div>
                        if *open {
                            <ul>
                                // TODO: Map wiki data to folder or file depending on contained wiki data.
                                <li>{"Testing"}</li>
                                <li>{"Testing"}</li>
                                <li>{"Testing"}</li>
                            </ul>
                        }
                    </li>
                }
            },
        }
    } else {
        html! {
            <li class={css}>
                <div style="display: flex;" {onclick}>
                    <Icon width={"1em"} height={"1em"} class={ if *open { "arrow open" } else { "arrow" } } icon_id={IconId::FeatherChevronDown} />
                    <Icon class="folder-icon" height={"1em"} class="folder-icon" icon_id={IconId::FeatherFolder} />
                    {props.item_name.clone()}
                </div>
                if *open {
                    <ul>
                        // TODO: Map wiki data to folder or file depending on contained wiki data.
                        <li>{"Testing"}</li>
                        <li>{"Testing"}</li>
                        <li>{"Testing"}</li>
                    </ul>
                }
            </li>
        }
    }
}