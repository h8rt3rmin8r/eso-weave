# Contract: Canonical Player State 1.0.0

## Envelope

Every response contains `schema_version`, `snapshot_revision`, `captured_at`, `service_generation`, `capabilities`, and all six domain objects. Fields are stable and present even when their observation reports `unknown`, `unavailable`, or `dormant`.

Each observation uses the metadata model from [data-model.md](../data-model.md). A domain may declare shared source and protocol metadata that its children inherit. Retained display values become `stale`; current focus and signal facts never inherit that retention.

## Complete public inventory

The table maps every current external fact represented by `GameObservations`, `PixelBusReader`, `AppView`, fishing, auto-potion, weave configuration, and native binding resolution. Arrays have fixed semantic members even when unavailable.

| Canonical path | Type and members | Current authority |
| --- | --- | --- |
| `application.lifecycle` | enum `running`, `suspended` | application model |
| `application.version` | semantic version string | build metadata |
| `application.catalog` | availability and immutable catalog release identity | catalog model |
| `application.addons.data` | managed lifecycle, ownership, compatibility, configured enablement, loaded/reload state, catalog activity, encounter activity, and safe remediation | data-addon lifecycle model |
| `game.installation` | availability, provider, install classification; no path | game observations |
| `game.runtime` | process running state and process identity without executable path | game observations |
| `game.focus` | focused, unfocused, or unknown | game observations |
| `game.context` | derived active, dormant, or unavailable context | game observations |
| `game.surface` | `none`, `system_menu`, `map`, `inventory`, `mail`, `character`, `guild_store`, `crown_store`, `journal`, `chat_entry`, `other`, `unknown` | game and PixelBus observations |
| `game.world` | `active`, `transitioning`, `unknown` | PixelBus world observation |
| `pixel_bus.layout` | protocol revision, layout revision, mode, geometry validity, and compatibility | PixelBus reader |
| `pixel_bus.signal` | heartbeat, signal lost, sample generation, observation age, and capture latency | PixelBus reader |
| `pixel_bus.addon` | PixelBeacon managed installation and compatibility state | addon lifecycle and PixelBus reader |
| `pixel_bus.fishing_signal` | `none`, `waiting`, or `bite` | PixelBus fishing observation |
| `player.weapon.active_bar` | `front`, `back`, `unknown` | PixelBus weapon observation |
| `player.weapon.front_class` | `dual_wield`, `two_handed`, `sword_and_shield`, `bow`, `destruction_staff`, `restoration_staff`, `unknown` | PixelBus weapon observation |
| `player.weapon.back_class` | same enum | PixelBus weapon observation |
| `player.combat` | `in_combat`, `out_of_combat`, `unknown` | PixelBus combat observation |
| `player.movement` | `on_foot`, `mounted`, `sprinting`, `unknown` | PixelBus movement observation |
| `player.life` | `alive`, `dead`, `recovering`, `unknown`; recovery path `ghost`, `world_activation`, `no_load` | PixelBus life observation |
| `player.roll_dodge` | `active`, `inactive`, `unknown` | PixelBus roll-dodge observation |
| `player.travel` | `pending`, `inactive`, `unknown` | PixelBus travel observation |
| `player.resources.health` | percent 0 through 100 or unknown | PixelBus resource observation |
| `player.resources.stamina` | percent 0 through 100 or unknown | PixelBus resource observation |
| `player.resources.magicka` | percent 0 through 100 or unknown | PixelBus resource observation |
| `player.ultimate.current` | nonnegative points or unknown | PixelBus Ultimate observation |
| `player.ultimate.maximum` | nonnegative points or unknown | PixelBus Ultimate observation |
| `player.ultimate.front_cost` | nonnegative points or unknown | PixelBus Ultimate observation |
| `player.ultimate.back_cost` | nonnegative points or unknown | PixelBus Ultimate observation |
| `player.ultimate.active_cost` | derived cost for active bar or unknown | canonical projection |
| `player.ultimate.ready` | true, false, or unknown | canonical projection |
| `player.cooldowns.skills` | fixed slots 1 through 5, each `ready`, `remaining_ms`, or `unknown` | PixelBus cooldown observation |
| `player.cooldowns.ultimate` | `ready`, `remaining_ms`, or `unknown` | PixelBus cooldown observation |
| `player.quickslot.classification` | `empty`, `non_potion`, `potion`, or unavailable with reason | PixelBus quickslot observation |
| `player.quickslot.non_potion_kind` | `item`, `collectible`, `quest_item`, `emote`, `quick_chat`, `other` when applicable | PixelBus quickslot observation |
| `player.quickslot.potion_availability` | `usable`, `depleted`, `blocked` when applicable | PixelBus quickslot observation |
| `player.quickslot.cooldown` | `ready`, `remaining_ms`, or unknown | PixelBus quickslot observation |
| `player.quickslot.item_id` | unsigned item identifier or unavailable | PixelBus quickslot observation |
| `player.quickslot.unavailable_reason` | `no_signal`, `legacy_protocol`, `corrupt_protocol`, `unsupported_api`, `invalid_selection`, `inconsistent_facts` | PixelBus quickslot observation |
| `player.bindings` | fixed records for skill 1 through 5, Ultimate, synergy, attack, block, interact, and quickslot | native binding resolver |
| `player.bindings[].state` | `valid`, `unbound`, `conflicting`, `unsupported`, or unavailable | native binding resolver |
| `player.bindings[].chord` | normalized control plus modifiers only when valid | native binding resolver |
| `automation.fishing.requested` | boolean | persisted settings and session intent |
| `automation.fishing.effective_state` | `disabled`, `armed`, `waiting`, `reeling`, `recast` | fishing controller |
| `automation.fishing.stop_reason` | `user_stop`, `no_cast_detected`, `signal_lost`, `game_inactive`, `unfocused`, `suspended`, `player_unavailable`, `world_unavailable`, `travel_pending`, `settings_changed`, `interact_unavailable`, or none | fishing controller |
| `automation.auto_potion.requested` | boolean | persisted settings and session intent |
| `automation.auto_potion.effective_state` | `off`, `dormant`, `blocked`, `ready`, `triggered` | auto-potion controller |
| `automation.auto_potion.reason` | dormant, blocked, or trigger reason including resource, observed percent, and threshold where applicable | auto-potion controller |
| `automation.weave.slots` | fixed seven-slot interpretation configuration with active flag, weave type, and override | persisted weave configuration |
| `automation.weave.slots[].effective_delay_ms` | resolved per-slot delay for the current bar profile | canonical projection |
| `interpretation.auto_potion.thresholds` | enabled health, stamina, and magicka watches and thresholds | persisted auto-potion configuration |
| `interpretation.weave.front_timing` | configured front-bar timing and auto-timing mode | persisted weave configuration |
| `interpretation.weave.back_timing` | configured back-bar timing and auto-timing mode | persisted weave configuration |
| `interpretation.latency` | configured compensation facts that affect timing interpretation | persisted settings |
| `interpretation.pixel_bus` | block size, color tolerance, heartbeat timeout, fishing interval, and idle interval | reader configuration |

## Capability metadata

Capabilities report the external schema version, current MCP protocol revisions accepted by RMCP, state domains, database identifiers, supported operations, and source protocol support. A field remains present when its source protocol is unavailable; its observation explains why.

## Coherence rules

- `snapshot_revision` changes whenever any public value or its knowledge/freshness metadata changes.
- A response serializes one immutable snapshot reference.
- `captured_at` is the publication time, not a substitute for leaf `observed_at`.
- Signal or focus loss immediately updates lifecycle and freshness, even if value retention keeps a prior fact visible.
- Derived values cite their input authorities and inherit the least-fresh input.
- HTTP and MCP serialization must compare equal after transport envelope removal.

## Deliberately non-public current state

| Internal state | Reason |
| --- | --- |
| Rendered status lines, labels, colors, and log filters | Presentation, not canonical facts |
| Live log entries and raw diagnostics | May contain sensitive operational detail and are not player state |
| Filesystem paths, addon staging paths, and database paths | Local information disclosure without client value |
| Bearer token and credential storage metadata | Secret |
| Raw pixels, capture buffers, image handles, and window handles | High-volume implementation detail |
| Input-hook handles, injected-event markers, queues, and command channels | Safety-critical implementation detail |
| Mutable controller internals, timers, debounce counters, and retry counters | Unstable implementation detail; externally meaningful effective state is public |
| PixelBus death-episode latch, prior-heartbeat latch, and configuration generation counters | Internal state-machine bookkeeping; their public outcomes are already represented |
| UI geometry, selected tab, modal state, and transient toast state | Application presentation, not player state |
| Raw SQL connection objects and catalog import bookkeeping | Internal storage machinery; database schema discovery is public |

Any new source observation must be deliberately added to this inventory or the non-public table before issue #177 can pass its maintained inventory test.
