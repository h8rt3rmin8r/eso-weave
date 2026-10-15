# Automatic Catalog Maintenance

The Catalog contains game-data definitions used to identify abilities, effects and other observed IDs. One combined PixelBeacon package collects supported definitions automatically. ESO Weave consumes completed naturally saved data, accumulates compatible facts, builds and verifies an immutable Catalog, then selects it automatically. Native encounter logging is not required.

The only maintenance control is **Disable Catalog maintenance** in File > Settings. It applies directly and persists independently of **Disable native encounter log processing**. Both controls are gray/inactive and both services dormant without a recognized compatible combined package. Compatible presence does not grant permission to replace manually maintained addon files.

## Automatic collector input

<a id="collector-assisted-candidate"></a>

The package owns separate beacon, startup-configuration and Catalog modules. It retains the `EsoWeaveDataSaved` global table/schema, while its `PixelBeacon.txt` manifest makes current ESO saves use `SavedVariables/PixelBeacon.lua`. The legacy `SavedVariables/EsoWeaveData.lua` remains untouched and separately readable. A shared global name does not load that older manifest's file into the new addon. The old encounter recorder is not loaded or automatically mirrored.

For each actual environment, the desktop observes the legacy Catalog file and the current manifest-derived file independently through the same strict stable-file parser. Compatible accepted facts accumulate in application-owned storage; previous verified facts remain available while the new Lua collector starts its generation. Different API/version domains retain their own provenance. Neither game save is copied, rewritten, or used as an inbound command transport.

Catalog starts when required game APIs are ready. Relevant changes are coalesced; combat and unsuitable transitions defer work, followed by automatic resumption or bounded restart. The last completed snapshot remains usable while the next generation is unfinished. Scan, sort, serialization, checksums and retained generations all have bounded work/resource limits. See [Discovery Collector](discovery-collector.md) for actual sources, coverage and gaps.

ESO owns SavedVariables persistence. A priority-save request requests scheduling, not an immediate disk flush or proof of durability. The desktop reads already present and later stable completed generations without a new-save observation boundary. No reload, logout, save, export, collection command or native-logging action is required to operate maintenance.

## Verified build and activation

<a id="reviewed-candidate-import"></a>

The in-process compiler remains the one Catalog build authority. Restricted parsing does not execute Lua. Source identity, checksums, declared versions, schema, integrity, foreign keys and semantic coverage are checked. The actual packaged baseline accepts independently collected facts; later scopes/locales/parents accumulate instead of replacing other supported facts. Evidence lineage is retained separately from effective semantic coverage. Bounded absence is not proof of deletion.

Live and PTS remain distinct. Each discovered actual root has its own provenance and registry; PTS cannot replace the current Live selection. Exact source game/API identities remain attached to facts. A newer client version alone is not acquired/populated data, and unsupported definitions remain unknown.

Candidates use the existing immutable verified candidate contract as an internal pipeline stage. The known managed automatic source path needs no origin acknowledgement or user candidate import/install/review decision. This does not authorize arbitrary unrelated directory files or relax integrity checks. Existing maintainer utilities remain developer tools, not a required player workflow.

The service verifies and opens the staged replacement before atomic selection. Existing readers retain one coherent immutable generation for each operation. Accepted activation updates the Catalog authority and external query path; background results reach retained in-process consumers without a Catalog window. Semantic replay, provenance-only changes and encounter-only shared-file rewrites do not cause effective selection churn.

## Recovery, rollback, and receipts

Transient input/storage failures retry with bounded backoff. Missing/partial/locked/changing saved data stays a waiting or retry state. Invalid or unsupported new data never replaces the last verified working generation. A corrupt/missing selection or selected artifact first recovers an appropriate retained verified generation, then uses a compatible bundled fallback when needed.

Compatible retained version domains remain available for historical lookup; older observations are not relabeled as current. Superseded generations within a domain can be pruned. Publication stops at declared storage pressure instead of evicting historical support: the current implementation caps domains at 256 and retained bytes at 4 GiB. Install/terminal receipt retention is bounded separately. Receipt failure after successful selection does not undo or corrupt that selection.

A shared eligibility model cancels queued/in-flight desktop work when disabled or when its package root disappears. Re-enable reconciles retained progress automatically. Cancellation is honored before the short atomic commit boundary; already committed state remains valid. One root or independent native preference cannot relabel or disable another eligible context's work.

Desktop scheduling stops immediately on disable. The package-owned startup configuration applies the Catalog preference to loaded Lua at its next natural addon load/game session; the application does not force reload or write inbound commands to live SavedVariables. Internal/API state distinguishes requested preference, persistence result, eligibility and unknown already-loaded Lua state.

## Evidence and delivery boundaries

Filesystem, Lua and database fixtures establish their tested desktop/module contracts. They do not prove current in-game save cadence, exhaustive game API visibility or installed platform behavior. Incremental native processing and complete encounter-viewer retirement remain later program issues; the native disable preference/cancellation seam exists before that worker. Stored history and authenticated local database reads remain available independently of maintenance eligibility.
