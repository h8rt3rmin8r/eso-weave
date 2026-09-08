# Contract: Loopback Documentation Service

1. Bind only `127.0.0.1:0` and publish the selected port only as an HTTP URL.
2. Accept one bounded request per connection with short read and write timeouts.
3. Parse only a method, target, and HTTP version plus a bounded header terminator.
4. Accept GET and HEAD. Reject every other method with 405 and `Allow: GET, HEAD`.
5. Redirect `/` to `/eso-weave/`.
6. Route only the exact `/eso-weave/` mount into the embedded manifest.
7. Reject percent encoding, backslashes, controls, dot segments, malformed targets, and unknown paths.
8. Return body bytes only for GET; HEAD returns the same status and headers with no body.
9. Include media type, length, close connection, no-store cache, nosniff, frame denial, referrer restriction, and restrictive CSP headers.
10. Never read a request path from disk or expose app state, configuration, logs, mutation, listing, or proxy behavior.
11. Reuse one listener and URL until application drop requests shutdown and joins the worker.
