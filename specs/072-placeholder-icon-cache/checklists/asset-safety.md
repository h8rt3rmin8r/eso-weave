# Asset and Cache Safety Checklist

**Purpose**: Protect local files, bounded decoding, and redistribution policy.

## Acquisition

- [x] Only a user-selected local directory is enabled.
- [x] Lookup requests come from selected catalog references.
- [x] Network and archive extraction paths are absent.
- [x] Symlinks, reparse points, traversal, and aliases are rejected.
- [x] User originals are never changed or deleted.

## Decode and transform

- [x] Input bytes, dimensions, pixels, frames, layers, and depth are bounded.
- [x] DDS encoding support and native compiler dependencies are disabled.
- [x] Output is one deterministic RGBA PNG transformation.
- [x] Corrupt and unsupported files degrade safely.
- [x] Fixtures are synthetic and project-created.

## Publication and provenance

- [x] Each canonical reference maps explicitly to an object hash.
- [x] Candidate generations become visible only when complete.
- [x] Published generation content is immutable and content-addressed.
- [x] Manifests disclose no personal path.
- [x] Placeholder and user-local redistribution classes remain distinct.
- [x] No third-party image byte enters a repository or package artifact.
