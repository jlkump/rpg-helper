use rpg_helper::api::rpg::timeline::DateSpec;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props
{
    pub current: DateSpec,
    pub onchange: Callback<DateSpec>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: AttrValue,
}

/// User creates a list of required attribute values,
/// which is simply an input list of tags.
/// The list of tags is displayed first and can be edited in-panel
/// After the list of tags is the ordering equation. The equation
/// can only contain tags with the names given in the list before,
/// prefixed with either lhs or rhs.
///
/// The callback on this element is only fired when a valid DateSpec can be created
/// from changes to the input. Otherwise, no callback will be fired.
/// If there is a discrepancy between the true value and the provided data value,
/// then the "reset" button can be clicked to reset to the true data value.
#[function_component(DateSpecEditor)]
fn date_editor(props: &Props) -> Html
{
    let required = use_state(|| props.current.required_values.clone());
    let ordering = use_state(|| props.current.ordering.clone());

    let reset =
    {
        let current = props.current.clone();
        let required = required.clone();
        let ordering = ordering.clone();
        Callback::from(move |_: MouseEvent|
        {
            required.set(current.required_values.clone());
            ordering.set(current.ordering.clone());
        })
    };
    
    html!
    {

    }
}