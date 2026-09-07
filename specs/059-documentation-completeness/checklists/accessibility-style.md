# Accessibility and Style Checklist: Documentation Completeness

**Purpose**: Ensure the completed manual is readable, navigable, inclusive, and consistent with project prose policy.

**Created**: 2026-09-07

**Feature**: [spec.md](../spec.md)

## Structure and Navigation

- [x] CHK001 Every page has one descriptive H1 and a logical heading hierarchy with no skipped levels.
- [x] CHK002 Link text names its destination or task and does not use ambiguous phrases such as "click here".
- [x] CHK003 Each page identifies the intended task or contract near the start and links to the next likely action.
- [x] CHK004 Long procedures use ordered steps; unordered lists are reserved for choices or non-sequential facts.
- [x] CHK005 Tables have descriptive headers, simple cell structure, and an adjacent explanation when reading order or relationships are not obvious.
- [x] CHK006 Navigation labels and page titles use consistent, searchable player language while preserving exact UI labels where users must match the application.
- [x] CHK007 The same concept uses the same capitalization and terminology across task, concept, reference, and developer pages.

## Images, Diagrams, and Non-Text Content

- [x] CHK008 Every informative screenshot and image has concise alt text that explains its purpose rather than repeating its filename or nearby caption.
- [x] CHK009 Decorative images use empty alt text or an equivalent presentation-only treatment.
- [x] CHK010 Every diagram has an accessible text or table equivalent that conveys all nodes, decisions, transitions, and outcomes.
- [x] CHK011 Repository-owned SVG diagrams include a descriptive `<title>` and `<desc>` and remain understandable when CSS, color, or images are unavailable.
- [x] CHK012 Screenshots do not carry required instructions, status meanings, or exact values that are absent from surrounding text.
- [x] CHK013 Diagrams and annotations meet WCAG 2.2 AA text contrast and 3:1 meaningful graphical-object contrast in supported themes.
- [x] CHK014 State, severity, readiness, and flow are never communicated by red, green, purple, or any other color alone.
- [x] CHK015 Visuals remain legible at 200 percent browser zoom and reflow without horizontal page scrolling at 320 CSS pixels, except for intrinsically tabular or code content with an accessible scrolling treatment.
- [x] CHK016 No Mermaid syntax is added unless the repository first adopts, pins, tests, and documents a Mermaid-capable mdBook integration.

## Keyboard and Assistive Technology Content

- [x] CHK017 Interface guidance explains keyboard access for menus, Settings, disclosure, toggles, selectors, text fields, lifecycle buttons, Live Log, and truncated-value disclosure where supported.
- [x] CHK018 Dynamic status documentation uses the exact visible text and programmatic meaning, not only dot color, fill color, position, or animation.
- [x] CHK019 Resource guidance explains the visible label, numeric or non-numeric state, proportional fill, progress value, threshold, and stable Ready text as complementary cues.
- [x] CHK020 Collapse and expansion guidance names the section, its expanded state, keyboard activation, focus behavior, and persistence.
- [x] CHK021 Focus-dependent automation guidance distinguishes application keyboard focus from visible keyboard focus indicators in the documentation site.
- [x] CHK022 Any new custom documentation interaction has semantic HTML, keyboard parity, visible focus, correct expanded or current state, and no keyboard trap.
- [x] CHK023 Status changes that a reader must observe are described in text and do not require motion, flashing, hover, or audio.

## Language and Safety

- [x] CHK024 User pages lead with the action or outcome, define specialist terms on first use, and avoid exposing internal function names as instructions.
- [x] CHK025 Developer pages describe stable contracts before current implementation details and label the latter explicitly.
- [x] CHK026 Guarantees, observed behavior, provisional estimates, version-sensitive ESO facts, diagnosis, and recommended operator action are not blended into one unlabeled statement.
- [x] CHK027 Unknown, unavailable, inactive, empty, zero, blocked, dormant, and unsupported remain distinct terms wherever the application distinguishes them.
- [x] CHK028 Safety guidance never tells readers to disable focus gates, broad input protections, managed-marker checks, or fail-closed behavior.
- [x] CHK029 Linux permission guidance offers the narrow supported alternatives and does not recommend world-writable devices or broad recursive permission changes.
- [x] CHK030 Troubleshooting avoids destructive commands and identifies the exact recoverable target before any removal step.
- [x] CHK031 Privacy language distinguishes screen sampling, local files, structured logs, game process memory, network traffic, and the startup API version request.

## Project Prose and File Hygiene

- [x] CHK032 Prose avoids cliches, marketing inflation, unnecessary contrast formulas, and claims of compliance that were not actually tested.
- [x] CHK033 Sentences use commas, parentheses, colons, or standard hyphens; en dash and em dash characters are absent.
- [x] CHK034 Code identifiers, commands, paths, visible labels, and exact status strings use consistent Markdown formatting.
- [x] CHK035 External links point to authoritative primary sources, and version-sensitive claims include a repository or primary-source citation.
- [x] CHK036 No page copies large implementation excerpts when a stable contract, short example, or source link is clearer.
- [x] CHK037 All changed text and SVG files are UTF-8 without BOM, use LF endings, end with a newline, and contain no mojibake or trailing whitespace.
- [x] CHK038 Local site build, link check, documentation policy, keyboard navigation, focus visibility, 200 percent zoom, responsive reflow, dark theme, and light theme checks are recorded before completion.

