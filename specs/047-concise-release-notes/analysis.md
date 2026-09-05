# Analysis: Concise Release Notes

## Coverage matrix

| Requirement | Design artifact | Implementation evidence | Validation |
|---|---|---|---|
| FR-001 to FR-003 | research decisions 1 and 3, release-notes contract | `CHANGELOG.md` structure and `scripts/release-notes.sh` exact parser | valid extraction and exclusion fixtures |
| FR-004 | research decision 2, HighlightsExcerpt model | item and word checks in `scripts/release-notes.sh` | six-item, 120-word, and overflow tests |
| FR-005 | research decision 4, generated-note model | tag-specific Markdown link emitted by the generator | exact-output test |
| FR-006 | ValidationResult model | fail-closed generator diagnostics | invalid fixture matrix |
| FR-007 to FR-008 | workflow integration contract | `.github/workflows/release.yml` verify and assembly steps | YAML review and hosted CI |
| FR-009 | specification test matrix | `scripts/release-notes.test.sh` | local and hosted execution |
| FR-010 | quickstart | `docs/releasing.md` | documentation review |
| FR-011 to FR-012 | specification and pinned-artifact gate | Unreleased Highlights and dated `CHANGELOG.md` decision | generator preview and changelog review |
| FR-013 | scope boundary | unchanged Cargo version and no release tag | final Git and GitHub audit |

## Cross-artifact consistency

- The specification, research, model, contract, and quickstart all set the same one-to-six item and 120-word limits.
- The workflow design preserves full changelog verification and changes only release-body presentation.
- Every generated link uses the exact selected version tag, never `main`.
- Issue #51 closes on merge; v0.12.0 publication remains a separate post-merge operation.
- No clarification marker remains.

## Risk review

### Replacing one oversized format with another

Risk: maintainers reproduce the full changelog inside Highlights.

Control: objective item and word ceilings run in pull-request CI and at tag verification.

### Silently incomplete release history

Risk: consuming only Highlights might weaken the full changelog requirement.

Control: `changelog-section.sh` remains the first tag-time integrity gate. The new generator is an additional presentation gate.

### Mutable documentation link

Risk: linking to `main` shows details that do not match an older binary.

Control: the generated URL uses the release's own `vX.Y.Z` tag.

### Parser ambiguity

Risk: loose heading matching captures another version or subsection.

Control: compare bracketed version keys exactly, recognize a literal level-three Highlights heading, and stop at the next level-two or level-three heading.

### Unsupported browser-layout claim

Risk: a test claims assets are above the fold for every viewport.

Control: enforce repository-controlled prose limits and avoid pixel claims about GitHub's responsive interface.

## Implementation conclusion

The artifacts and implementation are mutually consistent. The contract suite passes against LF and CRLF fixtures, exact boundary values, malformed inputs, missing sections, and over-budget excerpts. The generated v0.12.0 candidate contains only five Highlights bullets and the immutable full-changelog link. No product behavior, permission, release trigger, asset, Cargo version, or tag changed. No critical conflict or unresolved finding remains.
