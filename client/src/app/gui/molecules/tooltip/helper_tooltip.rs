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
                top: 20px;
                left: 20px;
                position: absolute;
                height: 250px;
                width: 250px;
                background-color: var(--text-default);
                max-height: 0;
                max-width: 0;
                transition: 0.2s all ease-out;
                z-index: 10;
            }

            .tooltip.hover
            {
                max-height: 250px;
                max-width: 250px;
            }

            .click-outside
            {
                position: fixed;
                top: 0;
                left: 0;
                width: 100vw;
                height: 100vh;
                background-color: blue;
                opacity: 10%;
                z-index: 9;
            }
        "#
    };

    let tooltip_open = use_state(|| false);
    let onmouseenter = 
    {
        let tooltip_open = tooltip_open.clone();
        Callback::from(move |_|
        {
            tooltip_open.set(true);
        })
    };

    let onmouseleave = 
    {
        let tooltip_open = tooltip_open.clone();
        Callback::from(move |_|
        {
            tooltip_open.set(false);
        })
    };

    let forced_open = use_state(|| false);
    let onclick =
    {
        let forced_open = forced_open.clone();
        Callback::from(move |_|
        {
            log::info!("Forced tooltip open");
            forced_open.set(!*forced_open);
        })
    };

    //TODO: Onclick, set "forced open" and spawn an invisible div beneath the tooltip that fills the screen.
    //      When that div is clicked, set "forced open" false
    html!
    {
        <div {class} {onmouseenter} {onmouseleave} {onclick}>
            {props.children.clone()}
            <div class={if *tooltip_open || *forced_open {"tooltip hover"} else {"tooltip"}}></div>
            if *forced_open
            {
                <div class="click-outside"></div>
            }
            
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