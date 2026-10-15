# S130 current surface audit

Audit date: 2026-10-15. The owner-approved delivered native profile is
normal/separate version 15. Normal/inline remains issue #296; verbose remains
issue #297. No player format-repair workflow or owner field-test gate is added.

## Updated and removed surfaces

| Current surface | Disposition and reason |
| --- | --- |
| `src/app/ui.rs` | Removed entire Encounter History destination/viewer, source inspection/import, recording guide/copy actions, deletion dialogs, Catalog modal/worker/state/helpers, Data Details, separate lifecycle aliases, Catalog row and unused instruction-row layout. Retained background activation handoff, one package lifecycle, settings, two preferences and unrelated rendering. |
| `src/app/strings.rs` | Renamed visible field to PixelBeacon; removed retired Data/Catalog/menu/help inventories. Corrected package lifecycle tooltips to combined files and natural Lua load. |
| `src/app/encounter_history.rs` | Deleted UI-only worker, events and presentation. Domain readers and application-owned derivation replace its required authority. |
| `tests/app_ui_sizing.rs`, `app_strings.rs`, `app_encounter_history.rs` | Removed modal-only expectations, retained unrelated geometry/contrast checks, added keyboard/accessibility retirement checks across states/themes/layouts and direct history metric/recommendation/source integrity checks. |
| `tests/support/documentation_capture.rs` | Removed deleted view projections from scene checks; retained scene truth, safe sandbox and zero-input guarantees. |
| `docs/src/features/encounter-capture.md` | Rewritten as Automatic Native Processing: eligible discovery/catchup/append, explicit supported/deferred profiles, independent disable preferences, preserved data and local reads. No replacement viewer. |
| `features/interface.md`, `pixelbeacon.md` | Current menu/dashboard/lifecycle/controls and actual numeric API authority. Removed all old window/row instructions and future-watcher claims. |
| `getting-started/first-launch.md`, `troubleshooting.md` | One package and automatic services; retained independent lifecycle/input diagnosis. Automatic waiting/unsupported/recoverable states replace source-picking/import/deletion steps. |
| `reference/status-reference.md`, `settings.md`, `logging.md` | Removed obsolete status and source-inspection rows; documented independent preferences, background observations and game logger ownership. Application diagnostics remain distinct. |
| `reference/encounter-data-and-metrics.md` | Reconciled native schema 2/store schema 7, retained source hashes, historical addon qualifications and application-owned generation receipts/calculations. Historical contracts explicitly remain historical. |
| `reference/local-api-and-mcp.md`, `local-extension-state-fields.json` | Actual native worker phase/count/diagnostic fields; current logging activity remains unknown from files. Fixed authenticated read-only database authority remains independent of collection. |
| `reference/glossary.md`, `docs/project/content-coverage.json` | Retired PixelBeacon Status search term becomes Package Installation, with compatible aliases and alphabetical glossary. Encounter lineage becomes automatic/source-specific. Unrelated evidence/obligations remain pinned. |
| `development/architecture.md`, `encounter-ingestion.md`, `test-strategy.md` | Background ownership, canonical compatibility, generation/cancellation/storage boundaries and relocated test responsibilities replace UI-owned/manual execution. |
| `development/catalog-compiler.md`, `catalog-updates.md`, `companion-addon-commands.md` | Existing compiler/diagnostic utilities remain contributor tools; automatic player behavior and empty desktop-to-addon command transport are explicit. Removed future watcher and legacy player commands. |
| `README.md`, `docs/src/README.md`, `SUMMARY.md`, feature/reference indexes | Automatic data entry and navigation; established page paths remain valid with current title. |
| `docs/project/encounter-model.json` | Added explicit current operation boundary and native processing policy. Historical recorder/source/replay research and S127 completed-file contributor utility remain for compatibility with concrete labels. |
| `docs/project/documentation-visualization-audit.json` | Reconciled automatic-native page title/current source capability rationale. Existing diagram warrants and unrelated pages retained. |
| Encounter lineage/troubleshooting SVGs and prose equivalents | Updated source/automatic processing, actual source/progress/store boundaries and qualified derived results. Preserved node/edge topology, SVG title/description, alternatives, accessible equivalents and zoom/containment checks. |
| Seven deterministic screenshot PNGs and `documentation-screenshots.json` | Regenerated 28 source variants; curated seven, updated receipts/hashes/crops/dimensions and adjacent image dimensions. Six 760-pixel crops now retain every Skills row; the settings image remains 900 pixels. |
| `scripts/curate-documentation-captures.ps1`, screenshot maintenance | Fixed old 640-pixel crop that cut controls mid-row; preserves complete image geometry and authorized MSI bytes. Dated pinned decision recorded by primary. |
| `.github/scripts/docs-policy.mjs`, unit fixtures, render smoke | Replace old exact manual steps with automatic behavior and forbidden-workflow checks. Native current model policy has positive/negative contract coverage. Real corpus is 20 fences and 66 tables across 27 table-bearing pages after removing obsolete command/source UI material. |
| `tests/documentation.rs` | Release-profile guide asserts current automatic controls/profile/followups and scans actual embedded HTML/SVG/search data for retired workflows. Existing loopback service tests remain unchanged. |
| `docs/theme/eso-weave.js`, `eso-weave.css` | Complete-height screenshots exposed a preexisting 200-percent-zoom caption overflow. Captioned raster images now fit the visual viewport using measured header/caption/padding space without upscaling or changing aspect ratio. SVGs retain their existing scrolling/readability behavior. Panel bounds include borders/padding. Existing browser containment gate remains unchanged. |

## Reviewed compatibility and unchanged current surfaces

- The established `features/encounter-capture` path is retained to avoid breaking
  public links; its current title and navigation describe Automatic Native
  Processing. Current fragment references were updated to the new sections.
- `application.addons.data` is an explained compatibility view of Catalog capability
  in the combined package. It does not imply a second addon installation.
- Legacy `EsoWeaveDataSaved`, manifest-derived save filenames, historical schemas,
  historical addon fixtures, archived plans and completed specifications preserve
  source compatibility and historical truth. They do not function as current
  player manuals. Current native discovery and whole-file contributor tools have
  distinct source-hash meanings documented explicitly.
- Native completed-file inspection/import, historical readers and backup/deletion
  commands remain contributor/domain utilities. No product UI exposes them and no
  normal player operation requires them.
- `docs/project/catalog-sources.json` and source/rights pages retain bounded source,
  license and local-art rules. Explicit local icon source selection is an internal
  library feature, not Catalog maintenance. Candidate preparation/review is labeled
  maintainer activity and cannot replace automatic player operation.
- The existing Catalog lifecycle and local-extension authority diagrams already
  describe S129 automatic activation and fixed authenticated databases correctly.
  They retain accessibility/geometry validation; encounter changes do not grant
  gameplay input or arbitrary database/file authority.
- Installation, responsible use, release/packaging guidance, brand, input safety,
  state machines and unrelated screenshots were inspected. Existing one-package
  or source-independent rules remain appropriate. The authorized MSI image remains
  byte-identical (`ec989990f5f4d43b8418955df8cd993a24de15e48cf4a15c9c60cf6ce9d0945a`).
- Debug `assets/dev-docs` is deliberately a labeled development fixture. Only
  release-profile embedding proves the complete current manual.

## Frozen projection changes

Content coverage changes only the removed field's canonical search term and the
encounter lineage facts. Its reviewed digest is
`a57b9b48292481c1672569a5cac4867e7da266d4c83f6d00343c16e202cab044`.
Historical baseline commits/blob identities, coverage source obligations and all
six safety crosswalk requirements remain intact. The troubleshooting text fence
hash changes with the automatic-processing diagnostic branch; the retired player
command fence is removed rather than retained as empty material.

## Evidence boundary

See `surface-verification.md` for actual repository/generated/embedded results.
Synthetic app captures and fixture tests prove their named scope. No installed
game behavior, logging flush cadence or unsupported serialization profile is
claimed, and no owner field-testing task or release/tag is required.
