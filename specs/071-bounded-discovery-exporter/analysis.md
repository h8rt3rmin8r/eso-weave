# Post-implementation analysis: S071

Date: 2026-09-09

## Result

The specification, plan, tasks, implementation, tests, constitution, issue #115,
and canonical documentation are consistent. No unresolved critical or high
finding remains.

## Resolved findings

1. Constitution 2.0.1 permitted only PixelBeacon while issue #115 and the S068
   source contract required an independently removable discovery collector.
   Constitution 2.1.0 now permits exactly that user-initiated, read-only,
   local SavedVariables bridge while preserving the memory, packet, upload,
   gameplay-mutation, automation, and multi-account prohibitions.
2. An early implementation attempted to infer PTS through an undocumented API.
   The collector now requires the user to select `live` or `pts` explicitly at
   start and the desktop independently verifies the declared channel.
3. The initial 1 MiB chunk design conflicted with the 64 KiB string ceiling.
   Source, addon, parser, schema, tests, and documentation now use one 64 KiB
   limit for both strings and chunks.
4. An invented fixture represented a traversal category as stable identity.
   Fixtures now retain only API-returned stable IDs and label iterator positions
   as version-scoped attributes.
5. Managed removal initially tolerated unrelated files in the collector folder.
   Any unexpected entry now changes status to unmanaged, so install and removal
   preserve it without mutation.

## Requirement traceability

| Requirement group | Implementation evidence | Verification evidence |
| --- | --- | --- |
| Separate bounded addon | `addon/EsoWeaveCollector/`, `src/collector/lifecycle.rs` | `tests/collector_addon.rs`, `tests/collector_lifecycle.rs` |
| Explicit, frame-budgeted lifecycle | Collector slash commands, update budget, combat and deactivation pause, checkpoint, cancel, progress | Static addon contract tests and embedded checksum test |
| Approved discovery and honest scope | Five adapters, stable API IDs, bounded category coverage, warnings | Live and PTS fixtures plus import validation |
| Constrained SavedVariables data | `src/collector/parser.rs`, typed envelope and chunk validation | Hostile syntax, limit, shape, checksum, status, and channel rejection tests |
| Atomic compiler staging | `src/collector/import.rs`, S070 normalization, same-directory candidate replacement | Existing-output preservation, alias rejection, repeated byte stability, live/PTS compile tests |
| Privacy and redistribution | Hashed scope in receipt, local-only text, reference-only icon paths, no upload or image bytes | Receipt and staged bundle assertions plus documentation policy |
| Documentation and governance | Collector guide, architecture, status, rights, troubleshooting, changelog, Plan 038, migration ledger | mdBook and documentation policy gates |

## Constitution check

- Test-first implementation was demonstrated by the initial missing-module
  failure before production collector code existed.
- Parser and addon collection are bounded by bytes, records, strings, chunks,
  depth, tokens, entries, time, and records per tick.
- Collector lifecycle is marker-gated, confined to its dedicated subtree, and
  tested not to mutate PixelBeacon or foreign entries.
- The importer evaluates no Lua and publishes no active catalog directly.
- All new text is UTF-8 without BOM, uses LF after formatting, and passes the
  repository punctuation and mojibake policy.

## Scope confirmation

S071 does not add encounter capture, installed archive extraction, icon byte
distribution, automatic catalog builds, network transfer, or application UI.
Those remain with issues #132, #116, #117, and #118 as applicable.

## Delivery evidence

- PR #139 passed Windows and Ubuntu CI, dependency review, documentation build
  and policy, issue linkage, CodeQL analysis, and the CodeQL result gate.
- The automatic Codex review failed in the external service without producing a
  finding. The single authorized final round was then requested against commit
  `6880f15` and also failed externally because that existing ref could not be
  resolved. It produced no finding, and the failure was acknowledged on the PR.
- No third Codex review was requested. There are no review threads or code
  scanning alerts to resolve.
