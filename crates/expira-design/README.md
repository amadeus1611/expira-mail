# expira-design

The EXPIRA Console design kernel in Rust: the base every expira-mail UI builds on. It is written fresh from the canon in `amadeus1611/GSSC-Integrated-Corporation`, branch `claude/gifted-mendel-3orlc0`:
- `expira-system/DESIGN_ENGINE.md` (v45);
- `console/src/tokens.css`, `core/motion.js` and `core/pour.js`;
- `bench/drafts/sidebar-next.css` and `dark-next.css`.

| Module | Holds |
|---|---|
| `tokens` | Light (Gate A paper) and dark (v45 near-black) palettes, fonts, type scale (9.5px floor), space, corners (6/4/3px), lead-in, measure |
| `motion` | `EASE`, `DOCK_CLOSE` and the Gate A curves; v45 durations and reduced motion; the critically damped spring; focus blur; the bloom |
| `color` | sRGB, compositing, WCAG contrast |
| `css` | One stylesheet built from the above: light, dark by system or `data-theme`, reduced motion |

Tests hold the canon to its own claims:
- every text token is AA on every surface in both themes;
- nothing overshoots;
- the spring matches the console's `--spring` token to within 0.002;
- blur is gone before a surface lands;
- close is slower than open, and both are under 400ms.

## Open
- **Steel-grey captions on a selected row in dark measure about 3.6:1, which is under AA (4.5).** The selected row is `color-mix(gold 9%, line-2)` over the sidebar. The canon claims AA only for a hovered row. This needs an owner decision: lighten `--mute` on selected rows, or accept it.
- The v45 dark draft does not restate state or series colours, so the v41 dark values are kept.
