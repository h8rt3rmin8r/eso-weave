# Phase 0 Research: Latency-Adaptive Delays

All decisions were made under the Build-Phase Autopilot Protocol against the
constitution, the master specification (section 7.4), and existing code patterns.
None were escalated.

## Decision: Compute the adaptation in the pure sequence builder

- **Decision**: Add `effective_delay(base: u32, latency_ms: Option<u16>, cfg:
  &LatencyConfig) -> u32` and `sequence_for_adapted(slot, timing, latency_ms,
  cfg)` to `src/weave/sequence.rs`. The adapted builder applies `effective_delay`
  to the slot-resolved `d_weave` and `d_bash` only; `d_heavy` and the cooldown are
  untouched.
- **Rationale**: `sequence.rs` is where all weave timing already resolves (via
  `slot.d_weave(timing)` etc.), so applying the adaptation there computes the
  effective delay exactly where it is consumed and keeps the logic pure and
  unit-testable, satisfying Principle III and FR-007.
- **Alternatives considered**: Scaling a whole `TimingConfig` up front in the
  engine (rejected: would not respect per-slot overrides, which are resolved inside
  the builder). Applying the scaling in `RealSink::wait` (rejected: not pure, not
  testable, and would also scale non-weave waits).

## Decision: `sequence_for` delegates to the adapted builder with the feature disabled

- **Decision**: Keep the existing `pub fn sequence_for(slot, timing)` and have it
  call `sequence_for_adapted(slot, timing, None, &LatencyConfig::default())`.
- **Rationale**: `LatencyConfig::default()` is `enabled = false`, so
  `effective_delay` returns the base unchanged; existing callers and the S003 tests
  keep passing byte-for-byte, structurally guaranteeing the no-regression property
  (FR-008). No existing signature changes.
- **Alternatives considered**: Changing `sequence_for`'s signature to take latency
  (rejected: forces every caller and test to change and risks silent behavior
  drift). A separate parallel builder duplicating the match arms (rejected:
  duplication; the delegate keeps one source of truth).

## Decision: effective_delay clamps the bonus to `[0, 300]`

- **Decision**: `effective_delay` returns `base` when the feature is disabled or
  the latency is `None`; otherwise it computes `bonus = round(k * latency)` with
  `f64::round` (round-half-away-from-zero), clamps `bonus` to `[0, 300]`, and
  returns `base.saturating_add(bonus)`.
- **Rationale**: Clamping the bonus to `[0, 300]` is equivalent to clamping the
  effective delay to `[base, base + 300]` (FR-001) because the bonus is added to
  `base`; it also makes a non-positive bonus (small latency, `k = 0`, or a
  negative `k` that slipped through) never shorten the base, and `saturating_add`
  prevents overflow. `f64::round` is exactly round-half-away-from-zero (FR-002).
- **Alternatives considered**: Clamping the final delay with `min`/`max` on the sum
  (equivalent but less direct). Integer-only fixed-point arithmetic for `k`
  (rejected: `k` is a user-facing decimal; `f64` with a single round is simpler and
  the range is tiny).

## Decision: The engine holds `current_latency: Option<u16>` with a `set_latency` intake

- **Decision**: `WeaveEngine` gains `latency: LatencyConfig` and `current_latency:
  Option<u16>`, plus `set_latency(&mut self, latency: Option<u16>)`.
  `WeaveEngine::handle` builds sequences via `sequence_for_adapted(&slot,
  &self.config.timing, self.current_latency, &self.latency)`.
- **Rationale**: A single `Option<u16>` intake models both a fresh Latency value
  (`Some(ms)`) and the loss of the signal (`None`), satisfying FR-006 (clearing
  reverts to base). The worker loop that maps reader `PixelBusEvent::Latency` and
  `SignalLost` onto `set_latency` is out of scope; `set_latency` is the seam and is
  tested directly.
- **Alternatives considered**: Separate `on_latency(ms)` and `clear_latency()`
  methods (rejected: one `Option` intake is simpler and expresses the states
  directly). Storing latency inside `LatencyConfig` (rejected: config is persisted
  user settings; the current latency is transient runtime state and must not be
  written to the config file per the constitution).

## Decision: Persist `LatencyConfig` as a new additive `latency` settings section

- **Decision**: Add `latency: serde_json::Value` to `Settings`, owned by the weave
  module, with `WeaveEngine::load`/`store` extended to read and write it. `k` is
  validated as finite and within `[0.0, 4.0]`, falling back to 0.25 with an
  `InvalidValue` notice; `enabled` defaults to false.
- **Rationale**: Mirrors the established additive opaque-section pattern (`timing`,
  `skills`, `beacon`, `fishing`) and the constitution's additive-settings rule.
  Keeping it separate from the `timing` section preserves `TimingConfig`/`TimingJson`
  as strictly the four base delays.
- **Alternatives considered**: Folding `enabled`/`k` into the `timing` section
  (rejected: mixes base delays with the adaptation concern and complicates
  `TimingJson`). A typed field on `Settings` (rejected: breaks the opaque-section
  pattern and couples config to the weave module).
