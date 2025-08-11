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
    let active = use_state(|| false);

    let onclick = 
    {
        let active = active.clone();
        Callback::from(
            move |_|
            {
                active.set(!*active);
            }
        )
    };

    html!
    {
        <nav class={"navbar"}>
            <span class={"content-container"}>
                <a class={"logo"}>{"RPG Helper"}</a>
                <button class={if *active { "nav-toggle active" } else { "nav-toggle" }} {onclick}>
                    <span class="bar"></span>
                    <span class="bar"></span>
                    <span class="bar"></span>
                </button>
                <ul class={if *active { "menu active" } else { "menu" }}>
                    <li><a>{"Home"}</a></li>
                    <li><a>{"Dashboard"}</a></li>
                    <li><a>{"About"}</a></li>
                </ul>
            </span>
        </nav>
    }
}