# Spec Analysis: S094

## Pre-implementation gate

Status: PASS after planned constitution 4.0.0 amendment.

- Scope is consistent across spec, plan, research, model, contracts, and tasks.
- Every functional requirement maps to at least one task.
- The selected source matrix is explicit and no event-family expansion is hidden.
- Raw source sequence is separate from one-to-many normalized projection order.
- Whole-observation loss, terminal reserve, and hostile-import bounds are explicit.
- V1 compatibility forbids fabricated raw evidence and hash rewriting.
- Current metrics and recommendations remain compatibility consumers.
- Issue #186 remains the coordinator for later capture expansion and pure Rust
  renormalization.

No clarification marker, unresolved constitution violation, or unowned
requirement remains.

## Red-green evidence

Red evidence on 2026-09-15:

- `cargo test --locked --test encounter_addon`: 10 tests ran, 4 passed and 6
  new S094 contracts failed at the missing Lua raw-authority boundary.
- `cargo test --locked --test encounter_import`: 12 tests ran, 8 passed and 4
  new S094 contracts failed at capture-v2 model and store boundaries.
- `node --test .github/scripts/docs-policy.test.mjs`: 133 tests ran, 128 passed
  and 5 new S094 policy contracts failed at the schema-v1 privacy policy.

Green evidence on 2026-09-15:

- `cargo test --locked --test encounter_import`: 14 passed after v1/v2 model,
  validation, canonicalization, transactional store migration, and the
  100,000-observation production-ceiling round trip.
- `cargo test --locked --test encounter_addon`: 16 passed, including the
  production Lua to restricted Rust parser and SQLite exact-value round trips.
- Encounter CLI, history, metrics, recommendations, data-addon, and
  documentation compatibility suites passed.
- `node --test .github/scripts/docs-policy.test.mjs`: 133 passed after the v2
  machine-readable contract validator and exact-local-value policy changes.
- Formatting, strict all-target clippy, the full workspace test suite, the
  locked release build, mdBook tests and build, generated documentation policy,
  scoped spelling, and diff checks passed.
- An existing Windows test helper was corrected to accept a connection reset as
  EOF after a complete bounded documentation response. Production documentation
  service behavior was unchanged, and its suite plus the full suite passed.
- All changed text files passed strict UTF-8 decoding, BOM absence, and
  mojibake checks.

## Local independent review

Status: PASS with no remaining findings.

- The Lua capture review identified boundary-path, source-signature, exact API
  value, and finite-number coverage gaps. The implementation and regression
  suite now exercise each boundary through serialization, restricted parsing,
  SQLite import, reload, and canonical equality.
- The Rust storage and security review identified diagnostic-redaction,
  version-isolation, finite-number, migration, and overflow weaknesses. Each
  was corrected and covered by hostile-input or migration tests.
- The domain and specification review identified projection-linkage, raw-loss,
  completion-status, and metric-quality inconsistencies. Each contract was
  reconciled without expanding the S094 source-family boundary.
- Final re-reviews from all three reviewers reported no actionable findings.

## Post-implementation gate

Status: PASS after local independent review.

- Raw observations are authoritative for schema v2 and remain local-only.
- Selected callbacks and normalization-dependent API reads preserve exact
  ordered scalar values, including nil positions and future scalar arguments.
- Normalized events remain a compatible projection with explicit raw source and
  one-to-many ordinal links.
- Bounds fail by whole observation, declare exact loss, and retain terminal
  capacity.
- Legacy schema-v1 imports and migrated store rows preserve their canonical bytes
  and hashes without fabricated raw evidence.
- No event-family expansion, remote transport, automated capture, or
  recommendation-policy expansion entered S094.
