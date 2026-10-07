# S124 Automated Validation Guide

Use the repository hidden launcher on Windows. Cargo gates run sequentially in the foreground and are watched to completion.

1. Inspect targeted baseline tests in local_service, database_query, MCP integration and Lua import coverage. Run relevant existing test targets before upgrades.
2. Resolve the scoped packages, then run `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked` and the release-profile application build.
3. Run existing trust-policy and documentation policy/tests, release-note generator tests and rendered-documentation gate. Expect zero failed checks, attributable dependency inventory and consistent Plan049/050 lifecycle.
4. On a clean preparation commit, inspect `cargo release 0.17.4`, then execute `cargo release 0.17.4 --execute --no-confirm`. Expect synchronized candidate identities and no tag, push or package publication.
5. Validate notes from the v0.17.4 section and the exact candidate version/date/fixture fields. Publish the official PR and inspect all CI/review results on its final head.

No installation or game-session action is part of this guide. Original update PRs remain open for post-merge supersession.
