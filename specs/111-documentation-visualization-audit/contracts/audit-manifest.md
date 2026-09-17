# Contract: Documentation Visualization Audit Manifest

The manifest is maintainer evidence, not published user documentation. Its root is a UTF-8 JSON object with schema version `1` and the entities defined in [data-model.md](../data-model.md).

## Publication join

The validator parses unique Markdown links from `docs/src/SUMMARY.md`, prefixes them with `docs/src/`, and requires exact set equality with page-record paths. Order follows the summary so reviewers can traverse the audit in publication order.

## Warrant truth

An approval requires all four gate values to be true. A rejection must have at least one false gate or explicitly demonstrate that the present medium already serves the reader better. Every candidate has complete alternatives, authority, text-equivalent, offline, update, and rationale fields regardless of decision.

## Handoff truth

An approved candidate carries one positive GitHub issue number and canonical repository issue URL. Issue numbers are unique across candidates. Rejected candidates carry `null`. The policy validates shape and uniqueness; hosted review verifies the issue content and open state.

## Scope boundary

The manifest cannot authorize a graphic, remote dependency, or runtime change by itself. Each linked implementation issue requires its own spec and pull request. S111 changes no published figure asset.
