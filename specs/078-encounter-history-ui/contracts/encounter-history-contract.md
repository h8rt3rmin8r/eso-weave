# Encounter History Contract

## Storage and import

1. The desktop store is `<application-root>/encounters/encounters.sqlite`.
2. A missing store lists as empty and is not created by listing.
3. Import begins only from the Import Current Capture control.
4. The source is `<selected-environment>/SavedVariables/EsoWeaveEncounter.lua` and
   the expected channel is the same selected environment.
5. Import uses the S076 hostile-data boundary unchanged.

## Detail

1. A detail request names one complete session and encounter identity.
2. The service validates and loads raw data from the S076 store.
3. The service opens the current catalog path read-only and requires exact channel and
   API compatibility.
4. It calculates the S077 projection in memory and publishes no derived file or row.
5. A detail failure does not discard or replace the current raw summary snapshot.
6. Startup selection, installation, and rollback replace the active catalog path;
   an already selected encounter is then reprojected against that replacement.

## Presentation

1. Every headline is labeled Observed.
2. Every metric family exposes its S077 quality.
3. Degraded quality exposes all exact loss ranges and reasons.
4. Catalog and algorithm versions, semantic/raw hashes, channel, API, known count,
   and all unknown IDs remain visible.
5. Unavailable values render as `Unavailable`, never numeric zero.
6. The interface makes no Combat Metrics parity or completeness claim.

## Deletion

1. Delete Encounter requires a confirmation for the selected identity.
2. Delete All requires a distinct confirmation naming all local encounter data.
3. Cancel and window close perform no deletion.
4. Success refreshes history; failure preserves the store and reports a safe message.

## Isolation

The history service has no references to input, weave, fishing, potion, game focus,
Pixel Bus, upload, telemetry, or catalog mutation services.
