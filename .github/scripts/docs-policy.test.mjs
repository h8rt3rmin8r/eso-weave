import assert from "node:assert/strict";
import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import test from "node:test";

import {
  contrastRatio,
  validateBrandCss,
  validateBrandJavascript,
  validateCatalogSourceContract,
  projectEncounterMetrics,
  validateEncounterEvidence,
  validateEncounterFixture,
  validateEncounterModelContract,
  validateContentCoverage,
  validateContentCoverageRepository,
  validateCorpusSnapshot,
  validateGeneratedSite,
  validateMigrationLedger,
  validateSourceTree,
  validateSettingsRuntimeClaims,
  validateTextHygiene,
  validateWorkflowText,
} from "./docs-policy.mjs";

const requiredEncounterKinds = [
  "encounter-start", "encounter-end", "damage", "healing", "effect", "resource", "cast",
  "bar-change", "death", "resurrection", "boss-health", "performance", "quickslot", "discontinuity",
];
const requiredEncounterMetrics = [
  "observed-dps", "observed-hps", "ability-damage-share", "effect-uptime", "ordered-cast-sequence",
];

function validEncounterContract() {
  return {
    schema_version: 1,
    as_of: "2026-09-09",
    source_snapshots: [
      { id: "eso-api-live", channel: "live", revision: "f76cf16c4e5be7b234d15dc7f676febffa64c5bb", license: "technical-reference-only", uri: "https://github.com/esoui/esoui" },
      { id: "libcombat", channel: "not-applicable", revision: "80817e6929c7626832f9b9114d3b12bad8d642c1", license: "Artistic-2.0", uri: "https://github.com/solinur/LibCombat" },
      { id: "combat-metrics", channel: "not-applicable", revision: "6ec1deea4ef8801800dfe88ec79b1f94d0d6303b", license: "Artistic-2.0", uri: "https://github.com/solinur/CombatMetrics" },
    ],
    storage_planes: { catalog: "catalog.sqlite", raw: "user-owned-append-only", derived: "user-owned-rebuildable" },
    capture_envelope: { identity: ["session_id", "sequence"], duration_clock: "monotonic_ms", actor_identity: "encounter-local-opaque" },
    ordering_policy: { authority: "sequence", reject_duplicates: true, reject_undeclared_gaps: true, reject_backward_monotonic_time: true },
    loss_policy: { marker: "discontinuity", degrade_spanning_metrics: true, expose_ranges: true },
    privacy_policy: { local_only_default: true, upload_default: false, omitted_by_default: ["account-name", "character-name", "chat", "guild", "location"] },
    integrity_policy: { raw_immutable: true, derived_rebuildable: true, execute_input: false, bounded_import: true, atomic_import: true },
    catalog_join_policy: { retain_unknown_ids: true, rejoin_without_raw_mutation: true, preserve_channel: true },
    actor_policy: { identity: "encounter-local-opaque", roles: ["player", "pet", "npc", "boss"], pet_owner_relationship: "encounter-local-actor-id", ability_aliases: "derived-versioned-catalog-relationship" },
    build_snapshot_policy: { retention: "derived-versioned", catalog_version_required: true, consent_required_for_personal_identity: true },
    retention_policy: { export: "explicit-user-action", delete: "user-controlled-by-encounter-or-all", backup: "user-owned-with-schema-and-hash", corruption_recovery: "reject-invalid-import-and-preserve-last-valid-store", compression: "optional-local-gzip", production_budget: "verification-required" },
    recommendation_policy: { requires_encounter_version: true, requires_catalog_version: true, requires_metric_quality: true, correlation_is_not_causation: true, action_automation_coupling: false },
    catalog_schema_requirements: { entities: ["ability"], relationships: ["ability-alias"], join_key: "stable-numeric-source-id-plus-channel-and-api-version", unknown_id_supported: true },
    transport_policy: { pixel_bus_bulk_transport: false, automation_independent: true, future_transport: "bounded-saved-variables-import" },
    event_kinds: requiredEncounterKinds.map((id) => ({ id, raw: true })),
    metrics: requiredEncounterMetrics.map((id) => ({ id, algorithm_version: "s069-v1", source_range_required: true, quality_required: true })),
    parity_roadmap: requiredEncounterKinds.map((id, index) => ({ capability: id, source_event: id, calculation: "deterministic", confidence: "design-only", privacy_impact: "none", state: index === 0 ? "fixture-proved" : "verification-required", target_phase: "capture", acceptance: "evidence", owner: index === 0 ? "issue #113" : "issue #132", risk: "medium" })),
    follow_up_order: ["capture", "import", "calculation", "ui", "recommendations"],
    follow_up_issues: { capture: 132, import: 133, calculation: 134, ui: 135, recommendations: 136, live_parity_verification: 131 },
    synthetic_fixture: { encounter: "specs/069-encounter-model/fixtures/dummy-encounter.json", projection: "specs/069-encounter-model/fixtures/dummy-projection.json", proves_live_parity: false },
  };
}

function validEncounterFixture() {
  const base = { session_id: "s1", encounter_id: "e1" };
  const event = (sequence, monotonic_ms, kind, payload = {}) => ({ ...base, sequence, monotonic_ms, kind, payload });
  const events = [
    event(1, 0, "encounter-start"), event(2, 900, "cast", { actor_id: "a1", ability_id: 100 }),
    event(3, 1000, "damage", { source_actor_id: "a1", target_actor_id: "a2", ability_id: 100, amount: 1000, direction: "outgoing" }),
    event(4, 2000, "effect", { target_actor_id: "a1", ability_id: 200, change: "gained" }),
    event(5, 2500, "resource", { actor_id: "a1", resource: "magicka", value: 8000 }),
    event(6, 2800, "cast", { actor_id: "a1", ability_id: 999999 }),
    event(7, 3000, "damage", { source_actor_id: "a1", target_actor_id: "a2", ability_id: 999999, amount: 1500, direction: "outgoing" }),
    event(8, 4000, "healing", { source_actor_id: "a1", target_actor_id: "a1", ability_id: 300, effective_amount: 800, direction: "outgoing" }),
    event(9, 4500, "bar-change", { actor_id: "a1", bar: 2 }),
    event(12, 6000, "discontinuity", { missing_sequence_from: 10, missing_sequence_to: 11, reason: "capture-overflow" }),
    event(13, 6900, "cast", { actor_id: "a1", ability_id: 100 }),
    event(14, 7000, "damage", { source_actor_id: "a1", target_actor_id: "a2", ability_id: 100, amount: 500, direction: "outgoing" }),
    event(15, 7200, "boss-health", { actor_id: "a2", percent: 42 }), event(16, 7500, "death", { actor_id: "a1" }),
    event(17, 8000, "effect", { target_actor_id: "a1", ability_id: 200, change: "faded" }),
    event(18, 8500, "performance", { fps: 60, latency_ms: 70 }), event(19, 8800, "quickslot", { actor_id: "a1", slot: 1 }),
    event(20, 9000, "resurrection", { actor_id: "a1" }), event(21, 10000, "encounter-end"),
  ];
  return { envelope: { schema_version: 1, session_id: "s1", encounter_id: "e1", started_monotonic_ms: 0, ended_monotonic_ms: 10000, first_sequence: 1, last_sequence: 21, privacy_profile: "minimal-local" }, events };
}

const requiredCatalogCategories = [
  "player-skills",
  "crafted-abilities",
  "ability-metadata",
  "effects-and-status",
  "items-and-gear",
  "item-sets",
  "champion-skills",
  "consumables",
  "mundus-effects",
  "companions-races-classes",
  "combat-statistics",
  "constants",
  "localized-text",
  "icon-references",
  "icon-bytes",
];

function validCatalogContract() {
  const category = (id) => ({
    id,
    category: id,
    stable_key: `${id}-id`,
    enumeration_method: "api-iterator",
    visibility: ["channel"],
    completeness: "bounded",
    source_ids: ["live-api"],
    redistribution: id === "icon-bytes" ? "prohibited" : "allowed",
    failure_modes: ["partial"],
    validation: ["stable IDs are unique"],
    field_verification: "issue #129",
  });
  return {
    schema_version: 1,
    as_of: "2026-09-09",
    project_facts: {
      noncommercial: true,
      educational: true,
      telemetry: false,
      placeholders_allowed: true,
    },
    completeness_values: ["exhaustive", "bounded", "opportunistic", "unknown"],
    redistribution_values: ["allowed", "attribution-required", "user-generated-only", "prohibited", "unresolved"],
    source_snapshots: [
      {
        id: "live-api",
        family: "stock-ui",
        channel: "live",
        game_version: "12.0.8",
        api_version: 101050,
        locale: "all",
        revision: "f76cf16c4e5be7b234d15dc7f676febffa64c5bb",
        sha256: "f9faa484d8f2d875d00ab78b32e9e236e06922101b25e356f2101591db2e3d83",
        uri: "https://github.com/esoui/esoui/blob/f76cf16c4e5be7b234d15dc7f676febffa64c5bb/ESOUIDocumentation.txt",
        acquired_at: "2026-09-09",
        license_scope: "technical-reference-only",
        recommended: true,
      },
      {
        id: "pts-api",
        family: "stock-ui",
        channel: "pts",
        game_version: "12.1.4",
        api_version: 101051,
        locale: "all",
        revision: "1baf1131560c2bcd38ffd2bd070728273b25f934",
        sha256: "baf4e5173ce9ac478e76a5b81820b55bfe6525dfa45830b4dbc706916ba48055",
        uri: "https://github.com/esoui/esoui/blob/1baf1131560c2bcd38ffd2bd070728273b25f934/ESOUIDocumentation.txt",
        acquired_at: "2026-09-09",
        license_scope: "technical-reference-only",
        recommended: true,
      },
    ],
    promotion_policy: {
      automatic: false,
      requires_live_api_version: true,
      requires_live_revision: true,
      requires_content_hash: true,
      requires_reviewer: true,
      preserve_original_channel: true,
    },
    collector_policy: {
      format: "restricted-data-envelope",
      execute_lua: false,
      byte_limit_required: true,
      record_limit_required: true,
      atomic_import: true,
      upload_default: false,
      user_data_separate: true,
      provisional_max_snapshot_bytes: 67108864,
      provisional_max_records: 500000,
      provisional_max_string_bytes: 65536,
    },
    redistribution_decisions: {
      zenimax_icon_bytes: "prohibited",
      prebuilt_extracted_icon_pack: "prohibited",
    },
    categories: requiredCatalogCategories.map(category),
  };
}

const repositoryRoot = path.resolve(import.meta.dirname, "..", "..");

async function migrationLedger() {
  return JSON.parse(await readFile(path.join(repositoryRoot, "docs", "project", "migration-ledger.json"), "utf8"));
}

async function contentCoverage() {
  return JSON.parse(await readFile(path.join(repositoryRoot, "docs", "project", "content-coverage.json"), "utf8"));
}

function virtualCoverage(manifest) {
  const existingPaths = new Set(["docs/src/SUMMARY.md"]);
  const textFiles = new Map();
  const summaryEntries = new Set();
  for (const row of manifest.obligations) {
    existingPaths.add(row.destination);
    summaryEntries.add(row.destination);
    const prose = [...row.content_anchors, ...(row.search_terms ?? [])].join("\n");
    textFiles.set(row.destination, `${textFiles.get(row.destination) ?? "# Published page\n"}\n${prose}\n`);
    for (const reference of [...row.source_evidence, ...row.test_evidence]) {
      const evidence = typeof reference === "string" ? manifest.evidence[reference] : reference;
      if (evidence.kind === "TestGap") continue;
      existingPaths.add(evidence.path);
      textFiles.set(evidence.path, `${textFiles.get(evidence.path) ?? ""}\n${evidence.anchor}\n`);
    }
  }
  for (const page of manifest.pages) {
    existingPaths.add(page.path);
    summaryEntries.add(page.path);
    textFiles.set(page.path, `${textFiles.get(page.path) ?? "# Published page\n"}\n${page.required_anchors.join("\n")}\n`);
  }
  for (const entry of manifest.search_map) {
    existingPaths.add(entry.target);
    summaryEntries.add(entry.target);
    textFiles.set(entry.target, `${textFiles.get(entry.target) ?? "# Published page\n"}\n${entry.canonical}\n${entry.aliases.join("\n")}\n`);
  }
  for (const diagram of manifest.diagrams) {
    existingPaths.add(diagram.destination);
    summaryEntries.add(diagram.destination);
    textFiles.set(
      diagram.destination,
      `${textFiles.get(diagram.destination) ?? "# Published page\n"}\n${diagram.title}\n${diagram.text_equivalent}\n${diagram.alt_text}\n${diagram.content_anchors.join("\n")}\n`,
    );
  }
  return { existingPaths, summaryEntries, textFiles };
}

function virtualCorpus(ledger) {
  const existingPaths = new Set([
    "README.md",
    "docs/README.md",
    "docs/src/SUMMARY.md",
    "docs/project/build-plans/README.md",
    "docs/archive/build-plans/README.md",
  ]);
  for (const artifact of ledger.artifacts) {
    for (const destination of artifact.destinations) existingPaths.add(destination);
    if (artifact.replacement) existingPaths.add(artifact.replacement);
  }
  for (const unit of ledger.specificationUnits) {
    for (const destination of unit.destinations) existingPaths.add(destination.path);
  }
  for (const plan of ledger.plans) {
    existingPaths.add(plan.destination);
    for (const spec of plan.specs) existingPaths.add(spec);
  }
  for (const invariant of ledger.safetyCrosswalk) existingPaths.add(invariant.destination);
  for (const plan of ledger.postBaselinePlans) {
    existingPaths.add(plan.destination);
    for (const spec of plan.specs) existingPaths.add(spec);
  }
  const activeRows = ledger.postBaselinePlans
    .filter((plan) => plan.lifecycle === "Active")
    .map((plan) => `| [plan-${plan.id}.md](plan-${plan.id}.md) | Active | Current work |`)
    .join("\n");
  const archiveRows = ledger.postBaselinePlans
    .filter((plan) => plan.lifecycle === "Archived")
    .map((plan) => `| [${plan.id}](plan-${plan.id}.md) | Complete, Archived | [PR #99](https://example.com/pull/99) |`)
    .join("\n");
  const textFiles = new Map([
    ["README.md", "# ESO Weave\n\n[Documentation](https://h8rt3rmin8r.github.io/eso-weave/)\n"],
    ["docs/README.md", "# Documentation Lifecycle\n\n[Ledger](project/migration-ledger.md)\n[Ultimate archive](archive/website/ultimate-resource-meter.md)\n"],
    ["docs/book.toml", '[book]\nsrc = "src"\n'],
    ["docs/src/SUMMARY.md", "# Summary\n\n- [Home](README.md)\n"],
    ["docs/archive/website/ultimate-resource-meter.md", "[Canonical Ultimate](../../src/features/ultimate-resource.md)\n"],
    ["docs/project/build-plans/README.md", `# Current Build Plans\n\n| Plan | Status | Scope |\n| --- | --- | --- |\n${activeRows}\n`],
    ["docs/archive/build-plans/README.md", `# Archived Build Plans\n\n| Plan | Current disposition | Delivery evidence |\n| --- | --- | --- |\n${archiveRows}\n`],
  ]);
  for (const plan of ledger.postBaselinePlans) {
    textFiles.set(plan.destination, `# Plan ${plan.id}\n\nSubstantive plan record.\n`);
  }
  for (const unit of ledger.specificationUnits) {
    for (const destination of unit.destinations) {
      textFiles.set(destination.path, `${textFiles.get(destination.path) ?? "# Destination\n"}\n${destination.requiredExcerpt}\n`);
    }
  }
  for (const invariant of ledger.safetyCrosswalk) {
    textFiles.set(invariant.destination, `${textFiles.get(invariant.destination) ?? "# Destination\n"}\n${invariant.requiredExcerpt}\n`);
  }
  return {
    existingPaths,
    currentPaths: new Set(existingPaths),
    textFiles,
  };
}

async function fixture() {
  const root = await mkdtemp(path.join(tmpdir(), "eso-weave-docs-policy-"));
  const docs = path.join(root, "docs");
  const source = path.join(docs, "src");
  const output = path.join(root, "target", "docs-site", "html");
  await mkdir(path.join(source, "guide"), { recursive: true });
  await mkdir(path.join(source, "assets"), { recursive: true });
  await mkdir(path.join(output, "guide"), { recursive: true });
  await mkdir(path.join(output, "assets"), { recursive: true });
  await mkdir(path.join(output, "theme"), { recursive: true });

  await writeFile(
    path.join(source, "SUMMARY.md"),
    "# Summary\n\n- [Home](README.md)\n- [Guide](guide/README.md)\n",
  );
  await writeFile(path.join(source, "README.md"), "# Home\n\n[Guide](guide/#guide)\n");
  await writeFile(path.join(source, "guide", "README.md"), "# Guide\n\n![Mark](../assets/mark.svg)\n");
  await writeFile(path.join(source, "404.md"), '# Page Not Found\n\n<a href="/eso-weave/">Home</a>\n');
  await writeFile(path.join(source, "assets", "mark.svg"), "<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>\n");

  const html = (title, body) =>
    `<!doctype html><html lang="en"><head><link rel="stylesheet" href="/eso-weave/book.css"></head><body><main><h1>${title}</h1>${body}</main><script src="/eso-weave/searcher-test.js"></script><script src="/eso-weave/theme/eso-weave-test.js"></script></body></html>`;
  await writeFile(path.join(output, "index.html"), html("Home", '<a href="/eso-weave/guide/">Guide</a>'));
  await writeFile(
    path.join(output, "guide", "index.html"),
    html("Guide", '<img alt="Mark" src="/eso-weave/assets/mark.svg" srcset="data:image/svg+xml;base64,AAAA 1x, /eso-weave/assets/mark.svg 2x">'),
  );
  await writeFile(path.join(output, "404.html"), html("Page Not Found", '<a href="/eso-weave/">Home</a>'));
  await writeFile(path.join(output, "book.css"), "body { color: #e6edf3; background: #0e1116; }\n");
  await writeFile(path.join(output, "searcher-test.js"), "const search = true;\n");
  await writeFile(path.join(output, "searchindex-test.js"), "Object.assign(window.search, {doc_urls:[\"guide/index.html\"]});\n");
  await writeFile(
    path.join(output, "theme", "eso-weave-test.js"),
    'const main = document.querySelector("main"); main.id = "main-content"; main.tabIndex = -1; const link = {}; link.href = "#main-content"; link.textContent = "Skip to main content"; link.addEventListener("click", () => main.focus()); document.body.prepend(link);\n',
  );
  await writeFile(path.join(output, "assets", "mark.svg"), "<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>\n");

  return { root, docs, source, output };
}

test("accepts a complete, case-correct source tree", async (t) => {
  const f = await fixture();
  t.after(() => rm(f.root, { recursive: true, force: true }));
  assert.deepEqual(await validateSourceTree(f.docs), []);
});

test("rejects a published page omitted from SUMMARY", async (t) => {
  const f = await fixture();
  t.after(() => rm(f.root, { recursive: true, force: true }));
  await writeFile(path.join(f.source, "orphan.md"), "# Orphan\n");
  assert.match((await validateSourceTree(f.docs)).join("\n"), /not listed in SUMMARY\.md/);
});

test("does not treat a prose SUMMARY link as a navigation entry", async (t) => {
  const f = await fixture();
  t.after(() => rm(f.root, { recursive: true, force: true }));
  await writeFile(path.join(f.source, "orphan.md"), "# Orphan\n");
  await writeFile(
    path.join(f.source, "SUMMARY.md"),
    "# Summary\n\n- [Home](README.md)\n- [Guide](guide/README.md)\n\nSee [Orphan](orphan.md).\n",
  );
  assert.match((await validateSourceTree(f.docs)).join("\n"), /orphan\.md is not listed/);
});

test("rejects duplicate, escaping, and case-mismatched SUMMARY paths", async (t) => {
  const f = await fixture();
  t.after(() => rm(f.root, { recursive: true, force: true }));
  await writeFile(
    path.join(f.source, "SUMMARY.md"),
    "# Summary\n\n- [Home](README.md)\n- [Again](README.md)\n- [Case](Guide/README.md)\n- [Escape](../../README.md)\n",
  );
  const errors = (await validateSourceTree(f.docs)).join("\n");
  assert.match(errors, /listed more than once/);
  assert.match(errors, /escapes docs\/src/);
  assert.match(errors, /case does not match/);
});

test("rejects a local source link with a missing target or fragment", async (t) => {
  const f = await fixture();
  t.after(() => rm(f.root, { recursive: true, force: true }));
  await writeFile(
    path.join(f.source, "README.md"),
    "# Home\n\n[Missing](missing.md)\n[Bad heading](guide/#absent)\n",
  );
  const errors = (await validateSourceTree(f.docs)).join("\n");
  assert.match(errors, /missing local target/);
  assert.match(errors, /missing fragment/);
});

test("accepts Markdown destination titles and balanced parentheses", async (t) => {
  const f = await fixture();
  t.after(() => rm(f.root, { recursive: true, force: true }));
  await writeFile(path.join(f.source, "guide", "(advanced).md"), "# Advanced\n");
  await writeFile(
    path.join(f.source, "SUMMARY.md"),
    '# Summary\n\n- [Home](README.md)\n- [Guide](guide/README.md "Setup guide")\n- [Advanced](guide/(advanced).md "Advanced guide")\n',
  );
  await writeFile(path.join(f.source, "README.md"), '# Home\n\n[Guide](guide/ "Setup guide")\n');
  assert.deepEqual(await validateSourceTree(f.docs), []);
});

test("ignores Markdown links inside inline code and fenced examples", async (t) => {
  const f = await fixture();
  t.after(() => rm(f.root, { recursive: true, force: true }));
  await writeFile(
    path.join(f.source, "README.md"),
    "# Home\n\n`[inline](missing-inline.md)`\n\n```markdown\n[fenced](missing-fenced.md)\n```\n\n~~~\n[tilde](missing-tilde.md)\n~~~\n\n[Guide](guide/)\n",
  );
  assert.deepEqual(await validateSourceTree(f.docs), []);
});

test("rejects inline README links that mdBook renders as missing README.html", async (t) => {
  const f = await fixture();
  t.after(() => rm(f.root, { recursive: true, force: true }));
  await writeFile(path.join(f.source, "README.md"), "# Home\n\n[Guide](guide/README.md)\n");
  assert.match((await validateSourceTree(f.docs)).join("\n"), /use the directory URL/);
});

test("accepts a complete generated site mounted below the repository path", async (t) => {
  const f = await fixture();
  t.after(() => rm(f.root, { recursive: true, force: true }));
  assert.deepEqual(await validateGeneratedSite(f.output, "/eso-weave/"), []);
});

test("rejects remote runtime resources and missing generated assets", async (t) => {
  const f = await fixture();
  t.after(() => rm(f.root, { recursive: true, force: true }));
  await writeFile(
    path.join(f.output, "index.html"),
    '<!doctype html><html lang="en"><head><link rel="stylesheet" href="../../outside.css"></head><body><main><h1>Home</h1><img alt="Remote" src="https://example.com/logo.svg"><img alt="Unquoted" src=https://example.com/unquoted.png><img alt="Candidates" srcset="local.png 1x, https://example.com/remote.png 2x"><iframe src="https://example.com/embed"></iframe><iframe src=/outside></iframe><video poster="https://example.com/poster.jpg"></video><object data="https://example.com/object"></object><embed src="https://example.com/plugin"><svg><image href="https://example.com/image.svg"></image><use xlink:href="https://example.com/sprite.svg#icon"></use></svg><script src="/eso-weave/missing.js"></script></main></body></html>',
  );
  const errors = (await validateGeneratedSite(f.output, "/eso-weave/")).join("\n");
  assert.ok((errors.match(/remote runtime resource/gu)?.length ?? 0) >= 9);
  assert.match(errors, /missing generated resource/);
  assert.match(errors, /resource escapes site base/);
});

test("rejects remote and missing CSS resources embedded in HTML", async (t) => {
  const f = await fixture();
  t.after(() => rm(f.root, { recursive: true, force: true }));
  await writeFile(
    path.join(f.output, "index.html"),
    '<!doctype html><html lang="en"><head><style>@import "https://example.com/theme.css"; .hero { background: url("assets/missing.png"); }</style></head><body><main><h1 style="background:url(https://example.com/pixel.png)">Home</h1><p style=background:url(https://example.com/unquoted.png)>Copy</p></main><script src="/eso-weave/searcher-test.js"></script></body></html>',
  );
  const errors = (await validateGeneratedSite(f.output, "/eso-weave/")).join("\n");
  assert.equal(errors.match(/remote CSS runtime resource/gu)?.length, 3);
  assert.match(errors, /missing CSS resource assets\/missing\.png/);
});

test("requires the 404 recovery link to target the site root", async (t) => {
  const f = await fixture();
  t.after(() => rm(f.root, { recursive: true, force: true }));
  await writeFile(path.join(f.output, "404.html"), '<!doctype html><html lang="en"><body><main><h1>Page Not Found</h1><a href="./">Home</a></main></body></html>');
  assert.match((await validateGeneratedSite(f.output, "/eso-weave/")).join("\n"), /missing site-root recovery link/);
});

test("tokenizes mixed data and local srcset candidates", async (t) => {
  const f = await fixture();
  t.after(() => rm(f.root, { recursive: true, force: true }));
  await writeFile(
    path.join(f.output, "index.html"),
    '<!doctype html><html lang="en"><body><main><h1>Home</h1><img alt="First" srcset="data:image/png;base64,AAAA 1x, assets/missing-one.png 2x"><img alt="Middle" srcset="assets/mark.svg 1x, data:image/png;base64,BBBB 2x, assets/missing-two.png 3x"></main><script src="/eso-weave/searcher-test.js"></script><script src="/eso-weave/theme/eso-weave-test.js"></script></body></html>',
  );
  const errors = (await validateGeneratedSite(f.output, "/eso-weave/")).join("\n");
  assert.equal(errors.match(/missing generated resource/gu)?.length, 2);
  assert.match(errors, /missing-one\.png/);
  assert.match(errors, /missing-two\.png/);
});

test("rejects an edit URL that duplicates the docs/src path", async (t) => {
  const f = await fixture();
  t.after(() => rm(f.root, { recursive: true, force: true }));
  await writeFile(
    path.join(f.output, "index.html"),
    '<!doctype html><html lang="en"><body><main><h1>Home</h1><a href="https://github.com/example/project/edit/main/docs/src/src/README.md" rel="edit">Edit</a></main></body></html>',
  );
  assert.match((await validateGeneratedSite(f.output, "/eso-weave/")).join("\n"), /edit link duplicates/);
});

test("rejects missing same-page and nested generated fragments", async (t) => {
  const f = await fixture();
  t.after(() => rm(f.root, { recursive: true, force: true }));
  await writeFile(
    path.join(f.output, "index.html"),
    '<!doctype html><html lang="en"><body><main><h1 id="home">Home</h1><p id="home">Duplicate</p><a href="#absent">Same</a><a href="guide/#absent">Nested</a></main><script src="/eso-weave/searcher-test.js"></script></body></html>',
  );
  const errors = (await validateGeneratedSite(f.output, "/eso-weave/")).join("\n");
  assert.equal(errors.match(/missing generated fragment/gu)?.length, 2);
  assert.match(errors, /duplicate id home/);
});

test("rejects missing and escaping local CSS resources", async (t) => {
  const f = await fixture();
  t.after(() => rm(f.root, { recursive: true, force: true }));
  await writeFile(
    path.join(f.output, "book.css"),
    '@font-face { src: url("assets/missing.woff2"); } .bad { background: url("../../outside.png"); }\n',
  );
  const errors = (await validateGeneratedSite(f.output, "/eso-weave/")).join("\n");
  assert.match(errors, /missing CSS resource/);
  assert.match(errors, /CSS resource escapes site/);
});

test("requires brand tokens, visible focus, reduced motion, and responsive targets", () => {
  const validCss = `
    :root {
      --eso-ink: #0e1116; --eso-gold: #f2b03c; --eso-teal: #2dd4bf;
      --eso-text: #e6edf3; --eso-focus: #f2b03c;
    }
    @font-face { font-family: "Inter"; src: url("../assets/brand/fonts/Inter-Regular.ttf"); }
    @font-face { font-family: "Inter"; src: url("../assets/brand/fonts/Inter-SemiBold.ttf"); }
    .light, .rust {
      --bg: #f7f5f0; --fg: #14110b; --links: #075e57;
      --sidebar-bg: #ffffff; --table-header-bg: #ece8de; --eso-focus: #704800;
    }
    :focus-visible { outline: 3px solid var(--eso-focus); }
    @media (prefers-reduced-motion: reduce) { * { scroll-behavior: auto; } }
    @media (max-width: 40rem) { button { min-height: 44px; min-width: 44px; } }
  `;
  assert.deepEqual(validateBrandCss(validCss), []);
  assert.match(validateBrandCss(":root {}").join("\n"), /focus-visible/);
});

test("rejects text and link colors below WCAG AA normal-text contrast", () => {
  assert.ok(contrastRatio("#e6edf3", "#0e1116") >= 4.5);
  assert.ok(contrastRatio("#2dd4bf", "#0e1116") >= 4.5);
  assert.ok(contrastRatio("#14110b", "#f7f5f0") >= 4.5);
  assert.ok(contrastRatio("#075e57", "#f7f5f0") >= 4.5);

  const lowContrast = `
    :root {
      --eso-ink: #777777; --eso-gold: #f2b03c; --eso-teal: #888888;
      --eso-text: #888888; --eso-focus: #888888;
    }
    @font-face { font-family: "Inter"; src: url("../assets/brand/fonts/Inter-Regular.ttf"); }
    @font-face { font-family: "Inter"; src: url("../assets/brand/fonts/Inter-SemiBold.ttf"); }
    .light, .rust {
      --bg: #ffffff; --fg: #eeeeee; --links: #eeeeee;
      --sidebar-bg: #ffffff; --table-header-bg: #eeeeee; --eso-focus: #eeeeee;
    }
    :focus-visible { outline: 3px solid var(--eso-focus); }
    @media (prefers-reduced-motion: reduce) { * { scroll-behavior: auto; } }
    @media (max-width: 40rem) { button { min-height: 44px; } }
  `;
  assert.match(validateBrandCss(lowContrast).join("\n"), /contrast is below 4\.5:1/);
  assert.match(validateBrandCss(lowContrast).join("\n"), /focus contrast is below 3:1/);
});

test("requires a first-control skip link targeting main content", () => {
  const valid = 'const main = document.querySelector("main"); main.id = "main-content"; main.tabIndex = -1; const link = {}; link.href = "#main-content"; link.textContent = "Skip to main content"; link.addEventListener("click", () => main.focus()); document.body.prepend(link);';
  assert.deepEqual(validateBrandJavascript(valid), []);
  assert.match(validateBrandJavascript("const main = true;").join("\n"), /main-content target/);
  assert.match(validateBrandJavascript('const main = document.querySelector("main"); main.id = "main-content";').join("\n"), /transfer focus/);
});

test("accepts main-only Pages deployment with job-scoped writes", () => {
  const validWorkflow = `
name: docs
on: [push, pull_request, workflow_dispatch]
permissions:
  contents: read
jobs:
  build:
    permissions:
      contents: read
    steps:
      - name: Install pinned documentation tools
        run: cargo install typos-cli --version '=1.50.1' --locked
      - name: Check spelling
        run: typos docs/src docs/README.md README.md
      - name: Configure GitHub Pages
        if: github.ref == 'refs/heads/main' && github.event_name != 'pull_request'
        uses: actions/configure-pages@cccccccccccccccccccccccccccccccccccccccc
      - name: Upload checked Pages artifact
        if: github.ref == 'refs/heads/main' && github.event_name != 'pull_request'
        uses: actions/upload-pages-artifact@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
  deploy:
    if: github.ref == 'refs/heads/main' && github.event_name != 'pull_request'
    needs: build
    permissions:
      pages: write
      id-token: write
    environment:
      name: github-pages
    steps:
      - uses: actions/deploy-pages@bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
  `;
  assert.deepEqual(validateWorkflowText(validWorkflow), []);
});

test("S059 requires the exact pinned spelling tool and spelling gate", () => {
  const workflow = `
name: docs
permissions:
  contents: read
jobs:
  build:
    permissions:
      contents: read
    steps:
      - name: Install pinned documentation tools
        run: cargo install typos-cli --version '=1.50.1' --locked
      - name: Check spelling
        run: typos docs/src docs/README.md README.md
      - name: Configure GitHub Pages
        if: github.ref == 'refs/heads/main' && github.event_name != 'pull_request'
        uses: actions/configure-pages@cccccccccccccccccccccccccccccccccccccccc
      - name: Upload checked Pages artifact
        if: github.ref == 'refs/heads/main' && github.event_name != 'pull_request'
        uses: actions/upload-pages-artifact@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
  deploy:
    if: github.ref == 'refs/heads/main' && github.event_name != 'pull_request'
    needs: build
    permissions:
      pages: write
      id-token: write
    environment:
      name: github-pages
    steps:
      - uses: actions/deploy-pages@bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
`;
  assert.deepEqual(validateWorkflowText(workflow), []);
  assert.match(validateWorkflowText(workflow.replace("'=1.50.1'", "'=1.50.2'")).join("\n"), /typos-cli 1\.50\.1/);
  assert.match(validateWorkflowText(workflow.replace("run: typos docs/src docs/README.md README.md", "run: typos docs/src")).join("\n"), /spelling gate/);

  const unreachable = workflow
    .replace("run: cargo install typos-cli --version '=1.50.1' --locked", "run: echo omitted install")
    .replace("run: typos docs/src docs/README.md README.md", "run: echo omitted spelling") + `
junk:
  run: cargo install typos-cli --version '=1.50.1' --locked
  spelling: |
    run: typos docs/src docs/README.md README.md
`;
  assert.match(validateWorkflowText(unreachable).join("\n"), /build job.*typos-cli/);
  assert.match(validateWorkflowText(unreachable).join("\n"), /build job.*spelling gate/);

  const reversed = workflow.replace(
    `      - name: Install pinned documentation tools
        run: cargo install typos-cli --version '=1.50.1' --locked
      - name: Check spelling
        run: typos docs/src docs/README.md README.md`,
    `      - name: Check spelling
        run: typos docs/src docs/README.md README.md
      - name: Install pinned documentation tools
        run: cargo install typos-cli --version '=1.50.1' --locked`,
  );
  assert.match(validateWorkflowText(reversed).join("\n"), /install pinned typos-cli before/);
});

test("rejects write escalation and guards outside their required scopes", () => {
  const unsafeWorkflow = `
name: docs
on: [push, pull_request]
permissions:
  contents: read
jobs:
  build:
    permissions:
      contents: read
      pages: write
    steps:
      - name: Configure GitHub Pages
        uses: actions/configure-pages@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
      - name: Upload checked Pages artifact
        uses: actions/upload-pages-artifact@bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
      # if: github.ref == 'refs/heads/main' && github.event_name != 'pull_request'
  deploy:
    needs: build
    permissions:
      pages: write
      id-token: write
    environment:
      name: github-pages
    steps:
      - uses: actions/deploy-pages@cccccccccccccccccccccccccccccccccccccccc
  unrelated:
    if: github.ref == 'refs/heads/main' && github.event_name != 'pull_request'
    steps: []
  `;
  const errors = validateWorkflowText(unsafeWorkflow).join("\n");
  assert.match(errors, /build job permissions/);
  assert.match(errors, /deploy job requires a main-only guard/);
  assert.match(errors, /Configure GitHub Pages step requires a main-only guard/);
  assert.match(errors, /Upload checked Pages artifact step requires a main-only guard/);
});

test("rejects write-all permissions", () => {
  assert.match(validateWorkflowText("permissions: write-all\njobs:\n").join("\n"), /write-all/);
});

test("rejects pull_request_target and top-level Pages writes", () => {
  const unsafeWorkflow = `
on: pull_request_target
permissions:
  contents: read
  pages: write
  id-token: write
jobs:
  deploy:
    steps:
      - uses: actions/deploy-pages@main
  `;
  const errors = validateWorkflowText(unsafeWorkflow).join("\n");
  assert.match(errors, /pull_request_target/);
  assert.match(errors, /top-level Pages write/);
  assert.match(errors, /immutable commit SHA/);
  assert.match(errors, /main-only guard/);
});

test("accepts the frozen migration ledger and its virtual post-migration corpus", async () => {
  const ledger = await migrationLedger();
  const corpus = virtualCorpus(ledger);
  assert.deepEqual(validateMigrationLedger(ledger, corpus), []);
  assert.deepEqual(validateCorpusSnapshot(ledger, corpus), []);
});

test("rejects omitted and duplicate baseline artifacts", async () => {
  const ledger = await migrationLedger();
  const omitted = structuredClone(ledger);
  omitted.artifacts.pop();
  assert.match(validateMigrationLedger(omitted, virtualCorpus(omitted)).join("\n"), /baseline artifact coverage/);

  const duplicate = structuredClone(ledger);
  duplicate.artifacts[1] = structuredClone(duplicate.artifacts[0]);
  assert.match(validateMigrationLedger(duplicate, virtualCorpus(duplicate)).join("\n"), /duplicate baseline artifact/);
});

test("rejects dangling and unsafe dispositions", async () => {
  const ledger = await migrationLedger();
  const dangling = structuredClone(ledger);
  dangling.artifacts.find((artifact) => artifact.disposition === "Move").destinations = ["docs/project/missing.md"];
  assert.match(validateMigrationLedger(dangling, virtualCorpus(ledger)).join("\n"), /destination does not exist/);

  const unsafe = structuredClone(ledger);
  const deleted = unsafe.artifacts.find((artifact) => artifact.disposition === "Delete");
  delete deleted.replacement;
  deleted.evidence = null;
  assert.match(validateMigrationLedger(unsafe, virtualCorpus(unsafe)).join("\n"), /Delete requires a replacement and evidence/);

  const misplaced = structuredClone(ledger);
  misplaced.artifacts.find((artifact) => artifact.disposition === "Archive").destinations = ["docs/project/history.md"];
  assert.match(validateMigrationLedger(misplaced, virtualCorpus(misplaced)).join("\n"), /Archive destination must be under docs\/archive/);
});

test("rejects project or archive publication and stale live paths", async () => {
  const ledger = await migrationLedger();
  const published = virtualCorpus(ledger);
  published.textFiles.set(
    "docs/src/SUMMARY.md",
    "# Summary\n\n- [History](../archive/build-plans/README.md)\n- [Governance](../project/governance.md)\n",
  );
  assert.match(validateCorpusSnapshot(ledger, published).join("\n"), /published navigation crosses the docs\/src boundary/);

  const stale = virtualCorpus(ledger);
  stale.textFiles.set("CONTRIBUTING.md", "Follow docs/releasing.md.\n");
  assert.match(validateCorpusSnapshot(ledger, stale).join("\n"), /stale live documentation path/);
});

test("requires exact historical exceptions", async () => {
  const ledger = await migrationLedger();
  const wildcard = structuredClone(ledger);
  wildcard.historicalExceptions.push({ file: "CHANGELOG.md", literal: "docs/*", occurrences: 1, reason: "Too broad." });
  assert.match(validateMigrationLedger(wildcard, virtualCorpus(wildcard)).join("\n"), /historical exception must be exact/);

  const unmatched = virtualCorpus(ledger);
  const exception = ledger.historicalExceptions[0];
  unmatched.textFiles.set(exception.file, `${exception.literal}\n`.repeat(exception.occurrences + 1));
  assert.match(validateCorpusSnapshot(ledger, unmatched).join("\n"), /historical exception occurrence mismatch/);

  const liveBypass = structuredClone(ledger);
  liveBypass.historicalExceptions.push({
    file: "README.md",
    literal: "docs/releasing.md",
    occurrences: 1,
    reason: "This must not exempt a live file.",
  });
  assert.match(validateMigrationLedger(liveBypass, virtualCorpus(liveBypass)).join("\n"), /approved CHANGELOG exceptions/);
});

test("requires contiguous Complete and Archived legacy plans", async () => {
  const ledger = await migrationLedger();
  const broken = structuredClone(ledger);
  broken.plans[4].id = "099";
  broken.plans[5].completion = "Pending";
  broken.plans[6].lifecycle = "Active";
  const errors = validateMigrationLedger(broken, virtualCorpus(broken)).join("\n");
  assert.match(errors, /plan IDs must be contiguous from 001 through 027/);
  assert.match(errors, /must be Complete and Archived/);

  const vague = structuredClone(ledger);
  vague.plans[0].evidence = "Implementation exists somewhere.";
  assert.match(validateMigrationLedger(vague, virtualCorpus(vague)).join("\n"), /plan 001 requires spec and delivery evidence/);
});

test("freezes specification units, safety invariants, and baseline blobs", async () => {
  const ledger = await migrationLedger();
  const broken = structuredClone(ledger);
  broken.artifacts[0].blob = "0000000000000000000000000000000000000000";
  broken.specificationUnits[1] = structuredClone(broken.specificationUnits[0]);
  broken.safetyCrosswalk.pop();
  const errors = validateMigrationLedger(broken, virtualCorpus(broken)).join("\n");
  assert.match(errors, /frozen baseline blob does not match/);
  assert.match(errors, /specification unit coverage/);
  assert.match(errors, /safety crosswalk must contain the six required invariants/);

  const weakened = structuredClone(ledger);
  weakened.specificationUnits[1].destinations[0].requiredExcerpt = "#";
  assert.match(validateMigrationLedger(weakened, virtualCorpus(weakened)).join("\n"), /preservation excerpts do not match the frozen manifest/);
});

test("rejects erased specification units and safety statements", async () => {
  const ledger = await migrationLedger();
  const corpus = virtualCorpus(ledger);
  assert.deepEqual(validateMigrationLedger(ledger, corpus), []);

  const erasedUnit = structuredClone(corpus);
  const overview = ledger.specificationUnits.find((unit) => unit.heading === "1. Overview");
  erasedUnit.textFiles = new Map(corpus.textFiles);
  erasedUnit.textFiles.set(overview.destinations[0].path, "# Empty destination\n");
  assert.match(validateMigrationLedger(ledger, erasedUnit).join("\n"), /required excerpt is missing/);

  const erasedSafety = structuredClone(corpus);
  const signalLoss = ledger.safetyCrosswalk.find((item) => item.id === "fishing-signal-loss-fail-closed");
  erasedSafety.textFiles = new Map(corpus.textFiles);
  erasedSafety.textFiles.set(signalLoss.destination, "# Fishing\n");
  assert.match(validateMigrationLedger(ledger, erasedSafety).join("\n"), /safety excerpt is missing/);

  const multiDestination = ledger.specificationUnits.find((unit) => unit.heading === "10. PixelBeacon Companion Addon");
  const exactUltimate = multiDestination.destinations.find((item) => item.requiredExcerpt.includes("B25 Ultimate Current"));
  const erasedProtocolFact = structuredClone(corpus);
  erasedProtocolFact.textFiles = new Map(corpus.textFiles);
  erasedProtocolFact.textFiles.set(
    exactUltimate.path,
    erasedProtocolFact.textFiles.get(exactUltimate.path).replace(exactUltimate.requiredExcerpt, ""),
  );
  assert.match(validateMigrationLedger(ledger, erasedProtocolFact).join("\n"), /10\. PixelBeacon Companion Addon: required excerpt is missing/);

  for (const phrase of [
    "R carries bits 0 through 7",
    "GetUnitPower",
    "Values from 0 through 510",
    "out-of-range maximum makes both current and maximum unavailable",
    "GetSlotAbilityCost",
    "HOTBAR_CATEGORY_PRIMARY",
    "HOTBAR_CATEGORY_BACKUP",
    "unused slot, nil API result",
    "sampled and published together",
    "special or temporary hotbar",
  ]) {
    const anchor = multiDestination.destinations.find((item) => item.requiredExcerpt.includes(phrase));
    assert.ok(anchor, `missing test anchor for ${phrase}`);
    const removed = structuredClone(corpus);
    removed.textFiles = new Map(corpus.textFiles);
    removed.textFiles.set(anchor.path, removed.textFiles.get(anchor.path).replace(anchor.requiredExcerpt, ""));
    assert.match(
      validateMigrationLedger(ledger, removed).join("\n"),
      /10\. PixelBeacon Companion Addon: required excerpt is missing/,
      phrase,
    );
  }

  for (const [heading, phrases] of [
    ["7. Input Engine", [
      "menu gate can only relax interception",
      "addon too old to publish the gate",
      "sample that fails validation",
      "lost beacon signal",
      "Unavailable menu evidence fails closed",
      "sink still releases any mouse button",
      "slow low-level hook callback",
      "below the display server and behaves identically",
      "capture-style rebinding control",
      "rejects conflicting assignments",
    ]],
    ["11. Auto-Potion", [
      "Quickslot key defaults",
      "Every resource watch is off by default",
      "OR rule is not configurable",
      "Until the next evaluation",
      "controller ticks on the pixel-bus worker",
      "logging records categorical effective-state changes",
      "treating unknown as permissive",
      "checks menu and suspension directly",
      "blocks action without clearing the requested setting",
      "World State is positively observed as Active",
      "Travel is positively observed as Inactive",
      "Explicit Sprinting is not present",
      "Roll Dodge is not an Auto Potion prerequisite",
      "never reaches the hook thread",
    ]],
    ["12. Graphical User Interface", [
      "resource meter is unanimated",
      "programmatic progress value",
      "Observed zero remains a numeric empty bar",
      "Dormant and unavailable states have no numeric value",
      "WCAG 2.2 AA contrast",
      "color is never the only state cue",
      "compact group boundary follows Ultimate before Game Context",
      "identical text for both access paths",
      "fixed trailing region is reserved only for System and State",
      "At most two lifecycle actions",
      "primary-then-secondary horizontal row",
      "managed-marker uninstall guard and confirmation",
      "grows sub-linearly with the window on both axes",
      "colorizes events by level",
      "resizable between a six-line readable minimum and the space above it",
    ]],
  ]) {
    const unit = ledger.specificationUnits.find((item) => item.heading === heading);
    for (const phrase of phrases) {
      const anchor = unit.destinations.find((item) => item.requiredExcerpt.includes(phrase));
      assert.ok(anchor, `missing test anchor for ${phrase}`);
      const removed = structuredClone(corpus);
      removed.textFiles = new Map(corpus.textFiles);
      removed.textFiles.set(anchor.path, removed.textFiles.get(anchor.path).replace(anchor.requiredExcerpt, ""));
      assert.match(validateMigrationLedger(ledger, removed).join("\n"), /required excerpt is missing/, phrase);
    }
  }
});

test("requires the table of contents unit to map to SUMMARY", async () => {
  const ledger = await migrationLedger();
  const broken = structuredClone(ledger);
  broken.specificationUnits[0].destinations[0].path = "docs/src/README.md";
  assert.match(validateMigrationLedger(broken, virtualCorpus(broken)).join("\n"), /Table of Contents.*SUMMARY\.md/);
});

test("derives post-baseline lifecycle and permits an evidenced archive followed by a new active plan", async () => {
  const ledger = await migrationLedger();
  const baseline = virtualCorpus(ledger);
  assert.deepEqual(validateMigrationLedger(ledger, baseline), []);
  assert.deepEqual(validateCorpusSnapshot(ledger, baseline), []);

  const withActive = structuredClone(ledger);
  let activeIndex = withActive.postBaselinePlans.findIndex((plan) => plan.lifecycle === "Active");
  if (activeIndex === -1) {
    const latestId = withActive.postBaselinePlans.at(-1).id;
    const activeId = String(Number(latestId) + 1).padStart(3, "0");
    withActive.postBaselinePlans.push({
      id: activeId,
      completion: "In Progress",
      lifecycle: "Active",
      destination: `docs/project/build-plans/plan-${activeId}.md`,
      specs: [`specs/${activeId}-active-slice`],
      evidence: "Issue #100 tracks the active delivery.",
    });
    activeIndex = withActive.postBaselinePlans.length - 1;
  }
  const active = virtualCorpus(withActive);
  assert.deepEqual(validateMigrationLedger(withActive, active), []);
  assert.deepEqual(validateCorpusSnapshot(withActive, active), []);

  const transitioned = structuredClone(withActive);
  const activeId = transitioned.postBaselinePlans[activeIndex].id;
  transitioned.postBaselinePlans[activeIndex] = {
    ...transitioned.postBaselinePlans[activeIndex],
    completion: "Complete",
    lifecycle: "Archived",
    destination: `docs/archive/build-plans/plan-${activeId}.md`,
    evidence: "Merged in PR #99.",
  };
  const nextId = String(Number(activeId) + 1).padStart(3, "0");
  transitioned.postBaselinePlans.push({
    id: nextId,
    completion: "In Progress",
    lifecycle: "Active",
    destination: `docs/project/build-plans/plan-${nextId}.md`,
    specs: [`specs/${nextId}-next-slice`],
    evidence: "Issue #101 tracks the active delivery.",
  });
  const archivedAndNext = virtualCorpus(transitioned);
  assert.deepEqual(validateMigrationLedger(transitioned, archivedAndNext), []);
  assert.deepEqual(validateCorpusSnapshot(transitioned, archivedAndNext), []);

  const erasedArchive = structuredClone(archivedAndNext);
  erasedArchive.textFiles = new Map(archivedAndNext.textFiles);
  erasedArchive.textFiles.set(`docs/archive/build-plans/plan-${activeId}.md`, `# Plan ${activeId}\n`);
  assert.match(validateMigrationLedger(transitioned, erasedArchive).join("\n"), /invalid Archived lifecycle/);

  const ambiguous = structuredClone(archivedAndNext);
  ambiguous.currentPaths = new Set(archivedAndNext.currentPaths);
  ambiguous.existingPaths = new Set(archivedAndNext.existingPaths);
  ambiguous.currentPaths.add(`docs/project/build-plans/plan-${activeId}.md`);
  ambiguous.existingPaths.add(`docs/project/build-plans/plan-${activeId}.md`);
  assert.match(
    [...validateMigrationLedger(transitioned, ambiguous), ...validateCorpusSnapshot(transitioned, ambiguous)].join("\n"),
    /(?:invalid Archived lifecycle|active project build plan must match)/,
  );

  const inverted = structuredClone(transitioned);
  inverted.postBaselinePlans[activeIndex] = {
    ...inverted.postBaselinePlans[activeIndex],
    completion: "In Progress",
    lifecycle: "Active",
    destination: `docs/project/build-plans/plan-${activeId}.md`,
    evidence: "Issue #80 tracks active delivery.",
  };
  inverted.postBaselinePlans[inverted.postBaselinePlans.length - 1] = {
    ...inverted.postBaselinePlans[inverted.postBaselinePlans.length - 1],
    completion: "Complete",
    lifecycle: "Archived",
    destination: `docs/archive/build-plans/plan-${nextId}.md`,
    evidence: "Merged in PR #100.",
  };
  assert.match(
    validateMigrationLedger(inverted, virtualCorpus(inverted)).join("\n"),
    /Active post-baseline plan must be the final entry/,
  );
});

test("does not accept plan index rows hidden in code or comments", async () => {
  const ledger = await migrationLedger();
  const activeLedger = structuredClone(ledger);
  let activePlan = activeLedger.postBaselinePlans.find((plan) => plan.lifecycle === "Active");
  if (activePlan === undefined) {
    const latestId = activeLedger.postBaselinePlans.at(-1).id;
    const activeId = String(Number(latestId) + 1).padStart(3, "0");
    activePlan = {
      id: activeId,
      completion: "In Progress",
      lifecycle: "Active",
      destination: `docs/project/build-plans/plan-${activeId}.md`,
      specs: [`specs/${activeId}-active-slice`],
      evidence: "Issue #100 tracks the active delivery.",
    };
    activeLedger.postBaselinePlans.push(activePlan);
  }
  const active = virtualCorpus(activeLedger);
  active.textFiles.set(
    "docs/project/build-plans/README.md",
    `# Current Build Plans\n\n\`\`\`markdown\n| [plan-${activePlan.id}.md](plan-${activePlan.id}.md) | Active | Hidden |\n\`\`\`\n`,
  );
  assert.match(validateCorpusSnapshot(activeLedger, active).join("\n"), /missing its Active index row/);

  const archivedLedger = structuredClone(ledger);
  archivedLedger.postBaselinePlans[0] = {
    ...archivedLedger.postBaselinePlans[0],
    completion: "Complete",
    lifecycle: "Archived",
    destination: "docs/archive/build-plans/plan-028.md",
    evidence: "Merged in PR #99.",
  };
  const archived = virtualCorpus(archivedLedger);
  archived.textFiles.set(
    "docs/archive/build-plans/README.md",
    "# Archived Build Plans\n\n<!--\n| [028](plan-028.md) | Complete, Archived | [PR #99](https://example.com/pull/99) |\n-->\n",
  );
  assert.match(validateCorpusSnapshot(archivedLedger, archived).join("\n"), /missing its Complete, Archived evidence row/);
});

test("rejects a changed mdBook source directory", async () => {
  const ledger = await migrationLedger();
  const corpus = virtualCorpus(ledger);
  corpus.textFiles.set("docs/book.toml", '[book]\nsrc = "project"\n\n[other]\nsrc = "src"\n');
  assert.match(validateCorpusSnapshot(ledger, corpus).join("\n"), /docs\/book\.toml.*src.*src/);
});

test("requires lifecycle discovery and the archived Ultimate replacement link", async () => {
  const ledger = await migrationLedger();
  const corpus = virtualCorpus(ledger);
  corpus.textFiles.set("docs/README.md", "# Documentation Map\n");
  corpus.textFiles.set("docs/archive/website/ultimate-resource-meter.md", "# Historical announcement\n");
  const errors = validateCorpusSnapshot(ledger, corpus).join("\n");
  assert.match(errors, /archived Ultimate article must link its canonical replacement/);
  assert.match(errors, /docs\/README\.md must discover project\/migration-ledger\.md/);
  assert.match(errors, /docs\/README\.md must discover archive\/website\/ultimate-resource-meter\.md/);

  const disguised = virtualCorpus(ledger);
  disguised.textFiles.set(
    "docs/README.md",
    "# Documentation Map\n\n`project/migration-ledger.md`\n<!-- archive/website/ultimate-resource-meter.md -->\n",
  );
  disguised.textFiles.set(
    "docs/archive/website/ultimate-resource-meter.md",
    "# Historical announcement\n\n```text\n../../src/features/ultimate-resource.md\n```\n",
  );
  const disguisedErrors = validateCorpusSnapshot(ledger, disguised).join("\n");
  assert.match(disguisedErrors, /archived Ultimate article must link its canonical replacement/);
  assert.match(disguisedErrors, /docs\/README\.md must discover project\/migration-ledger\.md/);
});

test("rejects project or archive content in the generated search index", async (t) => {
  const f = await fixture();
  t.after(() => rm(f.root, { recursive: true, force: true }));
  await writeFile(
    path.join(f.output, "searchindex-test.js"),
    'Object.assign(window.search, {doc_urls:["guide/index.html"], body:"Migration Ledger"});\n',
  );
  assert.match((await validateGeneratedSite(f.output)).join("\n"), /non-published content.*Migration Ledger/);

  await writeFile(path.join(f.output, "searchindex-aaa.js"), "const benign = true;\n");
  await writeFile(path.join(f.output, "searchindex-test.js"), 'const leaked = "Current Build Plans";\n');
  assert.match((await validateGeneratedSite(f.output)).join("\n"), /(?:multiple generated search indexes|non-published content.*Current Build Plans)/);

  await rm(path.join(f.output, "searchindex-aaa.js"));
  await writeFile(
    path.join(f.output, "searchindex-test.js"),
    'Object.assign(window.search, {doc_urls:["docs/project/private.html"]});\n',
  );
  assert.match((await validateGeneratedSite(f.output)).join("\n"), /publishes a project or archive document URL/);

  await writeFile(
    path.join(f.output, "searchindex-test.js"),
    'Object.assign(window.search, {doc_urls:["guide/index.html"], body:"https://example.com/docs/project/releasing.md"});\n',
  );
  assert.doesNotMatch((await validateGeneratedSite(f.output)).join("\n"), /project or archive document URL/);
});

test("applies text hygiene to non-doc files and rejects forbidden dashes", () => {
  assert.match(validateTextHygiene("specs/058/example.md", Buffer.from("bad\r\n", "utf8")).join("\n"), /LF line endings/);
  assert.match(validateTextHygiene("specs/058/example.md", Buffer.from(`bad ${String.fromCodePoint(0x2014)} dash\n`, "utf8")).join("\n"), /forbidden dash/);
  assert.match(validateTextHygiene("specs/058/example.md", Buffer.from([0xef, 0xbb, 0xbf, 0x61])).join("\n"), /UTF-8 BOM/);
  assert.match(validateTextHygiene("specs/058/example.md", Buffer.from([0xff])).join("\n"), /not valid UTF-8/);
  assert.match(
    validateTextHygiene("specs/058/example.md", Buffer.from(`bad ${String.fromCodePoint(0x00c3)}x text\n`, "utf8")).join("\n"),
    /mojibake/,
  );
});

test("rejects a root README over 120 lines", async () => {
  const ledger = await migrationLedger();
  const corpus = virtualCorpus(ledger);
  corpus.textFiles.set("README.md", Array.from({ length: 121 }, (_, index) => `line ${index + 1}`).join("\n"));
  assert.match(validateCorpusSnapshot(ledger, corpus).join("\n"), /README.md exceeds 120 lines/);
});

test("requires package-safe absolute links in the root README", async () => {
  const ledger = await migrationLedger();
  const corpus = virtualCorpus(ledger);
  corpus.textFiles.set("README.md", "# ESO Weave\n\n[Installation](docs/src/getting-started/installation.md)\n");
  assert.match(validateCorpusSnapshot(ledger, corpus).join("\n"), /package copy requires an absolute link/);
});

test("S059 accepts the exact evidence-backed completeness manifest", async () => {
  const manifest = await contentCoverage();
  assert.deepEqual(validateContentCoverage(manifest, virtualCoverage(manifest)), []);
});

test("S059 freezes every semantic obligation key", async () => {
  const manifest = await contentCoverage();
  for (const [field, value] of [
    ["id", "LOG-999"],
    ["area", "Substituted area"],
    ["audience", ["Maintainer"]],
    ["statement", "A substituted statement that erases the original contract."],
    ["destination", "docs/src/README.md"],
    ["source_evidence", ["S-UI"]],
    ["test_evidence", ["T-UI"]],
    ["coverage", "Deferred"],
    ["labels", ["Diagnostic"]],
    ["content_anchors", ["Choose a path"]],
    ["follow_up", { issue: 92, url: "https://github.com/h8rt3rmin8r/eso-weave/issues/92", disposition: "Substituted disposition for the semantic freeze fixture." }],
  ]) {
    const changed = structuredClone(manifest);
    changed.obligations[0][field] = value;
    assert.match(validateContentCoverage(changed, virtualCoverage(changed)).join("\n"), /semantic projection/, field);
  }
  const changedEvidence = structuredClone(manifest);
  changedEvidence.evidence[manifest.obligations[0].source_evidence[0]].claim = "A substituted evidence claim.";
  assert.match(validateContentCoverage(changedEvidence, virtualCoverage(changedEvidence)).join("\n"), /semantic projection/, "evidence");
});

test("S059 rejects missing, duplicate, partial, and non-published obligations", async () => {
  const manifest = await contentCoverage();
  const missing = structuredClone(manifest);
  missing.obligations.pop();
  assert.match(validateContentCoverage(missing, virtualCoverage(missing)).join("\n"), /exact obligation identifiers/);

  const duplicate = structuredClone(manifest);
  duplicate.obligations.push(structuredClone(duplicate.obligations[0]));
  assert.match(validateContentCoverage(duplicate, virtualCoverage(duplicate)).join("\n"), /duplicate obligation/);

  const partial = structuredClone(manifest);
  partial.obligations[0].coverage = "Partial";
  assert.match(validateContentCoverage(partial, virtualCoverage(partial)).join("\n"), /may not remain Partial/);

  const unsafe = structuredClone(manifest);
  unsafe.obligations[0].destination = "docs/project/private.md";
  assert.match(validateContentCoverage(unsafe, virtualCoverage(unsafe)).join("\n"), /published docs\/src/);
});

test("S059 rejects unshaped, missing, or unresolvable evidence", async () => {
  const manifest = await contentCoverage();
  const unshaped = structuredClone(manifest);
  unshaped.obligations[0].source_evidence = [{ path: "src/input/mod.rs" }];
  assert.match(validateContentCoverage(unshaped, virtualCoverage(unshaped)).join("\n"), /invalid source evidence/);

  const absent = structuredClone(manifest);
  const evidence = absent.evidence[absent.obligations[0].source_evidence[0]];
  const snapshot = virtualCoverage(absent);
  snapshot.textFiles.set(evidence.path, "source without the required stable symbol\n");
  assert.match(validateContentCoverage(absent, snapshot).join("\n"), /evidence anchor is missing/);

  const noTests = structuredClone(manifest);
  noTests.obligations.find((row) => row.id === "LOG-001").test_evidence = [];
  assert.match(validateContentCoverage(noTests, virtualCoverage(noTests)).join("\n"), /test evidence or a TestGap/);
});

test("S059 rejects escaping, absolute, and case-drifted evidence paths", async () => {
  const manifest = await contentCoverage();
  for (const badPath of ["../eso-weave/src/input/mod.rs", path.resolve(repositoryRoot, "src", "input", "mod.rs"), "src/Input/mod.rs"]) {
    const changed = structuredClone(manifest);
    changed.evidence["S-INPUT"].path = badPath;
    const errors = await validateContentCoverageRepository(repositoryRoot, changed);
    assert.match(errors.join("\n"), /evidence path.*(?:repository-relative|exact case)/, badPath);
  }
});

test("S059 maps every platform claim dimension to shaped source evidence", async () => {
  const manifest = await contentCoverage();
  const platform = manifest.obligations.find((row) => row.id === "PLT-001");
  assert.deepEqual(platform.source_evidence, [
    "S-GAME-LINUX", "S-GAME-WINDOWS",
    "S-INPUT-LINUX", "S-INPUT-WINDOWS", "S-PERMISSION-LINUX",
    "S-CAPTURE-LINUX", "S-CAPTURE-WINDOWS",
    "S-PLATFORM-LINUX", "S-PLATFORM-WINDOWS",
  ]);
  for (const reference of platform.source_evidence) {
    assert.equal(manifest.evidence[reference].kind, "Source", reference);
    assert.ok(manifest.evidence[reference].anchor.length >= 12, reference);
  }
  const incomplete = structuredClone(manifest);
  incomplete.obligations.find((row) => row.id === "PLT-001").source_evidence.pop();
  assert.match(validateContentCoverage(incomplete, virtualCoverage(incomplete)).join("\n"), /semantic projection/);
});

test("S059 rejects missing substantive anchors and incomplete page dimensions", async () => {
  const manifest = await contentCoverage();
  const row = manifest.obligations.find((item) => item.coverage === "Covered");
  const missingAnchor = virtualCoverage(manifest);
  missingAnchor.textFiles.set(row.destination, missingAnchor.textFiles.get(row.destination).replace(row.content_anchors[0], ""));
  assert.match(validateContentCoverage(manifest, missingAnchor).join("\n"), /substantive anchor is missing/);

  const incomplete = structuredClone(manifest);
  incomplete.pages[0].required_anchors.pop();
  assert.match(validateContentCoverage(incomplete, virtualCoverage(incomplete)).join("\n"), /exact page profile/);

  const omittedPage = structuredClone(manifest);
  omittedPage.pages.pop();
  assert.match(validateContentCoverage(omittedPage, virtualCoverage(omittedPage)).join("\n"), /exact page profiles/);

  const omittedSearch = structuredClone(manifest);
  omittedSearch.search_map.pop();
  assert.match(validateContentCoverage(omittedSearch, virtualCoverage(omittedSearch)).join("\n"), /every contracted canonical term/);
});

test("S062 has no unresolved deferred documentation obligations", async () => {
  const manifest = await contentCoverage();
  assert.equal(manifest.obligations.filter((row) => row.coverage === "Deferred").length, 0);
  const broken = structuredClone(manifest);
  const deferred = broken.obligations.find((row) => row.id === "DEF-005");
  deferred.coverage = "Deferred";
  assert.match(validateContentCoverage(broken, virtualCoverage(broken)).join("\n"), /Deferred.*follow-up/);
});

test("S062 rejects obsolete settings runtime claims", () => {
  assert.deepEqual(
    validateSettingsRuntimeClaims(
      "Fishing controls and scalar reader fields apply live. Block Size remains staged.",
    ),
    [],
  );
  for (const claim of [
    "All settings apply immediately.",
    "Any change to the draft is applied live.",
    "Live reader fields require restart.",
    "Some Fishing and Pixel Bus settings currently require an application restart.",
    "Color Tolerance and sampling intervals are saved for the next application start.",
    "Fishing and Pixel Bus changes are saved but are not propagated to their running components, so restart ESO Weave after changing them.",
    "Fishing Interact Key is not exposed.",
    "The current modal has no editor for the stored interact key.",
    "The current modal does not expose Fishing's stored interact key.",
    "The Settings modal does not currently expose that Interact Key.",
    "The modal exposes Arm Timeout, Reel Delay, and Recast Delay, but no interact-key control.",
    "There is no supported in-app editor for it.",
    "The modal does not expose the Fishing Interact Key.",
  ]) {
    assert.match(validateSettingsRuntimeClaims(claim).join("\n"), /obsolete/u, claim);
  }
});

test("S059 search aliases must be visible on the canonical published target", async () => {
  const manifest = await contentCoverage();
  const snapshot = virtualCoverage(manifest);
  const entry = manifest.search_map[0];
  const target = snapshot.textFiles.get(entry.target);
  const masked = entry.aliases.map((alias) => `\`${alias}\``).join("\n");
  snapshot.textFiles.set(entry.target, target.replace(entry.aliases.join("\n"), masked));
  assert.match(validateContentCoverage(manifest, snapshot).join("\n"), /required search alias/);

  const absentCanonical = structuredClone(manifest);
  absentCanonical.search_map[0].target = "docs/project/search.md";
  assert.match(validateContentCoverage(absentCanonical, virtualCoverage(absentCanonical)).join("\n"), /search target must be published/);
});

test("S059 diagrams require useful non-color text equivalents", async () => {
  const manifest = await contentCoverage();
  const broken = structuredClone(manifest);
  broken.diagrams[0].alt_text = "diagram";
  broken.diagrams[0].text_equivalent = "See colors.";
  assert.match(validateContentCoverage(broken, virtualCoverage(broken)).join("\n"), /diagram.*text equivalent/);

  const missing = structuredClone(manifest);
  const snapshot = virtualCoverage(manifest);
  missing.diagrams[0].destination = "docs/src/missing.md";
  assert.match(validateContentCoverage(missing, snapshot).join("\n"), /diagram destination.*(?:exist|SUMMARY)/);

  const erased = virtualCoverage(manifest);
  const diagram = manifest.diagrams[0];
  erased.textFiles.set(diagram.destination, erased.textFiles.get(diagram.destination).replace(diagram.content_anchors[0], ""));
  assert.match(validateContentCoverage(manifest, erased).join("\n"), /diagram content anchor/);
});

test("S059 requires every frozen alias or a directly linked glossary explanation", async () => {
  const manifest = await contentCoverage();
  const changed = structuredClone(manifest);
  changed.search_map[1].aliases = [];
  assert.match(validateContentCoverage(changed, virtualCoverage(changed)).join("\n"), /semantic projection/);

  const snapshot = virtualCoverage(manifest);
  const entry = manifest.search_map[0];
  snapshot.textFiles.set(entry.target, snapshot.textFiles.get(entry.target).replaceAll(entry.aliases[0], ""));
  assert.match(validateContentCoverage(manifest, snapshot).join("\n"), /required search alias/);

  snapshot.textFiles.set(
    "docs/src/reference/glossary.md",
    `# Glossary\n\n- **\`${entry.aliases[0]}\`:** See [${entry.canonical}](../README.md).\n`,
  );
  assert.doesNotMatch(validateContentCoverage(manifest, snapshot).join("\n"), /required search alias/);

  snapshot.textFiles.set(
    "docs/src/reference/glossary.md",
    `# Glossary\n\n\`\`\`text\n${entry.aliases[0]} [${entry.canonical}](../README.md)\n\`\`\`\n`,
  );
  assert.match(validateContentCoverage(manifest, snapshot).join("\n"), /required search alias/);
});

test("accepts the complete ESO catalog source contract", () => {
  assert.deepEqual(validateCatalogSourceContract(validCatalogContract()), []);
});

test("rejects missing catalog categories and transient durable keys", () => {
  const contract = validCatalogContract();
  contract.categories = contract.categories.filter((row) => row.id !== "items-and-gear");
  contract.categories[0].stable_key = "luaindex";
  const errors = validateCatalogSourceContract(contract);
  assert.ok(errors.some((error) => error.includes("missing required category items-and-gear")));
  assert.ok(errors.some((error) => error.includes("transient stable_key")));
});

test("rejects unsupported exhaustive claims and unresolved source references", () => {
  const contract = validCatalogContract();
  contract.categories[0].completeness = "exhaustive";
  contract.categories[0].field_verification = "issue #129";
  contract.categories[1].source_ids = ["missing-source"];
  const errors = validateCatalogSourceContract(contract);
  assert.ok(errors.some((error) => error.includes("cannot be exhaustive while field verification is pending")));
  assert.ok(errors.some((error) => error.includes("unknown source missing-source")));
});

test("requires reproducible provenance for every declared source snapshot", () => {
  const contract = validCatalogContract();
  contract.source_snapshots.push({ id: "community-reference", channel: "not-applicable", recommended: false });
  contract.categories[0].source_ids = ["community-reference"];
  const errors = validateCatalogSourceContract(contract);
  for (const field of ["family", "locale", "revision", "uri", "acquired_at", "license_scope"]) {
    assert.ok(errors.some((error) => error.includes(`requires ${field}`)), field);
  }
});

test("requires exhaustive categories to use enumerable evidence with relationship or count checks", () => {
  const contract = validCatalogContract();
  const category = contract.categories[0];
  category.completeness = "exhaustive";
  category.field_verification = "none";
  category.enumeration_method = "known-id";
  let errors = validateCatalogSourceContract(contract);
  assert.ok(errors.some((error) => error.includes("requires an enumerable method")));

  category.enumeration_method = "api-iterator";
  errors = validateCatalogSourceContract(contract);
  assert.ok(errors.some((error) => error.includes("requires count or relationship validation")));

  category.validation.push("enumerated count matches the declared source relationship");
  assert.deepEqual(validateCatalogSourceContract(contract), []);
});

test("rejects position-based durable keys regardless of separator", () => {
  for (const stableKey of ["iterator_position", "array-position", "mutable_position"]) {
    const contract = validCatalogContract();
    contract.categories[0].stable_key = stableKey;
    assert.ok(
      validateCatalogSourceContract(contract).some((error) => error.includes("transient stable_key")),
      `${stableKey} must be rejected`,
    );
  }
});

test("rejects implicit PTS promotion and distributable game icon bytes", () => {
  const contract = validCatalogContract();
  contract.promotion_policy.automatic = true;
  contract.promotion_policy.preserve_original_channel = false;
  contract.categories.find((row) => row.id === "icon-bytes").redistribution = "allowed";
  contract.project_facts.placeholders_allowed = false;
  const errors = validateCatalogSourceContract(contract);
  assert.ok(errors.includes("catalog contract must forbid automatic PTS promotion"));
  assert.ok(errors.includes("catalog promotion policy must preserve original channel provenance"));
  assert.ok(errors.includes("catalog contract must prohibit redistribution of game icon bytes"));
  assert.ok(errors.includes("catalog contract must retain project-created placeholders"));
});

test("keeps authoritative icon redistribution decisions prohibited", () => {
  for (const decision of ["zenimax_icon_bytes", "prebuilt_extracted_icon_pack"]) {
    const contract = validCatalogContract();
    contract.redistribution_decisions[decision] = "allowed";
    assert.ok(
      validateCatalogSourceContract(contract).some((error) => error.includes(`must prohibit ${decision}`)),
      `${decision} must remain prohibited`,
    );
  }
});

test("rejects executable, unbounded, non-atomic, or uploaded collector input", () => {
  const contract = validCatalogContract();
  contract.collector_policy.execute_lua = true;
  contract.collector_policy.byte_limit_required = false;
  contract.collector_policy.record_limit_required = false;
  contract.collector_policy.atomic_import = false;
  contract.collector_policy.upload_default = true;
  contract.collector_policy.user_data_separate = false;
  contract.collector_policy.provisional_max_snapshot_bytes = 0;
  contract.collector_policy.provisional_max_records = -1;
  contract.collector_policy.provisional_max_string_bytes = 1.5;
  const errors = validateCatalogSourceContract(contract);
  for (const phrase of [
    "execute Lua", "byte limit", "record limit", "atomic", "uploaded by default", "user data separate",
    "positive integer provisional_max_snapshot_bytes", "positive integer provisional_max_records",
    "positive integer provisional_max_string_bytes",
  ]) {
    assert.ok(errors.some((error) => error.includes(phrase)), phrase);
  }
});

test("accepts the complete external encounter model contract", () => {
  assert.deepEqual(validateEncounterModelContract(validEncounterContract()), []);
});

test("rejects unsafe encounter transport, privacy, storage, and derivation policies", () => {
  const contract = validEncounterContract();
  contract.storage_planes.raw = "catalog.sqlite";
  contract.privacy_policy.upload_default = true;
  contract.integrity_policy.raw_immutable = false;
  contract.transport_policy.pixel_bus_bulk_transport = true;
  contract.transport_policy.automation_independent = false;
  const errors = validateEncounterModelContract(contract).join("\n");
  assert.match(errors, /raw observations.*separate/i);
  assert.match(errors, /uploaded by default/i);
  assert.match(errors, /raw observations must be immutable/i);
  assert.match(errors, /Pixel Bus/i);
  assert.match(errors, /automation/i);
});

test("requires every encounter family, metric traceability, and synthetic evidence boundary", () => {
  const contract = validEncounterContract();
  contract.event_kinds.pop();
  contract.metrics[0].quality_required = false;
  contract.synthetic_fixture.proves_live_parity = true;
  const errors = validateEncounterModelContract(contract).join("\n");
  assert.match(errors, /missing required event kind discontinuity/i);
  assert.match(errors, /metric observed-dps.*quality/i);
  assert.match(errors, /synthetic.*live parity/i);
});

test("requires a complete parity matrix and concrete ordered owners", () => {
  const contract = validEncounterContract();
  delete contract.parity_roadmap[0].privacy_impact;
  contract.follow_up_issues.calculation = "later";
  const errors = validateEncounterModelContract(contract).join("\n");
  assert.match(errors, /parity row encounter-start requires privacy_impact/i);
  assert.match(errors, /follow-up calculation requires an issue number/i);
});

test("requires complete, traceable metric receipts", () => {
  const fixture = validEncounterFixture();
  const projected = projectEncounterMetrics(fixture, new Set([100, 200, 300]));
  const expected = {
    schema_version: 1,
    algorithm_version: "s069-v1",
    raw_content_sha256: projected.raw_content_sha256,
    metrics: structuredClone(projected.metrics),
    loss_ranges: structuredClone(projected.metrics["observed-dps"].loss_ranges),
    catalog_receipts: [
      { ...projected.catalog_receipt, catalog_snapshot: "v1" },
      { ...projectEncounterMetrics(fixture, new Set([100, 200, 300, 999999])).catalog_receipt, catalog_snapshot: "v2" },
    ],
    storage: { production_retention_recommendation: "verification-required" },
    parity_claim: "synthetic-determinism-only",
  };
  delete expected.metrics["observed-dps"].value;
  delete expected.metrics["observed-dps"].first_sequence;
  delete expected.metrics["observed-hps"].unit;
  delete expected.metrics["effect-uptime"].quality;
  const errors = validateEncounterEvidence(fixture, expected).join("\n");
  assert.match(errors, /observed-dps.*value/i);
  assert.match(errors, /observed-dps.*first_sequence/i);
  assert.match(errors, /observed-hps.*unit/i);
  assert.match(errors, /effect-uptime.*quality/i);
});

test("requires complete catalog join receipts", () => {
  const fixture = validEncounterFixture();
  const initial = projectEncounterMetrics(fixture, new Set([100, 200, 300]));
  const resolved = projectEncounterMetrics(fixture, new Set([100, 200, 300, 999999]));
  const expected = {
    schema_version: 1,
    algorithm_version: "s069-v1",
    raw_content_sha256: initial.raw_content_sha256,
    metrics: structuredClone(initial.metrics),
    loss_ranges: structuredClone(initial.metrics["observed-dps"].loss_ranges),
    catalog_receipts: [
      { ...initial.catalog_receipt, catalog_snapshot: "v1" },
      { ...resolved.catalog_receipt, catalog_snapshot: "v2" },
    ],
    storage: { production_retention_recommendation: "verification-required" },
    parity_claim: "synthetic-determinism-only",
  };
  delete expected.catalog_receipts[0].known_ids;
  delete expected.catalog_receipts[1].catalog_snapshot;
  const errors = validateEncounterEvidence(fixture, expected).join("\n");
  assert.match(errors, /initial catalog receipt requires known_ids/i);
  assert.match(errors, /resolved catalog receipt requires catalog_snapshot/i);
});

test("projects deterministic metrics and catalog receipts from the dummy encounter", () => {
  const fixture = validEncounterFixture();
  assert.deepEqual(validateEncounterFixture(fixture), []);
  const projected = projectEncounterMetrics(fixture, new Set([100, 200, 300]));
  assert.equal(projected.metrics["observed-dps"].value, 300);
  assert.equal(projected.metrics["observed-hps"].value, 80);
  assert.equal(projected.metrics["ability-damage-share"].values["100"], 0.5);
  assert.equal(projected.metrics["ability-damage-share"].values["999999"], 0.5);
  assert.equal(projected.metrics["effect-uptime"].values["200"], 0.6);
  assert.deepEqual(projected.metrics["ordered-cast-sequence"].values, [100, 999999, 100]);
  assert.deepEqual(projected.catalog_receipt.unknown_ids, [999999]);
  assert.ok(Object.values(projected.metrics).every((metric) => metric.quality === "degraded"));

  const shuffled = structuredClone(fixture);
  shuffled.events.reverse();
  assert.deepEqual(projectEncounterMetrics(shuffled, new Set([100, 200, 300])), projected);
  const resolved = projectEncounterMetrics(fixture, new Set([100, 200, 300, 999999]));
  assert.deepEqual(resolved.catalog_receipt.unknown_ids, []);
  assert.equal(resolved.raw_content_sha256, projected.raw_content_sha256);
});

test("rejects duplicate sequences, undeclared gaps, and backward monotonic time", () => {
  const duplicate = validEncounterFixture();
  duplicate.events[1].sequence = 1;
  assert.match(validateEncounterFixture(duplicate).join("\n"), /duplicate sequence/i);

  const gap = validEncounterFixture();
  gap.events = gap.events.filter((event) => event.kind !== "discontinuity");
  assert.match(validateEncounterFixture(gap).join("\n"), /undeclared sequence gap/i);

  const backwards = validEncounterFixture();
  backwards.events.find((event) => event.sequence === 13).monotonic_ms = 5000;
  assert.match(validateEncounterFixture(backwards).join("\n"), /backward monotonic time/i);
});

test("rejects malformed discontinuities and private fixture fields", () => {
  const malformed = validEncounterFixture();
  for (const event of malformed.events.filter((event) => event.sequence >= 12)) event.sequence -= 2;
  malformed.envelope.last_sequence -= 2;
  const marker = malformed.events.find((event) => event.kind === "discontinuity");
  marker.payload.missing_sequence_from = 50;
  marker.payload.missing_sequence_to = 60;
  assert.match(validateEncounterFixture(malformed).join("\n"), /discontinuity.*preceding missing range/i);

  const privateFixture = validEncounterFixture();
  privateFixture.events[2].payload.subject = { account_name: "private" };
  assert.match(validateEncounterFixture(privateFixture).join("\n"), /prohibited private field account_name/i);

  const privateEnvelope = validEncounterFixture();
  privateEnvelope.envelope.metadata = { character_name: "private" };
  assert.match(validateEncounterFixture(privateEnvelope).join("\n"), /prohibited private field character_name/i);

  const missingReason = validEncounterFixture();
  missingReason.events.find((event) => event.kind === "discontinuity").payload.reason = "";
  assert.match(validateEncounterFixture(missingReason).join("\n"), /discontinuity.*non-empty reason/i);
});

test("unions overlapping effect intervals across actor instances", () => {
  const fixture = validEncounterFixture();
  for (const event of fixture.events.filter((event) => event.sequence >= 12)) event.sequence += 3;
  fixture.envelope.last_sequence += 3;
  const marker = fixture.events.find((event) => event.kind === "discontinuity");
  marker.payload.missing_sequence_from = 12;
  marker.payload.missing_sequence_to = 14;
  fixture.events.push(
    { session_id: "s1", encounter_id: "e1", sequence: 10, monotonic_ms: 5000, kind: "effect", payload: { target_actor_id: "a2", ability_id: 200, effect_instance_id: "other", change: "gained" } },
    { session_id: "s1", encounter_id: "e1", sequence: 11, monotonic_ms: 5500, kind: "effect", payload: { target_actor_id: "a2", ability_id: 200, effect_instance_id: "other", change: "faded" } },
  );
  assert.equal(projectEncounterMetrics(fixture).metrics["effect-uptime"].values["200"], 0.6);
});

test("preserves the earliest active time across repeated effect gains", () => {
  const fixture = validEncounterFixture();
  for (const event of fixture.events.filter((event) => event.sequence >= 5)) event.sequence += 1;
  fixture.envelope.last_sequence += 1;
  const marker = fixture.events.find((event) => event.kind === "discontinuity");
  marker.payload.missing_sequence_from += 1;
  marker.payload.missing_sequence_to += 1;
  fixture.events.push({
    session_id: "s1", encounter_id: "e1", sequence: 5, monotonic_ms: 2200, kind: "effect",
    payload: { target_actor_id: "a1", ability_id: 200, change: "gained" },
  });
  assert.equal(projectEncounterMetrics(fixture).metrics["effect-uptime"].values["200"], 0.6);
});
