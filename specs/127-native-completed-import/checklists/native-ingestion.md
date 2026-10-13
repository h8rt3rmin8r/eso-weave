# Native Ingestion Requirements Checklist

**Purpose**: Requirements quality for source boundaries, identity, preservation and capability truth.
**Created**: 2026-10-13
**Feature**: [spec.md](../spec.md)
**Audience**: Author and PR reviewer, before implementation.

## Completeness and clarity

- [x] CHK001 Is native source selection distinct from addon capture and application logs? [Completeness, Spec FR-001]
- [x] CHK002 Are version/readiness/count metadata and non-disclosure requirements explicit? [Clarity, Spec FR-002, FR-010]
- [x] CHK003 Are completed and unfinished combat dispositions defined? [Clarity, Spec FR-003, Clarifications]
- [x] CHK004 Are native original fields and source identity distinguished from addon callbacks? [Consistency, Spec FR-004]
- [x] CHK005 Are metric availability and missing-observation reasons specified? [Completeness, Spec FR-005]

## Preservation and failure boundaries

- [x] CHK006 Are same-path and copied-file reimport outcomes defined? [Coverage, Spec FR-006, US4]
- [x] CHK007 Are migration and late-rejection rollback requirements explicit? [Coverage, Spec FR-007]
- [x] CHK008 Are native-specific limits and responsiveness defined without applying addon limits to native files? [Clarity, Spec FR-008, SC-006]
- [x] CHK009 Are unknown, malformed, unsupported and incomplete-line dispositions explicit? [Coverage, Spec FR-009, Clarifications]
- [x] CHK010 Are source immutability, public-fixture sanitization and diagnostic boundaries consistent? [Consistency, Spec FR-010]

## Integration and evidence

- [x] CHK011 Are source refresh generations and changed-file outcomes addressed? [Coverage, Spec US3, Edge Cases]
- [x] CHK012 Are addon history and native provenance isolated? [Consistency, Spec FR-011]
- [x] CHK013 Are supported completed formats separated from future live/platform qualification? [Consistency, Spec FR-012]
- [x] CHK014 Are chronological planning, bounded reviews and owner merge defined? [Completeness, Spec FR-013]
- [x] CHK015 Are representative, adversarial and preservation outcomes measurable? [Measurability, Spec SC-001-SC-007]

## Notes

15/15 requirements-quality items passed. Exact record layouts and operational
bounds are plan decisions derived from the observed file and primary contracts.
