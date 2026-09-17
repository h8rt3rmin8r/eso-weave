# Local Extension Documentation and Verification Checklist

**Purpose**: Verify the completeness, parity, lifecycle, privacy, and maintainability boundaries unique to S109

**Created**: 2026-09-17

## Public Guidance

- [x] One canonical entry point owns enablement through troubleshooting
- [x] UI labels and warning copy are named exactly
- [x] Discovery and authentication do not expose a real credential or user path
- [x] HTTP and standard MCP examples are both required
- [x] Read-only agent workflows remain non-orchestrating and non-action-driving

## Contract Semantics

- [x] Envelope revision and generation fields are distinguished
- [x] Knowledge and freshness axes are independently documented
- [x] Every public state path has required metadata
- [x] Both databases, typed query values, bounds, truncation, and errors are covered
- [x] Compatibility rules define additive and breaking changes

## Executable Evidence

- [x] Production HTTP and official RMCP adapters share one deterministic fixture
- [x] Capabilities, state, inventory, success, and error parity are compared
- [x] Only transport framing and elapsed query time may be normalized
- [x] Port collision, recovery, disconnect, restart, and shutdown are exercised
- [x] Documentation inventory drift causes a focused automated failure

## Safety and Scope

- [x] Tests use synthetic data and placeholders only
- [x] No remote binding, writes, telemetry, agent hosting, or new observations enter scope
- [x] Existing safety-critical and trust gates remain mandatory
- [x] Plan 045 and issue #180 closure evidence are included

## Notes

Checklist passed on 2026-09-17 before implementation planning.
