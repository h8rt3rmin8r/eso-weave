# Research: Persistent Data Addon Foundation

## Permanent addon identity

**Decision**: Use `ESO Weave Data` as the product title and `EsoWeaveData` as
the folder, manifest, bootstrap, SavedVariables file, and global root prefix.

**Rationale**: The name describes the package's stable ownership rather than one
temporary function. It remains accurate for catalog discovery, encounter
capture, and later approved data modules without implying that PixelBeacon is
part of the bulk data plane.

**Alternatives considered**:

- Retain `EsoWeaveCollector`: rejected because encounter capture would appear
  subordinate to catalog collection.
- Retain `EsoWeaveEncounter`: rejected for the inverse ambiguity.
- Use a generic `EsoWeaveAddon`: rejected because it obscures the data-only
  boundary and could invite unrelated gameplay behavior.

## Package and module layout

**Decision**: Embed one manifest, a small bootstrap, `Catalog.lua`, and
`Encounter.lua`. Keep `/ewcollect` and `/ewencounter` as explicit user controls.
Each module owns unique event and update namespaces.

**Rationale**: One physical package fulfills #184 while separate source files,
namespaces, state subtrees, and activation rules retain high cohesion and allow
isolation tests. Existing commands describe capabilities, not obsolete package
identities, so renaming them would add user churn with no safety benefit.

**Alternatives considered**:

- Concatenate both Lua sources into one file: rejected because it hides event
  ownership and makes independent dormancy harder to review.
- Keep two hidden dependency packages: rejected because release contents would
  still contain three installed addons.
- Add a compatibility loader: rejected because no retained development data
  requires migration and #184 explicitly excludes it.

## Shared SavedVariables and clearing

**Decision**: Use `EsoWeaveDataSaved` with top-level version metadata plus
`catalog` and `encounter` subtrees. Parse the shared file once through the
restricted non-executing parser, then select the required subtree. Use a 128 MiB
outer stable-read limit while retaining all current per-module record, string,
chunk, event, and estimated-byte limits. Add `/ewcollect clear confirm`; remove
desktop deletion of the SavedVariables file.

**Rationale**: The current catalog payload permits 64 MiB and the encounter
payload permits 32 MiB estimated content. A 128 MiB outer cap accommodates both
plus serialization overhead without weakening the inner validators. Deleting
or rewriting a shared file would destroy the unrelated module and may race
ESO's in-memory SavedVariables owner.

**Alternatives considered**:

- Keep two SavedVariables globals in one file: rejected because package state
  would still lack one versioned ownership envelope and module projection.
- Reuse the 64 MiB outer cap: rejected because two individually valid module
  payloads could exceed it together.
- Retarget the desktop delete action: rejected as destructive cross-module
  mutation.

## Rust ownership boundary

**Decision**: Add `src/data_addon.rs` as the only deployment and embedded-file
authority. Keep `src/collector` and `src/encounter` as domain authorities, but
make both parsers consume the shared root and select their module subtree.

**Rationale**: Package lifecycle is a cross-module concern; parsing and business
rules are not. This avoids a broad rename through established catalog and
encounter storage code while eliminating the false collector-owned lifecycle.

## Encounter ingestion direction

**Decision**: Treat native `Encounter.log` tailing as the provisional preferred
bulk transport, terminal native-log import as the second choice, and the current
terminal SavedVariables import as the safe fallback. A hybrid may add
callback-only facts later, but bulk data remains off PixelBus. Do not claim the
preferred path approved until separate Windows and Linux or Proton operator
receipts satisfy the field matrix.

**Rationale**: ESO API 101050 and 101051 expose logging enablement, status,
format controls, and log version. The declared native record vocabulary covers
combat boundaries, casts, combat events, effects, unit state, equipment, maps,
zones, and trials. Repository evidence cannot establish incremental flush
cadence, callback parity, file sharing, crash durability, privacy transformations,
or platform behavior.

**Alternatives considered**:

- Continuous SavedVariables polling: rejected because ESO owns in-memory state
  and normally flushes only at supported lifecycle boundaries.
- PixelBus event transport: rejected because bulk event data would violate the
  latency-sensitive screen contract.
- Claim native logging as final from published descriptions: rejected because
  batching and field coverage require measured operator evidence.

**Primary evidence**:

- ESO API 101050 encounter-log functions:
  <https://github.com/esoui/esoui/blob/f76cf16c4e5be7b234d15dc7f676febffa64c5bb/ESOUIDocumentation.txt#L18147-L18163>
- Stock command and native line schema:
  <https://github.com/esoui/esoui/blob/f76cf16c4e5be7b234d15dc7f676febffa64c5bb/esoui/ingame/slashcommands/slashcommands_shared.lua#L43-L94>
- Native line-type enumeration:
  <https://github.com/esoui/esoui/blob/f76cf16c4e5be7b234d15dc7f676febffa64c5bb/ESOUIDocumentation.txt#L3073-L3099>
- Official encounter-logging announcement:
  <https://forums.elderscrollsonline.com/en/discussion/467949/encounter-logging>

## Companion-to-addon command decision

**Decision**: No-go for real-time desktop-to-addon command ingress. The minimum
approved desktop command vocabulary is zero. Keep explicit user slash or future
in-addon controls. Do not file a real-time implementation issue unless ESO adds
a documented inbound API or the operator separately accepts a live-account
binding experiment after reviewing unknown server visibility.

**Rationale**: Public ESO API evidence exposes binding inspection and protected
mutation, outbound-only clipboard and URL functions, and no inbound socket,
file-watch, clipboard, or URL primitive. A custom action plus generated key is
the only plausible immediate route, but ZOS confirms keybindings are persisted
server-side and public evidence does not define whether custom action metadata
is synchronized or visible. Delivery can also fail silently under focus, menu,
action-layer, conflict, and platform input conditions, with no supported
acknowledgement path.

**Alternatives considered**:

- Custom binding plus generated input: rejected for unknown account-associated
  metadata, contextual delivery failure, and absent acknowledgement.
- Piggyback a native action: rejected because it creates gameplay side effects.
- Type slash commands through synthesized input: rejected because it is state
  dependent and can leak text into chat.
- Write SavedVariables while ESO runs: rejected because it is not real-time and
  ESO's later flush may overwrite it.
- Offline next-load intent: technically possible only when ESO is stopped, but
  no approved workflow currently needs it. It requires a separate issue.

**Primary evidence**:

- ESO binding API:
  <https://github.com/esoui/esoui/blob/f76cf16c4e5be7b234d15dc7f676febffa64c5bb/ESOUIDocumentation.txt#L13012-L13047>
- ESO outbound URL API:
  <https://github.com/esoui/esoui/blob/f76cf16c4e5be7b234d15dc7f676febffa64c5bb/ESOUIDocumentation.txt#L17963-L17973>
- ESO outbound clipboard API:
  <https://github.com/esoui/esoui/blob/f76cf16c4e5be7b234d15dc7f676febffa64c5bb/ESOUIDocumentation.txt#L25091-L25095>
- ZOS staff statement on server-side binding persistence:
  <https://forums.elderscrollsonline.com/en/discussion/518523/where-are-keybinds-saved>

## Governance amendment

**Decision**: Advance the constitution from 2.2.0 to 3.0.0 and redefine the
three-package boundary as exactly two managed packages with three narrow
surfaces: PixelBeacon output, catalog discovery, and encounter capture.

**Rationale**: The existing principle explicitly requires separate Collector
and Encounter packages. Replacing that rule is a backward-incompatible
governance redefinition under the constitution's own semantic-versioning rule.
The amendment preserves rather than weakens every module-specific restriction.

## Documentation delivery

**Decision**: Update the canonical manual and maintainer release procedure. Do
not add an unrelated blog subsystem.

**Rationale**: ESO Weave's published manual is the established user and
contributor announcement surface. The repository has no blog content system;
creating one would be unrelated architecture expansion.

## Issue reconciliation

- Research decision: <https://github.com/h8rt3rmin8r/eso-weave/issues/187>
- Platform evidence owner: <https://github.com/h8rt3rmin8r/eso-weave/issues/190>
- Command-ingress decision: <https://github.com/h8rt3rmin8r/eso-weave/issues/189>

S092 closes the repository-verifiable research and decision scope in #187 and
#189. Issue #190 owns the Windows and Linux/Proton evidence that cannot truthfully
be produced before an installable artifact and representative ESO environments
exist. No native-log implementation is authorized until that issue qualifies the
provisional candidate.
