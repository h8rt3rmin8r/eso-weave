# Research: Compact Work-Slice References

## Inventory

- Published `docs/src` contains 44 canonical `S###` references.
- The layout risk is concentrated in 31 occurrences of 14 unique long Rust test symbols across Coverage Matrix, Test Strategy, and State Machines.
- Twenty-two of those occurrences are in tables and nine are in narrative evidence lists.
- `s069-v1` appears twice and is a real case-sensitive persisted algorithm identifier, not work-slice provenance.
- Exact affected test anchors already live in source, and most relationships are represented by stable labels in `docs/project/content-coverage.json`.

## Decisions

### Published form

Use `S###` as the only visible work-slice form. Replace redundant phrases such as `Build slice S012` with the compact token.

### Evidence preservation

Replace long test symbols with behavioral summaries. Link `S###` labels to matching specs and evidence descriptions to relevant GitHub source files. Preserve exact symbols in source and the unpublished content-coverage manifest.

### Policy boundary

Scan every Markdown file under `docs/src` and every generated HTML file. Markdown scanning includes inline code and link labels but masks fenced code, comments, link destinations, and HTML tags. HTML scanning includes inline-code text but removes `pre`, script, style, comments, and tags.

### Narrow exception

Allow only the exact persisted identifier `s069-v1`, and only in Architecture and Encounter Data and Metrics. A generic lowercase-version exception would let malformed provenance escape policy.

### Responsive proof

Reject every visible slice-shaped token longer than the four-character `S###` form and validate generated output. General table presentation remains in issue #127.

## Alternatives Rejected

- Hiding all inline code from policy would miss every current long-symbol offender.
- Publishing a second exhaustive test-symbol index would duplicate unstable implementation detail and preserve the layout risk.
- Editing the frozen content-coverage manifest would expand scope without improving the reader-facing contract.
- Treating every `s###-...` value as an exception would be broader than the known algorithm requirement.
