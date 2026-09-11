# Data Model: Compact Work-Slice References

## WorkSliceReference

- `label`: visible token matching `S[0-9]{3}`
- `spec_url`: optional repository URL whose visible label remains compact
- `context`: prose, table, caption, alternative text, link label, or inline code

## EvidenceReference

- `description`: behavioral summary suitable for reader-facing content
- `slice`: related `WorkSliceReference`
- `source_url`: repository URL for the relevant source or test file
- `exact_anchor`: retained only in canonical source or unpublished project evidence

## ValidationFinding

- `document`: source or generated page
- `kind`: malformed slice, expanded phrase, concrete spec path, or long symbol
- `token`: offending visible text
- `location`: line number when source text provides one

## Exception

- `kind`: fenced literal, hidden destination, HTML attribute, or persisted identifier
- `scope`: smallest syntax region that policy masks
- `identifier`: `s069-v1` only for the persisted-identifier case
- `pages`: Architecture and Encounter Data and Metrics for the persisted-identifier case

## Relationships

One work slice may support multiple evidence references. Each evidence reference links to one source file and retains its exact implementation anchor outside published reader prose.
