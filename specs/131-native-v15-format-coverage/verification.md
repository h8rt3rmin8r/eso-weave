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

## Publication and review boundary

Automatic feature-branch push and official PR publication are owner-authorized.
No review round has been used before publication. The PR conversation will
carry review findings, resolutions and exact-head hosted-check receipts.
At most two Codex rounds are permitted. Stop for owner final review and merge.

No game or uploader code was executed, no source settings or logs were changed,
and no binary/raw private source log is committed. Static producer inspection,
synthetic tests and hosted checks do not establish installed-game behavior.
No owner field-testing task, automatic merge, tag or release is included.
