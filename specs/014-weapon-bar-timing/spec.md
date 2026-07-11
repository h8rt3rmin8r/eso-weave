# Feature Specification: Weapon-Bar-Aware Adaptive Timing

**Feature Branch**: `014-weapon-bar-timing`

**Created**: 2026-07-11

**Status**: Draft

**Input**: User description: "Detect which weapon bar is active and apply per-bar
skill-delay timing, with intelligent default presets derived from the equipped
weapon type. Closes research item R1 (evidence-based weave-delay defaults tied to
ESO's global cooldown). The PixelBeacon addon relays the active bar and each bar's
weapon class through a new pixel-bus block; the app decodes it, selects per-bar
timing at runtime, and offers weapon-type default presets."

## Clarifications

### Session 2026-07-11

- Q: What does "per-bar" apply to: the whole skill layout or just the delays?
  (decided under autopilot) -> A: Per-bar applies to the timing (the d_weave,
  d_heavy, d_bash delays), not to the seven skill slots or their key mappings. The
  app keeps one set of skill slots; the active bar selects which timing profile
  (front or back) those slots use. This matches "split out our skill delay settings
  to be unique per-bar" and avoids a larger data-model change to the slots.
- Q: How do auto presets and manual per-bar values interact? (decided under
  autopilot) -> A: A single "auto timing from weapon" preference. When on, each
  bar's delays follow the detected weapon class preset. When off, the user's manual
  per-bar values are used. Turning auto on does not erase the manual values; it just
  supersedes them while enabled.
- Q: How is the weapon type relayed without hardcoding game enum integers?
  (decided under autopilot) -> A: The addon computes a small normalized weapon-class
  code in Lua from the named weapon-type constants and relays only that code, so the
  out-of-process reader decodes a stable class, never the raw game enum integers.
- Q: What happens when there is no weapon-bar signal at all (an older addon without
  the new block, or the beacon not installed)? (decided under autopilot) -> A: The
  app treats the active bar as the front (primary) bar and uses the front timing
  profile, and reports the weapon classes as unknown. The feature degrades to the
  prior single-profile behavior rather than failing, so users without the updated
  addon are unaffected.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Correct timing on whichever bar is active (Priority: P1)

A player weaves on their front bar, swaps to the back bar, and the app applies the
timing appropriate to the now-active bar without any manual switch. The active bar
is detected from the game (relayed by the companion addon), and the app selects the
front or back timing profile accordingly, holding the last known bar through
transient or indeterminate states.

**Why this priority**: This is the core capability. A single shared timing is wrong
for a two-bar build; per-bar timing driven by the detected active bar is the whole
point of the slice and delivers value on its own once presets or manual values
exist.

**Independent Test**: With the addon reporting a bar change, confirm the app's
effective timing switches between the front and back profiles, and that a
transient or indeterminate bar report holds the last good profile rather than
flipping.

**Acceptance Scenarios**:

1. **Given** the app has distinct front and back timing profiles, **When** the
   active bar changes from front to back, **Then** the effective timing used for
   weaving becomes the back profile.
2. **Given** the active bar is reported as indeterminate (locked or none), **When**
   the app processes that report, **Then** it holds the last known good bar and
   profile rather than switching.
3. **Given** repeated bar reports arrive that do not represent a real change,
   **When** they are processed, **Then** the effective bar and profile do not churn.
4. **Given** the companion signal is lost, **When** no bar can be determined,
   **Then** the app falls back to a defined default profile rather than erroring.

---

### User Story 2 - Sensible timing from the equipped weapon (Priority: P1)

A player enables "auto timing from weapon" and each bar's delays are set from the
weapon class equipped on that bar, so a dual-wield bar uses a short heavy-attack
delay while a two-handed or staff bar uses a longer one, without hand-tuning.

**Why this priority**: Weapon-derived defaults are the "intelligent timings" the
user asked for and make the per-bar capability useful immediately, with no manual
timing knowledge required.

**Independent Test**: With auto timing on and a known weapon class per bar, confirm
each bar's effective heavy-attack delay matches the preset for that class; with a
faster-weapon bar and a slower-weapon bar, confirm the faster bar's delay is
shorter.

**Acceptance Scenarios**:

1. **Given** auto timing is on and the front bar is a fast-heavy weapon class and
   the back bar is a slow-heavy weapon class, **When** the presets apply, **Then**
   the front bar's heavy-attack delay is shorter than the back bar's.
2. **Given** auto timing is on, **When** the detected weapon class on a bar changes,
   **Then** that bar's delays update to the new class preset.
3. **Given** auto timing is off, **When** the user sets manual per-bar delays,
   **Then** those manual values are used regardless of the detected weapon class.
4. **Given** the user turns auto timing off after it was on, **When** they view the
   per-bar values, **Then** their previously entered manual values are intact.

---

### User Story 3 - See the active bar and equipped weapons (Priority: P2)

A player glances at the app and sees which bar the game currently has active and the
weapon class detected on each bar, so they can confirm the app is tracking correctly
and understand why a given timing profile is in effect.

**Why this priority**: Visibility builds trust in the automatic behavior and helps
diagnose a mis-detection, but the timing works without it, so it follows the P1
capability.

**Independent Test**: With the addon reporting a known bar and weapon classes,
confirm the app displays the active bar and each bar's weapon class, and updates
when they change.

**Acceptance Scenarios**:

1. **Given** the addon reports the front bar active with known weapon classes,
   **When** the user views the app, **Then** it shows the active bar and the front
   and back weapon classes.
2. **Given** the reported bar or a weapon class changes, **When** the app updates,
   **Then** the displayed values change to match.
3. **Given** no companion signal, **When** the user views the app, **Then** the
   weapon-bar display shows an unknown or not-detected state rather than stale data.

---

### User Story 4 - Documented evidence-based timing defaults (Priority: P3)

A contributor or advanced user opens the specification appendix and finds the
evidence-based weave-delay defaults (the global-cooldown context and per-weapon
heavy-attack durations) that back the presets, closing the long-standing research
item.

**Why this priority**: Durable documentation value that closes the deferred research
item, but it enables and justifies the presets rather than being a runtime feature.

**Independent Test**: Open the specification appendix and confirm it records the
global-cooldown context, the per-weapon-class heavy-attack defaults, their sources,
and the in-game validation still owed, and that the open-items section marks the
research item closed.

**Acceptance Scenarios**:

1. **Given** the appendix exists, **When** a contributor needs a default value or
   its rationale, **Then** the appendix provides the per-weapon-class default and
   its source.
2. **Given** the presets shipped in the app, **When** compared to the appendix,
   **Then** they match the documented defaults.

---

### Edge Cases

- The active-bar report fires far more often than real swaps; only an actual change
  in the determined bar may change the effective profile (no churn on every attack).
- A locked or none bar, and the period right after a loading screen, death, or a
  companion reload, are indeterminate; the app holds the last good value and
  re-establishes a fresh baseline when the signal settles.
- A weapon class that cannot be determined (empty or unrecognized) is reported as an
  unknown class, and auto timing for that bar falls back to a defined default rather
  than a wrong preset.
- One-hand-and-shield heavy-attack timing is not yet quantified by sources; its
  preset ships as a documented estimate flagged for in-game validation.
- The companion addon change must not break the existing status, fishing, and
  latency signals, and must not disturb the managed-marker line that gates safe
  uninstall.
- The exact heavy-attack preset values and the pixel signal itself require in-game
  verification that cannot be performed offline; shipping values are community
  estimates until validated.

## Requirements *(mandatory)*

### Functional Requirements

Detection and relay:

- **FR-001**: The companion addon MUST determine the active weapon bar (front or
  back) from the game and relay it to the app through the pixel signal.
- **FR-002**: The companion addon MUST determine a normalized weapon class for each
  bar (for example none, dual wield, two handed, sword and shield, bow, destruction
  staff, restoration staff) from the equipped weapons and relay both bars' classes.
- **FR-003**: The relayed weapon class MUST be a stable normalized code computed by
  the addon, so the app never depends on raw game enum integers.
- **FR-004**: The companion addon MUST only re-emit the signal on an actual change in
  the determined bar or classes, edge-detecting rather than emitting on every attack.
- **FR-005**: The companion addon MUST treat a locked or none bar as indeterminate
  and hold the last good value, and MUST re-establish a fresh baseline after a
  loading screen and on death and revive.
- **FR-006**: The weapon-bar signal MUST be relayed through a new pixel-bus block
  appended to the existing blocks, without altering the meaning of the existing
  status, fishing, and latency blocks.
- **FR-007**: The companion addon change MUST preserve the managed-marker line so the
  safe-uninstall verification continues to pass, and MUST bump the addon manifest
  version metadata.

Decode and timing:

- **FR-008**: The app MUST decode the new block into an active-bar signal and the two
  bars' weapon classes, and MUST expose these to the rest of the app.
- **FR-009**: The app MUST maintain a front and a back timing profile and select the
  active profile at runtime from the decoded active-bar signal.
- **FR-010**: The app MUST provide weapon-class default presets for the delays, with
  a shorter heavy-attack delay for faster-heavy weapon classes and a longer one for
  slower-heavy classes.
- **FR-011**: The app MUST provide a single "auto timing from weapon" preference;
  when on, each bar's delays follow that bar's detected weapon-class preset; when
  off, the user's manual per-bar delays are used.
- **FR-012**: Turning auto timing off MUST preserve the user's manual per-bar delay
  values entered previously.
- **FR-013**: When the active bar or a weapon class cannot be determined, or no
  weapon-bar signal is present at all, the app MUST fall back to the front (primary)
  timing profile and report weapon classes as unknown, rather than erroring or using
  stale data. This degrades to the prior single-profile behavior for users without
  the updated addon.

Visibility and persistence:

- **FR-014**: The app MUST display the detected active bar and each bar's weapon
  class, updating as they change and showing an unknown state when no signal is
  present.
- **FR-015**: The per-bar timing profiles and the auto-timing preference MUST be
  persisted and restored across restarts.

Research and process:

- **FR-016**: The project MUST record an evidence-based timing appendix in the master
  specification: the global-cooldown context, the per-weapon-class heavy-attack
  defaults with sources, and the in-game validation still owed, and MUST mark the
  corresponding open research item closed.
- **FR-017**: Changes to pinned contract surfaces (the pixel-bus contract, the reader
  contract, and the addon manifest) MUST each be accompanied by a dated decision
  recorded in the changelog.
- **FR-018**: All correctness-bearing logic (the weapon-class mapping mirrored by the
  reader, the block decode, the active-bar profile selection, and the presets) MUST
  live in the tested modules with unit tests; the rendering layer stays thin.

### Key Entities

- **Active bar signal**: The currently active weapon bar (front, back, or unknown)
  as determined by the addon and decoded by the app.
- **Weapon class**: A normalized category of the weapon equipped on a bar, used to
  choose a timing preset; distinct from the raw game weapon type.
- **Timing profile**: A set of delays (light, heavy, bash) for one bar; the app holds
  a front and a back profile and selects one by the active bar.
- **Weapon-class preset**: The default delays associated with a weapon class,
  applied per bar when auto timing is on.
- **Weapon-bar block**: The new pixel-bus block carrying the active bar and both
  bars' weapon classes.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With distinct front and back profiles, a reported bar change switches
  the effective timing profile, and an indeterminate or repeated report does not
  change it.
- **SC-002**: With auto timing on, each bar's heavy-attack delay equals the preset
  for that bar's detected weapon class, and a faster-heavy class yields a shorter
  delay than a slower-heavy class.
- **SC-003**: Turning auto timing off restores and uses the user's manual per-bar
  values, which were preserved while auto was on.
- **SC-004**: The app displays the active bar and both weapon classes and updates
  them on change, showing an unknown state with no signal.
- **SC-005**: Per-bar timing and the auto-timing preference survive an application
  restart.
- **SC-006**: The existing status, fishing, and latency signals continue to decode
  correctly after the new block is added, and safe uninstall still verifies the
  managed marker.
- **SC-007**: The specification appendix records the defaults and their sources and
  marks the research item closed, and the shipped presets match it.

## Assumptions

- Per-bar applies to the timing profile (the delays), not to the seven skill slots or
  their key mappings; the active bar selects which profile the slots use.
- The companion addon can read the active weapon pair and the equipped weapon types
  from the game and compute a normalized class, and can relay them through a new
  pixel block within the existing signal surface.
- The heavy-attack preset values are community estimates pending in-game validation,
  and the one-hand-and-shield value is a flagged estimate; presets are adjustable
  configuration, so shipping estimates is acceptable and does not block the design.
- The pixel signal, the exact preset timings, and the one-hand-and-shield duration
  require in-game verification that cannot be performed offline; the design and code
  are validated by automated tests, and in-game validation is an explicit follow-up.
- This slice stays within the bounded-scope contract: it reads only the rendered
  pixel signal, never game memory or network, and it traces to the master
  specification's timing, pixel-bus, and open-items sections.
