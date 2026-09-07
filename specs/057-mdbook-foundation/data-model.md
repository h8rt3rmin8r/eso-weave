# Data Model: Documentation Site Foundation

## Book Source

| Field | Contract |
| --- | --- |
| Root | `docs` |
| Published source | `docs/src` |
| Navigation authority | `docs/src/SUMMARY.md` |
| Special page | `docs/src/404.md` |
| Generated root | `target/docs-site` |
| Pages artifact | `target/docs-site/html` |
| Hosted base path | `/eso-weave/` |

## Published Page

- Has one repository-relative path with exact case.
- Appears exactly once in `SUMMARY.md`, except the special 404 page.
- Has one non-empty level-one heading.
- Uses document-relative local assets and links.
- May link to external references but may not load an external runtime resource.

## Site Artifact

- Is produced by mdBook 0.5.4 from the Book Source.
- Passes mdbook-linkcheck2 0.13.0 and repository policy checks.
- Contains local search, theme, and brand resources.
- Resolves nested content and 404 resources under `/eso-weave/`.
- Is uploaded and deployed without rebuilding only on trusted main runs.

## Deployment State

`source -> validated -> uploaded -> deployed`

- Pull requests may reach `validated` only.
- Main pushes and manual main runs may reach `deployed`.
- A failed check prevents every later transition.
