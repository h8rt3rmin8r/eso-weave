# Tasks: Latency-Adaptive Delays

**Feature**: `specs/008-latency-adaptive-delays` | **Branch**: `008-latency-adaptive-delays`

Test-first per constitution Principle III. The correctness-bearing logic (the
effective-delay formula and its integration) is pure and tested with crafted
latency and configurations before it lands. The key guarantee (no regression to
the existing base sequences when off or without latency) is covered by required
tests. Paths are repository-relative.

## Phase 1: Setup

- [x] T001 Add `LatencyConfig` (fields `enabled: bool` default false, `k: f64` default 0.25) with a `Default` impl, and the `MAX_LATENCY_BONUS_MS: u32 = 300` constant, to `src/weave/types.rs`; re-export `LatencyConfig` and `MAX_LATENCY_BONUS_MS` from `src/weave/mod.rs`.

## Phase 2: Foundational

(None beyond setup; the pure formula is authored under US1 below.)

## Phase 3: User Story 1 - Delays scale with latency when enabled (P1)

- [x] T002 [P] [US1] Write failing tests in `tests/weave_latency.rs`: `effective_delay(50, Some(120), &{enabled:true, k:0.25})` is 80; light-attack, bash-attack, and block-casting sequences built via `sequence_for_adapted` scale `d_weave` (and `d_bash` for bash) by the bonus; a heavy-attack sequence's `d_heavy` wait is unchanged.
- [x] T003 [US1] Implement `effective_delay(base, latency_ms, cfg)` (round-half-away-from-zero, bonus clamped to `[0, MAX_LATENCY_BONUS_MS]`, saturating add) and `sequence_for_adapted(slot, timing, latency_ms, cfg)` in `src/weave/sequence.rs`, applying the effective delay to `d_weave` and `d_bash` only (FR-001 to FR-003, FR-007). Run US1 tests to green.

## Phase 4: User Story 2 - Off by default and safe without data (P1, no-regression)

- [x] T004 [P] [US2] Write failing tests in `tests/weave_latency.rs`: with the default `LatencyConfig`, `sequence_for_adapted(slot, timing, Some(latency), &default)` equals `sequence_for(slot, timing)` for every weave type; with the feature enabled but `latency == None`, the sequence equals the base sequence; `d_heavy` and (via the engine) `global_cooldown` are unaffected at every latency.
- [x] T005 [US2] Implement `sequence_for` as a delegate to `sequence_for_adapted(slot, timing, None, &LatencyConfig::default())` in `src/weave/sequence.rs` (FR-004, FR-005, FR-008). Run US2 tests to green and confirm the existing `tests/weave_sequence.rs` stays green (structural no-regression).

## Phase 5: User Story 3 - Bounds keep the adjustment sane (P2)

- [x] T006 [P] [US3] Write failing tests in `tests/weave_latency.rs`: a latency/`k` whose `round(k*latency)` exceeds 300 yields `effective_delay == base + 300` (cap inclusive at exactly 300); `k = 0.0` and a latency giving a zero bonus yield `effective_delay == base`; a latency that would overflow saturates within the clamp.
- [x] T007 [US3] Confirm the clamp and saturation in `effective_delay` satisfy the bounds (FR-001); adjust the implementation only if a bound test fails. Run US3 tests to green.

## Phase 6: User Story 4 - Configurable and persisted (P2)

- [x] T008 [P] [US4] Write failing tests in `tests/weave_latency.rs`: a `WeaveEngine` with an enabled config and a custom `k` round-trips through `store`/`load`; an absent `latency` section yields defaults (off, `k = 0.25`); a non-finite or out-of-range `k` falls back to 0.25 with an `InvalidValue` notice.
- [x] T009 [US4] Add the additive opaque `latency` section to `Settings` in `src/config/mod.rs` (field, `RawSettings`, default, known-keys) and extend `WeaveEngine::load`/`store` in `src/weave/mod.rs` to read/write `LatencyConfig` (validate `k` finite in `[0.0, 4.0]`, fall back to 0.25 with a notice; `enabled` defaults false) (FR-009, FR-010). Run US4 tests to green.

## Phase 7: Engine intake and integration

- [x] T010 [P] Write failing tests in `tests/weave_latency.rs`: with the feature enabled, `set_latency(Some(200))` then a handled weave (through a `MockSink`) produces scaled `d_weave`/`d_bash` waits; `set_latency(None)` then a handled weave produces base waits; `set_latency` never affects `global_cooldown`.
- [x] T011 Add `latency: LatencyConfig` and `current_latency: Option<u16>` fields to `WeaveEngine`, the `set_latency(Option<u16>)`, `latency_config`, and `set_latency_config` methods, and change `WeaveEngine::handle` to build via `sequence_for_adapted(&slot, &self.config.timing, self.current_latency, &self.latency)` in `src/weave/mod.rs` (FR-006, FR-007). Run intake tests to green.

## Phase 8: Polish and cross-cutting

- [x] T012 [P] Add or update module and item documentation across `src/weave/sequence.rs`, `src/weave/types.rs`, and the new `WeaveEngine` items.
- [x] T013 Update `CHANGELOG.md` `[Unreleased]` with an Added line for latency-adaptive delays and a dated Decisions entry for the pure effective-delay computation in the sequence builder, the `sequence_for`-delegates-for-no-regression approach, and the additive `latency` settings section.
- [x] T014 Run CI parity: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`, plus `cargo check --target x86_64-unknown-linux-gnu`.

## Dependencies and order

- Setup (T001) first. US1 (T002 to T003) authors the pure formula and adapted
  builder. US2 (T004 to T005) adds the delegate and the no-regression guarantee on
  top. US3 (T006 to T007) pins the bounds (largely already satisfied by T003). US4
  (T008 to T009) is the config surface. The engine intake and integration (T010 to
  T011) depend on the config type and the adapted builder. Polish (T012 to T014)
  last; T014 is the gate.

## Parallel opportunities

- The test-authoring tasks (T002, T004, T006, T008, T010) share
  `tests/weave_latency.rs` and land sequentially despite the `[P]` marker.
- Documentation (T012) is independent once the code is in place.

## MVP scope

US1 (scaling when enabled) plus US2 (off-by-default and no-regression) are the
minimum viable increment: the effective-delay computation integrated into the
builder with the guarantee that existing timing is unchanged unless the feature is
explicitly enabled with live latency. Bounds hardening, config persistence, and the
engine intake complete the feature.
