use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::api::display::error::DisplayError;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum Color
{
    RGB { r: u8, g: u8, b: u8 },
    RGBA { r: u8, g: u8, b: u8, a: f32 },
    HSL { h: f32, s: f32, l: f32 },
    HSLA { h: f32, s: f32, l: f32, a: f32 },
    HEX(u32),
}

impl Color
{
    pub fn rgb(r: u8, g: u8, b: u8) -> Color
    {
        Color::RGB { r, g, b }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color
    {
        Color::RGBA { r, g, b, a }
    }

    pub fn hex(hex: u32) -> Color
    {
        Color::HEX(hex)
    }

    pub fn hsl(h: f32, s: f32, l: f32) -> Self
    {
        Color::HSL
        {
            h: h.rem_euclid(360.0),
            s: s.clamp(0.0, 1.0),
            l: l.clamp(0.0, 1.0),
        }
    }

    pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Self
    {
        Color::HSLA
        {
            h: h.rem_euclid(360.0),
            s: s.clamp(0.0, 1.0),
            l: l.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        }
    }

    /// Convert to RGB values
    pub fn to_rgb(&self) -> (u8, u8, u8)
    {
        match *self
        {
            Color::RGB { r, g, b } | Color::RGBA { r, g, b, .. } => (r, g, b),
            Color::HSL { h, s, l } | Color::HSLA { h, s, l, .. } => hsl_to_rgb(h, s, l),
            Color::HEX(hex) => 
            {
                let (r, g, b, _) = hex_to_rgba(hex);
                (r, g, b)
            },
        }
    }

    /// Convert to RGBA values
    pub fn to_rgba(&self) -> (u8, u8, u8, f32)
    {
        match *self
        {
            Color::RGB { r, g, b } => (r, g, b, 1.0),
            Color::RGBA { r, g, b, a } => (r, g, b, a),
            Color::HSL { h, s, l } => 
            {
                let (r, g, b) = hsl_to_rgb(h, s, l);
                (r, g, b, 1.0)
            }
            Color::HSLA { h, s, l, a } => 
            {
                let (r, g, b) = hsl_to_rgb(h, s, l);
                (r, g, b, a)
            },
            Color::HEX(hex) => hex_to_rgba(hex),
        }
    }

    /// Convert to HSL values
    pub fn to_hsl(&self) -> (f32, f32, f32)
    {
        let (r, g, b) = self.to_rgb();
        rgb_to_hsl(r, g, b)
    }

    /// Convert to HSLA values
    pub fn to_hsla(&self) -> (f32, f32, f32, f32)
    {
        let (h, s, l) = self.to_hsl();
        let a = self.alpha();
        (h, s, l, a)
    }

    pub fn to_hex(&self) -> u32
    {
        if let Color::HEX(hex) = *self
        {
            hex
        }
        else
        {
            rgba_to_hex(self.to_rgba())
        }
    }

    pub fn into_rgb(self) -> Self
    {
        let (r, g, b) = self.to_rgb();
        Self::rgb(r, g, b)
    }

    pub fn into_rgba(self) -> Self
    {
        let (r, g, b, a) = self.to_rgba();
        Self::rgba(r, g, b, a)
    }

    pub fn into_hsl(self) -> Self
    {
        let (h, s, l) = self.to_hsl();
        Self::hsl(h, s, l)
    }

    pub fn into_hsla(self) -> Self
    {
        let (h, s, l, a) = self.to_hsla();
        Self::hsla(h, s, l, a)
    }

    pub fn into_hex(self) -> Self
    {
        let hex = self.to_hex();
        Self::hex(hex)
    }

    /// Get the alpha channel value
    pub fn alpha(&self) -> f32
    {
        match *self
        {
            Color::RGBA { a, .. } | Color::HSLA { a, .. } => a,
            Color::HEX(hex) =>
            {
                let (_, _, _, a) = hex_to_rgba(hex);
                a
            }
            _ => 1.0,
        }
    }

    /// Blends the color of self with other with the following equation:
    /// self * alpha + other * (1.0 - alpha).
    /// 
    /// Expects alpha to be [0.0, 1.0]
    pub fn blend_with(&self, other: &Self, alpha: f32) -> Self
    {
        let (l_r, l_g, l_b) = self.to_rgb();
        let (l_r, l_g, l_b) = (l_r as f32, l_g as f32, l_b as f32);
        let (r_r, r_g, r_b) = other.to_rgb();
        let (r_r, r_g, r_b) = (r_r as f32, r_g as f32, r_b as f32);
        let minus_alpha = 1.0 - alpha;
        let r = (l_r * alpha + r_r * minus_alpha).round() as u8;
        let g = (l_g * alpha + r_g * minus_alpha).round() as u8;
        let b = (l_b * alpha + r_b * minus_alpha).round() as u8;
        Color::rgb(r, g, b)
    }

    pub fn from_str(s: &str) -> Result<Color, DisplayError>
    {
        let s = s.trim();
        
        // Handle hex colors
        if s.starts_with('#')
        {
            let hex_str = &s[1..];
            
            // Parse hex string to u32
            let hex_value = match hex_str.len()
            {
                6 =>
                {
                    // RGB format - add full alpha
                    u32::from_str_radix(hex_str, 16)
                        .map(|v| (v << 8) | 0xFF)
                        .map_err(|_| DisplayError::InvalidColorFormat(s.to_string()))?
                }
                8 =>
                {
                    // RGBA format
                    u32::from_str_radix(hex_str, 16)
                        .map_err(|_| DisplayError::InvalidColorFormat(s.to_string()))?
                }
                _ => return Err(DisplayError::InvalidColorFormat(s.to_string()))
            };
            
            return Ok(Color::HEX(hex_value));
        }
        
        // Handle function notation (rgb, rgba, hsl, hsla)
        if let Some(paren_pos) = s.find('(')
        {
            let func_name = &s[..paren_pos];
            
            // Check for closing parenthesis
            if !s.ends_with(')')
            {
                return Err(DisplayError::InvalidColorFormat(s.to_string()));
            }
            
            // Extract values between parentheses
            let values_str = &s[paren_pos + 1..s.len() - 1];
            let values: Vec<&str> = values_str.split(',').map(|v| v.trim()).collect();
            
            match func_name
            {
                "rgb" if values.len() == 3 =>
                {
                    let r = values[0].parse::<u8>()
                        .map_err(|_| DisplayError::InvalidColorFormat(s.to_string()))?;
                    let g = values[1].parse::<u8>()
                        .map_err(|_| DisplayError::InvalidColorFormat(s.to_string()))?;
                    let b = values[2].parse::<u8>()
                        .map_err(|_| DisplayError::InvalidColorFormat(s.to_string()))?;
                    Ok(Color::RGB { r, g, b })
                }
                "rgba" if values.len() == 4 =>
                {
                    let r = values[0].parse::<u8>()
                        .map_err(|_| DisplayError::InvalidColorFormat(s.to_string()))?;
                    let g = values[1].parse::<u8>()
                        .map_err(|_| DisplayError::InvalidColorFormat(s.to_string()))?;
                    let b = values[2].parse::<u8>()
                        .map_err(|_| DisplayError::InvalidColorFormat(s.to_string()))?;
                    let a = values[3].parse::<f32>()
                        .map_err(|_| DisplayError::InvalidColorFormat(s.to_string()))?;
                    Ok(Color::RGBA { r, g, b, a })
                }
                "hsl" if values.len() == 3 =>
                {
                    let h = parse_hsl_value(values[0], false)?;
                    let s = parse_hsl_value(values[1], true)?;
                    let l = parse_hsl_value(values[2], true)?;
                    Ok(Color::HSL { h, s, l })
                }
                "hsla" if values.len() == 4 =>
                {
                    let h = parse_hsl_value(values[0], false)?;
                    let s = parse_hsl_value(values[1], true)?;
                    let l = parse_hsl_value(values[2], true)?;
                    let a = values[3].parse::<f32>()
                        .map_err(|_| DisplayError::InvalidColorFormat(s.to_string()))?;
                    Ok(Color::HSLA { h, s, l, a })
                }
                _ => Err(DisplayError::InvalidColorFormat(s.to_string()))
            }
        }
        else
        {
            Err(DisplayError::InvalidColorFormat(s.to_string()))
        }
    }

    pub fn to_string(&self) -> String
    {
        match *self
        {
            Color::RGB { r, g, b } => format!("rgb({}, {}, {})", r, g, b),
            Color::RGBA { r, g, b, a } => format!("rgba({}, {}, {}, {})", r, g, b, a),
            Color::HSL { h, s, l } => format!("hsl({}, {}, {})", h, s, l),
            Color::HSLA { h, s, l, a } => format!("hlsa({}, {}, {}, {})", h, s, l, a),
            Color::HEX(hex) =>
            {
                let (r, g, b) = self.to_rgb();
                format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, hex & 0xFF)
            },
        }
    }
}

impl Display for Color
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.to_string())
    }
}

impl Default for Color
{
    fn default() -> Self
    {
        Color::rgb(255, 255, 255)
    }
}

// Helper function to parse HSL values (handles both percentage and decimal notation)
fn parse_hsl_value(s: &str, is_percentage: bool) -> Result<f32, DisplayError> {
    let s = s.trim();
    
    if is_percentage && s.ends_with('%') {
        // Parse percentage
        let value_str = &s[..s.len() - 1];
        value_str.parse::<f32>()
            .map(|v| v / 100.0)
            .map_err(|_| DisplayError::InvalidColorFormat(s.to_string()))
    } else {
        // Parse as direct value
        // For hue: it's in degrees (0-360)
        // For saturation/lightness: it's 0-1 if not percentage
        s.parse::<f32>()
            .map_err(|_| DisplayError::InvalidColorFormat(s.to_string()))
    }
}

// Helper functions for color space conversions
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    )
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let l = (max + min) / 2.0;

    if delta == 0.0 {
        return (0.0, 0.0, l);
    }

    let s = if l < 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };

    let h = if max == r {
        ((g - b) / delta + if g < b { 6.0 } else { 0.0 }) * 60.0
    } else if max == g {
        ((b - r) / delta + 2.0) * 60.0
    } else {
        ((r - g) / delta + 4.0) * 60.0
    };

    (h, s, l)
}

fn hex_to_rgba(hex: u32) -> (u8, u8, u8, f32)
{
    let r = ((hex >> 24) & 0xFF) as u8;
    let g = ((hex >> 16) & 0xFF) as u8;
    let b = ((hex >> 8) & 0xFF) as u8;
    let a = ((hex & 0xFF) as u8) as f32;
    (r, g, b, (a / 255.0))
}

fn rgba_to_hex(rgba: (u8, u8, u8, f32)) -> u32
{
    ((rgba.0 as u32) << 24) + ((rgba.1 as u32) << 16) + ((rgba.2 as u32) << 8) + ((rgba.3 * 255.0) as u32)
}

#[cfg(test)]
mod unit_tests
{
    use super::*;

    #[test]
    fn color_conversion_1()
    {
        let c = Color::rgba(25, 53, 45, 1.0);
        let (r, g, b) = c.to_rgb();
        assert_eq!(r, 25);
        assert_eq!(g, 53);
        assert_eq!(b, 45);
    }

    #[test]
    fn color_conversion_2()
    {
        let c = Color::hex(0xFF00FFFF);
        let (r, g, b) = c.to_rgb();
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 255);
    }

    #[test]
    fn color_conversion_3()
    {
        let c = Color::hex(0xFF00FFFF);
        let (r, g, b, a) = c.to_rgba();
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 255);
        assert_eq!(a, 1.0);
    }

    #[test]
    fn color_conversion_4()
    {
        let c = Color::hex(0xFF00FF00);
        let (r, g, b, a) = c.to_rgba();
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 255);
        assert_eq!(a, 0.0);
    }

    #[test]
    fn color_conversion_5()
    {
        let c = Color::rgba(255, 255, 255, 0.0);
        let hex = c.to_hex();
        assert_eq!(hex, 0xFFFFFF00);
    }

    #[test]
    fn color_string_1()
    {
        let c = Color::rgba(255, 255, 255, 1.0);
        assert_eq!(c.into_hex().to_string(), "#ffffffff");
    }
    
    #[test]
    fn color_string_2()
    {
        let c = Color::rgba(255, 255, 255, 0.0);
        assert_eq!(c.into_hex().to_string(), "#ffffff00");
    }

    #[test]
    fn parse_color_1()
    {
        let c = Color::from_str("rgb(124, 245, 0)").unwrap();
        assert_eq!(c, Color::rgb(124, 245, 0));
    }

    #[test]
    fn parse_color_2()
    {
        let c = Color::from_str("rgb(256, 245, 0)");
        assert!(c.is_err());
    }

    #[test]
    fn parse_color_3()
    {
        let c = Color::from_str("rgb(1.0, 0.5, 0)");
        assert!(c.is_err());
    }

    #[test]
    fn parse_color_4()
    {
        let c = Color::from_str("rgba(245, 55, 234, 0.5)").unwrap();
        assert_eq!(c, Color::rgba(245, 55, 234, 0.5));
    }

    #[test]
    fn parse_color_5()
    {
        let c = Color::from_str("hsl(56, 0.5, 0.5)").unwrap();
        assert_eq!(c, Color::hsl(56.0, 0.5, 0.5));
    }

    #[test]
    fn parse_color_6()
    {
        let c = Color::from_str("hsla(56, 0.5, 0.5, 0.6)").unwrap();
        assert_eq!(c, Color::hsla(56.0, 0.5, 0.5, 0.6));
    }

    #[test]
    fn parse_color_7()
    {
        let c = Color::from_str("#FF3455FF").unwrap();
        assert_eq!(c, Color::hex(0xFF3455FF));
    }

    #[test]
    fn parse_color_8()
    {
        let c = Color::from_str("#FF3455").unwrap();
        assert_eq!(c, Color::hex(0xFF3455FF));
    }
}