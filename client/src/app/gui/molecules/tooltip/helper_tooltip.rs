/// This tooltip is a simple one that simply displays some set
/// of helper text when hovered over an html element.
/// 
/// It can be toggled on by clicking on the element.
use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Properties, Clone, PartialEq)]
pub struct Props
{
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: AttrValue,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub tooltip_text: AttrValue,
}

#[styled_component(HelperTooltip)]
pub fn helper_tooltip(props: &Props) -> Html
{
    let class = css!
    {
        r#"
            position: relative;

            .tooltip
            {
                right: 0;
                position: absolute;
                height: 250px;
                width: 250px;
                background-color: var(--text-default);
            }
        "#
    };

    html!
    {
        <div {class} style="position: relative;">
            {props.children.clone()}
            <div class="tooltip"></div>
        </div>
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