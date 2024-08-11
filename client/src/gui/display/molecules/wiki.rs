use std::{cell::RefCell, rc::Rc};

use yew::prelude::*;
use stylist::yew::styled_component;
use yew_icons::{Icon, IconId};

use crate::{gui::{contexts::{data_context::use_data_context, theme::use_theme}, display::atoms::loading::Loader}, model::data_model::{primatives::wiki::{WikiData, WikiFolder, WikiPage}, storage::wiki::WikiIndex}};

#[derive(Properties, PartialEq, Clone)]
pub struct WikiTreeItemProps {
    pub data: Option<Rc<RefCell<WikiData>>>,
    pub onselected: Callback<Rc<RefCell<WikiData>>>,
}

#[styled_component(WikiTreeItem)]
pub fn wiki_tree_item(props: &WikiTreeItemProps) -> Html {
    let theme = use_theme();

    let css = css!(
        r#"
            position: relative;

            & > div {
                position: relative;
                display: flex;
                padding-top: 2.5px;
                padding-left: 0.4rem;
                padding-bottom: 2.5px;
            }

            .item-name {
                position: relative;
                padding: 4px;
                margin-left: 0.5em;

                border-radius: 2.5px;

                width: 80%;
                cursor: pointer;
            }

            .item-name > p {
                margin: 0;
                margin-left: 1.5em;
            }

            .item-name:hover {
                background: ${hover};
            }

            .edit-options {
                display: grid;
                grid-template-columns: auto auto auto auto;
                margin-right: 2px;
                margin-left: auto;
                transition: opacity 0.25s;
                opacity: 0%;
            }

            .edit-options.file {
                grid-template-columns: auto auto;
            }

            & > div:hover .edit-options {
                opacity: 100%;
            }

            .arrow {
                position: absolute;
                left: 0;
                top: 2px;
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

            .icon {
                position: absolute;
                top: 4px;

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
        match &*data.as_ref().borrow() {
            WikiData::Page(p) => {
                let file_onclick = {
                    let callback = props.onselected.clone();
                    let data = data.clone();
                    Callback::from(move |_| { 
                        let data = data.clone();
                        callback.emit(data) 
                    })
                };
                let trash_onclick = {
                    Callback::from(|_| {
                        // Use WikiRef to delete or edit
                    })
                };
                let edit_onclick = {
                    Callback::from(|_| {})
                };
                html! {
                    <li class={css}>
                        <div>
                            <div class="item-name" onclick={file_onclick}>
                                <Icon class="icon" height={"1em"} icon_id={IconId::FeatherFile} />
                                <p>{p.get_name().to_string()}</p>
                            </div>
                            <div class="edit-options file">
                                <Icon class="edit-icon" height={"1em"} icon_id={IconId::FeatherEdit3} onclick={edit_onclick}/>
                                <Icon class="edit-icon" height={"1em"} icon_id={IconId::FeatherTrash2} onclick={trash_onclick}/>
                            </div>
                        </div>
                    </li>
                }
            },
            WikiData::Folder(f) => {
                let file_new_onclick = {
                    Callback::from(|_| {})
                };
                let folder_new_onclick = {
                    Callback::from(|_| {})
                };
                let trash_onclick = {
                    Callback::from(|_| {})
                };
                let edit_onclick = {
                    Callback::from(|_| {})
                };

                let child_display = if *open {
                    f.get_children().iter().map(|wiki_node| { html! {<WikiTreeItem data={Some(wiki_node.clone())} onselected={props.onselected.clone()}/>}}).collect()
                } else {
                    html! {}
                };
                html! {
                    <li class={css}>
                        <div>
    
                            <Icon width={"1em"} height={"1em"} class={ if *open { "arrow open" } else { "arrow" } } icon_id={IconId::FeatherChevronDown} />
                            <div class="item-name" {onclick} >
                                <Icon class="icon" height={"1em"} icon_id={IconId::FeatherFolder} />
                                <p>{f.get_name().to_string()}</p>
                            </div>
                            <div class="edit-options">
                                <Icon class="edit-icon" height={"1em"} icon_id={IconId::FeatherFilePlus} onclick={file_new_onclick}/>
                                <Icon class="edit-icon" height={"1em"} icon_id={IconId::FeatherFolderPlus} onclick={folder_new_onclick}/>
                                <Icon class="edit-icon" height={"1em"} icon_id={IconId::FeatherEdit3} onclick={edit_onclick}/>
                                <Icon class="edit-icon" height={"1em"} icon_id={IconId::FeatherTrash2} onclick={trash_onclick}/>
                            </div>
                        </div>
                        {child_display}
                    </li>
                }
            },
        }
    } else {
        html! {
            <li><div style="display: flex;"><Loader color={theme.border_light.clone()}/>{"Loading..."}</div></li>
        }
    }
}


#[derive(Properties, PartialEq, Clone)]
pub struct WikiPageDisplayProps {
    pub data: Rc<RefCell<WikiData>>,
    pub onselected: Callback<Rc<RefCell<WikiData>>>,
    pub edit_option: bool,
}

#[styled_component(WikiPageDisplay)]
pub fn wiki_display_page(props: &WikiPageDisplayProps) -> Html {
    html! {

    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct WikiPageEditProps {
    pub data: Rc<RefCell<WikiData>>,
    pub onselected: Callback<Rc<RefCell<WikiData>>>,
}

#[styled_component(WikiPageEdit)]
pub fn wiki_edit_page(props: &WikiPageEditProps) -> Html {
    html! {
        
    }
}