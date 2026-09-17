# Quickstart: Canonical Player-State HTTP API

1. Run `cargo test --test player_state --locked`.
2. Run `cargo test --test local_service --locked`.
3. Start the service with an ephemeral test port and a deterministic publisher.
4. Authenticate `GET /api/v1/capabilities` and verify schema 1.0.0 and HTTP-only state operations.
5. Authenticate `GET /api/v1/player-state` and verify all six domains share one revision and generation.
6. Publish active, focus-loss, signal-loss, dormant, and recovered content, then verify truthful knowledge and monotonic revisions.
7. Stall a reader, publish a replacement, and stop the service. Verify publication proceeds and shutdown remains bounded.
8. Run full CI parity.
