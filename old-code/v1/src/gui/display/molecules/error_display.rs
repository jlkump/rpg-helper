use yew::{prelude::*, virtual_dom::VNode};
use stylist::yew::styled_component;
use yew_icons::{Icon, IconId};
use yewdux::use_store;

use crate::{error::Error, gui::display::atoms::autosave::MILI_TO_MINUTES, store::{remove_error, Errors}};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {

}

#[styled_component(ErrorDisplay)]
pub fn error_display(props: &Props) -> Html {
    let css = css!(
        r#"
            position: fixed;
            top: -50px;
            left: 50%;
        "#
    );

    let (s, _) = use_store::<Errors>();
    html! {
        <div class={css}>
            {s.as_ref().errors.iter().map(|i| { html! { <ErrorPanel err={i.clone()}/> } }).collect::<Vec<VNode>>()}
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct PanelProps {
    err: Error
}

#[styled_component(ErrorPanel)]
fn error_panel(props: &PanelProps) -> Html {
    let css = css!(
        r#"
        "#
    );

    let onclick = {
        let (_, dis) = use_store::<Errors>();
        let err = props.err.clone();
        Callback::from(move |_| { remove_error(&err, dis.clone()) })
    };
    let ontimeout = {
        let (_, dis) = use_store::<Errors>();
        let err = props.err.clone();
        Callback::from(move |_| { remove_error(&err, dis.clone()) })
    };
    html! {
        <div class={css}>
            <Icon icon_id={IconId::FontAwesomeSolidXmark} {onclick}/>
            <Timeout callback={ontimeout} />
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct TimeoutProps {
    callback: Callback<()>,
}

enum Msg {}

struct Timeout {
    timeout: gloo::timers::callback::Timeout,
}

impl Component for Timeout {
    type Message = Msg;

    type Properties = TimeoutProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            timeout: gloo::timers::callback::Timeout::new(MILI_TO_MINUTES / 2, { 
                let props = ctx.props().clone();
                move || {props.callback.emit(())}
            })
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {}
    }
}