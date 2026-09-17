# Documentation Visualization Audit

This audit records the S111 review of every Markdown destination published by
`docs/src/SUMMARY.md` on 2026-09-17. The machine-readable authority is
[`documentation-visualization-audit.json`](documentation-visualization-audit.json).
It preserves the ordered 49-page inventory, every page decision, the complete
four-gate warrants, cross-page clusters, form coverage, and official issue
handoffs.

The audit approves four future figures. It does not implement a graphic or
change published prose. Existing diagrams and screenshots remain authoritative
where this review found them sufficient.

## Decision method

A candidate is approved only when all four questions have a positive answer:

1. Does a named reader task require the proposed view?
2. Does the subject carry enough relationship, sequence, spatial, hierarchical,
   comparative, diagnostic, or quantitative load to justify a graphic?
3. Would the graphic materially reduce comprehension effort compared with the
   current prose, table, screenshot, or existing figure?
4. Can repository-owned authorities keep the figure accurate, accessible, and
   useful offline?

Candidate status means that the page received a complete warrant. It does not
mean approval. Pages marked `covered_existing` already have an adequate figure
elsewhere. Pages marked `no_candidate` do not carry enough unresolved visual
load to warrant a full candidate record.

## Approved implementation handoffs

| Candidate | Destination | Reader payoff | Official issue |
| --- | --- | --- | --- |
| First-observation troubleshooting decision tree | Getting Started: Troubleshooting | Choose the next evidence check from a visible symptom without skipping a shared prerequisite | [#220](https://github.com/h8rt3rmin8r/eso-weave/issues/220) |
| Reviewed catalog evidence lifecycle | Development: Reviewed Catalog Candidate Pipeline | See where source evidence changes form, where review occurs, and where explicit selection and rollback begin | [#221](https://github.com/h8rt3rmin8r/eso-weave/issues/221) |
| Encounter evidence lineage | Reference: Encounter Data and Metrics | Trace raw identity, declared loss, catalog resolution, versioned metrics, and separately gated advice | [#222](https://github.com/h8rt3rmin8r/eso-weave/issues/222) |
| Local API and MCP authority map | Reference: Local API and MCP | Distinguish transport framing from shared authentication, state, database, and query authorities | [#223](https://github.com/h8rt3rmin8r/eso-weave/issues/223) |

Each issue owns one independently reviewable figure, its complete text
equivalent, offline delivery, accessibility checks, policy coverage, and update
triggers. None changes application behavior. Issue #222 must keep native-log
ingestion explicitly provisional while #190 remains open.

## Cluster resolution

The catalog candidate consolidates six pages: source rights, discovery
collection, compilation, candidate review, user selection, and the local icon
side path. One shared lifecycle is less likely to drift than six partial flows.

The encounter candidate consolidates the feature workflow, provisional
ingestion decision, and durable data-and-metrics reference. It shows one
provenance chain without claiming Combat Metrics parity or converting advice
into action authority.

Two other overlaps were clustered and rejected. Configuration plus settings is
better served by exact lookup tables, and status plus logging has no governed,
representative dataset for a durable quantitative dashboard.

## Restrained rejections

| Candidate | Form considered | Why it was rejected |
| --- | --- | --- |
| First-launch timeline | Temporal | The short numbered procedure and governed screenshots already carry the task directly. |
| Application interface map | Spatial | Current screenshots preserve the literal interface more faithfully than a reconstruction. |
| Action-authorization comparison | Comparative | The existing authorization SVG, truth table, and text equivalent already answer the question. |
| Configuration ownership tree | Hierarchical | The hierarchy is shallow, while tables preserve exact names, defaults, and persistence. |
| Live status and log dashboard | Quantitative | The pages serve semantic lookup, and no representative, versioned, privacy-safe dataset exists. |
| Release pipeline timeline | Temporal | The chronological maintainer procedure, command evidence, matrices, and failure table are the safer operational authority. |

The audit also considered relational and diagnostic forms. Those forms account
for the four approvals where readers otherwise reconstruct boundaries or
branches across several sections. No quota was used; approval follows the four
gates only.

## Existing coverage retained

Four existing flow diagrams continue to cover architecture ownership, action
authorization, Pixel Bus validation, and safety recovery. Governed screenshots
continue to cover first launch, settings, interface layout, and PixelBeacon
placement. Exact reference tables remain the preferred visualization for
settings, statuses, weave delay defaults, state transitions, and coverage.

## Maintenance contract

`docs/src/SUMMARY.md` is the publication authority. The documentation policy
requires an ordered audit record for every unique Markdown destination and
fails on an omitted, duplicate, or reordered page. It also requires complete
warrants, all seven considered visualization forms, resolved clusters, and one
unique canonical issue per approval.

Future published pages require an audit decision in the same change. A change
to an approved candidate's named source contract triggers review of that
candidate and its implementation issue. A rejected idea should be reopened only
when its failed gate changes, such as a genuinely branching workflow or a new
governed quantitative dataset.
