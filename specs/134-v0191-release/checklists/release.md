# Release Requirements Checklist

Purpose: requirements-quality gate for the S134 author and PR reviewer.
Created: 2026-10-17. Feature: [spec.md](../spec.md). Standard depth.

## Completeness and clarity

- [x] CHK001 Are all five patch outcomes explicitly required in concise Highlights? [Completeness, Spec FR-001, SC-001]
- [x] CHK002 Is the authoritative candidate identity and common preparation date specified? [Clarity, Spec FR-002, SC-002]
- [x] CHK003 Are independent addon contracts and the exact retained package matrix distinguished from application identity? [Consistency, Spec FR-003, Assumptions]
- [x] CHK004 Is the evidence required before candidate handoff defined without treating candidate work as published or installed behavior? [Completeness, Spec FR-004, FR-005]

## Lifecycle and failure coverage

- [x] CHK005 Are candidate closure and later publication explicitly separate? [Consistency, Spec FR-006, User Story 3]
- [x] CHK006 Are merge, tag, publication, deployment and field-check exclusions explicit? [Clarity, Spec FR-007]
- [x] CHK007 Are dirty-tree, invalid Highlights, identity drift, existing tag and failed-check cases specified? [Coverage, Spec Edge Cases]
- [x] CHK008 Are exactly-once Project membership and evidence-backed states defined? [Measurability, Spec FR-008, SC-005]

All eight requirements-quality items pass before implementation. This checklist
assesses the written requirements; execution evidence belongs in verification.md.
