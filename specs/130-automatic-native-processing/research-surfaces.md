# S130 surface and documentation research

Research date: 2026-10-15. Implementation baseline: S129 after PR #295.
This inventory is research evidence, not implementation or installed-game proof.

## Presentation retirement and authority transfer

`src/app/ui.rs::EsoWeaveApp` still owns the complete legacy Catalog and Encounter
History presentation state. S129 removed production Catalog worker attachment
and its visible menu entry, but the production type still exports
`with_catalog_updates`, `set_catalog_update_open`, `catalog_update_open`,
`set_data_addon_details_open`, `set_confirm_data_uninstall`,
`with_encounter_history`, `set_native_encounter_source`, and history-count and
selection helpers. Remove obsolete production entry points as well as rendering.

The remaining File menu opens Encounter History. `frame_ui` still drains catalog
and history workers, requests saved-source inspection, and renders Catalog Update,
Data Details, Encounter History, and encounter deletion modals when their state
flags are set. Named removal seams are `drain_catalog_updates`,
`refresh_catalog_availability`, `choose_default_catalog_candidate`,
`encounter_source_ui`, `saved_source_ui`, `request_saved_source_inspection`,
`drain_encounter_history`, `request_history_refresh`,
`request_catalog_changed_history_detail`, `data_addon_details_modal`,
`catalog_update_modal`, `encounter_history_window`, `request_history_import`, and
`encounter_delete_confirmation`. Remove associated render helpers from
`recording_command` through `should_accept_history_detail` when unused.

Preserve the `frame_ui` maintenance activation handoff. Its
`install_active_catalog` currently updates the canonical query path, history
service catalog path, and model Catalog, then invalidates selected UI details.
Replace only the UI detail refresh responsibility. S129 already shares a cloned
history service and database-query service with `MaintenanceRuntime` in
`src/main.rs`; these are the existing background authority seams.

`src/app/encounter_history.rs` is an entirely UI-oriented worker/presentation
module. Its commands include source inspection, manual native/saved import,
refresh, detail loading, and deletion. `EncounterDetail::from_projection` calls
`generate_recommendations`; required recommendation computation cannot disappear
with this module. Move durable/background consumer ownership first. Retain the
underlying `src/encounter/history.rs` service, parser, storage, native readers,
metric/recommendation modules, and useful maintainer CLI utilities.

`src/main.rs` constructs `EncounterHistoryService` independently from the GUI and
passes it to `MaintenanceRuntime::spawn_with_history`, then separately calls
`app.with_encounter_history(service)`. Remove the GUI attachment, retaining the
service connection to background work.

## Model, strings, and layout

`src/app/mod.rs` retains `DataAddonPrimaryAction`, `DataAddonView`,
`data_addon_view`, `status_line_catalog`, data-only `UiIntent` variants, and
`AppView::{data_addon,catalog_line}`. Their current strings still describe two
addons, slash commands, required reloads, and manual import. Delete presentation
projections and stale data-only intent ingress after validating canonical
maintenance/API observations do not rely on them. Preserve package ownership and
compatibility facts in their existing lower-level authorities.

`src/app/strings.rs` still calls the field `PixelBeacon Status`, sends users to
Data Details and `/ewencounter` or `/ewcollect`, and exposes Encounter History,
Catalog, and separate Data labels/tooltips in centralized inventories. Change the
field to `PixelBeacon`, remove retired strings and inventory entries.

`src/app/ui.rs::SYSTEM_STATE_STATUS_TITLES` and System and State rendering still
include Catalog. `DATA_DETAILS_STATUS_TITLES` plus data lifecycle button-width
constants and several dashboard height calculations survive from old rows.
Recompute row/card geometry from retained controls; preserve the two disable
checkboxes in settings and their maintenance eligibility authority.

## Test migration

- `tests/app_encounter_history.rs` mixes historical storage/worker tests with
  rendered viewer tests. Move meaningful source integrity, source-kind dispatch,
  catalog replacement, retained sessions, batch import, and recommendation
  calculation assertions into service/consumer suites before removing the UI
  worker. Viewer, copy-command, source-selection, and deletion-dialog tests should
  become absence and unaffected-state checks, not a replacement viewer.
- `tests/app_ui_sizing.rs` has retained manual-modal helpers around lines 2272+
  and tests for Data Details, catalog trust acknowledgement, install progress,
  copied commands, and stale candidates. Remove obsolete UI-specific cases and
  production helpers. Extend the existing S129 single-lifecycle and menu checks
  to both retired destinations, Catalog row, PixelBeacon label, all themes,
  narrow/wide sizes, short settings, focus navigation, collapse, and log geometry.
- `tests/app_view_model.rs` retains separate Data projections and data-only
  lifecycle intents around lines 2023+. Preserve their filesystem ownership and
  stale-intent safety evidence by moving assertions to the retained combined
  lifecycle tests where needed. Existing optout persistence and absence checks
  near the beginning remain relevant.
- `tests/app_strings.rs` must require PixelBeacon and forbid retired inventories.
- `tests/support/documentation_capture.rs::validate_scene_view` and
  `validate_healthy_view` currently inspect `view.catalog_line` and
  `view.data_addon`; replace with retained canonical authorities and assert
  forbidden rendered labels are absent. Do not weaken no-input checks.

## Current corpus audit

Direct stale-string scan found current references in getting-started
`first-launch.md` and `troubleshooting.md`; features `interface.md`,
`pixelbeacon.md`, `encounter-capture.md`; development `encounter-ingestion.md`
and `companion-addon-commands.md`; reference `logging.md`, `status-reference.md`,
`glossary.md`, and `encounter-data-and-metrics.md`; and
`docs/project/content-coverage.json`. Issue #294 also requires auditing otherwise
nonmatching architecture, automatic Catalog, settings, API, rights, release,
navigation, README, current guidance, source/model JSON, and visualization
inventories for semantic drift.

The production `addon/PixelBeacon/Catalog.lua` already has no slash-command
registration or chat workflow. The shipped bootstrap does not load the recorder.
Legacy `/ewcollect` and `/ewencounter` examples remain in historical test fixtures
and tests. Preserve fixtures as historical parser/harness evidence, with explicit
compatibility context. `/pbquickslot` is unrelated and must remain.

`src/bin/catalog-compiler.rs::usage` retains developer import/inspect/backup/delete
and collector lifecycle commands. These can remain useful internal tools; ensure
docs label them contributor utilities and never require them for player operation.

`docs/src/reference/local-extension-state-fields.json` documents
`application.data_maintenance` as eligibility without a running worker. Update
that meaning to the actual delivered native worker facts; preserve
`application.addons.data` only as an explained compatibility observation of the
combined package.

## Policy, diagrams, and frozen obligations

`.github/scripts/docs-policy.mjs::validateEncounterWorkflowDocs` (around 1655)
literally requires manual source inspection/import, recording commands, refresh,
and delete instructions across twelve active pages. Replace these requirements
and mirrored unit fixtures with positive automatic-service/disable-control/data
access requirements and negative checks for removed current workflows.

`SEARCH_TARGETS` explicitly requires `PixelBeacon Status`. Its corresponding
content-coverage search alias and glossary target must change together.
`DOCUMENTATION_DIAGRAMS`, `validateDocumentationDiagrams`, policy test
`diagramRecords`/labels, and `.github/scripts/docs-render-smoke.mjs::DIAGRAMS`
all pin encounter lineage and troubleshooting alt text, headings, visible labels,
and text equivalents. Native completed-file import and addon-only lineage must
be replaced consistently. Keep SVG accessibility, connector metadata, fallback
font, zoom, no-script, image containment, theme, and layout checks effective.
Audit the Catalog and authority diagrams too, retaining already-correct S129
automatic activation and independent Live/PTS boundaries.

`contentContractDigest` freezes evidence, obligations, page anchors, search map,
and diagram semantic projections from `docs/project/content-coverage.json`.
The ledger preservation digest freezes twenty source-unit destinations/excerpts
and six safety crosswalk records from `docs/project/migration-ledger.json`.
Record exactly changed current excerpts/evidence/search/diagram facts and update
those semantic digests deliberately if required. Preserve migration baseline
commit IDs, baseline blob identities, obligation coverage, and unrelated safety
requirements. Do not recalculate away unexplained drift.

## Capture and build workflow

There are seven deterministic scenes, two themes, and two viewports (28 PNGs).
Generate through `cargo test --locked --test documentation_capture --
--capture-to target/documentation-captures/s130`. Scenes are first launch,
healthy System and State, signal lost, unmanaged addon, weaving configuration,
Auto Potion ready, and Auto Potion blocked. The harness keeps synthetic provenance
and verifies no input; retain these guarantees.

`scripts/curate-documentation-captures.ps1` validates receipts and SHA-256, crops
the dark-wide application sources, and copies a maintainer-supplied MSI image.
Reuse the already authorized checked-in MSI image via a temporary copy if needed;
do not silently replace its bytes or fabricate new provenance. Six dashboard
crops are 1280x640, while Auto Potion ready currently uses 1280x900. The prose
maintenance page still says 820 for that crop, so align to the final actual
inventory rather than carrying this drift forward.

Update `docs/project/documentation-screenshots.json` receipts, hashes, sizes,
caption, alt, source scenes and crops with regenerated PNGs. Check final controls
and no Catalog row/PixelBeacon Status label in all variants.

Pinned production tools are mdBook 0.5.4 and mdbook-linkcheck2 0.13.0 in `build.rs`;
the docs CI additionally pins typos-cli 1.50.1. Release-profile builds call mdBook
and embed `target/docs-site/html`; debug builds use `assets/dev-docs` and do not
prove the release guide. Run policy/render unit tests, mdbook test/build/link
checks, generated-site policy and browser render smoke, spelling, release-profile
embedded-documentation tests and real generated search/navigation stale scans.
Record the dated changelog decision for pinned script/policy/release-guidance
changes. This work authorizes no release/tag and requires no owner field test.
