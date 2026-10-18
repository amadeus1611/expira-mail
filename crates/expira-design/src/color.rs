//! Colours and WCAG contrast.

use std::fmt;

/// An sRGB colour with straight alpha (0 to 1).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba {
    /// Red, 0 to 255.
    pub r: u8,
    /// Green, 0 to 255.
    pub g: u8,
    /// Blue, 0 to 255.
    pub b: u8,
    /// Opacity, 0 to 1.
    pub a: f64,
}

/// An opaque colour from `0xRRGGBB`.
#[must_use]
pub const fn hex(v: u32) -> Rgba {
    Rgba {
        r: (v >> 16 & 0xFF) as u8,
        g: (v >> 8 & 0xFF) as u8,
        b: (v & 0xFF) as u8,
        a: 1.0,
    }
}

/// A translucent colour from `0xRRGGBB` and an opacity.
#[must_use]
pub const fn hexa(v: u32, a: f64) -> Rgba {
    let c = hex(v);
    Rgba { a, ..c }
}

impl Rgba {
    /// This colour composited over an opaque `bed`.
    #[must_use]
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "rounded and clamped to 0..=255 first"
    )]
    pub fn over(self, bed: Rgba) -> Rgba {
        let mix = |f: u8, b: u8| {
            (f64::from(f) * self.a + f64::from(b) * (1.0 - self.a))
                .round()
                .clamp(0.0, 255.0) as u8
        };
        Rgba {
            r: mix(self.r, bed.r),
            g: mix(self.g, bed.g),
            b: mix(self.b, bed.b),
            a: 1.0,
        }
    }

    /// WCAG 2 relative luminance of the opaque colour.
    #[must_use]
    pub fn luminance(self) -> f64 {
        let lin = |c: u8| {
            let s = f64::from(c) / 255.0;
            if s <= 0.040_45 {
                s / 12.92
            } else {
                ((s + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * lin(self.r) + 0.7152 * lin(self.g) + 0.0722 * lin(self.b)
    }

    /// WCAG 2 contrast ratio between this colour (composited over `bed`) and `bed`.
    #[must_use]
    pub fn contrast_on(self, bed: Rgba) -> f64 {
        let (a, b) = (self.over(bed).luminance(), bed.luminance());
        (a.max(b) + 0.05) / (a.min(b) + 0.05)
    }
}

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.a >= 1.0 {
            write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        } else {
            write!(f, "rgba({},{},{},{})", self.r, self.g, self.b, self.a)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contrast_matches_wcag_extremes() {
        let r = hex(0x00_0000).contrast_on(hex(0xFF_FFFF));
        assert!((r - 21.0).abs() < 1e-9);
        assert!((hex(0x77_7777).contrast_on(hex(0x77_7777)) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn prints_as_css() {
        assert_eq!(hex(0x0B_1A3F).to_string(), "#0B1A3F");
        assert_eq!(hexa(0x0B_1A3F, 0.09).to_string(), "rgba(11,26,63,0.09)");
    }
}
