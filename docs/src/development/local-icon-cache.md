# Local Icon Cache

S072 provides a library boundary for resolving catalog virtual icon paths from
an explicitly selected user-owned directory. It does not search an ESO install,
extract archives, download files, or activate a cache in the application. The
distributable catalog continues to contain references and a project-created
placeholder, never game image bytes.

## Build contract

Call `build_generation` with a source root, an application-data cache root, the
verified catalog semantic SHA-256, and the exact virtual paths needed by the
caller. A source tree mirrors those virtual paths below its root. Resolution is
ASCII case-insensitive by component and fails closed on ambiguous matches,
traversal, links or reparse points, non-files, unsafe root aliases, and excessive
directory fanout.

Only PNG and DDS are accepted. Each source is limited to 8 MiB, each dimension
to 1024 pixels, and decoded area to 1,048,576 pixels. DDS input must contain one
two-dimensional array layer and at most 16 declared mip levels. Only mip level
zero is decoded. The locked decoder excludes its encoding and native ISPC
features.

Decoded pixels become deterministic RGBA PNG objects. Source files are read
only and remain unchanged. Unsupported, missing, inaccessible, corrupt, or
oversized input maps to the same neutral 64 by 64 project placeholder with a
typed reason.

## Immutable generations

Objects are published without replacement at
`objects/<content-sha256>.png`. A canonical manifest binds every normalized
virtual path to exactly one object and records the catalog semantic hash,
transformation identity, source hash where available, dimensions, origin,
availability, redistribution state, and fallback reason. It contains no source
or cache root.

The manifest hash names `generations/<manifest-sha256>/manifest.json`. A
generation becomes visible only after its complete candidate and every object
have passed verification. Opening a generation rechecks the manifest identity,
schema, ordering, provenance, file types, object hashes, dimensions, and PNG
decodability. Link-like cache directories and files are rejected.

There is no mutable `current` pointer, cleanup policy, or implicit application
selection in S072. Later update orchestration may select a verified generation
without weakening its immutability.

## Size and clearing

Each generation accepts at most 500,000 references and a 64 MiB manifest. Each
unique transformed object is stored once across generations. S072 does not set
a total cache quota or delete unreferenced objects because it has no activation
authority and cannot know which generation a later caller retained.

To clear test or direct-library output, first stop every consumer, preserve any
generation needed for diagnosis, then remove only the exact cache root that was
supplied to `build_generation`. Never point that cache root at the source tree;
the builder rejects equal or nested roots. Source assets are outside the cache
and are not removed by this procedure.

## Rights and privacy boundary

Ready objects are marked `user-local-only`; placeholder objects are marked
`allowed`. User-local objects and manifests stay outside source control,
packages, documentation assets, and releases. No path in this workflow performs
a network request or sends local data elsewhere.

Use only files you are entitled to use locally. The project decision remains to
ship references and generic placeholders while leaving asset acquisition and
possession with the end user.

## Verification

The synthetic suite creates its own PNG and DDS bytes and proves deterministic
output, deduplication, explicit fallback mappings, immutable publication,
source preservation, manifest privacy, and hostile path and cache rejection:

```console
cargo test --locked --test icon_cache
```
