# Diagram Rendering Compatibility

This record resolves the rendering investigation for the four static flow
diagrams introduced by S082. It is maintainer evidence, not a promise that every
unlisted browser or embedded web view is supported.

## Diagnosis

The blank normal state reported in issue #154 was not reproduced on the public
Architecture page in the Codex isolated Chromium browser on Windows 11. The page
and direct SVG both loaded, and the diagram remained visible in navy and light.

Two concrete compatibility faults were reproduced:

1. Every SVG had a `viewBox` but no intrinsic `width` or `height`, leaving fallback
   sizing to client inference.
2. The responsive `.docs-flow-diagram img` width rule also applied to mdBook's
   expanded clone. At a 1280 by 720 viewport, the 400 by 650 Architecture source
   stretched to the viewport ratio. The generated clone also repeated the
   primary alternative text.

S086 adds dimensions equal to each viewBox, scopes full-width behavior to the
primary image, restores auto dimensions for the expanded clone, and marks that
clone decorative after mdBook creates it. The meaningful primary alternative and
complete adjacent text equivalent remain unchanged.

## Automated matrix

The local receipt on 2026-09-11 used Chrome 152.0.7977.83 on Windows 11. One
dependency-free DevTools run exercised every combination below against the actual
generated mdBook figure DOM and styles.

| Dimension | Values |
| --- | --- |
| Diagrams | Architecture ownership, Action authorization, Safety recovery, Pixel Bus validation |
| Themes | Navy, Light |
| CSS viewport widths | 320, 1280 |
| States | Normal, Expanded |
| Total observations | 32 |

All 32 observations passed image decode, positive geometry, computed visibility,
intrinsic aspect ratio, container or viewport containment, at least 95 percent
opaque pixels, at least four opaque colors, and more than 1 percent
non-background paint. The expanded wrapper was hidden before activation and
visible afterward. The four asset requests returned 200 as `image/svg+xml`.

## Delivery evidence

- On 2026-09-11, all four currently deployed GitHub Pages asset URLs returned 200
  as `image/svg+xml`. The public Architecture figure was visible in navy and light.
- Documentation policy requires each generated SVG to be byte-identical to its
  checked-in source and requires the exact generated mdBook zoom DOM.
- Release-profile `build.rs` requires all four generated asset paths before it can
  emit the sorted embedded manifest. The loopback service maps `.svg` to
  `image/svg+xml`, applies `nosniff`, and serves immutable embedded bytes.
- The browser smoke binds only to `127.0.0.1`, serves generated repository files,
  records the media type, and permits no external rendering dependency.

The same generated output is therefore the Pages artifact and the release-build
embedding source. Repository evidence closes issue #154 without creating a
separate release-verification blocker.

## Explicit boundary

Chrome is the automated representative renderer because it is present on the
GitHub-hosted Ubuntu runner and the supported Windows development environment.
Current Edge and Firefox remain manual compatibility observations rather than CI
promises. Unknown embedded web views are untested, but explicit SVG geometry and
the preserved text equivalents provide bounded fallback behavior.

Issue #155 retains general click-to-expand behavior, visible affordances,
keyboard and focus semantics, background inertness, raw HTML figures, screenshots,
and brand assets. S086 changes only the four existing diagram figures.
