# Document Boundary Contract

| Location | Authority | Published | Mutable lifecycle |
| --- | --- | --- | --- |
| `docs/src/**` | Canonical shipped behavior and architecture | Yes | Current |
| `docs/project/**` | Maintainer process and active planning | No | Current |
| `docs/archive/**` | Historical record | No | Closed |
| `docs/book.toml`, `docs/theme/**` | Documentation infrastructure | Used by build | Current |
| `docs/README.md` | Lifecycle map | No | Current |
| Root `README.md` | Concise repository gateway | GitHub only | Current |

`docs/src/SUMMARY.md` may reference only Markdown within `docs/src`. Generated
navigation and search must contain no project or archive records. The policy
also pins mdBook's configured source directory to `src` and rejects project or
archive sentinels in the generated search index.

Long-form user or architecture explanations have one canonical page. Historical
records may quote or describe old behavior but must identify their historical
lifecycle and must not claim current authority.
