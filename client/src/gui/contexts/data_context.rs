use std::{ops::Deref, rc::Rc};

use stylist::yew::styled_component;
use yew::{hook, html, use_state, Children, ContextProvider, Html, Properties, UseStateHandle};

use crate::model::data_model::storage::{game::Game, intermediate_view::IntermediateView, view_context::ViewContext};

#[derive(Debug, Clone)]
pub(crate) struct ViewDataCtx {
    inner: UseStateHandle<ViewContext>,
}

impl ViewDataCtx {
    pub fn new(inner: UseStateHandle<ViewContext>) -> Self {
        Self { inner }
    }

    pub fn set(&self, data: ViewContext) {
        self.inner.set(data)
    }
}

impl Deref for ViewDataCtx {
    type Target = ViewContext;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PartialEq for ViewDataCtx {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct DataProviderProps {
    pub children: Children,
    pub data: UseStateHandle<ViewContext>,
}

#[styled_component]
pub(crate) fn DataProvider(props: &DataProviderProps) -> Html {
    let ctx = ViewDataCtx::new(props.data.clone());

    html! {
        <ContextProvider<ViewDataCtx> context={ctx}>
            {props.children.clone()}
        </ContextProvider<ViewDataCtx>>
    }
}

#[hook]
pub(crate) fn use_data_context() -> Option<ViewDataCtx> {
    use yew::use_context;
    use_context::<ViewDataCtx>()
}