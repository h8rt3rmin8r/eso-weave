# ESO Weave brand assets

This directory contains the runtime identity masters, local fonts, exact BrandBuilder recovery bytes, and the S117 adoption record for official package `eso-weave-brand-1.0.0-bb2.0.0`.

The package came from `https://brand.shruggie.tech/eso-weave/downloads/eso-weave-brand-1.0.0-bb2.0.0.zip` and has SHA-256 `b37ac1459666ae33d772229bd5247699c2c845971eabed267ff85465b68a1ba1`. See [the published brand standard](../../docs/src/development/brand-standard.md) for implementation guidance.

## Authority and integrity

- `brand-kit-adoption.json` records exact versions, authority order, migration scope, runtime tokens, adapter deviations, and every consumed artifact hash.
- `eso-weave-glyph.svg` is the authoritative badge-less glyph. Its SHA-256 is `552f3203f0001b15e3adea9b720cb2f78be1427a12410f3e304170d973fef5ea`.
- `eso-weave-mark.svg` is the authoritative badged reduced mark. Its SHA-256 is `696d256c4ec0eae9aed315a1b489bbf5115ec33827e966a6e993708bf3f3109f`.
- `fonts/` contains Inter 400/500/600 and Geist Mono 400 under the SIL Open Font License. `LICENSE-BRAND.md` retains the kit's authoritative reserved-mark and font-license boundary.
- `window-icon-256.png` is the official runtime reference byte.
- `recovery/shruggie-brandbuilder-2.0.0.skill` is the exact offline recovery distribution. Its SHA-256 is `26578eb150a9c24d9e625fb77b192e0415a6ac8faf67c83834ac914f2da15e90`.

Do not edit, normalize, trace, simplify, or replace either authoritative SVG. Do not add an artificial crossing overlay, knockout, outline, or substrate separator to a single-ink derivative.

## Platform assets

S117 adopts the kit's generated classic Win32 ICO at `assets/icon.ico`. Existing runtime, Linux, AppImage, Windows installer, banner, logo, documentation, and social assets already match the kit's official reference bytes. MSIX and other non-shipping platform suites are not retained.

Committed files under `packaging/**` and the classic application icon are pinned artifacts. Any changed byte requires a dated decision in `CHANGELOG.md`.

## Legacy raster regeneration

`generate.sh` remains available to reproduce legacy compatibility and reference rasters with ImageMagick 7:

```sh
bash assets/brand/generate.sh
```

The official BrandBuilder package is the conformance authority. Do not use the legacy script to overwrite the adopted classic Win32 icon or infer a new identity variant.

## Verification

```powershell
node .github/scripts/brand-kit-policy.mjs
node --test .github/scripts/brand-kit-policy.test.mjs
```

Verification is local and does not require a network connection.
