# Loss and Security Checklist

- [x] Nil, false, zero, empty string, finite fractions, negative zero, unknown
  enums, Unicode, and future scalar arguments are covered.
- [x] Unsupported and oversized values omit the whole observation.
- [x] Terminal reserve and exact raw loss reconciliation are required.
- [x] Restricted parsing, stable reads, immutable storage, and atomic migration
  remain required.
- [x] Diagnostics, logs, receipts, public fixtures, and UI remain value-free.
- [x] Raw storage remains local with explicit user deletion and backup control.

