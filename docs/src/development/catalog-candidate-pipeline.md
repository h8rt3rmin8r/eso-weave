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

Remote acquisition is optional. The request and command must both enable it.
Only HTTPS raw content from the approved repository host at an immutable 40-byte
commit revision is admitted. Redirects are denied, reads have a timeout and byte
limit, and content is cached only after its declared SHA-256 matches. Refresh
failure is fatal unless the request explicitly allows a verified stale cache;
`sources.json` records that choice.

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

`pipeline-verify` checks the exact file allowlist, canonical manifest, artifact
sizes and hashes, directory identity, catalog integrity, channel, and semantic
checksum. Removal thresholds and baseline channel checks run before publication.

## Automation boundary

The pinned `catalog-candidate` workflow runs manually or on a schedule with
read-only repository permission. It builds and verifies invented Live and PTS
candidates and uploads them for review. It contains no repository write,
installation, promotion, or release step. Active catalog selection, rollback,
and end-user update behavior belong to the later update-orchestration slice.
