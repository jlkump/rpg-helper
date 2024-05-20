use stylist::yew::styled_component;
use yew::{html, Html, Properties};

use crate::data::meta_type::MetaTypeInstance;

use super::style::StyleSheet;

#[derive(Clone, Copy, PartialEq)]
struct Display;

impl Display {
    pub fn update_display(&mut self, s: DisplaySheet) {
        todo!()
    }

    pub fn update_style(&mut self, s: StyleSheet) {
        todo!()
    }

    pub fn display(&self) -> Html {
        todo!()
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct DisplayProps {
    display: Display
}

#[styled_component(DisplayComp)]
pub fn display_comp(props: &DisplayProps) -> Html {
    props.display.display()
}


struct DisplaySheet {

}

impl DisplaySheet {
    pub fn get_display_for<T>(&self, t: MetaTypeInstance, details: T) {
        // TODO: Define details for display, such as whether or not a type displays a property of a type. 
        // Also whether or not the display is a brief panel display or detailed panel display
    }
}