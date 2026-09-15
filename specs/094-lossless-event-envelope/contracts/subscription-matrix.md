# Contract: S094 Subscription and Observation Matrix

| Source | Kind | Reviewed values | Trigger/rate | Compatibility projection |
| --- | --- | ---: | --- | --- |
| EVENT_PLAYER_COMBAT_STATE | callback | event code + inCombat | boundary changes | encounter-start/end |
| EVENT_PLAYER_DEACTIVATED | callback | event code | player deactivation | partial terminal |
| EVENT_COMBAT_EVENT | callback | event code + 17 values + future extras | unfiltered delivery | damage, healing, death, resurrection when recognized |
| EVENT_EFFECT_CHANGED | callback | event code + 16 values + future extras | unfiltered delivery | effect |
| EVENT_POWER_UPDATE | callback | event code + 6 values + future extras | unfiltered delivery | resource |
| EVENT_ACTION_SLOT_ABILITY_USED | callback | event code + slot + future extras | unfiltered delivery | cast and optional quickslot |
| EVENT_ACTIVE_WEAPON_PAIR_CHANGED | callback | event code + pair + locked + future extras | unfiltered delivery | bar-change |
| EVENT_PLAYER_DEAD | callback | event code + future extras | unfiltered delivery | death |
| EVENT_PLAYER_ALIVE | callback | event code + future extras | unfiltered delivery | resurrection |
| EVENT_BOSSES_CHANGED | callback | event code + forceReset + future extras | unfiltered delivery | triggers boss sampling |
| EVENT_ACTIVE_QUICKSLOT_CHANGED | callback | event code + slot + future extras | unfiltered delivery | quickslot |
| GetSlotBoundId | api-sample | inputs + bound ID | slot callbacks | cast/quickslot |
| GetCurrentQuickslot | api-sample | no input + slot | action-slot callbacks | quickslot decision |
| DoesUnitExist | api-sample | unit tag + boolean | 500 ms boss sample | boss-health decision |
| GetUnitPower | api-sample | unit tag + power type + three returns | changed boss sample | boss-health |
| GetFramerate | api-sample | no input + FPS | 1 second | performance |
| GetLatency | api-sample | no input + latency | 1 second | performance |
| capture-finish | lifecycle | reason + requested-complete | exactly once | encounter-end |
| clock-reset | lifecycle | previous and new raw clock | on rollback | discontinuity |

`EVENT_ADD_ON_LOADED` is excluded because it initializes package state and is not
an encounter observation. No new gameplay event family is introduced by S094.

