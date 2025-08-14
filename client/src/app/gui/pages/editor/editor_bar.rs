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

    // Editor buttons
    // - Move tool
    // 
    // - Create panel
    html!
    {
        <div style="width: 60vw; height: 60px; background: rgba(203, 195, 179, 0.5); border-radius: 4px; margin: 20px;">
            <button>{"Test"}</button>
        </div>
    }
}