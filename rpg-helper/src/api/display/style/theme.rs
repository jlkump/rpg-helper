use once_cell::sync::Lazy;

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
    pub plain_font: Font,       // Plain body font
    pub header_font: Font,      // Header font

    // Simple color variants
    pub text: Color,            // Plain body text
    pub text_accent: Color,     // Hyperlinks and icons 
    pub background: Color,      
    pub primary: Color,         // CTAs and buttons
    pub secondary: Color,       // Less important buttons and info cards
    pub accent: Color,          // Unique buttons and borders

    // Advanced color variants
    pub text_minor_faint: Color,
    pub text_medium_faint: Color,
    pub text_major_faint: Color,
    pub text_max_faint: Color,

    pub secondary_50: Color,
}

impl Theme
{
    pub fn new() -> ThemeBuilder
    {
        todo!()
    }

    pub fn default_light() -> &'static Theme
    {
        static DEFAULT_LIGHT_THEME: Lazy<Theme> = Lazy::new(||
            Theme
            {
                plain_font: Font::Winky,
                header_font: Font::Macondo,

                text: Color::rgba(36, 34, 34, 1.0),
                text_accent: Color::hex(0xd39218FF),
                background: Color::rgb(236, 233, 228),
                primary: Color::rgb(111, 35, 32),
                secondary: Color::rgb(203, 195, 179),
                accent: Color::rgb(225, 165, 81),

                text_minor_faint: Color::rgba(58, 54, 54, 0.75),
                text_medium_faint: Color::rgba(58, 54, 54, 0.60),
                text_major_faint: Color::rgba(58, 54, 54, 0.50),
                text_max_faint: Color::rgba(58, 54, 54, 0.35),

                secondary_50: Color::rgb(222, 214, 204)
            }
        );

        &*DEFAULT_LIGHT_THEME
    }
}

pub struct ThemeBuilder
{
}