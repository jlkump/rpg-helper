use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::contexts::theme::use_theme;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub loading: bool,
    #[prop_or(false)]
    pub hover: bool,
    #[prop_or("".to_string())]
    pub src: String,
    #[prop_or("3em".to_string())]
    pub width: String,
    #[prop_or("3em".to_string())]
    pub height: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[styled_component(ProfilePortrait)]
pub fn profile_portrait(props: &Props) -> Html {
    let theme = use_theme();
    let style = css!(
        r#"
            border: 4px solid ${border};
            border-radius: 25%;
            width: ${width}; 
            height: ${height};
            box-shadow: 5px 0px 5px ${shadow}, -5px 0px 5px ${shadow};
        "#,
        border = theme.border_colored,
        shadow = theme.hover_dropshadow,
        width = props.width,
        height = props.height
    );

    let hover = if props.hover {
        css!(r#"
            &:hover {
                border: 4px solid ${hover};
            }
        "#, hover = theme.text_colored_highlight)
    } else {
        css!()
    };

    html! {
        if props.loading {
            <div class={classes!(props.class.clone(), hover, style, css!("background-image: linear-gradient(${bg_1}, ${bg_2});", bg_1=theme.skeleton_background_primary.clone(), bg_2=theme.skeleton_background_secondary.clone()))} style={props.style.clone()}></div>
        } else {
            <img class={classes!(props.class.clone(), hover, style)} style={props.style.clone()} src={props.src.clone()} />
        }
    }
}