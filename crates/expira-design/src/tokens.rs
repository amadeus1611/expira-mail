//! Colour, type, space and shape tokens.
//!
//! Light is the Gate A paper palette (`console/src/tokens.css`, v41). Dark is the v45 near-black stack
//! built from the brand ink (`DESIGN_ENGINE.md` §4.3, `bench/drafts/dark-next.css`), which the canon says
//! replaces the v41 dark block. Brand stack: navy `#0B1A3F`, slate `#2C3549`, gold `#AE8A47` / `#C9A35C`,
//! ink `#09101E`.

use crate::color::{Rgba, hex, hexa};

/// Light or dark appearance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    /// Warm paper, navy ink.
    Light,
    /// Near black from the brand ink, off-white text.
    Dark,
}

/// Every colour role, for one theme.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Palette {
    /// Page.
    pub bg: Rgba,
    /// Sidebar: the darkest surface.
    pub side: Rgba,
    /// Cards, menus, sheets.
    pub panel: Rgba,
    /// Insets and wells.
    pub well: Rgba,
    /// Long-form reading surface.
    pub sheet: Rgba,
    /// Headings and numbers.
    pub ink: Rgba,
    /// Body text.
    pub text: Rgba,
    /// Secondary text.
    pub soft: Rgba,
    /// Captions and micro labels.
    pub mute: Rgba,
    /// Hairlines, three steps.
    pub line: [Rgba; 3],
    /// Gold for rules, lead-ins, marks and focus rings (never text in light).
    pub gold: Rgba,
    /// Gold used as text.
    pub gold_ink: Rgba,
    /// Highlights that pass over gold.
    pub gold_hi: Rgba,
    /// Soft gold wash behind a match.
    pub gold_wash: Rgba,
    /// Text selection: the only colour a resting row carries.
    pub sel: Rgba,
    /// Success, warning, failure; always paired with a glyph or a word.
    pub state: [Rgba; 3],
    /// Categorical data series, fixed order: research, finance, builder, legal, arbiter.
    pub series: [Rgba; 5],
}

impl Palette {
    /// The palette for `theme`.
    #[must_use]
    pub const fn of(theme: Theme) -> Palette {
        match theme {
            Theme::Light => LIGHT,
            Theme::Dark => DARK,
        }
    }

    /// Every surface text may sit on.
    #[must_use]
    pub const fn surfaces(&self) -> [Rgba; 5] {
        [self.bg, self.side, self.panel, self.well, self.sheet]
    }
}

const LIGHT: Palette = Palette {
    bg: hex(0xFA_F8F3),
    side: hex(0xF3_EFE4),
    panel: hex(0xFF_FFFF),
    well: hex(0xF4_F1E8),
    sheet: hex(0xFF_FEFB),
    ink: hex(0x0B_1A3F),
    text: hex(0x2B_303B),
    soft: hex(0x4F_5664),
    mute: hex(0x65_6B78),
    line: [
        hexa(0x0B_1A3F, 0.09),
        hexa(0x0B_1A3F, 0.16),
        hexa(0x0B_1A3F, 0.26),
    ],
    gold: hex(0xA8_862F),
    gold_ink: hex(0x84_6936),
    gold_hi: hex(0xC9_A35C),
    gold_wash: hexa(0xA8_862F, 0.08),
    sel: hexa(0xA8_862F, 0.26),
    state: [hex(0x2F_6B4F), hex(0x8A_6214), hex(0x8C_3A31)],
    series: [
        hex(0x34_599E),
        hex(0x9A_6D12),
        hex(0x0A_9188),
        hex(0xB0_532F),
        hex(0x8D_5AA6),
    ],
};

const DARK: Palette = Palette {
    bg: hex(0x06_090F),
    side: hex(0x04_060B),
    panel: hex(0x0C_111B),
    well: hex(0x0A_0E17),
    sheet: hex(0x09_0D15),
    ink: hex(0xF7_F5F0),
    text: hex(0xF7_F5F0),
    soft: hex(0xA9_ADB5),
    mute: hex(0x7D_838E),
    line: [
        hexa(0xF7_F5F0, 0.055),
        hexa(0xF7_F5F0, 0.10),
        hexa(0xF7_F5F0, 0.19),
    ],
    gold: hex(0xC9_A35C),
    gold_ink: hex(0xC9_A35C),
    gold_hi: hex(0xE9_CF91),
    gold_wash: hexa(0xC9_A35C, 0.09),
    sel: hexa(0xC9_A35C, 0.32),
    // v41 dark state and series values; the v45 draft does not restate them.
    state: [hex(0x8F_C3A6), hex(0xE0_B765), hex(0xE5_9A8F)],
    series: [
        hex(0x5B_7FD0),
        hex(0xB0_8A3A),
        hex(0x23_998F),
        hex(0xC9_6A48),
        hex(0xA4_7BC0),
    ],
};

/// Display serif (headings, accents in italic).
pub const FONT_DISPLAY: &str =
    r#""Libre Baskerville","Hoefler Text",Georgia,"Times New Roman",serif"#;
/// Body sans.
pub const FONT_BODY: &str =
    r#""Inter",-apple-system,BlinkMacSystemFont,"Segoe UI","Helvetica Neue",Arial,sans-serif"#;

/// Type scale in px: micro (the floor), chrome, chrome-2, ui, body, lead, h3, h2, h1, hero.
pub const TYPE_SCALE: [(&str, f64); 10] = [
    ("micro", 9.5),
    ("chrome", 10.5),
    ("chrome-2", 11.0),
    ("ui", 12.0),
    ("body", 13.0),
    ("lead", 14.5),
    ("h3", 16.0),
    ("h2", 20.0),
    ("h1", 28.0),
    ("hero", 40.0),
];

/// Nothing is set smaller than this, in px.
pub const TYPE_FLOOR: f64 = 9.5;

/// Space scale in px: a 4px base with half steps below 20px for dense chrome.
pub const SPACE: [f64; 17] = [
    2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0, 20.0, 24.0, 28.0, 32.0, 36.0, 40.0, 44.0,
    48.0,
];

/// Corners (v45): floating cards and the selection bar.
pub const RADIUS_CARD: f64 = 6.0;
/// Corners (v45): modules (tiles, search field, account row, back and forward).
pub const RADIUS_MODULE: f64 = 4.0;
/// Corners (v45): rows and small controls.
pub const RADIUS_ROW: f64 = 3.0;
/// Gold lead-in on a hairline rule, px.
pub const LEAD_IN: f64 = 34.0;
/// Reading measure of the text column, px.
pub const MEASURE: f64 = 520.0;

#[cfg(test)]
mod tests {
    use super::*;

    /// The canon: all text tokens pass AA (4.5:1) on every surface in both themes.
    #[test]
    fn text_is_aa_on_every_surface() {
        for theme in [Theme::Light, Theme::Dark] {
            let p = Palette::of(theme);
            for bed in p.surfaces() {
                for (name, ink) in [
                    ("ink", p.ink),
                    ("text", p.text),
                    ("soft", p.soft),
                    ("mute", p.mute),
                    ("gold_ink", p.gold_ink),
                ] {
                    let r = ink.contrast_on(bed);
                    assert!(r >= 4.5, "{theme:?} {name} {r:.2} on {bed}");
                }
            }
        }
    }

    #[test]
    fn type_scale_respects_floor() {
        assert!(TYPE_SCALE.iter().all(|&(_, s)| s >= TYPE_FLOOR));
    }
}
