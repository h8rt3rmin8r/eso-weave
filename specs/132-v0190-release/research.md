# S132 Research Decisions

Date: 2026-10-15. Protected-main baseline: 8cc94d7c169aa85057ec36daf2e8b5f3cc0e1672.

## Dependency integration

Decision: apply only the Cargo.toml/Cargo.lock content from #284 head 369767958cdb022e678ebdb4cb4a81a0d928fda3, preserving the baseline application and all other dependencies. The inspected diff changes only rmcp 3.5.0 to 3.5.1, serde_json 1.0.151 to 1.0.152 and tokio-util 0.7.19 to 0.7.20, including lockfile checksums.
Rationale: the owner's named scope includes #284, and one release PR allows one final merge. Both normal and test-client rmcp pins must agree.
Alternative: stop for a separate #284 merge. Rejected because candidate inclusion is already authorized and can be reviewed atomically. #284 stays open until equivalent content is confirmed on main.

## Release identity and evidence

Decision: reuse release.toml without changes. Its allow-branch rule accepts the chosen release branch; tag, push and publish are false. Run the installed cargo-release dry run before the authorized local execute command.
Rationale: the protected-main release procedure explicitly separates candidate preparation from annotated tag publication. Version is owned by Cargo.toml; addon manifests remain independent.
Alternatives: manually edit all version fields, or tag before merge. Rejected because both bypass the established reviewed identity flow.

## Guidance and validation

Decision: correct stale two-addon prose in maintainer and canonical release pages. After synchronizing protected main, the agent overview already describes complete v15 coverage; only its managed plan pointer needs updating. Preserve the package matrix and all historical release statements. Record the pinned release-guide edit as a dated changelog decision.
Rationale: S129-S131 have already implemented one combined package and all four v15 profiles; release readers need current content.
Alternative: leave stale guidance for another slice. Rejected because it directly contradicts the candidate's release content.

Decision: reuse existing automated gates, including the missing-Highlights rejection as red evidence. No runtime source, new tests, installed/game checks or field-verification issue is required.
Rationale: this is a scoped release and dependency integration task, not new runtime behavior. Required safety-critical and integration suites still run unchanged.

All technical questions are resolved from protected-base tooling and the inspected dependency diff. No additional technology choice or research delegation is needed.
