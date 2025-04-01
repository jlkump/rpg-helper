use yew::prelude::*;
use yew_icons::{Icon, IconId};
use stylist::yew::styled_component;

use crate::gui::contexts::theme::use_theme;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub z_index: i32,
    pub active: UseStateHandle<bool>,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub on_close_callback: Callback<()>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[styled_component(Popup)]
pub fn popup(props: &Props) -> Html {
    let theme = use_theme();
    let style = css!(
        r#"
            display: none;

            &.active {
                margin: 0px;
                position: fixed;
                display: flex;
                justify-content: center;
                align-items: center;
                z-index: ${z_index};

                top: 0%;
                left: 0%;
                width: 100vw;
                height: 100vh;
                background: rgba(57, 54, 54, 0.25);
            }

            .popup {
                position: relative;
                padding: 20px;
                min-width: 160px;
                border: 3px solid ${popup_border};
                background: ${paper};
                box-shadow: 8px 8px 0px ${dropshadow};
                z-index: ${popup_index};
            }

            .close {
                color: ${text_colored};
                cursor: pointer;
                position: absolute;
                top: 0%;
                right: 0%;
                margin: 5px;
            }

            .close:hover {
                color: ${text_highlight};
            }
        "#,
        z_index=props.z_index,
        popup_index=props.z_index,
        popup_border=theme.border_popup,
        paper=theme.paper,
        dropshadow=theme.dropshadow,
        text_colored=theme.text_colored,
        text_highlight=theme.text_colored_highlight,
    );
    let popup_hovered = use_state(|| false);
    let onenter = {
        let popup_hovered = popup_hovered.clone();
        Callback::from(move |_: MouseEvent| { popup_hovered.set(true) })
    };
    let onexit = {
        let popup_hovered = popup_hovered.clone();
        Callback::from(move |_: MouseEvent| { popup_hovered.set(false) })
    };

    let bg_onclick = {
        let active = props.active.clone();
        let callback = props.on_close_callback.clone();
        let popup_hovered = popup_hovered.clone();
        Callback::from(move |_: MouseEvent| { 
            if !*popup_hovered {
                active.set(false); 
                callback.emit(());
            }
        })
    };

    let x_click = {
        let active = props.active.clone();
        let callback = props.on_close_callback.clone();
        Callback::from(move |_: MouseEvent| { 
            active.set(false); 
            callback.emit(());
        })
    };
    let bg_class = classes!(style, if *props.active { "active" } else { "" });
    html! {
        <div class={bg_class} onclick={bg_onclick}>
            <div class={classes!("popup", props.class.clone())} style={props.style.clone()} onmouseenter={onenter} onmouseleave={onexit}>
                <Icon icon_id={IconId::FontAwesomeSolidXmark} class={"close"} onclick={x_click}/>
                {props.children.clone()}
            </div>
        </div>
    }
}