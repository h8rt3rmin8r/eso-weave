# S129 Backend and Runtime Acceptance Receipt

Session: 2026-10-15. This receipt records repository and synthetic fixture evidence. It does not claim installed-game behavior or completion of the final-head gates.

## Verification sequence

1. The independent packaged-baseline regression failed against the original implementation: coverage removal count 15 exceeded threshold 0. Evidence: `target/s129-catalog-red-workspace-temp.log`.
2. The focused backend matrix passed 55 tests: automatic Catalog 9, candidate pipeline 14, Catalog update 21, encounter history 11. Evidence: `target/s129-catalog-second.log`. Earlier compiler 15, collector import 7, data addon 5 and package 5 suites also passed in `target/s129-catalog-matrix.log`; that earlier combined run still contained the subsequently corrected Catalog failures.
3. The first repository-wide pass exercised 10 automatic Catalog regressions successfully. Evidence: `target/s129-full-first.log`. Its other failures and environment limitations are tracked in the main verification report.
4. A deterministic runtime regression demonstrated the postcommit publication defect: the selected immutable Catalog was committed while the coalesced consumer path still pointed to the packaged Catalog after cancellation in the Complete callback. Evidence: `target/s129-runtime-postcommit-red.log`.
5. The corrected exact regression passed. Evidence: `target/s129-runtime-postcommit-green.log`.
6. Seven deterministic runtime cases passed in `target/s129-runtime-matrix-first.log`: committed publication, partial-source retry and recovery, periodic reconciliation despite identical file size/time metadata, semantic replay, package disappearance and precommit stop, eligibility-generation invalidation, primary-root removal and reordered restart, and read-only startup preference failure/retry without adopting a manual package.

## Implemented acceptance paths

| Concern | Implementation and evidence |
| --- | --- |
| Independent population and accumulation | `catalog::accumulate` reconstructs verified immutable input, unions supported typed facts, relationships and locales, resolves scalar conflicts by observation time and digest, and retains evidence used by surviving facts. `tests/catalog_automatic.rs` begins with the actual packaged baseline source. |
| Coverage and unsupported sources | Coverage comparison uses category, locale and scope rather than acquisition snapshot identity. Collector declarations accept bounded, opportunistic and unknown coverage; derived text/icon-reference coverage reflects actual records. Integrity, removal thresholds and rights-change gates remain active. |
| Automatic activation | `maintain_collector_capture` consumes a stable complete snapshot without a manual observation boundary, holds the operation lock across build/verification/selection, opens the candidate read-only before selection, and skips content-equivalent generations. |
| Cancellation and publication | The final selection boundary is fenced against cancellation. Runtime publishes an Activated outcome even when a stop arrives after that boundary. Precommit cancellation and package disappearance retain the previous selection. |
| Root and channel identity | A persistent fingerprint binds the default registry once. Other roots have stable separate registries regardless of discovery ordering or which eligible Live root supplies current reads. The fingerprint includes the actual game parent as well as the AddOns target so shared AddOns junctions do not combine separate SavedVariables roots. PTS selection never replaces main Live selection. |
| Recovery and history | Accepted-domain lineage distinguishes committed versions from staged candidates. Selection loss/corruption recovers accepted data before packaged fallback. Historical resolvers retain exact channel/API/game identity, with the existing same-API patch compatibility as a separate fallback. History clones share verified registry registrations. Startup registers retained registries independently of current addon/source availability. |
| Retry and bounds | One worker and one queued job serialize builds. Discovery is bounded to 16 roots, scheduling rotates fairly, retries back off from job completion, and metadata hints cannot prevent the periodic full reconciliation. Eligibility changes invalidate cached observations; stale jobs cannot restore them. Superseded same-domain storage is pruned while historical domains are retained; publication pressure stops new selection rather than evicting historical support. |
| Preferences and startup files | Both service tokens begin canceled while presence is absent and remain canceled through preference changes until recognized presence. The coordinator reports `startup_config_persisted` as null for no eligible roots, false for any mismatched/unwritten literal, and true only when every eligible root's on-disk startup representation matches. This is separate from application settings durability and never establishes already-loaded Lua state. Config write failures use bounded existing-log diagnostics. |

## Final parent verification

The parent completed cargo fmt, all-target/all-feature Clippy with warnings denied,
and the full locked Rust suite: 1,171 passed, zero failures or ignored tests.
This includes all thirteen automatic Catalog cases, seven runtime cases and the
final model/root-fingerprint changes. See verification.md and
target/s129-full-final.log. Hosted final-head checks and review remain separate.

All runtime fixture operations target temporary directories, use non-running test game state, and preserve the source SavedVariables fixtures. No actual game profile, installed game session, release or tag was used.
