//! EXPIRA design kernel.
//!
//! The house style of the EXPIRA Console (`expira-system/DESIGN_ENGINE.md`, v45) as Rust: every colour,
//! size and motion value a UI built on it may use, plus the curves that drive its motion. A UI layer
//! (web or native) reads from here and never hard-codes a value. Rules it carries:
//! - light and dark are designed separately, never inverted;
//! - every text token passes WCAG AA (4.5:1) on every surface (tested);
//! - nothing overshoots: one critically damped spring, monotonic curves;
//! - blur only on entry and exit, gone before the move lands, `none` at rest;
//! - reduced motion is a state change.

pub mod color;
pub mod css;
pub mod motion;
pub mod tokens;
