/// The wiki api layers ontop the rpg layer
/// It provides meta-data about tags in a md-like format
///
/// It also provides the user with the ability to create pages
/// in a wiki style, which can link to other pages. The pages
/// are written in markdown with the ability to display images
/// as well as reference some of the data specific to a game
/// or ruleset.
pub mod error;
pub mod name;
pub mod note;
pub mod page;
pub mod syntax;

/// The wiki is the data struct that holds all the names,
/// notes, and pages for display purposes.
/// 
/// A wiki can be layered atop an existing layer, just like a ctx.
/// This is useful for when, during a game, the game master
/// would like to define some wiki data, but only wishes for it
/// to exist in the context of the game itself. (There is the
/// option for the game master to propigate the change
/// to the base wiki if they have edit permissions)
pub struct Wiki
{

}