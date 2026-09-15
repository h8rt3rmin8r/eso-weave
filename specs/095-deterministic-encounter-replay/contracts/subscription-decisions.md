# S095 Encounter Subscription Decisions

## Evidence and universe

The reviewed universe is the encounter-relevant callback and API families in ESO
UI source commits `f76cf16c4e5be7b234d15dc7f676febffa64c5bb` for Live API
101050 and `1baf1131560c2bcd38ffd2bd070728273b25f934` for PTS API
101051, plus the currently pinned LibCombat and Combat Metrics candidate
inventories. The same official signatures apply to selected sources at both API
versions. The known PTS delta for `EVENT_DUEL_FINISHED` adds the platform
display name, but that family remains excluded. Derived LibCombat and Combat
Metrics messages are candidate projections, not direct ESO sources.

## Included callbacks

| Source and signature | Trigger, filter, cost | Fanout and replay correlation | Value and rationale | Evidence and test |
| --- | --- | --- | --- | --- |
| `EVENT_PLAYER_COMBAT_STATE(eventCode, inCombat)` | Boundary change; explicit arm and clean start; one per change | Start is linked to the callback; terminal decision precedes `capture-finish` | Authoritative one-shot scope | Live/PTS docs; addon boundary tests |
| `EVENT_PLAYER_DEACTIVATED(eventCode)` | Deactivation; active capture only; rare | Partial terminal through `capture-finish` | Prevents a false complete result | Live/PTS docs; deactivation SQLite test |
| `EVENT_COMBAT_EVENT(eventCode, result, isError, abilityName, abilityGraphic, abilityActionSlotType, sourceName, sourceType, targetName, targetType, hitValue, powerType, damageType, log, sourceUnitId, targetUnitId, abilityId, overflow, ...)` | Unfiltered selected delivery; high burst | Zero or one damage, healing, death, or resurrection fact linked to the callback | Core descriptive metrics; unknown results stay raw | Live/PTS docs; full Lua differential test |
| `EVENT_EFFECT_CHANGED(eventCode, changeType, effectSlot, effectName, unitTag, beginTime, endTime, stackCount, iconName, buffType, effectType, abilityType, statusEffectType, unitName, unitId, abilityId, sourceType, ...)` | Unfiltered selected delivery; high burst | One effect fact linked to the callback | Uptime without unit-buff polling | Live/PTS docs; differential and rounding tests |
| `EVENT_POWER_UPDATE(eventCode, unitTag, powerIndex, powerType, value, maximum, effectiveMaximum, ...)` | Selected delivery; moderate to high rate | One resource fact linked to the callback | Event-driven resource timeline | Live/PTS docs; differential test |
| `EVENT_ACTION_SLOT_ABILITY_USED(eventCode, slot, ...)` | Action-driven; active capture; up to three API reads | Cast and optional quickslot use share contiguous callback ordinals | Ordered cast evidence and quickslot use | Live/PTS docs; API batch differential tests |
| `EVENT_ACTIVE_WEAPON_PAIR_CHANGED(eventCode, activePair, locked, ...)` | Action-driven; low rate | One bar-change fact | Bar context for casts | Live/PTS docs; differential test |
| `EVENT_PLAYER_DEAD(eventCode, ...)` | Player death; rare | One player death fact | Covers transitions absent from combat results | Live/PTS docs; differential test |
| `EVENT_PLAYER_ALIVE(eventCode, ...)` | Player alive; rare | One player resurrection fact | Completes player life intervals | Live/PTS docs; differential test |
| `EVENT_BOSSES_CHANGED(eventCode, forceReset, ...)` | Roster change; up to 12 bounded API reads | Zero to six changed boss facts use callback ordinals | Immediate boss roster refresh | Live/PTS docs; boss batch tests |
| `EVENT_ACTIVE_QUICKSLOT_CHANGED(eventCode, slot, ...)` | Selection change; one API read | One selected quickslot fact linked to callback | Selected quickslot identity | Live/PTS docs; nil and differential tests |

## Included API and lifecycle observations

| Source and signature | Trigger, filter, worst-case cost | Fanout and replay correlation | Value and rationale | Evidence and test |
| --- | --- | --- | --- | --- |
| `GetSlotBoundId(slot[, hotbarCategory]) -> abilityId` | Each slot use, optional used quickslot, and quickslot change | Return joins the immediately preceding callback batch | Callback supplies a slot, not the stable ID | Live/PTS docs; nil/batch tests |
| `GetCurrentQuickslot() -> slot` | Once per action-slot callback | Decides whether the callback also projects quickslot use | Required deterministic branch input | Live/PTS docs; nil test |
| `DoesUnitExist(unitTag) -> exists` | `boss1` through `boss6` every 500 ms and on roster callback; 12 periodic calls per second | Gates the adjacent power query | Bounded roster presence | Live/PTS docs; boss batch tests |
| `GetUnitPower(unitTag, POWERTYPE_HEALTH) -> value, maximum, effectiveMaximum` | Existing bosses every 500 ms; at most 12 calls per second | Raw sample always retained; projection only when the three-value signature changes | Boss health timeline | Live/PTS docs; changed and nil tests |
| `GetFramerate() -> fps` | Once per second | Primary source of a paired performance fact | Client performance context | Live/PTS docs; rounding/nil tests |
| `GetLatency() -> latencyMs` | After each frame sample; once per second | Adjacent secondary input to the frame-linked fact | Network context | Live/PTS docs; batch/nil tests |
| `capture-finish(reason, requestedComplete)` | Exactly once through terminal reserve | Encounter-end and optional discontinuity use contiguous ordinals | Terminal truth | Repository lifecycle contract; terminal tests |
| `clock-reset(previousRawMs, newRawMs)` | Once on rollback, then capture terminates | Discontinuity plus partial terminal | Temporal integrity | Repository lifecycle contract; reset tests |

The 250 ms update callback is a scheduler, not a retained source. It can initiate
500 ms boss and one-second performance samples but produces no observation itself.

## Excluded families

| Family | Reviewed sources | Why excluded now | Reconsider when |
| --- | --- | --- | --- |
| Activation, zone, and duel boundaries | `EVENT_PLAYER_ACTIVATED`, `EVENT_ZONE_CHANGED`, `EVENT_DUEL_STARTED`, `EVENT_DUEL_FINISHED` | Combat state already scopes the capture; these widen location and identity exposure | Measured encounter segmentation failures remain after current verification |
| Group roster | `EVENT_GROUP_UPDATE` | No current metric requires roster snapshots; identity fanout is material | An approved composition metric has bounded identity and value contracts |
| Hotbar and loadout | `EVENT_HOTBAR_SLOT_CHANGE_REQUESTED`, `EVENT_ACTION_SLOT_UPDATED`, `EVENT_ACTION_SLOTS_FULL_UPDATE` | Current cast and bar facts do not require a full build snapshot | An explicit-consent, versioned build snapshot is approved |
| Inventory quickslot | `EVENT_INVENTORY_ITEM_USED` | Selected and used quickslot facts need no inventory mutation event | Live evidence shows unresolved attribution after catalog join |
| Alternate life cycle | `EVENT_UNIT_DEATH_STATE_CHANGED`, `EVENT_PLAYER_REINCARNATED`, `EVENT_RESURRECT_REQUEST`, `EVENT_RESURRECT_RESULT` | Current combat-result and player dead/alive sources cover required facts | Live verification finds a material missed transition |
| Periodic player/group stats | `GetPlayerStat` and group `GetUnitPower` fanout | High polling cost and no current consumer | A bounded metric proves representative value and cost |
| Unit-buff polling | `GetUnitBuffInfo` fanout | `EVENT_EFFECT_CHANGED` supplies current effect facts event-first | Live comparison proves a gap with a bounded polling remedy |

`EVENT_ADD_ON_LOADED` remains package initialization, not an encounter source.
No family is excluded because its values are inconvenient to retain; each
decision is based on current product value, cost, privacy, and replay necessity.
