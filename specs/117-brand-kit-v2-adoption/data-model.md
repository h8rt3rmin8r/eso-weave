# Data Model: BrandBuilder 2.0 Kit Adoption

## `BrandKitAdoption`

- `schema_version`: local record schema
- `package`: ID, filename, canonical URL, archive SHA-256, source revision, release tag
- `versions`: all seven governed domain versions
- `authority`: ordered contract paths
- `migration`: required, optional-adopted, optional-not-applicable, and unaffected surfaces
- `recovery`: retained path, extract destination, and SHA-256
- `artifacts`: repository path, source kit path, role, disposition, and SHA-256
- `runtime_tokens`: semantic role plus exact dark and light hexadecimal values
- `adapter_deviations`: source path, affected roles, higher authority, and rationale

The record is immutable for this kit. A later kit requires a new explicit version adoption rather than editing version meaning in place.

## `SemanticPalette`

- surfaces: background, card, overlay, secondary, hover
- text: primary, muted, on-action, on-emphasis, on-destructive
- actions: primary, emphasis, destructive
- structure: border, input, focus
- state: disabled opacity, hover opacity, selected border width
- product domain: healthy, warning, health, stamina, magicka, ultimate

General UI fields trace directly to governed roles. Product-domain roles are named separately and must pass their documented contrast tests.

## `TypographyBinding`

- semantic role: display, body, label, mono
- family: Inter or Geist Mono
- weight: 400, 500, or 600
- repository file and fallback family
- expected SHA-256

## `AssetBinding`

- repository destination
- official kit source
- platform and usage role
- disposition: adopted, already-current, retained-master, or not-applicable
- expected SHA-256
- pinned packaging flag

## `ConformanceFailure`

- category: metadata, token, recovery, font, master, or platform asset
- key or path
- expected value
- observed value

Validation exits unsuccessfully when any failure exists and reports each mismatch independently.
