// This component is able to show various sheets by clicking different tabs at the top.
// Each sheet can be assigned a unique tab icon to represent its data
use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {

}

#[styled_component(TabbedSheet)]
pub fn tabbed_sheet(props: &Props) -> Html {
    html! {
        <div>
            <h1>{"Tabbed Sheet (TODO)"}</h1>
        </div>
    }
}