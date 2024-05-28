use std::sync::Arc;

use stylist::yew::styled_component;
use yew::prelude::*;

use crate::gui::contexts::style::theme::use_theme;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tabs: Vec<Html>, // Expects just text, icons, or imgs, doesn't display large elements well
    pub content: Vec<Html>,
    #[prop_or(false)]
    pub no_hard_border: bool
}

#[styled_component(TabbedPane)]
pub fn tabbed_pane(props: &Props) -> Html {
    if props.content.len() <= 0 {
        return html!();
    }

    let selected_tab = Arc::new(use_state(|| 0 as usize));

    let theme = use_theme();
    let style = css!(
        r#"
            display: flex;
            flex-direction: column;
        "#
    );

    let content_container = if props.no_hard_border {
        css!()
    } else {
        css!(
            r#"
                border-bottom: 2px solid ${border};
                border-right: 2px solid ${border};
                border-left: 2px solid ${border};
            "#,
            border=theme.border_hard
        )
    };

    let mut callbacks = vec![];
    for (i, _) in props.tabs.iter().enumerate() {
        let selected_tab_ref = selected_tab.clone();
        callbacks.push(Callback::from(move |_| {selected_tab_ref.set(i);}));
    }

    html! {
        <div class={style}>
            <TabBar tabs={props.tabs.clone()} selected_tab={**selected_tab} {callbacks} no_hard_border={props.no_hard_border}/>
            <div class={content_container}>
                { props.content.iter().nth(**selected_tab).unwrap().clone() }
            </div>
        </div>
    }
} 

#[derive(Properties, PartialEq)]
struct TabBarProps {
    tabs: Vec<Html>,
    selected_tab: usize,
    callbacks: Vec<Callback<MouseEvent>>,
    no_hard_border: bool
}

#[styled_component(TabBar)]
fn tab(props: &TabBarProps) -> Html {

    let theme = use_theme();

    let style = css!(
        r#"
            -webkit-user-select: none; /* Safari */
            -ms-user-select: none; /* IE 10 and IE 11 */
            user-select: none; /* Standard syntax */

            display: flex;
            flex-direction: row;
            align-items: stretch;
        "#,
    );

    let border_style = if props.no_hard_border {
        format!("1px solid {}", theme.border_light)
    } else {
        format!("2px solid {}", theme.border_hard)
    };
    let tab_unselected = css!(
        r#"
            display: flex;
            justify-content: center;
            align-items: center;
            min-width: 100px;

            background: ${bg};
            border-top: 1px solid ${border};
            border-right: 1px solid ${border};
            border-left: 1px solid ${border};
            border-bottom: ${unselected};
        "#,
        bg=theme.panel_primary,
        border=theme.border_light,
        unselected=border_style
    );

    let tab_selected = css!(
        r#"
            display: flex;
            justify-content: center;
            align-items: center;
            min-width: 100px;

            background: ${bg};
            border-top: ${border};
            border-right: ${border};
            border-left: ${border};
        "#,
        bg=theme.paper,
        border=border_style,
    );

    let filler_style = css!(
        r#"
            border-bottom: ${border};
            margin-top: auto;
            width: 100%;
        "#,
        border=border_style
    );

    let mut tabs = vec![];
    for (i, e) in props.tabs.iter().enumerate() {
        tabs.push(html! {
            if i != props.selected_tab {
                <div class={tab_unselected.clone()} onclick={props.callbacks.iter().nth(i).unwrap()}>
                    { e.clone() }
                </div>
            } else {
                <div class={tab_selected.clone()}>
                    { e.clone() }
                </div>
            }
        });
    }

    html! {
        <div class={style}>
            { for tabs }
            <div class={filler_style} />
        </div>
    }
}