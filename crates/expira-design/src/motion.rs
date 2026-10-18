//! Motion: durations, curves, the spring, the focus blur and the bloom.
//!
//! v45 (`DESIGN_ENGINE.md` §4.2): one curve for everything (`EASE`), a travel curve for large moves
//! (`DOCK_CLOSE`), fast in and soft out. Nothing overshoots. Blur is gone before the move ends and is
//! `none` at rest. Reduced motion turns every duration to 1ms and every blur to 0.

/// A CSS `cubic-bezier(x1, y1, x2, y2)` timing curve.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bezier(pub f64, pub f64, pub f64, pub f64);

impl Bezier {
    /// Progress (0 to 1) at time `t` (0 to 1): solves x(s) = t, returns y(s).
    #[must_use]
    pub fn at(self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        let bz = |s: f64, p: f64, q: f64| {
            3.0 * p * s * (1.0 - s) * (1.0 - s) + 3.0 * q * s * s * (1.0 - s) + s * s * s
        };
        let (mut lo, mut hi) = (0.0, 1.0);
        for _ in 0..40 {
            let mid = f64::midpoint(lo, hi);
            if bz(mid, self.0, self.2) < t {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        bz(f64::midpoint(lo, hi), self.1, self.3)
    }

    /// The CSS form.
    #[must_use]
    pub fn css(self) -> String {
        format!("cubic-bezier({},{},{},{})", self.0, self.1, self.2, self.3)
    }
}

/// The one curve, entering and leaving alike: fast, then a long soft settle (`--sb-ease`).
pub const EASE: Bezier = Bezier(0.19, 1.0, 0.22, 1.0);
/// Travel curve for large moves: the dock closing, a resize settling (`--sb-dock-close`).
pub const DOCK_CLOSE: Bezier = Bezier(0.3, 0.85, 0.15, 1.0);
/// Gate A entry: fast start, long settle (`--ease-out`).
pub const EASE_OUT: Bezier = Bezier(0.16, 1.0, 0.3, 1.0);
/// Gate A soft close: travels, then decelerates into rest (`--ease-soft-close`).
pub const SOFT_CLOSE: Bezier = Bezier(0.4, 0.0, 0.1, 1.0);
/// Continuous travel between two held states (`--ease-inout`).
pub const INOUT: Bezier = Bezier(0.65, 0.0, 0.35, 1.0);

/// Durations in ms (v45 sidebar tokens, then Gate A).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Durations {
    /// Appear (`--sb-in`).
    pub appear: u32,
    /// Leave (`--sb-out`).
    pub leave: u32,
    /// Layout moves, the hover plate, one clock per change (`--sb-move`).
    pub r#move: u32,
    /// The dock opening (`--sb-dock`).
    pub dock: u32,
    /// The dock closing (`--sb-dock-out`).
    pub dock_out: u32,
    /// A resize settling (`--sb-settle`).
    pub settle: u32,
    /// Hover colour and opacity (`--t-instant`).
    pub instant: u32,
    /// Gate A small-surface entry (`--t-enter`).
    pub enter: u32,
    /// Gate A small-surface exit (`--t-exit`).
    pub exit: u32,
    /// Base for edges and chart pours, scaled ×1 to ×2.5 by length (`--t-draw`).
    pub draw: u32,
    /// Between queued siblings (`--stagger`).
    pub stagger: u32,
}

/// Full motion.
pub const DURATIONS: Durations = Durations {
    appear: 420,
    leave: 320,
    r#move: 460,
    dock: 560,
    dock_out: 720,
    settle: 760,
    instant: 90,
    enter: 160,
    exit: 280,
    draw: 420,
    stagger: 40,
};

/// Reduced motion: a state change.
pub const DURATIONS_REDUCED: Durations = Durations {
    appear: 1,
    leave: 1,
    r#move: 1,
    dock: 1,
    dock_out: 1,
    settle: 1,
    instant: 1,
    enter: 1,
    exit: 1,
    draw: 1,
    stagger: 0,
};

/// The critically damped spring (ζ = 1): x = 1 − e^(−ωt)(1 + ωt), ω = 8.4 over the unit,
/// pinned to 1 at the end. Its duration comes from the duration it is paired with.
#[must_use]
pub fn spring(t: f64) -> f64 {
    if t >= 1.0 {
        return 1.0;
    }
    let x = 8.4 * t.max(0.0);
    1.0 - (-x).exp() * (1.0 + x)
}

/// The spring sampled into a CSS `linear()` easing, `steps` segments.
#[must_use]
pub fn spring_css(steps: u32) -> String {
    let pts: Vec<String> = (0..=steps)
        .map(|i| {
            let v = spring(f64::from(i) / f64::from(steps));
            let s = format!("{v:.4}");
            s.trim_end_matches('0').trim_end_matches('.').to_owned()
        })
        .collect();
    format!("linear({})", pts.join(", "))
}

/// Focus blur in px at progress `p` (0 out of focus, 1 at rest): `max` at 0, easing as k² to none at
/// `gone_at` (0.6 for v45 moves, 0.7 for the bloom), so the landing is sharp.
#[must_use]
pub fn focus_blur(max: f64, p: f64, gone_at: f64) -> f64 {
    let k = (1.0 - p / gone_at).clamp(0.0, 1.0);
    if k < 0.01 { 0.0 } else { max * k * k }
}

/// Blur radii in px for the bloom: small surfaces blur whole, large ones only their content.
pub const BLUR_ENTER: f64 = 8.0;
/// Small-surface exit blur.
pub const BLUR_EXIT: f64 = 6.0;
/// Large-surface entry blur (content only).
pub const BLUR_ENTER_LG: f64 = 6.0;
/// Large-surface exit blur (content only).
pub const BLUR_EXIT_LG: f64 = 5.0;
/// Above this area (px², about 600 × 600) a surface counts as large.
pub const BIG_AREA: f64 = 360_000.0;

/// One frame of the bloom.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame {
    /// Opacity, 0 to 1.
    pub opacity: f64,
    /// Uniform scale.
    pub scale: f64,
    /// Downward offset in px.
    pub dy: f64,
    /// Blur in px (on the surface if small, on its content if large).
    pub blur: f64,
}

/// The bloom at open-progress `p` (0 closed, 1 open): fade up a little ahead, settle from a hair under
/// full size (0.965 small, 0.985 large), small surfaces drop 4px, blur gone by 70%.
#[must_use]
pub fn bloom(p: f64, big: bool, opening: bool) -> Frame {
    let p = p.clamp(0.0, 1.0);
    let s0 = if big { 0.985 } else { 0.965 };
    let b = match (big, opening) {
        (false, true) => BLUR_ENTER,
        (false, false) => BLUR_EXIT,
        (true, true) => BLUR_ENTER_LG,
        (true, false) => BLUR_EXIT_LG,
    };
    Frame {
        opacity: (p * 1.15).min(1.0),
        scale: s0 + (1.0 - s0) * p,
        dy: if big { 0.0 } else { (1.0 - p) * 4.0 },
        blur: focus_blur(b, p, 0.7),
    }
}

/// A move between open-progress `from` and `to`: its duration scales with the distance left, so a
/// reversal mid-flight carries on from where it is (opening is fast, closing soft).
#[must_use]
pub fn reversal_ms(from: f64, to: f64, d: &Durations) -> f64 {
    let base = if to > from { d.enter } else { d.exit };
    (f64::from(base) * (to - from).abs()).max(60.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOKEN_SPRING: [f64; 33] = [
        0.0, 0.029, 0.0979, 0.1867, 0.2826, 0.3776, 0.467, 0.5482, 0.6204, 0.6833, 0.7374, 0.7834,
        0.8222, 0.8546, 0.8815, 0.9037, 0.922, 0.937, 0.9492, 0.9591, 0.9672, 0.9737, 0.979,
        0.9832, 0.9866, 0.9893, 0.9915, 0.9932, 0.9946, 0.9957, 0.9966, 0.9973, 1.0,
    ];

    /// The Rust spring reproduces the console's `--spring` token.
    #[test]
    fn spring_matches_console_token() {
        for (i, &want) in (0_u32..).zip(TOKEN_SPRING.iter()) {
            let got = spring(f64::from(i) / 32.0);
            assert!((got - want).abs() < 0.002, "step {i}: {got} vs {want}");
        }
    }

    /// Nothing overshoots: every curve is monotonic and stays in 0..=1.
    #[test]
    fn nothing_overshoots() {
        for c in [EASE, DOCK_CLOSE, EASE_OUT, SOFT_CLOSE, INOUT] {
            let mut last = 0.0;
            for i in 0..=200 {
                let y = c.at(f64::from(i) / 200.0);
                assert!((0.0..=1.0 + 1e-9).contains(&y) && y + 1e-9 >= last, "{c:?}");
                last = y;
            }
        }
        let mut last = 0.0;
        for i in 0..=200 {
            let y = spring(f64::from(i) / 200.0);
            assert!(y <= 1.0 && y >= last);
            last = y;
        }
    }

    #[test]
    fn bezier_endpoints() {
        assert!(EASE.at(0.0).abs() < 1e-6 && (EASE.at(1.0) - 1.0).abs() < 1e-6);
    }

    /// Blur is none at rest and gone before the landing.
    #[test]
    fn blur_resolves_before_landing() {
        assert!((focus_blur(8.0, 0.0, 0.6) - 8.0).abs() < 1e-9);
        assert!(focus_blur(8.0, 0.6, 0.6).abs() < f64::EPSILON);
        assert!(bloom(1.0, false, true).blur.abs() < f64::EPSILON);
        assert!(bloom(0.7, true, true).blur.abs() < f64::EPSILON);
    }

    #[test]
    fn bloom_settles_from_a_hair_under() {
        let f = bloom(0.0, false, true);
        assert!((f.scale - 0.965).abs() < 1e-9 && f.opacity == 0.0 && (f.dy - 4.0).abs() < 1e-9);
        let f = bloom(1.0, true, false);
        assert!((f.scale - 1.0).abs() < 1e-9 && (f.opacity - 1.0).abs() < 1e-9 && f.dy == 0.0);
    }

    /// Fast in, slower soft close, both under the 400ms where motion reads as waiting.
    #[test]
    fn close_is_slower_than_open_and_under_400ms() {
        let d = DURATIONS;
        assert!(d.exit > d.enter && d.exit < 400 && d.leave < 400);
        assert!(reversal_ms(0.5, 1.0, &d) < reversal_ms(0.0, 1.0, &d));
        assert!((reversal_ms(1.0, 0.0, &d) - 280.0).abs() < 1e-9);
    }
}
