import assert from "node:assert/strict";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import test from "node:test";

import {
  contrastRatio,
  validateBrandCss,
  validateBrandJavascript,
  validateGeneratedSite,
  validateSourceTree,
  validateWorkflowText,
} from "./docs-policy.mjs";

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
