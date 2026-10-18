//! The tokens as one CSS stylesheet, so a web UI reads the same values as Rust.

use std::fmt::Write as _;

use crate::motion::{
    BLUR_ENTER, BLUR_ENTER_LG, BLUR_EXIT, BLUR_EXIT_LG, DOCK_CLOSE, DURATIONS, EASE, EASE_OUT,
    INOUT, SOFT_CLOSE, spring_css,
};
use crate::tokens::{
    FONT_BODY, FONT_DISPLAY, LEAD_IN, MEASURE, Palette, RADIUS_CARD, RADIUS_MODULE, RADIUS_ROW,
    TYPE_SCALE, Theme,
};

fn colours(p: &Palette) -> String {
    let mut s = String::new();
    let mut put = |k: &str, v: String| {
        let _ = write!(s, "--{k}:{v};");
    };
    put("bg", p.bg.to_string());
    put("side", p.side.to_string());
    put("panel", p.panel.to_string());
    put("well", p.well.to_string());
    put("sheet", p.sheet.to_string());
    put("ink", p.ink.to_string());
    put("text", p.text.to_string());
    put("soft", p.soft.to_string());
    put("mute", p.mute.to_string());
    for (i, l) in p.line.iter().enumerate() {
        let k = if i == 0 {
            "line".to_owned()
        } else {
            format!("line-{}", i + 1)
        };
        put(&k, l.to_string());
    }
    put("gold", p.gold.to_string());
    put("gold-ink", p.gold_ink.to_string());
    put("gold-hi", p.gold_hi.to_string());
    put("gold-wash", p.gold_wash.to_string());
    put("sel", p.sel.to_string());
    for (k, c) in ["ok", "warn", "bad"].iter().zip(p.state) {
        put(k, c.to_string());
    }
    for (i, c) in p.series.iter().enumerate() {
        put(&format!("s{}", i + 1), c.to_string());
    }
    s
}

/// The full stylesheet: light on `:root`, dark for the system setting (unless `data-theme="light"`)
/// and for `data-theme="dark"`, and reduced motion.
#[must_use]
pub fn stylesheet() -> String {
    let d = DURATIONS;
    let mut root = String::from("color-scheme:light;");
    root.push_str(&colours(&Palette::of(Theme::Light)));
    let _ = write!(
        root,
        "--f-display:{FONT_DISPLAY};--f-body:{FONT_BODY};--r-card:{RADIUS_CARD}px;--r-module:{RADIUS_MODULE}px;--r-row:{RADIUS_ROW}px;--lead-in:{LEAD_IN}px;--measure:{MEASURE}px;"
    );
    for (k, v) in TYPE_SCALE {
        let _ = write!(root, "--fs-{k}:{v}px;");
    }
    let _ = write!(
        root,
        "--ease:{};--ease-dock-close:{};--ease-out:{};--ease-soft-close:{};--ease-inout:{};--spring:{};",
        EASE.css(),
        DOCK_CLOSE.css(),
        EASE_OUT.css(),
        SOFT_CLOSE.css(),
        INOUT.css(),
        spring_css(32)
    );
    let _ = write!(
        root,
        "--t-appear:{}ms;--t-leave:{}ms;--t-move:{}ms;--t-dock:{}ms;--t-dock-out:{}ms;--t-settle:{}ms;--t-instant:{}ms;--t-enter:{}ms;--t-exit:{}ms;--t-draw:{}ms;--stagger:{}ms;",
        d.appear,
        d.leave,
        d.r#move,
        d.dock,
        d.dock_out,
        d.settle,
        d.instant,
        d.enter,
        d.exit,
        d.draw,
        d.stagger
    );
    let _ = write!(
        root,
        "--blur-enter:{BLUR_ENTER}px;--blur-exit:{BLUR_EXIT}px;--blur-enter-lg:{BLUR_ENTER_LG}px;--blur-exit-lg:{BLUR_EXIT_LG}px;"
    );
    let dark = format!("color-scheme:dark;{}", colours(&Palette::of(Theme::Dark)));
    format!(
        ":root{{{root}}}\n\
         @media (prefers-color-scheme:dark){{:root:not([data-theme=\"light\"]){{{dark}}}}}\n\
         :root[data-theme=\"dark\"]{{{dark}}}\n\
         @media (prefers-reduced-motion:reduce){{:root{{--t-appear:1ms;--t-leave:1ms;--t-move:1ms;--t-dock:1ms;--t-dock-out:1ms;--t-settle:1ms;--t-enter:1ms;--t-exit:1ms;--t-draw:1ms;--stagger:0ms;--blur-enter:0px;--blur-exit:0px;--blur-enter-lg:0px;--blur-exit-lg:0px}}}}\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carries_both_themes_and_reduced_motion() {
        let css = stylesheet();
        assert!(css.contains("--bg:#FAF8F3;"));
        assert!(css.contains("--bg:#06090F;"));
        assert!(css.contains(":root[data-theme=\"dark\"]"));
        assert!(css.contains("prefers-reduced-motion"));
        assert!(css.contains("--ease:cubic-bezier(0.19,1,0.22,1);"));
        assert!(css.contains("--spring:linear(0, 0.029"));
        assert_eq!(css.matches('{').count(), css.matches('}').count());
    }
}
