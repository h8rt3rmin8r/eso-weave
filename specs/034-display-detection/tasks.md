# Tasks: Out-Of-Band Display Detection

**Feature**: 034-display-detection | **Date**: 2026-07-27
**Input**: [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/display-descriptor.md](contracts/display-descriptor.md), [quickstart.md](quickstart.md)

Test-first per constitution principle III. Every task below except the platform
probes and the worker wiring is desk-testable; the three that are not are the
ones that call the operating system, and they are deliberately thin.

## Phase 1: Setup

- [x] T001 Confirm the baseline is green: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all --locked`, all in the foreground and watched to completion.

## Phase 2: Foundational

Blocking prerequisites: the geometry primitives and the module every later phase
adds to.

- [x] T002 Create `src/pixelbus/display.rs` with `Size` and `Point` (both `Copy + Eq`), declare `mod display;` in `src/pixelbus/mod.rs`, and re-export the module's public items. Module docs state the feature's boundary: pure, no input or output, and the X11 multi-head limitation from plan Decision 7. [FR-002]
- [x] T003 Create `tests/pixelbus_display.rs` with the geometry-primitive tests, so the new suite exists and is wired before anything depends on it.

## Phase 3: User Story 3, the stored settings are read and never trusted over the screen (P2)

Sequenced first despite being P2. It is the only phase with no operating system
dependency at all, it is where every malformed-input requirement lives, and the
parser's output is an input to the reconciliation that User Story 1 needs. Doing
it first means the harder phases are built on a tested foundation rather than
beside an untested one.

### Tests

- [x] T004 [US3] Write the failing parser tests in `tests/pixelbus_display.rs` for a realistic settings file: assert every key of interest is extracted with its value, including both resolution pairs, the raw mode value, the display index, the overscan pair, and both interface-scale settings. [FR-011, US3-1]
- [x] T005 [P] [US3] Write the failing version-suffix test: `UseCustomUIScale.2` and a hypothetical `.3` both match the base key, and `FULLSCREEN` never matches `FullscreenWidth`. [FR-013, US3-2]
- [x] T006 [P] [US3] Write the failing tolerance tests: empty input, unrelated content, a line truncated mid-value, a line that is not a `SET`, an unparsable numeric value, a half-present resolution pair, a duplicate key (last wins), and an unknown key. Each yields an absent or partial reading and nothing panics. [FR-014, SC-004, US3-3]
- [x] T007 [P] [US3] Write the failing no-mapping test: an unrecognized window-mode value is carried raw, and no named window mode is produced from any value. [FR-015]

### Implementation

- [x] T008 [US3] Add `StoredVideoSettings` and `parse_user_settings(&str)` to `src/pixelbus/display.rs`, following the line grammar and key-normalization rules in [data-model.md](data-model.md). Total function: every input yields a value, no input panics. [FR-011, FR-013, FR-014, FR-015]
- [x] T009 [P] [US3] Write the failing path test in `tests/beacon.rs`: `user_settings_path` returns the AddOns directory's parent joined with `UserSettings.txt`, returns `None` for a root path with no parent, and creates nothing on disk. [FR-012, FR-017]
- [x] T010 [US3] Add `pub fn user_settings_path(addons_root: &Path) -> Option<PathBuf>` to `src/beacon/mod.rs`. Pure path composition: no filesystem access, no existence check, and above all no directory creation. [FR-012, FR-017]

**Checkpoint**: the file half of the feature is complete and cannot panic.

## Phase 4: User Story 1, the companion knows how big the render surface is (P1)

### Tests

- [x] T011 [P] [US1] Write the failing descriptor-construction tests: `from_measured` rejects a zero width, a zero height, and both; carries every supplied field; and leaves absent probe fields absent rather than defaulting them. [FR-001, FR-004, FR-006, SC-006]
- [x] T012 [P] [US1] Write the failing provenance test: a measured descriptor reports `Measured`, a configured one reports `Configured`, and `scale()` is `None` when the DPI is absent and `dpi / 96` when present. [FR-003, FR-006]
- [x] T013 [P] [US1] Write the failing configured-descriptor tests: produced only when both stored pairs are present, equal, and non-zero; not produced for differing pairs, a single present pair, no pairs, or a zero pair; and never carries display geometry or an origin. [FR-022, SC-007, US3-6, US3-7]
- [x] T014 [P] [US1] Write the failing seam test: `MockSampler` gains a settable measured display, and a sampler that does not override `display()` returns `None`. [FR-026]

### Implementation

- [x] T015 [US1] Add `MeasuredDisplay`, `DisplayDescriptor`, `DisplaySource`, `from_measured`, `from_stored`, and `scale()` to `src/pixelbus/display.rs`. [FR-001, FR-002, FR-003, FR-004, FR-022]
- [x] T016 [US1] Add the defaulted `fn display(&self) -> Option<MeasuredDisplay> { None }` to `SurfaceSampler` in `src/pixelbus/mod.rs`, and let `MockSampler` return a settable value. Confirm no existing implementation needs a change. [FR-026]
- [x] T017 [US1] Implement `display()` for `GdiSampler` in `src/pixelbus/windows.rs`: `ClientToScreen` and `GetClientRect` for the surface and its origin, `MonitorFromWindow` plus `GetMonitorInfoW` for the display rectangle, and `GetDpiForMonitor` with `MDT_EFFECTIVE_DPI` for the scale. Return `None` for a failed resolution or a non-positive client rectangle; degrade the monitor and DPI fields independently. No new Cargo feature. [FR-005, FR-006, FR-010]
- [x] T018 [US1] Implement `display()` for `X11Sampler` in `src/pixelbus/linux.rs` using core `xproto` only: `get_geometry` for the surface, `translate_coordinates` against the root for the origin, and the connection setup's screen dimensions for the display size. DPI is always `None`. Match the existing sampler's title check and its `?`-on-error style so a failed call yields `None`. Document the multi-head union in the method's doc comment. [FR-005, FR-006, FR-010]

**Checkpoint**: a descriptor can be produced on both platforms.

## Phase 5: User Story 2, it stays correct when the window changes (P1)

The detector, and the read-gating that makes the file half affordable.

### Tests

- [x] T019 [P] [US2] Write the failing change-detection tests in `tests/pixelbus_display.rs`: a first measurement produces one update; an identical repeat produces none; a changed surface, a changed origin, a changed display, and a changed DPI each produce one. [FR-007, FR-009, US2-1, US2-2, US2-3]
- [x] T020 [P] [US2] Write the failing read-gating tests, counting closure invocations across a scripted sequence: an unchanged measurement invokes the closure zero times, a changed one invokes it exactly once, and a run of N identical measurements invokes it zero times in total. This is the test that turns FR-016 into a guarantee. [FR-016, SC-003]
- [x] T021 [P] [US2] Write the failing loss-and-recovery tests: a measurement going absent clears the descriptor, counts as one change, and invokes the closure zero times; a later measurement re-resolves without any reset. [FR-010, US2-4]
- [x] T022 [P] [US2] Write the failing pre-launch tests: with no measurement and none ever seen, the closure is invoked exactly once and not again while the measurement stays absent. [FR-016, FR-022]
- [x] T023 [P] [US2] Write the failing reconciliation tests: no settings, settings with no pairs, a match against exactly one pair (carrying the raw mode value), a match against both (ambiguous), and a match against neither (disagreed). [FR-019, FR-020, US3-4, US3-5]
- [x] T024 [P] [US2] Write the failing authority test: for agreeing, disagreeing, and partially present stored settings, the measured descriptor is byte-identical in every case. A stored reading changes nothing. [FR-018, SC-005]
- [x] T025 [P] [US2] Write the failing writes-nothing test: run detection with a real temporary directory containing a settings file, and with an empty one, and assert both are unchanged in content and in entries afterward. [FR-017, SC-008]

### Implementation

- [x] T026 [US2] Add `Reconciliation`, `StoredPair`, and `reconcile(measured, stored)` to `src/pixelbus/display.rs`. Observational only; it returns no descriptor and makes no choice. [FR-019, FR-020]
- [x] T027 [US2] Add `DisplayDetector`, `DisplayUpdate`, `current()`, and `update(measured, closure)` to `src/pixelbus/display.rs`, implementing the read-gating rule from [data-model.md](data-model.md) exactly: the closure is not called on an unchanged measurement and not called when a measurement is lost. [FR-007, FR-009, FR-016, FR-018, FR-022]
- [x] T028 [US2] Wire the detector into the pixel bus worker in `src/main.rs`: resolve the settings path once at startup from the beacon preferences, construct the detector alongside the reader, and call `update` once per loop iteration with a closure that reads and parses the file. Log each returned update at debug on the existing `eso_weave::pixelbus` target, including the reconciliation outcome. No new thread and no new timer. [FR-007, FR-008, FR-009, FR-021]

**Checkpoint**: the feature is complete and current.

## Phase 6: Polish and boundaries

- [x] T029 [P] Confirm the boundary by inspection and by the suite: `tests/pixelbus.rs` is unmodified, `addon/` is untouched, the manifest version is unchanged, and no block count, sample point, capture dimension, or colour contract moved. [FR-023, FR-024, FR-025, SC-009]
- [x] T030 [P] Document the descriptor in the master specification's pixel-bus section as the out-of-band input the future grid contract will derive from, explicitly noting that this slice adds no block. [FR-027]
- [x] T031 [P] Add the `CHANGELOG.md` unreleased entry: an `Added` line for display detection, plus dated decisions for the unmapped window-mode value (and the diagnostic that gathers evidence for it) and for extending `SurfaceSampler` rather than adding a second seam. [FR-027]
- [x] T032 Run the full merge gate in the foreground: fmt, clippy at deny-warnings, and `cargo test --all --locked`. Green before commit, per constitution principle IV. [SC-010]

## Dependencies

```text
Phase 1 (T001)
  └─> Phase 2 (T002, T003)
        ├─> Phase 3, US3 (T004..T010)          # no OS dependency
        ├─> Phase 4, US1 (T011..T018)          # needs T002 primitives
        │     └─> T013 also needs T008 (StoredVideoSettings)
        └─> Phase 5, US2 (T019..T028)          # needs T015 and T026
              └─> T028 needs T010 (settings path) and T017/T018 (probes)
                    └─> Phase 6 (T029..T032)
```

User Story 3 is fully independent of the operating system and can be completed
and shipped on its own. User Story 1 depends on Phase 2 only. User Story 2
depends on both, because the detector reconciles what they produce.

## Parallel execution

Within each phase the `[P]` tasks touch different test functions or different
files and can proceed together. The largest parallel batch is T019 through T025,
seven independent test tasks in one file, all written before T026 and T027 exist.

The two platform probes, T017 and T018, are also genuinely parallel: different
files, different operating systems, no shared symbol beyond `MeasuredDisplay`
from T015.

## Implementation strategy

The MVP is Phase 3 alone: a tested, panic-free parser for the game's stored
video settings, plus the path that locates it. It delivers the pre-launch half of
issue #3 and cannot regress anything, because nothing else calls it yet.

The natural second increment is Phases 2 and 4, which produce a measured
descriptor with no currency guarantee. The third, Phase 5, makes it stay correct
and is where the feature becomes what the issue asked for.

Nothing here is safe to stop halfway inside Phase 5: a detector wired into the
worker without its read-gating tests would read a file on every sampling
iteration, which is exactly the cost FR-016 exists to prevent.
