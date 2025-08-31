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
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(IconHtml)]
pub fn icon_html(props: &Props) -> Html
{
    let i_class = icon_to_class_str(&props.icon);

    html!
    {
        if props.disabled
        {
            <i class={classes!(i_class, props.class.clone(), "disabled")} style={props.style.clone()}></i>
        }
        else
        {
            <i class={classes!(i_class, props.class.clone())} style={props.style.clone()} onclick={props.onclick.clone()}></i>
        }
    }
}

fn icon_to_class_str(icon: &Icon) -> &'static str
{
    match icon
    {
        Icon::Delete => "fa-regular fa-trash-can",
        Icon::Add => "fa-regular fa-square-plus",
        Icon::Edit => "fa-solid fa-pen-to-square",
    }
}