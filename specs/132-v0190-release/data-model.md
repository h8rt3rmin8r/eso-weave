# S132 Release Record

## Candidate identity

- Application version: 0.19.0; one preparation date supplied by cargo-release.
- Representations: package manifest, application lockfile entry, README badge, bundled documentation version/date, version-sensitive capture-request fixture and versioned changelog section.
- Constraints: all representations agree; complete history is retained; post-rollover Unreleased is empty; addon contract versions remain unchanged.
- Dependency inventory: only rmcp 3.5.1, serde_json 1.0.152 and tokio-util 0.7.20 differ from baseline, apart from application version.

## Delivery lifecycle

- S132 / Plan058 / candidate #302: specify -> analyze PASS -> implement -> verified candidate -> PR review -> owner merge.
- Release coordinator #301: remains open through candidate merge, later authorized tag/pipeline publication and asset-integrity audit.
- Dependency PR #284: incorporated into candidate; retire only after equivalent content lands on main.
- Receipt: exact candidate head, gates, up to two review rounds and finding dispositions.

No runtime schema, persisted user configuration, catalog, encounter or addon data model changes.
