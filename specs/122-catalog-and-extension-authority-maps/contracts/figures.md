# S122 Figure Contracts and Update Authorities

## Catalog evidence lifecycle (#221)

Asset: `catalog-evidence-lifecycle.svg`, placed at
`docs/src/development/catalog-candidate-pipeline.md`, followed by
`Catalog lifecycle text equivalent`. Alternative: `Catalog evidence flows from
pinned sources or bounded collector captures through review to explicit Live
selection and rollback, while PTS remains preview only`.

Authorities (functions are review/update entry points):

- `src/collector/mod.rs`: `parse_capture`, `validate_envelope`.
- `src/collector/import.rs`: `import_capture`, `bundle_from_capture`.
- `src/catalog/model.rs`: `normalize_and_validate`.
- `src/catalog_pipeline/acquire.rs`: `acquire_sources`, `publish_source_cache`,
  `immutable_raw_uri`.
- `src/catalog_pipeline/mod.rs`: `build_candidate_with_fetcher_and_cancel`,
  `verify_candidate`, `inspect_candidate`, `validate_policy`, `validate_version`,
  `validate_bundle_sources`, `baseline_diff`, `enforce_thresholds`.
- `src/catalog/compiler.rs`: `build_catalog`, `verify_catalog`.
- `src/catalog_update/mod.rs`: `install`, `rollback`, `write_selection`,
  `write_atomic`, `candidate_compatibility`, `validate_catalog_access`.
- `src/catalog_update/contract.rs`: `CatalogSelection`.
- `docs/project/catalog-sources.json` and canonical source-rights, discovery,
  compiler, candidate and update pages; catalog-candidate workflow is read-only.

Update triggers: source/channel/rights policy, capture completeness/provenance,
normalization, candidate files/hash/diff thresholds, origin acknowledgement,
compatibility, staging/atomic selection/cancellation, rollback and cache rights.
Input arrows are alternatives; PTS has no arrow to runtime selection. Only
reports are redacted; local-only values and cached image rights are preserved.

## Local extension authority (#223)

Asset: `local-extension-authority-map.svg`, placed near Operations in
`docs/src/reference/local-api-and-mcp.md`, followed by
`Local authority text equivalent`. Alternative: `HTTP and MCP share bearer
authentication, one loopback generation, canonical player state and bounded
read-only queries over fixed catalog and encounter databases`.

Authorities:

- `src/local_service.rs`: `build_router`, `request_guard`, `owner_loop`, HTTP
  handlers, `DiscoveryRecord`, `publish_discovery`, `cleanup_discovery`.
- `src/mcp_state.rs`: `resources`, `read_resource`, `query_tool`, `call_tool`.
- `src/player_state.rs`: `SnapshotPublisher`, `PublishedSnapshot::document`.
- `src/database_query.rs`: `inventory`, `execute`, `path_for`, `open_defended`,
  `execute_blocking`, shared permits and `MAX_*` bounds.
- `src/main.rs`, `src/app/mod.rs`: application-selected database paths, common
  service construction and canonical publication.
- `docs/project/local-extension-contract.md`, public guide/field inventory,
  `tests/local_service.rs`, `tests/database_query.rs`, `tests/player_state.rs`,
  `tests/local_extension_contract.rs`.

Update triggers: discovery fields/ownership, listener/generation/shutdown,
auth/Host/Origin/body boundaries, routes/MCP mappings, snapshot semantics/schema,
fixed database identities, limits/typed results/errors/materialization and parity.
Discovery carries no credential. Exact effective Host and matching optional
Origin are required. Transport envelopes differ while canonical semantics agree.
No runtime operation or access boundary is changed by this figure.

## Shared presentation contract

Owned offline SVG, full adjacent prose, no color-only meaning, existing expansion.
Both normal/expanded paint and exact annotated topology are checked at narrow
and desktop width; two-theme 200 percent probes require measured effective
font size >=14 pixels, containment and equivalent availability. No-script pages
retain images and equivalents. Policy mutations remove stage/gate/equivalent
anchors and asset bytes; receipt mutations reject missing or unreadable evidence.
