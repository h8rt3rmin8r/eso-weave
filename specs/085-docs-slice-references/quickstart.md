# Quickstart: Verify Compact Work-Slice References

1. Run the focused documentation policy unit suite.
2. Build the mdBook site.
3. Run documentation policy against the generated site.
4. Run link checking, spelling, and text-integrity checks.
5. Search `docs/src` for lowercase numeric slice prefixes, long slice-prefixed symbols, expanded slice phrases, and concrete numbered spec paths.
6. Inspect the affected developer pages at a narrow width and confirm slice references do not obscure adjacent content.
7. Run the repository merge gate.

Expected result: published work-slice references are compact, evidence links remain usable, the `s069-v1` algorithm identifier remains intact, and every merge gate passes.
