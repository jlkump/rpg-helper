use rpg_helper::api::display::icon::Icon;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T>
where 
    T: PartialEq + Clone,
{
    pub buttons: Vec<EditorBarButton<T>>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[derive(Clone, PartialEq)]
pub struct EditorBarButton<T>
{
    icon: Icon,
    t: EditorBarButtonType<T>
}

#[derive(Clone, PartialEq)]
pub enum EditorBarButtonType<T>
{
    Toggle(Callback<T, bool>),
    OnClick(Callback<T>),
}

/// Provide a list of icons and callbacks
/// for when the icon is clicked.
/// The icon must also be defined as either toggle
/// or an on-click type
#[function_component(EditorBar)]
pub fn editor_bar<T: PartialEq + Clone>(props: &Props<T>) -> Html
{
    html!
    {
        <div>
            <h1>{"Title bar"}</h1>
        </div>
    }
}