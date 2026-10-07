# S124 Maintenance Candidate Contract

- The lockfile must exclude rustls >=0.23.13,<0.23.45 and account for the ten scoped update targets.
- HTTP/MCP access remains the existing authenticated read-only contract; origins, host checks, credential scope and method vocabulary receive no new authority.
- Credential generation retains its existing byte count and encoding; malformed or mismatched cursors remain rejected under the existing query context rules.
- Lua import retains exact supported scalar values and bounded hostile-input rejection. Existing addon versions and capture schemas remain independent.
- Both CodeQL Action references use the official v4.38.2 full commit, with existing scoped permissions and credential behavior.
- Candidate version surfaces agree on v0.17.4. Notes come from bounded Highlights and point to the eventual tag-specific full changelog.
- The PR closes #261/#262/#263 on owner merge, after every mandatory check and review thread is satisfied. No tag, package publication or third requested review round is permitted in S124.
