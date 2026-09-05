# Feature Specification: Negotiated Width-Aware Pixel Geometry

**Feature Branch**: `codex/045-negotiated-width-geometry`

**Created**: 2026-09-04

**Status**: Implemented

**Input**: GitHub issues #42 and #43, combined as S045

## User Scenarios & Testing

### User Story 1 - Keep every current signal on one row when it fits (Priority: P1)

As an operator, I want PixelBeacon to use the game client's actual width so the
current signal cells do not wrap after an arbitrary fixed number of columns.

**Why this priority**: The premature second row is the reported user-visible
defect. It makes a tiny overlay taller even when nearly the entire top edge is
unused.

**Independent Test**: Supply representative client widths and every supported
block size, decode the published layout, and confirm all 21 payload cells remain
on row zero whenever the header and payload fit.

**Acceptance Scenarios**:

1. **Given** a supported client at least 1024 physical pixels wide and a block
   size from 2 through 32, **When** PixelBeacon lays out the current header and
   21 signals, **Then** every cell occupies row zero.
2. **Given** a client width that crosses an exact block boundary, **When** the
   width changes, **Then** the chosen column count is the number of complete
   blocks that fit and wrapping begins only with the first cell beyond it.
3. **Given** a scale, resolution, window-mode, or monitor transition, **When**
   the effective physical width changes, **Then** the addon republishes and
   repositions the layout without restarting the companion.

---

### User Story 2 - Reject geometry disagreement before decoding payload (Priority: P1)

As an operator, I want damaged or incompatible geometry metadata to make the
pixel bus unavailable rather than shift valid-looking signals into the wrong
fields.

**Why this priority**: A one-column disagreement can turn one valid block into
another valid field. This is more dangerous than an obvious missing signal.

**Independent Test**: Corrupt each header component independently and assert
that no payload event is decoded, while the reader reports the precise bounded
failure state.

**Acceptance Scenarios**:

1. **Given** a valid header, **When** the reader samples payload, **Then** it
   uses only the published column count and payload offset.
2. **Given** a recognized header with an unsupported version, bad marker,
   failed checksum, or impossible column count, **When** the reader samples,
   **Then** every payload value is unavailable and no legacy fallback occurs.
3. **Given** a published extent wider than the measured client surface, **When**
   the reader validates it, **Then** the layout is rejected as implausible.

---

### User Story 3 - Upgrade safely and expose the active shape (Priority: P2)

As an operator, I want the companion to read an older addon safely and show the
shape it is actually using so I can distinguish a legacy layout, a negotiated
layout, and a broken header.

**Why this priority**: Managed installs are normally upgraded with the app, but
a user can run the new companion before reloading the addon. That transition
must remain diagnosable and non-destructive.

**Independent Test**: Feed a legacy status block with no new header, then a new
header, and assert the layout state, capture extent, log transition, and settings
caption all change truthfully.

**Acceptance Scenarios**:

1. **Given** a live version 13 or earlier addon, **When** the new reader sees the
   legacy status block at cell zero and no header magic, **Then** it decodes the
   old 16-column payload through an explicit legacy path.
2. **Given** no recognizable header and no legacy heartbeat, **When** the reader
   samples, **Then** it reports missing layout metadata and decodes no payload.
3. **Given** a negotiated, legacy, or invalid layout, **When** the settings view
   is opened, **Then** the footprint text names that live state and its known
   dimensions rather than presenting a fixed estimate as fact.

### Edge Cases

- The physical width is smaller than the three-cell header.
- The physical width exceeds the 16-bit column-count capacity.
- A resize changes the layout between two sampling batches.
- The old capture contains the new extent, or the new extent grows beyond it.
- Header magic is valid while one payload byte is absent or corrupt.
- The addon is present but hidden during a loading transition.
- The reader has no live surface measurement.
- A hand-edited addon publishes a column count that cannot fit its surface.

## Requirements

### Functional Requirements

- **FR-001**: PixelBeacon MUST be the sole authority that chooses the active
  column count.
- **FR-002**: The layout MUST reserve three invariant top-left cells before all
  payload cells so the reader can locate metadata without already knowing the
  layout.
- **FR-003**: The header MUST carry magic, a tolerance-safe protocol-version
  wire code, a 16-bit column count, distinct byte markers, and a complement
  checksum for each count byte.
- **FR-004**: PixelBeacon MUST derive complete physical-block capacity from the
  current `GuiRoot` width and global UI scale with zero reserved edge margin.
- **FR-005**: The chosen column count MUST be clamped to the header's valid
  three-through-65535 range.
- **FR-006**: Payload cell `i` MUST occupy logical cell `3 + i` under the
  negotiated layout.
- **FR-007**: The addon MUST recompute layout on screen-resize notification and
  on its existing periodic tick so scale-only changes converge.
- **FR-008**: The reader MUST validate the complete header before decoding any
  negotiated payload.
- **FR-009**: A recognized but invalid or unsupported header MUST fail closed
  without falling back to legacy positions.
- **FR-010**: A missing header MAY enter legacy mode only when cell zero is a
  valid legacy heartbeat.
- **FR-011**: Legacy mode MUST use payload offset zero and 16 columns.
- **FR-012**: When a live surface measurement is available, the reader MUST
  reject a published column count or occupied extent that cannot fit it.
- **FR-013**: Missing, corrupt, unsupported, and implausible layout states MUST
  be distinct typed observations.
- **FR-014**: The reader MUST announce layout state only when it changes.
- **FR-015**: The sampler contract MUST accept a requested extent per batch.
- **FR-016**: A steady negotiated layout MUST require one capture per sampling
  batch; bootstrap or extent growth MAY require one additional capture.
- **FR-017**: Capture dimensions and every payload point MUST derive from the
  same validated layout object.
- **FR-018**: The settings footprint text MUST report the live layout mode,
  columns, rows, and physical extent when available.
- **FR-019**: The managed addon manifest MUST advance for the incompatible
  on-screen protocol change.
- **FR-020**: Addon and reader constants MUST be checked for byte-for-byte
  agreement by automated tests.
- **FR-021**: Protocol tests MUST cover boundary wrap, exact extent, unique
  positions, lifecycle changes, corruption, version mismatch, implausible
  metadata, legacy compatibility, and capture batching.
- **FR-022**: Documentation MUST explicitly supersede the fixed-column contract
  from slice 035 and explain why the authority model changed.
- **FR-023**: Real-client validation remains in issue #44 and MUST NOT be claimed
  by this implementation slice.

### Key Entities

- **LayoutHeader**: Three fixed cells containing magic, version, high and low
  column bytes, markers, and checksums.
- **BusLayout**: A validated legacy or negotiated geometry containing columns,
  payload offset, row count, and occupied extent.
- **LayoutState**: The live layout observation, including unavailable reasons.
- **CaptureBatch**: One surface frame captured for a requested physical extent.

## Success Criteria

### Measurable Outcomes

- **SC-001**: All 21 current payload signals occupy one row for client widths of
  at least 1024 pixels at every supported block size.
- **SC-002**: For every tested count, no two payload indices share a point and
  every point lies within the computed capture extent.
- **SC-003**: Every single-field header corruption case yields no payload events.
- **SC-004**: A steady layout performs exactly one prepared capture per batch,
  while first acquisition or growth performs no more than two.
- **SC-005**: A legacy version 13 addon continues to decode through the explicit
  compatibility path.
- **SC-006**: A width transition is reflected in layout state and sampling
  points on the next successful batch.
- **SC-007**: The settings caption never labels an unknown or invalid layout as
  a valid fixed geometry.

## Assumptions

- `GuiRoot:GetWidth()` is expressed in UI units and multiplying by
  `GetUIGlobalScale()` yields the physical-pixel width used by the companion.
- `EVENT_SCREEN_RESIZED` covers resolution and window-size changes; the existing
  one-second tick is the backstop for scale-only or missed changes.
- Supported client widths remain at least 1024 physical pixels.
- The companion and managed addon are released together, while the new reader
  retains one-way compatibility with older addons.
- Issue #44 owns live Windows, X11/Proton, window-mode, monitor, and high-DPI
  evidence after a release contains this slice.

## Explicit Design Deviation

Slice 035 fixed the grid at 16 columns because independently deriving a count on
both sides could disagree silently. S045 does not repeat that rejected design.
It moves authority entirely into PixelBeacon and transmits the chosen count in a
validated invariant header. The reader never derives a competing count, so the
failure mode that justified the fixed constant no longer exists. The fixed
16-column geometry remains only as a bounded legacy decoder.
