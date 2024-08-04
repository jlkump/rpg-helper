//////////////////////////////////////
//      Data Transfer Model         //
//////////////////////////////////////
// All the serializable forms of the data model for network transfer.
// Any Raw Data from over the network will not have references to their encapsulating view context, so that must be built back upon transfer.
pub mod character;
pub mod location;
pub mod ruleset;
pub mod types;
pub mod values;
pub mod wiki;