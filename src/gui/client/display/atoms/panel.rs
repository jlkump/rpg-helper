// Various single panel displays

use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::client::use_theme;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub transparent: bool,
    #[prop_or_default]
    pub secondary_coloring: bool,
}

#[styled_component(Panel)]
pub fn panel(props: &Props) -> Html {
    let theme = use_theme();
    let style = if props.transparent {
        css!(
            r#"
                display: flex;
                align-content: center;
                justify-content: center;
            "#
        )
    } else {
        if props.secondary_coloring {
            css!(
                r#"
                    display: flex;
                    align-content: center;
                    justify-content: center;
                    background: ${bgcolor};
                "#,
                bgcolor=theme.panel_secondary
            )
        } else {
            css!(
                r#"
                    display: flex;
                    align-content: center;
                    justify-content: center;
                    background: ${bgcolor};
                "#,
                bgcolor=theme.panel_primary
            )
        }
    };

    html! {
        if let Some(s) = &props.style {
            <div class={classes!(props.class.clone(), style)} style={s.clone()}>
                { props.children.clone() }
            </div>
        } else {
            <div class={classes!(props.class.clone(), style)}>
                { props.children.clone() }
            </div>
        }
    }
}


// The following should be in molecules and not in atoms
// ValuePanel
// Displays any given inst as a numeric value. Ignores the name

// InstBriefPanel
// Displays the name followed by the value of a given inst,
// Ex:     Magic Theory: 2
// Perhaps shows a tool-tip panel of the sources for the value when the number is hovered?