# Implementation Plan: Formal Alphabetical Glossary

**Branch**: `codex/s079-formal-glossary` | **Date**: 2026-09-10 | **Spec**: `spec.md`
**Input**: Feature specification for issue #120

## Summary

Replace the split bullet glossary with one alphabetical, heading-based reference.
Merge the legacy inventory with the complete S059 terminology map, add labeled
alphabet navigation and bounded responsive styling, and enforce the structure
through pure documentation-policy tests plus generated search evidence.

## Technical Context

**Language/Version**: Markdown, HTML5 fragments, CSS, Node.js ECMAScript modules
**Primary Dependencies**: mdBook 0.5.4, mdbook-linkcheck2 0.13.0, Node test runner
**Storage**: Repository Markdown and JSON terminology contract
**Testing**: Pure policy fixtures, repository policy, mdBook build/linkcheck, spelling
**Target Platform**: Public GitHub Pages and bundled offline documentation
**Performance Goals**: Static page and search indexing with no new runtime dependency
**Constraints**: UTF-8 without BOM, LF, no forbidden dash characters, offline parity,
accessible headings and focus, no page-level overflow at 320 CSS pixels
**Scale/Scope**: One glossary page, about sixty entries, one CSS component, one
pure validator, focused test fixtures, planning and changelog records

## Constitution Check

- Full spec-kit artifacts and blocking analysis precede implementation.
- The slice changes documentation and its policy only, with no runtime or addon authority.
- Existing canonical documentation and S059 terminology remain authoritative.
- Test-first policy fixtures precede glossary and validator implementation.
- Documentation, encoding, spelling, link, and repository gates remain mandatory.
- No workflow, toolchain, release, license, packaging, or Rust source changes are planned.

**Gate result**: PASS.

## Project Structure

```text
specs/079-formal-glossary/
  spec.md
  plan.md
  research.md
  data-model.md
  contracts/formal-glossary.md
  checklists/requirements.md
  checklists/accessibility-and-search.md
  quickstart.md
  tasks.md
  analysis.md

docs/src/reference/glossary.md
docs/theme/eso-weave.css
.github/scripts/docs-policy.mjs
.github/scripts/docs-policy.test.mjs
docs/project/build-plans/{README.md,plan-039.md}
CHANGELOG.md
```

**Structure Decision**: Keep the Markdown page as the single authored glossary.
Consume the established JSON map only during policy validation, and add no glossary
generator or client-side search layer.

## Implementation Phases

1. Freeze specification, clarification, checklists, design, tasks, and analysis.
2. Add failing pure policy tests for structure, ordering, aliases, navigation, and links.
3. Implement the pure glossary validator and wire it into repository policy.
4. Replace the two glossary lists with one complete alphabetical reference.
5. Add the bounded responsive navigation style and generated search evidence check.
6. Update the active chronological build plan and changelog.
7. Run post-implementation analysis and all documentation and repository gates.
8. Publish, process at most two review rounds, and stop for the merge ritual.

## Design Decisions

1. Use H2 letters and H3 terms. Native headings provide stable mdBook anchors,
   search weight, and screen-reader navigation without extra scripting.
2. Keep aliases as labeled visible prose. Hidden keywords would violate the S059
   search contract and would be less useful to readers.
3. Include every S059 term, not merely the smaller legacy page inventory. This
   prevents the formal glossary from remaining incomplete at publication.
4. Link each mapped entry to its canonical S059 target. The glossary is a concise
   reference, while feature and concept pages retain full explanations.
5. Show only populated alphabet letters. Dead letters add focus stops without value.
6. Parse a deliberately narrow Markdown grammar in policy. The validator protects
   reviewable source structure without introducing a general Markdown parser.

## Complexity Tracking

No constitution violation or complexity exception is required.

## Post-Design Constitution Check

The design remains static, offline-capable, accessible, deterministic, and fully
reviewable. It adds no product authority or external dependency. Gate remains PASS.
