use super::{indexes::type_index::TypeIndex, meta_type::{MetaTypeInstance, Type}};


pub struct Timeline {
    timeline_events: TypeIndex,
    events: Vec<TimelineEventInstance>,
}

pub struct TimelineEvent {
    target_type: Type
}

pub struct TimelineEventInstance {
    t: TimelineEvent,
    // inst: MetaTypeInstance,
}

impl PartialEq for TimelineEventInstance {
    fn eq(&self, other: &Self) -> bool {
        todo!()
        // Compared using the Date value in the event
        // self.t == other.t
    }
}

impl PartialOrd for TimelineEventInstance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
        // self.t.partial_cmp(&other.t)
    }
}