# Data Model: Guided Documentation Screenshots

## Screenshot Record

| Field | Contract |
| --- | --- |
| `id` | Stable lowercase hyphenated identifier |
| `kind` | `deterministic-app`, `maintainer-supplied`, or `synthetic-illustration` |
| `destination` | Repository-relative path below `docs/src/assets/` |
| `pages` | Non-empty ordered list of published Markdown pages |
| `reader_question` | One concrete reason the image exists |
| `alt` | Meaningful alternative text used at every reference |
| `caption` | Adjacent explanation that does not rely on color alone |
| `width`, `height` | Intrinsic pixel dimensions for PNG, SVG view-box dimensions for illustration |
| `bytes` | Exact file size |
| `sha256` | Lowercase digest of the published bytes |
| `source` | Source contract appropriate to `kind`, including crop rectangle for deterministic captures |
| `update_trigger` | Observable reason to refresh or review the image |

## Deterministic Source

Adds S083 generator name, scene identifier, theme, viewport, generated filename, source dimensions, source receipt digest, and lossless crop rectangle. The published digest names the cropped bytes.

## Maintainer-Supplied Source

Adds a stable source label and original digest. It intentionally omits the temporary local filesystem path.

## Synthetic Source

Adds an authorship label and a mandatory `synthetic: true` flag. The SVG must contain a visible synthetic-example label and accessible title and description.

## Invariants

- IDs and destinations are unique.
- Every destination is local, versioned, and beneath `docs/src/assets/`.
- Every referenced page exists and contains the exact destination and alternative text.
- Every digest, size, and dimension matches the checked-in bytes.
- Exactly seven records are deterministic application PNGs, one is maintainer supplied, and one is synthetic.
- No record includes a user-profile path, live character data, current machine identifier, or external URL.
