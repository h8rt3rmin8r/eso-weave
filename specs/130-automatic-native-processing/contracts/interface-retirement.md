# Interface retirement and current documentation contract

S130 scope follows the owner's 2026-10-15 clarification: automatic incremental
native processing supports normal/separate version 15. Normal/inline is tracked
by [#296](https://github.com/h8rt3rmin8r/eso-weave/issues/296), and verbose layouts
by [#297](https://github.com/h8rt3rmin8r/eso-weave/issues/297). These profiles remain
unsupported until implemented; no automatic format conversion, game-setting
change, manual format-fix requirement, or owner field-test gate is introduced.

## Retained product surfaces

- System and State presents `PixelBeacon` and the existing independent
  `PixelBeacon Signal`. One Install/Update/Uninstall lifecycle manages the entire
  combined package through existing ownership and transaction checks.
- Settings retains exactly two direct catalog/encounter controls:
  `Disable Catalog maintenance` and `Disable native encounter log processing`.
  Each uses its canonical persisted preference. Both remain visible but inactive
  without an eligible combined package; services stop while stored data access
  remains independent. No stale callback may re-enable an ineligible service.
- Unrelated dashboard, weaving, fishing, Auto Potion, application logging,
  documentation access, and local API/MCP settings remain accessible.
- ESO retains ownership of its native logging feature. The desktop never toggles
  logging or serialization, synthesizes chat commands, or modifies source files.

## Required absence

The production UI, accessibility tree, strings/inventories, routing state, and
production interaction helpers expose no Encounter History destination or viewer,
Catalog Update destination or modal, Data Details, separate Data lifecycle or
uninstall dialog, Addon Next Step or Catalog row, catalog banner/notice,
Review/Dismiss acknowledgement, recording guide/copy command, source chooser/path
inspection/import, encounter list/detail/recommendation presentation, or encounter
deletion dialog. No passive replacement viewer or maintenance notice appears.

Remove obsolete presentation workers, events, state and layout reservations after
required service responsibilities transfer. In particular, preserve application
Catalog activation, query-path updates, retained history reads, supported
calculations and recommendation processing without selected encounters or open
windows. Production helpers cannot reopen a removed surface.

Absence holds for startup, absent/outdated/unmanaged package, invalid source,
unsupported profile, offline, processing success/failure, restart and recovery.
The retained package lifecycle may explain ordinary install/update failures;
background catalog/encounter processing must not add a replacement player task.

## Data and compatibility

Removing presentation never deletes or rewrites SavedVariables, installed Catalog
generations, canonical addon/native encounters or derived rows. Historical readers
and useful contributor import/inspect/backup/delete utilities remain available
through their domain boundaries, explicitly described as internal utilities.

Authenticated read-only HTTP/MCP access to the fixed catalog/encounters databases
continues while collection is disabled or the addon absent. Current public
observations report actual eligibility and native worker facts without claiming
active game logging from stale file existence. Compatibility identifiers such as
`application.addons.data` and historical SavedVariables/global/schema names may
remain only with a concrete compatibility explanation. Historical fixtures,
completed specifications and archived plans retain historical truth.

## Current documentation and assets

The active manual, navigation/search aliases, executable help, current guidance,
machine contracts, policy fixtures, SVG text/alternatives, screenshots/captions and
generated/release-embedded guide describe one combined package and automatic
services with the two independent disable preferences. Normal/separate version 15
is the delivered native capability; deferred layouts link to #296/#297 and never
claim parity. Missing native facts remain unavailable, rather than invented zero
values or addon-callback measurements.

Every affected surface receives an updated/removed/compatibility disposition in
the final audit. Policy tests positively require delivered automatic behavior and
reject removed current workflows. Preserve unrelated safety, link, accessibility,
source-preservation and corpus coverage requirements. Any changed frozen semantic
projection is reviewed and recorded explicitly; historical baseline commits/blobs
remain immutable.

## Deterministic verification and captures

- UI tests cover pointer/keyboard/accessibility absence, one package lifecycle,
  both controls and eligibility, narrow/wide and short layouts, supported themes
  and scales, System and State expansion, and application-log geometry.
- Service tests prove processing/calculations/retained reads work with every
  removed window permanently absent; UI cleanup does not mutate stored bytes.
- Regenerate all seven capture scenes in two themes and two viewports (28
  variants), preserving synthetic provenance and zero-input assertions. Curate
  seven published application images only after final UI integration. Update
  source receipts, crops, hashes, dimensions, alt text and captions together;
  preserve the authorized MSI image bytes.
- Validate changed diagram visible labels, SVG accessibility metadata and matching
  prose equivalents under existing containment, zoom, no-script, theme and
  fallback-font checks.
- Run pinned policy/unit, mdBook/link, generated-site/browser, spelling, capture,
  and release-profile embedded-guide checks. Scan generated HTML/navigation/search
  and embedded output in addition to current source. Debug fixture embedding alone
  does not prove the release guide.
- Preserve UTF-8 without BOM and LF text; scan deliverables for mojibake. Report
  repository/synthetic evidence precisely and leave installed-game behavior
  unclaimed. No release/tag is included.
