use std::ops::Deref;

use gloo::timers::callback::Timeout;
use web_sys::window;
use yew::prelude::*;
use stylist::{css, yew::styled_component};

use crate::gui::client::{display::atoms::loader::Loader, use_theme};

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    pub tooltip_content: Html,
    #[prop_or(true)]
    pub simple: bool,
}

pub enum Msg {
    MouseEnteredTooltip(i32, i32),
    MouseMoveTooltip(i32, i32),
    MouseExitedTooltip,
    MouseEnteredPane,
    MouseExitedPane,
    Display,
    Done,
    Close,
}

pub struct Tooltip {
    hovered_tooltip: bool,
    hard_pane: bool,
    hovered_pane: bool,
    display: bool,
    delay_display: Option<Timeout>,
    stay_hover_timeout: Option<Timeout>,
    empty_timeout: Option<Timeout>,
    mouse_pos: (i32, i32)
}

impl Tooltip {
    fn should_display_pane(&self) -> bool {
        self.display 
    }

    fn cancel(&mut self) {
        self.delay_display = None;
        self.stay_hover_timeout = None;
    }

    fn close(&mut self) {
        if !self.hovered_pane {
            self.hovered_pane = false;
            self.hovered_tooltip = false;
            self.hard_pane = false;
            self.display = false;
            self.empty_timeout = None;
        }

        self.cancel();
    }
}

impl Component for Tooltip {
    type Message = Msg;

    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {
            hovered_pane: false,
            hard_pane: false,
            hovered_tooltip: false,
            display: false,
            delay_display: None,
            stay_hover_timeout: None,
            empty_timeout: None,
            mouse_pos: (0, 0)
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MouseEnteredTooltip(x, y) => {
                self.hovered_tooltip = true;
                if !ctx.props().simple {
                    if self.delay_display.is_none() && !self.display {
                        let handle = {
                            let link = ctx.link().clone();
                            Timeout::new(250, move || link.send_message(Msg::Display))
                        };
                        self.delay_display = Some(handle);
                    }
                } else {
                    self.display = true;
                }
                self.mouse_pos = (x, y);
                false
            },
            Msg::MouseMoveTooltip(x, y) => {
                self.mouse_pos = (x, y);
                true
            },
            Msg::MouseExitedTooltip => {
                self.hovered_tooltip = false;
                if !ctx.props().simple {
                    if self.display {
                        let handle = {
                            let link = ctx.link().clone();
                            Timeout::new(700, move || link.send_message(Msg::Close))
                        };
                        self.empty_timeout = Some(handle);
                    }
                } else {
                    self.close();
                }
                true
            },
            Msg::MouseEnteredPane => {
                self.hovered_pane = true;
                true
            },
            Msg::MouseExitedPane => {
                self.hovered_pane = false;

                if self.display {
                    let handle = {
                        let link = ctx.link().clone();
                        Timeout::new(350, move || link.send_message(Msg::Close))
                    };
                    self.empty_timeout = Some(handle);
                }
                true
            },
            Msg::Display => {
                if !ctx.props().simple {
                    if self.stay_hover_timeout.is_none() && !self.hovered_pane {
                        let handle = {
                            let link = ctx.link().clone();
                            Timeout::new(700, move || link.send_message(Msg::Done))
                        };
                        self.stay_hover_timeout = Some(handle);
                    }
                }
                self.display = true;
                true
            },
            Msg::Done => {
                self.hard_pane = true;

                self.cancel();
                true
            },
            Msg::Close => {
                self.close();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let simple = ctx.props().simple;

        let on_entered_tooltip = ctx.link().callback(|m: MouseEvent|
            Msg::MouseEnteredTooltip(m.x(), m.y()));
        let on_exited_tooltip = ctx.link().callback(|_| Msg::MouseExitedTooltip);


        let on_mouse_move = if simple {
            Some(ctx.link().callback(|m: MouseEvent| Msg::MouseMoveTooltip(m.x(), m.y())))
        } else {
            None
        };

        let on_entered_pane = if simple {
            None
        } else {
            Some(ctx.link().callback(|_| Msg::MouseEnteredPane))
        };

        let on_exited_pane = if simple {
            None
        } else {
            Some(ctx.link().callback(|_| Msg::MouseExitedPane))
        };
        
        html! {
            <>
                if self.should_display_pane() {
                    <TooltipPane onmouseenter={on_entered_pane} onmouseleave={on_exited_pane} 
                        hard_border={self.hard_pane} pos={self.mouse_pos} is_simple={simple}>
                        { ctx.props().tooltip_content.clone() }
                    </TooltipPane>
                }
                <div onmouseenter={on_entered_tooltip} onmouseleave={on_exited_tooltip} onmousemove={on_mouse_move}>
                    { ctx.props().children.clone() }
                </div>
            </>
        }
    }
}

fn get_mouse_quadrant(mouse_x: f64, mouse_y: f64) -> Quadrant {
    let window = window().expect("Couldn't find window");
    let screen_x = mouse_x  / window.inner_width().expect("Couldn't get screen x").as_f64().unwrap();
    let screen_y = mouse_y / window.inner_height().expect("Couldn't get screen y").as_f64().unwrap();

    if screen_x < 0.5 {
        if screen_y < 0.5 {
            Quadrant::TopLeft
        } else {
            Quadrant::BottomLeft
        }
    } else {
        if screen_y < 0.5 {
            Quadrant::TopRight
        } else {
            Quadrant::BottomRight
        }
    }
}


#[derive(Clone, Properties, PartialEq)]
struct TooltipPaneProps {
    children: Html,
    hard_border: bool,
    is_simple: bool,
    onmouseenter: Option<Callback<MouseEvent>>,
    onmouseleave: Option<Callback<MouseEvent>>,
    pos: (i32, i32),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}

#[styled_component(TooltipPane)]
fn tooltip_pane(props: &TooltipPaneProps) -> Html {
    let theme = use_theme();
    let border = if props.hard_border {
        theme.border_tooltip_hard.clone()
    } else {
        theme.border_tooltip_light.clone()
    };

    let is_subtooltip = use_context::<TransformOffsetContext>().is_some();

    let quadrant = get_mouse_quadrant(props.pos.0 as f64, props.pos.1 as f64);
    let translate = if is_subtooltip {
        match quadrant {
            Quadrant::TopLeft => "translate(50%, 5%)",
            Quadrant::TopRight => "translate(-50%, 5%)",
            Quadrant::BottomLeft => "translate(100%, 0%)",
            Quadrant::BottomRight => "translate(-100%, 0%)",
        }
    } else {
        match quadrant {
            Quadrant::TopLeft => "translate(2%, 2%)",
            Quadrant::TopRight => "translate(-102%, 2%)",
            Quadrant::BottomLeft => "translate(2%, -102%)",
            Quadrant::BottomRight => "translate(-102%, -102%)",
        }
    };
    let positioning = if is_subtooltip {
        "absolute"
    } else {
        "fixed"
    };

    let pos = if is_subtooltip {
        (0, 0)
    } else {
        props.pos
    };


    let style = css!(
        r#"
            position: ${pos};

            -webkit-transform: ${translate};
            transform: ${translate};
            /*transform-origin: ${origin};*/

            left: ${pos_x}px;
            top: ${pos_y}px;
            border: 3px solid ${border};
            min-width: 160px;
            box-shadow: 8px 8px 4px ${hover};
            z-index: 1;
            background-color: ${bg};
        "#,
        pos=positioning,
        translate=translate,
        bg=theme.paper_dark,
        border=border,
        hover=theme.hover_dropshadow,
        pos_x=pos.0,
        pos_y=pos.1,
    );

    html! {
        <div class={style} onmouseenter={props.onmouseenter.clone()} onmouseleave={props.onmouseleave.clone()}>
            <TransformProvider>
                { props.children.clone() }
            </TransformProvider>
            if !props.hard_border && !props.is_simple {
                <div style="position: relative; width: 0; height: 0;">
                    <Loader color={theme.border_colored.clone()} style="position: absolute; top: -25px;" />
                </div>
            }
        </div>
    }
}


// This may need to be moved to a seperate file for use in the rest of the crate if multiple 
// components are having the Fixed and Translate positioning problem.
#[derive(Clone, Debug, PartialEq)]
struct TransformOffset; // Add the transform changes if needed. Rn, we only really care IF there was a transform used, not what the properties of the transform are.

#[derive(Clone, Debug)]
struct TransformOffsetContext {
    inner: UseStateHandle<TransformOffset>,
}

impl TransformOffsetContext {
    pub fn new(inner: UseStateHandle<TransformOffset>) -> Self {
        Self { inner }
    }
}

impl Deref for TransformOffsetContext {
    type Target = TransformOffset;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PartialEq for TransformOffsetContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}

#[derive(Debug, PartialEq, Properties)]
struct TransformProviderProps {
    pub children: Children,
}

#[styled_component]
fn TransformProvider(props: &TransformProviderProps) -> Html {
    let pos_offset = use_state(|| TransformOffset {});

    let pos_ctx = TransformOffsetContext::new(pos_offset);
    
    html! {
        <ContextProvider<TransformOffsetContext> context={pos_ctx}>
            {props.children.clone()}
        </ContextProvider<TransformOffsetContext>>
    }
}