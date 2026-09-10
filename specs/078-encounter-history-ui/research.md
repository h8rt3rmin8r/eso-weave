# Research: Quality-Aware Encounter History UI

## Existing authorities

- S076 already provides bounded stable import, deterministic listing, validated load,
  transactional one/all deletion, and a dedicated immutable raw store.
- S077 already provides deterministic calculation over a loaded capture and a
  read-only `CatalogAccess`, including metric quality, loss, catalog identity, and
  sorted known/unknown IDs.
- `AppModel` already resolves the configured Live or PTS AddOns directory through a
  path seam that respects manual overrides and platform discovery.
- The catalog update flow atomically replaces one stable catalog path. Reopening that
  path for a history request gives the latest accepted snapshot.
- Existing egui tests use `egui_kittest` and accessibility labels, so the history
  window can be tested without a desktop or window manager.

## Alternatives evaluated

### Derived SQLite history tables

Rejected. Summary facts already exist in the raw-store index, and one detail can be
recalculated deterministically. Persisting derived rows creates catalog invalidation,
algorithm migration, delete-cascade, and crash-recovery behavior with no demonstrated
performance need.

### Arbitrary file picker

Rejected for S078. It would add a new native-dialog dependency and expand the input
boundary. The product already has one explicit environment setting and a deterministic
SavedVariables location. A future export/import workflow can add a picker if users need
history portability.

### Render-thread list and calculation

Rejected. A capture can hold 100,000 events and a source file can reach 64 MiB. Even
read-only work at those limits can visibly stall egui. One worker is adequate because
all history mutations are intentionally serialized.

### Shared catalog handle across worker threads

Rejected. A path is the stable authority and is cheap to reopen. Passing SQLite-backed
state between threads complicates ownership and could keep a replaced catalog snapshot
alive after the user updates it.

## Selected approach

Implement a synchronous, typed history service and a capacity-one app worker. Treat an
absent store as an empty snapshot without creating it. Derive the source path only when
the user requests import. Retain summaries across detail errors. Render a resizable,
scrollable history window plus focused destructive confirmations. Keep UI language
explicitly observational and keep live comparison under issue #131.
