# Build Plan 015: Negotiated Pixel Geometry

Plan: 015
Status: active
Master specification: `docs/ESO-Weave-Specification.md`
Constitution: `.specify/memory/constitution.md`

## Purpose

The fixed 16-column grid introduced in slice 035 prevents unbounded horizontal
growth, but the current 21-signal payload wraps even when the game client has
ample unused width. Its width also cannot safely become an independently derived
runtime value because a one-column disagreement would remap otherwise valid
signal colors.

This plan replaces that fixed shipping geometry with one addon-authoritative,
versioned layout decision. A small invariant header makes the decision available
before the application locates any payload cell, and the application validates
it against the measured client surface before decoding data.

## Ordering

Slice 045 combines issues #42 and #43 because dynamic addon placement is not safe
without matching reader geometry, while reader geometry has no authority without
the addon's published count. The header contract lands first in tests, followed
by addon layout, reader capture, compatibility, diagnostics, and documentation.
Issue #44 remains a separate release-verification outcome because it requires a
fresh packaged build and live game environments.

## Slice 045: Negotiated Width-Aware Pixel Geometry

Feature under `specs/045-negotiated-width-geometry/`.

Scope:

- reserve three invariant top-left cells for magic, version, and a checksummed
  16-bit column count
- make PixelBeacon the sole column authority using live physical client width
- offset and reflow all 21 payload blocks on resize and periodic scale checks
- validate metadata and surface fit before sampling any payload
- request capture extents from the validated layout with bounded recapture
- retain fixed 16-column reads only behind a positive legacy heartbeat
- expose negotiated, legacy, waiting, and unavailable layout diagnostics
- advance the managed addon protocol generation to version 14

Done when issues #42 and #43 close from the merged pull request, protocol and
compatibility tests pass, all review threads are resolved, and CI is green.
