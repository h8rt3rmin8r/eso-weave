# Plan 031: Linux Input and Copy Parity

Status: Active

Sequence:

1. Establish one canonical application key universe and exhaustive Linux native
   mapping tests.
2. Advertise every supported application control plus every key reported by the
   selected physical keyboard before grabbing it.
3. Forward key events only and surface pass-through emission failures instead of
   silently dropping grabbed input.
4. Initialize menu evidence fail closed across input, Fishing, and Auto Potion,
   then open only after a valid Gameplay observation.
5. Align latency, Live Log, menu diagnostics, tests, and canonical documentation
   with the verified runtime contracts.

This slice closes issues #93 and #96. It does not include settings application
timing and Fishing binding issue #95, release work, or field verification.
