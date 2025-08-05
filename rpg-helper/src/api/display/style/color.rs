use serde::{Deserialize, Serialize};

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
}