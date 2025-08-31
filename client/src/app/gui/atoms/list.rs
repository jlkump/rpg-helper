use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T: Clone + PartialEq>
{
    /// The html panel is expected to fit within the scrollable base of this list editor
    pub data_to_html_panel: Callback<T, Html>,
    pub data_ref: Vec<T>,
    /// The header of the list. Using the children name to allow
    /// wrapping the list around the header html in the macro html! format.
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(List)]
pub fn list<T: Clone + PartialEq>(props: &Props<T>) -> Html
{
    html!
    {
        <div class={props.class.clone()} style={props.style.clone()}>
            {props.children.clone()} // The header of the list
            <div class="scroll-wrapper">
            {
                props.data_ref.iter().map(
                    |t| { props.data_to_html_panel.emit(t.clone()) }
                ).collect::<Html>()
            }
            </div>
        </div>
    }
}