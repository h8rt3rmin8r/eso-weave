# Security Checklist: Bundled Offline Documentation

- [x] CHK001 Listener authority is restricted to IPv4 loopback and an ephemeral port.
- [x] CHK002 Only GET and HEAD are accepted.
- [x] CHK003 Request parsing has explicit byte, line, timeout, and lifetime bounds.
- [x] CHK004 Paths match only a compile-time allow-list and never enter filesystem APIs.
- [x] CHK005 Percent encoding, backslashes, dot segments, malformed UTF-8, and traversal are rejected.
- [x] CHK006 Responses define content type, nosniff, frame, referrer, cache, and CSP headers.
- [x] CHK007 CSP permits only the minimum same-origin and inline behavior mdBook requires offline.
- [x] CHK008 No route exposes state, configuration, logs, mutation, listing, or proxy behavior.
- [x] CHK009 Browser launch is non-interactive and cannot create a Windows console.
- [x] CHK010 Shutdown unblocks I/O and joins the worker within a tested bound.
