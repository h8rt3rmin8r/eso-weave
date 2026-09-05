# Data Model: Concise Release Notes

## VersionSection

The complete changelog record selected by exact bracketed version heading.

| Field | Type | Rules |
|---|---|---|
| version | semantic version string | Exact match without the leading `v` |
| heading | Markdown level-two heading | `## [X.Y.Z]` with an optional date suffix |
| subsections | ordered Markdown blocks | May include Highlights, Added, Changed, Fixed, Removed, Security, and Decisions |

### Invariants

- The version section must exist and contain non-whitespace detail.
- Detailed subsections are preserved without a size limit.
- Only one Highlights subsection is valid for release generation.

## HighlightsExcerpt

The release-facing summary nested in a VersionSection.

| Field | Type | Rules |
|---|---|---|
| items | ordered top-level Markdown bullets | One through six |
| content | Markdown | Preserved exactly apart from outer blank lines |
| wordCount | nonnegative integer | At most 120 |

### Invariants

- Every excerpt begins with a top-level bullet.
- Nested and wrapped lines belong to their preceding top-level bullet.
- A level-three heading terminates the excerpt.
- Empty or oversized excerpts are invalid.

## GeneratedReleaseNotes

| Field | Type | Rules |
|---|---|---|
| highlights | HighlightsExcerpt | Required |
| changelogUrl | HTTPS URL | Repository and tag specific |

### Composition

```text
HighlightsExcerpt
blank line
[Read the full changelog for vX.Y.Z](tag-specific CHANGELOG.md URL)
```

## ValidationResult

| State | Meaning |
|---|---|
| valid | Output is emitted to stdout |
| invalid-version | Version input is malformed or absent from the changelog |
| invalid-repository | Repository slug is not `owner/name` |
| missing-highlights | Version section lacks the required subsection |
| empty-highlights | Subsection has no content |
| item-budget-exceeded | More than six top-level bullets |
| word-budget-exceeded | More than 120 words |

Invalid results emit a specific diagnostic to stderr and return nonzero without partial release-note output.
