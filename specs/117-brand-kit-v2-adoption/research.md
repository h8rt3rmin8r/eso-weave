# Research: BrandBuilder 2.0 Kit Adoption

## Delivered package

The official site currently publishes `eso-weave-brand-1.0.0-bb2.0.0.zip` at `https://brand.shruggie.tech/eso-weave/downloads/eso-weave-brand-1.0.0-bb2.0.0.zip`. The downloaded bytes hash to `b37ac1459666ae33d772229bd5247699c2c845971eabed267ff85465b68a1ba1`, matching the release checksum observed during the S117 assessment.

The bundle record pins source revision `f974fbb5c532a3394980be4dd985aee86e7e2c9e`, release tag `v2.0.0`, Brand Canon `1.2.1`, Interface Canon `1.0.0`, component recipes `1.1.0`, Web/React adapter `1.1.0`, egui adapter `1.0.0`, compiler `2.0.0`, and brand `1.0.0`.

## Authority decision

The consumer contract declares this precedence:

1. `brand.json`
2. `enforcement/bundle.json`
3. `enforcement/release-impact.json`
4. `enforcement/interface-canon.json`
5. `enforcement/component-recipes.json`
6. `enforcement/version-policy.json`
7. `enforcement/documentation-contract.json`
8. `enforcement/consumer-contract.json`
9. non-conflicting human guidance

The generated egui adapter is useful implementation evidence, but it is not allowed to override that order.

## Adapter defect and explicit deviation

The generated egui `tokens.rs` assigns light `background` to white, `card` to `#F8F8F6`, muted text to `#986000`, and destructive to the dark `#E9505F`. Higher-authority `brand.json`, Interface Canon aliases, and the supplied agent contract require light background `#F8F8F6`, card `#FFFFFF`, muted text `#6B6B6B`, and destructive `#C0293A`.

S117 deliberately does not copy those four generated adapter values. This is a required authority-preserving deviation from flawed generated output, not a local style preference. The repository records it so a later adapter release can remove the exception.

## Runtime integration

The existing `src/app/theme.rs` already centralizes theme colors and local fonts. Extending that module is lower risk than adding the generated adapter as a second crate or permanent parallel design system. The implementation will:

- rename legacy gold-specific roles to semantic action and emphasis roles;
- expose exact governed surface, text, border, action, destructive, focus, state, spacing, radius, and target values;
- retain documented product-domain resource colors with independent contrast checks;
- register Geist Mono as a named family beside Inter;
- update representative technical metadata helpers to use the named mono family;
- test resolved values and interaction geometry through the existing Rust suite.

## Interaction target decision

Interface Canon makes 44 logical units invariant across density settings. The current theme intentionally reduces controls from a 22-point baseline. Repeating that legacy behavior would preserve a known accessibility conflict, so S117 replaces the hit allocation with the 44-point floor. Compact styling may use restrained padding and visual fills inside that allocation, but the response rectangle cannot shrink below the contract.

## Fonts

The existing Inter Regular, Medium, and SemiBold files are byte-identical to the kit. Geist Mono Regular is absent and must be added. The kit hash is `990f0e094fe02b8872429209c09abf4c03d22183c33c2a2ddb891dc7f086271c`. The existing SIL OFL notice covers the family license text; the repository documentation will identify both bundled families and their approved weights.

## Identity and platform assets

The authoritative glyph and mark already match kit hashes `552f3203f0001b15e3adea9b720cb2f78be1427a12410f3e304170d973fef5ea` and `696d256c4ec0eae9aed315a1b489bbf5115ec33827e966a6e993708bf3f3109f`. Existing Linux, window, installer, banner, logo, social, and reference ICO assets are byte-identical to `assets/reference` in the kit.

The kit also supplies a generated Win32 classic ICO with hash `3b3830fb98662d7e1fb0277e38d94072bfc67f9eac81a2f03433e6f11ee3c20a`. Because this repository builds a classic Win32 executable, that derivative applies and will replace `assets/icon.ico`. MSIX assets do not apply because the repository does not build MSIX packages. Linux has no separate generated suite, so its exact reference byte remains current.

Changing `assets/icon.ico` is a pinned packaging decision and must be recorded in `CHANGELOG.md`.

## Recovery retention

Vendoring the entire kit would duplicate unrelated web and platform artifacts. Retaining only URLs would fail the offline recovery requirement. The bounded solution is to retain:

- a machine-readable adoption record with consumed contract hashes;
- the exact `shruggie-brandbuilder-2.0.0.skill` recovery distribution, hash `26578eb150a9c24d9e625fb77b192e0415a6ac8faf67c83834ac914f2da15e90`;
- consumed fonts and applicable platform assets;
- a validator that checks repository destinations against the record.

This preserves exact recovery and auditability without turning the product repository into a mirror of every generated target.

## Validation approach

A small repository script will parse the adoption JSON, hash declared files, verify exact token literals in the runtime theme, and fail with path-specific mismatches. Node is already a documentation policy dependency and available in CI, so using an `.mjs` validator avoids adding a shipped Rust dependency. Unit tests for the validator will prove failure behavior. Rust tests remain responsible for runtime color, contrast, font registration, and interaction geometry.

## Decision

Adopt the higher-authority semantic contract in the existing theme layer, add the missing mono face, adopt the applicable classic Win32 icon, retain exact offline recovery bytes, document unchanged reference assets, and enforce the entire mapping with Node and Rust tests.
