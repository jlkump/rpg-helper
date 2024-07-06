use serde::{Deserialize, Serialize};

use crate::model::data_model::primatives::{types::Type, values::{number::Number, Value}};

use super::{types::{EquationRef, MetaTypeRef, TypeRef}, values::{MetaInstRef, ValueRef}};


#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct Timeline {
    events: Vec<Event>,
    current_date: Date,
    // There is a current date unique for the Game and the player's characters.
    // The character may be behind the date of the game.
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct Date {
    year: Number,    // Defined specifically by a Year  meta-type required to be placed in the rule-set. Must be a num
    month: Number,   // Defined specifically by a Month meta-type required to be placed in the rule-set. Must be a num
    day: Number,     // Defined specifically by a Day   meta-type required to be placed in the rule-set. Must be a num
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // TODO
        todo!()
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct EventRef {

}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct Event {
    event_type: EventTypeRef,  // Defined by a EventType meta-type. The event type holds the reference to the effect
}

// impl PartialOrd for Event {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         if let Ok(EvalResult::Value(v)) = self.event_type.ordering.compute().eval_f32() {
//             if let Ok(EvalResult::Value(o)) = other.event_type.ordering.compute().eval_f32() {
//                 if v < o {
//                     return Some(std::cmp::Ordering::Less);
//                 } else if v > o {
//                     return Some(std::cmp::Ordering::Greater);
//                 } else {
//                     return Some(std::cmp::Ordering::Equal);
//                 }
//             }
//         }
//         None
//     }
// }

// Example EventType
//
// EventType {
//    name: "Adventure",
//    structure: MetaRef { // EventType-Adventure
//                  "Source Quality Min": 5
//                  "Source Quality Max": 5
//                  "AdvancementTotal": ["Range(Source Quality Min, Source Quality Max)", "WholeNumber(Input)"]
//               }
//    effect_type: AddToList,
//    operation: 
//    ordering: ..,
//    restrictions: 
// }

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct EventTypeRef {

}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct EventType {
    name: String,
    structure: MetaTypeRef,  // Structure of the event, for input by user.
    action: EventAction,
    ordering: EquationRef,  // Defined by some evaluation of the internal values. Used to order the events on the timeline
    restrictions: Vec<EquationRef>, // Expect bools. Will compute based on the structure of the input given by the user
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub enum EventAction {
    ChangeMetaRef(MetaInstRef, EquationRef), // Ref to modify and how to modify it
    AddValueToCharacter(TypeRef),
    RemoveValueFromCharacter(ValueRef),
    // Multiple?
}

// Event record ensures idempotent effects that can be undone or can be intersperced with new effects
#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct EventRecord {
    effect_type: EventTypeRef,
    target: ValueRef,
    prev_value: ValueRecord, 
    new_value: ValueRecord,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
struct ValueRecord {
    // Copy the references as their actual value to ensure the timeline is durable to changes in types.
    // When Value is MetaInstRef, copy the value as a MetaInst to ensure durability
}