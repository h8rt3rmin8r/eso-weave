# Contract: Responsive Documentation Tables

## Source Inventory

The published corpus contains exactly 54 Markdown tables across 26 pages. Policy identifies each placement by source page, table ordinal, and header signature. Inventory changes require an intentional contract and test update.

The named high-density pages contribute these exact counts:

| Page | Table count |
| --- | --- |
| `development/coverage-matrix.md` | 2 |
| `development/test-strategy.md` | 2 |
| `development/state-machines.md` | 4 |
| `reference/status-reference.md` | 7 |
| `reference/pixel-bus-protocol.md` | 2 |

## Generated Structure

Every generated table keeps this semantic core:

```text
table wrapper
└── table
    ├── thead
    │   └── tr
    │       └── th...
    └── tbody
        └── tr...
            └── td...
```

Progressive enhancement may add a shell and adjacent instruction but must not replace, duplicate, flatten, or change the display semantics of `table`, `thead`, `tbody`, `tr`, `th`, or `td`.

## Runtime State

Initialization is idempotent. Each table receives one classification and each existing wrapper receives at most one shell and hint.

An overflowing wrapper exposes:

- `data-docs-table-overflow="true"`;
- `tabindex="0"`;
- `role="region"`;
- a non-empty, page-unique `aria-label` derived from visible heading context and table order;
- `aria-describedby` targeting its visible instruction;
- a start, middle, or end position marker;
- a visible focus indicator when keyboard focused.

A fitting wrapper exposes:

- `data-docs-table-overflow="false"`;
- no `tabindex`, region role, accessible region name, or description relationship;
- a hidden instruction;
- position `none` and horizontal offset zero.

## Classification

- Compact: two columns unless a named-page profile requires more room. It retains natural width.
- Dense: three or four columns, or an explicit named-page profile. It receives a documented minimum inline size.
- Very dense: five or more columns. It receives the largest bounded minimum inline size.
- The five named pages receive explicit profiles sized for their comparison content.

Classification never changes table text, font scale, row order, headers, or cell relationships.

## Refresh Contract

Geometry refresh runs after initialization, font readiness, observed wrapper or table size changes, and window resize. Repeated signals batch into one animation frame.

When overflow ceases, obsolete interaction attributes disappear and `scrollLeft` resets to zero. When overflow continues, the offset remains bounded and the position marker updates after native scrolling.

## Presentation Contract

- Table text remains at the inherited documentation body scale.
- Cell content uses natural wrapping and never `word-break: break-all`.
- Dense minimum widths prevent character-stack columns.
- Overflow is limited to the table wrapper, never the complete page.
- A visible text instruction provides discoverability independently of scrollbar appearance or color.
- Focus styling meets the existing documentation focus contract.
- Themes retain readable header, row, border, instruction, and focus contrast.

## Print and Script Failure

Print hides the instruction and focus presentation, removes screen minimum widths, and exposes table overflow rather than intentionally clipping content.

If local JavaScript cannot run, mdBook's original wrapper still provides local horizontal containment and the semantic table remains unchanged. No remote resource is required.

## Browser Evidence

The required matrix is:

```text
5 named cases x 2 themes x 2 widths = 20 observations
```

Each observation proves semantics, classification, font and word-breaking rules, geometry, local and page containment, and exact overflow accessibility state.

Special journeys prove:

- a compact fitting table adds no redundant interaction;
- trusted ArrowRight input moves an overflowing focused wrapper but not the page;
- resize crosses an overflow threshold and updates state;
- true page scale reaches 2 while remaining contained;
- print hides interaction and removes screen clipping constraints;
- blocked scripts retain table semantics and local containment.

The run prints `ESO_WEAVE_TABLE_SMOKE_PASS_V1` only after S086, S087, S088, and S089 validation all pass.
