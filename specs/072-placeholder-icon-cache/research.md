# Research: Placeholder-First Local Icon Cache

All choices below were resolved under the autopilot policy. No unresolved
clarification remains.

## Decision 1: Keep asset bytes outside the catalog

**Decision**: Preserve catalog virtual paths as source metadata and create a
separate immutable cache manifest that explicitly maps each path to one object
hash.

**Rationale**: S070 has independent `icon_reference` and `icon_asset` tables but
no reference-to-asset join. User-local bytes also have a different ownership
and deletion lifecycle from the shipped database. A cache manifest fixes the
mapping without making local files redistributable catalog content.

**Rejected alternatives**:

- Filename inference remains ambiguous under aliases, case changes, and
  deduplicated content.
- A catalog schema migration would couple user-local cache availability to an
  immutable distributable artifact.
- SQLite BLOB storage would enlarge catalog updates and mix third-party bytes
  with source metadata.

## Decision 2: Enable only a user-supplied directory

**Decision**: Resolve selected references component by component beneath one
explicit user root. Scan only the named path's containing directories, reject
case ambiguity and link-like components, and never recurse the root.

**Rationale**: This gives useful local behavior while honoring the S068 rights
decision. It also avoids full archive extraction and limits work to references
the selected catalog already contains.

**Rejected alternatives**:

- Remote mirrors lack an approved data and redistribution contract.
- Installed archive extraction has no approved bounded method in current scope.
- Full directory indexing reads unrelated user content and scales with the
  directory rather than selected references.

## Decision 3: Decode DDS with a safe, decode-only dependency

**Decision**: Use `image_dds` 0.7 with `default-features = false` and only its
`ddsfile` and `image` features. Parse the DDS header, reject arrays, volume
textures, excessive mip counts, dimensions, and bytes, then decode only mip 0.

**Rationale**: The project already uses `image` for PNG. The image-rs
`image_dds` project documents safe DDS decoding across compressed and
uncompressed formats. Its documentation specifically advises disabling the
default `encode` feature when encoding is not needed because the encoder adds a
native ISPC toolchain that is not available on every target.

**Primary references**:

- [image_dds crate documentation](https://docs.rs/image_dds/0.7.2/image_dds/)
- [image_dds source and license](https://github.com/image-rs/image-dds)
- [ddsfile structure documentation](https://docs.rs/ddsfile/0.5.2/ddsfile/struct.Dds.html)

**Rejected alternatives**:

- A handwritten BC decoder expands unsafe parser and format surface area.
- An external converter executes another binary and complicates containment.
- Enabling encode support adds unnecessary platform-specific build risk.

PNG input uses the locked `image` decoder's explicit APNG inspection and
rejects animation before the default frame can be treated as a static icon.
Source acquisition opens one stable no-follow file handle, verifies the
handle's final path and metadata, and reads through a hard byte cap. Linux
resolves the open descriptor through `/proc/self/fd`; Windows uses
`GetFinalPathNameByHandleW`. This closes the path-check/read race without an
archive or recursive discovery dependency.

## Decision 4: Canonical deterministic PNG transformation

**Decision**: Convert each accepted source to one RGBA8 surface and encode with
explicit PNG compression and filter settings under transformation identifier
`rgba8-png-v1`.

**Rationale**: One output representation makes hashing, deduplication, egui
loading, and verification uniform. Explicit settings and locked dependencies
make repeated output byte-stable.

**Rejected alternatives**:

- Copying valid PNG bytes preserves metadata and prevents cross-format content
  deduplication.
- WebP would add a new decoder path and packaging surface.
- Resizing would manufacture visual differences and is unnecessary for the
  first bounded resolver.

## Decision 5: Publish immutable generations

**Decision**: Write shared objects by transformed hash, build a complete
canonical manifest in a temporary generation directory beneath the cache root,
verify it, then rename the directory to its manifest SHA-256. Existing matching
objects and generations are verified and reused, never replaced.

Manifest and object verification reads use the same stable no-follow,
root-confined, bounded handle primitive as source acquisition. This prevents a
metadata/read swap from redirecting cache verification and prevents concurrent
growth from allocating beyond the manifest or object limit.

**Rationale**: Readers can open one immutable generation without observing a
partially written manifest. A failed candidate may leave an unreferenced object
but cannot damage a prior generation. Cleanup remains explicit later work.

**Rejected alternatives**:

- Replacing a mutable `current` directory is not atomic across supported
  platforms.
- Deleting and rebuilding in place risks destroying the last good cache.
- Automatic garbage collection would add deletion authority beyond this issue.

## Decision 6: Generate one project-owned placeholder

**Decision**: Generate a neutral 64 by 64 RGBA image from project code and
store it through the same transformation and object path as local inputs.

**Rationale**: No binary game asset enters the repository, the fallback is
accessible and deterministic, and every manifest entry still has a real object.

**Rejected alternatives**:

- Per-entity placeholders could imply unsupported identity or type knowledge.
- An empty or transparent image is difficult to distinguish from loading or
  rendering failure.
