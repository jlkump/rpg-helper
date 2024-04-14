pub mod meta_type;
pub mod equation;
pub mod timeline;
pub mod indexes;


// TODO: Define DisplayIndex, which
// Maps a type to a DisplayData type
// A DisplayData holds what params are displayed
// A DisplayData also lets the user edit the data (if it is editable)
// Each field will have a "Is Mutable" flag

// TODO: Define a StyleIndex, which
// Which has a list of StyleSheets
// StyleSheets determine how a DisplayData is presented in HTML / CSS