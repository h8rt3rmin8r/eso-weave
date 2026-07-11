# Feature Specification: Latency-Adaptive Delays

**Feature Branch**: `008-latency-adaptive-delays`

**Created**: 2026-07-11

**Status**: Draft

**Input**: User description: "Latency-Adaptive Delays per master specification section 7.4. Enhance the weave engine so that, when latency data is available from the pixel bus, it scales two of the weave delays in real time by the formula effective_delay = base_delay + round(k * latency_ms), where base_delay is the per-slot-resolved (override-or-global) delay. The adjustment applies to d_weave and d_bash only; d_heavy and global_cooldown are never scaled. k defaults to 0.25 and is user-configurable. effective_delay is clamped to [base_delay, base_delay + 300]. The feature is off by default and requires PixelBeacon latency data: when disabled, or when no latency value is currently known (for example the beacon signal is lost), the engine uses the base delays unchanged. The weave engine consumes latency values that the Pixel Bus Reader produces (Latency events) and clears its current latency when the signal is lost. The adaptation lives in the pure sequence builder so it is unit-tested with crafted latency and configurations; the latency-adaptation config (enabled flag and k) persists as an additive settings section. Depends on features 003 (weave engine) and 005 (pixel bus reader). Excludes the GUI wiring and the worker loop that feeds Latency events to the engine; this slice delivers the effective-delay computation, its integration into weave sequence building, the engine's latency-value intake, and the config persistence."

## Clarifications

### Session 2026-07-11

Resolved under the Build-Phase Autopilot Protocol from the master specification
(section 7.4) and the constitution (no options were escalated).

- Q: What exactly is `base_delay` in the formula? -> A: The slot-resolved delay:
  the per-slot override when present, otherwise the global default. The latency
  bonus is applied on top of that resolved base, so a slot with a longer override
  scales from its own base, not the global one.
- Q: Which delays are scaled and which are not? -> A: Only `d_weave` and `d_bash`.
  `d_heavy` and `global_cooldown` are never scaled: the heavy-attack hold and the
  cooldown gate keep their base values regardless of latency.
- Q: How is the bonus rounded and clamped? -> A: The bonus is `round(k *
  latency_ms)` using round-half-away-from-zero, then the effective delay is clamped
  to `[base_delay, base_delay + 300]`. Equivalently the added bonus is clamped to
  `[0, 300]`, so a non-positive bonus never shortens the base and a large latency
  never adds more than 300 ms.
- Q: When are base delays used unchanged? -> A: Whenever the feature is disabled
  (the default), or no current latency value is known. A current latency value is
  known only while the beacon is reporting it; on signal loss the engine clears its
  current latency, so it reverts to base delays until a new latency arrives.
- Q: Where does the latency value come from and how is it fed in? -> A: From the
  Pixel Bus Reader's Latency events. The weave engine exposes an intake that sets
  the current latency (to a value on a Latency event, or to none on signal loss);
  the worker loop that actually pumps reader events into this intake is out of
  scope for this slice.
- Q: How is the adaptation persisted? -> A: As an additive settings section
  holding the enabled flag (default off) and `k` (default 0.25), following the
  project's additive, user-settings-only pattern. An out-of-range or invalid value
  falls back to its default with a surfaced notice.
- Q: What makes a persisted `k` invalid? -> A: `k` MUST be a finite number in the
  range `[0.0, 4.0]`; a value that is not finite (NaN or infinity) or that lies
  outside that range falls back to the default 0.25 with a surfaced notice. The
  upper bound of 4.0 is generous (even a 100 ms latency at `k = 4.0` already
  saturates the 300 ms cap) and simply rejects nonsensical input; `k = 0.0` is
  valid and makes the scaling inert.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Delays scale with latency when enabled (Priority: P1)

A player on a high-latency connection enables latency-adaptive delays so the
weave gaps stretch to match server latency, and the weaves land reliably instead
of being eaten by lag.

**Why this priority**: This is the entire feature: turning a known latency into a
proportionally longer weave gap. Without it there is nothing to deliver.

**Independent Test**: With the feature enabled and a crafted current latency, build
a weave sequence and confirm the `d_weave` and `d_bash` waits equal
`base + round(k * latency)` (clamped), while `d_heavy` and the cooldown are
unchanged.

**Acceptance Scenarios**:

1. **Given** the feature is enabled with `k = 0.25` and a current latency of 120
   ms, **When** a light-attack weave sequence is built, **Then** its `d_weave`
   wait equals `base_d_weave + 30`.
2. **Given** the feature is enabled with a current latency of 120 ms, **When** a
   bash-attack weave sequence is built, **Then** its `d_weave` and `d_bash` waits
   are each scaled by `+30` and its other steps are unchanged.
3. **Given** the feature is enabled, **When** a heavy-attack weave sequence is
   built, **Then** its `d_heavy` wait is unchanged (heavy is never scaled).

---

### User Story 2 - Off by default and safe without data (Priority: P1)

By default, and whenever no live latency is known, the engine uses the base delays
exactly as before, so enabling PixelBeacon is never required and a lost signal
never corrupts timing.

**Why this priority**: The feature must be strictly opt-in and must degrade to the
existing, well-tested base timing. A regression here would change every user's
weave timing unexpectedly.

**Independent Test**: With the feature disabled (default), build sequences and
confirm the delays equal the base delays; enable it but provide no current latency
and confirm the delays are still the base delays.

**Acceptance Scenarios**:

1. **Given** the default configuration (feature off), **When** any weave sequence
   is built, **Then** every delay equals its base value.
2. **Given** the feature is enabled but no current latency is known, **When** a
   sequence is built, **Then** every delay equals its base value.
3. **Given** the feature is enabled and a latency was known, **When** the signal
   is lost (the current latency is cleared), **Then** subsequent sequences use base
   delays until a new latency is known.

---

### User Story 3 - Bounds keep the adjustment sane (Priority: P2)

The added delay never shortens a weave and never balloons without limit, so an
extreme or spurious latency reading cannot make weaves fire early or stall for
seconds.

**Why this priority**: The clamp is the guardrail that keeps a bad latency reading
from producing unusable timing; it matters but sits on top of the core scaling.

**Independent Test**: Build sequences at a latency that would exceed the cap and
confirm the effective delay is `base + 300`; at a latency or `k` that would produce
a non-positive bonus, confirm the effective delay is exactly the base.

**Acceptance Scenarios**:

1. **Given** the feature is enabled, **When** `round(k * latency)` exceeds 300,
   **Then** the effective `d_weave` and `d_bash` equal `base + 300`.
2. **Given** the feature is enabled, **When** `round(k * latency)` is zero or
   negative, **Then** the effective delay equals the base (never less).

---

### User Story 4 - Configurable and persisted (Priority: P2)

A player can turn the feature on or off and adjust `k`, and those choices persist
across restarts.

**Why this priority**: The defaults ship the feature off; configurability lets a
player opt in and tune it, but it is not required for the core computation.

**Independent Test**: Round-trip a latency-adaptation configuration (enabled and a
custom `k`) through the settings section and confirm the loaded values equal the
saved ones; confirm an absent section yields the documented defaults and an invalid
value falls back with a notice.

**Acceptance Scenarios**:

1. **Given** a latency-adaptation configuration with custom values, **When** it is
   saved and reloaded, **Then** the reloaded values equal the saved values.
2. **Given** no latency-adaptation settings section, **When** the configuration is
   loaded, **Then** the defaults are used (feature off, `k = 0.25`).
3. **Given** an invalid `k` in the section, **When** it is loaded, **Then** it
   falls back to the default and a notice is surfaced.

---

### Edge Cases

- What happens at exactly the clamp boundary (`round(k * latency) == 300`)? The
  effective delay is `base + 300`; the cap is inclusive.
- What happens when a per-slot override sets a larger base `d_weave`? The bonus is
  computed from and added to that override base, and clamped relative to it.
- What happens if `k` is zero? The bonus is zero and every scaled delay equals its
  base (the feature is effectively inert while still enabled).
- What happens if a latency value arrives while the feature is disabled? The
  current latency may be tracked, but no scaling is applied while disabled.
- What happens to `global_cooldown` and `d_heavy` at any latency? They are never
  scaled.
- What happens when the effective delay would overflow? The computation saturates
  rather than wrapping; the clamp keeps the result within `[base, base + 300]`.

## Requirements *(mandatory)*

### Functional Requirements

Effective-delay computation:

- **FR-001**: The engine MUST compute an effective delay as `base_delay +
  clamp(round(k * latency_ms), 0, 300)`, where `base_delay` is the slot-resolved
  (override-or-global) delay, so the effective delay always lies in
  `[base_delay, base_delay + 300]`.
- **FR-002**: The rounding MUST be round-half-away-from-zero on `k * latency_ms`.
- **FR-003**: The scaling MUST apply only to `d_weave` and `d_bash`. `d_heavy` and
  `global_cooldown` MUST NOT be scaled at any latency.

Gating on enablement and data:

- **FR-004**: The feature MUST be off by default; when off, every weave delay MUST
  equal its base value.
- **FR-005**: When the feature is on but no current latency value is known, every
  weave delay MUST equal its base value.
- **FR-006**: The weave engine MUST expose an intake that sets the current latency
  to a known value or clears it; clearing it (as on signal loss) MUST revert to
  base delays until a new latency is set.

Integration:

- **FR-007**: The effective-delay computation MUST be applied within the pure weave
  sequence builder so that the produced `d_weave` and `d_bash` waits reflect the
  effective delays, and it MUST be unit-testable with a crafted latency and
  configuration, independent of any real reader or clock.
- **FR-008**: The existing base sequence building (the feature off, or no latency)
  MUST remain byte-for-byte identical to the current behavior, so no existing weave
  timing changes unless the feature is explicitly enabled with live latency.

Configuration:

- **FR-009**: The latency-adaptation configuration (an enabled flag defaulting to
  off, and `k` defaulting to 0.25) MUST be user-configurable. A valid `k` is a
  finite number in `[0.0, 4.0]`.
- **FR-010**: The configuration MUST persist as an additive settings section; an
  absent section MUST yield the defaults, and a `k` that is not finite or lies
  outside `[0.0, 4.0]` MUST fall back to the default 0.25 with a surfaced notice.

### Key Entities *(include if data involved)*

- **Latency Adaptation Config**: The enabled flag (default off) and the scale
  factor `k` (default 0.25).
- **Current Latency**: The most recent known server latency in milliseconds, or
  none when unknown (default and after signal loss).
- **Effective Delay**: A base delay adjusted by the clamped latency bonus, used for
  `d_weave` and `d_bash` only.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With the feature enabled and a known latency, the `d_weave` and
  `d_bash` waits equal `base + clamp(round(k * latency), 0, 300)` for
  representative latencies including the clamp boundary, in 100 percent of cases.
- **SC-002**: `d_heavy` and `global_cooldown` are unchanged at every latency, in
  100 percent of cases.
- **SC-003**: With the feature off, or on with no known latency, every delay equals
  its base value, in 100 percent of cases (byte-for-byte identical sequences to the
  pre-feature behavior).
- **SC-004**: A saved latency-adaptation configuration reloads equal to what was
  saved, and an absent section yields the defaults, in 100 percent of cases.
- **SC-005**: All computation and integration behavior is verifiable without a real
  reader, latency source, or clock, using crafted latency values and
  configurations.

## Assumptions

- Scope is master specification section 7.4. The GUI controls to enable the feature
  and set `k`, and the worker loop that pumps the Pixel Bus Reader's Latency and
  signal-loss events into the engine's intake, are out of scope and wired later.
- Feature 003 (weave engine) provides the base timing model, the per-slot override
  resolution, and the pure sequence builder this enhances; feature 005 (pixel bus
  reader) provides the Latency events whose value the engine consumes. Both exist.
- The latency value is the server latency in milliseconds as decoded by the reader;
  this slice does not re-derive or validate it beyond treating an absent value as
  "no data."
- The latency-adaptation settings section follows the project's additive,
  user-settings-only configuration pattern and requires no configuration schema
  version change.
