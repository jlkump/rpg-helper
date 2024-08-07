use super::{game::Game, intermediate_view::IntermediateView, IndexRef, Query, Storable};

#[derive(Debug, PartialEq, Clone)]
pub enum ViewContext<'a> {
    GameView(&'a Game<'a>),
    IntermediateView(&'a IntermediateView<'a>),
}

impl<'g> ViewContext<'g> {
    pub fn as_game_view(&self) -> Option<&'g Game> {
        match self {
            ViewContext::GameView(g) => Some(g),
            ViewContext::IntermediateView(_) => None,
        }
    }

    pub fn as_intermediate_view(&self) -> Option<&'g IntermediateView> {
        match self {
            ViewContext::GameView(_) => None,
            ViewContext::IntermediateView(v) => Some(v),
        }
    }

    pub fn value_to_ref<T: Storable, R: IndexRef<T>>(&self, v: &T) -> Query<R> {
        todo!()
    }
}