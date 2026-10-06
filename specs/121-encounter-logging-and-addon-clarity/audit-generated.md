# Generated Diagnostics and Analysis Copy Audit

This audit covers all public message branches in the history service/worker,
metric and recommendation presentation, recommendation reason generation, and
catalog-update collection/worker status. Internal filenames, receipt codes,
schema identifiers and algorithms remain unchanged. Raw input values never
enter these messages. [Shared wording contract](contracts/messaging.md).

| Source and surface/state | Previous wording | Confusion | Final wording or retained explanation | Help |
| --- | --- | --- | --- | --- |
| `src/encounter/history.rs`, source path unavailable | selected environment could not provide a terminal encounter capture | No meaning of terminal or setup action | Saved addon data location unavailable; choose Live/PTS/AddOns in Settings, record and save with /reloadui/logout/exit | encounter-capture |
| same, missing source file | no terminal encounter capture to import | Cannot distinguish no recording from no disk save | No saved addon data file; check environment/folder, record, save before import | encounter-capture |
| same, import rejection/write failure | capture rejected or could not be imported | No recovery; does not distinguish validation and access | Saved recording could not be imported, prior history preserved; check environment, finish/stop/save/retry; preserve unsupported/damaged data | troubleshooting |
| same, missing selected encounter | no longer present, Refresh history | Already explicit | Retain exact message: selected encounter no longer present; Refresh history | interface |
| same, missing/unavailable catalog (both branches) | active catalog unavailable, raw history remains available | Why catalog blocks metrics unclear | Game-data definitions unavailable; metrics unavailable, imported recordings stored; Catalog Update when compatible definitions available | encounter-data-and-metrics |
| same, corrupt/unsupported/checksum catalog | active catalog invalid or unsupported | No recovery | Game-data catalog damaged/unsupported; metrics unavailable, originals stored; select compatible catalog or verified rollback | catalog-updates |
| same, metadata validation failure | catalog metadata could not be validated | Metadata jargon | Catalog's game-data identity could not be checked; originals stored, select/rollback | catalog-updates |
| same, channel/API mismatch | active catalog channel or API does not match capture | Why and PTS limitation unclear | Environment/API differ; matching game definitions needed; current Catalog Update installs Live only, PTS metrics may remain unavailable | encounter-data-and-metrics |
| same, calculation failure | observed metrics could not be calculated | No next action | Original stored; Refresh/select again, application log if persistent | troubleshooting |
| same, all database access/read/delete failures | local encounter store unavailable/corrupt/unsupported, unchanged | Store jargon and no recovery | Imported history database may be unavailable/damaged/unsupported; left unchanged, check app log/folder access, keep a copy before recovery | troubleshooting |
| `src/app/encounter_history.rs`, delete missing/one/all | already absent / deleted local history / deleted local records | Scope of other copies unclear | Imported history on this computer only; ESO saved addon data unchanged and can be reimported | encounter-capture |
| same, empty import | no terminal encounters | Terminal jargon and no save instruction | No finished/interrupted encounters saved; current status in ESO, finish/stop/save then import | encounter-capture |
| same, new/duplicate/mixed imports | count imported / already present | Counts already concrete | Retain count/pluralization; enclosing UI explains imported desktop copies and save/source boundary | interface |
| same, metric Complete/Degraded | Complete / Degraded | Degraded unexplained | Complete retained as scoped observation quality; Incomplete observations replaces Degraded, exact gaps adjacent | encounter-data-and-metrics |
| same, loss ranges | Sequences X-Y (reason) | Sequence/limit codes unexplained | Missing observations X-Y, plain reason and exact code retained for reproducibility | encounter-data-and-metrics |
| same, all loss reasons | capture-overflow, record-limit, byte-limit, string-limit, unsupported-value, clock-reset, runtime-interrupted, user-stopped, callback-failed | Technical codes only | Recording/storage/size limit, unrecordable game value, backward clock, interruption/stop/failure; unknown code explicitly a declared gap. Exact code kept in parentheses | encounter-capture |
| same, missing metric and units | Unavailable, %, damage/s, effective healing/s | Scoped value already honest | Retain unavailable and explicit units; UI explains damage and effective healing, numeric game IDs | encounter-data-and-metrics |
| same, recommendation ready/qualified/suppressed | Evidence status: Ready/Qualified/Suppressed | Evidence gate jargon | Review prompts available / have limitations / unavailable | encounter-data-and-metrics |
| same, no advice due gate or threshold; advice present | s090-v1 gates / thresholds / deterministic review prompts | Policy identifier obscures outcome | No prompts because analysis requirements not met, reasons below; no available prompts, with reasons covering omitted targets or unmet thresholds; questions help review and do not establish cause/optimal rotation | encounter-data-and-metrics |
| same, two prompt bodies/titles | dominant damage share / low effect uptime and compare intention | Concrete scoped observations | Retain exact numeric ability/effect, percentage and intention questions; UI explains IDs/uptime and displays full technical citation in secondary disclosure | encounter-data-and-metrics |
| same, qualification | provisional, qualified review prompt | Qualified unexplained | Provisional review prompt with limitations, repeats exact applicable reason/gaps beside prompt | encounter-data-and-metrics |
| same, full citation | schema/algorithm/catalog semantic/hash wall | Provenance overwhelms action | Retain every identity value, with explanatory secondary technical disclosure; no identity removed | encounter-data-and-metrics |
| `src/recommendation/mod.rs`, insufficient duration/casts | observed duration/cast count, advice requires limits | Advice versus calculation unclear | Recorded fight length needs 10 seconds; at least three recorded skill casts for review prompts; exact counts retained | encounter-data-and-metrics |
| same, unsupported schema/algorithm | projection schema/algorithm unsupported, s090-v1 requires | Unexplained version jargon | Calculated-result format/calculation version unsupported; expected versions named, original unchanged | encounter-data-and-metrics |
| same, invalid span/loss/metric evidence | sequence span invalid / reversed loss range / inconsistent metric evidence | Technical diagnosis without meaning | Observation order, missing-observation order or completeness/coverage/calculation disagree; review prompts unavailable | encounter-data-and-metrics |
| same, material and minor declared loss | source sequences, suppression/qualified | Qualification scope unclear | Missing X of Y observations; at least 10% makes prompts unavailable, any available prompt carries a smaller-gap limitation | encounter-data-and-metrics |
| same, unknown IDs | numeric catalog IDs unresolved; known-target qualified | Catalog meaning unclear | Game-data IDs cannot be identified by definitions; any available known ability/effect prompt carries limitation | encounter-data-and-metrics |
| same, material unknown damage | unknown abilities account for percentage, damage advice omitted | Exact reason already concrete | Retain exact percentage and rule-local omission, supported by common ID explanation | encounter-data-and-metrics |
| same, omitted unknown target | threshold-crossing unknown target unresolved | Target/threshold jargon | Unidentified ability/effect met review threshold but no prompt because its game-data definition is missing | encounter-data-and-metrics |
| `src/catalog_update/mod.rs`, wait/build progress | SavedVariables flush after waiting boundary | Unclear who starts collection | Checking whether ESO saved new catalog data since desktop observation began | discovery-collector |
| same, unchanged/changing source | collector capture unchanged/still changing | Flush jargon/no collection step | Finish /ewcollect collection and save; wait for file save then retry building | discovery-collector |
| same, no catalog subtree | shared SavedVariables has no catalog capture | Shared-file jargon | Saved addon data contains no catalog collection; /ewcollect help, complete and save | discovery-collector |
| same, PTS build refusal | only complete Live collector capture builds active candidate | Scope unclear | Complete Live catalog collection only for desktop selection; PTS cannot be installed here | catalog-updates |
| same, path/selection/staging validation | controlled path/integrity reason | Maintainer details needed for review | Retain bounded technical validation details in Catalog Update; feature and operation defined in modal, never arbitrary capture payload | catalog-candidate-pipeline |
| `src/catalog_update/worker.rs`, refresh | Collector status refreshed | Could imply live collection | Installation refreshed; does not report current in-game activity | interface |
| same, installed/reload state | flush catalog data | No enable/start step | Enable ESO Weave Data in Add-Ons, /reloadui, /ewcollect help, finish/save collection | discovery-collector |
| same, install incomplete | ownership/path safety gate | Ownership-only diagnosis falsely excludes I/O or cleanup failures after replacement | Check folder/access/current files, replacement or cleanup may have occurred, preserve recovery files; PixelBeacon/saved data unaffected | troubleshooting |
| same, remove success/incomplete | managed removed / absent or not marker-owned | Scope and marker jargon; post-quarantine cleanup may fail | Addon files only on success; incomplete operation may have moved files aside, inspect current folder/preserve recovery files; other addons/saved data/imported history unchanged | encounter-capture |
| same, origin acknowledgement | candidate came from trusted source | Existing precise trust condition | Retain request; catalog modal explains candidate and explicit acknowledgement, no new gate | catalog-updates |
| same, concurrent operation | already running | No next action | Wait for finish or cancel before retrying | catalog-updates |
| same, no rollback target | no previous verified catalog | No available alternative | No verified previous catalog; keep selection or choose available candidate | catalog-updates |
| same, invalid selection | bundled fallback remains active | Fallback jargon | Bundled game-data definitions used; Catalog Update selects verified replacement | catalog-updates |
| same, pipeline rejection | complete integrity/policy verification failed | Policy jargon | Proposed catalog failed data-integrity/source-policy checks; keep current, obtain corrected trusted candidate | catalog-candidate-pipeline |
| same, I/O and JSON | storage could not complete / malformed contract | No recovery | Check application-folder access/disk space and retry; invalid update format needs compatible candidate/app | catalog-updates |
| same, cancel/progress/results | catalog operation cancelled and bounded stages | Concrete operation state | Retain scoped cancellation and numeric progress; desktop explains stage and current/busy result, cancellation does not erase recordings | catalog-updates |

## Coverage and tests

Source extraction includes all literals in these five files, not just changed
lines. Retained machine identifiers and internal filenames are not new public
labels. Public generator branches are all listed above, including errors reached
through worker results. Tests retain import atomicity/idempotency, kind/API
matching, exact loss and all recommendation evidence gates. New diagnostic and
loss-presentation fixtures require setup/save actions and explained omissions.
