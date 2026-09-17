# Quickstart: Verify Local Service Lifecycle

1. Start from a temporary configuration directory and verify `LocalServicePrefs::default()` is disabled with no credential.
2. Enable a controller configured for port zero. Wait for running status and verify the reported HTTP and MCP URLs use one effective loopback port.
3. Read discovery and verify process, generation, URLs, and schema version match status while no credential appears.
4. Send unauthenticated, wrong-token, wrong-Host, and wrong-Origin requests. Verify rejection and absence of permissive CORS headers.
5. Send an authenticated MCP initialize request and verify successful protocol negotiation with empty capabilities.
6. Use the explicit settings action to copy the credential and verify the secret is never rendered inline or written to discovery.
7. Disable and verify stopped status, discovery removal, and immediate port reuse.
8. Occupy a loopback port, enable against it, and verify `address_in_use`, no discovery, and no half-running service.
9. Exercise start-stop-start and stop-start-stop sequences, then verify the final requested state and generation ownership.
10. Drop a running controller and verify its listener and discovery are gone within 3 seconds.
11. Run focused tests, dependency inspection, repository text gates, and full CI parity.
