# S131 Verification and Delivery

Date: 2026-10-15. Scope: issues #296 and #297.

## Implemented behavior

The incremental schema-2 parser supports normal/inline and both verbose v15
profiles alongside existing normal/separate input. One bounded semantic decoder
is used by feed, context filtering, canonical validation, logical identity,
checkpoint restoration and projection. Exact original lines remain evidence.
Format switches need no persisted run state. Signed producer quantities remain
signed observed sums; negative observations qualify derived rates.

Frozen schema-1 compatibility utilities, existing normal schema-2 escaping,
canonical bytes, identities and unknown classification remain compatible.
Schema7 storage and checkpoint version 1 are unchanged. Automatic processing,
atomic progress, cancellation and fixed read-only HTTP/MCP services are reused.

## Evidence and local checks

- Producer grammar established before code from the pinned static v15 writer;
  official current/historical uploader assets and public parser were inspected.
- New profile tests were observed failing against S130 before implementation.
  A literal standalone-metadata regression also failed before its decoder fix.
- Format tests cover exact raw/context offsets, source quantities, inline
  defaults/optionals, repeated verbose fields, equipment, mixed profiles,
  signed values, unknown/malformed records, limits and unsupported versions.
  Every byte split is compared through checkpoint/restore for all three added
  profiles, with CRLF coverage and frozen schema-1/large-total regressions.
- Store tests exercise failed progress writes and omitted finalizers, atomic
  retry, deduplication, divergent evidence and deletion suppression. Automatic
  worker tests resume partial files across restart and independent disable.
  Authenticated HTTP and MCP read the same real stored native results without
  a Catalog. Existing stop-boundary, generation and historical tests remain.
- Documentation policy/render/brand/trust unit suite: 184 passed. Brand/trust,
  spelling, mdBook examples/build, generated-site policy and all browser smoke
  sentinels passed. Release binary, embedded connection test and all ten release
  documentation tests passed.
- Full locked Windows Rust suite: 1207 passed across 54 result groups; formatting
  and all-target/all-feature clippy passed. The final repeat after updating the
  release-only guide assertion also passed before commit.
- Final staged text pass: 54 files passed UTF-8/no-BOM/LF, punctuation/mojibake
  and JSON checks; git diff whitespace checks passed before publication.

## Publication and review boundary (before owner merge)

Published [PR #299](https://github.com/h8rt3rmin8r/eso-weave/pull/299) from
implementation commit `376b0e658e225bd61fae3dc98851de324054d8cf` under the
owner's push/PR authorization. Issues #296 and #297 are in Project PR review.

Codex round 1 (PR-open automatic trigger) completed at 2026-10-15 16:00:32 UTC
without findings. The bot's +1 reaction at 16:00:37 UTC and empty submitted-review
and inline-thread lists confirm the result. The
[review summary](https://github.com/h8rt3rmin8r/eso-weave/pull/299#issuecomment-6196377734)
identifies reviewed commit `376b0e6`. One of the two authorized rounds was used;
no second review is needed for a clean result. No finding remains unresolved.

All hosted gates passed on reviewed implementation head `376b0e6`: Windows and
Linux Rust checks/release builds, documentation build/browser validation,
protected trust policy, proposed policy, closing-issue, dependency review and
CodeQL. PR deployment was correctly skipped. The final commit updates delivery
records only; implementation code remains identical to the reviewed head.
The PR conversation carries the complete exact-head hosted-check receipt after
that final publication. Delivery stops for owner final review and merge.

No game or uploader code was executed, no source settings or logs were changed,
and no binary/raw private source log is committed. Static producer inspection,
synthetic tests and hosted checks do not establish installed-game behavior.
No owner field-testing task, automatic merge, tag or release is included.

## Owner merge and housekeeping (2026-10-15)

The owner merged PR #299 at 16:33:46 UTC as
`b28d34e686933297e2aaf4f4dadac88ace979370`. The merged tree equals final reviewed
delivery head `0dd0f4ec3e23b1ec54304fc947df2b8246120aaa`. Issues #296/#297 are
closed and Project Stage Done; milestone 21 has twelve closed and zero open
issues and is closed. Main was synchronized, the feature branch removed, and
Plan057 archived with its indexes and migration ledger. The refreshed repository
has no open issue; Dependabot PR #284 is separate maintenance. No new slice,
release, tag or field-verification claim is introduced by this housekeeping.

Documentation-only housekeeping passed 184 policy/render/brand/trust tests,
the repository trust scan, corpus/generated-site policy, nine-file UTF-8/no-BOM/LF
and JSON checks, and git diff whitespace checks. Rust and product sources are
unchanged by this administration.
