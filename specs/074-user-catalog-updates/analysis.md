# S074 Spec-kit Analysis

## Pre-implementation gate

Result: PASS.

The specification, clarification decisions, requirements checklist, update
safety checklist, research, data model, schemas, plan, quickstart, and tasks are
mutually consistent and implementable without unresolved clarification.

## Coverage matrix

| Concern | Requirement authority | Planned evidence |
| --- | --- | --- |
| Unified availability | FR-001 through FR-004 | Pure Live matrix and independent PTS fixtures |
| Explicit authority and origin | FR-005 through FR-009 | User-action, trusted-origin, channel, schema, and downgrade tests |
| Immutable install and selection | FR-010 through FR-017 | Root, copy, lock, pointer, restart, and recovery failure injection |
| Progress and cancellation | FR-018 through FR-020 | Ordered worker events, honest totals, cancel, commit, and shutdown races |
| Accessible UI | FR-021, FR-022 | App View and AccessKit interaction/sizing tests |
| Collector-assisted build | FR-023 through FR-027 | Lifecycle, fingerprint, hostile capture, S073, and cleanup tests |
| Receipts and privacy | FR-028 through FR-030 | Canonical contracts, disk failure, redaction scans, and preservation tests |
| Cross-platform and docs | FR-031, FR-032 | Shared contract tests and documentation policy |

## Architecture findings and resolutions

1. S073 manifests prove content integrity but carry no signature or authenticated
   origin. The initial S074 wording could have implied official authentication.
   FR-006A now requires an explicit trusted-origin acknowledgement and forbids
   that claim. A future signing authority remains separate work.
2. Requiring a durable failure receipt during complete disk exhaustion was
   impossible. FR-028 now makes persistence best effort while selection safety
   remains mandatory; `receipt-write-failed` is retained in memory.
3. `verify_candidate` requires the hash directory to sit under its declared
   channel. Import, staging, and installed roots therefore all preserve the
   `{channel}/{hash}` shape rather than weakening the verifier.
4. The current S073 user-capture output may be less complete than an accepted
   catalog. Collector-assisted builds use the active Live catalog as a
   zero-removal baseline so partial captures cannot silently discard data.
5. The compiler and collector already carry matching platform-specific atomic
   file replacement. A third private copy would be poor architecture. S074 may
   extract one crate-private helper, keeping public ownership unchanged.
6. API-version checking still owns marker-gated PixelBeacon maintenance, but no
   longer owns catalog status policy. Its bounded result becomes evidence for
   the single catalog availability resolver.

## Constitution gate

- Full specify, clarify, checklist, plan, tasks, and analyze artifacts exist
  before production implementation.
- No process-memory, packet, automatic input, upload, new addon category, or
  unattended update path is introduced.
- Existing PixelBeacon, collector, input, and fishing safety suites remain
  mandatory and unmodified in authority.
- Selection and receipts are derived runtime state outside user configuration.
- Test-first tasks cover every new filesystem, concurrency, privacy, and UI
  boundary.
- No pinned workflow, packaging, release, or governance change is planned.
- Full CI parity remains mandatory before every Rust commit.

No CRITICAL, HIGH, duplicate requirement, unresolved ambiguity, template token,
unsupported authenticity claim, impossible durability guarantee, or authority
conflict remains. Implementation may begin.

## Post-implementation gate

Result: PASS, with Linux parity delegated to pull-request CI.

- The public candidate inspection returns only redacted summary fields after the
  complete S073 verifier succeeds.
- The pure resolver covers all seven Live states and an independent PTS preview.
- Contract tests cover trusted-origin acknowledgement, PTS rejection,
  cancellation, cross-process locking, immutable install, restart resolution,
  rollback, invalid selection fallback, canonical redacted receipts, bounded
  recovery, and the later-flush collector boundary.
- The collector build reuses the S071 restricted parser and S073 pipeline with
  the accepted Live catalog as a zero-removal baseline. The result remains a
  review candidate until a separate explicit install.
- The application starts with the bundled catalog while a joined background
  worker resolves user selection, discovery, collector lifecycle, install,
  rollback, and cleanup. No update begins from startup or version evidence.
- The egui modal is responsive and scrollable, uses text with every status and
  progress state, exposes accessible controls, closes with Escape when idle,
  disables cancellation at selection commit, and uses no fabricated animation
  for indeterminate work.
- Atomic temporary-file replacement was extracted into one crate-private helper
  shared by the compiler, collector importer, and S074 selection/receipt writer,
  resolving pre-implementation finding 5 instead of adding a third copy.
- First-round review hardening separates saved-catalog startup resolution from
  candidate discovery, binds trust acknowledgement to one candidate identity,
  presents acquisition provenance before that acknowledgement, and serializes
  terminal receipt allocation under a dedicated cross-process lock.
- Format, strict all-target/all-feature Clippy, the complete locked test suite,
  both optimized binaries, mdBook test/build/link checking, generated-site
  policy, spelling, JSON parsing, diff whitespace, UTF-8/BOM, forbidden-dash,
  and mojibake checks pass locally on Windows.

No new dependency, remote candidate feed, signing claim, package mutation,
automatic install, capture upload, Lua execution, PTS promotion, or PixelBeacon
mutation was introduced. Pull-request CI remains the authority for the matching
Linux run.
