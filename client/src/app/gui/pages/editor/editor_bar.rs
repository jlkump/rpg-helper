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

    // TODO: Toggle bar to be shown
    html!
    {
        <div style="width: 60vw; height: 60px; background: rgba(203, 195, 179, 0.5); border-radius: 4px; margin: 20px;">
            <ul style="list-style: none; display: flex; gap: 1.5rem;">
                // Create new panel
                <li><i class="fa-regular fa-square-plus"></i></li>
                <li><i class="fa-solid fa-arrows-up-down-left-right"></i></li>
                <li><i class="fa-solid fa-arrow-pointer"></i></li>
                <li><i class="fa-solid fa-expand"></i></li>
                <li><i class="fa-regular fa-clone"></i></li>
                // For editing an individual panel
                <li><i class="fa-regular fa-square-caret-down"></i></li>
                <li><i class="fa-regular fa-square-caret-left"></i></li>
                <li><i class="fa-regular fa-square-caret-right"></i></li>
                <li><i class="fa-regular fa-square-caret-up"></i></li>
                <li><i class="fa-regular fa-trash-can"></i></li>
                <li><i class="fa-solid fa-palette"></i></li>
                <li><i class="fa-solid fa-inbox"></i></li>
            </ul>
        </div>
    }
}