/// This tooltip is a simple one that simply displays some set
/// of helper text when hovered over an html element.
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props
{
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(HelperTooltip)]
pub fn helper_tooltip(props: &Props) -> Html
{
    html!
    {

    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct IconProps
{
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(HelperTooltipIcon)]
pub fn helper_tooltip_icon(props: &IconProps) -> Html
{
    html!
    {

    }
}