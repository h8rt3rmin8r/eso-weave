# Research: Privacy-Minimized Encounter Capture

## Evidence Boundary

S075 implements the capture handoff defined by S069. It relies on the same
pinned primary source snapshots and does not claim that repository fixtures
prove live Combat Metrics parity.

| Source | Pinned revision | Use |
| --- | --- | --- |
| ESOUI API 101050 | `f76cf16c4e5be7b234d15dc7f676febffa64c5bb` | Callback signatures, time, performance, action slot, and unit APIs |
| LibCombat | `80817e6929c7626832f9b9114d3b12bad8d642c1` | Relevant event-family vocabulary and loss-aware capture precedent |
| Combat Metrics | `6ec1deea4ef8801800dfe88ec79b1f94d0d6303b` | Stored encounter families and later parity targets |

The ESOUI source is technical reference evidence only. Third-party code is not
copied. S075 distributes project-authored Lua and no game art or captured data.

## ESO API Findings

The pinned API documents:

- `EVENT_COMBAT_EVENT` with action result, amount, power or damage type, source
  and target unit IDs and types, ability ID, and overflow. Name fields exist but
  are intentionally ignored.
- `EVENT_EFFECT_CHANGED` with change, stack, timing, unit ID, ability ID, and
  effect classifications. Effect and unit names are intentionally ignored.
- `EVENT_POWER_UPDATE`, action-slot use, active weapon-pair changes, active
  quickslot changes, player combat state, player dead/alive, and boss changes.
- `GetGameTimeMilliseconds`, `GetFramerate`, `GetLatency`, `GetUnitPower`,
  `GetSlotBoundId`, and `GetCurrentQuickslot` for bounded samples.

Decision: direct events own damage, healing, effects, resources, casts, bars,
life state, quickslot selection, and encounter boundaries. A fixed update owns
changed boss-health samples and periodic performance samples. No callback string
is retained merely because the API provides it.

## Consent Model

Alternatives considered:

1. Always capture while the addon is enabled.
2. Persist a long-lived automatic-capture preference.
3. Explicitly arm one encounter.

Decision: explicitly arm one encounter. Option 1 creates unexpected local
history. Option 2 still risks forgotten background collection. Single-use arm
is simple, testable, and enough to unblock #133 and #131.

The user selects `live` or `pts` in the arm command. Arming during combat waits
until combat ends and the next clean combat boundary begins. This avoids a
partial start being mislabeled complete.

## Actor Identity

ESO callbacks provide unit IDs, unit tags, and names with different stability.
Cross-encounter identity is unnecessary for the first metric set.

Decision: map an in-memory key to the next positive integer for the duration of
one encounter. Prefer numeric combat unit IDs when present. For unit-tag-only
events use the tag only as an in-memory lookup key, never as exported data.
Unknown identity maps to actor 0 rather than a fabricated stable player.

The registry is bounded to 4,096 entries. Actor overflow maps later unknown
actors to 0, increments a warning counter, and preserves event totals without
leaking the source key.

## Event Classification

Combat results include damage, healing, interrupts, deaths, resurrection, and
many control results. S075 needs deterministic families without copying another
addon's classification logic.

Decision: define small project-owned constant sets for damage and healing action
results. Other `EVENT_COMBAT_EVENT` values are ignored except documented death
and resurrection results. Unknown result codes remain out of the raw baseline
until a versioned contract adds them.

Player death and alive callbacks supplement combat results for local life-state
boundaries. Duplicates are allowed as distinct observations because later
calculation owns semantic deduplication, but every record has a unique sequence.

## Bounds and Overflow

ESO serializes SavedVariables after callbacks, so exact file size is unavailable
inside the addon. Counting only Lua strings would understate table overhead.

Decision: enforce both a 100,000-event limit and a conservative 32 MiB estimate.
Every event estimate includes a fixed table overhead and recursively counts only
the bounded numeric, boolean, and stable-string payload forms. The last two event
slots and 2 KiB remain reserved for discontinuity and encounter end.

When regular capacity is exhausted:

1. source sequence continues for every omitted observation;
2. the first and last omitted sequence are retained in constant memory;
3. finalization appends a discontinuity whose sequence immediately follows the
   omitted range;
4. encounter end follows the discontinuity;
5. terminal status is partial and names `capture-overflow`.

This reports loss without unbounded memory or a false complete state.

## Clock Model

Raw game milliseconds can reset across lifecycle boundaries. Wall-clock time is
unsuitable for durations.

Decision: export elapsed monotonic milliseconds starting at zero. Each callback
adds only a nonnegative raw-clock delta. A backward clock records a one-sequence
declared clock-reset loss before continuing from a new raw baseline. Start and
finish Unix timestamps remain provenance strings only.

## Lifecycle Interruption

Decision: player deactivation, reload-related teardown, explicit stop, and
callback failure finalize a partial capture. All capture events and update
handlers are unregistered before the terminal message. An encounter ending by
normal combat state finalizes complete unless loss was already declared.

Saved armed intent may survive UI reload because it was explicit. An active
capture never resumes across reload; it is finalized partial first.

## Addon Boundary and Governance

The 2.1.0 constitution allows only PixelBeacon and ESO Weave Collector. Reusing
either would be architecturally incorrect:

- PixelBeacon is a low-volume screen signal with action-safety consumers.
- ESO Weave Collector is a manual, out-of-combat catalog enumerator.
- Encounter capture is high-volume combat observation with private user data.

Decision: amend the constitution to 2.2.0 for a third narrow bridge. The new
surface is read-only, user-armed, local-only, bounded, separately named, and
prohibited from automation. Any later installer must use its own managed marker
and containment suite.

## Test Strategy

Existing addon tests mostly inspect source text. That is useful for forbidden
surface checks but cannot establish consent or loss transitions.

Decision: add `mlua` as a test-only vendored Lua 5.1 runtime. The harness stubs
the documented ESO event manager, clock, unit, slot, and sampling calls, loads
the exact production addon source, dispatches callbacks, and reads the exact
SavedVariables table. This dependency does not enter release binaries.

Repository evidence covers complete capture, privacy, sequence/time, all event
families, overflow, clock reset, stop/deactivation, command authority, terminal
teardown, and source confinement. Issue #131 still owns live parity and exact
storage measurement.
