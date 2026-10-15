# S130 Revised Issue Acceptance Inventory

Owner-approved revision 2026-10-15: normal/separate native log v15. Normal/inline #296 and verbose/separate plus verbose/inline #297 follow separately. All other criteria remain required. All 83 revised criteria are implemented and reconciled against repository evidence in [acceptance-evidence.md](acceptance-evidence.md) and [verification.md](verification.md). Installed-game behavior and deferred formats are not claimed.

## #289

- [x] One-chunk parsing and deterministic arbitrary chunking yield equivalent canonical completed fights and source facts.
- [x] Tests split every meaningful tokenizer/boundary position, including multibyte Unicode, CRLF, quoted commas, escapes and nested groups.
- [x] Partial tail bytes wait for completion without loss, double interpretation or mixing source generations.
- [x] A valid END_COMBAT makes its completed fight available without END_LOG or an inactive source.
- [x] Repeated empty/no-new-byte feeds are idempotent and create no fake interruptions.
- [x] Multiple BEGIN_LOG runs isolate metadata, combat ordinals and clock context.
- [x] Valid checkpoint restart matches uninterrupted parsing.
- [x] Corrupt/incompatible/mismatched checkpoint state is rejected before encounters are produced; recovery follows declared replay rules.
- [x] Normal/separate version 15 has declared grammar and incremental/checkpoint tests. Unsupported verbose/inline layouts remain controlled outcomes without fabricated quantities or a player format-fixing instruction; their implementation is tracked in #296/#297.
- [x] Existing normal/separate version 15 fixtures retain equivalent original evidence and supported observations.
- [x] Unknown types remain retained/countable; malformed known shapes and unsupported versions remain distinct controlled outcomes.
- [x] Unterminated input, nested/orphan combat boundaries, clock reversal and genuinely interrupted fights cannot become fabricated complete fights.
- [x] Recovery uses an explicitly verified boundary and reports incomplete evidence internally/API.
- [x] Copy/reimport, rotation, restart and chunking fixtures prevent accidental duplicates while retaining distinct genuine runs.
- [x] Any native identity/canonical schema change preserves prior rows through compatible reading or transactional migration.
- [x] Limits apply to bounded current work rather than total source age/size.
- [x] Adversarial token/depth/line/context amplification cannot cause unbounded allocation or scanning.
- [x] Limit/error results preserve usable committed position and never silently discard retained source evidence.
- [x] Current damage/healing, separate overflow, local attribution and rate-quality restrictions remain correct.
- [x] The component has no UI, addon-presence, watcher-thread or gameplay-input dependency and emits no chat/UI prompts.


## #290

- [x] Processing starts automatically with desktop runtime when eligible, with no history/interface surface opened.
- [x] Both services remain dormant while combined PixelBeacon is absent, following shared eligibility.
- [x] Native disable stops desktop processing directly/durably; re-enable resumes automatically.
- [x] Deterministic discovery seams cover Windows known-folder/redirected Documents and supported Linux Steam/Proton roots.
- [x] Live/PTS attribution is correct and cannot inherit later UI selection.
- [x] Missing logs/directories, logs appearing after startup and idle periods create no interaction/warning.
- [x] App-closed catchup and live append share the idempotent pipeline.
- [x] Old file existence is never represented as proof of active native logging.
- [x] No game's native logging, format or anonymity setting is changed.
- [x] S130 automatically processes normal/separate version 15 without changing game settings. Verbose/inline remain explicitly unsupported, preserve good history and use controlled internal diagnostics; support follows in #296/#297.
- [x] Partial lines/UTF-8, incomplete fights and delayed flushes preserve parser state.
- [x] Rotation, copy, reopen, replacement and truncation tests prove correct generations/context and no accidental duplicates.
- [x] Checkpoint/fight persistence has a declared atomic boundary; failures on either side preserve exactly-once stored results.
- [x] Source bytes remain unchanged in every success/error journey.
- [x] Historical addon/native records, order, backups and supported calculations remain accessible after UI removal.
- [x] Catalog absence/failure/update does not block native retention or supported calculations.
- [x] No addon recorder is automatically enabled or mirrored.
- [x] Supported history deletion cannot resurrect records during catchup.
- [x] DB contention, disk-full, permissions and parser errors preserve history/retryable progress and never produce false success.
- [x] Disable, addon removal, shutdown and root/environment changes reconcile queued/in-flight work without stale post-stop commit.
- [x] Threads/watchers/file handles close without indefinite shutdown blocking.
- [x] Budgets/backoff prevent repeated whole-file parsing, unbounded catchup memory/queues or blocking input/paint work.
- [x] Internal/API state distinguishes dormant, waiting, processing and recoverable failure without restoring removed notices/status controls.
- [x] No normal/failure journey adds modal/banner/toast/chat/manual-import/repair interaction.


## #292

- [x] File contains neither Catalog Update nor Encounter History, and neither window can be opened by any remaining UI path, accessibility action, test-only interaction helper shipped in production, or stale view state.
- [x] Data Details, the data-only uninstall dialog, catalog modal, recording guide, encounter viewer, source inspection/import UI, and encounter deletion dialogs are removed completely rather than hidden behind another button or disclosure.
- [x] `PixelBeacon` is the visible field name; the removed status fields and their tooltips/accessibility names do not remain in active interface string inventories.
- [x] No normal catalog/encounter path emits Review/Dismiss, Repair Data, manual installation, collection progress, import prompts, repeated notices, or acknowledgement requests, including startup, absence, offline, invalid-input, unsupported-format, update, failure, and recovery states.
- [x] The two disable controls are the only interactive catalog/encounter settings. They use the canonical preferences and eligibility contract from #291, remain visible but inactive without the combined addon, and do not re-enable either service through a stale callback.
- [x] One PixelBeacon Install/Update/Uninstall lifecycle remains for the combined package. Separate data-addon actions and confirmations are gone; ordinary package handling continues through its proven ownership checks.
- [x] Player-facing legacy catalog/encounter commands, startup help, pause/completion messages, command-copy actions, and documentation links embedded in obsolete flows are retired. The combined addon does not introduce recurring catalog/encounter chat output.
- [x] Automatic collection, catalog activation, native ingestion, supported calculations, and external reads continue with all removed windows closed and without any interaction with the new controls.
- [x] Existing saved files and database rows remain unchanged by the interface cleanup. Historical readers and external database access remain available through the retained internal/public data seams.
- [x] Dashboard heights, minimum-size calculation, narrow/wide layouts, System and State expansion, application-log geometry, and tab order no longer reserve empty space for removed rows or depend on their remediation/error text.
- [x] Pointer, keyboard, and assistive-technology navigation expose no orphan controls or unreachable modal focus targets. All supported themes and supported scale factors remain legible without recreating maintenance notices.
- [x] Replace tests that require the deleted UI with meaningful absence and unaffected-behavior coverage. Preserve storage/parser/API tests rather than deleting them because the viewer is gone.
- [x] Update directly affected current interface and command documentation with this implementation. The final whole-corpus audit is tracked by #294.


## #293

- [x] Catalog/encounter processing and supported downstream refresh work with the UI windows permanently absent. No background operation requires an import/detail/refresh/install UI event or selected encounter.
- [x] Worker lifetime and state transitions are driven by the application/service lifecycle, canonical preferences, addon presence, and source generation. Shutdown is bounded, pending work is handled consistently, and stale completions cannot restart disabled work.
- [x] Catalog activation updates every retained consumer to a verified generation. Failed activation preserves the last valid catalog and data; concurrent readers see a coherent old or new generation.
- [x] Catalog-sensitive results contain the correct catalog/source/algorithm identity. Older in-flight results cannot overwrite current-generation derived results or public availability.
- [x] New native observations reach persistent storage and supported processing automatically. Repeated startup, unchanged input, and unchanged catalog generations do not duplicate raw rows or cause uncontrolled rebuild loops.
- [x] Existing supported addon/native store fixtures migrate or open successfully with raw canonical bytes, hashes, identities, loss metadata, ordering, and source kinds preserved. Unsupported/corrupt fixtures remain intact and produce controlled diagnostics.
- [x] Historical addon readers remain usable internally after the player-facing addon recording workflow is removed. No new live addon capture is required to keep existing history readable.
- [x] Supported native facts stay available without a compatible catalog. Missing source evidence and incompatible joins produce qualified unavailable values, with no fabricated API version, callback replay, metric parity, or zero substitution.
- [x] Local HTTP and MCP continue discovering/querying `catalog` and `encounters` after catalog updates, restart, addon removal, either disable setting, and full viewer removal, subject to the existing authentication/read-only contract.
- [x] Public addon/service/capability fields reflect one combined package and the actual persisted/effective state. Any compatibility aliases or schema-version changes have an explicit tested contract and updated field documentation.
- [x] Diagnostics contain no instructions to use removed UI or retired manual workflows, and do not expose raw account/character values, native lines, private paths, or secrets in public receipts or default logs.
- [x] No path in this issue deletes user records, resets a database, broadens read-only API authority, or adds a new user-facing recovery workflow.
- [x] Update affected architecture, storage/capability, API/MCP, and diagnostic documentation as part of implementation. Provide the final contract and compatibility notes to #294.


## #294

- [x] Complete a documented inventory of affected current prose, navigation, help, contracts, policies/tests, screenshots, diagrams, captions/alt text, and generated outputs. Each entry is updated, removed, or explicitly retained for a current compatibility purpose with a concrete reason.
- [x] The active guide describes one PixelBeacon package and the delivered automatic catalog/native processing behavior, including the two independent disable preferences and the absent-addon inactive/off behavior.
- [x] No active instruction sends the user to a removed field, banner, menu, window, dialog, recording command, import workflow, collection/build/install/review flow, or separate data-addon lifecycle.
- [x] The entire Encounter History viewer/menu removal is reflected in navigation, feature/reference pages, screenshots, diagrams, troubleshooting, executable help, and tests. No passive replacement viewer is claimed.
- [x] Descriptions of stored data, history preservation, API/MCP access, source provenance, native capabilities, and unavailable calculations match #293 and do not promise unsupported parity or completed field verification.
- [x] Current repository guidance and machine-readable contracts no longer require the old separate-addon/manual-process architecture. Historical references are clearly historical and do not function as current manuals.
- [x] All affected screenshots are regenerated from the final UI and their manifests, hashes, captions, alt text, capture scenarios, and render expectations agree. No removed rows, buttons, or banners remain visible in active illustrations of the application.
- [x] Changed diagrams and their textual/accessibility equivalents describe the same final process. Existing readability, scaling, keyboard-access, and meaningful-image checks remain effective.
- [x] Documentation policy tests stop requiring obsolete strings and instead catch reintroduction of removed current workflows. Update counts/inventories to the real corpus rather than keeping empty pages or obsolete figures to satisfy old totals.
- [x] Pinned documentation builds, link checks, policy tests, rendering smoke checks, navigation/search checks, and executable-embedded guide checks pass on the final implementation state. Record exact tool versions and actual results.
- [x] A targeted stale-reference scan covers source help/strings, current Markdown/JSON, assets/SVG text, generated HTML/search indexes, and embedded output. Any remaining old identifiers are reviewed as history or required compatibility, not unexplained current workflow text.
- [x] Text and generated deliverables are UTF-8 without BOM with LF endings where applicable, and a sanity check finds no mojibake or corrupted punctuation.
- [x] No new owner field-testing issue, user review/dismiss step, catalog repair task, or manual encounter import requirement is introduced as a documentation fix.
