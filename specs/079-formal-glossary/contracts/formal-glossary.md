# Contract: Formal Glossary Markdown

## Page structure

The source has this ordered shape:

```text
# Glossary
introductory prose
<nav aria-label="Glossary alphabet"> populated letter links </nav>
## A
### API Version
**Aliases:** ...
definition prose
**Related:** [descriptive page](relative-target.md)
```

The pattern repeats for every populated letter and canonical term.

## Entry rules

- H2 headings contain one uppercase letter.
- H3 headings contain a canonical term and occur exactly once.
- Letter groups and their entries are sorted with locale-independent,
  case-insensitive ordering.
- The `Aliases:` line is visible prose and contains every contracted alias.
- The definition is non-empty substantive prose specific to ESO Weave.
- The `Related:` line contains the S059 canonical target where one exists.
- Glossary-only legacy terms link to the closest authoritative published page.
- No `Search vocabulary` heading or second alias-only collection is allowed.

## Navigation rules

- One `nav` element has `aria-label="Glossary alphabet"`.
- Navigation links use the same ascending letters as populated H2 groups.
- Every link points to the mdBook anchor for its H2 letter.
- The navigation container wraps and every anchor retains visible keyboard focus.

## Validation failures

Policy validation fails for duplicate or unsorted terms, wrong letter groups,
missing required aliases, missing or incorrect related targets, dead or omitted
alphabet links, unlabeled navigation, empty definitions, or the former split list.
