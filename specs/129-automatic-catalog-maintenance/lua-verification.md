# S129 Lua Acceptance Receipt

Date: 2026-10-15. Scope: #286 and the Lua/startup boundary of #285/#291.
The parent owns configuration persistence, settings accessibility, per-root
desktop eligibility/cancellation, full CI and PR/review evidence. This receipt
must not be read as closing those independent desktop criteria.

## Verification state

- Before production edits, the seven automatic collector regressions ran against
  the old manual source: six failed and one passed. The failures were the
  expected absence of automatic completion, deferral/resume, replacement
  preservation, natural-load disable and bounded finalization behavior
  (target/s129-lua-red.log).
- Initial rewritten-source execution passed six of seven tests. The large
  synthetic capture exceeded the original finite simulated-tick test window;
  short-string JSON throughput was improved while retaining the same per-update
  step/byte/time budgets. The supported large-payload test now uses a finite
  100,000 simulated-tick ceiling and asserts every tick's limits.
- Parent integration then passed the original seven collector tests
  (target/s129-focused-integration.log). Additional acceptance/audit cases now
  bring the collector suite to sixteen tests. All sixteen passed against the
  frozen collector source in target/s129-ui-lua-api-final.log (2.20 seconds).
  The same run passed 63 UI and 89 view-model tests; an independent API inventory
  order failure was corrected separately and is not represented as a green run.
- Actual documentation source/diagram policy has zero findings
  (target/s129-docs-source-diagram.log). Render-smoke unit tests passed 17/17
  (target/s129-render-policy-unit.log). These are source/policy tests, not a
  rendered installed-game check.

## Acceptance mapping

| Criterion | Actual behavior and evidence |
| --- | --- |
| Automatic initialization | PixelBeacon Catalog registers one namespace/update and waits for EVENT_PLAYER_ACTIVATED, AreSkillsInitialized and actual source provenance. No collection slash command or desktop start action exists. automatic_load_collects_real_double_facts_and_all_shared_relationships_quietly, automatic_collector_defers_for_readiness_combat_and_loading_then_resumes_once, missing_actual_provenance_defers_instead_of_publishing_a_guessed_identity. |
| Actual environment/version/locale/scope | Strict package-owned StartupConfig supplies actual-root Live/PTS identity; public APIs supply exact game version, API, locale and character scope. Missing source identity defers instead of publishing a guessed identity. The publication boundary rechecks identity after scan/finalization. disabled_startup_is_naturally_applied_without_reading_or_deleting_saved_facts, source_identity_is_revalidated_before_publishing_a_completed_generation. |
| Native independence | Catalog does not consult or change native logging/processing. The harness runs identical Catalog facts in off/on/processing-disabled cases and checks independent update callbacks continue after Catalog failure. Actual supported beacon execution/optional-module failure coverage belongs to the package suite. fresh_initialization_native_states_and_catalog_failure_leave_other_modules_running. |
| Combat/loading/repeated initialization | Combat/inactive player views defer; exit/reactivation resumes automatically. Repeated activation cannot register duplicate collectors. Same-session source identity changes restart automatically. automatic_collector_defers_for_readiness_combat_and_loading_then_resumes_once and automatic_refresh_coalesces_events_and_unchanged_content_keeps_completed_identity. |
| Prior generation and restart | Saved catalog always denotes the prior complete generation until one atomic assignment publishes a complete new table. In-progress records/cursors/chunks stay runtime-only. A natural addon reload performs a bounded automatic restart; saved checkpoint counts are diagnostic, not fake resumable payload. completed_snapshot_survives_interruption_api_failure_and_automatic_restart. |
| Relevant refresh and no noise loop | Documented skill/crafted lock/script/collection/champion/hotbar/full-source events coalesce. New item/effect IDs schedule a refresh; known item/effect stack, durability or duration noise does not repeatedly scan. known_item_and_effect_noise_does_not_rescan_or_request_additional_saves and documented_unlock_events_schedule_refresh_without_a_player_action. |
| Natural save handoff | Changed complete generations request AddOnManager priority save, coalesced to at most one attempt per 30 seconds, including failed attempts. No durability acknowledgement or flush latency is claimed. The desktop consumes stable natural saves separately. The initial actual-envelope test checks the request, not a disk flush. |
| No game actions or routine announcements | Combined Catalog source has no SLASH_COMMANDS, chat route, forced reload, native logger operation, generated binding or input transport. Manifest loads core PixelBeacon before optional data modules. S129 docs-policy asserts these boundaries and all automatic lifecycle tests retain zero normal chat messages. |
| Startup preference | Catalog copies catalog_disabled once at natural addon load. Desktop preferences can change afterward without pretending the already-loaded Lua acknowledged them. Disabled startup preserves previous snapshots, performs no API scan and does not change the native preference. A later natural load applies persisted enablement. disabled_startup_is_naturally_applied_without_reading_or_deleting_saved_facts. |
| Unsupported/malformed data and transient failure | Unknown root/Catalog schema remains intact and inactive, with bounded diagnostics. Current schema malformed prior metadata cannot strand a valid new generation. Cold loads rebuild prior saved envelopes before trusting no-op reuse. Raw callback errors are not exposed. Failures retry after 30 seconds. unknown_saved_versions_and_malformed_config_remain_preserved_inactive, malformed_previous_metadata_cannot_strand_recovery_or_be_reused_as_a_noop, natural_restart_rebuilds_untrusted_prior_array_shapes_before_noop_reuse. |
| Useful categories and relationships | Twelve actual public source groups are emitted, with appropriate bounded/opportunistic/unknown scope. The importer derives localized text and virtual icon references. Shared scripts retain every supported parent. Passive ranks and crafted skills use their supported API views; constant-only scans give an explicit no-useful-definitions retry reason. See coverage.md, automatic_load_collects_real_double_facts_and_all_shared_relationships_quietly, passive_ranks_and_crafted_skill_lines_use_their_supported_api_views, a_constants_only_scan_reports_its_reason_and_retries_without_publishing_empty_game_data. |
| Bounded finalization and failure preservation | The large synthetic capture asserts per-tick steps/bytes, observes sorting/serialization over multiple ticks and then injects a legal source string whose escaped JSON exceeds the record limit. The failed late encoding preserves the old complete generation. scan_sort_serialization_and_checksums_yield_and_preserve_last_good_on_encoding_failure. |

## Every finalization path and its bound

The update loop issues at most 64 bounded steps, at most 4,096 serialized bytes,
and no new step after the observed 4 ms deadline. These are scheduling limits,
not a claim that a game API call, allocator or engine-controlled garbage
collection has a proven field latency.

1. Adapter steps perform a fixed small number of public reads/fact additions.
   Overall source strings/records and duplicate-parent counts have explicit
   caps. No adapter performs a whole-category loop in one step.
2. Bottom-up merge sort copies/compares one record per step. Only each record's
   fixed small metadata-object key set uses table.sort; there is no full-record
   table.sort path.
3. Coroutine JSON serialization yields each small token or at most 512 source
   string bytes. Escaped output and checksum advancement share the 4,096-byte
   update budget. A deferred token remains pending for a later update.
4. Record concatenation and chunk concatenation each operate on at most 64 KiB.
   Adler-32 combines already advanced record checksums in constant work; no
   checksum walks the full saved payload during chunk commit.
5. Coverage construction visits exactly twelve categories. Prior-envelope
   comparison shape-checks fixed metadata, at most twelve coverage rows and at
   most 64 warnings. It never concatenates an old unbounded warnings table.
   Chunk comparison visits one bounded chunk per step.
6. Publication revalidates actual source identity, swaps one completed table and
   releases runtime references. Priority-save scheduling makes one guarded API
   request per allowed interval. No synchronous disk flush or forced UI reload
   is introduced.

Storage limits: 500,000 records, 64 MiB measured source and encoded payload,
64 KiB per source string/encoded record/chunk, 1,024 chunks, 4,096 distinct
parents per entity, at most one saved complete generation, and one bounded
in-progress generation plus typed identity index. The shared desktop parser
separately limits the SavedVariables file to 128 MiB. A resource limit fails the
new generation without deleting the prior one.

## Limits and remaining scope

No known Lua acceptance criterion remains intentionally unimplemented after the
audit repairs. The current sixteen collector tests passed as recorded above.
Full combined-beacon, settings persistence/accessibility, absent-addon eligibility,
desktop stale-work cancellation and stable-file-to-active-Catalog tests remain
separately owned and are summarized in verification.md.

The tests use deterministic public-API doubles and saved-file evidence. They do
not prove game save timing, exhaustive ESO definition visibility, installed
platform behavior or a hard 4 ms bound on engine-owned operations. No owner game
interaction or field-testing issue is required for completion. Concrete source
gaps and retained behavior are listed in coverage.md; unknown IDs remain unknown
until backed by an actual source.
