/// The display api layers ontop of the wiki and rpg layer
/// It allows users to define the specification for how they
/// want things to appear on the client. 
/// 
/// This API is meant to be abstracted away from the display implementation
/// of the client, so html and css is not directly referenced here.
/// Instead, a specification for how things should look is defind
/// and the client decides how to display based on that specification.
/// 
/// This also includes things, such as the color style of tags,
/// coloring for panels, etc.
pub mod error;
pub mod style;
pub mod panel;