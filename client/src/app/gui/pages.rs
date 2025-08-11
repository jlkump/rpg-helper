use yew::prelude::*;

use crate::app::gui::molecules::navbar::Navbar;

pub mod editor;

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

#[function_component(BasePage)]
pub fn base_page(props: &Props) -> Html
{
    html!
    {
        <div>
            <Navbar />
            {props.children.clone()}
        </div>
    }
}