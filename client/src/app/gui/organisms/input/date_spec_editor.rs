use std::{cell::RefCell, collections::HashSet, ops::IndexMut, rc::Rc};

use rpg_helper::api::{data::tag::Tag, display::icon::Icon, rpg::timeline::DateSpec};
use yew::prelude::*;

use crate::app::gui::atoms::{icon::IconHtml, input::{equation_input::EquationInput, searchbar::Searchbar, tag_input::TagInput}, list::List};

#[derive(Properties, Clone, PartialEq)]
pub struct Props
{
    pub current: UseStateHandle<Rc<RefCell<DateSpec>>>,
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
    let tag_set = use_state(|| props.current.borrow().required_values.clone().into_iter().collect::<Vec<_>>());
    let reset =
    {
        let tag_set = tag_set.clone();
        let current = props.current.clone();
        Callback::from(move |_: MouseEvent|
        {
            log::info!("Reset clicked!");
            tag_set.set(current.borrow().required_values.clone().into_iter().collect::<Vec<_>>());
        })
    };


    let tag_to_html =
    {
        let tag_set = tag_set.clone();
        Callback::from(move |(i, t) : (usize, Tag) |
        {
            let tag_name = t.clone();
            log::info!("Initializing tag {} of index {}", t, i);
            let onclick =
            {
                let tag_set = tag_set.clone();
                Callback::from(move |_|
                {
                    log::info!("Removing tag {} of index {}", tag_name, i);
                    let mut tags = (*tag_set).clone();
                    tags.remove(i);
                    tag_set.set(tags);
                })
            };

            let tag_set = tag_set.clone();
            let onchange = Callback::from(move |new_tag|
            {
                log::info!("Changing index {} to tag {}", i, new_tag);
                let mut tags = (*tag_set).clone();
                tags[i] = new_tag;
                tag_set.set(tags);
            });
            html!
            {
                <div class="inline-wrap" key={format!("{}-{}", i, t.to_string())}>
                    <TagInput default_value={t.to_string()} style="margin-top: 4px; margin-bottom: 4px;" {onchange}/>
                    <IconHtml class="faint click" style="margin: 0 2px 0 2px;" icon={Icon::Delete} {onclick} />
                </div>
            }
        })
    };

    let add_clicked =
    {
        let tag_set = tag_set.clone();
        Callback::from(move |_|
        {
            log::info!("Added default tag to tag set!");
            let mut tags = (*tag_set).clone();
            tags.push(Tag::default());
            tag_set.set(tags);
        })
    };
    let add_disabled = tag_set.contains(&Tag::default());
    log::info!("{}]", tag_set.iter().fold("Current values: [".to_string(), |mut s, t: &Tag| { s.push_str(&format!("{}, ", t.to_str())); s }));


    let allowed: Vec<Tag> = tag_set.iter().flat_map(|t| [t.add_prefix(DateSpec::get_ordering_lhs_tag()), t.add_prefix(DateSpec::get_ordering_rhs_tag())]).collect();

    html!
    {
        <form>
            <h3>{"Date"}</h3>
            <hr class="full"/>
            <List<Tag> data_to_html_panel={tag_to_html} data_ref={tag_set.clone()}>
                <span class="inline-wrap">
                    {"Required Values"}
                    <IconHtml class="help faint" icon={Icon::Help} style="margin-left: 4px;" />
                    <IconHtml class="click toolbar-button" style="margin-left: auto;" icon={Icon::Add} onclick={add_clicked} disabled={add_disabled} />
                    <IconHtml class="click toolbar-button" style="font-size: 16px;" icon={Icon::Reset} onclick={reset}/>
                </span>
            </List<Tag>>
            <span class="inline-wrap">
                {"Ordering Equation"}
                <IconHtml class="help faint" icon={Icon::Help} style="margin-left: 4px;" />
            </span>
            <div class="inline-wrap">
                <EquationInput 
                    equation_id={props.current.borrow().ordering.name.clone()} 
                    default_value={"(rhs.Year - lhs.Year) * 365 + (rhs.Month - lhs.Month) * 30 + (rhs.Day - lhs.Day)"}
                    name={props.current.borrow().ordering.name.to_string()}
                    placeholder={"(rhs.Year - lhs.Year) * 365"}
                    onchange={Callback::from(|e| { log::info!("New equation: {:?}", e)})}
                    allowed_tag_values={allowed}
                    />
                <IconHtml class="faint click" style="margin: 0 2px 0 2px;" icon={Icon::Clear} />
            </div>
        </form>
    }
}