# Contract: Documentation Landing Identity

## Source contract

`docs/src/README.md` must contain, in order:

1. A `landing-wordmark` wrapper with the local
   `assets/brand/eso-weave-banner.png` image and an empty alternative.
2. One Markdown H1 containing a `visually-hidden` ESO Weave span and visible
   Documentation text.
3. One `project-metadata` definition list labeled Documentation snapshot.
4. Handle, Applies to, Released, and Repository terms with exact authority values.
5. A visible note that the values are a build-time snapshot, naming `Cargo.toml`
   and `CHANGELOG.md` as update authorities.
6. The existing introductory prose and required landing-page anchors.

The identity must not reference the square mark, clear logo, white logo, external
image URL, or a second visible ESO Weave heading.

## Authority contract

The validator receives landing Markdown, package TOML, changelog Markdown, approved
banner bytes, and published banner bytes. It must report errors when:

- package name, version, or repository cannot be extracted;
- the matching dated changelog release heading does not exist;
- any displayed metadata value is absent or differs;
- the repository is not linked to the exact canonical URL;
- the required identity structure or disclosure is absent;
- the published banner bytes differ from the approved source.

## Presentation contract

- `.landing-wordmark img` is block-level, centered, width-constrained, and never
  exceeds the content container.
- `.visually-hidden` uses the standard clipped accessible-text pattern without
  `display: none` or `visibility: hidden`.
- `.project-metadata` uses a multi-column layout when space permits and one column
  at the existing 40rem narrow breakpoint.
- Links inherit global focus-visible treatment.
- Theme compatibility relies on the banner's self-contained ink background.

## Generated-output contract

The built `index.html` must retain the semantic H1, metadata definition list,
exact values, snapshot disclosure, and local hashed or copied banner reference.
The banner file must exist in generated output and no landing image may depend on
HTTP or HTTPS.

## Compatibility contract

No runtime application, bundled documentation server, mdBook configuration,
search implementation, release pipeline, or package metadata behavior changes.
