# Feature Specification: Pixel Bus Grid Wrap

**Feature Branch**: `035-grid-wrap`

**Created**: 2026-07-27

**Status**: Draft

**Input**: GitHub issue #16 (wrap the pixel-bus blocks into a 2D grid so the bus
scales past a single strip). Build plan `docs/plans/plan-011.md`, slice 035.
Master specification section 10.3.

## Overview

The beacon is a row of squares that has only ever grown. It was four wide when it
was built and it is nine now, because the last four slices each added one. Nine
squares at the default size is 144 pixels, which is nothing. Two hundred squares
is 3200 pixels, which fits on no display anyone owns, and two hundred is a
conservative estimate of the observables worth publishing once buffs, cooldowns,
target state, and group state are counted.

This feature changes the shape rather than the contents. The squares wrap into
rows, so the beacon grows down instead of sideways and its width stops depending
on how many signals exist. Nothing is added, nothing is removed, and no square
changes what it means.

The interesting thing about this feature is that, done correctly, it does
absolutely nothing today. Nine squares wrapped at sixteen columns is nine squares
in one row, at exactly the coordinates they occupy now, captured from exactly the
region captured now. That is not a disappointing side effect; it is the property
that makes the change safe, and this specification treats proving it as a
deliverable rather than as a footnote.

The one real decision is how the two sides agree on where a square lands. That
question has an obvious answer and a correct answer, and they are different, which
is what the clarifications below are mostly about.

## Clarifications

### Session 2026-07-27

Answered under the build-phase autopilot decision policy from the constitution,
build plan 011, GitHub issue #16, and the existing block geometry and
cross-language agreement check. None were escalated.

- Q: Is the column count a fixed shared constant, or does each side compute it
  from the live client width? -> A: A fixed shared constant, and this reverses
  what the prerequisite issue (#3) assumed when it argued that the render
  resolution had to be known before the bus could wrap. The reversal turns on how
  the two options fail rather than on how they work. A derived count requires the
  addon (from its interface root, scaled) and the companion (from the window's
  client rectangle) to arrive at the *identical integer* from independent
  measurements. When they do, both work. When they differ by one, the companion
  does not read garbage and does not read nothing: it reads real, valid,
  marker-passing, checksum-passing colours from the wrong squares, and reports
  every signal from the second row onward as some other signal's value with full
  confidence. Rounding, interface-scale handling, overscan adjustment, and a
  resolution change mid-session are four independent ways to produce that, none of
  which announce themselves. A fixed constant has none of that exposure, and the
  project already has the machinery to enforce it: the block count is stated once
  per side and the build asserts the two agree by reading the addon source out of
  the binary. The column count joins it.
- Q: Then what was the display detection for? -> A: Validation, not derivation,
  and saying so plainly is better than letting the earlier framing stand. The
  measured client area is what tells us whether the grid actually fits on the
  screen. That is a real job and only the measurement can do it, because a square
  drawn past the edge of the client area is captured as black, fails its marker
  check, and decodes as absent, which is indistinguishable from an addon that was
  never installed. It is worth catching that case explicitly rather than letting
  an operator debug a signal that is being drawn correctly and cropped.
- Q: How many columns? -> A: Sixteen. Two constraints bound the choice and one
  consideration turns out not to. It must be at least the current block count, or
  the wrap would move existing squares and forfeit the no-change-today property.
  And the widest possible grid, at the largest supported square size, must fit
  inside the smallest client area the game supports; sixteen columns at
  thirty-two pixels is 512 pixels, which leaves at least half the width spare on
  any resolution the game runs at. The consideration that does not bind is cost:
  the captured area is the square size squared times the number of squares,
  rounded up to whole rows, regardless of how those squares are arranged. Sixteen
  and sixty-four capture the same number of pixels for the same block count. The
  column count is therefore a layout choice, not a performance one.
- Q: What does the application do when the grid does not fit? -> A: It says so
  and changes nothing. Refusing to sample would convert a partially cropped
  beacon into no beacon at all, which is strictly worse: the squares that do fit
  still decode correctly. And the measurement is not always available (the
  pre-launch case, and the X11 backend which reports no scale and a
  whole-screen display), so a check that gated anything would have to decide what
  to do when it cannot run, which is a decision with no good answer. Advisory
  only, recorded when the answer changes rather than every cycle.
- Q: Does the square-position helper change shape? -> A: No. It keeps taking a
  square size and an index, and consults the column constant itself, so every
  existing caller is untouched. The column arithmetic is factored into its own
  small function that takes the column count as a parameter, which is what makes
  the wrap testable at column counts other than the one shipped without changing
  the shipped constant.
- Q: Does the addon version really need to advance, given the drawn output is
  identical? -> A: Yes, and the reason is worth being honest about because the
  usual one does not apply. Normally the version advances so the companion can
  tell an addon that draws a signal from one that does not; here both versions
  draw the same pixels, so nothing breaks if an operator never updates. It
  advances anyway so the deployed addon actually contains the wrapping logic,
  which means the next slice to add a square inherits a working grid instead of
  having to ship the wrap and the square together and bump twice.
- Q: Where does a does-not-fit report surface, and how loudly? -> A: The live log
  at warning level, once per change of the outcome. Not the debug level the last
  four slices used for their new signals, and the difference is the point: those
  were observations about the game, which are interesting while diagnosing and
  noise otherwise. This is a misconfiguration the operator can act on, and its
  consequence (signals reading as absent while the addon is plainly installed and
  running) is one they would otherwise spend real time misdiagnosing. A line
  nobody sees at their normal log level would not do the job the check exists for.
- Q: Does the fit check consider where the grid sits on the screen, or only how
  big it is? -> A: Only how big it is, and that is not a simplification. The grid
  is anchored to the client area's top-left corner, so its offset within that area
  is always zero and the only question is whether its extent exceeds the client
  area's. The surface's own position on the desktop is irrelevant: a window half
  off the edge of a monitor is a capture problem the existing sampler already
  handles, not a grid-layout problem.
- Q: Can a descriptor derived from stored settings satisfy the fit check, or must
  it be a live measurement? -> A: A live measurement only. A configured
  descriptor is produced only when there is no window, and when there is no
  window the addon is not drawing anything, so the check has nothing to be about.
  Accepting one would mean reporting a fit problem for a grid that does not
  currently exist.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - The beacon stops being bounded by screen width (Priority: P1)

The beacon can carry many more signals than a single row of squares allows,
because squares past the end of a row continue on the next one instead of running
off the side of the screen.

**Why this priority**: This is the feature, and it is the constraint every future
signal is currently blocked by.

**Independent Test**: Compute square positions for block counts well past one row
and confirm they wrap, that no two squares share a position, and that the
occupied region grows downward rather than sideways.

**Acceptance Scenarios**:

1. **Given** a block count within one row, **When** positions are computed,
   **Then** every square is in row 0 and the layout is a single row.
2. **Given** a block count larger than one row, **When** positions are computed,
   **Then** squares continue on the next row, and the occupied width never
   exceeds one row's width no matter how many squares there are.
3. **Given** any block count, **When** positions are computed, **Then** no two
   squares occupy the same position and no index is skipped.
4. **Given** any block count, **When** the sampled region is computed, **Then**
   it covers every drawn square and no more rows or columns than are occupied.

---

### User Story 2 - Both sides agree on where every square is (Priority: P1)

The application and the addon place and read each square at the same coordinates,
and a change to one that is not matched by the other fails the build rather than
shipping.

**Why this priority**: Equal to the first, and arguably above it. A wrap the two
sides disagree about is worse than no wrap: every square after the first row
decodes as a different square's value, and each one passes its marker and
checksum, so nothing looks wrong.

**Independent Test**: The build asserts the two sides state the same column count
by reading the addon source, as it already does for the block count.

**Acceptance Scenarios**:

1. **Given** the shipped addon and the application, **When** the build runs,
   **Then** it asserts both state the same column count, and fails naming the
   disagreement if they do not.
2. **Given** either side's column count is changed alone, **When** the build
   runs, **Then** it fails.
3. **Given** any square index, **When** each side computes its position
   independently, **Then** the results are identical.

---

### User Story 3 - Nothing changes today (Priority: P1)

An operator running the application across this change sees no difference at all:
the same squares in the same places, read the same way, with the same signals.

**Why this priority**: Equal to the other two, because it is what makes this
change safe to land before anything needs it. A geometry change that alters
observable behaviour on the day it ships has to be validated against every
existing signal; one that provably does not, does not.

**Independent Test**: Assert every square's position and the sampled region at the
current block count are identical to the values the pre-wrap arithmetic produced.
Fully testable at the desk.

**Acceptance Scenarios**:

1. **Given** the current block count, **When** each square's position is
   computed, **Then** it equals the position the single-row arithmetic produced.
2. **Given** the current block count, **When** the sampled region is computed,
   **Then** it equals the region sampled before this change.
3. **Given** the heartbeat square, **When** its position is computed at any
   column count, **Then** it is the first position of the first row, so
   signal-loss detection is unaffected.
4. **Given** every existing signal, **When** it is decoded after this change,
   **Then** it behaves exactly as before.

---

### User Story 4 - A grid that does not fit is reported, not hidden (Priority: P2)

If a combination of square size and square count would put part of the grid
outside the game's client area, the operator is told, rather than left to work out
why some signals read as missing while the addon is installed and running.

**Why this priority**: Below the others because it cannot happen at the shipped
settings and is a guard against a future combination. It earns its place because
the failure it catches is silent and misattributable: a cropped square looks
exactly like an absent addon.

**Independent Test**: Evaluate the fit rule across grid extents and client areas
at the desk, including the case where no measurement is available.

**Acceptance Scenarios**:

1. **Given** a grid that fits inside the measured client area, **When** the fit
   is evaluated, **Then** nothing is reported.
2. **Given** a grid wider or taller than the measured client area, **When** the
   fit is evaluated, **Then** it is reported once, and sampling continues
   unchanged.
3. **Given** no measurement is available, **When** the fit is evaluated,
   **Then** nothing is reported and nothing is blocked.
4. **Given** an unchanging fit outcome, **When** it is evaluated repeatedly,
   **Then** it is reported once, not once per cycle.
5. **Given** only a descriptor derived from stored settings, **When** the fit is
   evaluated, **Then** nothing is reported, because no window means no drawn
   grid to be about.

---

### Edge Cases

- **The square count is an exact multiple of the column count.** The last row is
  full and no partial row exists. The occupied region must not include a
  phantom empty row.
- **The square count is one more than a multiple.** The last row holds one
  square, and the region is a full row wide and one row taller. The cells beside
  that square are not drawn by the addon and are not read by the application;
  whatever the game renders there is irrelevant because nothing samples it.
- **The square count is smaller than the column count**, which is today. The
  region is only as wide as the squares in use, not as wide as a full row.
- **The square size is at its smallest or largest supported value.** Positions
  scale with it; the column count does not change. The sampled point of each
  square must remain a whole pixel at every supported size, exactly as it does
  now.
- **The grid does not fit.** Covered by User Story 4: reported, never enforced.
- **The window is dragged slowly across the size at which the grid stops
  fitting.** The outcome genuinely changes each time the threshold is crossed, so
  each crossing is reported. This is the change detection working rather than
  failing: the transitions are human-paced, they stop when the drag stops, and a
  warning that appears and disappears as the window crosses a boundary is an
  accurate description of what is happening.
- **The measurement is unavailable**, which is the pre-launch case and every X11
  session's scale. The fit check simply does not run.
- **An operator is running the previous addon version.** Its squares are drawn at
  the same coordinates the wrapped application reads, because at the current
  square count the two layouts are identical, so the mixed combination works. This
  is a consequence of User Story 3 rather than a separate compatibility path.

## Requirements *(mandatory)*

### Functional Requirements

**The wrap**

- **FR-001**: A square's position MUST be determined by its index and a column
  count, wrapping to the next row when a row is full: the column is the index
  modulo the column count and the row is the index divided by it.
- **FR-002**: Both the application and the addon MUST place squares using that
  same rule, so a given index resolves to the same position on each side.
- **FR-003**: The sampled region and the addon's drawn extent MUST derive from
  the same grid arithmetic, covering every occupied square and no unoccupied
  row or column. The region's width MUST be the lesser of the square count and
  the column count, so a grid using a fraction of one row does not sample a full
  row's width.
- **FR-004**: The heartbeat square MUST occupy the first position of the first
  row under any column count, because signal-loss detection anchors on it.
- **FR-005**: No two square indices may resolve to the same position, and every
  index below the square count MUST resolve to a position inside the occupied
  region. The converse does not hold and MUST NOT be required: when the square
  count is not a multiple of the column count the final row is partial, so the
  region legitimately contains positions no index maps to.
- **FR-006**: Each square's sampled point MUST remain a whole pixel at every
  supported square size, as it is today.

**The shared column count**

- **FR-007**: The column count MUST be a fixed value, stated exactly once on each
  side of the contract, and MUST NOT be computed from the display resolution, the
  client area, or any other measured quantity on either side.
- **FR-008**: The build MUST assert that both sides state the same column count,
  by the same mechanism that already asserts they state the same square count, and
  MUST fail naming the disagreement when they do not.
- **FR-009**: The column count MUST be at least the current square count, so the
  wrap does not move any existing square.
- **FR-010**: One full row of squares, at the largest supported square size, MUST
  fit within the narrowest client area the game supports, taken as 1024 physical
  pixels (see Assumptions). This bounds the grid's width at the time the column
  count is chosen. It deliberately does not bound the height: the row count grows
  with the square count over the project's life, so no fixed bound could be
  stated, and checking the height is exactly the job FR-016 exists to do at
  runtime.
- **FR-011**: The column arithmetic MUST be exercisable at column counts other
  than the shipped one, so the wrap's properties can be tested without changing
  what ships.

**Nothing changes today**

- **FR-012**: At the current square count, every square's position MUST be
  identical to the position produced before this change.
- **FR-013**: At the current square count, the sampled region MUST be identical
  to the region sampled before this change.
- **FR-014**: FR-012 and FR-013 MUST each be asserted by a test, not left as a
  consequence of the arithmetic. They are the evidence that this is a contract
  change and not a behaviour change.
- **FR-015**: No existing square's encoding, marker value, checksum rule,
  meaning, or update cadence may change, and no existing test may be weakened,
  skipped, or made conditional.

**The fit check**

- **FR-016**: The application MUST be able to determine whether the grid's extent
  fits within the measured client area, from the square size, the square count,
  the column count, and the measured surface. The comparison is of extents only:
  the grid is anchored at the client area's top-left, so its offset is always
  zero and the surface's own position on the desktop is not part of the question.
- **FR-017**: A grid that does not fit MUST be reported and MUST NOT change any
  behaviour. Sampling, decoding, and every signal continue exactly as they would
  otherwise.
- **FR-018**: When no measurement is available the check MUST NOT run and MUST
  NOT block, report, or degrade anything. A descriptor derived from stored
  settings rather than from a live window MUST NOT satisfy the check, because it
  is produced only when no window exists and therefore describes no drawn grid.
- **FR-019**: The report MUST be emitted at warning level in the live log, and
  only when the fit outcome changes rather than on every sampling cycle. Warning
  rather than the debug level the recent signal slices use, because this is an
  actionable misconfiguration rather than an observation, and its symptom is
  otherwise easy to misattribute to a missing addon.

**Distribution and documentation**

- **FR-020**: The addon manifest version MUST advance so the application offers
  the update, and the deployed addon therefore carries the wrapping logic before
  any square depends on it.
- **FR-021**: The master specification's pixel-bus section MUST express square
  positions as grid coordinates and MUST document the wrap rule and the fixed
  column count. The changelog MUST record the feature plus a dated decision for
  the fixed column count, which every later slice inherits.

**Boundaries**

- **FR-022**: This feature MUST NOT add, remove, or repurpose any square, signal,
  or marker.
- **FR-023**: This feature MUST NOT change any input, weave, fishing, or
  suppression behaviour.
- **FR-024**: This feature MUST NOT consume the additional capacity it creates.
  Filling the grid is a later feature.

### Key Entities

- **Grid position**: A column and a row derived from a square's index and the
  column count. Replaces the single offset that positioned a square before.
- **Grid extent**: The occupied region in physical pixels: as wide as the lesser
  of the square count and the column count, and as tall as the number of rows the
  squares occupy.
- **Column count**: A fixed number stated once per side of the contract and
  asserted equal by the build. Not measured, not negotiated, not configurable.
- **Fit outcome**: Whether the grid extent lies within the measured client area,
  or unknown when there is no measurement. Advisory; nothing branches on it.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: For square counts spanning several rows, every index resolves to a
  distinct position inside the region, the occupied width never exceeds one row,
  and the region covers exactly the occupied rows and columns. Verified by test
  over a range of counts and column counts, including exact multiples and partial
  rows.
- **SC-002**: At the current square count, every square position and the sampled
  region are identical to their pre-change values, verified by explicit
  assertion against the previous arithmetic rather than by inspection.
- **SC-003**: The build fails when either side's column count is changed alone.
- **SC-004**: Every pre-existing signal decodes exactly as before, and every
  pre-existing test passes unmodified.
- **SC-005**: The fit check reports a grid that does not fit, stays silent for
  one that does, stays silent when no measurement exists, stays silent for a
  descriptor derived from stored settings, and reports at most once per change of
  outcome. Verified by test across all five cases.
- **SC-006**: The heartbeat square is at the first position of the first row for
  every column count tested.
- **SC-007**: With the game running and the updated addon installed, every signal
  behaves as it did before the change, and the beacon is visually unchanged.
- **SC-008**: The full merge gate passes with no test weakened, skipped, or made
  conditional.

## Assumptions

- **The addon can position a square by row as easily as by column.** It anchors
  each square relative to a container by offset today; a vertical offset is the
  same operation as a horizontal one.
- **Squares are square and uniformly sized**, so one number describes both the
  column pitch and the row pitch. True today and unchanged here.
- **The smallest client area the game supports is at least 1024 pixels wide.**
  Used only to check that the widest grid at the largest square size fits with
  margin; the conclusion holds comfortably rather than marginally.
- **Nothing consumes the extra capacity yet.** As with the previous four slices,
  the mechanism lands before anything depends on it, so it can be seen to be
  correct first.
- **The area captured is set by the number of squares, not their arrangement**,
  so the column count is chosen for layout rather than for cost.

## Dependencies

- The existing block geometry (`block_center`, the capture region derivation) and
  the square-size sanitization that keeps sampled points on whole pixels.
- The cross-language agreement check established in slice 031, which reads the
  addon source embedded in the application binary.
- The display descriptor from slice 034, used here only to evaluate fit.
- The bundled addon and its manifest.
- The master specification's pixel-bus section.
