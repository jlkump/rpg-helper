use crate::api::display::style::{color::Color, font::Font};

// A theme is used to color the client
// A user is able to customize and create their own theme.
// For this, they are given two customization modes:
//      - Basic, which lets them choose the following:
//
//      - Advanced, which lets them choose more specific options (that being all of the options in theme)
// Themes can be overlayed to be more specific for specific panel or text elements,
pub struct Theme
{
    plain_font: Font,       // Plain body font
    header_font: Font,      // Header font

    text: Color,            // Plain body text
    text_accent: Color,     // Hyperlinks and icons 
    background: Color,      
    primary: Color,         // CTAs and buttons
    secondary: Color,       // Less important buttons and info cards
    accent: Color,          // Unique buttons and borders
}

impl Theme
{
    pub fn new() -> ThemeBuilder
    {
        todo!()
    }

    
}

pub struct ThemeBuilder
{
    basic_selection: bool,      // This begins true, but is set false anytime the 
}