# ESO Weave Brand Standard v1

This is the authoritative reference for the ESO Weave visual identity. The
application theme and every brand asset trace to the tokens and rules below. It was
established by build slice 012 (brand and UX polish).

Direction: "Arcane gold on ink." Near-black ink surfaces with a warm gold as the
primary action color, honoring the Elder Scrolls Online heritage while reading as a
modern, clean tool rather than antique decoration. Teal is a supporting accent.

## The mark

The mark is two interlocking carets, an upward caret and a downward caret, woven
over and under: a gold strand and a teal strand working together. It sits on a
self-contained rounded ink badge so it stays legible at 16px on any background
(taskbar, wallpaper, light or dark theme). It replaces the antique two-fish gold
ouroboros.

Masters live in `assets/brand/`:

- `eso-weave-mark.svg` - badged mark (the icon everywhere).
- `eso-weave-glyph.svg` - carets only, transparent, for lockups on ink.

Rules:

- Do not recolor the strands outside the palette below. Gold is the primary
  strand; teal is the secondary strand.
- Do not place the badge-less glyph on light backgrounds (the gold strand loses
  contrast). Use the badged mark on light surfaces.
- The wordmark is "ESO" in text color followed by "Weave" in gold, set in Inter
  SemiBold with light tracking.

## Color tokens

Colors are named by role. The application theme maps these to both the dark
(default) and light modes.

### Dark (default)

| Role | Hex | Use |
|------|-----|-----|
| Ink base | `#0E1116` | Window and base surface |
| Panel | `#151B23` | Panels, control fills |
| Elevated | `#1C2530` | Hover and active fills |
| Stroke | `#2A3340` | Borders, separators |
| Gold (action) | `#F2B03C` | Primary buttons, active toggles, wordmark accent |
| Gold hover | `#FBCB6B` | Gold hover and active |
| Gold deep | `#D18F22` | Gold pressed, borders on gold |
| Teal (support) | `#2DD4BF` | Secondary accent, mark, info highlights |
| Text | `#E6EDF3` | Primary text |
| Muted | `#8B97A7` | Labels, secondary text |
| Status ok | `#34D399` | Running, healthy |
| Status warn | `#FB9E3C` | Warnings |
| Status err | `#F87171` | Errors, signal lost |

### Light

| Role | Hex | Use |
|------|-----|-----|
| Base | `#F7F5F0` | Window and base surface |
| Panel | `#FFFFFF` | Panels, control fills |
| Elevated | `#ECE8DE` | Hover and active fills |
| Stroke | `#DCD9D0` | Borders, separators |
| Gold (action) | `#E7A42C` | Primary buttons, active toggles |
| Gold deep | `#C6871F` | Gold text and borders on light |
| Teal (support) | `#0D9488` | Secondary accent, info highlights |
| Text | `#14110B` | Primary text |
| Muted | `#6B6455` | Labels, secondary text |
| Status ok | `#059669` | Running, healthy |
| Status warn | `#B45309` | Warnings |
| Status err | `#DC2626` | Errors, signal lost |

On filled gold buttons, text is near-ink (`#241704`) in both themes for contrast.

## Typography

- Primary UI typeface: Inter, SIL Open Font License 1.1, bundled with the
  application under `assets/brand/fonts/` (with `OFL.txt`). Regular for body,
  SemiBold for the wordmark and emphasis.
- The application registers Inter as the proportional family and keeps the GUI
  framework default fonts as glyph fallback.

## Spacing and shape

- Corner radius: small controls 6px, panels and cards 10px to 12px, the icon
  badge is a generous rounded square (24 of 100 in the mark viewBox).
- Spacing scale: base unit 4px; common gaps 6, 8, 12, 16px.
- Accent usage: gold marks the primary action on a surface and is used sparingly;
  teal is a supporting highlight, not a second primary. Status colors are semantic
  and separate from the accent.

## Assets and reproduction

All rasters are regenerated from the SVG masters by `assets/brand/generate.sh`
(ImageMagick 7). See [assets/brand/README.md](../../assets/brand/README.md) for the
list and the command. Packaging assets are pinned; regenerating them is recorded
as a dated decision in `CHANGELOG.md`.
