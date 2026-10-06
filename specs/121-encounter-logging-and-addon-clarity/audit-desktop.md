# Desktop message and state audit

Scope: every player-facing desktop surface for addon setup, encounter recording,
saved-data import, catalog collection, application logging, and the related
settings, confirmations, result quality, and recovery states. Source snapshots
were captured before implementation; the literal annex records exact removed and
added strings with source line anchors. The matrix also accounts for retained
families and the reason they remain valid.

No capture command, import rule, worker operation, package ownership rule, data
schema, or automated-input behavior changed. New commands in desktop text are
instructions for the player to enter inside ESO, never command ingress.

## Surface and state dispositions

| Source family | Previous wording or state | Final wording and disposition | Reason and help alignment |
| --- | --- | --- | --- |
| `src/app/strings.rs`, addon section titles | Data Addon Ownership; Data Addon Compatibility; Data Addon Enabled/Loaded/Reload; Data Runtime; Encounter Collection; Data Addon Next Step | Addon Management; Addon Package Version; Enabled in ESO; Loaded in ESO; Reload Reminder; ESO Client; Encounter Recording; Addon Next Step | Separate installed files, current in-game checks, recording, and catalog collection. `reference/status-reference.md` uses the same rows. |
| Central PixelBeacon status/install/update/remove help | Installation and API boundaries; unmanaged target terminology | Explain PixelBeacon supplies current game state, while ESO Weave Data records encounters and definitions. Unrecognized files are backed up/moved manually before Install. | No new ownership or API-support rule. Current/outdated/absent/unmanaged/missing-folder status and actions retained. `features/pixelbeacon.md`, troubleshooting. |
| Central ESO Weave Data installed/managed/package-version help | Filesystem installation; exact inventory/marker; matching build | Installation means files present. Enable in ESO Add-Ons and `/reloadui`; managed means recognized updateable copy. Package match does not establish loading or game API support. | Current versus saved and package versus game compatibility remain distinct. Setup and status guide. |
| Central enabled/loaded/reload/client help | Current-account source, process availability, lifecycle change | Check Add-Ons, `/ewencounter status` or `/ewcollect status` inside ESO. Reload reminder means changed files may not yet be in use. No reminder does not confirm loading. | Unknown enable/load facts stay unknown. Client running is not collection proof. |
| Central catalog/encounter status help | No live desktop channel; flush-bound historical SavedVariables | Catalog collection gathers definitions; encounter recording records fights. Current status is checked inside ESO; desktop sees last disk save after `/reloadui`, logout, or exit. | Removes transport jargon and explains saved addon data (SavedVariables) in place. |
| Central install/update/repair/remove help | Managed package; atomically replace/reinstall; confirmation | Explain selected environment, Add-Ons enablement, reload, and preservation of saved recordings and imported history. | Atomic implementation stays in maintainer code; actual preservation is player-facing. |
| `src/app/mod.rs::data_addon_view`, no folder | Configure supported AddOns folder | Select Live/PTS and use AddOns Folder Override when detection fails. | Supported next action without inventing a folder chooser. |
| Same, addon absent | Install then reload guidance | Install, enable in ESO Add-Ons, `/reloadui` if open. | Complete setup at point of need. |
| Same, installed current | No lifecycle action required | Files current; check Add-Ons enablement and `/ewencounter status`. | Installed must not imply loaded or collecting. Repair stays optional existing action. |
| Same, outdated | Update or repair managed package then reload if instructed | Update Data/Repair Data install bundled files, enable, `/reloadui`. | Existing action names remain exact. |
| Same, unmanaged | Move/remove unmanaged folder; never modified | Back up and move unrecognized folder out of AddOns before Install. | Explicit ownership boundary, no change in mutation eligibility. |
| Same, inspection unavailable, last-known absent/installed/unmanaged | Last-known status and lifecycle-work instruction | Retain exact last-known state, disable unsafe existing actions, explain selected environment/folder/permissions and retry. | Existing guarded behavior unchanged. |
| Same, enabled/loaded unknown; catalog/recording unknown | Unconfirmed; Unconfirmed (no live channel) | Unconfirmed retained with direct in-game checks; Unknown (check inside ESO) replaces transport wording. | Unknown is not an error or invented current activity. |
| Same, client active/inactive/launcher/unknown | Available/Unavailable/Unknown | Retained, with explicit client-running tooltip. | No conclusion about addon activity inferred. |
| Same, reload required/not required | Required/Not required | Retained with explained reminder; next-step text includes `/reloadui` even after removal. | Display projection only, reminder authority unchanged. |
| `AppModel` inspection/lifecycle generated diagnostics | Could not inspect; last known lifecycle state; unmanaged target | Plain file-check failure and last known installation state; folder/environment/access actions. Unrecognized files not changed/removed, back up before manual action. | Install/update/repair/remove generic failures retain bounded diagnosis, never raw paths or callback values. Already-absent removal remains no further action required. |
| `AppModel::addon_api_line` | Game API and client version; support/current check unknown | Retain exact numeric support/unsupported/pending/historical results; tooltip defines numbered addon interface versus client release, Live/PTS, update/reload, unknown result. | S120 compatibility behavior unchanged. |
| `ui.rs::main_view`, both removal confirmations | Remove addon? | Name managed addon files and exact preserved sibling addon, saved recordings, catalog collection, and imported history. PixelBeacon removal explains current readings stop after reload. | Removal touches addon package only. Wrapping preserves layout containment. |
| `ui.rs::data_addon_details_modal` | Evidence boundary disclaimer and status rows | Explain each addon's purpose and complete installation/enable/reload/current-status setup before distinct rows. | Retain all existing details and API facts. |
| Log title/menu/filter/settings | Live Log; lowest captured event; monthly log file | Application Log; Application Logging. Diagnostic troubleshooting events, severity/filter, saved file-logging choice, explicitly separate from native logs and encounter recordings. | No capture/filter persistence behavior changed. `reference/config-reference.md`. |
| Addon settings group, path and environment | PixelBeacon and Bus; detection-focused help | Addons and Game Readings; selected environment controls both addons and adjacent saved file source; Live regular game and PTS Public Test Server. | Existing settings unchanged. |
| `ui.rs::encounter_history_window`, introduction and setup | Private versioned observations/Combat Metrics disclaimer | Purpose-first imported recordings/results and declared missing events. Expandable, initially visible four-step setup; mode/channel changes require recording off with no retained session. An existing retained session needs deliberate clear, with save/import optional when preserving usable recordings. Mid-combat start is immediate with incomplete opening; outside combat waits; modes, channel, toggle, status, saving/import. | No desktop commands sent and no proof-of-import prerequisite. Native logs and application diagnostics explicitly separate. |
| History source environment | Implicit selected Live/PTS | Saved capture source shown explicitly, change Game Environment in Settings. | Existing source selection only. |
| History Refresh/import/delete controls | Refresh; terminal capture; delete | Existing action labels retained, new exact tooltips distinguish rereading imported history, finished/interrupted saved-file import, and local-only delete. | No new import readiness restriction. |
| History busy and worker missing | Working; Store Unavailable; per-user directory unresolved | Processing imported history/wait; Local History Unavailable; profile/folder-access/restart guidance. | Preserve disabled actions and diagnostic cause. |
| Last saved state | Historical read-only disk evidence; terminal encounters | Last saved recording state in Encounter History; explicit saved snapshot versus current `/ewencounter status`; finished encounters; mode/state/revision explanation. The saved channel is visible separately from the selected import source. | The panel describes a successfully imported saved snapshot. Refresh and failed imports may retain the previous snapshot; changing Game Environment does not automatically read another file. The main/Data Details Encounter Recording row stays Unknown (check inside ESO), even when imported metadata is available. |
| Last saved modes | Single/Continuous | Single fight/Continuous fights, explained stopping/waiting semantics. | Existing mode values unchanged. |
| Last saved controller states | Stopped/Waiting/Capturing/Interrupted/Failed | Recording off/Waiting for combat/Recording combat/Recording interrupted/Stopped after a failure. | Last-saved scope remains visible above. |
| Last saved session state, counts | active/stopped/failed; session ID, interruptions, revision | Retained, surrounding text explains recording session and saved-state revision. | Session lifecycle differs from individual encounter quality; exact IDs retained. |
| Six failure causes | Storage pressure, callback failure, clock reset, terminal reserve exhausted, interruption limit, invalid recovered state | Explain storage limit, unreadable event, changed clock, missing finish space, interruption limit, unreadable saved state. Optional preservation/import before explicit `/ewencounter clear confirm`; catalog/history kept; unknown versions need compatibility instead. | No import proof or new clear prerequisite. Failure codes remain unchanged underneath presentation. |
| Empty history/summary | No local encounters; terminal capture; stored/omitted | Record/save/import steps; Complete observations/Incomplete observations recording; events saved/omitted. | Partial is incomplete observations, not invented complete combat coverage. |
| Selected encounter detail/loading/error | Loading observed metrics; rebuild observed metrics | Loading wording retained; explain recalculation from stored observations and current catalog. Diagnostic headings clarify local history and differing recording/catalog versions. | Parent-owned generated diagnostics are audited separately. |
| One/all encounter deletion confirmation | Immutable local raw records; cannot undo | Imported local original observations/results removed; ESO saved file, addon files, catalog kept; retained source may be imported again. | Irreversible local deletion does not imply destroying every copy. |
| Quality/loss/metrics | Quality labels, sequence ranges, unexplained DPS/HPS/share/uptime | Define completeness, observation gaps, rate/share/uptime meanings. Parent loss explanation helper used consistently. Unknown ID count and preservation remain visible before technical disclosure. | Original observations and unknown IDs retained; no catalog name guessed. |
| Review correction, wrapped loss explanations | The existing fixed-height virtual list clipped explanatory loss labels when they wrapped at narrow width | Projection loss diagnostics use a bounded vertical scroll area that lays out each label at its actual height; no-loss output remains None and the area remains capped at 120 points. | First Codex review found the clipping. `tests/app_encounter_history.rs::s121_wrapped_loss_diagnostics_are_fully_painted_at_minimum_width` covers ten declared gaps at 360 points, including the last label after wheel scrolling. No loss facts, recording behavior, or retention rules changed. |
| Provenance and recommendation citations | Projection and Catalog Provenance wall of schema/version/hash fields | Explained secondary Technical details disclosures preserve every original version, checksum, ID, quality, and citation field. | No raw callback value exposure; numeric schema fields retain technical exactness. |
| Recommendations | Parent-owned statuses and provisional prompt bodies | Render final parent presentation unchanged; source citations secondary and explained. | No thresholds, availability rules, or capabilities changed. |
| Catalog modal introduction/empty/candidate/source details | Hash integrity disclaimer; S073 candidate; schema/source acquisition | Define catalog purpose and preserved history; complete reviewed update folder/help action; file format and checksum/source collection labels. | Maintainer workflow kept, trust checkbox and compatibility checks unchanged. |
| Catalog collection setup/buttons/source absent/clear | Collector lifecycle; Begin capture wait/Build from flushed capture; shared file owner | Installation/setup; Watch for saved catalog data/Build from saved catalog data; exact watch-before-collect command/status/save sequence. Clear explicitly leaves recordings/history/staged files/installed catalog and requires cancel first if active. | Desktop only observes saved file changes. No collection command ingress. |
| Catalog progress stage and total | Locating/validating/normalizing/integrity checking; unknown total | Finding/checking saved catalog data, preparing definitions, checking file integrity; unknown amount explained. | Existing worker stages retained. Parent owns generated progress messages. |
| `ui.rs::drain_catalog_updates`, startup/discovery/recovery | Recovered staging operations; candidate discovery; verified candidates | Unfinished temporary operations cleaned up; catalog update files located/checked, refresh and reviewed-file actions. | All StartupComplete/discovery-failed/empty/nonempty branches covered; parent owns supplied warning bodies. |
| Same, capture boundary and candidate built | Boundary recorded privately; local collector candidate built and verified; S073 verification | Watching a changed saved catalog-data file, in-game collect/complete/save steps; local update built and checked, explicitly not installed. | File boundary is not current activity; no desktop collection command sent. |
| Same, install/rollback complete | Restart safety verified; previous verified catalog restored | Catalog selected now and after restart; previous checked catalog restored. | Actual selection/rollback guarantees retained; previous catalog retained message remains accurate. |
| Same, failed/receipt write failed/cancelled | Diagnostic code first; redacted receipt; stayed safe | Actionable message first, diagnostic code secondary; failed troubleshooting-record save and folder-access action. Cancelled retains prior selection/catalog text. | No invented successful operation on receipt failure; diagnostic identity retained. Worker-supplied CollectorState/Progress messages audited by parent. |
| Catalog Live status variants | Current/new data/ready/collector required/unsupported schema/offline/unavailable | Current/new/ready/offline/unavailable retained as accurate plain outcomes; collection required becomes collect/save instruction; unsupported format names compatible update or ESO Weave upgrade. | PTS stays preview-only and never silently installed as Live. |
| Other relevant retained labels | Install/Update/Uninstall, Install/Update/Repair/Uninstall Data, Data Details, Close Data Details; history window/list/result headings; import/count/busy messages | Retained exact existing names, paired with repaired purpose/action/scope copy. | No unexplained new jargon or false current status; operation names match implemented controls. |

## Validation

- Tests first: `app_strings::s121_addon_and_log_help_explains_roles_saved_state_and_removal`
  failed on missing encounter purpose; `app_view_model::s121_data_addon_next_steps_explain_in_game_enablement_and_saved_data_boundary`
  failed on missing Add-Ons enablement. Both compiled and failed intended assertions.
- Existing view-model/lifecycle fixtures retain ownership/action/reload tests.
- Existing UI sizing tests cover both themes, narrow/wide, increased text size,
  all addon states, and modal painted-text overlap; setup and precise uninstall
  preservation are also asserted through accessible rendered labels.
- First-round review correction: the 360-point, ten-gap layout regression fixture
  fails against the former fixed-height loss list because its last wrapped label
  cannot be fully read after scrolling. The corrected list uses actual text
  heights; the parent verification receipt records the final green run.
- Final integrated Rust/format/docs checks are recorded in the parent verification
  receipt. No live ESO, installed application, or field checks performed.

## Exact literal change annex

The following removed/added literals are source-derived from the captured
baseline versus final files. They complement the state-family dispositions above;
their exact line anchors identify the final implementation and original context.

### src/app/strings.rs

| Disposition | Source anchor | Exact literal |
| --- | --- | --- |
| Previous | `src/app/strings.rs:22` | `"Data Addon Ownership"` |
| Previous | `src/app/strings.rs:23` | `"Data Addon Compatibility"` |
| Previous | `src/app/strings.rs:24` | `"Data Addon Enabled"` |
| Previous | `src/app/strings.rs:25` | `"Data Addon Loaded"` |
| Previous | `src/app/strings.rs:26` | `"Data Addon Reload"` |
| Previous | `src/app/strings.rs:27` | `"Data Runtime"` |
| Previous | `src/app/strings.rs:29` | `"Encounter Collection"` |
| Previous | `src/app/strings.rs:30` | `"Data Addon Next Step"` |
| Previous | `src/app/strings.rs:46` | `"PixelBeacon install state and API compatibility of both bundled companion addons. Open Data Details for the numeric game API, client release, and next action; package installation does not prove current API support or loading."` |
| Previous | `src/app/strings.rs:48` | `"ESO Weave did not modify this unmanaged PixelBeacon target. Move or remove it manually before using Install."` |
| Previous | `src/app/strings.rs:50` | `"Filesystem installation of the managed ESO Weave Data package. This does not prove that ESO enabled or loaded it."` |
| Previous | `src/app/strings.rs:52` | `"Managed means the exact package inventory and ESO Weave ownership marker were verified. Unmanaged targets are never modified."` |
| Previous | `src/app/strings.rs:54` | `"Whether the installed data-addon package matches this ESO Weave build."` |
| Previous | `src/app/strings.rs:56` | `"ESO Weave has no supported current-account source for this fact. Verify ESO Weave Data in ESO's Add-Ons menu."` |
| Previous | `src/app/strings.rs:58` | `"A running ESO process does not prove the addon loaded. Reload ESO after lifecycle changes."` |
| Previous | `src/app/strings.rs:60` | `"Required means run /reloadui or relog before expecting ESO to use the lifecycle change."` |
| Previous | `src/app/strings.rs:62` | `"ESO process availability only. It does not prove the data addon is enabled, loaded, or collecting."` |
| Previous | `src/app/strings.rs:64` | `"Catalog activity has no live desktop channel. SavedVariables evidence is flush-bound and must be treated as historical."` |
| Previous | `src/app/strings.rs:66` | `"Encounter activity has no live desktop channel. SavedVariables evidence is flush-bound and must be treated as historical."` |
| Previous | `src/app/strings.rs:68` | `"The next safe lifecycle step based on the currently available evidence."` |
| Previous | `src/app/strings.rs:70` | `"Open every ESO Weave Data evidence fact with its dedicated explanation."` |
| Previous | `src/app/strings.rs:74` | `"The immutable local game-data catalog version. An unavailable catalog leaves unrelated ESO Weave features working."` |
| Previous | `src/app/strings.rs:110` | `"Install or update the PixelBeacon addon in your AddOns folder."` |
| Previous | `src/app/strings.rs:112` | `"Reinstall the PixelBeacon addon: remove the managed copy and install the current one. Enabled only when the addon is installed."` |
| Previous | `src/app/strings.rs:114` | `"Remove the PixelBeacon addon. Only a folder marked as managed by ESO Weave is deleted."` |
| Previous | `src/app/strings.rs:116` | `"Install the managed ESO Weave Data package in the configured AddOns folder."` |
| Previous | `src/app/strings.rs:118` | `"Atomically replace an outdated managed data addon with the current package."` |
| Previous | `src/app/strings.rs:120` | `"Atomically reinstall the managed data addon without deleting the working copy first."` |
| Previous | `src/app/strings.rs:122` | `"Remove only an ESO Weave-managed data addon after confirmation."` |
| Previous | `src/app/strings.rs:249` | `"Live Log"` |
| Previous | `src/app/strings.rs:250` | `"Recent application events. Drag the divider above to resize."` |
| Previous | `src/app/strings.rs:252` | `"Capture and show events at or above this level. The saved choice also applies to file logging."` |
| Previous | `src/app/strings.rs:266` | `"Import, inspect, and delete private encounter observations stored only on this computer."` |
| Previous | `src/app/strings.rs:270` | `"Show or hide the live log panel."` |
| Previous | `src/app/strings.rs:279` | `"PixelBeacon and Bus"` |
| Previous | `src/app/strings.rs:281` | `"Logging"` |
| Previous | `src/app/strings.rs:369` | `"Use this AddOns folder instead of the auto-detected one. Leave blank to auto-detect."` |
| Previous | `src/app/strings.rs:373` | `"Which ESO install to target when detecting the AddOns folder."` |
| Previous | `src/app/strings.rs:393` | `"The lowest level of event that is captured."` |
| Previous | `src/app/strings.rs:397` | `"Also write captured events to a monthly log file."` |
| Final | `src/app/strings.rs:22` | `"Addon Management"` |
| Final | `src/app/strings.rs:23` | `"Addon Package Version"` |
| Final | `src/app/strings.rs:24` | `"Enabled in ESO"` |
| Final | `src/app/strings.rs:25` | `"Loaded in ESO"` |
| Final | `src/app/strings.rs:26` | `"Reload Reminder"` |
| Final | `src/app/strings.rs:27` | `"ESO Client"` |
| Final | `src/app/strings.rs:29` | `"Encounter Recording"` |
| Final | `src/app/strings.rs:30` | `"Addon Next Step"` |
| Final | `src/app/strings.rs:46` | `"PixelBeacon sends current game state to ESO Weave for weaving and fishing. This row shows installation and game API support, not whether ESO loaded the addon. Open Data Details for both addons' game-version support and next steps."` |
| Final | `src/app/strings.rs:48` | `"This PixelBeacon folder is not a verified ESO Weave-managed copy. Back it up and move it out of AddOns before using Install; ESO Weave will not change it."` |
| Final | `src/app/strings.rs:50` | `"ESO Weave Data records encounters and collects catalog game-data definitions. Installed means its files are present; enable it in ESO's Add-Ons menu and reload before using it. PixelBeacon supplies current gameplay readings separately."` |
| Final | `src/app/strings.rs:52` | `"Managed means ESO Weave recognizes and can update this installed copy. Unmanaged means the folder could not be verified as an ESO Weave-managed copy and will not be changed."` |
| Final | `src/app/strings.rs:54` | `"Whether the installed files match the addon bundled with this ESO Weave version. Game API support is listed separately; matching files do not prove the addon is loaded."` |
| Final | `src/app/strings.rs:56` | `"The desktop cannot read this account's current enablement setting. Check that ESO Weave Data is enabled in ESO's Add-Ons menu."` |
| Final | `src/app/strings.rs:58` | `"The desktop cannot confirm that ESO loaded this addon. Enable ESO Weave Data, run /reloadui, then use /ewencounter status or /ewcollect status inside ESO."` |
| Final | `src/app/strings.rs:60` | `"Required means the addon files changed while ESO may still be running old code. Run /reloadui or log out and back in. No reminder does not confirm that the addon is enabled or loaded."` |
| Final | `src/app/strings.rs:62` | `"Whether the ESO game client is running. This does not tell the desktop whether ESO Weave Data is enabled, loaded, or recording."` |
| Final | `src/app/strings.rs:64` | `"Catalog collection gathers game-data definitions, not combat recordings. Use /ewcollect status inside ESO for current activity. The desktop sees only the last save of ESO's addon data (SavedVariables), written after /reloadui, logout, or exit."` |
| Final | `src/app/strings.rs:66` | `"Use /ewencounter status inside ESO for current recording activity. The desktop sees only the last save of ESO's addon data (SavedVariables), written after /reloadui, logout, or exit. Import that saved capture through File > Encounter History."` |
| Final | `src/app/strings.rs:68` | `"The next installation or setup action supported by the information available. Recording controls are commands entered inside ESO."` |
| Final | `src/app/strings.rs:70` | `"Explain ESO Weave Data setup, game-version support, and which status can be checked only inside ESO."` |
| Final | `src/app/strings.rs:74` | `"The local catalog contains game-data definitions used to interpret recorded ability and effect IDs. It is separate from your recordings and application logs. If unavailable, encounter results may be unavailable while unrelated features keep working."` |
| Final | `src/app/strings.rs:110` | `"Install PixelBeacon in the selected ESO AddOns folder to provide current gameplay readings. Enable it in ESO's Add-Ons menu and run /reloadui if ESO is open."` |
| Final | `src/app/strings.rs:112` | `"Replace the ESO Weave-managed PixelBeacon copy with the bundled version. Enable it in ESO's Add-Ons menu and run /reloadui if ESO is open. Saved recordings and imported history are retained."` |
| Final | `src/app/strings.rs:114` | `"Remove only the ESO Weave-managed PixelBeacon addon files. Current gameplay readings stop after ESO reloads; ESO Weave Data, saved recordings, and imported history are retained."` |
| Final | `src/app/strings.rs:116` | `"Install ESO Weave Data in the selected Live or PTS AddOns folder. Enable it in ESO's Add-Ons menu, then run /reloadui if ESO is already open."` |
| Final | `src/app/strings.rs:118` | `"Replace an outdated ESO Weave-managed addon with this version's bundled copy. Existing saved recordings and imported history are retained. Reload ESO afterward if it is open."` |
| Final | `src/app/strings.rs:120` | `"Reinstall this version's ESO Weave Data files when the managed copy needs repair. Existing saved recordings and imported history are retained. Reload ESO afterward if it is open."` |
| Final | `src/app/strings.rs:122` | `"Remove the ESO Weave-managed addon files. ESO's saved recordings, collected catalog data, and imported history on this computer are retained. Reload ESO afterward if it is open."` |
| Final | `src/app/strings.rs:249` | `"Application Log"` |
| Final | `src/app/strings.rs:250` | `"Recent ESO Weave diagnostic events for troubleshooting, separate from ESO combat logs and encounter recordings. Drag the divider above to resize."` |
| Final | `src/app/strings.rs:252` | `"Record and show application diagnostics at this level or higher. Your saved choice also applies to file logging, not encounter recording or ESO's native combat logs."` |
| Final | `src/app/strings.rs:266` | `"Import saved ESO Weave Data recordings, view their results, and delete imported copies stored on this computer."` |
| Final | `src/app/strings.rs:270` | `"Show or hide ESO Weave troubleshooting events; this does not start or stop encounter recording."` |
| Final | `src/app/strings.rs:279` | `"Addons and Game Readings"` |
| Final | `src/app/strings.rs:281` | `"Application Logging"` |
| Final | `src/app/strings.rs:369` | `"Use this ESO AddOns folder for both companion addons and the saved capture source in its neighboring SavedVariables folder. Leave blank to auto-detect the selected game environment."` |
| Final | `src/app/strings.rs:373` | `"Choose Live (the regular game) or PTS (Public Test Server) for addon installation and the saved capture source. Use the same channel in ESO's recording commands; Live and PTS data are kept distinct."` |
| Final | `src/app/strings.rs:393` | `"The lowest severity of ESO Weave application diagnostics to record and display. This does not change encounter recording or ESO combat logs."` |
| Final | `src/app/strings.rs:397` | `"Also write ESO Weave troubleshooting diagnostics to a monthly log file. Encounter recordings and ESO's native combat-log files are separate."` |

### src/app/mod.rs

| Disposition | Source anchor | Exact literal |
| --- | --- | --- |
| Previous | `src/app/mod.rs:361` | `"Configure a supported ESO AddOns folder in Settings."` |
| Previous | `src/app/mod.rs:369` | `"Install ESO Weave Data, then follow any reload guidance."` |
| Previous | `src/app/mod.rs:377` | `"No lifecycle action is required."` |
| Previous | `src/app/mod.rs:385` | `"Update or repair the managed package, then reload ESO if instructed."` |
| Previous | `src/app/mod.rs:394` | `"Move or remove the unmanaged EsoWeaveData folder manually; ESO Weave will not modify it."` |
| Previous | `src/app/mod.rs:414` | `"Configure a supported ESO AddOns folder before attempting lifecycle work."` |
| Previous | `src/app/mod.rs:430` | `"Unconfirmed (no live channel)"` |
| Previous | `src/app/mod.rs:2133` | `"ESO Weave Data could not be inspected. Verify AddOns folder access before retrying."` |
| Previous | `src/app/mod.rs:2919` | `"ESO Weave Data could not be inspected. The last known lifecycle state is retained; verify AddOns folder access before retrying."` |
| Previous | `src/app/mod.rs:2992` | `"ESO Weave Data is unmanaged. Move or remove the EsoWeaveData folder manually; no files were modified."` |
| Previous | `src/app/mod.rs:3037` | `"ESO Weave Data uninstall was refused because the target is unmanaged. Move or remove only the EsoWeaveData folder manually."` |
| Previous | `src/app/mod.rs:3311` | `"Compatibility of the bundled PixelBeacon and ESO Weave Data packages with published game UI sources. Client release and addon API are separate. Installed packages may still need Update and /reloadui; an unknown check does not confirm support."` |
| Final | `src/app/mod.rs:361` | `"Choose Live or PTS in Settings and set AddOns Folder Override if automatic detection cannot find your ESO AddOns folder."` |
| Final | `src/app/mod.rs:369` | `"Install ESO Weave Data, enable it in ESO's Add-Ons menu, then run /reloadui if ESO is open."` |
| Final | `src/app/mod.rs:377` | `"The addon files are current. Check ESO Weave Data in ESO's Add-Ons menu, then use /ewencounter status inside ESO for recording status."` |
| Final | `src/app/mod.rs:385` | `"Use Update Data or Repair Data to install the bundled files, then run /reloadui if ESO is open. Enable ESO Weave Data in ESO's Add-Ons menu."` |
| Final | `src/app/mod.rs:394` | `"Back up and move the unrecognized EsoWeaveData folder out of AddOns before installing. ESO Weave will not change it."` |
| Final | `src/app/mod.rs:414` | `"Check the selected Live or PTS environment, AddOns folder, and access permissions in Settings before retrying installation or removal."` |
| Final | `src/app/mod.rs:430` | `"Unknown (check inside ESO)"` |
| Final | `src/app/mod.rs:441` | `"Run /reloadui or log out and back in so ESO uses the changed addon files. {remediation}"` |
| Final | `src/app/mod.rs:2138` | `"ESO Weave Data files could not be checked. Check the selected game environment, AddOns folder, and access permissions in Settings, then retry."` |
| Final | `src/app/mod.rs:2924` | `"ESO Weave Data files could not be checked. This is the last known installation state. Check the selected game environment, AddOns folder, and permissions in Settings, then retry."` |
| Final | `src/app/mod.rs:2997` | `"The EsoWeaveData folder could not be recognized as an ESO Weave-managed copy. No files were changed. Back it up and move it out of AddOns before installing."` |
| Final | `src/app/mod.rs:3042` | `"The EsoWeaveData folder could not be recognized as an ESO Weave-managed copy, so it was not removed. Back it up before moving or removing that folder manually."` |
| Final | `src/app/mod.rs:3316` | `"Game API is ESO's numbered addon interface, separate from the client release version. This checks whether the bundled PixelBeacon and ESO Weave Data files support the published API for the selected Live or PTS environment. Update the installed addon files and run /reloadui when needed. Unknown means the check has no current result, not confirmed support."` |

### src/app/ui.rs

| Disposition | Source anchor | Exact literal |
| --- | --- | --- |
| Previous | `src/app/ui.rs:760` | `"Recovered {recovered_staging} interrupted catalog staging operation(s)."` |
| Previous | `src/app/ui.rs:764` | `"Candidate discovery failed: {}"` |
| Previous | `src/app/ui.rs:774` | `"No verified candidates were found in the catalog import folders."` |
| Previous | `src/app/ui.rs:776` | `"Catalog candidate review refreshed."` |
| Previous | `src/app/ui.rs:786` | `"Waiting for user/game. In ESO, run /reloadui, log out, or exit to flush SavedVariables."` |
| Previous | `src/app/ui.rs:790` | `"Capture boundary recorded privately. ESO Weave is not reading in-memory game state."` |
| Previous | `src/app/ui.rs:808` | `"Local collector candidate built and verified. Review it before installation."` |
| Previous | `src/app/ui.rs:812` | `"The local candidate passed S073 verification. Installation remains a separate explicit action."` |
| Previous | `src/app/ui.rs:833` | `"Catalog installed and selected. Restart safety verified."` |
| Previous | `src/app/ui.rs:852` | `"Previous verified catalog restored."` |
| Previous | `src/app/ui.rs:868` | `"{}: {}"` |
| Previous | `src/app/ui.rs:872` | `"receipt-write-failed: The operation stayed safe, but its redacted receipt could not be stored."` |
| Previous | `src/app/ui.rs:1582` | `"Remove the PixelBeacon addon?"` |
| Previous | `src/app/ui.rs:1607` | `"Remove the ESO Weave Data addon?"` |
| Previous | `src/app/ui.rs:2299` | `"Each line has its own evidence boundary. Installation and ESO process state never prove enablement, loading, or collection."` |
| Previous | `src/app/ui.rs:2345` | `"Updates are always user initiated. Imported hashes prove integrity, not who supplied the files."` |
| Previous | `src/app/ui.rs:2363` | `"No verified candidates found. Place a complete S073 candidate under the application data catalog/import/live or catalog/import/pts folder, then refresh."` |
| Previous | `src/app/ui.rs:2407` | `"Catalog {} \| schema {} \| {} sources"` |
| Previous | `src/app/ui.rs:2419` | `"Integrity identity: {}..."` |
| Previous | `src/app/ui.rs:2429` | `"Source acquisition: {}"` |
| Previous | `src/app/ui.rs:2459` | `"Collector-assisted builds are local-only. ESO saves addon data only after /reloadui, logout, or exit. Captures are parsed as restricted data, never executed or uploaded."` |
| Previous | `src/app/ui.rs:2462` | `"Collected categories: player skills, crafted abilities, item sets, champion skills, companions, races, and classes. Account and character names are excluded or represented only by a one-way scope key."` |
| Previous | `src/app/ui.rs:2466` | `"ESO Weave Data lifecycle: {}. Manage it from System and State."` |
| Previous | `src/app/ui.rs:2471` | `"Build from flushed capture"` |
| Previous | `src/app/ui.rs:2473` | `"Begin capture wait"` |
| Previous | `src/app/ui.rs:2497` | `"The collector capture location is unavailable until ESO's AddOns directory is configured."` |
| Previous | `src/app/ui.rs:2509` | `"ESO owns the shared data file. In ESO, use /ewcollect clear confirm to clear only catalog data."` |
| Previous | `src/app/ui.rs:2533` | `"Progress total is not yet known."` |
| Previous | `src/app/ui.rs:2621` | `"Private local observations. Metrics are versioned observations, not complete encounter or Combat Metrics parity claims."` |
| Previous | `src/app/ui.rs:2642` | `"Import the terminal ESO Weave Encounter capture from the selected Live or PTS environment."` |
| Previous | `src/app/ui.rs:2662` | `"Working..."` |
| Previous | `src/app/ui.rs:2668` | `"Store Unavailable"` |
| Previous | `src/app/ui.rs:2670` | `"A per-user application data directory could not be resolved. Encounter history is disabled."` |
| Previous | `src/app/ui.rs:2685` | `"Last saved capture state"` |
| Previous | `src/app/ui.rs:2687` | `"Historical, read-only disk evidence. Use /ewencounter status inside ESO for current state."` |
| Previous | `src/app/ui.rs:2706` | `"Session {} \| {} terminal encounter{} \| {} interruption{}"` |
| Previous | `src/app/ui.rs:2716` | `"Hard failure: {}"` |
| Previous | `src/app/ui.rs:2725` | `"No local encounters. Import a terminal capture to begin."` |
| Previous | `src/app/ui.rs:2767` | `"{} \| {} capture \| {} stored, {} omitted"` |
| Previous | `src/app/ui.rs:2817` | `"Select the encounter again to rebuild observed metrics."` |
| Previous | `src/app/ui.rs:2860` | `"Delete local encounter {}? This removes its immutable raw record and cannot be undone."` |
| Previous | `src/app/ui.rs:2867` | `"Delete every local encounter raw record? This cannot be undone."` |
| Previous | `src/app/ui.rs:3045` | `"Catalog Version Mismatch"` |
| Previous | `src/app/ui.rs:3054` | `"Partial"` |
| Previous | `src/app/ui.rs:3060` | `"Single"` |
| Previous | `src/app/ui.rs:3061` | `"Continuous"` |
| Previous | `src/app/ui.rs:3068` | `"Waiting"` |
| Previous | `src/app/ui.rs:3069` | `"Capturing"` |
| Previous | `src/app/ui.rs:3070` | `"Interrupted"` |
| Previous | `src/app/ui.rs:3077` | `"storage pressure"` |
| Previous | `src/app/ui.rs:3078` | `"callback failure"` |
| Previous | `src/app/ui.rs:3079` | `"clock reset"` |
| Previous | `src/app/ui.rs:3080` | `"terminal reserve exhausted"` |
| Previous | `src/app/ui.rs:3081` | `"interruption limit"` |
| Previous | `src/app/ui.rs:3082` | `"invalid recovered state"` |
| Previous | `src/app/ui.rs:3114` | `"Sequences {}-{} ({})"` |
| Previous | `src/app/ui.rs:3186` | `"Projection and Catalog Provenance"` |
| Previous | `src/app/ui.rs:3257` | `"Quality: {} \| Source Sequences: {}-{}"` |
| Previous | `src/app/ui.rs:3845` | `"Locating sources"` |
| Previous | `src/app/ui.rs:3846` | `"Waiting for capture"` |
| Previous | `src/app/ui.rs:3847` | `"Validating"` |
| Previous | `src/app/ui.rs:3848` | `"Normalizing"` |
| Previous | `src/app/ui.rs:3851` | `"Integrity checking"` |
| Previous | `src/app/ui.rs:3870` | `"A verified Live catalog candidate is ready for review."` |
| Previous | `src/app/ui.rs:3873` | `"A local collector capture is required before building this update."` |
| Previous | `src/app/ui.rs:3876` | `"An imported candidate uses an unsupported catalog schema."` |
| Final | `src/app/ui.rs:760` | `"Cleaned up {recovered_staging} unfinished temporary catalog operation(s) from an earlier run."` |
| Final | `src/app/ui.rs:765` | `"Could not find catalog update files: {}"` |
| Final | `src/app/ui.rs:777` | `"No checked catalog update files were found in the catalog import folders. Add a complete reviewed candidate and choose Refresh candidates."` |
| Final | `src/app/ui.rs:779` | `"The list of checked catalog update files has been refreshed. Review a candidate before installing it."` |
| Final | `src/app/ui.rs:789` | `"Waiting for ESO to save addon data. After collecting inside ESO, run /reloadui, log out, or exit, then build from the saved file."` |
| Final | `src/app/ui.rs:793` | `"ESO Weave is watching for a changed saved catalog-data file. Start collection inside ESO, wait for completion, then save with /reloadui, logout, or exit. No current game activity is read by this workflow."` |
| Final | `src/app/ui.rs:811` | `"A local catalog update was built and its files checked. Review it before installation."` |
| Final | `src/app/ui.rs:815` | `"The local catalog update passed file and format checks. It has not been installed; review it and choose Install selected Live catalog when appropriate."` |
| Final | `src/app/ui.rs:836` | `"The catalog is installed and selected for use, including after ESO Weave restarts."` |
| Final | `src/app/ui.rs:855` | `"The previously checked catalog has been restored and selected."` |
| Final | `src/app/ui.rs:872` | `"{} (diagnostic code: {})"` |
| Final | `src/app/ui.rs:878` | `"The catalog operation's troubleshooting record could not be saved. Check application data-folder access. This does not mean the selected catalog was lost (diagnostic code: receipt-write-failed)."` |
| Final | `src/app/ui.rs:1588` | `"Remove the managed PixelBeacon addon files? Current gameplay readings will stop after ESO reloads. ESO Weave Data, saved recordings, and imported encounter history are kept."` |
| Final | `src/app/ui.rs:1613` | `"Remove the managed ESO Weave Data addon files? ESO's saved recordings and collected catalog data, imported encounter history, and PixelBeacon are kept. Reload ESO if it is open."` |
| Final | `src/app/ui.rs:2305` | `"ESO Weave Data records encounters and collects game-data definitions. PixelBeacon separately supplies current gameplay readings. Installed files do not confirm that ESO enabled or loaded either addon."` |
| Final | `src/app/ui.rs:2307` | `"Setup: install ESO Weave Data for the selected Live or PTS environment, enable it in ESO's Add-Ons menu, and run /reloadui if ESO is open. For current recording status, enter /ewencounter status inside ESO."` |
| Final | `src/app/ui.rs:2352` | `"A catalog contains game-data definitions used to identify recorded abilities and effects. Updating it does not start encounter recording or delete history. You choose when to install a reviewed update; file checks detect changes but do not identify its supplier."` |
| Final | `src/app/ui.rs:2370` | `"No checked catalog update files found. Place a complete reviewed catalog candidate in the application data catalog/import/live or catalog/import/pts folder, then choose Refresh candidates. See the offline guide's Catalog Update workflow for the required files."` |
| Final | `src/app/ui.rs:2414` | `"Catalog {} \| file format version {} \| {} data sources"` |
| Final | `src/app/ui.rs:2426` | `"File identity (SHA-256 checksum): {}..."` |
| Final | `src/app/ui.rs:2436` | `"How source data was collected: {}"` |
| Final | `src/app/ui.rs:2466` | `"For maintainers: ESO Weave Data can collect game definitions inside ESO to build a catalog update locally. This is separate from encounter recording. Saved addon files are read as data, never run as code or uploaded."` |
| Final | `src/app/ui.rs:2468` | `"Choose Watch for saved catalog data before collecting. Inside ESO, enter /ewcollect start live or /ewcollect start pts for the selected environment. Use /ewcollect status for progress; after collection completes, run /reloadui, log out, or exit ESO, then choose Build from saved catalog data. These desktop buttons do not send commands to ESO."` |
| Final | `src/app/ui.rs:2470` | `"Collected categories: player skills, crafted abilities, item sets, champion skills, companions, races, and classes. Account and character names are not stored; any grouping identifier is a one-way value instead of the names themselves."` |
| Final | `src/app/ui.rs:2474` | `"ESO Weave Data installation: {}. Install, update, or repair it from System and State; enable it in ESO's Add-Ons menu."` |
| Final | `src/app/ui.rs:2479` | `"Build from saved catalog data"` |
| Final | `src/app/ui.rs:2481` | `"Watch for saved catalog data"` |
| Final | `src/app/ui.rs:2485` | `"Watch for changes to ESO's saved catalog collection, or build a local update from changed saved data. This does not start collection inside ESO."` |
| Final | `src/app/ui.rs:2506` | `"The saved catalog-data file cannot be located. Choose the correct Live or PTS environment and AddOns folder in Settings, then retry."` |
| Final | `src/app/ui.rs:2518` | `"Inside ESO, use /ewcollect clear confirm to delete only the addon's catalog collection data, then /reloadui, logout, or exit to save that deletion. Encounter recordings, imported history, staged update files, and installed catalogs are kept. Cancel a running or paused collection first."` |
| Final | `src/app/ui.rs:2542` | `"The amount of catalog data to process is not yet known."` |
| Final | `src/app/ui.rs:2630` | `"Import recordings made by ESO Weave Data and view results stored on this computer. Results cover only the recorded observations; missing events can leave them incomplete."` |
| Final | `src/app/ui.rs:2633` | `"Saved capture source: {}. Change Game Environment in Settings to select Live or PTS (Public Test Server)."` |
| Final | `src/app/ui.rs:2634` | `"How to record and import"` |
| Final | `src/app/ui.rs:2637` | `"1. Install ESO Weave Data, enable it in ESO's Add-Ons menu, and run /reloadui if ESO is already open."` |
| Final | `src/app/ui.rs:2638` | `"2. Inside ESO, while recording is off with no retained session, choose /ewencounter mode single (one fight) or /ewencounter mode continuous (multiple fights), then /ewencounter channel live or /ewencounter channel pts to match the game you are using. To change an existing session's mode or environment, save and import first if you want to keep it, then /ewencounter clear confirm to delete only the addon recording copy."` |
| Final | `src/app/ui.rs:2639` | `"3. Enter /ewencounter toggle to start. Outside combat it waits for the next fight; during combat it starts immediately with an incomplete opening. Single fight mode stops after that fight; continuous mode waits between fights. Enter /ewencounter status for current activity, and /ewencounter toggle again to stop while retaining recordings."` |
| Final | `src/app/ui.rs:2640` | `"4. Run /reloadui, log out, or exit ESO to save addon data (SavedVariables) to disk. Return here and choose Import Saved Capture. Refresh only rereads imported history; it does not import new ESO data."` |
| Final | `src/app/ui.rs:2641` | `"ESO's native combat logs and ESO Weave's application diagnostics are separate. This window imports the addon's saved recordings."` |
| Final | `src/app/ui.rs:2650` | `"Reread the encounter copies already imported on this computer. Use Import Saved Capture to read newly saved ESO data."` |
| Final | `src/app/ui.rs:2663` | `"Import finished or interrupted encounters from ESO Weave Data's last saved file for the selected Live or PTS environment. Stop recording and run /reloadui, log out, or exit ESO first when you need to save new data."` |
| Final | `src/app/ui.rs:2675` | `"Delete imported encounter copies on this computer after confirmation. ESO's saved recording file is kept."` |
| Final | `src/app/ui.rs:2684` | `"Processing imported history. Wait before starting another operation."` |
| Final | `src/app/ui.rs:2690` | `"Local History Unavailable"` |
| Final | `src/app/ui.rs:2692` | `"ESO Weave could not find this user's application data folder, so it cannot open local encounter history. Check your user profile and application-folder access, then restart ESO Weave."` |
| Final | `src/app/ui.rs:2707` | `"Last saved recording state"` |
| Final | `src/app/ui.rs:2709` | `"This is the addon's state from the last successfully imported saved file, not current activity. Refresh and failed imports keep this older summary; check Saved channel below. Enter /ewencounter status inside ESO for current recording status."` |
| Final | `src/app/ui.rs:2717` | `"Single fight mode stops after one encounter; continuous fights mode waits between encounters until you stop it. Interrupted means recording was broken by a reload or similar interruption. The revision counts saved state changes."` |
| Final | `src/app/ui.rs:2729` | `"Session {} \| {} finished encounter{} \| {} interruption{}"` |
| Final | `src/app/ui.rs:2739` | `"Recording stopped after a failure: {}"` |
| Final | `src/app/ui.rs:2742` | `"To keep usable retained recordings, save and import them first. When you choose to discard the addon's failed encounter data, enter /ewencounter clear confirm inside ESO. This keeps catalog collection and imported history. Then use /ewencounter toggle to start a new session. Unknown-version data may need a compatible ESO Weave version instead."` |
| Final | `src/app/ui.rs:2749` | `"No imported encounters yet. Record inside ESO, save addon data with /reloadui, logout, or exit, then choose Import Saved Capture."` |
| Final | `src/app/ui.rs:2791` | `"{} \| {} recording \| {} events saved, {} events omitted"` |
| Final | `src/app/ui.rs:2818` | `"Delete this imported encounter copy. ESO's saved recording file is kept."` |
| Final | `src/app/ui.rs:2842` | `"Select the encounter again to recalculate its results from the saved observations and current catalog definitions."` |
| Final | `src/app/ui.rs:2885` | `"Delete imported encounter {} from this computer? Its original stored observations and calculated results will be removed. This local deletion cannot be undone. ESO's saved recording file, addon files, and catalog definitions are kept; a retained recording can be imported again."` |
| Final | `src/app/ui.rs:2892` | `"Delete all imported encounters from this computer, including their original stored observations and calculated results? This local deletion cannot be undone. ESO's saved recording file, addon files, and catalog definitions are kept; retained recordings can be imported again."` |
| Final | `src/app/ui.rs:3070` | `"Recording and Catalog Versions Differ"` |
| Final | `src/app/ui.rs:3078` | `"Complete observations"` |
| Final | `src/app/ui.rs:3079` | `"Incomplete observations"` |
| Final | `src/app/ui.rs:3085` | `"Single fight"` |
| Final | `src/app/ui.rs:3086` | `"Continuous fights"` |
| Final | `src/app/ui.rs:3092` | `"Recording off"` |
| Final | `src/app/ui.rs:3093` | `"Waiting for combat"` |
| Final | `src/app/ui.rs:3094` | `"Recording combat"` |
| Final | `src/app/ui.rs:3095` | `"Recording interrupted"` |
| Final | `src/app/ui.rs:3096` | `"Stopped after a failure"` |
| Final | `src/app/ui.rs:3102` | `"recording reached its storage limit"` |
| Final | `src/app/ui.rs:3103` | `"a game event could not be recorded"` |
| Final | `src/app/ui.rs:3104` | `"the game clock changed during recording"` |
| Final | `src/app/ui.rs:3106` | `"no reserved space remained to finish recording"` |
| Final | `src/app/ui.rs:3108` | `"recording reached its interruption limit"` |
| Final | `src/app/ui.rs:3109` | `"the saved recording state could not be read"` |
| Final | `src/app/ui.rs:3135` | `"Complete means no declared gaps in this recording, not every possible combat event. Incomplete observations can produce partial results. Unknown ability or effect IDs remain unresolved until matching catalog definitions are available."` |
| Final | `src/app/ui.rs:3152` | `"DPS is recorded outgoing damage per second. Effective HPS is recorded effective healing per second. Ability damage share is each ability's portion of recorded damage; effect uptime is its recorded active time. These results describe the saved observations, not a complete Combat Metrics report."` |
| Final | `src/app/ui.rs:3218` | `"Unresolved recorded IDs: {}. Matching catalog definitions are needed to interpret these IDs; original observations are retained."` |
| Final | `src/app/ui.rs:3221` | `"Technical details: calculation versions and source identity"` |
| Final | `src/app/ui.rs:3223` | `"These versions identify the calculation format and rules, the matching game-data catalog, and the original stored observations. SHA-256 checksums identify the exact contents; they do not prove who supplied them."` |
| Final | `src/app/ui.rs:3285` | `"Technical details: recommendation source"` |
| Final | `src/app/ui.rs:3288` | `"These recording IDs, checksums, and calculation versions identify the observations behind this review prompt."` |
| Final | `src/app/ui.rs:3300` | `"Quality: {} \| Recorded observation numbers: {}-{}"` |
| Final | `src/app/ui.rs:3888` | `"Finding saved catalog data"` |
| Final | `src/app/ui.rs:3889` | `"Waiting for ESO to save catalog data"` |
| Final | `src/app/ui.rs:3890` | `"Checking saved catalog data"` |
| Final | `src/app/ui.rs:3891` | `"Preparing game-data definitions"` |
| Final | `src/app/ui.rs:3894` | `"Checking catalog file integrity"` |
| Final | `src/app/ui.rs:3913` | `"A Live catalog update has passed file checks and is ready for review."` |
| Final | `src/app/ui.rs:3916` | `"Collect game definitions with ESO Weave Data inside ESO, then save addon data before building this catalog update."` |
| Final | `src/app/ui.rs:3919` | `"The update files use a catalog format this ESO Weave version cannot read. Obtain a compatible reviewed update or update ESO Weave."` |

