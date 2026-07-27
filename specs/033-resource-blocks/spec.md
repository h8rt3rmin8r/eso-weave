# Feature Specification: PixelBeacon Resource Blocks

**Feature Branch**: `033-resource-blocks`

**Created**: 2026-07-27

**Status**: Draft

**Input**: GitHub issue #2 (add Health, Stamina, and Magicka resource blocks to
PixelBeacon, 1 percent colour mapping). Build plan `docs/plans/plan-010.md`,
slice 033. Master specification section 10.3.

## Overview

The beacon strip already tells the companion what weapons the player is holding,
whether they are in combat, and whether a menu is open. It does not tell it how
much health, stamina, or magicka they have, which is the most basic state in the
game and the one every combat decision eventually turns on.

This feature adds those three, each as a percentage of the character's current
maximum, published as three new squares and shown in the application. As with
combat state, nothing acts on the values yet; this adds the observable and stops.

The interesting question here is not what to publish but how to encode it, and the
answer changes the shape of the work. Every previous signal on this strip has been
a small set of discrete states, where the encoding's job is to make each state
unmistakable for any other. A percentage is not that. It is a number, it is ordered,
and being off by one is not the same kind of error as reading the wrong state. That
difference is what this specification is really about, and it is why the gating
deliverable the issue describes (a hundred hand-chosen colours, each provably
distinguishable from its neighbours) turns out not to be needed.

## Clarifications

### Session 2026-07-27

Answered under the build-phase autopilot decision policy from the constitution,
build plan 010, GitHub issue #2, the live `esoui/esoui` source, and the existing
beacon blocks. None were escalated.

- Q: A hundred-entry colour lookup table, as the issue specifies, or a numeric
  channel? -> A: A numeric channel. This reverses the issue's stated preference and
  dissolves its gating deliverable, so the reasoning matters. The issue rejected a
  numeric channel as "more fragile at 1-step resolution" and preferred a table
  whose entries could be "spaced for reliable distinction". Two things are wrong
  with that. First, the premise is contradicted by shipped evidence: the latency
  block already encodes a number in a channel, and it decodes correctly in the
  field today. Second, and more importantly, the table is not actually safer. Under
  a table, a capture that shifts a channel by one lands on whichever entry happens
  to be nearest, which can be any percentage at all; under a numeric channel, the
  same shift reads as one percent off. **The failure mode of the encoding the issue
  preferred is unbounded error; the failure mode of the one it rejected is bounded
  error.** For an ordered quantity that is the whole argument.
- Q: What does "distinguishable" mean for a percentage, then? -> A: Not
  distinguishability at all, but bounded error and monotonicity. The requirement
  worth stating is that a decoded value never differs from the published value by
  more than the reader's tolerance, and that reading higher always means the
  resource is higher. Demanding that 47 and 48 be tolerance-separated would be
  demanding a property nobody needs, at the cost of a hundred hand-picked colours
  that would each need checking.
- Q: 1 percent granularity, or the 5 percent fallback the issue allows? -> A: 1
  percent, and the fallback is moot. It existed because a hundred-entry table might
  not have been makeable; a numeric channel represents 0 to 100 directly with room
  to spare, so the coarser option buys nothing.
- Q: One block for all three resources, or one each? -> A: One each. A block has
  three channels and this strip's discipline spends one on a validity marker and
  one on a checksum, which leaves exactly one for a payload. Packing three
  percentages into three channels would mean no marker and no checksum, so an
  unrelated colour behind a missing block would decode as plausible resource
  values, which is the failure every other block on this strip is built to avoid.
- Q: These change constantly in combat. Does that flood anything? -> A: It would
  have, and this is the one place the previous two slices' pattern is deliberately
  broken. Combat state and menu state log every change at debug level, which is
  right for a signal that changes a few times a minute. Three resources at 1
  percent under fast sampling can change many times a second, so logging them the
  same way would bury every other line in the operator's live log. Resource changes
  are recorded at trace level only. Events are still emitted on change, because
  they are cheap and the display needs them.
- Q: Should a resource change be emitted at all if nothing consumes it? -> A: Yes,
  for display, and the boundary is enforced the same way combat state's is: the
  values are stored where the interface can read them and nothing in any decision
  path reads them.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - The operator can see their resources in the companion (Priority: P1)

The operator plays with the application running. Their health, stamina, and
magicka appear in the application as percentages and track what the game shows,
without the operator taking any action.

**Why this priority**: This is the feature.

**Independent Test**: With the game running, spend and regenerate each resource and
confirm the application follows each one.

**Acceptance Scenarios**:

1. **Given** the addon is installed and the beacon is being read, **When** the
   player's health changes, **Then** the application reports the new percentage.
2. **Given** the same, **When** stamina or magicka changes, **Then** each is
   reported independently and is not confused with the others.
3. **Given** a resource is full, **When** the application reads it, **Then** it
   reports 100; **Given** a resource is empty, **Then** it reports 0.
4. **Given** the player's maximum changes (a buff, a set swap), **When** the
   application reads the resource, **Then** the percentage is relative to the new
   maximum, because a percentage of a stale maximum is meaningless.

---

### User Story 2 - A misread is small, never wild (Priority: P1)

A capture that shifts a colour channel slightly produces a percentage that is
slightly wrong, not one that is arbitrarily wrong. The operator never sees health
jump to an implausible value because a pixel was off by one.

**Why this priority**: Equal to the first, because it is the property that makes
the encoding choice defensible, and because a wildly wrong resource reading is the
kind of thing a future consumer would act on catastrophically.

**Independent Test**: Decode every published value with every channel perturbed
across the tolerance range and confirm the error is bounded. Testable at the desk.

**Acceptance Scenarios**:

1. **Given** any published percentage, **When** the sample is perturbed within the
   reader's tolerance, **Then** decoding yields either a percentage within that
   tolerance or unavailable, and never a different percentage.
2. **Given** two published percentages where one is higher, **When** both decode,
   **Then** the decoded values preserve that order.
3. **Given** a sample that fails validation, **When** it is decoded, **Then** the
   result is unavailable rather than a plausible-looking number.

---

### User Story 3 - No false readings from an older addon (Priority: P2)

An operator running an addon that predates these blocks sees the resources
reported as unavailable, never as numbers.

**Why this priority**: The same compatibility obligation every block on this strip
carries. A fabricated 100 percent health is worse than a blank.

**Independent Test**: Decode arbitrary colours at the three positions and confirm
none yields a value.

**Acceptance Scenarios**:

1. **Given** an addon that draws no resource blocks, **When** the application
   samples them, **Then** all three report unavailable.
2. **Given** the beacon signal is lost, **Then** all three clear to unavailable
   rather than holding their last values.

---

### Edge Cases

- **A resource's maximum is zero or unreadable.** The percentage is undefined;
  the block publishes unavailable rather than dividing by zero or reporting 0,
  which would read as "empty" and be actively misleading.
- **The player is dead.** Health is genuinely 0 and is published as 0. This is a
  real value, not an error.
- **Values change faster than the sampling interval.** Intermediate values are
  missed. Inherent and acceptable: these are levels, not events, so the next sample
  reports the truth.
- **An older addon, or a lost signal.** Covered by User Story 3: unavailable.
- **The strip is now nine blocks wide.** Each new block widens the captured region.
  No functional consequence, but it is the point at which the strip's growth is
  worth noting against the grid-wrap work issue #3 enables.

## Requirements *(mandatory)*

### Functional Requirements

**The signal**

- **FR-001**: The addon MUST publish Health, Stamina, and Magicka, each as a whole
  percentage from 0 to 100 of that resource's current maximum, on its own square.
- **FR-002**: Each percentage MUST be computed against the resource's current
  maximum at the time of publication, so a changed maximum is reflected
  immediately.
- **FR-003**: A resource whose maximum cannot be read, or is zero, MUST be
  published as unavailable rather than as any percentage. Both sides MUST agree on
  a single wire representation for unavailable, distinct from every value in the
  published range. No new decode rule is needed for it: any payload outside the
  published range already decodes to unavailable, so the agreement is about the
  addon and the companion naming the same value rather than each choosing one.
- **FR-004**: Each square MUST carry a validity mark distinct from every other mark
  on the strip, and MUST carry a checksum, so an absent block cannot be read as a
  value.
- **FR-005**: Each square MUST be drawn whenever the addon is loaded and rendering,
  and MUST NOT be hidden to express a state.
- **FR-006**: Each square MUST be redrawn only when its published percentage
  changes.
- **FR-007**: The addon MUST publish resource changes promptly enough that the
  displayed values track play, using the game's own change notifications rather
  than only a periodic poll.

**Decoding, and the bounded-error property**

- **FR-008**: The companion MUST decode each square into a percentage from 0 to
  100, or unavailable when validation fails. A payload slightly above 100, within
  the reader's tolerance, MUST decode to 100 rather than being rejected: a full
  resource is the ordinary out-of-combat state, and rejecting it on any upward
  drift would make the most common value the least stable reading on the strip.
- **FR-009**: For any sample perturbed within the reader's per-channel tolerance,
  decoding MUST yield either a percentage within that tolerance of the published
  one, or unavailable. It MUST NOT yield a different percentage. Refusing to decode
  is a safe outcome; a plausible but wrong value is not, and that distinction is
  the entire reason for choosing this encoding over a lookup table.

  Note why the guarantee is not simply "always within tolerance": the payload and
  its checksum are validated together, so a sample whose channels both drift the
  same way by the full tolerance sums to twice it and is rejected. That is correct
  behavior, not a shortfall. The existing latency block has the same property.
- **FR-010**: Decoding MUST be monotonic: if one published percentage is greater
  than another, the decoded values MUST preserve that ordering.
- **FR-011**: Each resource MUST decode independently; a failure to decode one MUST
  NOT affect the other two.
- **FR-012**: The companion MUST clear a resource to unavailable when the beacon
  signal is lost and on any sample that does not decode, rather than holding a
  stale value.

**Surfacing and boundaries**

- **FR-013**: The companion MUST present all three resources in the interface.
- **FR-014**: The companion MUST record resource changes at trace level only, not
  at the debug level used for combat and menu state, because these change orders of
  magnitude more often and would otherwise bury every other line in the live log.
- **FR-015**: This feature MUST NOT change any behavior based on resource values.
  Weave timing, key synthesis, input interception, the menu gate, and the fishing
  controller behave exactly as they do today.
- **FR-016**: This feature MUST NOT alter any input path or safety behavior, and
  MUST NOT weaken any existing test.

**Geometry and distribution**

- **FR-017**: Each square's position MUST derive from the configured square size,
  and the strip length MUST remain stated once per side of the contract with the
  existing automated agreement check extended to the new values.
- **FR-018**: The addon manifest version MUST advance so the application offers the
  update, and its description MUST name the new signals.
- **FR-019**: The master specification's pixel-bus section MUST document the new
  squares, and the changelog MUST record the feature plus a dated decision for the
  encoding choice, which reverses the issue's stated preference.

### Key Entities

- **Resource level**: A whole percentage from 0 to 100, or unavailable.
  Unavailable means the companion could not read it, or the addon could not compute
  it; it is distinct from 0, which means the pool is genuinely empty.
- **Beacon strip**: Nine squares after this feature.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With the game running, each of the three resources tracks the game's
  own display across a validation run covering spending and regenerating each one.
- **SC-002**: For every publishable percentage and every channel perturbation
  within the reader's tolerance, decoding yields either a value within that
  tolerance or unavailable, and never a different percentage. Verified exhaustively
  over the full space, not by sampling.
- **SC-003**: For every publishable percentage, decoding preserves ordering.
- **SC-004**: No colour outside the encoding decodes as a resource value.
- **SC-005**: With an addon that does not publish these blocks, all three report
  unavailable and every pre-existing signal behaves identically.
- **SC-006**: A steady resource produces no repeated announcements, and the live
  log at the operator's default level contains no resource lines at all.
- **SC-007**: The full merge gate passes with no test weakened, skipped, or made
  conditional.

## Assumptions

- **A percentage is the useful form, not the raw value.** Raw pools range into the
  tens of thousands and vary by character; a percentage is comparable across
  characters and is what a future consumer would gate on. This follows the issue.
- **Being one percent wrong is harmless; being fifty percent wrong is not.** This
  asymmetry is the basis of the encoding decision and of FR-009.
- **The game exposes current and maximum for each pool to an addon, with a change
  notification.** Verified present in the live API source.
- **Nothing consumes these values yet.** As with combat state, the observable is
  added first so it can be seen to be correct before anything depends on it.

## Dependencies

- The bundled PixelBeacon addon and its manifest.
- The beacon strip reader, its block geometry, its marker registry, and the
  cross-language agreement check established in the previous two slices.
- The master specification's pixel-bus section.
