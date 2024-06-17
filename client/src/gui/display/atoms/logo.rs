use yew::prelude::*;
use stylist::yew::styled_component;
use yew_router::components::Link;

use crate::{router::Route, gui::contexts::style::theme::use_theme};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or(LogoSize::Standard)]
    pub size: LogoSize,
    #[prop_or(true)]
    pub text: bool,
}

#[derive(Clone, PartialEq)]
pub enum LogoSize {
    Small,
    Standard,
    Large,
    Huge,
}

#[styled_component(Logo)]
pub fn logo(props: &Props) -> Html {
    let theme = use_theme();
    let logo_style = css!(
        r#"
            border-radius: 20px;
            padding: 10px;

            -webkit-user-select: none; /* Safari */
            -ms-user-select: none; /* IE 10 and IE 11 */
            user-select: none; /* Standard syntax */

            cursor: pointer;
            a {
                color: ${logo};
                text-decoration: none;
            }
        "#,
        logo=theme.logo
    );

    let size = match props.size {
        LogoSize::Small => 1.0,
        LogoSize::Standard => 2.5,
        LogoSize::Large => 4.0,
        LogoSize::Huge => 8.0,
    };

    let image_style = css!(
        r#"
            width: ${size}em;
            height: ${size}em;
        "#,
        size=size
    );

    let text_style = css!(
        r#"
            font-size: ${size}em;
        "#,
        size={size - 1.0}
    );

    html! {
        <div class={logo_style}>
            <Link<Route> to={Route::Home} classes={css!("display: flex; flex-direction: row; align-items: center;")}><img src="/img/generic/Dice RPG Icon.svg" class={image_style}/>
                if props.text {
                    <h3 class={text_style}>{"RPG Helper"}</h3>
                }
            </Link<Route>>
        </div>
    }
}