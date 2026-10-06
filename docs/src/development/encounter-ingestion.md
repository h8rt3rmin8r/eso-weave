# Encounter Ingestion

**Use [Encounter Capture](../features/encounter-capture.md) for the supported
recording and import workflow.** ESO Weave Data records fights in ESO, ESO saves
them in its addon data file, and **Import Saved Capture** copies accepted fights
into desktop history. It validates completed fights and independently replays
eligible current captures before one atomic import transaction. Bulk encounter
records never use Pixel Bus.

ESO's native `Encounter.log` files are a separate provisional candidate.
Incremental tailing means reading new completed lines as ESO writes them;
terminal import means reading the file after the recording has ended. Neither
candidate is the desktop Application Log or File Logging setting. Native-log
qualification does not change or block the supported saved-addon path.

ESO API 101050 and 101051 document controls for enabling encounter logging,
querying its state and version, and selecting supported verbose and inline
formats. The native record vocabulary covers combat boundaries, casts, effects,
unit state, equipment, maps, zones, and trials. Those declarations do not prove
flush cadence, file-sharing behavior, crash durability, callback parity, or
privacy behavior on a user's platform.

The native-log candidate remains provisional. Historical issue #190 describes
the qualification contract; its disposition is not proof of platform behavior.
Qualification would require separately owned Windows and Linux/Proton records of at least three
representative encounters per scenario. S096 single and continuous modes do not
promote or depend on this candidate. Receipts must include source versions,
format and anonymity settings,
record and error counts, replacement or truncation behavior, post-boundary p50,
p95 and maximum latency, CPU, memory, throughput, backlog, and comparison with
callback and terminal SavedVariables coverage. Missing evidence stays unknown.

Readers must consume only newline-terminated records. File replacement or
truncation creates an explicit discontinuity, and unknown or malformed record
types remain visible. The complete qualification contract and pinned primary
sources are in the S092 research and ingestion decision artifacts.
