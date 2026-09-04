# Contract: Pull Request Closing Policy

## Accepted references

The pull request body passes when it contains at least one same-repository issue reference with a supported keyword:

```text
Closes #45
Fixes: #46
RESOLVED #47
```

Supported keyword variants are:

- close, closes, closed
- fix, fixes, fixed
- resolve, resolves, resolved

Keywords are case-insensitive and may be followed by a colon. Issue numbers must be positive integers.
References inside HTML comments, including the pull request template's examples,
are ignored.

Every issue in a multi-issue pull request requires a complete reference:

```text
Closes #45
Closes #46
Closes #47
```

`Closes #45, #46, #47` is insufficient because GitHub does not interpret the trailing numbers as independent closing references.

## Exemptions

The check passes without a closing reference only when one of these conditions is true:

- The author is `dependabot[bot]`.
- The pull request has the `dependencies` label.
- The pull request has the `skip: issue-link` label.

`skip: issue-link` is reserved for rare release or repository-administration work that genuinely has no atomic issue. Its use requires a rationale in the pull request body and maintainer review.

## Security boundary

- The workflow uses the `pull_request` event.
- Permissions are read-only.
- Pull request body, author, labels, branch, and title are untrusted.
- Untrusted values travel through environment variables, never expression interpolation inside executable source.
- The workflow does not use `pull_request_target`, secrets, or write operations.

## Non-goals

The check does not prove that an issue exists, is open, or semantically matches the change. Reviewers remain responsible for those judgments.
