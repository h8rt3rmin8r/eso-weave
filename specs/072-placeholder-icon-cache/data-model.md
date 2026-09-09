# Data Model: Placeholder-First Local Icon Cache

## `IconCacheRequest`

- `source_root`: existing user-selected directory
- `cache_root`: user-local cache directory, distinct from `source_root`
- `catalog_semantic_sha256`: 64 lowercase hexadecimal characters
- `references`: canonical virtual paths selected from one catalog

The request is process-local and is never serialized, so roots cannot leak into
the manifest.

## `NormalizedIconPath`

- `canonical`: source spelling retained in the manifest
- `lookup_key`: slash-separated ASCII-lowercase relative path
- `components`: validated path components used for local resolution

The leading game slash is removed for filesystem lookup. Drive prefixes,
backslashes, empty segments, `.` and `..`, NUL, and platform prefixes are
invalid.

## `IconCacheManifest`

- `schema_version`: `1`
- `catalog_semantic_sha256`: catalog generation authority
- `transformation_id`: `rgba8-png-v1`
- `placeholder_sha256`: project-owned fallback object
- `entries`: sorted unique `IconCacheEntry` array

Canonical pretty JSON with LF and a trailing newline is hashed to create the
generation identity. The identity is carried in the directory name and receipt,
not recursively inside the manifest.

## `IconCacheEntry`

- `canonical_path`: original valid catalog spelling
- `lookup_key`: normalized case-insensitive key
- `object_sha256`: transformed PNG object
- `source_sha256`: optional local source-byte hash
- `width`, `height`: transformed dimensions
- `media_type`: `image/png`
- `origin`: `user-supplied` or `project-placeholder`
- `availability`: `ready` or `placeholder`
- `redistribution`: `user-local-only` or `allowed`
- `fallback_reason`: optional `missing`, `unsupported`, `invalid`,
  `permission-denied`, or `io-failed`

No field contains the local source or cache root.

## Object layout

```text
cache-root/
├── objects/
│   └── <sha256>.png
└── generations/
    └── <manifest-sha256>/
        └── manifest.json
```

Objects and generations are immutable. Existing content is verified before
reuse. Candidate directories use a temporary random name but random values do
not enter the manifest.

## `IconCacheGeneration`

Loaded and verified view of one generation:

- manifest and generation hash
- canonical path index
- cache root used only to produce local object paths

Lookup returns a typed state and object path. A loaded generation never mutates
disk.

## State transitions

```text
Reference
  -> valid local source -> Ready
  -> absent source -> Placeholder(Missing)
  -> unsupported extension/format -> Placeholder(Unsupported)
  -> invalid or corrupt input -> Placeholder(Invalid)
  -> permission failure -> Placeholder(PermissionDenied)
  -> other source I/O failure -> Placeholder(IoFailed)

Candidate
  -> objects complete -> manifest complete -> verified -> immutable generation
  -> destination failure -> rejected, prior generations unchanged
```

## Invariants

1. Every manifest lookup key is unique.
2. Every manifest object exists and hashes to its filename.
3. Every object decodes as one bounded PNG with recorded dimensions.
4. User-supplied entries are never redistribution-allowed.
5. Placeholder entries all map to the declared project placeholder hash.
6. Generation directory name equals the manifest byte hash.
7. Roots and personal identifiers are absent from serialized data.
