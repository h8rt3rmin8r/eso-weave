# Contract: Published Work-Slice References

## Canonical form

Visible work-slice provenance matches `S[0-9]{3}` exactly.

## Rejected visible forms

- Lowercase or incorrectly padded numeric forms
- Numeric `build slice` or `work slice` phrases
- Concrete `specs/NNN-name` paths
- Slice-prefixed implementation symbols such as `s060_long_test_name`

## Allowed boundaries

- Canonical labels such as `S060`
- Canonical link labels whose destination contains a concrete spec path
- Literal content in fenced code samples
- Values inside Markdown link destinations or HTML attributes
- The exact persisted algorithm identifier `s069-v1` on its two documented pages

## Evidence navigation

A normalized reference identifies behavior in reader language, includes compact slice provenance when relevant, and links to the appropriate repository specification or source file. Exact symbols remain at the destination and in unpublished evidence records.

## Presentation

No visible work-slice token may exceed the four-character canonical form. Generated-site checks apply the same rule to rendered text. General table responsiveness remains a separate work slice.
