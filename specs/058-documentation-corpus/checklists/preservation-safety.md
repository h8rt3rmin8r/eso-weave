# Preservation Safety Checklist: Documentation Corpus Reorganization

**Purpose**: Prevent loss of authoritative content while retiring legacy sources.

**Created**: 2026-09-07

**Feature**: [spec.md](../spec.md)

## Baseline

- [x] CHK001 All 49 baseline `docs` files and the website article are frozen in the ledger.
- [x] CHK002 All 20 technical-specification H2 units are frozen in source order.
- [x] CHK003 All 27 legacy plans are represented exactly once.
- [x] CHK004 Baseline commit `bdc7b228b78ef535d07357e7b33a36bbade917cc` is recorded.

## Authority and Loss Prevention

- [x] CHK005 Every split unit has an existing canonical destination and preservation evidence.
- [x] CHK006 Every moved or archived source has an exact-case destination.
- [x] CHK007 Every removed source names a retained replacement.
- [x] CHK008 The six constitutional safety invariants remain discoverable in canonical documentation.
- [x] CHK009 Root README detail is removed only after its user guidance has a canonical destination.
- [x] CHK010 Ultimate marketing prose is archived and its durable explanation has one canonical page.

## Verification

- [x] CHK011 Negative fixtures fail on omissions, duplicates, unsafe deletion, and dangling destinations.
- [x] CHK012 mdBook navigation, local links, fragments, and generated search remain complete.
- [x] CHK013 Project and archive records do not enter published output.
- [x] CHK014 UTF-8, LF, punctuation, mojibake, whitespace, and ignored-output checks pass.
