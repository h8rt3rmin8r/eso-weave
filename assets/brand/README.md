# ESO Weave brand assets

This directory holds the ESO Weave brand masters and the script that regenerates
every rasterized asset from them. The full brand standard (palette, typography,
usage) lives in [docs/brand/ESO-Weave-Brand-v1.md](../../docs/brand/ESO-Weave-Brand-v1.md).

## Masters

- `eso-weave-mark.svg` - the badged mark (two woven carets, gold and teal, on a
  self-contained ink badge). This is the icon used everywhere.
- `eso-weave-glyph.svg` - the carets only, transparent, for lockups on ink
  backgrounds (wordmark, banner, social image).
- `fonts/` - Inter (SIL Open Font License 1.1), the bundled UI typeface, with
  `OFL.txt`.
- `window-icon-256.png` - generated; the runtime window icon bundled into the app.

The weave over/under is baked into each SVG as an overlaid path segment (no
`clipPath`), so any rasterizer, including ImageMagick's built-in SVG renderer,
produces correct output.

## Regenerating rasters

Requires ImageMagick 7 (the `magick` command). From a shell:

```sh
bash assets/brand/generate.sh
```

This rewrites, from the SVG masters:

- `assets/icon.ico` (16/32/48/64/128/256) - app, installer, shortcut and exe icon
- `assets/brand/window-icon-256.png` - runtime window icon
- `assets/eso-weave-logo-clear.png`, `assets/eso-weave-logo-white.png`
- `assets/eso-weave-banner.png` - README banner
- `assets/eso-weave-social.png` (1280x640) - GitHub social-preview image
- `packaging/linux/eso-weave.png`, `packaging/appimage/AppDir/eso-weave.png`
- `packaging/windows/dialog.bmp` (493x312), `packaging/windows/banner.bmp` (493x58)
  - installer wizard art (kept light so the default WiX dark text stays readable)

The committed rasters are the source of truth for the build; the script exists so
they can be reproduced exactly. Files under `packaging/**` are pinned artifacts:
regenerating them is recorded as a dated decision in `CHANGELOG.md`.
