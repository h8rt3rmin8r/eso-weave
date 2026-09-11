# Data Model: Responsive Documentation Tables

## Table Placement

Represents one authored Markdown table.

Fields:

- `page`: canonical `docs/src` path.
- `source_line`: one-based header line.
- `ordinal`: one-based table order within the page.
- `header_signature`: ordered visible header labels.
- `column_count`: number of header columns.
- `row_count`: number of body rows.
- `maximum_source_row_length`: audit signal for dense content.
- `named_evidence_page`: whether the placement belongs to one of the five explicit issue pages.
- `classification`: compact, dense, or very dense.

Validation:

- Exactly 54 placements exist across 26 pages.
- Page plus ordinal is unique.
- Each placement has at least two columns, one header row, and one body row.
- Coverage Matrix has two, Test Strategy has two, State Machines has four, Status Reference has seven, and Pixel Bus Protocol has two placements.

## Table Shell

Represents progressive enhancement surrounding an existing mdBook table wrapper.

Fields:

- `wrapper`: the existing local horizontal overflow element.
- `hint`: one adjacent visible instruction owned by the shell.
- `table`: the unchanged semantic table.
- `classification`: width tier applied to the table.
- `table_index`: one-based page order.
- `context_heading`: nearest preceding visible heading.
- `accessible_name`: unique table region name derived from heading and index.
- `hint_id`: unique document identifier.
- `ready`: idempotence marker.

Relationships:

- One shell owns exactly one wrapper, hint, and table.
- One wrapper contains its original one table.
- One overflowing wrapper describes itself through its shell's hint.

## Overflow State

Represents the current measured interaction state.

Fields:

- `overflowing`: `scroll_width > client_width + tolerance`.
- `scroll_position`: start, middle, end, or none.
- `scroll_left`: current horizontal offset.
- `maximum_scroll_left`: reachable horizontal extent.
- `focusable`: true only while overflowing.
- `named_region`: true only while overflowing.
- `instruction_visible`: true only while overflowing.

Transitions:

```text
Fit -> Overflowing
  Add focusability, named region, instruction relationship, cue, and position.

Overflowing -> Overflowing
  Preserve bounded scroll offset and refresh start, middle, or end position.

Overflowing -> Fit
  Reset scroll offset, remove focusability and region attributes, hide instruction,
  and record position none.
```

## Table Observation

Represents generated-browser evidence for one table at one theme and width.

Fields:

- `case_id`, `page`, `selector`, `theme`, `viewport_width`.
- `table_tag`, `header_count`, `row_count`, `cell_count`.
- `table_display`, `header_display`, `row_display`, `cell_display`.
- `wrapper_width`, `table_width`, `scroll_width`, `maximum_scroll_left`.
- `overflowing`, `state_matches_geometry`, `locally_contained`, `page_contained`.
- `minimum_header_width`, `minimum_cell_font_size`, `word_break`.
- `focusable`, `role`, `accessible_name`, `hint_visible`, `hint_text`, `description_matches`.
- `focus_outline_visible`, `classification`.

Validation:

- Semantic elements retain table-family display roles.
- Overflow state exactly matches measured geometry.
- Overflowing observations have complete accessible interaction metadata.
- Fitting observations omit redundant interaction metadata.
- Page horizontal extent never exceeds its viewport tolerance.
- Font size remains readable and word breaking is not `break-all`.

## Table Receipt

Represents one complete browser run.

Fields:

- `sentinel`: S089 pass token.
- `inventory_total`: generated table count.
- `table_observations`: 20 named-page matrix cells.
- `compact_observation`: fitting-table evidence.
- `keyboard_journey`: focused native horizontal scrolling evidence.
- `resize_journey`: fit and overflow state transition evidence.
- `zoom_observation`: true 200 percent page-scale evidence.
- `print_observation`: static print semantics and cue suppression.
- `no_script_observation`: script-free semantics and local containment.
- `failures`: ordered validation failures.

Receipt completeness:

- Exactly five named cases times two themes times two widths equals 20 observations.
- Keyboard, resize, zoom, print, blocked-script, and fitting-table evidence appears once each.
- S086 diagram, S087 syntax, and S088 figure receipts remain valid in the same run.
