pub mod ability;
pub mod character;
pub mod creation;
pub mod dice;
pub mod event;
pub mod game;
pub mod input;
pub mod inventory;
pub mod location;
pub mod player;
pub mod ruleset;
pub mod timeline;

pub mod reserved_tags
{
    use crate::api::data::tag::Subtag;
    
    /// Comes from Claude b/c IDK how to do macros.
    /// This seems correct, though I wouldn't be surprised if it caused some problems.
    /// I have performed some basic tests and this seems to do as expected, so will be in
    /// use until it breaks.
    #[macro_export]
    macro_rules! reserved_subtags
    {
        ($($name:ident = $str:expr),* $(,)?) =>
        {
            // Generate the constant array of strings
            pub(super) const RESERVED_SUBTAG_STRINGS: &[&str] = &[ $($str),* ];
            
            // Generate static Subtag for each entry
            reserved_subtags!(@generate_statics 0, $($name = $str),*);
        };
        
        // Helper to generate static declarations with indices
        (@generate_statics $index:expr, $name:ident = $str:expr) =>
        {
            pub static $name: once_cell::sync::Lazy<Subtag> = 
                once_cell::sync::Lazy::new(||
                {
                    Subtag::reserved_new($index)
                });
        };
        
        (@generate_statics $index:expr, $name:ident = $str:expr, $($rest_name:ident = $rest_str:expr),+) =>
        {
            pub static $name: once_cell::sync::Lazy<Subtag> = 
                once_cell::sync::Lazy::new(||
                {
                    Subtag::reserved_new($index)
                });
            
            reserved_subtags!(@generate_statics $index + 1, $($rest_name = $rest_str),+);
        };
    }

    // This is the set of tags which are pre-made and reserved
    // to be a part of the tag registry. They are the first
    // items in the registry before anything else is inserted and thus
    // their intern id is always known.
    // 
    // By using this macro, these Subtags can be directly accessed
    // in code without needing to use a TagRegistry.
    reserved_subtags!
    {
        DEFAULT = "default",
        ABILITY = "ability",
        // ITEM
        TIMELINE = "timeline",
    }
}