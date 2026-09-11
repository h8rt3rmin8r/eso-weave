# Contract: Brand Standard Visuals

## Source contract

`docs/src/development/brand-standard.md` must:

1. Render local banner, mark, and glyph files with descriptive alternative text.
2. Name and explain every asset and valid surface context.
3. Label dark and light surface cards, with no glyph in the light card.
4. Identify clear and white raster logos as generated compatibility outputs.
5. Link to all three local published files.
6. State clear space, minimum sizes, preserved aspect ratio, and no recoloring.
7. Preserve 13 dark and 12 light palette rows.
8. Put exactly one labeled chip beside each visible uppercase hex value.

## Asset identity contract

| Authority | Published copy |
| --- | --- |
| `assets/eso-weave-banner.png` | `docs/src/assets/brand/eso-weave-banner.png` |
| `assets/brand/eso-weave-mark.svg` | `docs/src/assets/brand/eso-weave-mark.svg` |
| `assets/brand/eso-weave-glyph.svg` | `docs/src/assets/brand/eso-weave-glyph.svg` |

Each pair must compare byte-for-byte.

## Presentation contract

- `.brand-asset-gallery` uses a responsive grid that becomes one column at the existing 40rem narrow breakpoint.
- `.brand-surface` supplies an explicit border and controlled local background.
- Images preserve intrinsic aspect ratio and never exceed their card width.
- `.brand-swatch` has a fixed visible size, fill from the authored token, and an explicit border plus inset highlight that remain visible across themes.
- Page-specific styles do not change global table behavior.

## Generated output contract

- `development/brand-standard.html` retains the asset gallery, surface labels, all chip labels, and local links.
- `assets/brand/` contains all three generated output files.
- Documentation pull-request and main-branch workflows trigger when any approved asset authority changes.
- No remote image URL or page-time script is introduced by S081.

## Failure behavior

Documentation policy returns actionable S081 errors for incomplete tokens, wrong labels or fills, altered or missing assets, prohibited glyph placement, missing guidance, missing responsive rules, or absent generated output.
