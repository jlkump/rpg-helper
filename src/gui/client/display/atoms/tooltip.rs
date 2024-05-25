use gloo::timers::callback::{Interval, Timeout};
use yew::prelude::*;
use stylist::{css, yew::styled_component, Style};

use crate::gui::client::{display::atoms::{colored_panel::ColoredPanel, loader::Loader}, use_theme, Theme};

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    pub tooltip_content: Html,
    #[prop_or(Position::Right)]
    pub position: Position,
}

#[derive(Clone, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Left,
    Right
}

pub enum Msg {
    MouseEnteredTooltip(String, String),
    MouseExitedTooltip,
    MouseEnteredPane,
    MouseExitedPane,
    Done,
    Close,
    Cancel,
}

pub struct Tooltip {
    hovered_tooltip: bool,
    hard_pane: bool,
    hovered_pane: bool,
    stay_hover_timeout: Option<Timeout>,
    empty_timeout: Option<Timeout>,
    mouse_pos: (String, String)
}

impl Tooltip {
    fn should_display_pane(&self) -> bool {
        self.hovered_pane || self.hovered_tooltip || self.hard_pane 
    }

    fn cancel(&mut self) {
        self.stay_hover_timeout = None;
        self.empty_timeout = None;
    }
}

impl Component for Tooltip {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            hovered_pane: false,
            hard_pane: false,
            hovered_tooltip: false,
            stay_hover_timeout: None,
            empty_timeout: None,
            mouse_pos: ("0px".to_string(), "0px".to_string())
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MouseEnteredTooltip(x, y) => {
                self.hovered_tooltip = true;
                if self.stay_hover_timeout.is_none() && !self.hovered_pane {
                    let handle = {
                        let link = ctx.link().clone();
                        Timeout::new(1000, move || link.send_message(Msg::Done))
                    };
                    self.stay_hover_timeout = Some(handle);
                }
                self.mouse_pos = (x, y);
                true
            },
            Msg::MouseExitedTooltip => {
                self.hovered_tooltip = false;

                self.stay_hover_timeout = None;
                if self.empty_timeout.is_none() && self.hard_pane {
                    let handle = {
                        let link = ctx.link().clone();
                        Timeout::new(700, move || link.send_message(Msg::Close))
                    };
                    self.empty_timeout = Some(handle);
                }
                true
            },
            Msg::MouseEnteredPane => {
                self.hovered_pane = true;
                true
            },
            Msg::MouseExitedPane => {
                self.hovered_pane = false;
                self.hard_pane = false;

                self.cancel();
                true
            },
            Msg::Done => {
                self.hard_pane = true;

                self.cancel();
                true
            },
            Msg::Close => {
                if !self.hovered_pane {
                    self.hovered_pane = false;
                    self.hovered_tooltip = false;
                    self.hard_pane = false;
                }

                self.cancel();
                true
            },
            Msg::Cancel => {
                self.cancel();
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_entered_tooltip = ctx.link().callback(|m: MouseEvent| Msg::MouseEnteredTooltip(format!("{}px", m.x() + 10), format!("{}px", m.y())));
        let on_exited_tooltip = ctx.link().callback(|_| Msg::MouseExitedTooltip);
        let on_entered_pane = ctx.link().callback(|_| Msg::MouseEnteredPane);
        let on_exited_pane = ctx.link().callback(|_| Msg::MouseExitedPane);
        let on_tooltip_clicked = ctx.link().callback(|_| Msg::MouseEnteredPane);
        html! {
            <>
                if self.should_display_pane() {
                    <TooltipPane onmouseenter={on_entered_pane} onmouseleave={on_exited_pane} hard_border={self.hard_pane} pos={self.mouse_pos.clone()}>
                        { ctx.props().tooltip_content.clone() }
                    </TooltipPane>
                }
                <div onmouseenter={on_entered_tooltip} onmouseleave={on_exited_tooltip} onmousedown={on_tooltip_clicked}>
                    { ctx.props().children.clone() }
                </div>
            </>
        }
    }
}

#[derive(Clone, Properties, PartialEq)]
struct TooltipPaneProps {
    children: Html,
    hard_border: bool,
    onmouseenter: Option<Callback<MouseEvent>>,
    onmouseleave: Option<Callback<MouseEvent>>,
    pos: (String, String),
}

#[styled_component(TooltipPane)]
fn tooltip_pane(props: &TooltipPaneProps) -> Html {
    let theme = use_theme();
    let border = if props.hard_border {
        theme.border_tooltip_hard.clone()
    } else {
        theme.border_tooltip_light.clone()
    };
    let style = css!(
        r#"
            position: fixed;
            left: ${pos_x};
            top: ${pos_y};
            border: 4px solid ${border};
            min-width: 160px;
            box-shadow: 8px 8px 4px ${hover};
            z-index: 1;
            background-color: ${bg};
        "#,
        bg=theme.paper_dark,
        border=border,
        hover=theme.hover_dropshadow,
        pos_x=props.pos.0,
        pos_y=props.pos.1,
    );
    html! {
        <div style="position: relative;">
            <div class={style} onmouseenter={props.onmouseenter.clone()} onmouseleave={props.onmouseleave.clone()}>
                { props.children.clone() }
                if !props.hard_border {
                    <div style="position: relative;">
                        <Loader color={theme.border_colored.clone()} style="position: absolute; top: -25px;" />
                    </div>
                }
            </div>
        </div>
    }
}

// #[styled_component(Tooltip)]
// pub fn tooltip(props: &Props) -> Html {
//     let theme = use_theme();
//     let tooltip = css!(
//         r#"
//             position: absolute;
//             background: ${bg};
//         "#,
//         bg=theme.paper_dark
//     );
//     html! {
//         <>
//             <div class={tooltip}>
//                 <h2>{"Tooltip content!"}</h2>
//             </div>
//             <div onmouseenter={Callback::from(|_| ())} onmouseleave={Callback::from(|_| ())}>
//                 { props.children.clone() }
//             </div>
//         </>
//     }
// }