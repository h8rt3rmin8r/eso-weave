# S120 Data Model

## Native desktop binding fact

One existing fixed-size fact per action: portable primary and modifier bits, or unavailable/unbound/conflicting/unsupported. Positively identified controller assignments do not participate in desktop conflict counting. Controller-only remains unsupported. Unknown desktop keys and unsupported desktop chords remain rejected.

## Version evidence

Channel (Live or PTS), observed head revision and timestamp, client release from bounded recent history, independently parsed numeric API from head-pinned documentation. Body limits, validated identifiers, and a bounded source-age window determine freshness. A network-successful stale source is unknown, not current.

## Compatibility result

For each package, compare independently observed numeric API against reviewed embedded declarations and installed declarations when present. Unknown evidence stays unknown; unsupported reviewed API needs an application update; supported embedded but outdated installed package needs install/update and reload. Remembered values are historical only. Recompute on each startup rather than using last-seen client as warning suppression.

## Diagnostic state

Last logged loss cause is separate from retained values/deadlines. Same cause produces no new log event. Changes and recovery produce one event; retention expiry produces one event as its snapshot is discarded.
