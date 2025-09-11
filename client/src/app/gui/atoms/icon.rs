use rpg_helper::api::display::icon::Icon;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props
{
    pub icon: Icon,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: AttrValue,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(IconHtml)]
pub fn icon_html(props: &Props) -> Html
{
    let i_class = icon_to_class_str(&props.icon);

    let additional = if props.disabled
    {
        "disabled icon"
    }
    else
    {
        "icon"
    };

    html!
    {
        <span class={classes!(additional, props.class.clone())} style={props.style.clone()} onclick={if props.disabled { None } else { props.onclick.clone() }}>
            <i class={i_class}></i>
        </span>
    }
}

fn icon_to_class_str(icon: &Icon) -> &'static str
{
    match icon
    {
        Icon::Delete => "fa-regular fa-trash-can",
        Icon::Add => "fa-regular fa-square-plus",
        Icon::Edit => "fa-solid fa-pen-to-square",
        Icon::Help => "fa-solid fa-circle-question",
        Icon::Search => "fa-solid fa-magnifying-glass",
        Icon::Clear => "fa-solid fa-xmark",
        Icon::Reset => "fa-solid fa-arrow-rotate-left",
    }
}