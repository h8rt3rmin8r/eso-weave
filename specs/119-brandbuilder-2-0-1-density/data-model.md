# Data Model: BrandBuilder 2.0.1 Density Correction

## Brand Kit Package

- Package id and filename
- Brand and BrandBuilder versions
- Canonical immutable URL and archive SHA-256
- Source revision and release tag
- Component version map, including egui adapter 1.0.1

## Recovery Distribution

- Repository path
- Bundle source path
- Extraction target
- SHA-256

## Native Density Profile

- Input profile: precise pointer or conservative
- Density: comfortable
- Minimum control height
- Horizontal and vertical item spacing
- Horizontal and vertical button padding
- Text-scale growth rule

## Relationships

- One adoption record identifies one immutable package and one recovery distribution.
- The normal ESO Weave theme consumes the package's comfortable precise-pointer profile.
- Product-owned widgets may exceed the profile minimum without redefining it.
