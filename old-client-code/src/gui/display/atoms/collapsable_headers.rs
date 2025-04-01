use yew::prelude::*;
use stylist::yew::styled_component;
use yew_icons::{Icon, IconId};

use crate::gui::contexts::theme::use_theme;

#[derive(Properties, PartialEq, Clone)]
pub struct HeaderProps {
    pub header_type: HeaderType,
    pub children: Html,
    pub content: Html,
    #[prop_or("1em".to_string())]
    pub arrow_size: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[derive(PartialEq, Clone)]
pub enum HeaderType {
    H1, H2, H3, H4, H5, H6,
}

impl HeaderType {
    fn to_html(&self, class: Classes, style: Option<AttrValue>, children: Html) -> Html {
        match &self {
            HeaderType::H1 => html! { <h1 {class} {style}>{children}</h1> },
            HeaderType::H2 => html! { <h2 {class} {style}>{children}</h2> },
            HeaderType::H3 => html! { <h3 {class} {style}>{children}</h3> },
            HeaderType::H4 => html! { <h4 {class} {style}>{children}</h4> },
            HeaderType::H5 => html! { <h5 {class} {style}>{children}</h5> },
            HeaderType::H6 => html! { <h6 {class} {style}>{children}</h6> },
        }
    }
}

#[styled_component(Header)]
pub fn header(HeaderProps { header_type, children, arrow_size, content, class, style }: &HeaderProps) -> Html {
    let collapsed = use_state(|| false);
    let theme = use_theme();
    let root_css = css!(
        r#"

            .arrow {
                top: -5px;
                align-self: center;
                cursor: pointer;
                color: ${arrow_color};
                transition: transform 0.3s;
                transform: translate(0, 15%) rotate(-90deg);
            }

            .arrow:hover {
                color: ${arrow_hover};
            }

            .arrow.open {
                transform: translate(0, 15%) rotate(0deg);
            }
        "#,
        arrow_color=theme.header_collapse_arrow,
        arrow_hover=theme.header_collapse_arrow_hover,
    );
    let onclick = {
        let collapsed = collapsed.clone();
        Callback::from(move |_| collapsed.set(!*collapsed))
    };
    html! {
        <>
            {header_type.to_html(classes!(root_css, class.clone()), style.clone(), html! {
                <>
                if !*collapsed {
                    <Icon width={arrow_size.clone()} height={arrow_size.clone()} class="arrow open" icon_id={IconId::FeatherChevronDown} {onclick} />
                } else {
                    <Icon width={arrow_size.clone()} height={arrow_size.clone()} class="arrow" icon_id={IconId::FeatherChevronDown} {onclick} />
                }
                {children.clone()}
                </>
            })}
            if !*collapsed {
                {content.clone()}
            }
        </>
    }
}