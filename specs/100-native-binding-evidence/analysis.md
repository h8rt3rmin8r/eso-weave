# Spec-Kit Analysis: Native ESO Binding Evidence

**Date**: 2026-09-15

**Artifacts analyzed**: GitHub issues #188 and #206 through #208, `spec.md`, `research.md`, `data-model.md`, `contracts/binding-evidence.md`, `plan.md`, `quickstart.md`, `checklists/requirements.md`, and `tasks.md`.

## Findings

### A1. Parent scope was not independently closable

**Resolution**: Parent #188 was converted to an epic and decomposed into #206 for evidence, #207 for combat-controller consumption, and #208 for Fishing, Auto Potion, settings, and documentation cleanup. S100 closes only #206.

### A2. Choosing one of multiple ESO slots would invent authority

**Resolution**: The contract inspects all advertised slots, deduplicates exact normalized chords, and reports conflicting whenever more than one distinct assignment remains.

### A3. Raw one-byte control colors were unsafe under tolerance

**Resolution**: The contract expands each control nibble by 17 across red and green. This preserves bounded color tolerance without allowing adjacent portable controls to decode as one another.

### A4. Shared binding markers could permit action transposition

**Resolution**: Blue carries an action-and-control check nibble. A cell rendered for one action fails in every other action position.

### A5. Packed blue tolerance could alter modifier authority

**Resolution**: Blue is validated exactly. Color drift can only make evidence unavailable; it cannot authorize a neighboring modifier set or action checksum.

### A6. Protocol count compatibility needed an explicit freeze

**Resolution**: Version 5 is frozen at 29 blocks. Version 6 alone samples B29 through B39 and every older layout returns an all-unavailable binding set.

### A7. Portable discovery and platform synthesis were at risk of coupling

**Resolution**: S100 defines evidence types but leaves existing `Key`, `MouseButton`, controllers, settings, and synthesis untouched. Platform mapping and held-modifier ownership stay in #207.

### A8. Read-only intent lacked permanent enforcement

**Resolution**: Static tests reject the complete prohibited mutation and persistence surface, with injected fixtures proving each rule.

## Coverage and consistency

- Every requirement maps to one or more tasks and measurable outcomes.
- All eleven actions appear in the same documented order in the issue, spec, model, contract, plan, and tasks.
- All five states have explicit publisher and decoder semantics.
- Test-first, full CI parity, addon minimalism, fixed-size transport, and pinned-workflow decision requirements are represented.
- No unresolved clarification marker, placeholder, or scope contradiction remains.

## Gate result

PASS. The S100 specification package is complete, internally consistent, bounded to issue #206, and ready for test-first implementation.
