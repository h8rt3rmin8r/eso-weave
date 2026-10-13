# S127 Implementation Audit

Date: 2026-10-13. Branch: codex/s127-native-completed-import.
Issue: #272. Protected base: 9bfe9e07644d4611cd29bec3293e33457e4c4d69.

## Scope and evidence

All four stories and 13 functional requirements are implemented. The completed-file
normal/separate native version15 profile has source-specific models, bounded
validation, isolated run/context grouping and explicit unknown/incomplete reports.
Native input never executes code. Original fields stay ordered and local;
default details and errors omit raw payloads. No API/addon identity is inferred.

Schema6 keeps native runs/fights alongside addon evidence. Preflight, migration
and append share one transaction. Schema1-5 migration, collisions, exhausted
ordering, deletion/reimport, backup, immutable content/index validation and
read-only inspection are covered. Historical fixture reconstruction explicitly
removes schema6 objects before declaring an older schema.

The serialized worker carries native selection/path/channel/generation, checks
the inspected source byte hash before import, and dispatches native facts before
addon catalog gates. Desktop import/detail journeys run at 420 and 760 pixels.
The existing 62 sizing tests remain unchanged. The encounter-specific harness
hosts new width coverage rather than duplicating a second UI fixture in
app_ui_sizing.rs. Source controls move below a selected detail to preserve narrow
addon loss-text painting.

Native algorithm1 calculates paired duration, counts and reviewed observed
quantities. Local-player quantities require observed local-unit relations; hit
and overflow stay separate. Unknown evidence, clock reversal and zero duration
reduce rate availability. Unsupported uptime, callback-only evidence, replay,
API joins and recommendations have reasons.

## Verification and dispositions

- Foreground fmt check, all-target/all-feature Clippy with warnings denied, and
  all locked Rust tests pass (1114 tests).
- 181 documentation/render/brand/trust policy fixtures pass.
- Spelling, repository trust, brand adoption, release-note and Debian fixtures pass.
- mdBook examples, build/link checking, generated-site policy and complete headless
  diagram/syntax/figure/table smoke tests pass, including 200 percent zoom.
- The bundled optimized release binary builds successfully.
- Sanitized representative fixture: two complete fights with original shapes and
  synthetic strings/identifiers. Synthetic hostile, unknown, incomplete, UTF-8,
  escape, boundary and store-integrity cases pass.
- Private source proof: 43518907 bytes, 298596 records, 3 runs and 205 complete
  fights. Isolated import added205; identical reimport added0 and reported205
  existing; history contained205. Source byte hash stayed unchanged. Temporary
  private history was removed. Public records contain counts only.
- Initial integrated UI verification caught a narrow painting regression;
  moving source controls and preserving addon row height resolved it.
- Rendering caught the old unsupported-native probe and text-equivalent section
  boundary. The new native branch summary now belongs to the complete equivalent,
  with unchanged zoom/containment checks. Frozen historical coverage is retained.
- No dependency, toolchain, addon/controller/capture schema or release tag change.

## Remaining scope

Live tailing, native logger activation from desktop, every format variant and
installed Windows/Linux/Proton behavior remain unimplemented or unverified.
Repository and private-file evidence do not prove live callback parity. No owner
field-verification request is created. Official PR, at most two review rounds,
green final-head CI and owner final review/merge remain the delivery sequence.

After publication, a boundary regression reproduced valid unsigned64 amounts
overflowing a64-bit aggregate. Native totals now use128-bit arithmetic, bounded
by the record budget. The new regression, fmt, Clippy and all1114 Rust tests pass.
Original source/canonical evidence is unchanged by this projection correction.

Round1 Codex comment4257130239 (PRRT_kwDOTU-tqc6rzQUh) correctly identified
duplicate full-store validation for addon detail. UI requests now pass recorded
source kind to the worker; addon detail no longer probes native history first.
Native facts also survive unrelated catalog replacement without recalculation.
The mixed-source worker regression and both desktop widths pass.

Full correction verification: fmt/Clippy and optimized build pass. Two
default-harness reruns encountered Windows OS10053 in unchanged loopback tests.
Focused documentation/local-service suites passed; the complete1114-test local
recheck passed with one harness thread. CI retains its default harness.
