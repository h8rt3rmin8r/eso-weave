# Research: Documentation Landing Identity

## Decision 1: Preserve repository metadata as the authority

**Decision**: Read package name, version, and repository from `Cargo.toml`. Read
the release date from the dated `CHANGELOG.md` heading for that exact version.

**Rationale**: These sources already govern package and release behavior. A policy
comparison lets authored Markdown remain portable while preventing silent drift.

**Alternatives considered**:

- Add a second JSON metadata file. Rejected because it creates another authority.
- Query GitHub at page load. Rejected because bundled documentation is offline and
  a remote response would make the page nondeterministic.
- Generate the page in CI only. Rejected because local mdBook and bundled builds
  must produce the same source-controlled result.

## Decision 2: Use the existing approved PNG byte-for-byte

**Decision**: Copy `assets/eso-weave-banner.png` into
`docs/src/assets/brand/eso-weave-banner.png` without resizing or re-encoding.

**Rationale**: The approved asset is a crisp 2000 by 650 full-color wordmark and
only 73,763 bytes. CSS can constrain it without degrading source quality.

**Alternatives considered**:

- Use the existing square SVG mark. Rejected by issue #121.
- Use a monochrome raster. Rejected because it loses the approved color identity.
- Create a new derivative. Rejected because no transformation is necessary and
  broader asset work belongs to issue #122.

## Decision 3: Separate visible and accessible heading text

**Decision**: Place the banner first, use an empty image alternative, then render
a Markdown H1 whose ESO Weave span is visually hidden and whose Documentation
text remains visible.

**Rationale**: The visual banner supplies the product name, while the accessible
H1 still reads ESO Weave Documentation. The empty image alternative prevents the
same words from being announced twice.

**Alternatives considered**:

- Announce the banner and use a Documentation-only H1. Rejected because the H1
  would lose the full page identity.
- Keep the full visible H1. Rejected because it visibly repeats the wordmark.
- Use a raw HTML H1. Rejected because the existing source policy requires a
  Markdown level-one heading and there is no need to weaken it.

## Decision 4: Use a semantic definition list for metadata

**Decision**: Render a labeled `dl` with handle, applies-to version, release date,
and repository entries, followed by a visible build-time snapshot explanation.

**Rationale**: A definition list preserves label-value relationships for assistive
technology and adapts from a compact grid to one column at narrow widths.

**Alternatives considered**:

- Use a table. Rejected because issue #127 owns table responsiveness and the
  content is naturally a short set of definitions.
- Use badges. Rejected because image badges complicate accessibility and offline use.
- Use unlabeled prose. Rejected because readers cannot scan applicability quickly.

## Decision 5: Validate source authorities and generated delivery

**Decision**: Add one pure source validator plus generated HTML assertions to the
existing documentation policy.

**Rationale**: Pure tests cover mutation cases cheaply. Repository execution reads
the real authorities and built output, proving both authored alignment and final
asset delivery.

## Verification implications

- Hash or byte comparison proves the docs banner is the approved source asset.
- Exact extracted values prove static metadata alignment.
- Generated HTML must contain the local banner path, semantic metadata, visible
  snapshot disclosure, and no remote image dependency.
- CSS must constrain the image and collapse metadata at the narrow breakpoint.
