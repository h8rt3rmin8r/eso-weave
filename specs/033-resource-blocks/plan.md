# Implementation Plan: PixelBeacon Resource Blocks

**Branch**: `033-resource-blocks` | **Date**: 2026-07-27 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/033-resource-blocks/spec.md`

## Summary

Add three blocks (B6 health, B7 stamina, B8 magicka) carrying each resource as a
whole percentage of its current maximum, decode them, and show them in the
application. Nothing consumes the values.

Each block follows the strip's established shape (green marker, payload, blue
complement checksum) with the payload being the percentage itself rather than an
index into a colour table. That one choice removes the feature's largest stated
deliverable, replaces an unbounded failure mode with a bounded one, and makes the
correctness property provable by enumeration instead of by inspection of a hundred
hand-picked colours.

## Technical Context

**Language/Version**: Rust 1.96.0 (pinned), edition 2021, plus Lua for the addon

**Primary Dependencies**: unchanged; no new dependency

**Storage**: None. Resource levels are live observed state and are never persisted.

**Testing**: `cargo test --all --locked`. The bounded-error and monotonicity properties are proven by enumerating the full publishable range against every in-tolerance perturbation.

**Target Platform**: Windows 10/11 x64 and Linux x64

**Project Type**: Desktop companion application, single Rust crate

**Performance Goals**: Three more point samples per strip read and a wider captured region (nine blocks rather than six). No cadence change; slice 032 already made the fast cadence conditional on the application being able to intercept.

**Constraints**: Resources change orders of magnitude more often than any previous signal, so the logging level differs deliberately. Text: UTF-8 without BOM, LF, no em-dashes or en-dashes anywhere including code comments.

**Scale/Scope**: Three addon blocks, one decoder reused three times, three view rows.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

| Principle | Assessment |
| --- | --- |
| I. Spec-Driven Development | PASS. Traces to build plan 010 slice 033 and master specification section 10.3; full sequence run, two checklists precede this plan. |
| II. Safety-Critical Surfaces Are Sacrosanct | PASS. No input path, no interception decision, no beacon lifecycle change. The values are stored where the interface can read them and read by nothing else, enforced the same way slice 031 enforced it for combat state. |
| III. Test-First With Explicit Seams | PASS. One pure decoder and one pure view derivation, both exhaustively testable; the existing sampler seam is unchanged. |
| IV. CI Parity Before Every Commit | PASS. |
| V. Bounded Scope: Outside The Game | PASS. Published through the existing screen-signal contract. |

**Post-design re-check (after Phase 1)**: PASS, unchanged.

## Project Structure

### Documentation (this feature)

```text
specs/033-resource-blocks/
├── plan.md, research.md, data-model.md, quickstart.md
├── contracts/pixel-bus-resources.md
├── checklists/{requirements.md, encoding.md}
└── tasks.md
```

### Source Code (repository root)

```text
addon/PixelBeacon/
├── PixelBeacon.lua      # Three resource blocks, markers, NUM_BLOCKS 9, power events
└── PixelBeacon.txt      # Manifest version 7 to 8

src/
├── pixelbus/mod.rs      # ResourceLevel, decode_resource, three markers and sample
│                        # points, three BlockSamples fields, event, registry entries
├── weave/mod.rs         # Resource storage (stored, never read for decisions)
└── app/                 # ResourceView, routing, strings, three UI rows

tests/
├── pixelbus.rs          # Exhaustive bounded-error and monotonicity proofs, rejection
├── beacon.rs            # Cross-side agreement for the three markers
├── weave_engine.rs      # The no-consumer boundary
└── app_view_model.rs    # View derivation and routing
```

## Decisions

### Decision 1: numeric payload, not a colour lookup table

**Chosen**: the payload channel carries the percentage directly (0 to 100), with a
green marker and a blue complement checksum, exactly like the latency block.

This reverses issue #2, which specifies a hundred-entry table and names building it
as the gating deliverable. The full argument is in [research.md](research.md); the
short form is that the two encodings have different failure modes and only one of
them is acceptable for an ordered quantity. A table maps a one-step channel error
onto whichever entry is nearest in colour space, which bears no relation to
nearness in percentage; a numeric channel maps it onto one percent. The issue's
stated reason for preferring the table ("a raw numeric channel is more fragile at
1-step resolution") is also contradicted by the latency block, which has encoded a
number in a channel since slice 001 and decodes correctly in the field.

Consequences worth stating plainly: the feature's largest deliverable disappears,
the 5 percent fallback the issue allows becomes unnecessary, and the correctness
property becomes provable by enumeration rather than by inspecting a hundred
colours by eye.

### Decision 2: three blocks, not one

**Chosen**: one block per resource.

A block has three channels; this strip spends one on a marker and one on a
checksum, leaving one payload. Packing three percentages into three channels would
buy back two squares at the cost of both validation mechanisms, so an arbitrary
colour behind a missing block would decode as three plausible resource values. That
is precisely the false reading every block since slice 031 has been built to
prevent, and resources are the worst signal to get wrong, because a future consumer
would act on "health is 8 percent".

### Decision 3: the three markers

**Chosen**: `0x16` health, `0x6D` stamina, `0xBB` magicka.

The strip's greens were, before this feature: `0x00`, `0x2D`, `0x5A`, `0x80`,
`0xA5`, `0xD2`, `0xFF`. The three new values sit in the widest remaining gaps:

| Marker | Nearest existing green | Distance | Margin over default tolerance |
| --- | --- | --- | --- |
| `0x16` (22) | `0x00` (0) | 22 | 11x |
| `0x6D` (109) | `0x80` (128) | 19 | 9.5x |
| `0xBB` (187) | `0xA5` (165) | 22 | 11x |

The nibble-swap pairing that produced `0xA5`/`0x5A` and `0x2D`/`0xD2` is abandoned
here, and deliberately: the remaining swap pairs land badly (`0xD6` is four away
from the menu marker `0xD2`), and with ten markers in a 256-wide channel the
spacing is tight enough that margin has to govern rather than aesthetics. The
registry test enforces the rule regardless of how the values were chosen.

### Decision 4: what "unavailable" looks like on the wire

**Chosen**: payload `0xFF`, which is outside the publishable range.

No new decode rule is needed, because any payload outside 0 to 100 already decodes
to unavailable. The value is fixed so the two sides agree rather than each picking
one, and it is the far end of the channel so it can never be confused with a real
percentage even under a large drift. This was the second finding of the encoding
checklist; the requirement said the addon must publish unavailable but nothing said
what that meant on the wire.

### Decision 5: trace, not debug, for resource changes

**Chosen**: resource changes are logged at trace level.

This deliberately breaks the pattern of the two preceding slices, which log combat
and menu transitions at debug. Those change a few times a minute. Three resources
at 1 percent granularity under a 100 ms cadence can change many times a second, and
at debug they would push every other line out of the operator's live log, which is
the tool used to diagnose every field defect this project has had. Events are still
emitted on change; only the log line moves.

### Decision 6: where the values are stored

**Chosen**: on the weave engine, beside combat state, read by nothing.

Same reasoning as slice 031's Decision 5, and the same enforcement: a test asserts
the engine behaves identically across resource values, so a later slice wiring
resources into timing has to break that test deliberately.

## Complexity Tracking

No constitution violations. No entries.
