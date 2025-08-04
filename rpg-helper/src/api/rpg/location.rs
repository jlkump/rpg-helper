use crate::api::data::tag::Tag;

/// A location layers some contextual data ontop of the character
/// when they are in the location.
/// 
/// All locations require a tag for identification which begins with "location"
/// For example, "location.home", "location.kingdom.palace"
///
/// A character should never contain a location tag and should instead
/// gain location tags from being layered on by a location.
/// 
/// Sub-regions of a location can be identified by sub-tags.
pub struct Location
{
    name: String,
    identifier: Tag,
}