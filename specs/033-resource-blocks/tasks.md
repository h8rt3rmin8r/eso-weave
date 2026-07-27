# Tasks: PixelBeacon Resource Blocks

**Feature**: 033-resource-blocks | **Date**: 2026-07-27
**Input**: [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/pixel-bus-resources.md](contracts/pixel-bus-resources.md), [quickstart.md](quickstart.md)

Test-first per constitution principle III.

## Phase 1: Setup

- [x] T001 Confirm the baseline is green: fmt, clippy at deny-warnings, and the full suite, all in the foreground.

## Phase 2: Foundational

- [x] T002 Raise `NUM_BLOCKS` to 9 in `src/pixelbus/mod.rs`, add `health_point`, `stamina_point`, and `magicka_point` from `block_center(block_px, 6..8)`, and update the two tests in `tests/pixelbus.rs` that pin the count and the geometry contract table (the table currently enumerates six centres and six-block capture widths). [FR-017]
- [x] T003 Add `health`, `stamina`, and `magicka` fields to `BlockSamples`. The derived `Default` means no existing construction changes; confirm that holds. [FR-017]

## Phase 3: User Story 2, a misread is small never wild (P1)

Sequenced before User Story 1 because the bounded-error property is the reason this
encoding was chosen over the one the issue specified, and proving it first stops
the decoder being written to a weaker standard.

### Tests

- [x] T004 [US2] Write the failing exhaustive bounded-error test in `tests/pixelbus.rs`: for every publishable percentage 0 to 100, and every perturbation of the payload and checksum channels within the default tolerance, assert decoding yields either a value within tolerance or unavailable, and never a different percentage. [FR-009, SC-002]
- [x] T005 [P] [US2] Write the failing monotonicity test in `tests/pixelbus.rs`: over the full range, a greater published percentage never decodes below a lesser one. [FR-010, SC-003]
- [x] T006 [P] [US2] Write the failing rejection tests: a wrong marker, a broken checksum, and a payload well above 100 (including the explicit unavailable value) each yield unavailable, AND that a payload just above 100 (within tolerance) decodes to 100 rather than being rejected, so a full resource does not flap. [FR-008, FR-003]

### Implementation

- [x] T007 [US2] Add `ResourceLevel`, `ResourceSet`, the three markers (`0x16`, `0x6D`, `0xBB`), `RESOURCE_UNAVAILABLE`, the three `BLOCK_CENTER_GREENS` entries, and `decode_resource(sample, marker, tolerance)` to `src/pixelbus/mod.rs`, following the latency block's marker-and-checksum shape. One decoder serves all three blocks; the marker is a parameter. [FR-004, FR-008]

**Checkpoint**: the encoding's correctness property is proven.

## Phase 4: User Story 1, the operator can see their resources (P1)

### Tests

- [x] T008 [P] [US1] Write the failing arbitrary-colour test in `tests/pixelbus.rs`: sweep the colour cube against each of the three markers and assert nothing outside the encoding decodes as a percentage, including every other block's rendered colour. [FR-008, SC-004]
- [x] T009 [P] [US1] Write the failing independence test: a bad sample for one resource leaves the other two decoding correctly. [FR-011]
- [x] T010 [P] [US1] Write the failing reader tests: a change emits one event, a steady set emits none, and each resource clears on signal loss and on a non-decoding block. [FR-006, FR-012, SC-006]
- [x] T011 [P] [US1] Write the failing marker-registry test extension: ten block-centre greens remain pairwise separated beyond the default tolerance. [FR-004]
- [x] T012 [P] [US1] Write the failing view tests in `tests/app_view_model.rs` for a percentage and for not-detected, and the routing test. [FR-013]
- [x] T013 [P] [US1] Write the failing boundary test in `tests/weave_engine.rs`: engine behavior is identical across resource values. [FR-015]

### Implementation

- [x] T014 [US1] Add the reader's resource state, the `PixelBusEvent::Resources` variant carrying a `ResourceSet`, decode-and-emit-on-change wiring in `observe`, the three samples in `sample_and_observe`, and a **trace**-level log line on change. Not debug: see FR-014. [FR-006, FR-012, FR-014]
- [x] T015 [US1] Add resource storage to the weave engine (stored, read by nothing) and route `Resources` to it in `src/app/routing.rs`. Confirm `map_event` returns `None` for the new variant. [FR-015]
- [x] T016 [US1] Add `ResourceView` and its derivation to `src/app/mod.rs`, the `AppView` fields, strings, and three rows in `src/app/ui.rs` after the menu row. [FR-013]
- [x] T017 [US1] In `addon/PixelBeacon/PixelBeacon.lua`: raise `NUM_BLOCKS` to 9, add the three markers and the unavailable value, create and position the three blocks, and add a change-detected `renderResources()` computing each percentage from `GetUnitPower("player", COMBAT_MECHANIC_FLAGS_*)` against the current maximum, publishing the unavailable value when a maximum is zero or unreadable. [FR-001, FR-002, FR-003, FR-005, FR-006]
- [x] T018 [US1] Wire `EVENT_POWER_UPDATE` for the three pools and re-read on the existing fast tick as a backstop, so the blocks track play rather than lag a tick. [FR-007]
- [x] T019 [US1] Advance `addon/PixelBeacon/PixelBeacon.txt` to version 8 on both version lines, extend the description, and advance the version-pin test in `tests/beacon.rs`. [FR-018]

## Phase 5: User Story 3, no false readings from an older addon (P2)

- [x] T020 [US3] Confirm the compatibility path with a test: with no resource blocks sampled, all three report unavailable and no event is emitted. Largely covered by T008 and T010; this states it as the compatibility guarantee. [SC-005]

## Phase 6: Polish

- [x] T021 [P] Extend the cross-language agreement test in `tests/beacon.rs` to `NUM_BLOCKS` and the three markers. [FR-017]
- [x] T022 [P] Update section 10.3 of `docs/ESO-Weave-Specification-v0.2.0.md` with B6 to B8, the numeric encoding, the bounded-error guarantee, and the trace-level note. [FR-019]
- [x] T023 [P] Update `CHANGELOG.md`: an `Added` entry plus a dated decision for the encoding reversal and one for the marker exhaustion note. [FR-019]
- [x] T024 Run the full merge gate in the foreground. [Constitution IV, SC-007]
- [x] T025 Verify against the diff that no safety surface or existing test assertion changed, and that no resource value is read in any decision path. [FR-015, FR-016]
- [x] T026 Verify text hygiene across every touched file.

## Dependencies

```text
T001 -> T002, T003 -> Phase 3 (T004 to T006 then T007)
                   -> Phase 4 (T008 to T013 then T014 to T019)
                   -> Phase 5 -> Phase 6 (T021 to T023 parallel; then T024 to T026)
```

## Implementation strategy

Phases 1 to 4 are the shippable feature. Phase 3 leads because the bounded-error
proof is the justification for the whole encoding decision; writing the decoder
first would invite a decoder that merely works on the happy path.

## Analyze gate record (2026-07-27)

Zero CRITICAL findings; no early halt. Coverage was already 20 of 20 FRs. One
substantive finding, fixed:

- **A full resource would have flapped to unavailable.** The decode rule rejected
  any payload above 100, but a published 100 with upward capture drift reads 101 or
  102. Full is the normal out-of-combat state, so the most common value would have
  been the least stable reading on the strip. The rule now accepts up to
  `100 + tolerance` and clamps, which keeps the error inside tolerance and makes the
  top of the range as stable as the middle. Bounded error is preserved; the explicit
  unavailable payload sits far enough away that it is unaffected.

## Completion record (2026-07-27)

All tasks complete. Notes worth keeping:

- The colour-table deliverable that issue #2 named as gating was never built,
  because the encoding decision removed the need for it. What replaced it is a
  test: the bounded-error property is enumerated over the full publishable range
  crossed with every in-tolerance perturbation of all three channels. That test
  cannot be written for a lookup table, which is the clearest statement of why the
  encodings are not equivalent.
- The addon-side unavailable payload constant was initially unused in Rust and
  tripped the dead-code lint. Rather than silencing it, it was made public and the
  cross-language test now asserts the Lua value equals it, which is a stronger
  check than the literal that was there before.
- The clamp for payloads just above 100 came from the analyze gate, not from
  implementation. Without it a full pool would have flapped to unavailable on any
  upward capture drift.
