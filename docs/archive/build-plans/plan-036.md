# Plan 036: Release Governance and Debian Metadata

Status: Complete, Archived

Sequence:

1. Amend the constitution and tracked operating guidance with the chronological
   pre-publication, publication, and artifact-dependent verification lifecycle.
2. Add explicit Debian maintainer metadata and a generated-package validator
   for required control fields.
3. Exercise the validator in pull-request CI and gate the tagged release package
   before upload or publication.
4. Close repository issues #104 and #105 while separate issue #107 waits in
   Release verification for the first downloadable package containing S066.
5. Validate governance, package fixtures, a real generated package, text policy,
   and full CI parity before delivery.

This slice changes no application runtime or package payload layout. It updates
pinned workflow, script, and release-guidance surfaces under a dated changelog
decision.

Completion evidence: issues #104 and #105 closed when S066 merged in PR #108.
