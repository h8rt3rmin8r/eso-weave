# S129 Automatic Data Contract

## Package

One PixelBeacon manifest/inventory/version loads startup configuration, isolated bootstrap, beacon and Catalog. Legacy Encounter.lua does not load/auto-resume. Preserve the EsoWeaveDataSaved global/table schema and retain the old SavedVariables/EsoWeaveData.lua unchanged. PixelBeacon.txt determines the current SavedVariables/PixelBeacon.lua file; the global name does not override that filename. Lua never loads the legacy file or resumes its recorder. Staged migration validates exact managed snapshots, rolls back precommit failure, distinguishes postcommit cleanup failure and preserves conflicts.

beacon::inspect_presence(addons_root) returns compatible facts independent of mutation ownership. install_configured and write_startup_config take actual environment, block size and Catalog optout. Compatible manual installs may be eligible without being overwriteable.

## Preferences and cancellation

catalog_disabled and native_processing_disabled default false and persist independently. Both controls inactive if no recognized compatible combined context; both services off in that state. Each actual root is evaluated separately; Settings selection cannot relabel jobs. Persistence failures preserve current-run requested stop and report failed durable success internally. Desktop cancellation immediate, Lua preference next natural load.

## Collection and accumulation

Retain last complete catalog envelope while runtime work proceeds. Bounded stages include sorting/encoding/checksum. Actual metadata and supported facts only. Native logging is independent. Stable complete saved input, not file mtime alone, drives desktop work. Preserve independent scopes/locales/all supported parents; lineage change alone is not coverage loss or semantic change. Version-sensitive facts never relabel across incompatible channel/API/game identity.

## Service

CatalogUpdateService::maintain_collector_capture(path,cancel,progress) consumes stable completed data with no observation boundary/acknowledgement, serializes operations and returns Activated or Unchanged. The automatic coordinator reconciles both legacy then current paths from catalog_saved_variables_paths with independent strict stable reads and retry state. Compatible accepted facts merge only in application-owned accumulation, with prior usable generations retained; neither game's SavedVariables file is copied, moved, or rewritten. Verify/open before atomic selection. Preserve last-good/compatible history and recover retained verified selection before fallback. Activated emits coherent current catalog path/access event; readers finish consistently on their captured generation. Receipts and semantic selection remain distinct.

Queue, read, retry, retained-generation and shutdown budgets are explicit implementation constants with tests. Ignore shared encounter-only rewrites. Cancellation prevents stale activation; no-op avoids repeated expensive invalidation. Source parsing never executes Lua or commands. Diagnostics remain bounded and omit raw private values.
