# Data Model: User-Initiated Catalog Updates

## Catalog target

`CatalogTarget` is either `bundled` or `user` with one lowercase 64-character
candidate SHA-256. It never stores a path. Only a verified Live S073 candidate
may become a user target.

## Catalog selection

`selection.json` contains:

- schema version `1`;
- monotonically increasing generation;
- active catalog target;
- optional previous catalog target; and
- optional last receipt generation.

The file is canonical JSON with a trailing newline, bounded to 16 KiB, read from
a no-follow regular handle, and atomically replaced in its parent directory.
Missing selection means Bundled generation zero. Invalid selection degrades to
Bundled with a visible diagnostic and is never rewritten silently.

## Catalog roots

All mutable roots are descendants of `<config>/catalog/`:

- `import/live/<sha256>/` and `import/pts/<sha256>/`: user-provided S073 candidates;
- `staging/.update-<random>/`: incomplete same-filesystem work;
- `versions/live/<sha256>/`: immutable accepted candidates;
- `receipts/<generation>.json`: redacted operation evidence;
- `sources/` and `icons/`: S073 local build caches;
- `selection.json`: active and rollback pointer; and
- `operation.lock`: cross-process advisory lock.

Existing roots must be real directories. Links, reparse points, aliasing, wrong
channel parents, extra files, and non-regular contracts fail closed.

## Candidate summary

A public redacted S073 inspection result contains:

- candidate and semantic hashes;
- channel, game, API, catalog, schema, locale, and tool versions;
- exact total candidate bytes;
- source count and acquisition method counts;
- entity, text, relation, coverage, and icon diff counts;
- ready and placeholder icon counts;
- warnings and compatibility; and
- source kind (`reviewed` or `collector-assisted`).

It contains no paths, localized text, source bytes, capture values, icon bytes,
account or character identity, or credentials.

The summary explicitly labels its result as integrity verification. S073 has no
signature or authenticated distribution authority, so origin remains a separate
user trust decision.

## Availability

`CatalogAvailability` contains one Live state, optional active and candidate
summaries, optional independent PTS preview, and check freshness.

Live state precedence is:

1. Unsupported schema;
2. Update ready to import;
3. Collector capture required;
4. New live data available;
5. Offline/stale check;
6. Catalog current.

PTS never participates in that precedence and is displayed separately.

## Update operation

An operation owns:

- monotonically unique in-process ID;
- source (`reviewed candidate`, `collector capture`, or `rollback`);
- current stage;
- optional exact progress `{completed,total,unit}`;
- stable detail code and redacted display text;
- cancellability and commit-boundary state;
- monotonic start time and elapsed duration; and
- terminal result.

Transitions are forward-only. `Complete`, `Cancelled`, and `Failed` are terminal.
The selection commit boundary spans verified first open through atomic pointer
replacement and durable receipt resolution.

## Update receipt

Each durable receipt contains schema version, generation, operation kind, old and new
targets, candidate and semantic hashes when known, channel, versions, source
counts, result, final stable stage, and redacted finding codes. It contains no
wall-clock-derived identity or local path. Receipt filenames use the selection
generation for successful selection changes and an operation sequence for
non-selection outcomes. If receipt persistence fails, the operation still
preserves selection and reports `receipt-write-failed` in memory.

## Collector observation

The waiting state stores only a private in-memory capture fingerprint observed
when instructions were shown: size, modification stamp when available, and
SHA-256. Import requires a later stable observation with changed content and a
complete S071 envelope. The fingerprint is not persisted or logged.
