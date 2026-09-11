# ESO Weave Brand Standard v1

This is the authoritative reference for the ESO Weave visual identity. The application theme and every brand asset trace to the tokens and rules below. Build slice S012 established the identity, and S081 published its approved assets as this visual reference.

Direction: "Arcane gold on ink." Near-black ink surfaces with a warm gold as the primary action color honor the Elder Scrolls Online heritage while reading as a modern, clean tool rather than antique decoration. Teal is a supporting accent.

## Approved assets

These examples use the three approved files. The dark and light cards are intentional placement tests, not alternate artwork.

<div class="brand-asset-gallery">
<figure class="brand-surface brand-surface--dark" aria-label="Dark ink surface">
<div class="brand-surface__assets">
<img class="brand-asset--banner" src="../assets/brand/eso-weave-banner.png" alt="ESO Weave full-color banner wordmark">
<img class="brand-asset--mark" src="../assets/brand/eso-weave-mark.svg" alt="ESO Weave badged mark">
<img class="brand-asset--glyph" src="../assets/brand/eso-weave-glyph.svg" alt="ESO Weave badge-less glyph">
</div>
<figcaption><strong>Dark ink surface.</strong> The full-color banner, badged mark, and badge-less glyph are all approved here.</figcaption>
</figure>
<figure class="brand-surface brand-surface--light" aria-label="Light surface">
<div class="brand-surface__assets">
<img class="brand-asset--banner" src="../assets/brand/eso-weave-banner.png" alt="ESO Weave full-color banner wordmark on light">
<img class="brand-asset--mark" src="../assets/brand/eso-weave-mark.svg" alt="ESO Weave badged mark on light">
</div>
<figcaption><strong>Light surface.</strong> Use the full-color banner or self-contained badged mark. Do not place the badge-less glyph here.</figcaption>
</figure>
</div>

### File selection

- **Full-color banner**: the horizontal wordmark for documentation headers and wide identity placements. [Download the full-color banner](../assets/brand/eso-weave-banner.png).
- **Badged mark**: the universal icon. Its rounded ink badge preserves the woven carets on dark, light, and unpredictable surfaces. [Download the badged mark](../assets/brand/eso-weave-mark.svg).
- **Badge-less glyph**: the woven carets without a container, reserved for ink-surface lockups where the surrounding background is controlled. [Download the badge-less glyph](../assets/brand/eso-weave-glyph.svg).

The files `eso-weave-logo-clear.png` and `eso-weave-logo-white.png` are generated compatibility outputs, not masters. Do not use them as editable sources or treat them as alternate approved marks. The SVG masters and banner authority remain under `assets/` in the repository; the links above are byte-identical local copies for public and bundled offline documentation.

### Reproduction rules

- Preserve aspect ratio. Never stretch, compress, rotate, outline, or add effects to an asset.
- Keep clear space equal to at least one strand width around the mark or glyph. Keep the banner clear of neighboring text by at least the height of its capital letters.
- Minimum sizes are 16 CSS pixels for the badged mark, 32 CSS pixels for the glyph, and 160 CSS pixels wide for the banner. Below those sizes, use text instead.
- Do not recolor either strand. Gold is primary and teal is supporting.
- Do not place the badge-less glyph on light backgrounds because the gold strand loses contrast. Use the badged mark instead.
- The wordmark is "ESO" in text color followed by "Weave" in gold, set in Inter SemiBold with light tracking.

## Color tokens

Colors are named by role. The application theme maps these to both dark and light modes. Each chip is a visual sample; its adjacent role, hexadecimal value, and use remain the authoritative non-color description.

### Dark (default)

| Role | Hex | Use |
|------|-----|-----|
| Ink base | <span class="brand-swatch" role="img" aria-label="Ink base, #0E1116 color swatch" style="--swatch-color: #0E1116"></span> `#0E1116` | Window and base surface |
| Panel | <span class="brand-swatch" role="img" aria-label="Panel, #151B23 color swatch" style="--swatch-color: #151B23"></span> `#151B23` | Panels, control fills |
| Elevated | <span class="brand-swatch" role="img" aria-label="Elevated, #1C2530 color swatch" style="--swatch-color: #1C2530"></span> `#1C2530` | Hover and active fills |
| Stroke | <span class="brand-swatch" role="img" aria-label="Stroke, #2A3340 color swatch" style="--swatch-color: #2A3340"></span> `#2A3340` | Borders, separators |
| Gold (action) | <span class="brand-swatch" role="img" aria-label="Gold (action), #F2B03C color swatch" style="--swatch-color: #F2B03C"></span> `#F2B03C` | Primary buttons, active toggles, wordmark accent |
| Gold hover | <span class="brand-swatch" role="img" aria-label="Gold hover, #FBCB6B color swatch" style="--swatch-color: #FBCB6B"></span> `#FBCB6B` | Gold hover and active |
| Gold deep | <span class="brand-swatch" role="img" aria-label="Gold deep, #D18F22 color swatch" style="--swatch-color: #D18F22"></span> `#D18F22` | Gold pressed, borders on gold |
| Teal (support) | <span class="brand-swatch" role="img" aria-label="Teal (support), #2DD4BF color swatch" style="--swatch-color: #2DD4BF"></span> `#2DD4BF` | Secondary accent, mark, info highlights |
| Text | <span class="brand-swatch" role="img" aria-label="Text, #E6EDF3 color swatch" style="--swatch-color: #E6EDF3"></span> `#E6EDF3` | Primary text |
| Muted | <span class="brand-swatch" role="img" aria-label="Muted, #8B97A7 color swatch" style="--swatch-color: #8B97A7"></span> `#8B97A7` | Labels, secondary text |
| Status ok | <span class="brand-swatch" role="img" aria-label="Status ok, #34D399 color swatch" style="--swatch-color: #34D399"></span> `#34D399` | Running, healthy |
| Status warn | <span class="brand-swatch" role="img" aria-label="Status warn, #FB9E3C color swatch" style="--swatch-color: #FB9E3C"></span> `#FB9E3C` | Warnings |
| Status err | <span class="brand-swatch" role="img" aria-label="Status err, #F87171 color swatch" style="--swatch-color: #F87171"></span> `#F87171` | Errors, signal lost |

### Light

| Role | Hex | Use |
|------|-----|-----|
| Base | <span class="brand-swatch" role="img" aria-label="Base, #F7F5F0 color swatch" style="--swatch-color: #F7F5F0"></span> `#F7F5F0` | Window and base surface |
| Panel | <span class="brand-swatch" role="img" aria-label="Panel, #FFFFFF color swatch" style="--swatch-color: #FFFFFF"></span> `#FFFFFF` | Panels, control fills |
| Elevated | <span class="brand-swatch" role="img" aria-label="Elevated, #ECE8DE color swatch" style="--swatch-color: #ECE8DE"></span> `#ECE8DE` | Hover and active fills |
| Stroke | <span class="brand-swatch" role="img" aria-label="Stroke, #DCD9D0 color swatch" style="--swatch-color: #DCD9D0"></span> `#DCD9D0` | Borders, separators |
| Gold (action) | <span class="brand-swatch" role="img" aria-label="Gold (action), #E7A42C color swatch" style="--swatch-color: #E7A42C"></span> `#E7A42C` | Primary buttons, active toggles |
| Gold deep | <span class="brand-swatch" role="img" aria-label="Gold deep, #C6871F color swatch" style="--swatch-color: #C6871F"></span> `#C6871F` | Gold text and borders on light |
| Teal (support) | <span class="brand-swatch" role="img" aria-label="Teal (support), #0D9488 color swatch" style="--swatch-color: #0D9488"></span> `#0D9488` | Secondary accent, info highlights |
| Text | <span class="brand-swatch" role="img" aria-label="Text, #14110B color swatch" style="--swatch-color: #14110B"></span> `#14110B` | Primary text |
| Muted | <span class="brand-swatch" role="img" aria-label="Muted, #6B6455 color swatch" style="--swatch-color: #6B6455"></span> `#6B6455` | Labels, secondary text |
| Status ok | <span class="brand-swatch" role="img" aria-label="Status ok, #059669 color swatch" style="--swatch-color: #059669"></span> `#059669` | Running, healthy |
| Status warn | <span class="brand-swatch" role="img" aria-label="Status warn, #B45309 color swatch" style="--swatch-color: #B45309"></span> `#B45309` | Warnings |
| Status err | <span class="brand-swatch" role="img" aria-label="Status err, #DC2626 color swatch" style="--swatch-color: #DC2626"></span> `#DC2626` | Errors, signal lost |

On filled gold buttons, text is near-ink (`#241704`) in both themes for contrast.

## Typography

- Primary UI typeface: Inter, SIL Open Font License 1.1, bundled with the application under `assets/brand/fonts/` with `OFL.txt`. Regular is for body text, and SemiBold is for the wordmark and emphasis.
- The application registers Inter as the proportional family and keeps the GUI framework default fonts as glyph fallback.

## Spacing and shape

- Corner radius: small controls 6px, panels and cards 10px to 12px, and the icon badge uses a generous rounded square (24 of 100 in the mark viewBox).
- Spacing scale: base unit 4px; common gaps 6, 8, 12, and 16px.
- Accent usage: gold marks the primary action on a surface and is used sparingly. Teal is a supporting highlight, not a second primary. Status colors are semantic and separate from the accent.

## Assets and reproduction

`assets/brand/generate.sh` uses ImageMagick 7 to regenerate compatibility and packaging rasters from the SVG masters. Committed packaging assets remain pinned; regenerating them requires a dated decision in `CHANGELOG.md`. S081 does not regenerate or alter those source assets.
