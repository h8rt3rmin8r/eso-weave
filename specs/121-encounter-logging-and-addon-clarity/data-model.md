# Presentation Data Model

No storage or controller schema changes.

| Entity | Fields and relationships | Validation |
| --- | --- | --- |
| Audit entry | Path/location, surface/state, old copy, confusion, final copy or retention reason, guide and coverage | Every discovered relevant entry receives a disposition |
| Addon view | Installation/ownership/package version/API, unknown enablement/loading, reload, game process, saved activity, next step | Installation never proves recording or loading |
| Recording snapshot | Existing mode/channel/state/session/count/interruption/failure fields | Current in-game status versus last disk save explicit |
| Lineage | Ordered original data/loss, validated identity, kind-specific catalog, metric algorithm/quality, recommendation policy | No derived replacement of originals, guessed names or unqualified native input |

Encounter clear preserves catalog and desktop history. Catalog clear preserves encounters. Desktop delete preserves saved game data. Uninstall preserves SavedVariables and imported history. Stop retains data. Catalog cancel retains an incomplete unusable snapshot; catalog start may replace prior inactive collection.
