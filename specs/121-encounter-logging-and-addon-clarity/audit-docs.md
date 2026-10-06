# S121 Public and Bundled Help Audit

This is the docs/help portion of the complete #249 audit, implemented with the
#222 encounter-lineage figure. The desktop and addon matrices account for
runtime strings separately. Every guide below comes from `docs/src`, which is
the single source for GitHub Pages and the executable's offline help. No live
ESO session or installed application was used.

## Inventory method and disposition

The source scan covered every published page for encounter/capture, combat
logging, SavedVariables, ESO Weave Data, catalog collection, lifecycle controls,
Application/Live Log and their old technical labels. Relevant guide fronts,
links, inline command examples, code blocks, tables, tooltips represented by
help, captions and maintained figure text were then read in context. Technical
contracts remain secondary to a repaired complete player workflow; no entry
below is left unresolved by merely linking a glossary.

Locations name the exact file and owning heading. A family contains the
reachable states listed in its State column. "Retained" means reviewed and
justified, not omitted from the audit. Screenshot bytes and their manifest
captions remain unchanged; nearby help explicitly identifies older labels.

## Public message and state matrix

| ID | Surface and exact source | State | Former wording or omission | Final wording or disposition | Why / coverage |
| --- | --- | --- | --- | --- | --- |
| H01 | `docs/src/features/encounter-capture.md`, introduction | New player, all modes | "explicit local observation tool", "bounded ESO SavedVariables spool" | ESO Weave Data records fights for imported Encounter History; single versus continuous explained before implementation detail | Names the user task, where it happens and why the addon exists |
| H02 | Same, introduction | Addon/logging confusion | Package, PixelBeacon and log roles scattered among references | PixelBeacon supplies current readings/input features; Application Log and File Logging diagnose the app; native combat logs are a separate provisional route | Distinguishes every recording/log role at the workflow entrance |
| H03 | Same, Install and choose a mode | Missing package, managed update/repair, selected environment | Starts with repository `addon/EsoWeaveData` and manifest-version history | Settings Live/PTS, Install/Update/Repair Data, enable in ESO's Add-Ons menu, reload, choose mode/channel and toggle | Complete installation through start, matching desktop names |
| H04 | Same, Install and choose a mode | Existing retained session, changing mode/channel | "Choose the mode and channel while capture is off" | Must be off with no retained session; optional save/import, deliberate clear, then change selection | Stopping preserves the session and alone does not permit reconfiguration |
| H05 | Same, command table | Toggle/status/clear/help/aliases | "disable current authority", "selected/requested/active", "clear retained encounter evidence" | Start/stop retains recordings; chosen/requested/actual state and retained/finished counts; clear scope explicit; aliases retained as compatibility detail | Does not confuse current in-memory status counts with disk saves |
| H06 | Same, Save, import and view | Single finished, continuous active/off, stop mid-combat | Full sequence buried under storage contract | Finish or stop; save through reload/logout/exit; File > Encounter History > Import Saved Capture; choose fight and read metrics then prompts | Exact actions, with stop-mid-fight partial meaning |
| H07 | Same, Save, import and view | Duplicate, growing snapshot, busy, disabled import, changing file | No practical distinction among Refresh, import and saving | Refresh rereads desktop history; repeated import skips duplicates; completed saved continuous fights may import while current fight is omitted; wait for busy work, save/retry changing file | Root confirmed current `validate_controller_spool` separates terminal records and current fight |
| H08 | Same, Clear the intended copy | Addon clear, desktop delete-one/all, catalog clear, uninstall | Separate deletion domains existed without a user comparison | Lists exact removed and preserved copies; importing unchanged saved file can restore deleted desktop fights; ESO must save cleared state | Gives deliberate clear/delete effects without new prerequisites |
| H09 | Same, Clear the intended copy | Unknown version, invalid supported-version state | Clear recovery not adequately qualified | Unsupported saved-data version stays preserved and cannot be cleared; compatible-release action, no version edits; supported-version invalid state may be deliberately cleared | Does not offer an operation the addon refuses |
| H10 | Same, Captured facts and implementation detail through Save and ownership boundary | Complete, legacy, partial, loss, clock interruption, recovered failure | Raw schema, terminal, replay and spool vocabulary mixed into first-use instructions | Retained below practical workflow, with "Last saved recording state" and explicit partial replay-indeterminate rule | Maintainer contract detail is useful after user actions; raw values/versions remain truthful |
| H11 | `docs/src/reference/encounter-data-and-metrics.md`, introduction | Imported versus live history | Begins with S069-S096 implementation chronology | Imported copies on this computer; metrics then qualified questions; setup link; catalog, saved addon data and logs defined | Reader can understand the reference before version history |
| H12 | Same, Encounter evidence lineage / Encounter lineage text equivalent | Every lineage stage | Scattered prose, no owned figure | Six-stage local SVG and full numbered equivalent | Details ordered observations, validation/replay/loss, original storage, kind-specific catalog, metrics and separate prompts |
| H13 | Same, equivalent stages 1-3 | Single/continuous, current/last save, legacy/partial/invalid batch, exact repeat/conflict | Current/saved and immutable identities separated across long sections | Encounter ordinal then raw sequence, last-save lag, eligible complete replay, partial/legacy outcomes, batch preservation, exact skip and changed-content refusal | Figure and prose name the same gates and preserve original values |
| H14 | Same, equivalent stages 4-5 | Wrong channel/API, kind mismatch, unknown ID, missing catalog, no denominator, declared loss | Catalog lookup and degraded metrics buried below ownership | Exact Live/PTS and API; ability/effect kind isolation; unknown IDs unresolved; summary retained when calculation unavailable; Incomplete observations and unavailable rates | Unknown or unavailable never becomes zero, a known entity of another kind or complete data |
| H15 | Same, equivalent stage 6 / Provisional recommendations | Available, qualified, suppressed, unknown targets/version/quality | Advice-oriented engineering status without the user outcome at the figure | s090-v1 separate gates; at most two questions; Review prompts available / have limitations / unavailable; threshold and identity detail retained | Does not claim causes, optimal rotations, input authority or Combat Metrics parity |
| H16 | Same, native qualification and Combat Metrics comparison boundary | Native route, field parity, historical closed issues | "Implementation work proceeds", verification issues described as future gates | Implemented sequence and historical comparison contract; issue disposition is not measurement; native path remains provisional | A closed issue neither upgrades evidence nor creates a new field-check task |
| H17 | `docs/src/features/interface.md`, Responsive dashboard | Installed/current/outdated/unmanaged, enable/load unknown | Data Addon Ownership, Compatibility, Runtime; "evidence facts", current Unconfirmed unexplained | Addon Management/Package Version, Enabled/Loaded in ESO with Unconfirmed explained, Reload Reminder and ESO Client; collection Unknown (check inside ESO) | Matches exact separate desktop rows and current observations |
| H18 | Same, Responsive dashboard | Last saved recording, history import/refresh, delete | "historical and read-only", "capture state", no import distinction | Encounter History shows Last saved recording state from the last successful import; Refresh/failed imports preserve it and Saved channel may differ from selected environment; System/Data Details current recording stays unknown; in-game status commands; Import Saved Capture versus Refresh; precise retained-copy delete explanation | No false live control or acknowledgement |
| H19 | Same, Offline documentation and Live log and settings | Offline help, stale readings, diagnostic pane | Live Log, visible HUD Freshness row | Application Log; freshness transition logged without temporary row; offline read-only help contract retained | Synchronizes new diagnostic name and S120's removed notification; no screenshot refresh claim |
| H20 | `docs/src/getting-started/first-launch.md`, steps 1/4/5 | First installation, update/reload, healthy unknown states | Dedicated evidence rows, Unconfirmed without workflow | Enable in ESO and reload; exact new row names and unknown meanings; link to complete recording setup | Managed files do not prove enablement/loading/recording |
| H21 | Same, near first-launch and healthy/lost screenshot figures | Historical reference screenshots | Images presented with old labels and freshness row | Nearby help says images use earlier labels/row; current Application Log and data labels, transition logged without row | Existing asset/caption integrity preserved; current behavior clearly qualified |
| H22 | `docs/src/getting-started/troubleshooting.md`, data-addon section | Missing/outdated/unmanaged/folder missing/reload/lifecycle failure/API unknown or unsupported | Compatibility/Ownership labels, "evidence facts", "earlier flush" | New labels, in-game check/reload actions, last-save meaning, selected channel and unsupported API release action | Full package setup/recovery family, no unsupported mutation offered |
| H23 | Same, Encounter capture is waiting, interrupted, or failed | Off/waiting/recording/interrupted/failed, unknown version, invalid supported version | Stopped/Waiting/Capturing/Interrupted/Failed, "flush and import it" as blanket failure recovery | Exact desktop state names, current ESO status versus saved desktop state; off/no-retained-session mode-change guard; optional preservation, deliberate supported clear, unknown-version refusal | Current activity and saved metadata not conflated; no new mandatory preservation gate |
| H24 | Same, desktop history outcomes | Empty/disabled/busy/duplicate/growing/current fight/invalid/conflict/catalog unavailable/loss/unknown/delete failure | Outcome/recovery scattered among implementation references | One explicit recovery list plus exact saved data and desktop-copy effects | Covers progress, readiness, failure and data-quality branches without raw payload exposure |
| H25 | Same, Discovery collector capture or import fails | Combat pause/resume, cancel, new start, reload, wrong/invalid saved input | Cancellation/deletion consequence unstated | Cancel retains incomplete unusable collection; next inactive start replaces prior collection; reload does not reconstruct runtime | Prevents silent data-loss assumptions; maintains existing actual operations |
| H26 | Same, catalog update recovery and shared diagnostic figure/text | Waiting for save/build, cancel/atomic select/fallback/receipt failure | Begin capture wait, Build from flushed capture, Live Log | Watch for saved catalog data, Build from saved catalog data, Application Log; exact atomic cancellation/selection behavior retained | Inline prose, SVG visible label, text equivalent and pinned plain-block hash agree |
| H27 | `docs/src/reference/status-reference.md`, System and State | Every data row and current/unknown/last save | Data Addon names, Unconfirmed (no live channel), flush-bound historical evidence | Exact new labels, Unconfirmed enable/load, collection Unknown (check inside ESO), separate Encounter History summary from last successful import, retained across Refresh/failed imports, Saved channel and in-game next action | Lookup table matches actual `data_addon_view` values; ESO Client Available/Unavailable/Unknown retained and explained as process only |
| H28 | Same, lifecycle API table and controller model | Internal lifecycle states, current and last save | API codes and controller enums | Retained under explicit API/model headings; player-facing labels have explanations in main table | Exact stable machine values help maintainers; not promoted to live desktop certainty |
| H29 | `docs/src/reference/settings.md`, Addons and Game Readings | Live/PTS/folder override, addon enable/load | PixelBeacon and Bus group title, no data-addon setting explanation | Exact new group, Live released game/PTS Public Test Server, correct AddOns path, enable/reload in ESO, package roles | Explains why environment affects both install and import |
| H30 | Same, Application Logging | Log level/file output/off | Logging group, Live Log | Application Logging group, exact Write Log to File, Application Log; no encounter/native-log activation | Keeps levels, sink/filter/persistence semantics, gives correct control names |
| H31 | `docs/src/reference/logging.md`, introduction/Application Log | Diagnostic ring/file, encounter/native confusion | File Logging debug/trace wording and Live Log | Application Log diagnostics, File Logging monthly diagnostic output, neither records fights; native candidate and addon workflow links | Separate logs, capture and history at point of use; exact group/toggle referenced |
| H32 | `docs/src/reference/glossary.md`, API Version/Application Log | Search, old label, API support | Live Log entry under L; single-addon API explanation | Application Log under A, old Live Log retained only as search alias; reviewed support for both addons | Canonical term and alphabet maintained, glossary supplements repaired fronts |
| H33 | `docs/src/development/discovery-collector.md`, introduction/Manage and collect/Remove | Maintainer task, selected channel, active/paused/cancelled/finished/failed/reload/clear/unknown version | Catalog role not contrasted, "cancel to discard the active run" | Definitions versus fights; resume explicitly after combat; cancel retains unusable partial; inactive start replaces prior; clear scope and version refusal | Actual collector behavior documented, no invented runtime recovery |
| H34 | `docs/src/development/catalog-updates.md`, front/Collector-assisted candidate | Trusted candidate selection, save wait/build, start/resume/cancel/clear/uninstall | Capture jargon, Begin capture wait / Build from flushed capture | Catalog definitions explained; complete in-game collector sequence; new exact buttons; cleared subtree and preserved desktop/catalog copies | Existing trust acknowledgement, cancellation, rollback and receipt contracts retained |
| H35 | `docs/src/development/encounter-ingestion.md`, front/qualification | Supported addon path, incremental/terminal native candidate | "provisional preferred bulk-ingestion candidate" first; "supported safe fallback" | Supported addon recording first; incremental and terminal explained; native path provisional and distinct from diagnostics | Historical API controls not misrepresented as proven platform ingestion |
| H36 | `docs/src/development/companion-addon-commands.md`, front | Player versus maintainer authority | Begins with no-go command vocabulary | Player setup link and explanation of instructions/import versus in-game user commands; technical no-ingress decision retained | Static instructions do not imply desktop control |
| H37 | `docs/src/getting-started/responsible-use.md`, Supported boundary/Privacy | Both modes, raw privacy, startup API source | "one bounded local envelope", obsolete API-number bump/fallback description | Bounded one/multiple-fight saved session, last-successful-import summary with Refresh/failure retention and Saved channel, selected Live/PTS reviewed API comparison, truthful unknown/remembered warning | Retains existing privacy and gameplay disclaimer; no new restriction introduced |
| H38 | `docs/src/reference/catalog-sources-and-rights.md`, collector/Implemented source boundaries | Limits, rights, implemented selection, historical closed issues | Selection and rollback "remain issue #118 work"; limits await #129 | Link to implemented confirmed update/rollback, provisional bounds and historical evidence contract | No source/rights boundary weakened; no false backlog or field result |
| H39 | `docs/src/development/architecture.md`, encounter contracts | Current/saved boundary, metrics/prompts, native candidate | Last-saved/historical loose label, issue #190 isolated qualification | Exact Last saved recording state from last successful import with retained summary/Saved channel explanation and native historical-contract qualification; Application Log naming synchronized | Maintainer topology, empty ingress and immutable-storage facts retained |
| H40 | `docs/src/reference/configuration.md`, `docs/src/concepts/scope-and-platform.md`, `docs/src/development/coverage-matrix.md`, `docs/src/features/weaving.md` | Persisted log controls, permission/debug guidance | Live Log naming | Application Log naming | Cross-links and troubleshooting surfaces agree with current menu |
| H41 | `docs/src/features/pixelbeacon.md`, `docs/src/development/catalog-compiler.md`, `docs/src/development/catalog-candidate-pipeline.md` | Independent addon, compiler, review/trust/version/rollback | Existing source-appropriate technical descriptions | Retained with linked repaired collector/update/help pages | Current authority and detailed maintainer paths remain accurate; no extra figure or command is required |
| H42 | `docs/src/README.md`, `docs/src/features/README.md`, `docs/src/SUMMARY.md`, `docs/README.md`, root `README.md` | Help discovery and publication source | Existing links/indexes; root README contains no conflicting encounter/log copy | Retained | Relevant links lead to repaired canonical guides and offline/public publication remains one source |
| H43 | `docs/src/development/test-strategy.md`, `docs/src/reference/external-sources.md`, encounter model/source contract JSON references | Maintainer evidence and source pins | Test/model vocabulary and historical protocol identities | Retained | Named automated guarantees, stable schema/algorithm IDs and pinned primary authorities are necessary; no user setup instruction depends on unexplained terms |

## Figure source contracts and update triggers

The owned `docs/src/assets/diagrams/encounter-evidence-lineage.svg` has six
annotated nodes and five connected orthogonal edges. Its unsupported native-log
candidate is a visibly separate footer with no connection into the supported
flow. `docs/src/reference/encounter-data-and-metrics.md` contains the complete
equivalent, every gate and outcome, source sensitivity, idempotency, exact
hash/channel/API/kind identity, partial/legacy replay, unavailable denominators,
and independently qualified `s069-v1` and `s090-v1` results.

Source authorities and update triggers are recorded in
`docs/project/diagram-rendering-compatibility.md`: Encounter.lua recording,
model/validate/mod/replay import, store identities, metrics/catalog lookup,
recommendation policy and native ingestion reference. Keep the SVG, full text,
content-coverage manifest, source/generated diagram policy and render receipt
matrix synchronized when any authority changes.

The existing static/offline viewer, native dialog, no-script fallback and print
contract are reused. No runtime library, remote figure, game command, schema or
capability is added. `docs/project/documentation-figure-system.md` now counts
23 placements, 22 meaningful images, six flow diagrams and the unchanged 13
captions. The historical visualization audit records #222's delivery separately
from the unchanged #221/#223 approvals.

## Intentional policy updates and automated evidence

- Test-first red: new lineage/gate mutation test fails against the five-diagram
  policy; existing policy cases pass. Six-diagram renderer matrix tests fail
  against the old 40-cell/five-layout inventory. Workflow omission/recovery
  regression fails before its validator is implemented.
- Green owned tests: 157 policy/renderer cases, including missing stages, version
  and channel labels, unresolved IDs, native qualification, missing workflow
  instructions and dedicated 200 percent receipt regressions.
- The updated runtime contract requires 48 paint observations, six direct-SVG
  topology observations and a measured-font 200 percent lineage probe. The
  parent owns the rebuilt full generated-site/browser receipt and parity gate;
  this document does not substitute source checks for that receipt.
- `DIA-007` is intentionally registered in `docs/project/content-coverage.json`.
  The pinned semantic projection changes from
  `e80fd8d1e1c1171b11743053bd44ffde97230d7ba5b53e4f68406204b745f65f`
  to `e701a9027d80cebbb86501f4136ea1fd79fa67fc6cbb75666f2428986863adb1`,
  including the governed Application Log rename and lineage registration.
- The troubleshooting plain-block digest changes from
  `2041b1a45d75c539e4eff851077493e0b0253d4843625e49eb50fefdc90b1ae5`
  to `1050ad5bedb0a28540c41596d97b179287cf98c8cb61422bcf45da7615635ac2`
  solely for the matching Application Log label. No fence, table, screenshot or
  approved brand asset is removed or silently reclassified.
- UTF-8 without BOM and LF are retained; no long dash or mojibake was introduced.
  No field verification was performed or requested.

## Retained history-summary semantic regression

The last saved recording state is the snapshot from the last successfully
imported file, not necessarily the newest disk save or selected environment.
Refresh and failed imports preserve that snapshot. The full guide, interface,
status lookup, troubleshooting, architecture and responsible-use pages now
identify its origin and direct the reader to Saved channel and current in-game
status. The existing workflow mutation test was extended first and failed red
when the origin/retention/channel instructions were absent. Its validator now
requires all three instructions; the S121 docs regressions pass.
