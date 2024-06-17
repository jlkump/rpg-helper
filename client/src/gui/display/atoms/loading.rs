use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::contexts::style::theme::use_theme;

#[derive(Clone, Properties, PartialEq)]
pub struct LoaderProps {
    pub color: AttrValue,
    #[prop_or_default]
    pub style: AttrValue,
    #[prop_or("25px".to_owned().into())]
    pub size: AttrValue,
}

#[styled_component(Loader)]
pub fn loader(props: &LoaderProps) -> Html {
    let style = css!(
        r#"
            .loader {
                width: ${size};
                aspect-ratio: 1;
                border-radius: 50%;
                background: 
                    radial-gradient(farthest-side,${color} 94%,#0000) top/6px 6px no-repeat,
                    conic-gradient(#0000 30%,${color});
                -webkit-mask: radial-gradient(farthest-side,#0000 calc(100% - 6px),#000 0);
                animation: l13 1s infinite linear;
            }
            @keyframes l13{ 
                100%{transform: rotate(1turn)}
            }
        "#,
        color=props.color,
        size=props.size,
    );
    html! {
        <div class={style} style={props.style.clone()}>
            <div class="loader"></div>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct SkeletonPaneProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: AttrValue,
    #[prop_or_default]
    pub outer: Option<AttrValue>,
    #[prop_or_default]
    pub center: Option<AttrValue>,
}

#[styled_component(SkeletonPane)]
pub fn loader(props: &SkeletonPaneProps) -> Html {
    let theme = use_theme();
    let outer = props.outer.clone().unwrap_or(theme.skeleton_background_primary.clone().into());
    let center = props.center.clone().unwrap_or(theme.skeleton_background_secondary.clone().into());
    let style = css!(
        r#"
            background-image: linear-gradient(to right, ${outer}, ${center}, ${outer});
        "#,
        outer = outer,
        center = center
    );
    html! {
        <div class={classes!(props.class.clone(), style)} style={props.style.clone()}></div>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct SkeletonTextAreaProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: AttrValue,
    #[prop_or("1em".to_string())]
    pub font_size: String,
    #[prop_or(6)]
    pub num_lines_text: i32,
}


#[styled_component(SkeletonTextArea)]
pub fn loader(props: &SkeletonTextAreaProps) -> Html {
    let style = css!(
        r#"
            .text {
                height: ${font_size};
                margin: 3px;
            }
        "#,
        font_size=props.font_size,
    );
    let text_lines = vec![
        html! {
            <div style="display: flex;">
                <SkeletonPane class={classes!("text", css!("width: ${width};", width="10%"))}/>
                <SkeletonPane class={classes!("text", css!("width: ${width};", width="40%"))}/>
                <SkeletonPane class={classes!("text", css!("width: ${width};", width="20%"))}/>
                <SkeletonPane class={classes!("text", css!("width: ${width};", width="30%"))}/>
            </div>
        },
        html! {
            <div style="display: flex;">
                <SkeletonPane class={classes!("text", css!("width: ${width};", width="20%"))}/>
                <SkeletonPane class={classes!("text", css!("width: ${width};", width="40%"))}/>
                <SkeletonPane class={classes!("text", css!("width: ${width};", width="40%"))}/>
            </div>
        },
        html! {
            <div style="display: flex;">
                <SkeletonPane class={classes!("text", css!("width: ${width};", width="30%"))}/>
                <SkeletonPane class={classes!("text", css!("width: ${width};", width="70%"))}/>
            </div>
        },
        html! {
            <div style="display: flex;">
                <SkeletonPane class={classes!("text", css!("width: ${width};", width="20%"))}/>
                <SkeletonPane class={classes!("text", css!("width: ${width};", width="30%"))}/>
                <SkeletonPane class={classes!("text", css!("width: ${width};", width="10%"))}/>
                <SkeletonPane class={classes!("text", css!("width: ${width};", width="40%"))}/>
            </div>
        },
        html! {
            <div style="display: flex;">
                <SkeletonPane class={classes!("text", css!("width: ${width};", width="40%"))}/>
                <SkeletonPane class={classes!("text", css!("width: ${width};", width="20%"))}/>
                <SkeletonPane class={classes!("text", css!("width: ${width};", width="40%"))}/>
            </div>
        },
    ];

    let mut content = vec![];
    for i in 0..props.num_lines_text {
        let j = i as usize % text_lines.len();
        content.push(text_lines.iter().nth(j).unwrap().clone());
    }
    html! {
        <div class={classes!(props.class.clone(), style)} style={props.style.clone()}>
            {for content}
        </div>
    }
}