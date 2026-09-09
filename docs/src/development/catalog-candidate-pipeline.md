# Reviewed Catalog Candidate Pipeline

S073 provides a maintainer-only path from pinned source inputs to an immutable
review candidate. It composes the bounded collector importer, deterministic
catalog compiler, verifier, semantic diff, and local icon cache. It does not add
another source parser and cannot change the catalog used by the application.

## Request and source gates

Every request fixes the mode, Live or PTS channel, game version, API version,
catalog version and schema, locales, tool version, sources, baseline, and removal
thresholds. Input paths are workspace-relative. Local files are opened through
bounded no-follow handles and cached by exact SHA-256 without replacement.
Normalized source snapshots must match the acquired channel, version, locale,
revision, hash, license scope, and redistribution decision. Collector snapshots
retain the fixed local-only rights assigned by the restricted importer.

Remote acquisition is optional. The request and command must both enable it.
Only HTTPS raw content from the approved repository host at an immutable
40-character commit revision is admitted. Redirects are denied, reads have a
timeout and byte limit, and content is cached only after its declared SHA-256
matches. Refresh failure is fatal unless the request explicitly allows a
verified stale cache;
`sources.json` records that choice. Cold and warm non-refresh retrieval both use
the stable `pinned-remote` provenance value, so cache temperature cannot change
candidate identity. New source objects remain staged until the complete
candidate verifies and installs. Icon generations are built in the same run
staging area, then copied into the user-local cache only after installation.
An allowed stale-cache fallback also appears as a stable warning in
`validation.json`.

Live and PTS remain independent. The request, every applicable source, normalized
bundle, optional baseline, compiled catalog, and manifest must agree on channel.
There is no automatic PTS promotion.

## Candidate contract

`pipeline-build` writes exactly these review files:

- `catalog.sqlite`;
- `manifest.json` and `checksums.json`;
- `sources.json`, `validation.json`, and `diff.json`;
- `build-report.json` and `verify-report.json`; and
- `icons.json`.

Reports contain stable source identities, hashes, counts, and results. They omit
local paths, normalized source content, captures, localized catalog values, and
icon bytes. The candidate ID is the SHA-256 of its canonical manifest. Existing
candidates are verified and reused, never replaced.

Coverage completeness downgrades are explicit regressions and consume the same
configured threshold as removed coverage rows. Any localized-text redistribution
change blocks publication.

`pipeline-verify` checks the exact file allowlist, canonical manifest, artifact
sizes and hashes, directory identity, catalog integrity, channel, and semantic
checksum. Removal thresholds and baseline channel checks run before publication.

## Automation boundary

The pinned `catalog-candidate` workflow runs manually or on a schedule with
read-only repository permission and a 30 minute matrix-job timeout. It builds
and verifies invented Live and PTS candidates and uploads them for review. It
contains no repository write, installation, promotion, or release step. Active
catalog selection, rollback, and end-user update behavior are provided by the
[user-initiated catalog update workflow](catalog-updates.md). Candidate integrity
does not authenticate its download origin, so installation keeps a separate
trusted-source acknowledgement.
