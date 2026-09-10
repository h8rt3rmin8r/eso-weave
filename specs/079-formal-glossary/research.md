# Research: Formal Alphabetical Glossary

## Existing authorities

- `docs/src/reference/glossary.md` has fourteen definition bullets and forty-two
  Search vocabulary bullets, including duplicate concepts and sparse aliases.
- `docs/project/content-coverage.json` is the S059 terminology authority for
  canonical terms, required aliases, and published destinations.
- mdBook 0.5.4 derives stable section anchors and indexes visible heading and body
  text into the same public and offline HTML output.
- The existing theme already supplies global keyboard focus visibility and narrow
  viewport wrapping. Only a bounded alphabet-navigation layout rule is needed.
- The documentation policy already validates S059 alias presence and canonical
  destinations, but it does not enforce a formal glossary structure or ordering.

## Alternatives evaluated

### Raw HTML definition list

Rejected. A `dl` is semantically attractive, but raw HTML would make Markdown link
handling and source policy parsing more fragile. It also gives each term no native
mdBook heading anchor unless extra identifiers are maintained separately.

### One Markdown table

Rejected. The alias and definition text is long, so a four-column table would
recreate the narrow-width collapse that issue #127 separately addresses. Tables
also make heading-based scanning and direct term links harder.

### Generated glossary from JSON

Rejected. Generation would create a second authored source, another build step,
and review indirection for a modest reference page. The current Markdown can be
both the human authority and the generated-site input.

## Selected approach

Use a labeled HTML `nav` containing simple anchor links, then native Markdown H2
letter groups and H3 canonical terms. Each entry uses an explicit `Aliases:` line,
a definition paragraph, and a `Related:` line with Markdown links. Extend the
documentation policy with a pure glossary validator and focused rejection tests.
The validator consumes the existing terminology map so a later map change must
update the glossary in the same review.

## Verification strategy

- Pure policy fixtures reject missing aliases, duplicate or unsorted terms,
  mismatched letter navigation, missing related targets, and the old Search
  vocabulary heading.
- Repository validation checks the real glossary against the S059 map.
- mdBook build and linkcheck validate anchors and destinations.
- Generated-site policy and an explicit search-index check confirm visible terms
  survive rendering and indexing.
- Theme inspection at narrow and wide widths confirms wrapping and focus behavior.
