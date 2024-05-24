use yew::prelude::*;
use stylist::yew::styled_component;

use crate::gui::client::use_theme;
// Character Portrait
// For displaying the character's picture
// The user can define the dimensions of the portrait image
// through the config files, allowing for different portraits depending on
// the game being played
#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub image: String,
    #[prop_or("100px".to_string())]
    pub height: String,
    #[prop_or("100px".to_string())]
    pub width: String,
}

#[styled_component(CharacterPortrait)]
pub fn character_portrait(props: &Props) -> Html {
    let theme = use_theme();
    let style = css!(
        r#"
            border: 3px solid ${border};
            border-radius: 5px;
            margin: 10px;
        "#,
        border = theme.image_border
    );
    html! {
        <img src={props.image.clone()} width={props.width.clone()} height={props.height.clone()} class={style}/>
    }
}