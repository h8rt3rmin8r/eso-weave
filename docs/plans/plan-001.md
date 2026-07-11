# Build Plan 001: Master Specification Decomposition

Plan: 001
Status: active
Master specification: `docs/ESO-Weave-Specification-v0.1.0.md`
Constitution: `.specify/memory/constitution.md`

## Purpose

This build plan decomposes the master specification into an ordered set of work
slices. Each slice is scoped to become exactly one spec-kit feature under
`specs/NNN-name/`, produced and implemented through the Build-Phase Autopilot
Protocol (`docs/build-autopilot.md`). The master specification remains the
source of technical scope; this plan supplies the sequence and the slice
boundaries.

A build plan is not a spec-kit feature plan. This document is the higher level
roadmap; the per-feature `specs/NNN-name/plan.md` files are generated later by
`/speckit.plan`, one per slice. See `docs/plans/README.md` for the distinction
and the plans index.

## Slice 0: Constitution (prerequisite)

Before the first feature slice, author the project constitution via
`/speckit.constitution`. The constitution is governance, not a `specs/` feature,
so it does not receive a slice number. It must land first because the
`/speckit.analyze` gate and every autopilot decision are evaluated against it;
running features against an unfilled template would make that gate hollow.

## Slices

Each row below is one feature slice. The spec-kit feature number is assigned by
`create-new-feature.ps1` at creation time; the numbers here are the intended
build order.

| Slice | Feature | Draws from spec | Depends on | Key deliverables and safety surfaces |
| --- | --- | --- | --- | --- |
| 001 | Foundations | 5, 11, 12, 14 | none | Cargo.toml (single-sourced version), rust-toolchain.toml, single-crate `src/` skeleton, Config Store (schema_version, `.invalid` fallback, settings only), Logging (tracing levels, always-on ring buffer, monthly file sink). Establishes CI parity so fmt, clippy, and test run. |
| 002 | Input Engine | 6 | 001 | `Input` trait, Windows backend (WH_KEYBOARD_LL plus SendInput, injected-input flag, timeBeginPeriod), Linux backend (evdev grab plus uinput), mock backend, threading contract (hook thread never blocks; worker thread), focus-scoping. Safety-critical: recursion breaking, focused-window-only suppression, no blocking on the hook thread. |
| 003 | Weave Engine | 7 (excluding 7.4) | 002 | Skill model, weave types and sequences, timing model (GCD, cooldown gating, d_weave, d_heavy, d_bash). Platform-agnostic, unit-tested against the mock backend. |
| 004 | PixelBeacon Addon | 9 | 001 | `addon/PixelBeacon/` Lua shim (PixelBeacon.txt manifest plus PixelBeacon.lua), three-block pixel bus (heartbeat, fishing state, latency), bite-detection contract, managed-marker line. In-game deliverable; parallelizable with 003. |
| 005 | Pixel Bus Reader | 9.3 | 002, 004 | GDI (Windows) and X11 or XWayland (Linux) sampling backends, decode of heartbeat, fishing state, and latency, checksum channel, SignalLost on heartbeat timeout. |
| 006 | Beacon Manager | 9.4, 9.5 | 001 | AddOns discovery (Windows known-folder API; Linux libraryfolders.vdf plus Proton app id 306130), install, verify, uninstall. Safety-critical: uninstall only when the managed-marker line is verified present; discovery never writes outside the resolved AddOns directory. |
| 007 | Fishing Controller | 8 | 002, 005 | Fishing state machine on the `BiteDetector` seam, v1 `PixelBusDetector`, interact-key synthesis, SignalLost degradation (disable fishing rather than blind-fire inputs). |
| 008 | Latency-Adaptive Delays | 7.4 | 003, 005 | Weave enhancement consuming latency events, `effective_delay` formula with clamp, off by default. |
| 009 | GUI | 10 | 001, 003, 006, 007 | egui or eframe main window, status indicators, skills table, settings (10.3), live log viewer (ring buffer, colorized, pause and filter). Wires all subsystems together. |
| 010 | Packaging and CI | 13 | 009 | MSI (cargo-wix), .deb (cargo-deb), AppImage, `assets/icon.ico`, evdev permission docs; verified against the existing `release.yml`. Pinned artifacts (`packaging/**`, `scripts/**`) changed under dated CHANGELOG decisions. |

## Slice notes

Only the slices with non-obvious scope or safety-critical surfaces are
elaborated here; the table is the source of truth for the rest.

- 002 Input Engine. The threading contract is a hard requirement: interception
  callbacks never sleep or perform blocking work, or Windows silently removes
  the low-level hook. The hook thread classifies and suppresses the key and
  hands an event to a dedicated worker thread that runs all timed sequences.
  Synthetic input is flagged so the engine never intercepts its own output.
  Interception is active only while the ESO window holds keyboard focus.

- 005 Pixel Bus Reader. Loss of the beacon heartbeat (block B0 absent beyond
  `heartbeat_timeout_ms`, default 2000) raises SignalLost. The reader validates
  the checksum channel and applies per-channel tolerance to guard against
  misreads before emitting typed events.

- 006 Beacon Manager. Uninstall removes the `PixelBeacon` folder only when the
  `## X-ESO-Weave-Managed: true` marker line is present in its manifest; an
  unmanaged folder is never deleted. Discovery resolves the AddOns directory
  through the platform APIs above and never assumes a literal path.

- 007 Fishing Controller. The `BiteDetector` trait is the seam that lets the
  controller be built and tested against a stub before the pixel-bus stack is
  complete. On SignalLost the controller disables fishing rather than sending
  interact inputs blindly.

## Dependency and parallelism

001 is the gate for everything. 002 unlocks the weave vertical (003, then 008)
and, together with 004 and 005, the fishing vertical (005, 006, 007). The weave
and fishing verticals are deliberately decoupled (the weave engine has no
in-game dependency; only fishing requires PixelBeacon), so they can proceed in
parallel once 002 exists. 009 integrates the subsystems and 010 ships them.

## Out of plan 001: research backlog

The master specification section 16 Open Items (R1 through R5) are research and
post-v1 items, not slices in this plan: weave-delay defaults research (R1),
audio-cue bite detector (R2), pure-Wayland sampling path (R3), PixelBeacon
APIVersion upkeep (R4), and interact-key discovery from keybind exports (R5).
They are tracked in the specification and folded into future build plans as they
mature.
