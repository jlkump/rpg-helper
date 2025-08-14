use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::contexts::theme::use_theme;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[styled_component(ScrollDiv)]
pub fn scroll_div(props: &Props) -> Html {
    let theme = use_theme();
    let scroll_style = css!(
        r#"
            position: relative;
            border-top: 3px solid ${border};
            border-bottom: 3px solid ${border};
            padding: 8px;
            box-shadow: 10px 0px 10px ${shadow}, -10px 0px 10px ${shadow};
            margin-top: 10px;
            margin-bottom: 10px;

            .top_right {
                position: absolute;
                top: -10.5px;
                right: 0;
                width: 0px;
                height: 0px;
                border-style: solid;
                border-width: 8px 0 0 10px;
                border-color: transparent transparent transparent ${border};
            }

            .top_left {
                position: absolute;
                top: -10.5px;
                left: 0;
                width: 0px;
                height: 0px;
                border-style: solid;
                border-width: 0 0 8px 10px;
                border-color: transparent transparent ${border} transparent;
            }

            .bottom_left {
                position: absolute;
                bottom: -10.5px;
                left: 0;
                width: 0px;
                height: 0px;
                border-style: solid;
                border-width: 0 10px 8px 0;
                border-color: transparent ${border} transparent transparent;
            }

            .bottom_right {
                position: absolute;
                bottom: -10.5px;
                right: 0;
                width: 0px;
                height: 0px;
                border-style: solid;
                border-width: 8px 10px 0 0;
                border-color: ${border} transparent transparent transparent;
            }
        "#,
        border = theme.decorative_scroll_border,
        shadow = theme.decorative_scroll_drop_shadow
    );
    html! {
        <div class={classes!(scroll_style, props.class.clone())} style={props.style.clone()}>
            <div class="top_right"></div>
            <div class="top_left"></div>
            <div class="bottom_left"></div>
            <div class="bottom_right"></div>
            {props.children.clone()}
        </div>
    }
}