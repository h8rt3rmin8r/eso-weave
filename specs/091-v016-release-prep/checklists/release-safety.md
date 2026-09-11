# Release Safety Checklist: v0.16.0

- [x] Preparation does not bump versions or create a tag.
- [x] Pinned changes have a dated changelog decision.
- [x] Rollover rules are field-scoped and cardinality-checked.
- [x] Release notes meet item and word budgets.
- [x] The release command is authorized only after reviewed preparation merges.
- [x] Tag publication depends on green verification and package builds.
- [x] Independent installed and live checks never block publication.
- [x] The unrelated untracked draft remains outside every commit and stash scope.
