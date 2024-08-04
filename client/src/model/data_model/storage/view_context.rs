use std::rc::{Rc, Weak};

use super::{game::Game, intermediate_view::IntermediateView, IndexRef, Query, Storable};

#[derive(Debug, Clone)]
pub enum ViewContext {
    GameView(Weak<Game>),
    IntermediateView(Weak<IntermediateView>),
}

impl PartialEq for ViewContext {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::GameView(l0), Self::GameView(r0)) => {
                match (l0.upgrade(), r0.upgrade()) {
                    (Some(l0), Some(r0)) => l0.as_ref().id == r0.as_ref().id,
                    _ => false
                }
            },
            (Self::IntermediateView(l0), Self::IntermediateView(r0)) => {
                match (l0.upgrade(), r0.upgrade()) {
                    (Some(l0), Some(r0)) => l0.as_ref() == r0.as_ref(),
                    _ => false
                }
            },
            _ => false,
        }
    }
}

impl ViewContext {
    pub fn as_game_view(&self) -> Option<Rc<Game>> {
        match self {
            ViewContext::GameView(g) => g.upgrade(),
            ViewContext::IntermediateView(_) => None,
        }
    }

    pub fn as_intermediate_view(&self) -> Option<Rc<IntermediateView>> {
        match self {
            ViewContext::GameView(_) => None,
            ViewContext::IntermediateView(v) => v.upgrade(),
        }
    }

    pub fn value_to_ref<T: Storable, R: IndexRef<T>>(&self, v: &T) -> Query<R> {
        todo!()
    }
}