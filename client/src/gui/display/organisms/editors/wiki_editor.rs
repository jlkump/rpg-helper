use std::{cell::RefCell, rc::Rc};

use validator::ValidationErrors;
use yew::{prelude::*, virtual_dom::VNode};
use stylist::yew::styled_component;
use yew_icons::{Icon, IconId};

use crate::{gui::{contexts::{data_context::use_data_context, theme::use_theme}, display::{atoms::{loading::Loader, form_input::FormInput}, molecules::wiki::WikiTreeItem}}, model::data_model::storage::wiki::WikiIndex};

#[derive(Properties, PartialEq, Clone)]
pub struct WikiEditorProps {
    pub wiki_data: Option<Rc<WikiIndex>>,
}

#[styled_component(WikiEditor)]
pub fn wiki_editor(props: &WikiEditorProps) -> Html {
    let theme = use_theme();
    let css = css!(
        r#"
            display: flex;
            height: 100%;

            ul {
                list-style-type: none;
                padding-left: 0.5rem;
                margin-left: 0.5rem;
                border-left: 2px solid ${border};
            }

            .tree {
                display: flex;
                flex-direction: column;
                flex: 30%;

                height: 100%;
            }

            .tree-edit {
                display: flex;
                justify-content: space-around;
                align-items: center;
                margin-left: 5px;
                margin-right: 5px;
            }

            .edit-icon {
                padding: 4px;
                margin-left: 2px;
                margin-right: 2px;
                cursor: pointer;
            }

            .edit-icon:hover {
                background: ${hover};
                border-radius: 2.5px;
            }

            .file-tree {
                border: 2px solid ${border};
                border-radius: 3px;

                margin-right: 4px;
                margin-left: 4px;
                margin-bottom: 4px;
                height: 100%;

                background: ${bg};
            }
            
            .display {
                display: flex;
                flex: 70%;
                margin: 4px;

                border: 2px solid ${border};
                border-radius: 3px;
            }

            @media screen and (max-width: 500px) {
                flex-direction: column;
            }
        "#,
        hover=theme.panel_primary_hover,
        border=theme.border_light,
        bg=theme.panel_primary,
    );

    let selected_page = use_state(|| None);
    let on_selected = {
        let selected_page = selected_page.clone();
        Callback::from(move |p| selected_page.set(Some(p)))
    };
    html! {
        <div class={css}>
            <div class="tree">
                <h5 style="align-self: center;">{"Wiki Pages"}</h5>
                <div class="tree-edit">
                    <FormInput<String> 
                        input_type="search" 
                        name="search" 
                        placeholder="Search..."
                        input_ref={NodeRef::default()}
                        onchange={Callback::from(|_| {})}
                        onblur={Callback::from(|(_, _)| {})}
                        to_type={Callback::from(|_| { String::from("") })}
                        errors={Rc::new(RefCell::new(ValidationErrors::new()))}
                        style="width: 150px;"
                    />
                    <Icon class="edit-icon" height={"1em"} icon_id={IconId::FeatherFilePlus} />
                    <Icon class="edit-icon" height={"1em"} icon_id={IconId::FeatherFolderPlus} />
                    <Icon class="edit-icon" height={"1em"} icon_id={IconId::BootstrapSortAlphaDown} />
                    <Icon class="edit-icon" height={"1em"} icon_id={IconId::BootstrapSortAlphaUp} />
                    <Icon class="edit-icon" height={"1em"} icon_id={IconId::LucideClock4} />
                </div>
                <div class="file-tree">
                    if let Some(data) = &props.wiki_data {
                        {data.get_root_pages().into_iter().map(|d| html! { <WikiTreeItem data={d} onselected={on_selected.clone()}/> }).collect::<Vec<VNode>>()}
                    } else {
                        <div style="width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;">
                            <Loader color={theme.text_colored.clone()} />
                        </div>
                    }
                </div>
            </div>
            <div class="display">
                if let Some(selected) = &*selected_page {

                } else {
                    <div style="width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;">{"Click a page to view and edit"}</div>
                }
            </div>
        </div>
    }
}

