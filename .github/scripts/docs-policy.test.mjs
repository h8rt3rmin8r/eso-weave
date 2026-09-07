import assert from "node:assert/strict";
import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import test from "node:test";

import {
  contrastRatio,
  validateBrandCss,
  validateBrandJavascript,
  validateCorpusSnapshot,
  validateGeneratedSite,
  validateMigrationLedger,
  validateSourceTree,
  validateTextHygiene,
  validateWorkflowText,
} from "./docs-policy.mjs";

const repositoryRoot = path.resolve(import.meta.dirname, "..", "..");

async function migrationLedger() {
  return JSON.parse(await readFile(path.join(repositoryRoot, "docs", "project", "migration-ledger.json"), "utf8"));
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
      "no menu",
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
  const active = virtualCorpus(ledger);
  assert.deepEqual(validateMigrationLedger(ledger, active), []);
  assert.deepEqual(validateCorpusSnapshot(ledger, active), []);

  const transitioned = structuredClone(ledger);
  transitioned.postBaselinePlans[0] = {
    ...transitioned.postBaselinePlans[0],
    completion: "Complete",
    lifecycle: "Archived",
    destination: "docs/archive/build-plans/plan-028.md",
    evidence: "Merged in PR #99.",
  };
  transitioned.postBaselinePlans.push({
    id: "029",
    completion: "In Progress",
    lifecycle: "Active",
    destination: "docs/project/build-plans/plan-029.md",
    specs: ["specs/059-next-slice"],
    evidence: "Issue #101 tracks the active delivery.",
  });
  const archivedAndNext = virtualCorpus(transitioned);
  assert.deepEqual(validateMigrationLedger(transitioned, archivedAndNext), []);
  assert.deepEqual(validateCorpusSnapshot(transitioned, archivedAndNext), []);

  const erasedArchive = structuredClone(archivedAndNext);
  erasedArchive.textFiles = new Map(archivedAndNext.textFiles);
  erasedArchive.textFiles.set("docs/archive/build-plans/plan-028.md", "# Plan 028\n");
  assert.match(validateMigrationLedger(transitioned, erasedArchive).join("\n"), /invalid Archived lifecycle/);

  const ambiguous = structuredClone(archivedAndNext);
  ambiguous.currentPaths = new Set(archivedAndNext.currentPaths);
  ambiguous.existingPaths = new Set(archivedAndNext.existingPaths);
  ambiguous.currentPaths.add("docs/project/build-plans/plan-028.md");
  ambiguous.existingPaths.add("docs/project/build-plans/plan-028.md");
  assert.match(
    [...validateMigrationLedger(transitioned, ambiguous), ...validateCorpusSnapshot(transitioned, ambiguous)].join("\n"),
    /(?:invalid Archived lifecycle|active project build plan must match)/,
  );

  const inverted = structuredClone(transitioned);
  inverted.postBaselinePlans[0] = {
    ...inverted.postBaselinePlans[0],
    completion: "In Progress",
    lifecycle: "Active",
    destination: "docs/project/build-plans/plan-028.md",
    evidence: "Issue #80 tracks active delivery.",
  };
  inverted.postBaselinePlans[1] = {
    ...inverted.postBaselinePlans[1],
    completion: "Complete",
    lifecycle: "Archived",
    destination: "docs/archive/build-plans/plan-029.md",
    evidence: "Merged in PR #100.",
  };
  assert.match(
    validateMigrationLedger(inverted, virtualCorpus(inverted)).join("\n"),
    /Active post-baseline plan must be the final entry/,
  );
});

test("does not accept plan index rows hidden in code or comments", async () => {
  const ledger = await migrationLedger();
  const active = virtualCorpus(ledger);
  active.textFiles.set(
    "docs/project/build-plans/README.md",
    "# Current Build Plans\n\n```markdown\n| [plan-028.md](plan-028.md) | Active | Hidden |\n```\n",
  );
  assert.match(validateCorpusSnapshot(ledger, active).join("\n"), /missing its Active index row/);

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
