# Feature Specification: Bundled Offline Documentation

**Feature Branch**: `codex/s063-bundled-offline-docs`

**Created**: 2026-09-07

**Status**: Implemented

**Input**: Issue #82: bundle the searchable mdBook documentation in ESO Weave and open it securely from the application.

## User Scenarios & Testing

### User Story 1 - Open Complete Documentation Offline (Priority: P1)

As an operator, I need an obvious Documentation action that opens the same navigable and searchable guide shipped by the project, even when no network is available.

**Independent Test**: Build a production binary, disconnect external networking, choose Help > Documentation twice, and prove the guide, navigation, search, theme, fonts, and images work from one reused local endpoint.

**Acceptance Scenarios**:

1. **Given** a supported packaged build, **When** the operator chooses Help > Documentation, **Then** the default browser opens the bundled guide without contacting a remote documentation host.
2. **Given** the guide is open offline, **When** the operator navigates, searches, changes theme, or opens a deep page, **Then** all required assets and pages load from the bundled corpus.
3. **Given** Documentation has already been opened, **When** it is opened again, **Then** the application reuses the same server and address instead of starting another listener.
4. **Given** the browser cannot be opened, **When** Documentation is requested, **Then** the application remains usable and presents an actionable error.

---

### User Story 2 - Serve Only Bundled Public Files (Priority: P1)

As an operator, I need the local documentation endpoint to expose only immutable public documentation, so opening help does not expose files, application state, or network-reachable services.

**Independent Test**: Exercise allowed files, missing files, traversal encodings, unsupported methods, oversized requests, and non-loopback assumptions against the server contract.

**Acceptance Scenarios**:

1. **Given** the documentation service starts, **When** its address is inspected, **Then** it is bound only to an ephemeral port on `127.0.0.1`.
2. **Given** a known bundled path, **When** it is requested with GET or HEAD, **Then** the matching immutable bytes and correct content type are returned with restrictive security headers.
3. **Given** a missing, malformed, encoded, traversal, or non-allow-listed path, **When** it is requested, **Then** no filesystem access occurs and a bounded error response is returned.
4. **Given** a method other than GET or HEAD, **When** it is submitted, **Then** the service rejects it and performs no mutation.
5. **Given** the application exits, **When** the service owns an active listener, **Then** the listener and worker terminate with the application.

---

### User Story 3 - Ship Reproducible Documentation (Priority: P2)

As a maintainer, I need release builds to embed the validated documentation generated from canonical sources, so packages cannot silently omit or drift from the hosted guide.

**Independent Test**: Run the exact Windows and Linux production build path with the pinned documentation toolchain, inspect the binary and package behavior, then repeat with missing or wrong-version tools and prove the build stops.

**Acceptance Scenarios**:

1. **Given** the pinned documentation tools and canonical sources, **When** a production build runs, **Then** it validates and embeds the generated site without committing generated HTML.
2. **Given** documentation generation is missing, fails validation, or uses an unsupported tool version, **When** a production build runs, **Then** the build fails before a shippable binary is produced.
3. **Given** a normal debug or test build, **When** documentation tools are unavailable, **Then** compilation remains practical through a deterministic checked fixture that cannot be mistaken for release content.
4. **Given** release workflows for Windows and Linux, **When** they build packages, **Then** they use the same pinned documentation generator and exercise the bundled guide path.
5. **Given** the embedded site changes executable or package size, **When** the slice is reviewed, **Then** the measured before-and-after impact is recorded.

### Edge Cases

- The operating system has no registered default browser or rejects the launch.
- A browser requests `/`, the configured `/eso-weave/` mount, a nested directory, a query string, a fragment, or a favicon.
- A request contains percent encoding, backslashes, dot segments, invalid UTF-8, more bytes than the request cap, or multiple request lines.
- A client connects and stalls while the application is closing.
- Two documentation actions occur nearly simultaneously.
- The listener cannot bind even to an ephemeral loopback port.
- Generated mdBook asset names change because content hashing changes.
- The HTML references an asset that was not included in the generated allow-list.
- CSP blocks an mdBook behavior required for navigation or search.

## Requirements

### Functional Requirements

- **FR-001**: The main menu MUST expose a keyboard-accessible Help > Documentation action with stable user-facing copy.
- **FR-002**: Activating Documentation MUST start or reuse one application-owned documentation service and open its root URL in the operating system default browser.
- **FR-003**: The application MUST report browser-launch and server-start failures without crashing or blocking unrelated controls.
- **FR-004**: Production builds MUST generate the existing mdBook from `docs/book.toml` and `docs/src/` with the same exact tool versions used by documentation CI.
- **FR-005**: Production builds MUST fail when the required generator or renderer is absent, version-mismatched, generation fails, validation fails, or no complete site is available.
- **FR-006**: Generated HTML MUST remain ignored build output and MUST NOT be committed.
- **FR-007**: Every generated public site file MUST be compiled into the production executable as immutable bytes under a normalized allow-listed path.
- **FR-008**: Debug and test profiles MUST remain buildable without external documentation tools by embedding a deterministic development fixture that clearly identifies itself as non-release content.
- **FR-009**: The service MUST bind only to IPv4 loopback `127.0.0.1` on an operating-system-selected ephemeral port.
- **FR-010**: The service MUST support only GET and HEAD, MUST serve only the embedded allow-list, and MUST never read a request path from disk.
- **FR-011**: Path handling MUST reject malformed request targets, backslashes, percent encoding, dot segments, traversal attempts, and paths outside the `/eso-weave/` mount.
- **FR-012**: Requests and response work MUST be bounded by request-size, header-count or line, I/O timeout, and connection-lifetime limits.
- **FR-013**: Responses MUST include correct content types, `nosniff`, no-store caching, frame denial, referrer restriction, and a restrictive Content Security Policy that preserves offline mdBook navigation and search.
- **FR-014**: The documentation service MUST expose no application state, configuration, logs, mutation route, directory listing, proxy behavior, or non-documentation response.
- **FR-015**: Repeated opens MUST reuse the same live listener and URL, including concurrent requests.
- **FR-016**: Dropping the application-owned service MUST signal shutdown, unblock the listener, and join the worker within a bounded interval.
- **FR-017**: Browser opening MUST use native non-interactive launch behavior and MUST NOT create a console window on Windows.
- **FR-018**: Automated tests MUST cover asset lookup, mount routing, content types, GET and HEAD, 404 and 405 responses, traversal rejection, request bounds, service reuse, concurrent reads, and shutdown.
- **FR-019**: Windows and Linux CI and release jobs MUST install the exact pinned mdBook toolchain and exercise the production documentation build path before packaging.
- **FR-020**: Canonical installation, interface, architecture, test, and release documentation MUST describe the offline guide, trust boundary, build inputs, development fixture, and failure behavior.
- **FR-021**: S063 MUST close issue #82, archive completed plan 032 with PR #100 evidence, and establish plan 033 as the sole active build plan.
- **FR-022**: The slice MUST record a dated CHANGELOG decision for every modified pinned build, release, script, packaging, or release-guidance artifact.

### Key Entities

- **Embedded documentation manifest**: Sorted normalized path, media type, and immutable byte entries generated at compile time.
- **Documentation service**: Application-owned reusable listener, shutdown channel, worker handle, and root URL.
- **Request target**: Bounded raw HTTP target normalized only into a known mounted allow-list key.
- **Browser opener**: Platform seam that opens an HTTP URL without granting server or filesystem authority.
- **Development fixture**: Small checked site used only outside production profiles.

## Success Criteria

### Measurable Outcomes

- **SC-001**: One production binary opens a fully navigable and searchable guide with external networking disabled on Windows and Linux.
- **SC-002**: Automated tests prove 100 percent of generated HTML asset references resolve to embedded allow-list entries.
- **SC-003**: All tested malformed, traversal, unsupported-method, and unknown requests return bounded errors with zero filesystem access.
- **SC-004**: Repeated and concurrent documentation actions retain exactly one listener and one stable root URL.
- **SC-005**: Shutdown tests complete within one second with no surviving service worker.
- **SC-006**: A production build with either missing or wrong-version documentation tooling fails and emits a diagnostic naming the prerequisite.
- **SC-007**: Windows and Linux CI exercise the same release-profile documentation generation and embedding path used by package jobs.
- **SC-008**: Before-and-after executable and generated-site byte measurements are recorded in the slice evidence.
- **SC-009**: Format, strict Clippy, locked tests, documentation policy, spelling, text hygiene, mdBook, generated-site validation, and release-path smoke gates all pass.

## Assumptions

- The existing mdBook site is the single public documentation corpus and remains compatible with a same-origin loopback mount.
- Current generated filenames contain only ASCII path characters accepted by the strict request policy.
- An ephemeral IPv4 loopback listener is supported on the project’s Windows and Linux targets.
- The operating system default browser is the expected presentation surface; an embedded webview is unnecessary.

## Explicit Deviations

The issue permits extraction to a temporary directory as a fallback. S063 does not extract generated files because an in-memory allow-list has a smaller persistence and path-trust surface. The issue also suggests `file://` as a possible approach, but mdBook root routing and search are more reliably exercised through a same-origin loopback endpoint. Production generation runs as part of the Cargo release build rather than relying on a previously generated directory, so a direct production build cannot silently package stale or absent documentation.

The S059 content-coverage JSON is a frozen semantic projection and its policy rejects new obligation identifiers. S063 records its new delivery evidence in the dedicated spec, contracts, canonical architecture and test pages, rather than weakening that preservation boundary.

## Out of Scope

- Release field verification tracked by issue #84.
- A new release, release tag, or merge operation.
- An embedded webview, remote documentation fallback, documentation editor, analytics, or application-state API.
- Changes to the substantive feature documentation beyond explaining access and delivery.
