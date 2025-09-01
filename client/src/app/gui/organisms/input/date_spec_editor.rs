use std::{cell::RefCell, rc::Rc};

use rpg_helper::api::{data::tag::Tag, display::icon::Icon, rpg::timeline::DateSpec};
use yew::prelude::*;

use crate::app::gui::atoms::{icon::IconHtml, input::{equation_input::EquationInput, tag_input::TagInput}, list::List};

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
pub fn date_editor(props: &Props) -> Html
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
            log::info!("Reset clicked!");
            required.set(current.required_values.clone());
            ordering.set(current.ordering.clone());
        })
    };

    let tag_to_html =
    {
        Callback::from(move |t: Tag|
            {
                log::info!("Updating tag to html: {}", t);
                html!
                {
                    <TagInput default_value={t.to_string()} style="margin-top: 4px; margin-bottom: 4px;" />
                }
            }
        )
    };

    let data_ref: Vec<Tag> = (&*required).clone().into_iter().collect();
    (&*required).iter().for_each(|t| log::info!("Current values: {}", t));
    let allowed: Vec<Tag> = data_ref.iter().flat_map(|t| [t.add_prefix(DateSpec::get_ordering_lhs_tag()), t.add_prefix(DateSpec::get_ordering_rhs_tag())]).collect();
    html!
    {
        <form>
            <h3>{"Date"}</h3>
            <hr class="full"/>
            <List<Tag> data_to_html_panel={tag_to_html} data_ref={data_ref.clone()}>
                <span style="display: flex;">{"Required Values"}<IconHtml icon={Icon::Help} style="margin-left: auto; font-size: 16px; align-self:center;"/></span>
                <div>
                    <IconHtml icon={Icon::Add} />
                </div>
            </List<Tag>>
            <span>{"Ordering Equation"}</span>
            <EquationInput 
                equation_id={props.current.ordering.name.clone()} 
                default_value={"(rhs.Year - lhs.Year) * 365 + (rhs.Month - lhs.Month) * 30 + (rhs.Day - lhs.Day)"}
                name={props.current.ordering.name.to_string()}
                placeholder={"(rhs.Year - lhs.Year) * 365"}
                onchange={Callback::from(|e| { log::info!("New equation: {:?}", e)})}
                allowed_tag_values={allowed}
                 />
        </form>
    }
}