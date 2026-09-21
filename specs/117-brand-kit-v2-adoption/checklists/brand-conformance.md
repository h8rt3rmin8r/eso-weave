# Brand Conformance Checklist: BrandBuilder 2.0 Kit Adoption

## Authority and provenance

- [x] CHK001 Package ID, filename, source revision, release tag, URL, and archive hash match the delivered bundle
- [x] CHK002 All seven contract versions and authority precedence are pinned
- [x] CHK003 Recovery distribution is retained and matches its published hash

## Interface

- [x] CHK004 Dark and light semantic surfaces and text equal higher-authority contract values
- [x] CHK005 Action, emphasis, destructive, border, focus, selection, hover, active, and disabled roles are governed
- [x] CHK006 Interactive response allocations meet the 44-point minimum
- [x] CHK007 Focus and status non-color cues remain visible
- [x] CHK008 Required text and non-text contrast pairs pass

## Typography and identity

- [x] CHK009 Inter 400/500/600 and Geist Mono 400 are bundled, licensed, hashed, and registered
- [x] CHK010 Technical metadata uses the approved mono family at identified seams
- [x] CHK011 Authoritative SVG masters remain byte-identical
- [x] CHK012 Guidance uses the 32-pixel reduced threshold and prohibits artificial crossing overlays

## Platform and records

- [x] CHK013 Applicable Windows, Linux, installer, app, and documentation assets have an explicit adopted or already-current disposition
- [x] CHK014 Every changed pinned packaging byte has a dated changelog decision
- [x] CHK015 Machine validation covers metadata, tokens, fonts, masters, recovery, and platform assets
- [x] CHK016 Documentation identifies adapter deviations and excludes non-shipping target suites

## Delivery

- [x] CHK017 Focused tests fail before production changes and pass afterward
- [x] CHK018 Local CI-parity, docs, release build, encoding, mojibake, and forbidden-dash gates pass
- [ ] CHK019 Hosted CI and all review threads are green or resolved
- [ ] CHK020 No more than one second Codex review round is requested
