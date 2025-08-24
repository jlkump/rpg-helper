use once_cell::sync::Lazy;

use serde::{Deserialize, Serialize};

use crate::api::display::style::{color::Color, font::Font};

// A theme is used to color the client
// A user is able to customize and create their own theme.
// For this, they are given two customization modes:
//      - Basic, which lets them choose the following:
//
//      - Advanced, which lets them choose more specific options (that being all of the options in theme)
// Themes can be overlayed to be more specific for specific panel or text elements,
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Theme
{
    pub plain_font: Font,       // Plain body font
    pub header_font: Font,      // Header font

    // Main color choices
    pub background: Color,      // Background color used for most of the page
    pub paper: Color,           // The layered color ontop of background for most page elements
    pub primary: Color,         // CTAs and buttons, main headers
    pub secondary: Color,       // Less important buttons and info cards, secondary headers
    pub tertiary: Color,        // Tertiary headers
    pub accent: Color,          // Links and tags default

    pub text_default: Color,    // Plain body text
    pub text_primary: Color,    // Text ontop of a primary bg
    pub text_secondary: Color,  // Text ontop of a secondary bg
    pub text_tertiary: Color,  // Text ontop of a secondary bg
    pub text_accent: Color,     // Text ontop of an accent bg

    // Advanced color variants, blends with the background color
    pub paper_75: Color,
    pub paper_50: Color,
    pub paper_25: Color,
    pub primary_75: Color,
    pub primary_50: Color,
    pub primary_25: Color,
    pub secondary_75: Color,
    pub secondary_50: Color,
    pub secondary_25: Color,
    pub tertiary_75: Color,
    pub tertiary_50: Color,
    pub tertiary_25: Color,
    pub accent_75: Color,
    pub accent_50: Color,
    pub accent_25: Color,

    pub text_default_75: Color,
    pub text_default_50: Color,
    pub text_default_25: Color,
    pub text_primary_75: Color,
    pub text_primary_50: Color,
    pub text_primary_25: Color,
    pub text_secondary_75: Color,
    pub text_secondary_50: Color,
    pub text_secondary_25: Color,
    pub text_tertiary_75: Color,
    pub text_tertiary_50: Color,
    pub text_tertiary_25: Color,
    pub text_accent_75: Color,
    pub text_accent_50: Color,
    pub text_accent_25: Color,
}

impl Theme
{
    pub fn new() -> Theme
    {
        Self::default_light().clone()
    }

    /// Calculates the bended variants
    /// of colors based on the assigned 
    /// core colors.
    /// - background
    /// - paper
    /// - primary
    /// - secondary
    /// - tertiary
    /// - accent
    /// - text_default
    pub fn calculate_blends(&mut self)
    {
        self.paper_75 = Color::blend_with(&self.paper, &self.background, 0.75);
        self.paper_50 = Color::blend_with(&self.paper, &self.background, 0.50);
        self.paper_25 = Color::blend_with(&self.paper, &self.background, 0.25);
        self.primary_75 = Color::blend_with(&self.primary, &self.background, 0.75);
        self.primary_50 = Color::blend_with(&self.primary, &self.background, 0.50);
        self.primary_25 = Color::blend_with(&self.primary, &self.background, 0.25);
        self.secondary_75 = Color::blend_with(&self.secondary, &self.background, 0.75);
        self.secondary_50 = Color::blend_with(&self.secondary, &self.background, 0.50);
        self.secondary_25 = Color::blend_with(&self.secondary, &self.background, 0.25);
        self.tertiary_75 = Color::blend_with(&self.tertiary, &self.background, 0.75);
        self.tertiary_50 = Color::blend_with(&self.tertiary, &self.background, 0.50);
        self.tertiary_25 = Color::blend_with(&self.tertiary, &self.background, 0.25);
        self.accent_75 = Color::blend_with(&self.accent, &self.background, 0.75);
        self.accent_50 = Color::blend_with(&self.accent, &self.background, 0.50);
        self.accent_25 = Color::blend_with(&self.accent, &self.background, 0.25);

        self.text_default_75 = Color::blend_with(&self.text_default, &self.paper, 0.75);
        self.text_default_50 = Color::blend_with(&self.text_default, &self.paper, 0.50);
        self.text_default_25 = Color::blend_with(&self.text_default, &self.paper, 0.25);
        self.text_primary_75 = Color::blend_with(&self.text_primary, &self.primary, 0.75);
        self.text_primary_50 = Color::blend_with(&self.text_primary, &self.primary, 0.50);
        self.text_primary_25 = Color::blend_with(&self.text_primary, &self.primary, 0.25);
        self.text_secondary_75 = Color::blend_with(&self.text_secondary, &self.secondary, 0.75);
        self.text_secondary_50 = Color::blend_with(&self.text_secondary, &self.secondary, 0.50);
        self.text_secondary_25 = Color::blend_with(&self.text_secondary, &self.secondary, 0.25);
        self.text_tertiary_75 = Color::blend_with(&self.text_tertiary, &self.tertiary, 0.75);
        self.text_tertiary_50 = Color::blend_with(&self.text_tertiary, &self.tertiary, 0.50);
        self.text_tertiary_25 = Color::blend_with(&self.text_tertiary, &self.tertiary, 0.25);
        self.text_accent_75 = Color::blend_with(&self.text_accent, &self.accent, 0.75);
        self.text_accent_50 = Color::blend_with(&self.text_accent, &self.accent, 0.50);
        self.text_accent_25 = Color::blend_with(&self.text_accent, &self.accent, 0.25);
    }

    pub fn default_light() -> &'static Theme
    {
        static DEFAULT_LIGHT_THEME: Lazy<Theme> = Lazy::new(||
            {
                let mut t = Theme
                {
    
                    background: Color::rgb(236, 233, 228),
                    paper: Color::rgb(203, 195, 179),
                    primary: Color::rgb(111, 35, 32),
                    secondary: Color::rgb(150, 97, 0),
                    // secondary: Color::rgb(211, 146, 24),
                    tertiary: Color::rgb(19, 38, 75),
                    accent: Color::rgb(57, 131, 120),
    
                    plain_font: Font::Winky,
                    header_font: Font::Macondo,
                    text_default: Color::rgba(36, 34, 34, 1.0),
                    text_primary: Color::rgb(211, 146, 24),
                    text_secondary: Color::rgb(19, 38, 75),
                    text_tertiary: Color::rgb(211, 146, 24),
                    text_accent: Color::rgba(36, 34, 34, 1.0),
                    
                    paper_75: Color::default(),
                    paper_50: Color::default(),
                    paper_25: Color::default(),
                    primary_75: Color::default(),
                    primary_50: Color::default(),
                    primary_25: Color::default(),
                    secondary_75: Color::default(),
                    secondary_50: Color::default(),
                    secondary_25: Color::default(),
                    tertiary_75: Color::default(),
                    tertiary_50: Color::default(),
                    tertiary_25: Color::default(),
                    accent_75: Color::default(),
                    accent_50: Color::default(),
                    accent_25: Color::default(),
    
                    text_default_75: Color::default(),
                    text_default_50: Color::default(),
                    text_default_25: Color::default(),
                    text_primary_75: Color::default(),
                    text_primary_50: Color::default(),
                    text_primary_25: Color::default(),
                    text_secondary_75: Color::default(),
                    text_secondary_50: Color::default(),
                    text_secondary_25: Color::default(),
                    text_tertiary_75: Color::default(),
                    text_tertiary_50: Color::default(),
                    text_tertiary_25: Color::default(),
                    text_accent_75: Color::default(),
                    text_accent_50: Color::default(),
                    text_accent_25: Color::default(),
                };
                t.calculate_blends();
                t
            }
        );

        &*DEFAULT_LIGHT_THEME
    }
}