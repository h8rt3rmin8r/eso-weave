# Implementation Plan: PixelBeacon Menu-State Input Gate

**Branch**: `032-menu-state-gate` | **Date**: 2026-07-27 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/032-menu-state-gate/spec.md`

## Summary

Add a sixth block (B5) to the PixelBeacon strip carrying whether a native game UI
surface is active and which one, decode it in the companion, and use it to stop the
application interfering while the operator is in a menu or typing.

The design turns on one property, and everything else follows from it: **both gates
default to inactive, and an inactive gate reproduces today's behavior exactly.**
That is what makes it safe to edit the constitution's sacrosanct decision path.
Every existing test constructs its subject with the gate off, so every existing
test still exercises today's behavior with no edit at all, and the new safety
property is proven separately by exhaustive comparison rather than by sampling.

The feature covers two synthesis paths, not one. The weave path is gated at the
interception decision; the fishing path synthesizes on its own schedule and is
gated at its own initiation points. Missing the second was the defect the
input-safety checklist caught in the specification.

## Technical Context

**Language/Version**: Rust 1.96.0 (pinned), edition 2021, plus Lua for the bundled addon

**Primary Dependencies**: unchanged; no new dependency

**Storage**: None. Gate state is live observed state, never persisted.

**Testing**: `cargo test --all --locked`. The safety property is proven by an exhaustive cross-product test over the interception decision's inputs.

**Target Platform**: Windows 10/11 x64 and Linux x64

**Project Type**: Desktop companion application, single Rust crate

**Performance Goals**: The strip is captured at the fast cadence whenever the application can intercept, rather than once a second. This is a deliberate increase, justified in Decision 4, and bounded so that a suspended application still samples slowly.

**Constraints**: The interception decision must stay synchronous and non-blocking. The strip widens from six to seven block widths. Text: UTF-8 without BOM, LF, no em-dashes or en-dashes anywhere including code comments.

**Scale/Scope**: One addon block, one decoder, two gate guards, one cadence change, one view row.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

| Principle | Assessment |
| --- | --- |
| I. Spec-Driven Development | PASS. Traces to build plan 010 slice 032 and master specification section 10.3; full spec-kit sequence run, with two checklists preceding this plan. |
| II. Safety-Critical Surfaces Are Sacrosanct | PASS, and this is the principle the feature is built around. The added guards can only produce a pass, never a suppression, which is proven exhaustively rather than asserted. Focus scoping stays unconditional. Both gates default to inactive so no existing safety test changes meaning, and none needs even a mechanical edit because the gates are set through new methods rather than new constructor parameters. Fishing still degrades to disabled on signal loss, unchanged. |
| III. Test-First With Explicit Seams | PASS. The decoder, the decision comparison, the cadence selection, and the view derivation are pure. The existing `FishingSink` and `SurfaceSampler` seams are reused, and no new platform dependency is introduced. |
| IV. CI Parity Before Every Commit | PASS. Rust sources change, so the full gate runs in the foreground before commit. |
| V. Bounded Scope: Outside The Game | PASS. The signal is published through the existing screen-signal contract. No process memory, no packet traffic. |

**Post-design re-check (after Phase 1)**: PASS, unchanged. The design adds no
thread, no synchronization primitive beyond one more atomic flag alongside the two
the input engine already carries, and no persisted state. The cadence change is the
one item with a cost, and it is bounded by Decision 4 rather than unconditional.

## Project Structure

### Documentation (this feature)

```text
specs/032-menu-state-gate/
├── plan.md                 # This file
├── research.md             # Phase 0 output
├── data-model.md           # Phase 1 output
├── quickstart.md           # Phase 1 output
├── contracts/
│   └── pixel-bus-b5.md     # The B5 wire contract
├── checklists/
│   ├── requirements.md     # Spec quality
│   └── input-safety.md     # The constitutional boundary
└── tasks.md                # Phase 2 output
```

### Source Code (repository root)

```text
addon/PixelBeacon/
├── PixelBeacon.lua      # B5 render, UI-mode plus chat-entry gate, surface codes,
│                        # NUM_BLOCKS 6, fast tick renamed and carrying the gate
└── PixelBeacon.txt      # Manifest version 6 to 7, description line

src/
├── pixelbus/
│   └── mod.rs           # MenuSignal, decode_menu, BlockSamples.menu, NUM_BLOCKS = 6,
│                        # marker registry entry, reader state and event,
│                        # poll_interval gains the intercepting argument
├── input/
│   └── mod.rs           # menu_gated AtomicBool, set_menu_gated, one guard in classify
├── fishing/
│   └── mod.rs           # gated flag, set_gated, guards at the interact initiation points
├── app/
│   ├── routing.rs       # route_reader_event gains the input engine, routes MenuGate
│   ├── mod.rs           # MenuView, menu_view, AppView field
│   ├── strings.rs       # MENU_TITLE, MENU_TOOLTIP
│   └── ui.rs            # The menu row beside combat
└── main.rs              # Clone the input engine into the reader thread; cadence call

tests/
├── pixelbus.rs          # Decoder, tri-state, registry, reader events, poll_interval
├── input_engine.rs      # The exhaustive gated-versus-ungated safety proof
├── fishing.rs           # Gated initiation, and that in-flight work still completes
├── beacon.rs            # Cross-side agreement for the new constants
└── app_view_model.rs    # menu_view, routing
```

**Structure Decision**: No new module. The two guards land in the modules that
already own each synthesis path.

## Decisions

### Decision 1: B5 encoding and the surface code table

**Chosen**: green marker `0xD2` (reserved by slice 031), red carries the surface
code as `code * 24`, blue carries the complement checksum `255 - red`. Codes:

| Code | Red | Surface |
| --- | --- | --- |
| 0 | 0 | None, gameplay |
| 1 | 24 | System menu |
| 2 | 48 | Map |
| 3 | 72 | Inventory |
| 4 | 96 | Mail |
| 5 | 120 | Character and skills |
| 6 | 144 | Guild store |
| 7 | 168 | Crown store |
| 8 | 192 | Journal |
| 9 | 216 | Chat entry |
| 10 | 240 | Other, unenumerated |

Adjacent codes are 24 apart, twelve times the default tolerance. The gate is
`code != 0`.

The load-bearing property is that **the boolean is decided before the label**. The
addon determines "a surface is active" from the game's UI-mode state (ORed with
chat text entry), and only then tries to name it, falling back to code 10. So an
incorrect or outdated scene name degrades to "other menu" and never to "no menu".
That is what lets the code table be a convenience rather than a correctness
dependency, and it is why the table does not need exhaustive verification to ship.

### Decision 2: where the gate is enforced

**Chosen**: two guards, one at each point where new synthesis is initiated. In the
interception decision, an early pass alongside the existing suspend check. In the
fishing controller, an early return at the points that initiate an interact
keypress.

Rejected: **a single choke point at the synthesis sink.** Superficially attractive
because it is one place and cannot be forgotten, but it is wrong here. A sequence
is a series of key transitions; blocking them individually mid-sequence can drop a
key-up and leave a key held down in the game, which is the exact hazard FR-011
exists to prevent. Gating initiation keeps sequences atomic.

Rejected: **disabling fishing outright while gated**, by analogy with the existing
degrade-to-disabled on signal loss. That precedent is about firing inputs blind
with no signal, which is a different situation; here the signal is good and the
session is fine, so ending it because the operator glanced at the map would be a
worse experience than pausing.

### Decision 3: how the gate reaches the subsystems

**Chosen**: both subsystems expose a setter, and `route_reader_event` gains the
input engine as a parameter so it can call both from the one place that already
routes every other decoded signal.

The input engine already holds `focused` and `suspended` as atomics set from
outside; `menu_gated` is a third of exactly the same shape, so it introduces no new
concurrency concept. The fishing controller already lives behind a mutex the reader
thread holds while routing.

The alternative, returning a gate change for `main.rs` to apply, was rejected
because it would make this the only decoded signal not routed like the others, for
no benefit. Adding the parameter costs a mechanical edit at the existing call sites,
which FR-018 explicitly permits and which changes no assertion.

Note the property this buys: because the gates are set through methods rather than
constructor arguments, and both default to inactive, **no existing test needs even
a mechanical edit to keep passing.**

### Decision 4: the sampling cadence

**Chosen**: `poll_interval` gains a second condition. The fast cadence applies when
a fishing session is active (as today) **or** when the application is in a position
to intercept. The idle cadence applies otherwise.

This is the part that needed care. Making the fast cadence unconditional would have
made `interval_idle_ms` dead configuration, which is the exact mirror image of the
bug slice 016 fixed, where `interval_fishing_ms` was dead and fishing sampled once
a second. The insight that avoids it: the gate only matters when there is something
to gate. If the operator has suspended the application, nothing is intercepted and
nothing is synthesized, so the gate is irrelevant and the slow cadence is correct.
Both settings keep a real meaning, the capture cost rises only while the
application is actually working, and a suspended application gets cheaper rather
than more expensive.

### Decision 5: the addon-side publish cadence

**Chosen**: the existing fast tick, which currently drives fishing detection,
becomes a general fast tick and carries the gate as well. It is renamed to reflect
that it is no longer fishing-specific.

The alternative, the one-second general tick, cannot meet SC-002: it would make the
addon's publish delay roughly ten times the companion's sampling delay, so the
end-to-end latency would be dominated by the half nobody was measuring. This is
what FR-019a was added to prevent. A scene-callback approach was considered and
rejected as unnecessary once a fast tick already exists, and because UI mode is a
polled flag rather than an event.

### Decision 6: how the one-way property is proven

**Chosen**: an exhaustive cross-product test over every input to the interception
decision, run twice, comparing the gated and ungated outcomes.

The input space is small and closed: injected versus real origin, focused versus
not, a bound-and-active key versus a bound-and-inactive key versus an unbound key
versus a suspend-exempt key, suspended versus not, and key down versus up. The test
enumerates all of it and asserts two things for every point:

1. Whenever the ungated decision is to pass, the gated decision is also to pass.
2. The gated decision is never a suppression where the ungated decision was a pass.

Together these say the gate can only relax, which is FR-015 stated as an
assertion rather than a hope. A sampled test would not do: the whole risk is an
unconsidered combination.

## Complexity Tracking

No constitution violations. No entries.
