# Contract: Workflow Integrity

## Static Workflow Rules

- Every workflow declares explicit top-level permissions with `contents: read`.
- Every remote `uses` reference is an exact lowercase 40-character commit SHA followed by a release comment.
- Every checkout step sets `persist-credentials: false`.
- `pull_request_target`, `issue_comment`, `workflow_run`, and `repository_dispatch` triggers are prohibited.
- Secret references are prohibited except the release job's explicit `GITHUB_TOKEN` publication binding.
- `contents: write` is allowed only in the release publication job.
- `pages: write` and `id-token: write` are allowed only in the guarded documentation deployment job.
- Runtime-downloaded executables are verified against checked-in digests before execution.
- Release-only Cargo commands install exact versions with `--locked`.

## Hosted Rules

`main` requires:

- pull-request integration;
- strict current status checks;
- Linux CI, Windows CI, dependency review, CodeQL, and both issue-link checks;
- administrator enforcement;
- resolved conversations;
- linear history;
- no force-push or deletion.

Repository Actions require:

- read-only default workflow tokens;
- workflow review approval disabled;
- GitHub-owned Action publishers plus `Swatinem/rust-cache` only;
- full commit SHA references.

## Failure Behavior

Static policy violations exit nonzero before build or release work proceeds. Digest mismatch exits before the downloaded tool is marked executable. Hosted branch protection rejects updates that do not satisfy the integration contract.
