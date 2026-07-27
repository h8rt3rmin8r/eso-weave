# Implementation Plan: Out-Of-Band Display Detection

**Branch**: `034-display-detection` | **Date**: 2026-07-27 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/034-display-detection/spec.md`

## Summary

Add a display descriptor: how big the game's render surface is in physical
pixels, where it sits, which physical display it is on, and how that display is
scaled. Resolve it from operating system queries on the sampling cycle that
already runs, keep it current by change detection, cross-check it against the
game's stored video settings when it changes, and log the changes. Nothing
consumes it. The grid wrap this exists to unblock is a separate feature.

This is the only slice in build plan 010 that touches no addon code, changes no
block, and advances no manifest version. It is also the first one whose subject
is the machine rather than the character, which is why its risk is different:
there is no colour contract to get wrong, and instead the failure mode is a
descriptor that states something it does not actually know.

## Technical Context

**Language/Version**: Rust 1.96.0 (pinned), edition 2021. No Lua.

**Primary Dependencies**: unchanged; no new dependency and no new Cargo feature. The Windows calls are covered by the already-enabled `Win32_Graphics_Gdi` and `Win32_UI_HiDpi`; the Linux path uses only the core `xproto` the sampler already uses (see [research.md](research.md) R1, R2).

**Storage**: None. The descriptor is live observed state and is never persisted; the settings file is read only, never written.

**Testing**: `cargo test --all --locked`. The parser, the descriptor construction, the change detection, the read-gating, and the reconciliation are pure and unit-tested; the operating system calls sit behind the existing `SurfaceSampler` seam.

**Target Platform**: Windows 10/11 x64 and Linux x64. Capability differs by platform by design: Windows reports per-monitor geometry and an effective DPI, X11 reports the surface and the X screen with an unknown scale.

**Project Type**: Desktop companion application, single Rust crate

**Performance Goals**: No new thread and no new timer. Two or three additional operating system calls per sampling iteration on Windows, folded into a call sequence that already runs. Zero settings-file reads while the window is stationary.

**Constraints**: The descriptor must never be produced with a zero surface, must never fabricate a value it could not read, and must never write to disk. Text: UTF-8 without BOM, LF, no em-dashes or en-dashes anywhere including code comments.

**Scale/Scope**: One new module, one defaulted trait method, two platform implementations, one helper in the beacon path resolution, and about twenty lines of wiring in the worker loop.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

| Principle | Assessment |
| --- | --- |
| I. Spec-Driven Development | PASS. Traces to build plan 010 slice 034 and issue #3; full sequence run, two checklists precede this plan, one of which forced three requirements to be rewritten. |
| II. Safety-Critical Surfaces Are Sacrosanct | PASS, and one surface is touched adjacently and worth naming. This feature reads a file inside the game data directory, two levels from the only directory the application is permitted to write to. It never writes, creates, or removes anything, and that is a tested requirement (FR-017, SC-008) rather than an incidental property. No input path, no interception decision, no beacon lifecycle change. |
| III. Test-First With Explicit Seams | PASS. Every decision in the feature is in a pure type; the operating system stays behind the existing `SurfaceSampler` seam, extended rather than duplicated. |
| IV. CI Parity Before Every Commit | PASS. Noted as a risk rather than a violation: the repository has no test workflow, so the Linux path is only compiled at release time. This is why the Linux implementation deliberately uses only calls already demonstrated in the file it edits (research R2). |
| V. Bounded Scope: Outside The Game | PASS, and this feature is the strongest case of it in the plan: it reads no game memory, no packets, and not even the pixel bus. It reads two operating system queries and one text file the game itself wrote. |

**Post-design re-check (after Phase 1)**: PASS, unchanged. No entry in Complexity Tracking.

## Project Structure

### Documentation (this feature)

```text
specs/034-display-detection/
├── plan.md, research.md, data-model.md, quickstart.md
├── contracts/display-descriptor.md
├── checklists/{requirements.md, detection.md}
└── tasks.md
```

### Source Code (repository root)

```text
src/
├── pixelbus/
│   ├── display.rs       # NEW. Descriptor, stored settings, parser, detector,
│   │                    # reconciliation. Pure; no input or output.
│   ├── mod.rs           # `mod display` + re-exports, `SurfaceSampler::display()`
│   ├── windows.rs       # MonitorFromWindow, GetMonitorInfoW, GetDpiForMonitor
│   └── linux.rs         # get_geometry, translate_coordinates, screen setup
├── beacon/mod.rs        # `user_settings_path(addons_root)`, read only
└── main.rs              # Detector in the pixel bus worker; debug logging

tests/
├── pixelbus_display.rs  # NEW. Parser, descriptor, detector, reconciliation,
│                        # read-gating, and the writes-nothing proof
└── pixelbus.rs          # Unchanged, and required to stay unchanged
```

Nothing under `addon/` appears in that list, and that is a deliverable rather
than an omission: SC-009 requires the addon tree to be byte-for-byte unchanged.

## Decisions

### Decision 1: extend the existing seam, do not add a parallel one

**Chosen**: `SurfaceSampler` gains a defaulted `display()` method returning
`Option<MeasuredDisplay>`.

The alternative was a separate `DisplayProbe` trait. It would have meant a
second resolution function, a second boxed trait object in the worker, and a
second mock, all to express the boundary `SurfaceSampler` already is: this is
where the operating system starts. Both platform backends already hold exactly
the handle or title the probe needs, and the Windows one already performs most
of the measurement on every capture. The default returns no measurement, so
adding the method breaks no existing implementation and the mock opts in.

### Decision 2: the descriptor carries DPI, not a scale float

**Chosen**: the raw effective DPI as an optional integer; the scale is a
computed accessor (`dpi / 96`).

This keeps the whole descriptor integral, so it derives `Eq` and change
detection is exact rather than a float comparison, which matters because change
detection is what gates both the log line and the settings-file read. A stored
float would have made "did the descriptor change" a question with a tolerance,
which is a thing this codebase has enough of already.

### Decision 3: the settings read is a closure the detector calls, not I/O the detector does

**Chosen**: `DisplayDetector::update(measured, || read_settings())`, where the
closure is invoked only when the detector decides a read is warranted.

This is what makes FR-016 testable instead of aspirational. A test drives a
scripted sequence of measurements and counts closure invocations, proving that a
stationary window reads nothing. It also keeps the detector free of file paths,
error types, and platform behaviour, so every branch in it is reachable from a
unit test.

### Decision 4: the settings path is the AddOns directory's parent

**Chosen**: `beacon::user_settings_path(addons_root)`, returning
`addons_root.parent()?.join("UserSettings.txt")`.

Not a second derivation of the documents directory, and not a second statement
of the `Elder Scrolls Online/<environment>` segment. It inherits the manual path
override, the Linux Proton prefix handling, and every future fix to either, for
free. It also never creates anything, which is what the constitution's file
safety posture demands of anything operating in that tree.

**Limitation, stated rather than hidden**: the path is resolved once when the
worker starts, so changing the AddOns override mid-session is picked up on the
next launch. Acceptable because the stored settings are a cross-check and a
pre-launch fallback, never the authority, and because the measured path (which
is the authority) has no such staleness.

### Decision 5: no shared state, no channel, no view model row

**Chosen**: the detector owns the current descriptor and exposes it; nothing is
plumbed to another thread or into the interface.

This breaks the pattern of the three preceding slices, each of which routed its
new signal into the view model, and the reason is in the spec's clarifications:
those were signals about the character that an operator can look at and confirm,
this is geometry for a calculation that does not exist yet. Wiring a value
across a thread boundary to a consumer that has not been written would be
building the plumbing before knowing the shape of the thing it plumbs, and a
readout of numbers nobody can act on is clutter. The descriptor is public API;
the wrap feature will consume it, and it will bring its own opinion about where
the value needs to be.

### Decision 6: the mode value is recorded, never mapped

**Chosen**: report `FULLSCREEN` raw; produce a configured descriptor only when
both stored resolution pairs are identical; record the observed correspondence
between the matched pair and the raw value as a diagnostic.

The full argument is in [research.md](research.md) R4. In short: the mapping is
needed for exactly one decision, that decision is answered authoritatively by
the operating system whenever it matters, and guessing would produce a confident
wrong answer on exactly the installs where the two stored pairs differ. The
diagnostic turns issue #3's open verification item into something ordinary use
accumulates evidence for, rather than a manual procedure someone has to remember
to perform.

### Decision 7: X11 reports what X core knows, and says so

**Chosen**: core `xproto` only. Surface size and origin from the window, display
size from the X screen, scale unknown.

RandR would give per-monitor rectangles and is the better long-term answer, but
this repository compiles Linux only in the release pipeline, so an unverifiable
dependency change is a poor trade for a value the wrap layout does not need. The
multi-head consequence (the reported display size is the union of all heads) is
recorded in the data model and the module documentation rather than left for
someone to discover.

## Complexity Tracking

No constitution violations. No entries.
