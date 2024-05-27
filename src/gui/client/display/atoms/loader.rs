
use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::client::use_theme;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub color: AttrValue,
    #[prop_or_default]
    pub style: AttrValue,
    #[prop_or("25px".to_owned().into())]
    pub size: AttrValue,
}

#[styled_component(Loader)]
pub fn loader(props: &Props) -> Html {
    let style = css!(
        r#"
        /* HTML: <div class="loader"></div> */
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