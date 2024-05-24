use stylist::yew::styled_component;
use yew::{classes, html, Classes, Html, Properties};

use crate::gui::client::use_theme;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub secondary_coloring: bool,
}

#[styled_component(ColoredPanel)]
pub fn colored_panel(props: &Props) -> Html {
    let theme = use_theme();

    let style = if props.secondary_coloring {
        css!(
            r#"
                display: flex;
                align-content: center;
                justify-content: center;
                background: ${bgcolor};
                color: ${textcolor};
                margin-top: 3px;
                margin-bottom: 3px;
            "#,
            bgcolor=theme.panel_color_secondary,
            textcolor=theme.text_invert
        )
    } else {
        css!(
            r#"
                display: flex;
                align-content: center;
                justify-content: center;
                background: ${bgcolor};
                color: ${textcolor};
                margin-top: 3px;
                margin-bottom: 3px;
            "#,
            bgcolor=theme.panel_color_primary,
            textcolor=theme.text_invert
        )
    };

    html! {
        if let Some(s) = &props.style {
            <div class={classes!(style, props.class.clone())} style={s.clone()}>
                { props.children.clone() }
            </div>
        } else {
            <div class={classes!(style, props.class.clone())}>
                { props.children.clone() }
            </div>
        }
    }
}