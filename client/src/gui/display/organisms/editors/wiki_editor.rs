use std::rc::Rc;

use yew::prelude::*;
use stylist::yew::styled_component;
use yew_icons::{Icon, IconId};

use crate::{gui::{contexts::data_context::use_data_context, display::{atoms::loading::Loader, molecules::wiki::WikiTreeItem}}, model::data_model::storage::wiki::WikiIndex};

#[derive(Properties, PartialEq, Clone)]
pub struct WikiEditorProps {
    pub wiki_data: Option<Rc<WikiIndex>>,
}

#[styled_component(WikiEditor)]
pub fn wiki_editor(props: &WikiEditorProps) -> Html {
    let css = css!(
        r#"
            ul {
                list-style-type: none;
            }

            .tree {
            }



            .file {

            }
        "#
    );
    html! {
        <div class={css}>
            if let Some(data) = &props.wiki_data {

            } else {
                <ul style="padding: 0; margin: 0;">
                    <WikiTreeItem item_name={"test"} data={None}/>
                </ul>
            }
        </div>
    }
}

