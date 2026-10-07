# S123 Candidate Evidence

Date: 2026-10-06. Baseline: protected main `56d97ae` (S122 owner squash merge).
Scope: issue #255, candidate preparation only. No installed/game checks.

## Specification Gates

Specify, clarify, requirements checklist, plan and tasks completed in order.
Both checklists pass 12/12. Read-only analyze passed before implementation:
zero critical/high/medium findings, 8/8 functional requirements and 5/5 success
criteria covered by the 14 tasks, no unresolved clarification or constitution
conflict. Required Phase 0 research agents worked read-only on rollover and
archive authority. The optional agent-context hook updated the managed block.

## Baseline and Tool Authority

- Local and remote `v0.17.3` tag absent; hosted release endpoint returns 404.
- Installed cargo-release 1.1.2; effective configuration keeps verification
  enabled and publication/tag/push disabled. Closed stdin uses `--no-confirm`.
- Existing Unreleased notes gate rejects missing Highlights with exit 4 before
  implementation (`target/s123-notes-red.log`).
- Archive chronology: PR #225 (September 17), #252 (October 5), #253 and #254
  (October 6). S113-S115 remain unused reservations.

## Candidate Checks

Preparatory gates passed: release-note extractor and contract fixtures,
167 documentation/brand/render-policy tests, 13 trust-policy tests, repository
trust policy, Debian validator fixtures, spelling, brand adoption, mdBook
examples/build/link checks and generated-site policy. Whitespace and 24 changed
files' strict UTF-8/no-BOM/LF/punctuation/mojibake checks passed. Current Plan048
references now resolve to the archive, including historical spec paths.

Post-rollover Rust/build, candidate identity and rendering gates remain pending.

## Hosted Integration

Official PR and external review/CI pending. Requested Codex rounds: 0.
Final owner merge and later publication remain separate operations.
