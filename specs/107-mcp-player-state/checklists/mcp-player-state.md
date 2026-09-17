# MCP Player-State Contract Checklist

**Purpose**: Guard the resource, parity, security, and lifecycle boundaries of S107

**Created**: 2026-09-17

## Discovery

- [x] Exactly two fixed resources are specified
- [x] Exact URI, name, title, media type, and description intent are specified
- [x] Resource support is advertised without subscriptions or list-change notifications
- [x] Templates, prompts, tools, queries, and mutable operations are excluded

## Canonical Truth

- [x] Both resources derive from the existing immutable publisher
- [x] One read uses one snapshot reference
- [x] Service generation overlay matches HTTP
- [x] Capability truth changes MCP state to available and leaves queries unavailable
- [x] Unknown, unavailable, dormant, freshness, source, and protocol semantics are preserved

## Errors and Safety

- [x] Unknown URIs use resource-not-found
- [x] Error messages do not reflect untrusted URIs or internal details
- [x] Existing bearer, Host, Origin, body, cancellation, and stopping policy is reused
- [x] Slow and disconnected clients cannot hold producer locks
- [x] Existing safety-critical tests remain merge gates

## Verification

- [x] Official RMCP client interoperability is required
- [x] Parsed HTTP/MCP equality is required
- [x] Bootstrap, loss, recovery, concurrency, restart, disconnect, and shutdown are covered
- [x] Full local and hosted gates are required before operator handoff
