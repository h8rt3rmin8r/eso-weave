# S073 Data Model

## Pipeline request

The strict request contains:

- schema version and mode;
- version tuple: channel, game, API, catalog, supported schema, locales, tool;
- input source ID;
- source-policy path relative to the workspace;
- source pins;
- network and stale-cache decisions;
- optional same-channel baseline relative path;
- optional local icon source relative path;
- review thresholds.

Paths are execution inputs only. They never enter candidate reports.

## Source pin and inventory

A source pin has an ID, role (`normalized-bundle`, `collector-capture`, or
`provenance`), channel, game/API versions, revision, expected SHA-256, maximum
bytes, URI, optional local relative path, license scope, and redistribution
class. Exactly one byte location is active.

After acquisition, the source inventory retains all non-path identity fields
plus a stable result (`local`, `pinned-remote`, `network-refresh`, or
`stale-cache`), actual byte count, and verified SHA-256. Cold and warm cache
paths use the same `pinned-remote` value when the request does not refresh, so
cache temperature cannot change candidate identity. It does not retain a
filesystem path.

Every normalized bundle source snapshot must match an acquired source's full
channel, version, locale, revision, hash, license scope, and redistribution
identity. Non-collector snapshots also match the source ID and canonical URI.
Collector snapshots use the importer's fixed `user-local-savedvariables` URI
and must retain `user-generated-local-only` / `user-generated-only` rights.

New source and icon cache objects remain inside the run staging directory until
the complete candidate has passed verification and installation. Failed
parsing, validation, diff, threshold, icon, candidate checks, or final install
therefore publish neither source entries nor icon generations.

## Version tuple

```text
channel + game_version + api_version + catalog_version
        + catalog_schema + ordered locales + tool_version
```

The request, normalized bundle or capture, compiled catalog, icon receipt, and
candidate manifest must agree. Channel has no ordering or promotion operation.

## Candidate artifacts

```text
candidates/
└── <channel>/
    └── <manifest-sha256>/
        ├── catalog.sqlite
        ├── build-report.json
        ├── verify-report.json
        ├── diff.json
        ├── sources.json
        ├── validation.json
        ├── icons.json
        ├── checksums.json
        └── manifest.json
```

The manifest contains schema version, candidate state, version tuple, source
inventory hash, catalog semantic hash, icon generation identity, baseline
semantic hash when present, review thresholds, and an ordered artifact list.
The manifest does not hash itself. Its canonical SHA-256 names the directory.

## Validation finding

- stable code;
- stage name;
- severity (`warning` or `blocking`);
- redacted message.

Any blocking finding prevents publication. Successful candidates may carry
warnings, including one stable `stale-cache-reuse:<source-id>` finding for every
explicit stale-cache reuse and placeholder-only icons.

## State transitions

```text
requested -> validated -> acquired -> normalized -> built -> verified
          -> diffed -> icons-resolved -> policy-checked -> candidate-published
```

Any transition may move to `failed`. There is no transition from candidate to
accepted, active, committed, released, or promoted in S073.

## Invariants

1. Every source is verified before interpretation.
2. Every catalog and baseline channel equals the request channel.
3. Every candidate artifact is allowlisted and hash verified.
4. Every output identity is derived from canonical content, not local paths.
5. Source, capture, normalized input, and user icon bytes are never candidates.
6. Existing cache objects and candidates are never replaced.
7. Live and PTS directories and identities are always distinct.
