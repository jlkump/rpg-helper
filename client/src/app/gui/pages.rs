use stylist::yew::styled_component;
use yew::prelude::*;

use crate::app::{context::focus::{use_focus, FocusProvider}, gui::molecules::navbar::Navbar};

pub mod editor;
pub mod general;

#[derive(Properties, Clone, PartialEq)]
pub struct Props
{
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
        <FocusProvider>
            <InnerBasePage children={props.children.clone()} class={props.class.clone()} style={props.class.clone()} />
        </FocusProvider>
    }
}

#[styled_component(InnerBasePage)]
fn inner_base_page(props: &Props) -> Html
{
    let inner = css!(
        r#"
            .page
            {
                position: absolute;
                top: 0;
                left: 0;
                width: 100vw;
                height: 100vh;
                z-index: -1;
            }

            .fullpage-container
            {
                width: 100%; 
                height: calc(100vh - 83px);
            }
        "#
    );
    
    let fctx = use_focus();
    html!
    {
        <div class={classes!(inner, props.class.clone())} style={props.style.clone()}>
            <Navbar />
            {props.children.clone()}
            <div class={"page"} onclick={Callback::from(move |_| { fctx.clear_focus(); })}></div>
        </div>
    }
}