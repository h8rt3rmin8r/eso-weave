# Research: PixelBeacon In-Combat State Block

**Feature**: 031-combat-state-block | **Date**: 2026-07-27

Phase 0 output. Every unknown in the plan's Technical Context is resolved here.
Nothing is left marked NEEDS CLARIFICATION.

## R1: The game-side combat signal

**Decision**: Drive the block from `EVENT_PLAYER_COMBAT_STATE` and re-baseline
from `IsUnitInCombat("player")` on `EVENT_PLAYER_ACTIVATED`.

**Rationale**: Both names are verified present in the live `esoui/esoui` API
source. The event carries the new state and fires on every transition, so the
block moves the moment combat starts or ends. The direct query is needed because
an event does not fire for a state that is already true when the world finishes
loading, which is the same reason the weapon-bar block re-baselines on
`EVENT_PLAYER_ACTIVATED` today.

This is the reason build plan 010 sequences this slice first. Every sibling block
issue has either a settled API (this one) or an open verification item; issue #11
in particular has no confirmed sprint observable, so it is sequenced last.

**Alternatives considered**:

- **Poll `IsUnitInCombat` on the existing one-second tick, no event.** Simpler
  wiring, but up to a second of lag on both transitions, and it would redraw the
  block on a timer instead of on a change. The event is authoritative and free.
- **Poll as a backstop in addition to the event.** The weapon block does this, and
  it is defensible, but the weapon block needs it because equipment changes have
  no single event that covers every case. Combat state has exactly one event that
  covers every transition, so a backstop poll would add a redraw path for no
  coverage. Rejected as unnecessary; the `EVENT_PLAYER_ACTIVATED` re-baseline
  covers the one gap the event genuinely has.

## R2: Why the latency block's encoding pattern and not the weapon block's

**Decision**: Encode B4 as marker plus payload plus complement checksum, the
latency block's pattern.

**Rationale**: The two existing data blocks encode differently, and only one of
them is robust under the reader's own tolerance.

- The latency block (B2) validates a green marker `0xA5` within tolerance and a
  checksum `r + b == 255` within tolerance, then reads the payload. A color that
  is not a latency block essentially cannot pass both.
- The weapon block (B3) validates a green marker `0x5A` within tolerance, then
  reads the active bar with `ActiveBar::from_code(sample.b)`, which matches `0`,
  `1`, and `2` **exactly**. Those codes are one apart while the default tolerance
  is 2, so the encoding has no margin at all in that channel; it survives only
  because the capture path currently returns exact values.

FR-006 requires the two combat encodings to be separated by more than the
tolerance, which rules out the weapon block's approach outright. Adopting the
latency pattern satisfies the requirement and adds meaningful protection for User
Story 2 at no cost.

**Alternatives considered**:

- **Marker plus two widely spaced red values, no checksum.** Satisfies FR-006 and
  is simpler. Rejected because the checksum is one comparison and materially
  reduces the chance that unrelated screen content behind an absent block decodes
  as a state, which is the failure User Story 2 exists to prevent.
- **A distinct whole color per state with no marker.** Fewer moving parts, but
  nothing then identifies the block as a combat block, so a neighboring block
  sampled at the wrong offset could decode as combat. The marker is what makes a
  geometry error detectable rather than silent.

## R3: Choosing the marker value

**Decision**: `0x2D`.

**Rationale**: Greens already appearing at a block center are `0x00` (status
magenta), `0x80` and `0xFF` (the two fishing colors), `0xA5` (latency), and
`0x5A` (weapon). Candidate markers were scored by their minimum distance to that
set:

| Candidate | Min distance to existing greens |
| --- | --- |
| `0x2D` (45) | 45 |
| `0xD2` (210) | 45 |
| `0x33` (51) | 39 |
| `0xC3` (195) | 30 |

`0x2D` and `0xD2` tie at 45, more than twenty times the default tolerance of 2.
`0x2D` is chosen and `0xD2`, its nibble swap, is recorded in the source as the
natural marker for the next block. That continues the `0xA5` and `0x5A` nibble
pairing the strip already uses and leaves the next slice a pick that is known
good rather than one it has to rediscover.

State codes are `0xE0` in combat and `0x20` out of combat: 192 apart, and each
well clear of the marker channel it sits beside.

**Alternatives considered**: assigning the marker arbitrarily and checking it by
eye. Rejected in favor of the enforced registry in R4, since three more slices
will each need to make this same choice.

## R4: Making the color contract enforceable

**Decision**: A documented constant in `src/pixelbus/mod.rs` listing every green
that appears at a block center, plus a test asserting pairwise separation greater
than the default tolerance.

**Rationale**: This is checklist item CHK006, left open by `/speckit-checklist`
on the grounds that a registry with one consumer is speculative. It is no longer
speculative at plan time, because Decision 1 has to justify a value against
exactly this list, and three following slices will each have to do the same. A
constant plus a test turns "pick a distinct marker" from a convention into a
failing build when someone does not.

**Alternatives considered**:

- **Prose in the master specification.** Documents the values but enforces
  nothing, and the specification is already stale at v0.2.0.
- **A comment listing the values.** Same weakness, and it drifts the first time
  someone adds a block without reading it.

## R5: Widening `observe` without churn

**Decision**: `BlockSamples`, a struct with one `Option<Rgb>` field per block and
a derived `Default`.

**Rationale**: FR-014 requires that adding a block not force every caller and
test to be rewritten. Struct-update syntax is what delivers that: a construction
written `BlockSamples { status: Some(c), ..Default::default() }` keeps compiling
when a field is added. The alternatives do not have this property.

| Option | Arity safety | Position safety | Cost per future slice |
| --- | --- | --- | --- |
| Positional arguments (today) | Yes | No (all same type) | Every call site and test |
| `&[Option<Rgb>]` | No | No | Every call site and test |
| `[Option<Rgb>; NUM_BLOCKS]` | Yes | No | Every literal gains an element |
| `BlockSamples` struct | Yes | Yes (named fields) | None |

The struct is also the only option that fixes a latent hazard in the current
signature: `observe(b0, b1, b2, b3, now)` takes four arguments of the same type,
so transposing two of them compiles and silently decodes the wrong blocks. Named
fields make that a compile error.

## R6: Where the decoded value lives

**Decision**: On the weave engine, beside latency and weapon-bar state.

**Rationale**: The reader runs on a background thread and the interface reads on
the main thread (`src/main.rs` takes the weave and fishing mutexes around
`route_reader_event`; `AppModel::view` takes the weave mutex to read
`active_bar` and `weapon_classes`). The engine is therefore already the shared
home for beacon-derived observables and already correctly synchronized. Storing
combat there costs one line in routing and one in the view builder.

The tension with FR-016 is real and is resolved by test rather than by comment: a
test asserts the engine produces identical behavior with combat state set to each
of the three values. That makes the boundary a thing the suite defends.

**Alternatives considered**:

- **A dedicated `Arc<Mutex<CombatSignal>>` or atomic.** Semantically cleaner,
  since nothing pretends the engine consumes the value, but it introduces a second
  synchronization primitive and a new parameter through `route_reader_event`,
  which has call sites in `src/main.rs` and five in `tests/app_view_model.rs`.
  Rejected as plumbing for a value nothing reads. Slice 032's menu gate will have
  a genuine consumer on the input side and can justify its own home then.
- **Keep it inside the reader and expose a getter.** The reader is owned by the
  sampling thread and is not shared, so the interface cannot reach it without
  introducing the same synchronization the first alternative needs.

## R7: Presentation

**Decision**: A `CombatView { detected, state, role }` mirroring `WeaponBarView`,
rendered as a grid row directly after the weapon-bar row, showing "Not detected"
in the muted role when unavailable.

**Rationale**: This is settled by the clarification session and by the existing
code: `weapon_bar_view` already returns `detected: bool` plus display names plus
a `StatusRole`, and `ui.rs` already renders `"Not detected"` through
`status_color` when `detected` is false. Following it exactly means the operator
reads two adjacent fields with one convention, and the new row inherits the
grid alignment the weapon-bar row already solved.

## Open items carried forward

None blocking. One note for the following slices: the reader's `tolerance`
setting is loaded from config with no range validation (`load_reader_config`
passes it through unchanged, and it is a `u8`, so `255` is accepted). That
predates this feature and degrades every block equally, so FR-006 measures
against the default tolerance instead. Worth filing separately if it ever bites;
it is out of scope here and is recorded in the spec rather than silently ignored.
