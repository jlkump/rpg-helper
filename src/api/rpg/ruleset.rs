use crate::api::rpg::{event::EventSet, location::Location, timeline::DateSpec};

/// Contains:
///     - The templates for character creation
///         A template for a character defines:
///         - Required attributes and default equations & conditionals
///         - Required text data (and optional text data)
///         - Character options
///             - A title
///             - A description
///             - Changes to the character creation context
///             - Changes to the end character
///     - The templates for equations
///     - The date spec
///     - Event schemas
///     - Locations and maps
///     - Pre-existing characters (like NPCs)
pub struct Ruleset
{
    date_spec: DateSpec,
    events: EventSet,
    locations: Location,
}