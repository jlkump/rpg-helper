use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Properties, Clone, PartialEq)]
pub struct Props
{
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[styled_component(Navbar)]
pub fn navbar(props: &Props) -> Html
{
    html!
    {
        <span class={"navbar"}>
            <h1>{"RPG Helper"}</h1>
            <h3>{"Home"}</h3>
            <h3>{"Dashboard"}</h3>
            <h3>{"Settings"}</h3>

            <p>{"Profile"}</p>
        </span>
    }
}