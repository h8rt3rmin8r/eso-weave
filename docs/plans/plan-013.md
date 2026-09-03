# Build Plan 013: Runtime and Context Truth

Plan: 013
Status: complete
Master specification: `docs/ESO-Weave-Specification.md`
Constitution: `.specify/memory/constitution.md`

## Purpose

The v0.12.0 runtime epic identified one foundational defect behind several
surface failures: ESO Weave does not model whether ESO is installed or active,
and its menu value cannot distinguish valid gameplay from missing evidence.
Quickslot, auto-potion, and the dashboard all consume these facts, so repairing
them first removes duplicated guesses from every later slice.

## Ordering

Slice 041 combines issues #22 and #23 because they require the same observation
contract. Installation and runtime are established first, then focus,
PixelBeacon freshness, and surface evidence project into Game Context and the
shared dormant behavior. Quickslot and auto-potion remain outside this slice and
consume the result afterward.

## Slice 041: Game Runtime and Context Truth

Feature under `specs/041-game-runtime-context/`.

Scope:

- detect standalone, Steam, Epic, and Steam Proton installations from validated
  provider evidence
- distinguish Inactive, Launcher open, Active, and Unknown with Active taking
  precedence
- separate operating-system focus, PixelBeacon freshness, and valid in-game
  surface evidence
- replace Game Menu with Game Context and never present missing evidence as
  Gameplay
- make every game-derived metric dormant while ESO is not Active
- add runtime and focus safety gates without making basic weaving depend on the
  optional addon
- preserve transition-only logs, bounded polling, and recovery without restart

Done when issues #22 and #23 are closed, coordinator #21 is updated, the full
platform and view-model matrices pass, and local CI parity is green.
