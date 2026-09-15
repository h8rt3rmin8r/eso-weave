# Implementation Plan: Native ESO Binding Evidence

**Branch**: `codex/s100-native-binding-evidence` | **Date**: 2026-09-15 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/100-native-binding-evidence/spec.md`

## Summary

Implement issue #206 as the read-only evidence foundation for parent #188. PixelBeacon will inspect all native binding slots for eleven gameplay actions, publish one independently validated RGB cell per action, and refresh after binding lifecycle events. The desktop will add a portable keyboard-and-mouse chord model, decode a coherent eleven-action set, preserve versions 1 through 5, and leave every controller and user setting unchanged for the later #207 and #208 slices.

## Technical Context

**Language/Version**: Rust 2021 on pinned stable 1.93.1; ESO Lua 5.1; Markdown; YAML wording correction

**Primary Dependencies**: Existing pixel-bus reader, embedded PixelBeacon addon, `mlua` test runtime, Rust standard library

**Storage**: No new storage, SavedVariables, or desktop persistence

**Testing**: Rust unit and integration tests, ESO-compatible Lua harness, static addon policy tests, existing documentation and trust gates

**Target Platform**: Windows 10/11 and Linux desktop companion; ESO PC addon API 101050 and 101054

**Project Type**: Single-crate cross-platform desktop application plus embedded ESO addon

**Performance Goals**: Inspect at most eleven actions times the game's bounded binding-slot count on events and the existing one-second backstop; keep one pixel capture per reader iteration

**Constraints**: Strictly read-only binding access; one fixed cell per action; no strings on the wire; fail closed; no controller, input, settings, persistence, UI, or screenshot changes

**Scale/Scope**: Eleven actions, four modifier flags, ordinary keyboard and mouse registry, eleven appended payload blocks, one protocol generation

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **Principle I, Spec-Driven Development**: PASS. Parent #188 is decomposed into #206 through #208. This complete S100 package closes only #206 after clarify, plan, tasks, analysis, and implementation.
- **Principle II, Safety-Critical Surfaces**: PASS. Binding evidence is fail-closed and read-only. S100 does not synthesize from it or alter the F1 through F3 controls.
- **Principle III, Test-First**: PASS. Model, transport, compatibility, publisher, and prohibited-surface tests are written and observed failing before implementation.
- **Principle IV, CI Parity**: PASS. Full formatting, strict linting, all-target locked tests, release build, docs, trust, spelling, link, and encoding gates are required.
- **Principle V, Bounded Scope**: PASS. Evidence publication and decoding are separated from controller and settings migrations in #207 and #208.
- **Addon constraints**: PASS. PixelBeacon remains a minimal local screen signal with no SavedVariables, network access, command transport, or binding mutation.
- **Pinned artifacts and text hygiene**: PASS. The workflow notice correction receives a dated changelog decision. All text remains UTF-8 without BOM, LF, and free of forbidden dash characters.

## Design

### Portable model

Add `src/input/native.rs` with closed enums for actions, ordinary keyboard keys, mouse controls, modifier bits, chords, states, and a fixed binding set. Stable numeric codes are defined once by enum discriminants and verified against the addon mapping in integration tests.

This deliberately does not expand the existing manual `Key` or synthesis `MouseButton` types. Combining evidence with execution belongs to #207, where platform ownership and cancellation can be reviewed together.

### Addon discovery

Define the eleven action names and a portable lookup keyed by ESO `KEY_*` constants. A pure discovery function scans all binding slots, canonicalizes modifiers, deduplicates raw chords, applies conflict precedence, and returns one bounded fact. Event callbacks and the existing periodic update refresh and render the complete set idempotently.

### Wire contract

Append B29 through B39, advance the layout to protocol 6, freeze protocol 5 at 29 blocks, and bump the addon manifest to 22. Each cell expands the two control-code nibbles across red and green, then packs modifiers and an action-specific check nibble into blue. The transport remains fixed at 40 payload cells.

### Desktop decoding

Extend `BlockSamples` with an eleven-element binding sample array so call sites using `Default` remain source-compatible. Add pure per-cell and set decoders. The reader stores one coherent `NativeBindingSet`, exposes it, emits one change event, and resets it on signal loss. Older layouts do not sample the new cells.

### Read-only guard

Add a reusable policy validator in the beacon integration tests. It scans the embedded manifest and Lua source, rejects binding mutation and reset APIs, refuses binding manifests and custom action declarations, and exercises each rejection with injected fixtures.

### Documentation and workflow wording

Update the addon protocol, troubleshooting, Plan 043 chronology, its index, and changelog. Correct the S099 fallback notice so protected-main executions no longer claim the policy is absent from main.

## Project Structure

### Specification package

```text
specs/100-native-binding-evidence/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── binding-evidence.md
├── checklists/
│   └── requirements.md
├── analysis.md
└── tasks.md
```

### Repository integration

```text
src/input/native.rs
src/input/mod.rs
src/pixelbus/mod.rs
addon/PixelBeacon/PixelBeacon.lua
addon/PixelBeacon/PixelBeacon.txt
tests/native_bindings.rs
tests/pixelbus.rs
tests/beacon.rs
.github/workflows/ci.yml
docs/src/reference/addon-protocol.md
docs/src/troubleshooting.md
docs/project/build-plans/plan-043.md
docs/project/build-plans/README.md
CHANGELOG.md
.specify/feature.json
specs/100-native-binding-evidence/
```

**Structure Decision**: Keep the portable native model under `input` because it describes future physical input execution, while all wire encoding and sampling remain in `pixelbus`. The addon mapping mirrors the portable registry and is checked across the boundary.

## Verification Strategy

1. Add failing native model, per-cell transport, set decoding, compatibility, publisher, and no-mutation tests.
2. Implement the portable model and make its focused tests green.
3. Implement protocol 6 decoding and reader snapshot integration, preserving all frozen layouts.
4. Implement PixelBeacon discovery, rendering, lifecycle refresh, and manifest bump under the static guard.
5. Update canonical documentation, chronology, changelog, and the S099 notice.
6. Run focused tests, the spec-kit analysis gate, full Rust gates, release build, docs, trust, spelling, links, encoding, and diff review.
7. Publish the PR, address every CI and review result, trigger exactly one second `@Codex review` round, and request the operator's final merge ritual only after all checks and threads are satisfied.

## Complexity Tracking

No constitution violations or complexity exceptions are required.
