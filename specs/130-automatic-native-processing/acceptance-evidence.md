# S130 acceptance evidence index

Date: 2026-10-15. Criteria are numbered in their listed order within each issue
in issue-acceptance.md. All 83 criteria are covered below. This index identifies
the evidence; the final gate results and delivery state live in verification.md.
Repository fixtures and code inspection establish these claims within the tested
environment. They do not establish installed-game behavior or deferred formats.

| Issue / criteria | Concrete evidence |
| --- | --- |
| 289 / 1-5 | encounter_incremental tests every_byte_partition, completed_fight_is_returned_before_end_log, checkpoint_after_each_byte, pending_tail and split_initial_bom exercise exact byte partitions, quoted/nested content, UTF-8, CRLF, early completion and empty feeds. |
| 289 / 6-10 | Multiple-run/extracted-run identity, every-byte checkpoint restoration, corrupt/version/channel/state rejection, deferred_serialization, representative_fixture, retained encounter_native suite and the independent schema1 baseline hash in parser-verification.md. |
| 289 / 11-13 | chunked_unknown_evidence, reversed_endpoints, malformed_encoding_nested_boundaries, explicit_generation_termination and context_and_pending_fight_limits prove controlled recovery and incomplete accounting. |
| 289 / 14-15 | encounter_ingest alias/copy/migration/backup regressions, native_maintenance catchup/append/restart/copy/truncation, and retained native schema1 compatibility tests preserve original evidence and distinct runs. |
| 289 / 16-18 | Actual >256 MiB/>2 million-record and >10000-run streams; feed/tail/token/depth/context/fight/checkpoint bounds and corrupt checkpoint tests. See parser-verification.md for exact work limits. |
| 289 / 19-20 | Retained native projection tests and chunk-invariance projection comparison; pure parser dependencies in native/incremental.rs contain no UI, game input, package inspection or scheduler. |
| 290 / 1-3 | native_maintenance catchup test runs a service without a viewer or Catalog, pauses on direct disable, and resumes existing appended bytes; data_maintenance preference persistence and failed-disable/enable tests remain. |
| 290 / 4-6 | Windows known-folder/path-override tests remain; new multi-Steam-root/library test and late_logs_cross_environment cover immutable Live/PTS roots, late Logs creation and no empty database at startup. |
| 290 / 7-10 | catchup/append/restart/copy test, logging_activity unknown assertion, controlled unsupported profile tests and read-only source handles; no game logging/format/anonymity mutation exists. #296/#297 track deferred profiles. |
| 290 / 11-14 | Every-byte checkpoint parser tests; atomic ingest/CAS/omitted admission/checkpoint-write-failure tests; generation terminal archive/replacement tests; runtime source bytes compared before/after. |
| 290 / 15-18 | Mixed-history/old-schema migration/backup/read tests, missing-Catalog background derivation then activation, and durable native deletion suppression across copies/backups. No live addon recorder is enabled. |
| 290 / 19 | SQLite busy and injected checkpoint persistence failure demonstrate rollback, unchanged progress and retry. Runtime retains the last checkpoint on any source/store error and backs off. OS disk-full and permission failures use these same error paths; no physical disk-exhaustion or host ACL-change experiment is claimed. |
| 290 / 20-22 | Root/token admission races and completed-disable serialization; peer removal followed by successful peer derivation; native-only shutdown/restart eligibility; bounded Drop, read turns, directory cursor/quota fairness, cache and retry bounds in native.rs. |
| 290 / 23-24 | NativeServiceSnapshot exposes four phases with controlled diagnostics; app state publishes actual worker observations. UI absence matrix covers failure/startup states without notices or manual ingress. |
| 292 / 1-4 | File menu/accessibility absence tests, app_strings retired-string assertions, complete deletion of encounter_history.rs and Catalog presentation worker, and UI matrix over package states/themes/scales. |
| 292 / 5-7 | Two canonical disable controls, absent-package inactive matrix, stale-callback preference tests, one combined lifecycle and confirmation, foreign/unexpected-inventory preservation tests, retained Lua command-retirement tests. |
| 292 / 8-9 | Independent native service, existing automatic Catalog runtime, persistent EncounterProcessor, mixed-history migration/backup tests and authenticated read-only database tests. |
| 292 / 10-12 | Dashboard sizing, narrow/wide/breakpoint/expanded-state/log geometry, scale/theme/accessibility/contrast/tab-navigation tests; obsolete dialog tests transferred to lifecycle and domain assertions. |
| 292 / 13 | surface-audit.md, current guide changes, generated/search absence policy and embedded guide tests. |
| 293 / 1-2 | Native and Catalog runtimes own scheduling; bounded store processor has no selected encounter or viewer. Runtime/cancellation/disable/shutdown tests exercise direct lifecycle control. |
| 293 / 3-4 | Catalog runtime and database_query catalog-replacement tests preserve coherent readers; process_pending captures real Catalog context and admits derivations only under matching generation; encounter_ingest activation/restart test verifies receipts. |
| 293 / 5-7 | Incremental ingest atomic/idempotent/copy tests, stable-context pending cursor tests, old schema1-6 migration/read suites, historical addon projection/recommendation fixtures and retained Lua parser tests. |
| 293 / 8-9 | Missing-Catalog native facts and qualified addon unavailability tests; database_query, local_service HTTP/MCP parity/auth/cancellation/restart tests retain fixed authorities independent of collection state. No new API write authority is added. |
| 293 / 10-11 | Additive actual worker observation, documented data alias, maintained state field inventory and combined lifecycle alias tests; controlled domain diagnostics and current-source stale scan. |
| 293 / 12-13 | No background delete/reset operation; strict corrupt-store rejection and source preservation tests; affected architecture, encounter-model, API/MCP and storage contracts audited in surface-audit.md. |
| 294 / 1-6 | Complete disposition inventory in surface-audit.md; guide, help, current governance, machine contracts, navigation and generated/search policy assertions for the delivered one-package automatic workflow. Historical archives retain explicit historical status. |
| 294 / 7-9 | 28 source-built capture variants and seven curated PNGs, hashes/manifests/alt text, updated SVG equivalents, unchanged baseline readability gates and new workflow-reintroduction tests. Exact results are in surface-verification.md. |
| 294 / 10-11 | Pinned book/link/spelling/policy/browser matrices and release-embedded guide tests; source/current Markdown+JSON/SVG/generated HTML/search/embedded stale scans in surface-verification.md. |
| 294 / 12-13 | Final changed-file UTF-8/no-BOM/LF/mojibake scan and whitespace gate; no owner field-testing or manual repair/import follow-up was created. #296/#297 contain implementation work only. |

The only scope deviation is the owner's explicit normal/separate v15 decision.
No inline/verbose fidelity, runtime field verification, release or owner merge is
claimed by this evidence index.
