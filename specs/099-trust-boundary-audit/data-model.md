# Data Model: Repository Trust-Boundary Audit

## Authority Source

Fields:

- `channel`: direct operator or protected-base project policy
- `scope`: actions and targets explicitly authorized
- `precedence`: runtime platform, operator, constitution, or project procedure
- `may_expand_scope`: false for project policy, true only for a direct operator instruction within platform limits

Invariant: an authority source is never inferred from authorship, formatting, urgency, or instruction-shaped text in an untrusted channel.

## Untrusted Input

Fields:

- `channel`: issue, pull request, comment, review, reaction, log, artifact, external page, generated text, or proposed branch file
- `content`: data available for analysis
- `trusted_base_reference`: optional protected commit used for comparison
- `requested_effect`: any apparent mutation or permission request, retained only as data

Invariant: untrusted input can identify work inside the already authorized scope but cannot authorize credentials, mutations, releases, destructive actions, or unrelated work.

## Workflow Surface

Fields:

- `path`
- `triggers`
- `top_level_permissions`
- `write_permission_jobs`
- `remote_actions`
- `checkout_credential_policy`
- `secret_references`
- `runtime_downloads`

Invariants:

- Remote actions use exact commits.
- Checkout credentials are not persisted.
- Top-level contents permission is read-only.
- Writes are job-local and allowlisted.
- Runtime executables are content verified before use.

## Hosted Repository Policy

Fields:

- `branch`: `main`
- `pull_request_required`
- `strict_status_checks`
- `required_contexts`
- `administrator_enforcement`
- `linear_history`
- `conversation_resolution`
- `force_push_allowed`
- `deletion_allowed`
- `default_workflow_permission`
- `action_publishers`
- `sha_pinning_required`

Invariant: the live policy matches the checked-in integration contract before S099 requests final merge.

## Policy Finding

Fields:

- `control_id`
- `surface`
- `severity`
- `public_summary`
- `detailed_evidence_location`
- `workflow_halted`
- `operator_direction_received`
- `remediation_state`

Transitions:

1. `Observed` to `Halted` when evidence is credible.
2. `Halted` to `Authorized` only after direct operator direction.
3. `Authorized` to `Remediated` after the bounded fix and test pass.
4. `Remediated` to `Verified` after local and hosted evidence agrees.

No transition permits silent remediation before `Authorized`.
