use rpg_helper::api::display::icon::Icon;
use stylist::yew::styled_component;
use yew::prelude::*;

use crate::app::gui::atoms::icon::IconHtml;

#[derive(Properties, Clone, PartialEq)]
pub struct Props
{
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: AttrValue,
    #[prop_or_default]
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub placeholder: AttrValue,
}

#[styled_component(Searchbar)]
pub fn searchbar(props: &Props) -> Html
{
    let css = css!(
        r#"
            display: flex;
            align-items: center;
        "#
    );
    html!
    {
        <div class={classes!(css, "input", props.class.clone())} style={props.style.clone()}>
            <IconHtml icon={Icon::Search} />
            <div style="height: 18px; width: 2px; background-color: var(--text-default-25); margin-left: .25rem; margin-right: .25rem; border-radius: 4px;"></div>
            <input
                type="text"
                class={props.class.clone()}
                style="border: none; background: none;"
                name={props.name.clone()}
                placeholder={props.placeholder.clone()}
                // onchang={props.onchange.clone()}
            />
        </div>
    }
}