# S073 Pipeline Safety Checklist

**Purpose**: Protect source, channel, privacy, and publication boundaries.

- [x] Network acquisition is explicit, HTTPS-only, redirect-free, and allowlisted.
- [x] Every input and response has a hard byte and time limit.
- [x] Every source requires an immutable revision and exact SHA-256.
- [x] Local reads use stable no-follow handles and approved-root confinement.
- [x] Cache and candidate publication are atomic and no-clobber.
- [x] Live and PTS cannot compare, alias, or promote across channels.
- [x] Collector capture parsing remains non-executing and bounded.
- [x] Existing S070 validation and S072 icon limits remain authoritative.
- [x] Candidate files are allowlisted and exclude source and user-local bytes.
- [x] Reports exclude absolute paths, personal identifiers, and credentials.
- [x] Blocking findings prevent candidate publication.
- [x] Automation has read-only permissions and no mutation or release action.
- [x] Prior candidates and accepted catalogs survive every injected failure.
