# Feature Specification: Pixel-Bus Block Size Single Source of Truth

**Feature Branch**: `028-pixelbus-block-size`

**Created**: 2026-07-24

**Status**: Draft

**Input**: GitHub issue #1 (Investigate smaller PixelBeacon block size with a
configurable square-size advanced setting)

## Overview

The PixelBeacon overlay draws a horizontal strip of fixed-size colored squares
("blocks") at the top-left of the game client, and the companion application
reads one pixel at the center of each block to decode load status, fishing
state, server latency, and the active weapon bar. The size of those squares (16
by 16 physical pixels today) is currently written independently in several
places: the companion's block-center read coordinates, the companion's
screen-capture region, and the addon that draws the squares. Nothing keeps these
copies in agreement, so a change to one without the others would make the
companion read the wrong pixels. The squares are also larger on screen than the
single sampled pixel per block requires, making the overlay more intrusive than
it needs to be.

This feature makes the block size one authoritative value shared between the
addon and the companion, from which every other geometry number is derived on
both sides, and exposes that value as an advanced companion setting so a user can
shrink the overlay. Changing the setting keeps the two sides in lockstep and
re-deploys the addon so the drawn squares and the read coordinates always match.

## Clarifications

### Session 2026-07-24

- Q: What is the exact supported block-size set and how are out-of-range or odd
  values handled? -> A: Even integers from 2 to 32 inclusive; the default is 16;
  an invalid value (odd, out of range, or wrong type) is corrected to the nearest
  supported even value within [2, 32] (an odd value rounds down to the next even,
  a below-range value clamps to 2, an above-range value clamps to 32) with a
  non-fatal notice.
- Q: How is the addon re-deploy driven when the block size changes? -> A:
  Automatically on settings apply, consistent with the existing auto-apply
  settings model: when the applied block size differs from the deployed addon's
  size and the install is managed, the companion re-writes the deployed addon in
  place; if the install is unmanaged or the addon is not installed, no write
  happens and a notice explains why. No separate confirmation dialog is added
  beyond the setting's help text.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Block size is a single shared value (Priority: P1)

As a maintainer, when I need the drawn squares and the read points to match, I
rely on a single block-size value that both the addon and the companion derive
all of their geometry from, so the two can never silently disagree.

**Why this priority**: This is the core of the feature and the prerequisite for
every other change to the pixel bus (adding blocks, shrinking blocks). Without a
single source of truth, any geometry change risks a silent read/draw mismatch
that produces wrong or missing signals with no error.

**Independent Test**: With the default block size, the companion decodes all four
existing blocks (status, fishing, latency, weapon) exactly as it does today; unit
tests confirm the derived read points for several block sizes match the formula
the addon uses to place the squares.

**Acceptance Scenarios**:

1. **Given** the default block size, **When** the companion reads the bus,
   **Then** the four blocks are decoded from the same center coordinates as the
   current release (no behavior change).
2. **Given** any supported block size, **When** the companion computes its read
   points and capture region, **Then** each read point is the center of the
   correspondingly sized square that the addon draws for that block index.

### User Story 2 - Shrink the overlay with an advanced setting (Priority: P2)

As a user who finds the overlay too large, I open the advanced settings, choose a
smaller block size, and the overlay becomes smaller while the companion continues
to read every signal correctly.

**Why this priority**: This is the user-facing payoff of the single source of
truth: a less intrusive overlay. It depends on Story 1 being in place.

**Independent Test**: Change the block-size setting to a smaller supported value,
confirm the setting persists and the companion recomputes its read geometry, and
confirm the addon re-deploy is triggered so the drawn squares shrink to match.

**Acceptance Scenarios**:

1. **Given** a managed PixelBeacon install, **When** the user lowers the block
   size, **Then** the companion updates its stored read geometry (applied on the
   next restart) and the deployed addon is re-written with the new square size so
   drawn and read geometry match.
2. **Given** the block-size setting is changed, **When** the user views the
   setting, **Then** its help text states that changing it requires a PixelBeacon
   re-deploy, and the application surfaces that a re-deploy occurred (or is
   required).
3. **Given** the block-size setting is unchanged from its default, **When** the
   application starts, **Then** the deployed addon and read geometry are
   identical to the current release.

### User Story 3 - Invalid or unsupported sizes never break the app (Priority: P3)

As a user who hand-edits the config or enters an out-of-range value, I am
protected: the application clamps or rejects the value to a supported one,
records a non-fatal notice, and keeps running with a safe geometry.

**Why this priority**: Robustness. A block size that is odd or out of range would
produce a fractional or off-surface center point; the app must never crash or
sample a garbage coordinate.

**Independent Test**: Load a config with an invalid block size (odd, too small,
too large, wrong type) and confirm the app falls back to a supported value,
emits a notice, and does not panic.

**Acceptance Scenarios**:

1. **Given** a config with an out-of-range or odd block size, **When** settings
   load, **Then** the value is corrected to a supported one and a non-fatal
   notice is recorded, and no signal decoding is disrupted.

### Edge Cases

- What happens when the block size is odd (so the center pixel is fractional)?
  The value is rejected or corrected to an even supported value; the center is
  always an integer pixel.
- What happens when the user changes the block size but the deployed addon is not
  a managed install (a hand-installed or foreign copy)? The re-deploy is refused
  and the user is told the addon is unmanaged; no unmanaged folder is ever
  overwritten or deleted.
- What happens when the block size is changed while the addon is not installed at
  all? The companion still updates its own read geometry; no re-deploy is
  attempted, and the next normal install uses the chosen size.
- What happens at the smallest supported size with respect to capture-path
  filtering and UI scale? Correctness at very small sizes is an empirical,
  in-game question; the default size is unchanged and the smallest sizes carry a
  documented, owed in-game validation before they are recommended.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST define the block size as a single authoritative
  value shared, byte for byte in effect, between the drawn overlay and the
  companion reader.
- **FR-002**: The companion MUST derive every read coordinate (each block's
  center point) from the block size using a single documented formula:
  center of block index N is at (size * N + size / 2, size / 2).
- **FR-003**: The companion MUST derive its screen-capture region dimensions from
  the block size and the fixed number of blocks, rather than from independent
  constants.
- **FR-004**: The addon MUST derive its drawn geometry (overall strip width and
  each block's placement) from the single block-size value.
- **FR-005**: When the addon is deployed, the deployed copy MUST carry the block
  size that the companion is currently using, so drawn squares and read points
  agree; the deploy MUST preserve all other addon content, including the managed
  marker that gates uninstall.
- **FR-006**: The block size MUST be exposed as an advanced user setting in the
  existing Pixel Beacon settings area, with help text stating that changing it
  requires a PixelBeacon re-deploy.
- **FR-007**: Changing the block-size setting MUST update the persisted read
  geometry (the reader adopts the new size on the next application start, the same
  way the existing tolerance and interval settings take effect) and MUST drive a
  re-deploy of the addon so the two stay in lockstep. The re-deploy is automatic
  on settings apply (consistent with the existing auto-apply settings model) and
  occurs only when the applied size differs from the deployed addon's size. The
  user is told the new size takes full effect after an in-game `/reloadui` (addon
  redraw) and an application restart (reader geometry).
- **FR-008**: A re-deploy driven by a block-size change MUST only proceed for a
  managed install; an unmanaged addon folder MUST NOT be overwritten, and the
  user MUST be informed. When the addon is not installed, no re-deploy is
  attempted and the companion still updates its own read geometry.
- **FR-009**: The block-size setting MUST be persisted as an additive
  user-settings change that does not require a configuration schema version bump
  and remains backward compatible with existing config files.
- **FR-010**: The system MUST validate the block size to the supported set (even
  integers from 2 to 32 inclusive); an invalid or out-of-range value MUST be
  corrected to the nearest supported even value with a non-fatal notice, never a
  crash (odd rounds down to the next even, below-range clamps to 2, above-range
  clamps to 32).
- **FR-011**: The default block size MUST remain the current value (16), so an
  existing install and a fresh install with no user change behave exactly as the
  current release.
- **FR-012**: The determination of the smallest reliably readable block size MUST
  be recorded as an owed in-game validation (documented in the feature
  quickstart), and the default MUST NOT be lowered below the current value until
  that validation is done.

### Key Entities

- **Block size**: the single physical-pixel dimension of each square; the sole
  knob from which all pixel-bus geometry is derived on both the addon and the
  companion.
- **Block index**: the ordinal position of a block in the strip (status,
  fishing, latency, weapon), used with the block size to compute a center point.
- **Read geometry**: the companion's derived set of per-block center points and
  its capture-region dimensions.
- **Deployed addon**: the on-disk addon copy whose drawn square size must match
  the companion's block size; carries a managed marker that gates its removal.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With the default block size, the companion decodes all four
  existing blocks identically to the current release (zero observable behavior
  change), verified by the existing decoding tests continuing to pass.
- **SC-002**: For every supported block size, the companion's computed read
  points equal the addon's drawn block centers, verified by tests covering block
  sizes 2, 4, 8, and 16.
- **SC-003**: Deploying the addon at a chosen block size rewrites only the
  block-size value in the addon and preserves the managed marker, verified by a
  test that inspects the rendered addon content.
- **SC-004**: A block-size-driven re-deploy is refused when the target addon
  folder is unmanaged, verified by a test that no unmanaged folder is written or
  deleted.
- **SC-005**: Loading a configuration with an invalid block size yields a
  corrected value and a recorded notice with no crash, verified by a test.
- **SC-006**: The full merge gate (format check, lint with warnings as errors,
  and the test suite) passes.

## Assumptions

- The number of blocks on the bus is unchanged by this feature (four:
  status, fishing, latency, weapon); adding blocks is a separate future feature.
- The block-size setting lives in the existing Pixel Beacon settings cluster; no
  new "advanced settings" surface or collapsing UI framework is introduced, to
  keep the change within the current flat settings design.
- The supported block-size set is even integers from 2 to 32 inclusive (finalized
  in Clarifications); the default is 16.
- The re-deploy reuses the existing addon install path and its managed-marker
  guarantees; this feature does not change how uninstall verifies the marker.
- Determining the minimum reliably readable size across capture backends and UI
  scales requires a live game and is out of scope for automated verification; it
  is an owed in-game validation, and the default stays at 16 until then.

## Out of Scope

- Adding Health, Stamina, or Magicka resource blocks or any new block, and the
  associated color mapping table (GitHub issue #2).
- Out-of-band game-resolution and physical-screen detection and any grid-wrap
  layout (GitHub issue #3).
- Lowering the default block size below the current value before the owed in-game
  validation is complete.
- Sampling multiple points per block or any voting scheme.
