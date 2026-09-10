# Glossary Accessibility and Search Checklist

**Purpose**: Protect semantic navigation and vocabulary discovery
**Created**: 2026-09-10
**Feature**: [spec.md](../spec.md)

## Structure

- [x] One formal reference replaces the duplicate two-list structure
- [x] Letter groups and canonical terms use a consistent heading hierarchy
- [x] Every canonical term appears once and in alphabetical order
- [x] Aliases, definitions, and related pages have explicit labels or roles

## Navigation and layout

- [x] Alphabet navigation has an accessible label
- [x] Every navigation target exists and every populated letter is linked
- [x] Links remain keyboard focusable and retain the global focus indicator
- [x] Navigation wraps without page-level overflow at narrow width

## Search and meaning

- [x] Every S059 canonical term and alias is visible in its glossary entry
- [x] Former page terms and aliases remain represented
- [x] Player slang and technical vocabulary receive equivalent entry structure
- [x] Ambiguous state vocabulary retains its precise ESO Weave meaning

## Notes

The heading-based contract uses mdBook's native anchors and indexing. No custom
client-side search or accessibility runtime is required.
