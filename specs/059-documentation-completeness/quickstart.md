# Quickstart: Validate S059

S059 changes documentation, documentation policy, and spec-kit records only. It
does not authorize runtime, addon, packaging, or release behavior changes.

## Test-first loop

From the repository root, add each S059 policy rule with a negative fixture
before implementing the rule or its documentation content.

Run the focused fixtures and capture the expected red result:

```text
node --test --test-name-pattern="S059" .github/scripts/docs-policy.test.mjs
```

The red result must identify the intended missing topic, duplicate authority,
unsafe claim, incomplete page contract, inaccessible visual, absent search
alias, or invalid evidence. A syntax error, missing fixture file, or unrelated
policy failure is not acceptable red evidence.

Implement the smallest documentation or policy change that satisfies the
fixture, then rerun the focused command until it is green. Repeat this loop for
each coverage invariant before running the complete validation sequence.

## Complete local validation

Build into the same ignored output directory used by hosted documentation CI:

```text
node --test .github/scripts/docs-policy.test.mjs
mdbook test docs
mdbook build docs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
git diff --check
```

Run the repository parity checks because the manual makes claims about current
source and tested behavior, even though S059 itself is docs-only:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all --locked
```

If a command is unavailable locally, record that limitation and require its
hosted equivalent to pass. Do not replace a failed check with manual review.

## Evidence review

After the commands pass, verify all of the following:

1. Every shipped feature and logical process in the frozen S059 coverage
   inventory resolves to one exact published page and section anchor.
2. The feature-to-page and logic-to-page matrices contain no omissions,
   duplicate canonical authorities, project records, or archive records.
3. Every completed coverage item cites concrete source and test evidence by
   stable path and symbol or test name. Source line numbers are supporting
   detail only because they are not stable identifiers.
4. Each user procedure covers prerequisites, configuration, normal operation,
   failure signals, recovery, diagnostics, and the related concept page when
   those dimensions apply.
5. Platform-sensitive guidance distinguishes Windows and Linux. Safety-sensitive
   guidance states that Unknown, unavailable, stale, or invalid evidence never
   authorizes automated input.
6. Search aliases are visible published language that a player may use. Hidden
   comments and project-only metadata do not satisfy search coverage.
7. Images and diagrams have useful text alternatives, and no instruction or
   state distinction depends on color alone.
8. Published prose clearly labels user guarantees, implementation details,
   diagnostics, and version-sensitive information where those distinctions
   matter.
9. `docs/src/SUMMARY.md` offers coherent user and developer paths, and every
   published page remains listed exactly once.
10. All changed text is valid UTF-8 without a byte-order mark, uses LF line
    endings, contains no mojibake, and contains no en dash or em dash.

## Contradictions and preservation

Current runtime source and its tests are the authority for statements about
shipped behavior. When existing documentation disagrees with both, correct the
documentation and record the reviewed evidence. When source and tests disagree,
or when behavior cannot be established without changing or running the product,
leave the coverage item incomplete and surface the contradiction. Do not guess,
change code, or broaden S059 to make a documentation claim true.

S058 preservation excerpts remain binding. Prefer additions around their
canonical text. If an accurate S059 correction must alter a frozen excerpt,
update the migration ledger, its frozen manifest, and a negative preservation
fixture in the same change. Passing the S059 coverage checks never excuses a
failure of the S058 loss-prevention checks.
