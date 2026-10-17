# S135 Validation Guide

Prerequisites: Node.js, mdBook 0.5.4 and mdbook-linkcheck2 0.13.0 on PATH.
Run these commands from the repository root.

1. Read [audit.md](audit.md) and follow the cited issue closure evidence. Historical audit prose must be dated; not-planned closures must not imply passed field checks.
2. Run the existing policy fixture suites:

```text
node --test .github/scripts/docs-policy.test.mjs .github/scripts/docs-render-smoke.test.mjs .github/scripts/brand-kit-policy.test.mjs .github/scripts/trust-policy.test.mjs
mdbook build docs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
node .github/scripts/trust-policy.mjs
```

3. Compare the machine contracts against baseline after removing only the new lifecycle_context member. All other fields must be semantically identical. Compare constitution principle/gate text byte-for-byte with baseline; only Sync Impact metadata and the version footer may differ.
4. Run Git whitespace and strict UTF-8/no-BOM/LF/mojibake/punctuation checks for every changed text file. Review the diff for unexpected runtime, pinned artifact, bundled-guide or release identity changes.
5. Publish one PR with Closes #319; confirm Project Stage PR review, exact-head CI and completed automatic review with no unresolved threads. Leave the PR unmerged and auto-merge disabled.

No local Rust gate is required for this documentation-only slice under Constitution IV. Published-guide sources are unchanged; hosted docs CI supplies its normal build/link/render checks. No installation or game execution is part of this guide.
