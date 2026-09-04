# Quickstart: Delivery Pipeline Governance

## Validate repository artifacts

From the repository root:

```powershell
node --test .github/scripts/issue-link-policy.test.mjs
$env:PR_BODY = 'Closes #45'
$env:PR_AUTHOR = 'maintainer'
$env:PR_LABELS_JSON = '[]'
node .github/scripts/issue-link-policy.mjs
specify check
```

The policy command must report issue #45 and exit successfully. Repeat with `Related to #45`; it must exit nonzero with corrective guidance.

## Validate external GitHub state

1. Confirm issues #45, #46, and #47 remain open until the pull request merges.
2. Confirm the S044 pull request body includes one complete closing reference for each issue.
3. Confirm the `Issue linkage` workflow is green.
4. Open `ESO Weave Delivery` and confirm every repository issue appears once.
5. Confirm closed issues are Done and open issues carrying `needs: verification` are Release verification.
6. Confirm both delivery table and delivery board views are available.

## Validate the audit

Compare the committed audit record with current GitHub issue metadata. Any mismatch must have either an evidence-backed correction or an explicit unresolved disposition. Do not infer release verification from a merge alone.

## Full handoff gate

Run the repository's complete formatting, linting, test, and documentation gate before opening the pull request. Then wait for CI and the permitted Codex review rounds before requesting the merge ritual.
