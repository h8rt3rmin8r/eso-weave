# S126 Verification Receipt

Date: 2026-10-13
Branch: codex/s126-retained-recording-sessions
Issue: [#273](https://github.com/h8rt3rmin8r/eso-weave/issues/273)
Base: 7939729f302611f911c39216aea0c00f5dfe3ef1

## Specification and implementation

All spec-kit stages completed in order: specify, clarify, requirements/domain
checklists, plan with read-only research, tasks, blocking analyze and implement.
Requirements quality: 10/10; retention checklist: 12/12. All 13 requirements and
seven success outcomes are covered. Post-design archive-version and admission
refinements retain scope and constitution compliance.

The initial five retention regressions failed on S125 before implementation.
Additional red regressions reproduced malformed identifier disclosure and extra
legacy selection arguments; the implemented corrections pass.

## Local gates

| Gate | Result | Local log |
| --- | --- | --- |
| cargo fmt --all -- --check | PASS | target/s126-fmt.log |
| cargo clippy --all-targets --all-features -- -D warnings | PASS | target/s126-clippy.log |
| cargo test --all --locked | PASS, 1087 tests | target/s126-full-tests.log |
| cargo build --release --locked --bin eso-weave | PASS | target/s126-release-build.log |
| Policy fixture suite | PASS, 191 tests | target/s126-policy-tests.log |
| Documentation spelling, brand and trust policies | PASS | target/s126-spelling.log, s126-brand-policy.log, s126-trust-policy.log |
| mdBook test and build, including link checks | PASS | target/s126-mdbook-test.log, s126-mdbook-build.log |
| Complete documentation policy | PASS | target/s126-docs-policy.log |
| Headless Chrome rendering sentinels | PASS | target/s126-docs-render.log |
| Release-note and Debian validator fixtures | PASS | target/s126-release-notes-fixtures.log, s126-validate-debian-package-fixtures.log |
| git diff --check | PASS | Direct repository check |

An initial full run stopped at one shared-addon emitted-version assertion; that
assertion now names controller2 and retains catalog isolation. Documentation
fixtures initially required restoring the explicit clear-all command at its
point of use; its deletion-only meaning remains. Both gates passed after repair.

## Publication and review

Automatic branch push and official PR publication are explicitly authorized.
Remote publication, final-head CI and external review are pending.
At most two review rounds are permitted; owner merge remains the final boundary.
No merge, tag, release, native-log implementation or owner field-verification
request is included. Installed/game behavior remains unverified.
