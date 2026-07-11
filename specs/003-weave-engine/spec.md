# Feature Specification: Weave Engine

**Feature Branch**: `003-weave-engine`

**Created**: 2026-07-11

**Status**: Draft

**Input**: User description: "Weave Engine per master specification section 7 (7.1 to 7.3, excluding 7.4 latency adaptation). Seven skill slots with weave types and per-slot delay overrides, four fixed weave sequences using primary and secondary mouse plus the skill key, a global timing model with cooldown gating, inactive-slot pass-through, executed on the worker thread on top of the S002 Input Engine. Excludes latency adaptation, fishing, PixelBeacon, and GUI."

## Clarifications

### Session 2026-07-11

Resolved under the Build-Phase Autopilot Protocol from the master specification
and the constitution (no options were escalated).

- Q: How does an inactive slot pass its key through given S002 suppresses bound
  keys? -> A: Slot activity feeds the Input Engine: a bound key whose slot is
  inactive is not suppressed and not handed off, so it reaches the game
  unmodified. Only active slots are intercepted.
- Q: What consumes the handed-off actions and drives execution? -> A: A worker
  drains the S002 hand-off channel; each action maps to its slot, and if the slot
  is active and the global cooldown has elapsed, the slot's sequence is executed.
- Q: How is cooldown measured and enforced? -> A: Against a monotonic clock; the
  interval since the last executed weave must be at least global_cooldown, else
  the request is dropped. The clock is an injected seam so timing is testable
  without real waiting.
- Q: How do weave sequences produce mouse and key input? -> A: Through a
  synthesizer seam that emits primitive operations (mouse or key, down or up) and
  waits; a real synthesizer drives the Input Engine and the OS, and a mock
  records the ordered operations and waits for tests.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Correct weave sequences per type (Priority: P1)

Pressing an active skill slot's key executes that slot's weave sequence: the
correct ordered mix of primary and secondary mouse actions, the skill key, and
the configured waits, matching the weave type. The four weave types each produce
their defined sequence.

**Why this priority**: Producing the correct sequence is the engine's core value;
a wrong order or a missing step makes the weave useless or harmful.

**Independent Test**: For each weave type, drive the slot's action through the
engine with a mock synthesizer and a virtual clock and assert the exact ordered
sequence of emitted operations and waits.

**Acceptance Scenarios**:

1. **Given** a slot set to Light Attack, **When** its action fires, **Then** the
   engine emits a primary click, waits d_weave, then sends the skill key.
2. **Given** a slot set to Heavy Attack, **When** its action fires, **Then** the
   engine emits primary down, waits d_heavy, sends the skill key, then primary up.
3. **Given** a slot set to Bash Attack, **When** its action fires, **Then** the
   engine emits primary click, waits d_weave, sends the skill key, waits d_bash,
   secondary down, primary click, secondary up, in that order.
4. **Given** a slot set to Block Casting, **When** its action fires, **Then** the
   engine emits secondary down, sends the skill key, waits d_weave, then secondary
   up.

---

### User Story 2 - Cooldown gating (Priority: P1)

Weaves cannot fire faster than the global cooldown. A weave request that arrives
within the cooldown window after the last executed weave is dropped (its key was
already suppressed by the input layer). Once the cooldown has elapsed, the next
request executes.

**Why this priority**: Cooldown gating protects against overfiring and matches the
game's global cooldown; without it the engine would spam input.

**Independent Test**: With a virtual clock, fire two requests inside the cooldown
window and confirm only the first executes; advance the clock past the cooldown
and confirm the next request executes.

**Acceptance Scenarios**:

1. **Given** a weave has just executed, **When** another request arrives before
   global_cooldown has elapsed, **Then** it is dropped and no sequence runs.
2. **Given** the cooldown has elapsed, **When** a request arrives, **Then** its
   sequence executes and the cooldown restarts.

---

### User Story 3 - Inactive slots pass through (Priority: P2)

An inactive slot's key is not intercepted: it reaches the game unmodified, and no
weave runs. Activating the slot restores interception and weaving.

**Why this priority**: Slots 6 and 7 default inactive; misclassifying an inactive
slot as active would suppress a key the operator expects to work normally.

**Independent Test**: Mark a slot inactive and confirm the Input Engine passes its
key through (no suppression, no hand-off); mark it active and confirm interception
and weaving resume.

**Acceptance Scenarios**:

1. **Given** a slot is inactive, **When** its key is pressed while focused, **Then**
   the key passes through unmodified and no weave runs.
2. **Given** a slot is then activated, **When** its key is pressed while focused,
   **Then** the key is intercepted and the slot's sequence executes.

---

### User Story 4 - Configurable timing with per-slot overrides (Priority: P3)

Global timing values and per-slot overrides drive the waits in each sequence. A
per-slot override replaces the relevant global default for that slot only; a blank
override uses the global default. Slot and timing configuration persists across
restarts.

**Why this priority**: Skills with long cast times need per-slot gaps; without
overrides the operator cannot tune individual slots.

**Independent Test**: Set a per-slot d_weave override, fire that slot, and confirm
the emitted wait uses the override; fire another slot without an override and
confirm it uses the global default; reload configuration and confirm persistence.

**Acceptance Scenarios**:

1. **Given** a slot with a per-slot d_weave override, **When** it fires, **Then**
   the relevant wait uses the override value, not the global default.
2. **Given** a slot with no override, **When** it fires, **Then** the relevant
   wait uses the global default.
3. **Given** changed slot and timing configuration, **When** it is saved and
   reloaded, **Then** the values persist.

---

### Edge Cases

- What happens when an action arrives for a slot whose type does not use a given
  delay (for example d_bash for a Light Attack)? Only the delays relevant to the
  slot's weave type are applied; irrelevant overrides are ignored.
- What happens when two actions arrive nearly simultaneously? They are processed
  in order by the single worker; the second is subject to cooldown gating.
- What happens when a persisted timing value is missing or out of range? The
  missing or invalid value falls back to the global default with a notice.
- What happens when an action arrives for the suspend or fishing toggle? Those are
  not weave slots and are not executed as weaves by this engine.
- What happens when the engine is suspended mid-sequence? A new sequence does not
  start while suspended; the input layer already stops handing off non-exempt
  actions.

## Requirements *(mandatory)*

### Functional Requirements

Skill model:

- **FR-001**: The engine MUST model seven skill slots, each with a bound key, a
  weave type, an active flag, and per-slot delay overrides. Slots 6 and 7 are
  labeled Ultimate and Synergy.
- **FR-002**: An inactive slot MUST NOT be intercepted: its key passes through to
  the game unmodified and no weave runs. This is enforced by feeding slot activity
  to the Input Engine so only active slots are suppressed and handed off.

Weave sequences:

- **FR-003**: A Light Attack slot MUST execute: primary click, wait d_weave, send
  skill key.
- **FR-004**: A Heavy Attack slot MUST execute: primary down, wait d_heavy, send
  skill key, primary up.
- **FR-005**: A Bash Attack slot MUST execute: primary click, wait d_weave, send
  skill key, wait d_bash, secondary down, primary click, secondary up.
- **FR-006**: A Block Casting slot MUST execute: secondary down, send skill key,
  wait d_weave, secondary up.
- **FR-007**: Primary MUST be the left mouse button and secondary the right mouse
  button; the skill key MUST be the slot's bound key.

Timing and cooldown:

- **FR-008**: The engine MUST support global timing values in milliseconds with
  defaults global_cooldown 500, d_weave 50, d_heavy 1000, d_bash 125, all
  user-configurable.
- **FR-009**: Each slot MUST support per-slot overrides for the delays relevant to
  its weave type; a blank override MUST use the global default; overrides
  irrelevant to the type MUST be ignored.
- **FR-010**: The engine MUST NOT start a new weave within global_cooldown of the
  last executed weave; a request inside the window MUST be dropped (its key was
  already suppressed) and MUST NOT execute a sequence.
- **FR-011**: Cooldown MUST be measured against a monotonic clock exposed as a seam
  so timing is testable without real waiting.

Execution and integration:

- **FR-012**: The engine MUST drain the Input Engine hand-off channel on a worker
  and map each handed-off action to its slot.
- **FR-013**: Sequence execution MUST go through a synthesizer seam that emits
  primitive operations (mouse or key, down or up) and waits, so the engine is
  testable with a mock synthesizer and a virtual clock.
- **FR-014**: The engine MUST NOT execute the suspend or fishing toggle actions as
  weaves.

Persistence:

- **FR-015**: The slot configuration and the timing configuration MUST persist as
  additive settings sections that are backward compatible (absent sections load
  defaults, no schema version bump).
- **FR-016**: A persisted timing value that is missing or out of range MUST fall
  back to the relevant global default with a notice, rather than failing to load.

### Key Entities *(include if feature involves data)*

- **Skill Slot**: A bound key, a weave type, an active flag, and per-slot delay
  overrides. Seven slots; slots 6 and 7 are Ultimate and Synergy.
- **Weave Type**: One of Light Attack, Heavy Attack, Bash Attack, Block Casting,
  each defining a fixed operation sequence.
- **Timing Config**: The global delays (global_cooldown, d_weave, d_heavy, d_bash)
  in milliseconds.
- **Weave Step**: A primitive of the execution sequence, either an emitted
  operation (mouse or key, down or up) or a wait of a specified duration.
- **Input Operation**: A mouse button (primary or secondary) or key transition
  (down or up) synthesized during a sequence.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Each of the four weave types produces exactly its defined ordered
  sequence of operations and waits, verifiable for 100 percent of types.
- **SC-002**: Within the cooldown window, 0 percent of extra requests execute a
  sequence; after the window, the next request executes.
- **SC-003**: An inactive slot results in 0 interceptions and 0 weaves for its key;
  an active slot intercepts and weaves.
- **SC-004**: A per-slot override changes only that slot's relevant wait; other
  slots are unaffected, verifiable by comparing emitted waits.
- **SC-005**: Slot and timing configuration saved in one session is present and
  unchanged after a restart in 100 percent of cases.
- **SC-006**: Timing values are verifiable without real waiting, using the clock
  seam.

## Assumptions

- Scope is master specification section 7.1 through 7.3. Latency-adaptive delays
  (7.4), fishing, PixelBeacon, and the GUI are out of scope. Slot labels and the
  active flag are modeled here; their on-screen presentation and the capture-style
  rebinding control are the later GUI slice.
- The engine builds on the S002 Input Engine: it consumes handed-off actions,
  feeds slot activity back to the Input Engine's per-action activity, and
  synthesizes key and mouse input through the Input Engine's synthesis, extended
  with mouse buttons.
- The engine is platform-agnostic and unit-tested with a mock synthesizer and a
  virtual clock; the real synthesizer and real waiting are thin and run on the
  worker thread, never on the interception path.
- Configuration persists through the S001 Config Store as additive sections,
  consistent with the bindings section added in S002.
- The default slot keys and weave types follow master specification section 7.1
  (skills 1 to 5 active Light Attack; Ultimate and Synergy inactive Light Attack).
