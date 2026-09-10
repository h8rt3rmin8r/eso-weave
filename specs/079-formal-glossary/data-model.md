# Data Model: Formal Alphabetical Glossary

## GlossaryEntry

- `canonical`: unique visible H3 heading
- `aliases`: one or more visible phrases in the explicit Aliases field
- `definition`: substantive prose describing the ESO Weave meaning
- `related_targets`: one or more published Markdown links
- Invariant: canonical headings are globally unique and ordered case-insensitively
- Invariant: every S059 alias is contained in its canonical entry block

## LetterGroup

- `letter`: one uppercase Latin letter represented by an H2 heading
- `entries`: one or more GlossaryEntry records
- Invariant: groups and entries are strictly ascending
- Invariant: each entry canonical begins with the group letter

## AlphabetNavigation

- `label`: `Glossary alphabet`
- `letters`: ordered links to populated LetterGroup anchors
- Invariant: navigation and populated groups have an exact one-to-one mapping
- Invariant: no link targets an empty or absent group

## TerminologyContract

- `canonical`: established product, gameplay, platform, or development term
- `aliases`: required player or technical search phrases
- `target`: canonical published explanation
- Source: `docs/project/content-coverage.json`
- Invariant: each record maps to exactly one GlossaryEntry with its target link

## LegacyGlossaryInventory

- `terms`: canonical headings from the former definition and Search vocabulary lists
- `aliases`: phrases present beside those terms
- Invariant: every item remains visible after duplicate concepts are merged
