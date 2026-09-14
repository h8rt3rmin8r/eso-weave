# Encounter Ingestion

Native `Encounter.log` incremental tailing is ESO Weave's provisional preferred
bulk-ingestion candidate. Terminal native-log import is the second candidate,
and terminal import from `EsoWeaveDataSaved.encounter` remains the supported safe
fallback. Bulk encounter records never use Pixel Bus.

ESO API 101050 and 101051 document controls for enabling encounter logging,
querying its state and version, and selecting supported verbose and inline
formats. The native record vocabulary covers combat boundaries, casts, effects,
unit state, equipment, maps, zones, and trials. Those declarations do not prove
flush cadence, file-sharing behavior, crash durability, callback parity, or
privacy behavior on a user's platform.

The native-log candidate remains provisional until separately owned Windows and
Linux/Proton verification records at least three representative encounters per
scenario. Receipts must include source versions, format and anonymity settings,
record and error counts, replacement or truncation behavior, post-boundary p50,
p95 and maximum latency, CPU, memory, throughput, backlog, and comparison with
callback and terminal SavedVariables coverage. Missing evidence stays unknown.

Readers must consume only newline-terminated records. File replacement or
truncation creates an explicit discontinuity, and unknown or malformed record
types remain visible. The complete qualification contract and pinned primary
sources are in the S092 research and ingestion decision artifacts.
