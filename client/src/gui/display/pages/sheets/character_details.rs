use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::{contexts::theme::use_theme, display::atoms::{colored_panel::ColoredPanel, panel::Panel, tooltip::Tooltip}};

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    // layout: Layout, // TODO: Define character details through layouts instead of hard-coding the layout
}

// This is what is displayed on the left of the character viewer page
// It will show all pertinant important info about a character
#[styled_component(CharacterDetails)]
pub fn character_details(props: &Props) -> Html {
    let theme = use_theme();

    let side_panel_style = css!(
        r#"
            margin: 5px;

            display: flex;
            flex-direction: column;

            justify-content: center;
            align-items: stretch;

            border: 1px solid ${panelborder};
        "#,
        panelborder=theme.border_light
    );

    let header_panel_style = css!(
        r#"
            display: flex;
            align-content: center;
            justify-content: center;
            background: ${bgdark};
        "#,
        bgdark = theme.paper_dark
    );

    let panel_style = css!(
        r#"
            display: flex;
            align-content: center;
            justify-content: center;
            background: ${bgdark};
            margin: 1px;
        "#,
        bgdark = theme.paper_dark
    );

    html! {
        <div class={side_panel_style}>
            <div class={header_panel_style}>
                <Tooltip tooltip_content={html! { <CharacterDetails/>}} simple=true>
                    <h5 style="margin: 3px">{"Antonio Tremis"}</h5>
                </Tooltip>
            </div>
            <div style="display: flex; align-items: center; justify-content: center;">
                <img src="./img/default/Antonio Tremis - AI Portrait.png" width=128px height=128px/>
            </div>
            <ColoredPanel>
                {"Stats"}
            </ColoredPanel>
            <div style="display: grid">
                <Panel style="grid-column: 1; grid-row: 1;">
                    {"Size"}
                </Panel>
                <Panel style="grid-column: 2; grid-row: 1;">
                    {"0"}
                </Panel>
                <Panel style="grid-column: 1; grid-row: 2;">
                    {"Soak"}
                </Panel>
                <Panel style="grid-column: 2; grid-row: 2;">
                    {"-1"}
                </Panel>
            </div>
            <ColoredPanel>
                {"Characteristics"}
            </ColoredPanel>
            <div style="display: grid">
                <div class={panel_style.clone()} style="grid-column: 1; grid-row: 1;">
                    {"Intelligence"}
                </div>
                <div class={panel_style.clone()} style="grid-column: 2; grid-row: 1;">
                    {"4"}
                </div>
                <div class={panel_style.clone()} style="grid-column: 1; grid-row: 2;">
                    {"Perception"}
                </div>
                <div class={panel_style.clone()} style="grid-column: 2; grid-row: 2;">
                    {"-1"}
                </div>
                <div class={panel_style.clone()} style="grid-column: 1; grid-row: 3;">
                    {"Presence"}
                </div>
                <div class={panel_style.clone()} style="grid-column: 2; grid-row: 3;">
                    {"-1"}
                </div>
                <div class={panel_style.clone()}>
                    {"..."}
                </div>
                <div class={panel_style.clone()}>
                    {"..."}
                </div>
            </div>
        </div>
    }
}