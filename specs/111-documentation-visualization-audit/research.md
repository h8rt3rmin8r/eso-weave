# Research: Documentation Visualization Audit

## Decision 1: Treat `SUMMARY.md` as the publication boundary

**Decision**: Inventory the 49 unique Markdown destinations linked from `docs/src/SUMMARY.md`.

**Rationale**: mdBook uses this file as navigation and publication authority. Files outside it may be maintainer records, machine-readable references, or assets and are not published pages.

**Alternatives rejected**:

- Scan every Markdown file under `docs/src`. Rejected because `SUMMARY.md` is the explicit contract and a blind scan can include unpublished material.
- Reuse only the content-coverage page list. Rejected because the audit must detect drift against what mdBook actually publishes.

## Decision 2: Pair page coverage with serious-candidate warrants

**Decision**: Give every page a compact decision while reserving full warrant evidence for serious candidates.

**Rationale**: Repeating full gate fields for simple landing pages would create noise, but recording only candidates would make omissions invisible. The paired model proves completeness and keeps detailed judgment focused.

**Alternatives rejected**:

- Record only approved topics. Rejected because it cannot prove a complete audit or representative restraint.
- Apply full candidate fields to all 49 pages. Rejected because page inventory and candidate evaluation are different entities.

## Decision 3: Enforce the audit through existing documentation policy

**Decision**: Add a pure exported validator with fixture tests, then call it from the complete policy using the real summary and manifest.

**Rationale**: The repository already treats documentation manifests as maintained contracts. A pure validator gives fast negative tests while the complete gate detects actual path drift and malformed evidence.

**Alternatives rejected**:

- Add a standalone script. Rejected because it would create another gate entry point and could be omitted from CI.
- Rely on reviewer inspection. Rejected because exact 49-page joins and unique issue links are mechanical invariants.

## Decision 4: Approve only durable local implementations

**Decision**: An approval must name stable source authority, an adjacent text equivalent, identical public and offline bytes, and a concrete update trigger.

**Rationale**: A visualization that cannot be maintained or understood without vision would worsen the documentation contract.

**Alternatives rejected**:

- Permit remote embeds. Rejected because bundled offline documentation must remain complete.
- Defer accessibility to implementation. Rejected because feasibility is part of the durability gate.

## Decision 5: Create issues only after clustering

**Decision**: Complete candidate comparison and overlap clustering before creating one implementation issue per approval.

**Rationale**: Early issue creation encourages duplicate graphics and makes rejection cleanup noisy. Final issue bodies can carry the stable manifest contract verbatim.

**Alternatives rejected**:

- One umbrella visualization issue. Rejected because each reader task and destination should be independently reviewable and mergeable.
- One issue per page. Rejected because cross-page systems often need one authoritative graphic.
