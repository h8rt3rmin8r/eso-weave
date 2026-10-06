# S121 Public Messaging Audit

Issues: #249 (full interface/help repair) and #222 (encounter lineage).
Baseline: merged `729c298b9134dcb1360cba10372dd2332fefb5d9`, before S121.
The matrices record implemented replacements and justified retention, not a
proposed wording list. All relevant state families are included; the figure
supports the audit and does not replace it.

## Complete Surface Inventory

| Surface and all reachable state families | Implemented disposition record | Canonical help |
| --- | --- | --- |
| Main addon status, expanded System and State, details, package/API compatibility, enablement/loading unknown, current/historical source checks | [Desktop audit](audit-desktop.md) | first-launch, interface, status-reference, pixelbeacon |
| Install/update/repair/remove actions and confirmations: absent/missing folder/managed current/outdated/foreign/inspection error/busy/reload/failure | [Desktop audit](audit-desktop.md), [generated audit](audit-generated.md) | encounter-capture, troubleshooting |
| Encounter History setup, selected Live/PTS source, single/continuous, current/last-saved distinction, retained session, waiting/recording/stopped/interrupted/failed/invalid/unsupported | [Desktop audit](audit-desktop.md), [addon audit](audit-addon.md) | encounter-capture, interface, status-reference |
| Refresh/import/progress/results: no worker/no file/no finished records/duplicates/mixed batch/in-progress/validation/version/read/write failure | [Desktop audit](audit-desktop.md), [generated audit](audit-generated.md) | encounter-capture, encounter-ingestion, troubleshooting |
| History list/detail: mode/ordinal/times/counts/complete/partial/missing encounters/unavailable catalog/incompatible definitions/metrics failure | [Desktop audit](audit-desktop.md), [generated audit](audit-generated.md) | encounter-data-and-metrics |
| Observed results: unavailable rates/empty damage/effects/casts/exact gaps/unknown IDs/quality/provenance | [Desktop audit](audit-desktop.md), [generated audit](audit-generated.md) | encounter-data-and-metrics |
| Provisional recommendations: available/limited/suppressed/empty, each duration/cast/format/order/quality/loss/unknown-target gate, combined conditions and complete citations | [Generated audit](audit-generated.md), [desktop audit](audit-desktop.md) | encounter-data-and-metrics |
| Stop versus clear addon module versus delete one/all imported copies versus uninstall package, all confirmation/refusal/success/failure states | [Addon audit](audit-addon.md), [desktop audit](audit-desktop.md), [generated audit](audit-generated.md) | encounter-capture, discovery-collector |
| Every in-game encounter command/help/load/status/alias/progress/finish/recovery/failure message, dynamic labels and retained-versus-current counts | [Addon audit](audit-addon.md) (54 baseline message sites across both modules) | encounter-capture, companion-addon-commands |
| Catalog command/help/load/progress/category/pause/resume/cancel/start-replacement/clear/current-or-retained status | [Addon audit](audit-addon.md) | discovery-collector |
| Catalog Update alongside recording: candidate discovery/build-from-saved-data/Live selection/PTS preview/trust acknowledgement/compatibility/rollback/progress/receipt errors | [Desktop audit](audit-desktop.md), [generated audit](audit-generated.md) | catalog-updates, catalog-candidate-pipeline |
| Application Log/menu/filter, Application Logging settings, file sink, AddOns/environment setup, linked help | [Desktop audit](audit-desktop.md), [docs audit](audit-docs.md) | logging, configuration, settings |
| All linked public and bundled workflow/help/reference pages, retained maintainer vocabulary, screenshot-label historical context | [Docs audit](audit-docs.md) | Same docs/src corpus builds online and embedded help |
| Ordered observations to validated immutable originals to compatible catalog/metrics/qualified review prompts, provisional native route, text equivalent and maintained source contracts | [Docs audit](audit-docs.md) | encounter-data-and-metrics, diagram-rendering-compatibility |

## Audit Method and Completeness Boundary

Read every relevant centralized constant and inline label/help branch, every
generator whose output reaches those UI surfaces, both Lua modules' message
call sites and presentation maps, and the complete relevant public-guide chain.
Search beyond issue examples for capture/collection, SavedVariables/flush,
terminal/evidence/runtime, logs, mode/channel, loss/version and deletion terms.
Desktop exact removed/added literal annex and addon per-message table supplement
the state-family matrices. Final source anchors are reconciled after formatting.
Unchanged operation names, source IDs and machine format/version codes are
retained only with their explanation and scope, never as unexplained essential
setup or recovery instructions.

Application diagnostics, ESO native combat logs, addon fight recordings, ESO's
saved addon data, imported desktop history and catalog definitions are distinct
throughout. No new setup prerequisite, usage restriction, live-control channel,
schema migration or retention rule was introduced. Existing unknown states and
recovery limitations are explicit. Catalog start replacement and cancelled-data
retention are documented rather than silently changed.

## Corrections Found During Integration

- Empty review prompts can reflect unknown targets or suppressed rules even if
  thresholds were met. Final summary covers omitted prompts and unmet thresholds.
- Loss/unknown-ID limitations do not imply any prompt is visible if another gate
  suppresses advice. Final wording says any available prompts carry the limitation.
- Addon cleanup errors can happen after replacement or quarantine. Generated
  failure guidance asks users to inspect current/recovery files without promising
  unchanged installation. Other-addon/saved-data ownership boundaries remain.
- Mode/channel selection requires recording off and no retained session. Save and
  import are optional preservation steps before explicitly clearing the addon copy.
- Current addon counts are retained in memory; disk snapshot counts are separately
  labeled last saved. Partial replay is indeterminate, including mid-combat starts.

No unresolved messaging gap remains in the audited scope at completion. Actual
repository and hosted outcomes, unperformed field behavior and review status are
recorded in [verification.md](verification.md). This audit does not claim player
usability feedback, installed behavior or Combat Metrics parity was measured.
