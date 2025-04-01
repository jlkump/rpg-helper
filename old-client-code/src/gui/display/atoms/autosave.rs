use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use gloo::timers::callback::Interval;
use yew::prelude::*;

use crate::error::Error;

pub const MILI_TO_MINUTES: u32 = 60000;

#[derive(Properties, PartialEq, Clone)]
pub struct AutosaveProps<T> 
where
    T: PartialEq + Clone
{
    pub data_ref: Rc<RefCell<T>>,
    pub save_fn: Option<Callback<Rc<RefCell<T>>, Result<(), Error>>>, // Takes in the updated data, calls the api-backend update
    pub error_handler: Option<Callback<Error>>, 
    pub has_changes: UseStateHandle<bool>, // Is true when there are changes that need to be saved
    #[prop_or(MILI_TO_MINUTES)] // Every 1 minute, by default
    pub save_interval: u32, // In Miliseconds
}

pub struct Autosave<T> {
    save_interval: Interval,
    _phantom: PhantomData<T>
}

pub enum Msg {}

impl<T> Component for Autosave<T> 
where
    T: PartialEq + Clone + 'static
{
    type Message = Msg;

    type Properties = AutosaveProps<T>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            save_interval: Interval::new(ctx.props().save_interval, 
            {
                let prop_data = ctx.props().clone();
                move || {
                    if *prop_data.has_changes {
                        if let Some(f) = &prop_data.save_fn {
                            if let Err(e) = f.emit(prop_data.data_ref.clone()) {
                                if let Some(er) = &prop_data.error_handler {
                                    er.emit(e)
                                }
                            }
                        }
                    }
                }
            }),
            _phantom: PhantomData,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>{format!("Autosave test.")}</div>
        }
    }
}