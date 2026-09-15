# Replay and Security Checklist

- [x] Compatibility projections are untrusted comparison input
- [x] Raw observations and validated profile are the only replay inputs
- [x] Verified, divergent, indeterminate, and unavailable remain distinct
- [x] Raw loss cannot produce a verified or divergent claim
- [x] Unsupported profiles fail closed
- [x] API correlation uses bounded source-specific state machines
- [x] Numeric conversion and Lua rounding are checked
- [x] Actor and event ceilings apply during replay
- [x] Diagnostics contain no retained or compared values
- [x] Replay performs no I/O, mutation, network, or gameplay action
- [x] Legacy bytes and hashes remain immutable
- [x] Complete-current divergence is rejected before storage
