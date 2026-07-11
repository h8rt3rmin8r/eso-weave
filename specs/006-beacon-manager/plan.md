# Implementation Plan: Beacon Manager

**Branch**: `006-beacon-manager` | **Date**: 2026-07-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/006-beacon-manager/spec.md`

## Summary

Add a `beacon` module that manages the on-disk lifecycle of the embedded
PixelBeacon addon. The addon's two files (`PixelBeacon.txt` and
`PixelBeacon.lua`, authored in feature 004) are embedded in the binary with
`include_str!`, and the embedded version is single-sourced by parsing the
embedded manifest's `## Version:` line. Pure logic carries all correctness and
safety: manifest parsing (managed-marker detection and version extraction),
status classification into the four states, install (writes confined to the
`PixelBeacon` subtree of an injected AddOns root), and uninstall (delete the
`PixelBeacon` folder only when the managed-marker line is verified present in the
on-disk manifest). The AddOns root is resolved behind a discovery seam: the
Windows Documents known-folder path and the Linux Steam `libraryfolders.vdf` plus
compatdata path are thin platform backends, while the VDF parsing and path
composition are pure and unit-tested on the host. A best-effort running-game
probe feeds a pure reminder rule that decides whether to surface the
`/reloadui`-required notice.

## Technical Context

**Language/Version**: Rust 1.96.0, edition 2021 (unchanged).

**Primary Dependencies**: Reuses `dirs` (its `document_dir` resolves the Windows
Documents known folder through the shell API, satisfying the no-literal-path
requirement) and `std::fs`. Adds no new crates: the Steam `libraryfolders.vdf` is
parsed by a small purpose-built extractor rather than a VDF crate, keeping the
single-crate, minimal-dependency posture. On Windows the running-game probe uses
the already-present `windows-sys` process-snapshot APIs; on Linux it reads
`/proc`.

**Storage**: User settings only. A new opaque `beacon` settings section (a manual
AddOns path override and the selected `live`/`pts` environment) is added
additively, mirroring the existing `timing` and `skills` sections, so no config
`schema_version` bump is required. No runtime or derived state is persisted.

**Testing**: `cargo test`. Manifest parsing, status classification, install and
its subtree confinement, the marker-gated uninstall, the VDF library extraction,
the environment path composition, and the reminder rule are all unit-tested
against a temporary AddOns root and injected inputs. The known-folder and
process-probe backends are thin and validated on real hardware.

**Target Platform**: Windows 10 and 11 x64, Linux x64 (Steam Proton, app id
306130).

**Project Type**: Single desktop-application crate (unchanged).

**Performance Goals**: A handful of filesystem operations per lifecycle call; not
performance sensitive.

**Constraints**: Safety-critical. Uninstall deletes only after verifying the
managed-marker line in the on-disk manifest; no write or delete ever escapes the
resolved `PixelBeacon` folder. Both are enforced in pure, fully tested logic.

**Scale/Scope**: Two embedded files, four status states, one AddOns root per
environment, three lifecycle operations (install, verify, uninstall).

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

- **I. Spec-Driven Development**: PASS. Derived from `spec.md` (master spec
  sections 9.4 and 9.5), bounded by `docs/plans/plan-001.md` slice 006.
- **II. Safety-Critical Surfaces**: PASS and central. The marker-gated uninstall
  and the write/delete confinement to the resolved AddOns subtree are exactly the
  two non-negotiable surfaces named in the constitution. Both are pure logic with
  required, non-weakened tests (delete a managed folder; refuse an unmanaged and a
  manifest-less folder; never touch anything outside `PixelBeacon`).
- **III. Test-First With Explicit Seams**: PASS. The injected AddOns root, the
  injected `RunningState`, and the pure parsers are the seams; the discovery and
  process backends sit behind them. Tests precede implementation.
- **IV. CI Parity Before Every Commit**: PASS on the host; the Linux backend is
  type-checked against the linux target as in prior slices, and the VDF parser and
  path logic are compiled and tested unconditionally so host `cargo test` exercises
  them.
- **V. Bounded Scope: Outside The Game**: PASS. Only reads and writes addon files
  in the AddOns directory and performs a read-only process-existence probe; no
  game memory, network, or gameplay access.
- **Platform and Text Hygiene Constraints**: PASS. Settings remain user-only and
  additive; all new text is UTF-8 without BOM, LF, no em/en dashes.

No violations. Complexity Tracking is empty.

## Project Structure

### Documentation (this feature)

```text
specs/006-beacon-manager/
├── plan.md, research.md, data-model.md, quickstart.md
├── contracts/
│   └── beacon.md    # Environment, BeaconPrefs, BeaconStatus, lifecycle ops, discovery seam, reminder rule
├── checklists/{requirements.md, lifecycle-safety.md}
├── spec.md
└── tasks.md
```

### Source Code (repository root)

```text
src/beacon/
├── mod.rs        # Environment, BeaconPrefs, BeaconStatus, LifecycleError/Outcome,
│                 # embedded files + embedded version, manifest parsing
│                 # (has_managed_marker, parse_manifest_version), status(),
│                 # install(), uninstall() over an injected addons_root,
│                 # RunningState + reload_reminder rule, DiscoveryError,
│                 # resolve_addons_dir() dispatch, environment path composition
├── steam.rs      # steam library extraction from libraryfolders.vdf text
│                 # (pure, compiled on all targets, unit-tested on host)
├── windows.rs    # #[cfg(windows)] Documents known-folder resolution; ESO process probe
└── linux.rs      # #[cfg(target_os = "linux")] compatdata AddOns resolution via steam.rs; ESO process probe
tests/
└── beacon.rs     # manifest parsing, four-state classification, install + subtree
                  # confinement, marker-gated uninstall, VDF extraction, env path, reminder rule
```

**Structure Decision**: The pure logic (manifest parsing, four-state
classification, install with subtree confinement, marker-gated uninstall, VDF
library extraction, environment path composition, and the reminder rule) carries
all correctness and safety and is fully tested against a temporary AddOns root and
injected inputs. The Documents known-folder resolution and the ESO process probe
are thin `#[cfg]`-gated adapters, mirroring the maximal-testable-core pattern of
the prior slices (`input`, `pixelbus`). The `steam.rs` VDF extractor is compiled
unconditionally so the host test run exercises the Linux discovery logic.

## Complexity Tracking

No constitution violations. No entries.
