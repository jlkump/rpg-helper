use std::rc::Rc;
use std::cell::RefCell;
use log::info;
use yew::{function_component, hook, html, use_state, Children, ContextProvider, Html, Properties, UseStateHandle};

pub trait Focusable
{
    /// Used to notify focus events
    fn set_focus(&mut self, is_focused: bool);

    /// Used for comparison between focused elements
    fn get_id(&self) -> u32;
}

#[derive(Clone)]
pub struct FocusContext
{
    inner: UseStateHandle<Option<Rc<RefCell<dyn Focusable>>>>
}

impl FocusContext
{
    pub fn new(inner: UseStateHandle<Option<Rc<RefCell<dyn Focusable>>>>) -> FocusContext
    {
        FocusContext { inner }
    }
    
    /// Clear the current focus
    pub fn clear_focus(&self)
    {
        info!("Focus changed");
        if let Some(current) = (*self.inner).as_ref()
        {
            current.borrow_mut().set_focus(false);
        }
        self.inner.set(None);
    }
    
    /// Focus a new element (unfocuses the previous one if any)
    pub fn focus_element(&self, element: Rc<RefCell<dyn Focusable>>)
    {
        // Unfocus current element if there is one
        if let Some(current) = (*self.inner).as_ref()
        {
            current.borrow_mut().set_focus(false);
        }
        
        // Focus the new element
        element.borrow_mut().set_focus(true);
        self.inner.set(Some(element));
    }
}

impl PartialEq for FocusContext
{

    fn eq(&self, rhs: &Self) -> bool
    {
        match ((*self.inner).as_ref(), (*rhs.inner).as_ref())
        {
            (Some(left), Some(right)) =>
            {
                left.borrow().get_id() == right.borrow().get_id()
            },
            (None, None) => true,
            _ => false,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct FocusProviderProps
{
    pub children: Children,
}

#[function_component(FocusProvider)]
pub fn focus_provider(props: &FocusProviderProps) -> Html
{
    let focus_state = use_state(|| None::<Rc<RefCell<dyn Focusable>>>);
    let context = FocusContext::new(focus_state);
    
    html!
    {
        <ContextProvider<FocusContext> context={context}>
            {props.children.clone()}
        </ContextProvider<FocusContext>>
    }
}

#[hook]
pub fn use_focus() -> FocusContext
{
    use yew::use_context;

    use_context::<FocusContext>().expect("FocusContext not found")
}