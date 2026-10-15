# S129 package verification

Date: 2026-10-15. Scope: #285 and the package/configuration portion of #291.
Evidence below is repository and synthetic-fixture evidence, not installed-game verification.

## Acceptance mapping

| Requirement | Implementation and evidence |
| --- | --- |
| One intended package, authoritative inventory/version, deterministic load | `addon/PixelBeacon/PixelBeacon.txt` is version 24 and loads beacon, startup configuration, bootstrap, then Catalog. `beacon::INVENTORY` declares five files. `tests/data_addon.rs::legacy_entry_points_share_the_exact_pixelbeacon_inventory` checks repository and deployed inventories. |
| Preserve every beacon signal, geometry, configured size and decoder constants | `tests/beacon.rs` verifies protocol values, source state machines and configured geometry. Its full-file Lua 5.1 test compares the root and all 43 layout/payload cells through four bootstrap/Catalog registration or callback failures. The test discovered the pre-existing 200-local Lua limit; 103 constants now live in a private table, with unchanged values and legacy/current numeric extraction tests. |
| Isolated Catalog/bootstrap and no automatic legacy recorder | Beacon registers first and has no data bootstrap dependency. Catalog uses `PixelBeaconCatalog` namespaces. `Encounter.lua` exists only in historical test fixtures; the production inventory/manifest omit it. Collector tests verify quiet collection, independent state, and preservation of unknown/legacy SavedVariables. |
| Preserve SavedVariables and readable historical data | The file remains `SavedVariables/EsoWeaveData.lua`, root `EsoWeaveDataSaved`, schema/addon version 1. The Data-only migration fixture compares exact saved Catalog/encounter bytes and unrelated addon bytes. Lifecycle mutation paths are confined to the selected AddOns root; desktop history is outside those paths. Legacy encounter reader fixtures remain available. |
| Neither, Pixel-only, Data-only, both, current, conflicting and interrupted installations | `tests/s129_package.rs` covers automatic migration combinations, idempotent repeats, partial repair and unknown-file refusal. Package unit tests cover process interruption before/after commit and later recovery. An absent context is not automatically installed. |
| Staged replacement, ownership, rollback and exact recovery | `src/beacon/package.rs` combines bounded stable no-follow reads, exact inventories, marker checks, post-retirement snapshots, no-replace moves and fixed-name SHA256 receipts. Failpoints cover pre-staging, retirement, commit, changed files, directory swaps, raced targets and cleanup. Unknown recovery bytes remain preserved. |
| Distinguish committed cleanup failure; report actual disk state | `LifecycleError::CleanupPending` carries committed status/recovery path. Inspection reflects the actual target independently. Cleanup/retry and interrupted-commit tests verify retained prior bytes and recoverable current selection. |
| Live/PTS separation and reviewed API compatibility | Migration/configuration receive the actual root and environment. Integration tests preserve a PTS configuration while changing Live. A future observed API does not alter reviewed declarations; corrupt/partial/unknown-API package fixtures are ineligible. |
| Running/stopped/unknown states and natural loading | All unified lifecycle/configuration outcomes set `reload_required=false`. Beacon tests cover running-state combinations. No package lifecycle path forces reload, relog, restart, chat command or gameplay input. Disk changes do not acknowledge already-loaded Lua. |
| Compatible presence distinct from package ownership | `inspect_presence` checks identity, compatible version, declarations, bounded module signatures, valid startup configuration and a present self-consistent Catalog checksum. Manual package eligibility does not grant install/remove authority. A regression rejects truncated Catalog source with no checksum. |
| #291 startup preference for compatible manual packages | `src/beacon/package/configuration.rs` owns only the exact bounded generated startup representation. It updates that file without adopting the package or changing other modules/helpers, rechecks confined parent and source hashes at mutation/recovery boundaries, and uses no-replace rollback/recovery. Manual package status remains Unmanaged. |
| Compatibility lifecycle/CLI entry points | `src/data_addon.rs` delegates to unified lifecycle operations and retains SavedVariables parsing helpers; its install path cannot recreate a separate Data directory and preserves existing geometry/preferences. |
| Current policy, documentation and visuals | The constitution/CLAUDE scoped amendment supersedes the two-package/manual-Catalog rules. PixelBeacon, installation, first-launch and status-reference documentation explain one lifecycle and automatic maintenance. Offscreen deterministic captures use temporary combined-package fixtures; source/destination digests and captions remain governed. |

## Configuration authority decision

A compatible package's canonical `StartupConfig.lua` is the package-owned preference seam. User-requested preference changes authorize only that bounded file, independently of permission to replace/delete the surrounding addon. The implementation accepts the exact typed generated representation, verifies stable regular files and unchanged required source hashes, and preserves all other bytes. A readonly, changed, linked or raced target yields a diagnostic or retained recovery artifact, never broader package ownership. Desktop cancellation remains immediate; already-loaded Lua applies the saved startup preference at its next natural load.

## Recorded verification

- `cargo test --test app_view_model --test beacon --test data_addon --test s129_package --no-fail-fast`: 89 + 69 + 5 + 5 passed before the final configuration-only authority correction.
- `cargo test --lib beacon::package`: 15 Windows tests passed after the configuration correction and post-rename/pre-marker regression, including six configuration tests. Unix-only package/link-race tests require the Linux check.
- `cargo test --test s129_package --test data_addon --no-fail-fast`: 5 + 5 passed after the configuration correction and missing-checksum regression.
- Offscreen documentation capture produced 28 deterministic variants; seven affected application figures were curated with verified source and destination digests.
- Node documentation policy/render unit tests: 163/163 passed after the final captures and status-reference correction.
- Owned Rust formatting and UTF-8 without BOM/mojibake checks passed.
- Full final repository gates, hosted platform checks and the bounded review protocol are recorded by the parent slice verification, not inferred from these focused runs.

## Verification limits

- No installed ESO directories, game sessions or native gameplay input were used. Fixtures do not prove actual addon enablement/loading, game save timing, or platform field behavior. No owner field-testing gate is introduced.
- Presence recognition checks the declared package contract and bounded source signatures; it does not execute arbitrary manually modified Lua or establish its runtime semantics. The embedded full-file Lua source is compiled and exercised separately.
- A failure after replacement rename but before its commit marker now returns committed-state CleanupPending; the targeted regression retains previous bytes and verifies recovery.
- The 28-variant capture matrix was rerun after the final lifecycle/API status-label correction. All seven governed application images, source/destination digests, dimensions and captions reflect that render; the MSI source remained unchanged.
