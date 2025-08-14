use yew::{function_component, hook, html, use_state, Children, ContextProvider, Html, Properties, UseStateHandle};

#[derive(Clone)]
pub struct FocusContext
{
    inner: UseStateHandle<Option<String>>
}

impl FocusContext
{
    pub fn new(inner: UseStateHandle<Option<String>>) -> FocusContext
    {
        FocusContext { inner }
    }
    
    pub fn clear_focus(&self)
    {
        log::info!("Cleared focus");
        self.inner.set(None);
    }
    
    pub fn set_focus(&self, element: &str)
    {
        log::info!("Focus updated to: {}", element);
        self.inner.set(Some(element.to_string()));
    }

    pub fn toggle_focus(&self, element: &str)
    {
        if (*self.inner).as_ref().map(|s| s.as_str()) == Some(element)
        {
            self.clear_focus();
        }
        else
        {
            self.set_focus(element);
        }
    }

    pub fn get_focus(&self) -> Option<&str>
    {
        (*self.inner).as_ref().map(|s| s.as_str())
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
                left == right
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
    let focus_state = use_state(|| None::<String>);
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