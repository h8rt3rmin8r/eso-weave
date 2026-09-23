# Research: BrandBuilder 2.0.1 Density Correction

## R1. Upstream change classification

**Decision**: Treat ShruggieTech S044 as a patch correction for native egui consumers, not an identity redesign.

**Evidence**: Upstream commit `7547446ae95e5a9987294aade759f7b595c0adb6` advances BrandBuilder to 2.0.1 and the egui adapter to 1.0.1. Its migration guidance requires native egui consumers to regenerate and repin. Brand identity remains unchanged.

## R2. Correct ESO Weave profile

**Decision**: Use the comfortable precise-pointer profile.

**Rationale**: ESO Weave is a desktop application operated through keyboard, mouse, or trackpad and currently exposes no density selection. The kit's default `apply_style` path uses a precise-pointer target, while capability-aware consumers retain the conservative target for touch or imprecise pointers.

**Alternatives rejected**: Keeping 44 points reproduces the reported regression. Selecting compact density would introduce a new product preference without user authority. Adding touch detection would materially expand scope and platform behavior.

## R3. Governed values

**Decision**: Set item spacing to 8 by 2, button padding to 8 by 4, and normal minimum interaction size to 28 by 28 logical points.

**Rationale**: BrandBuilder derives these values from the existing 12-point control spacing, 44-point conservative target, and 8-point pointer hit slop. The derivation is `44 - 2 * 8 = 28`, with spacing ratios of two thirds horizontally, one sixth vertically, and one third for vertical button padding.

## R4. Application integration boundary

**Decision**: Update the existing theme rather than import the generated adapter crate.

**Rationale**: The repository already translates authoritative contracts into one local theme that also owns product-specific status and meter roles. Importing the generated crate would duplicate ownership and expand dependencies for three scalar corrections.

## R5. Publication boundary

**Decision**: Local implementation may use a CI-certified upstream candidate for red-green work, but remote publication waits for the immutable official 2.0.1 package and recovery artifact.

**Rationale**: The existing consumer contract requires exact release-backed package identity. A mutable branch or expiring workflow URL is not a durable provenance source.
