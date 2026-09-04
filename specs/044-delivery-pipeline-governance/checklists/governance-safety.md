# Governance Safety Checklist

**Purpose**: Prevent pipeline automation from weakening repository security or falsifying project history
**Created**: 2026-09-03

## Pull Request Policy

- [x] Closing-reference syntax matches GitHub's documented default-branch behavior
- [x] Multiple issues require complete closing keywords
- [x] Exemptions are narrow, visible, and documented
- [x] Pull request metadata is treated as untrusted input
- [x] Workflow permissions are read-only
- [x] `pull_request_target` is prohibited
- [x] Parser tests include expected failures and exemption cases

## Project Integrity

- [x] Stage options represent distinct delivery states
- [x] Existing labels and milestones remain the source of Priority, Effort, Area, and release grouping
- [x] Closed issues map to `Done`
- [x] Release-verification issues remain visibly incomplete
- [x] Unknown slice identifiers remain blank rather than being inferred

## Audit Integrity

- [x] Milestone corrections require merge or release evidence
- [x] Verification labels require an active field-evidence gate
- [x] Audit coverage includes open and closed issues
- [x] Corrections are recorded without rewriting unrelated issue content
- [x] Product code and releases are out of scope
