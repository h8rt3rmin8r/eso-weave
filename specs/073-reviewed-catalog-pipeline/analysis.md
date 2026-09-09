# S073 Spec-kit Analysis

## Pre-implementation gate

Result: PASS.

The specification, clarification decisions, requirements checklist, pipeline
safety checklist, research, data model, schemas, plan, quickstart, and tasks are
mutually consistent and implementable without unresolved clarification.

## Coverage matrix

| Concern | Requirement authority | Planned evidence |
| --- | --- | --- |
| Four explicit modes | FR-001 through FR-005 | Live, PTS, offline, and capture fixtures |
| Bounded acquisition | FR-006 through FR-009 | Local handle and mock HTTPS hostile cases |
| Existing boundary composition | FR-010 through FR-014 | Import, build, verify, diff, and icon tests |
| Review gates | FR-015 through FR-018 | Threshold and candidate privacy tests |
| Immutable publication | FR-019 through FR-021 | Failure, collision, reuse, and determinism tests |
| Least-privilege automation | FR-022, FR-023 | Workflow source policy tests |
| Diagnostics and docs | FR-024, FR-025 | Redaction and documentation policy |
| Application isolation | FR-026 | Configuration and startup regression suites |

## Architecture findings

1. Issue #117 describes acquisition, interpretation, orchestration, review, and
   automation. S070 and S071 already own interpretation, so S073 composes those
   modules rather than adding a second parser. This explicit deviation reduces
   duplicated authority while preserving the issue outcome.
2. Reproduction does not require raw bytes inside the candidate. Content hashes,
   immutable revisions, and an external content-addressed cache preserve exact
   identity while keeping restricted source and user bytes out of artifacts.
3. The S072 stable file reader is the correct primitive for new local input but
   is private to icon paths. A crate-private extraction is proportional and
   avoids platform security logic drift.
4. Application installation and progress UI would mix candidate construction
   with active-state mutation. They remain in blocked issue #118.

## Constitution gate

- Full spec-kit sequence exists before production implementation.
- No input, process-memory, packet, gameplay, upload, or new addon surface changes.
- Test-first tasks cover new hostile filesystem and network boundaries.
- Full CI parity remains mandatory before each Rust commit.
- The pinned workflow is required, least privilege, and receives a dated decision.
- No constitution amendment or exception is required.

No CRITICAL, HIGH, duplicate requirement, unresolved ambiguity, template token,
or authority conflict remains. Implementation may begin.

## Post-implementation gate

Result: PASS.

The implementation follows the frozen contracts and the deliberate composition
decision. It extracts one stable bounded file primitive, shares game-version
parsing, admits only exact policy identities or project-authored fixtures,
requires dual network consent, labels stale cache use, and passes verified bytes
to the S071, S070, and S072 authorities. Candidates match the nine-file
allowlist, their manifest matches the JSON schema, and verification rechecks
canonical form, every artifact, source and icon identities, channel, version,
and catalog integrity.

Offline mode was corrected during implementation to allow either exact channel.
Treating it as Live-only would have contradicted the specification and prevented
offline PTS review. It still rejects every network, refresh, and stale-cache
flag.

Verification evidence is green for focused pipeline and workflow tests, strict
all-feature Clippy, the complete locked Rust suite, both optimized binaries,
mdBook test and build, generated-site policy, spelling, JSON parsing, diff
hygiene, UTF-8 without BOM, forbidden-dash, placeholder, and mojibake scans. One
documentation server test produced a transient Windows connection reset on the
first full-suite run; its exact rerun and the subsequent complete suite passed.

No CRITICAL, HIGH, contract mismatch, unresolved clarification, template token,
privacy leak, workflow authority escalation, or application update behavior
remains. S073 is ready for pull-request review.

## First-round review hardening

The first automated review found four publication-boundary defects. The
implementation now compares normalized snapshots with full acquired source
identity and rights, uses stable provenance for cold and warm pinned-remote
inputs, counts completeness downgrades as coverage removals, and delays source
cache publication until the staged candidate verifies. Focused regression
tests cover each defect, including failure without a new cache object.

The authorized final review found five additional boundary gaps. Localized-text
redistribution changes now block independently of count thresholds, source and
icon cache publication follows successful candidate installation, stale-cache
reuse appears in validation findings, and the scheduled job has a policy-tested
30 minute timeout. A final-install collision test proves that neither cache is
published when the immutable candidate destination is invalid.
