# Research: Roll-Dodge Safety

No NEEDS CLARIFICATION markers remain. Decisions follow the autopilot policy.

## Current ESO event contract

Decision: observe `EVENT_COMBAT_EVENT` for ability ID 28549, filtered to
`COMBAT_UNIT_TYPE_PLAYER`, and use `ACTION_RESULT_EFFECT_GAINED` and
`ACTION_RESULT_EFFECT_FADED` as the entry and completion transitions.

Rationale: API 101050 documents `EVENT_COMBAT_EVENT` as carrying an ActionResult,
target combat unit type, and ability ID, and documents the native target and
ability filters. The current LibSprint implementation uses exactly this filtered
ability and result pair. Its May 2026 field report also records the critical edge
case: pressing roll dodge while sprinting can emit gained but no completion.
Am I Blocking+ independently filters effect gained for the same player ability.

Sources:

- <https://raw.githubusercontent.com/esoui/esoui/live/ESOUIDocumentation.txt>
- <https://www.esoui.com/downloads/info3827-LibSprint.html>
- <https://www.esoui.com/downloads/info3929-AmIBlockingPlus.html>

Rejected alternatives:

- Coordinates over time: unnecessary for a directly evented state, inaccurate
  against walls, and would conflate sprint, knockback, mount, and teleport motion.
- Stamina deltas: many legitimate effects share that signal and exhaustive cause
  exclusion belongs to the deferred effect database.
- Physical key observation: misses gamepad and cannot prove that ESO accepted the roll.

## Event and lifecycle matrix

Decision: normalize the current evidence as follows.

| Condition | ESO/addon evidence | Published state |
| --- | --- | --- |
| Successful roll entry | ability 28549 effect gained | Active immediately |
| Normal roll completion | ability 28549 effect faded | Inactive immediately |
| Sprint-rejected attempt | gained, no faded | Active, then watchdog Inactive |
| Duplicate gained/faded | repeated same result | No duplicate reader transition |
| Player death | `EVENT_PLAYER_DEAD` | Unknown |
| In-place resurrection | `EVENT_PLAYER_ALIVE` | Inactive |
| Zoning starts | `EVENT_PLAYER_DEACTIVATED` | Unknown |
| Late roll event after death or zoning | invalid lifecycle epoch | Remain Unknown |
| Zoning completes | completed activation rebaseline | Inactive |
| Beacon or game signal loss | reader/runtime authority | Unknown |

Rationale: Unknown distinguishes invalidated evidence from a positive inactive
observation. Activation is a safe baseline because a roll cannot survive a
completed world load. Player alive is also required because an in-place
resurrection need not emit player activation. Death and zoning cancel any pending
watchdog. Combat events are ignored while that lifecycle is invalid, so delayed
delivery cannot reopen the gate.

The companion's configurable fast interval is capped at 375 ms while interception
is active. This preserves slow configured polling while synthesis is disabled but
guarantees multiple supported reads within the 1,500 ms bounded Active publication.

## Missing-completion watchdog

Decision: arm one 1,500 ms deadline on effect gained; effect faded cancels it;
expiry publishes Inactive.

Rationale: the known defect can otherwise strand Active forever. The real faded
event remains authoritative and ordinarily arrives before the deadline. A fixed
1,500 ms interval is conservative relative to the sub-second roll animation
reported by current addon behavior while keeping a rejected attempt bounded. The
deadline uses `GetGameTimeMilliseconds`, which API 101050 exposes as monotonic
game time, and the existing 100 ms fast tick checks it without adding a timer.

Rejected alternatives:

- Unknown on timeout: it would leave generated weave work fail closed indefinitely
  and would not meet the bounded recovery outcome.
- A configurable timeout: no operator-facing choice improves this game contract.
- `zo_callLater`: independently scheduled callbacks are harder to cancel across
  death and zoning than one deadline checked by the existing tick.

## Wire representation and protocol generation

Decision: add B23 with marker `0xF9`, red payloads `0x20` Unknown, `0x80`
Inactive, and `0xE0` Active, and blue `255 - red`. Advance the negotiated header
to version 3 (`0x60`) and preserve version 1 at 22 blocks and version 2 at 23.

Rationale: a dedicated block keeps the state orthogonal. `0xF9` is the midpoint
of the widest remaining green-channel gap, six values from both `0xF3` and
`0xFF`, three times the default tolerance. The new protocol generation prevents
an older version-16 overlay from contributing an ordinary screen pixel as B23.

## Gate ownership and physical input

Decision: add a typed roll gate beside the life gate on InputEngine, publish
gate-closing reader evidence before controller locks, open recovery only after
WeaveEngine is synchronized, and pass both gates to RealSink for mid-sequence
cancellation.

Rationale: each boundary covers a distinct race. Opening before synchronizing the
worker could suppress a newly recovered physical key and then drop its handoff
against stale Active state. The classifier lets the physical
key pass through and avoids enqueueing. The worker drops a handoff that raced with
the state transition. The sink cancels a sequence already in a wait while still
releasing generated input held down. A small reusable atomic gate core avoids
copying synchronization logic while the public life and roll gate types retain
semantic type safety.

Rejected alternatives:

- Gate only WeaveEngine: the hook would swallow the player's key before the
  worker drops it.
- Gate only InputEngine: an already queued or running sequence could survive.
- Reuse LifeGate as the roll handle: that would encode two different facts in one
  name and make future consumers unable to express their intended dependency.

## Presentation

Decision: add Roll dodge to Live HUD with Active, Inactive, Not detected, and
Game not active values. The tooltip explicitly states that Active or unavailable
evidence blocks generated weave skills.

Rationale: roll dodge is a live player fact, while the collapsible System and
State panel remains focused on application, installation, signal, and automation
readiness. The explicit tooltip satisfies the diagnostic requirement without
adding a duplicate weave-status row.

## Pinned-file decision

No dependency or workflow pin changes are needed. S050 changes only application,
addon, tests, and documentation.
