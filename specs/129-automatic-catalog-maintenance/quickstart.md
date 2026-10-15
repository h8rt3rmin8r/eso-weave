# S129 Automated Validation Guide

Use temporary AddOns/SavedVariables/config roots only. Do not point fixtures at installed ESO.

1. Run migration fixtures for every legacy/current/conflicting installation and injected race/rollback stage; compare exact previous/neighbor bytes.
2. Run Lua initialization/combat/loading/change/budget/failure fixtures with configured Live/PTS and optout values; previous completed payload survives until bounded new completion.
3. Build from the actual packaged baseline plus independent completed capture, then merge changed/disjoint/localized/shared-parent captures. Query facts and semantic coverage; replay must be no-op.
4. Start background reconciliation with pre-existing data and later saved generations. Exercise partial/locked files, restart, failures, retained recovery, cancellation and independent contexts.
5. Run settings and headless UI fixtures for all presence/preference states, direct keyboard/mouse handling and persistence failure.
6. Run cargo fmt --all -- --check; cargo clippy --all-targets --all-features -- -D warnings; cargo test --all --locked; pinned docs/repository/render and release-profile embedded guide gates. Use hidden noninteractive child launchers and watch foreground completion.

Record actual results in verification.md. Fixtures do not prove game save latency, installed behavior or exhaustive API visibility.
