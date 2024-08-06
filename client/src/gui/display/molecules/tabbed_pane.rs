use std::sync::Arc;

use stylist::yew::styled_component;
use yew::prelude::*;

use crate::gui::contexts::theme::use_theme;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tabs: Vec<Html>, // Expects just text, icons, or imgs, doesn't display large elements well
    pub content: Vec<Html>,
}

#[styled_component(TabbedPane)]
pub fn tabbed_pane(props: &Props) -> Html {
    if props.content.len() <= 0 {
        return html!();
    }

    let selected_tab = use_state(|| 0 as usize);

    let theme = use_theme();
    let style = css!(
        r#"
            display: flex;
            flex-direction: column;
            height: 100%;

            border: 1px solid ${border};
        "#,
        border=theme.border_light,

    );

    let mut callbacks = vec![];
    for (i, _) in props.tabs.iter().enumerate() {
        let selected_tab_ref = selected_tab.clone();
        callbacks.push(Callback::from(move |_| {selected_tab_ref.set(i);}));
    }

    html! {
        <div class={style}>
            <TabBar tabs={props.tabs.clone()} selected_tab={*selected_tab} {callbacks} />
            { props.content.iter().nth(*selected_tab).unwrap().clone() }
        </div>
    }
} 

#[derive(Properties, PartialEq)]
struct TabBarProps {
    tabs: Vec<Html>,
    selected_tab: usize,
    callbacks: Vec<Callback<MouseEvent>>,
}

#[styled_component(TabBar)]
fn tab(props: &TabBarProps) -> Html {

    let theme = use_theme();
    let grid = props.tabs.iter().map(|_| "auto ").fold(String::new(), |mut acc, x| {acc.push_str(x); acc});
    let style = css!(
        r#"
            display: grid;
            grid-template-columns: ${grid};
            gap: 1px;
            background: ${border};

            .tab {
                display: flex;
                justify-content: center;
                align-items: center;
                min-width: 100px;
                width: 100%;
                text-align: justify;
            }

            .tab.unselected {
                background: ${bg_unselected};
                cursor: pointer;
                border-bottom: 1px solid ${border};
            }

            .tab.unselected:hover {
                background: ${hover};
            }

            .tab.selected {
                background: ${bg_selected};
            }

            .filler {
                border-bottom: 1px solid ${border};
                background: ${bg_unselected};
                width: 100%;
            }
            
            @media screen and (max-width: 800px) {
                .tab {
                    min-width: auto;
                }
            }
        "#,
        grid=grid,
        bg_unselected=theme.panel_primary,
        bg_selected=theme.paper,
        hover=theme.panel_primary_hover,
        border=theme.border_light,
    );

    let mut tabs = vec![];
    for (i, e) in props.tabs.iter().enumerate() {
        tabs.push(html! {
            if i != props.selected_tab {
                <div class={"tab unselected"} onclick={props.callbacks.iter().nth(i).unwrap()}>
                    { e.clone() }
                </div>
            } else {
                <div class={"tab selected"}>
                    { e.clone() }
                </div>
            }
        });
    }

    html! {
        <div class={style}>
            { for tabs }
            // <div class={"filler"} />
        </div>
    }
}