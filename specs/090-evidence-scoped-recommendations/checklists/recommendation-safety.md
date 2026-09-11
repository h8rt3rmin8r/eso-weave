# Recommendation Safety Checklist

**Purpose**: Prove that S090 advice remains evidence-scoped, provisional, local,
and unable to influence gameplay automation.

**Created**: 2026-09-11

**Feature**: [spec.md](../spec.md)

## Evidence Integrity

- [x] CHK001 One immutable S077 projection is the sole recommendation input.
- [x] CHK002 No catalog reopen can mix a newer catalog with an older projection
  receipt.
- [x] CHK003 Recommendation policy and projection algorithms have distinct stable
  versions.
- [x] CHK004 Every item carries encounter, raw, projection, calculation, catalog,
  channel, and API provenance.
- [x] CHK005 No timestamp, randomness, iteration-order accident, or environmental
  state can change output.
- [x] CHK006 Invalid or unavailable ratios cannot become numeric claims.
- [x] CHK007 Checked sequence arithmetic prevents overflow and double-counted loss.

## Advice Restraint

- [x] CHK008 Advice is always labeled provisional.
- [x] CHK009 Advice is a review prompt, not a causal or optimality claim.
- [x] CHK010 The policy uses only duration, casts, loss, catalog knowledge, damage
  share, and effect uptime that the projection actually provides.
- [x] CHK011 No DPS target, HPS target, build prescription, skill replacement, or
  rotation guarantee is inferred.
- [x] CHK012 Unknown target semantics never produce an ID-only recommendation.
- [x] CHK013 Unrelated unknown IDs qualify but do not globally suppress valid
  known-target advice.
- [x] CHK014 Output count and order are bounded and deterministic.

## Suppression and Qualification

- [x] CHK015 Short duration and low cast count suppress all advice.
- [x] CHK016 Sub-material loss qualifies and material loss suppresses all advice.
- [x] CHK017 Unknown IDs qualify known advice and material unknown damage share
  suppresses only the damage rule.
- [x] CHK018 Suppressed reports retain facts and reasons but contain no advice.
- [x] CHK019 Qualified items expose every applicable reason adjacent to their text.
- [x] CHK020 Exact threshold boundaries are testable without floating-point
  ambiguity.

## Isolation and Privacy

- [x] CHK021 Generation is pure, local, in-memory, and side-effect free.
- [x] CHK022 Recommendation types expose no command, callback, sink, UI intent, or
  action-authority surface.
- [x] CHK023 The recommendation module has no input, automation, addon, Pixel Bus,
  focus, network, telemetry, persistence, or logging dependency.
- [x] CHK024 Existing raw storage, catalog storage, metrics, and deletion contracts
  remain unchanged.
- [x] CHK025 UI presentation has no apply, execute, copy-to-action, or automation
  control.

## Verification

- [x] CHK026 Pure tests cover deterministic ready, qualified, suppressed, empty,
  invalid, and tie cases.
- [x] CHK027 Worker integration proves projection and report are generated from the
  same in-memory input.
- [x] CHK028 Headless wide and narrow UI tests prove visual separation and visible
  provenance.
- [x] CHK029 Static source checks enforce prohibited dependency and wording bounds.
- [x] CHK030 Live parity issue #131 remains nonblocking and cannot be cited as proof
  of S090 correctness.

## Notes

- Review completed before planning on 2026-09-11.
- The checklist evaluates requirement quality and planned controls. Implementation
  evidence is recorded in `tasks.md` and `quickstart.md`.
