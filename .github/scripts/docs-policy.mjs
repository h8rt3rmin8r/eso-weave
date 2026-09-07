import { readdir, readFile, stat } from "node:fs/promises";
import path from "node:path";
import { pathToFileURL } from "node:url";

const HEADING = /^#{1,6}\s+(.+?)\s*#*\s*$/gmu;
const HTML_LINK = /<a\b[^>]*?\bhref=["']([^"']+)["']/giu;

async function walk(root, suffix = "") {
  const results = [];
  for (const entry of await readdir(root, { withFileTypes: true })) {
    const absolute = path.join(root, entry.name);
    if (entry.isDirectory()) {
      results.push(...(await walk(absolute, suffix)));
    } else if (!suffix || entry.name.endsWith(suffix)) {
      results.push(absolute);
    }
  }
  return results;
}

function slash(value) {
  return value.split(path.sep).join("/");
}

function splitTarget(raw) {
  const cleaned = raw.trim().replace(/^<|>$/gu, "");
  const hash = cleaned.indexOf("#");
  return {
    pathname: decodeURIComponent(hash >= 0 ? cleaned.slice(0, hash) : cleaned),
    fragment: decodeURIComponent(hash >= 0 ? cleaned.slice(hash + 1) : ""),
  };
}

function markdownLinks(markdown) {
  markdown = maskMarkdownCode(markdown);
  const links = [];
  for (let cursor = 0; cursor < markdown.length; cursor += 1) {
    const image = markdown[cursor] === "!" && markdown[cursor + 1] === "[";
    const labelStart = image ? cursor + 1 : cursor;
    if (markdown[labelStart] !== "[") continue;
    let labelEnd = labelStart + 1;
    for (; labelEnd < markdown.length; labelEnd += 1) {
      if (markdown[labelEnd] === "\\") labelEnd += 1;
      else if (markdown[labelEnd] === "]") break;
    }
    if (markdown[labelEnd] !== "]" || markdown[labelEnd + 1] !== "(") continue;
    let depth = 0;
    let quote = "";
    let destinationEnd = labelEnd + 2;
    for (; destinationEnd < markdown.length; destinationEnd += 1) {
      const character = markdown[destinationEnd];
      if (character === "\\") {
        destinationEnd += 1;
        continue;
      }
      if (quote) {
        if (character === quote) quote = "";
        continue;
      }
      if ((character === '"' || character === "'") && /\s/u.test(markdown[destinationEnd - 1] ?? "")) {
        quote = character;
        continue;
      }
      if (character === "(") depth += 1;
      else if (character === ")" && depth > 0) depth -= 1;
      else if (character === ")") break;
    }
    if (markdown[destinationEnd] !== ")") continue;
    const inside = markdown.slice(labelEnd + 2, destinationEnd).trim();
    const destination = markdownDestination(inside);
    if (destination) links.push({ destination, image });
    cursor = destinationEnd;
  }
  return links;
}

function maskMarkdownCode(markdown) {
  const characters = markdown.split("");
  const lines = markdown.match(/.*(?:\r?\n|$)/gu) ?? [];
  let offset = 0;
  let fence = null;

  for (const line of lines) {
    const opening = line.match(/^ {0,3}(`{3,}|~{3,})/u);
    const closing = fence
      ? line.match(new RegExp(`^ {0,3}${fence.character}{${fence.length},}\\s*$`, "u"))
      : null;
    if (fence || opening) {
      for (let index = offset; index < offset + line.length; index += 1) {
        if (characters[index] !== "\r" && characters[index] !== "\n") characters[index] = " ";
      }
      if (closing) fence = null;
      else if (!fence) fence = { character: opening[1][0], length: opening[1].length };
    }
    offset += line.length;
  }

  for (let cursor = 0; cursor < characters.length; cursor += 1) {
    if (characters[cursor] !== "`") continue;
    let runLength = 1;
    while (characters[cursor + runLength] === "`") runLength += 1;
    const delimiter = "`".repeat(runLength);
    const remainder = characters.slice(cursor + runLength).join("");
    const closingOffset = remainder.indexOf(delimiter);
    if (closingOffset < 0) {
      cursor += runLength - 1;
      continue;
    }
    const end = cursor + runLength + closingOffset + runLength;
    for (let index = cursor; index < end; index += 1) {
      if (characters[index] !== "\r" && characters[index] !== "\n") characters[index] = " ";
    }
    cursor = end - 1;
  }
  return characters.join("");
}

function markdownDestination(inside) {
  if (inside.startsWith("<")) {
    const end = inside.indexOf(">");
    return end > 0 ? inside.slice(1, end) : "";
  }
  let depth = 0;
  for (let index = 0; index < inside.length; index += 1) {
    const character = inside[index];
    if (character === "\\") {
      index += 1;
      continue;
    }
    if (character === "(") depth += 1;
    else if (character === ")" && depth > 0) depth -= 1;
    else if (/\s/u.test(character) && depth === 0) return inside.slice(0, index).replace(/\\([!"#$%&'()*+,\-./:;<=>?@[\]^_`{|}~])/gu, "$1");
  }
  return inside.replace(/\\([!"#$%&'()*+,\-./:;<=>?@[\]^_`{|}~])/gu, "$1");
}

function isExternal(value) {
  return /^(?:[a-z][a-z0-9+.-]*:|\/\/)/iu.test(value);
}

async function exactCase(root, relativePath) {
  const segments = slash(relativePath).split("/").filter(Boolean);
  let current = root;
  for (const segment of segments) {
    let names;
    try {
      names = await readdir(current);
    } catch {
      return { exists: false, exact: false };
    }
    const exact = names.includes(segment);
    const folded = names.find((name) => name.toLocaleLowerCase("en-US") === segment.toLocaleLowerCase("en-US"));
    if (folded === undefined) {
      return { exists: false, exact: false };
    }
    if (!exact) {
      return { exists: true, exact: false };
    }
    current = path.join(current, segment);
  }
  return { exists: true, exact: true };
}

function headingIds(markdown) {
  const ids = new Set();
  const counts = new Map();
  for (const match of markdown.matchAll(HEADING)) {
    const base = match[1]
      .trim()
      .toLocaleLowerCase("en-US")
      .replace(/<[^>]*>/gu, "")
      .replace(/[^\p{Letter}\p{Number}\s_-]/gu, "")
      .replace(/\s+/gu, "-");
    const count = counts.get(base) ?? 0;
    ids.add(count === 0 ? base : `${base}-${count}`);
    counts.set(base, count + 1);
  }
  return ids;
}

async function validateMarkdownLinks(sourceRoot, file, contents) {
  const errors = [];
  for (const link of markdownLinks(contents)) {
    const raw = link.destination.trim();
    const image = link.image;
    if (isExternal(raw)) {
      if (image) {
        errors.push(`${slash(path.relative(sourceRoot, file))}: remote runtime resource ${raw}`);
      }
      continue;
    }
    const { pathname, fragment } = splitTarget(raw);
    if (pathname.startsWith("/")) {
      errors.push(`${slash(path.relative(sourceRoot, file))}: root-relative source link ${raw}`);
      continue;
    }
    if (path.basename(file) !== "SUMMARY.md" && /README\.md$/u.test(pathname)) {
      errors.push(`${slash(path.relative(sourceRoot, file))}: use the directory URL instead of inline README.md link ${raw}`);
      continue;
    }
    let target = pathname ? path.resolve(path.dirname(file), pathname) : file;
    if (pathname.endsWith("/")) target = path.join(target, "README.md");
    const relative = path.relative(sourceRoot, target);
    if (relative.startsWith("..") || path.isAbsolute(relative)) {
      errors.push(`${slash(path.relative(sourceRoot, file))}: local target escapes docs/src: ${raw}`);
      continue;
    }
    const caseResult = await exactCase(sourceRoot, relative);
    if (!caseResult.exists) {
      errors.push(`${slash(path.relative(sourceRoot, file))}: missing local target ${raw}`);
      continue;
    }
    if (!caseResult.exact) {
      errors.push(`${slash(path.relative(sourceRoot, file))}: target case does not match ${raw}`);
      continue;
    }
    if (fragment && target.endsWith(".md")) {
      const targetText = await readFile(target, "utf8");
      if (!headingIds(targetText).has(fragment)) {
        errors.push(`${slash(path.relative(sourceRoot, file))}: missing fragment ${raw}`);
      }
    }
  }
  return errors;
}

export async function validateSourceTree(docsRoot) {
  const sourceRoot = path.join(docsRoot, "src");
  const summaryPath = path.join(sourceRoot, "SUMMARY.md");
  const errors = [];
  let summary;
  try {
    summary = await readFile(summaryPath, "utf8");
  } catch {
    return ["docs/src/SUMMARY.md is missing"];
  }

  const listed = [];
  for (const line of summary.split(/\r?\n/gu)) {
    if (!/^\s*[-*+]\s+/u.test(line)) continue;
    for (const link of markdownLinks(line)) {
      const { pathname } = splitTarget(link.destination);
      if (!pathname.endsWith(".md")) continue;
      const resolved = path.resolve(sourceRoot, pathname);
      const relative = slash(path.relative(sourceRoot, resolved));
      if (relative.startsWith("../") || path.isAbsolute(relative)) {
        errors.push(`SUMMARY.md path escapes docs/src: ${pathname}`);
        continue;
      }
      listed.push(relative);
      const caseResult = await exactCase(sourceRoot, relative);
      if (caseResult.exists && !caseResult.exact) {
        errors.push(`SUMMARY.md path case does not match: ${pathname}`);
      } else if (!caseResult.exists) {
        errors.push(`SUMMARY.md references missing chapter: ${pathname}`);
      }
    }
  }

  const counts = new Map();
  for (const page of listed) counts.set(page, (counts.get(page) ?? 0) + 1);
  for (const [page, count] of counts) {
    if (count > 1) errors.push(`${page} is listed more than once in SUMMARY.md`);
  }

  const markdownFiles = await walk(sourceRoot, ".md");
  for (const file of markdownFiles) {
    const relative = slash(path.relative(sourceRoot, file));
    if (!["SUMMARY.md", "404.md"].includes(relative) && !counts.has(relative)) {
      errors.push(`${relative} is not listed in SUMMARY.md`);
    }
    const contents = await readFile(file, "utf8");
    if (relative !== "SUMMARY.md" && !/^#\s+\S+/mu.test(contents)) {
      errors.push(`${relative} has no non-empty level-one heading`);
    }
    errors.push(...(await validateMarkdownLinks(sourceRoot, file, contents)));
  }
  return errors;
}

function localOutputPath(outputRoot, htmlFile, raw, siteUrl) {
  const { pathname } = splitTarget(raw);
  if (!pathname) return htmlFile;
  let candidate;
  if (pathname.startsWith("/")) {
    if (!pathname.startsWith(siteUrl)) return null;
    const relative = pathname.slice(siteUrl.length);
    candidate = path.join(outputRoot, relative);
  } else {
    candidate = path.resolve(path.dirname(htmlFile), pathname);
  }
  if (pathname.endsWith("/")) candidate = path.join(candidate, "index.html");
  const relative = path.relative(outputRoot, candidate);
  return relative.startsWith("..") || path.isAbsolute(relative) ? null : candidate;
}

async function exists(file) {
  try {
    return (await stat(file)).isFile();
  } catch {
    return false;
  }
}

export async function validateGeneratedSite(outputRoot, siteUrl = "/eso-weave/") {
  const errors = [];
  for (const required of ["index.html", "404.html"]) {
    if (!(await exists(path.join(outputRoot, required)))) errors.push(`missing generated output ${required}`);
  }

  const outputFiles = (await walk(outputRoot)).map((file) => slash(path.relative(outputRoot, file)));
  if (!outputFiles.some((file) => /^searcher-[0-9a-z]+\.js$/u.test(file))) errors.push("missing generated searcher JavaScript");
  if (!outputFiles.some((file) => /^searchindex-[0-9a-z]+\.js$/u.test(file))) errors.push("missing generated search index");
  const brandScript = outputFiles.find((file) => /^theme\/eso-weave-[0-9a-z]+\.js$/u.test(file));
  if (!brandScript) {
    errors.push("missing generated ESO Weave behavior JavaScript");
  } else {
    errors.push(...validateBrandJavascript(await readFile(path.join(outputRoot, brandScript), "utf8")));
  }

  const htmlFiles = await walk(outputRoot, ".html");
  const htmlContents = new Map();
  const htmlIds = new Map();
  for (const file of htmlFiles) {
    const contents = await readFile(file, "utf8");
    htmlContents.set(file, contents);
    const ids = new Set();
    for (const match of contents.matchAll(/\bid=["']([^"']+)["']/giu)) {
      if (ids.has(match[1])) errors.push(`${slash(path.relative(outputRoot, file))}: duplicate id ${match[1]}`);
      ids.add(match[1]);
    }
    htmlIds.set(file, ids);
  }
  for (const file of htmlFiles) {
    const contents = htmlContents.get(file);
    const relative = slash(path.relative(outputRoot, file));
    if (!/<html\b[^>]*\blang=["']en["']/iu.test(contents)) errors.push(`${relative}: missing lang=en`);
    if (relative !== "toc.html" && !/<h1\b[^>]*>\s*(?:<[^>]+>)*\s*\S/iu.test(contents)) errors.push(`${relative}: missing non-empty h1`);
    if (/rel=["']edit["']/iu.test(contents) && contents.includes("/docs/src/src/")) {
      errors.push(`${relative}: edit link duplicates the docs/src path`);
    }
    const escapedSiteUrl = siteUrl.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
    if (relative === "404.html" && !new RegExp(`\\bhref\\s*=\\s*["']${escapedSiteUrl}["']`, "iu").test(contents)) {
      errors.push(`404.html: missing site-root recovery link ${siteUrl}`);
    }
    for (const match of contents.matchAll(/<img\b[^>]*>/giu)) {
      if (!/\balt=["'][^"']*["']/iu.test(match[0])) errors.push(`${relative}: image missing alt`);
    }
    for (const raw of runtimeResourceValues(contents)) {
      if (/^data:/iu.test(raw)) continue;
      if (isExternal(raw) || /^javascript:/iu.test(raw)) {
        errors.push(`${relative}: remote runtime resource ${raw}`);
        continue;
      }
      const target = localOutputPath(outputRoot, file, raw, siteUrl);
      if (target === null) errors.push(`${relative}: resource escapes site base ${raw}`);
      else if (!(await exists(target))) errors.push(`${relative}: missing generated resource ${raw}`);
    }
    const embeddedCss = [
      ...[...contents.matchAll(/<style\b[^>]*>([\s\S]*?)<\/style\s*>/giu)].map((match) => match[1]),
      ...[...contents.matchAll(/\bstyle\s*=\s*(?:"([^"]*)"|'([^']*)'|([^\s"'=<>`]+))/giu)].map(
        (match) => match[1] ?? match[2] ?? match[3] ?? "",
      ),
    ];
    for (const css of embeddedCss) {
      errors.push(...(await validateCssResources(css, outputRoot, file, siteUrl, relative)));
    }
    for (const match of relative === "print.html" ? [] : contents.matchAll(HTML_LINK)) {
      const raw = match[1];
      if (isExternal(raw)) continue;
      const { fragment } = splitTarget(raw);
      const target = localOutputPath(outputRoot, file, raw, siteUrl);
      if (target === null) errors.push(`${relative}: link escapes site base ${raw}`);
      else if (!(await exists(target))) errors.push(`${relative}: missing generated link ${raw}`);
      else if (fragment && target.endsWith(".html") && !htmlIds.get(target)?.has(fragment)) {
        errors.push(`${relative}: missing generated fragment ${raw}`);
      }
    }
  }

  for (const file of await walk(outputRoot, ".css")) {
    const contents = await readFile(file, "utf8");
    const relative = slash(path.relative(outputRoot, file));
    errors.push(...(await validateCssResources(contents, outputRoot, file, siteUrl, relative)));
  }
  return errors;
}

async function validateCssResources(css, outputRoot, contextFile, siteUrl, displayPath) {
  const errors = [];
  const references = [];
  for (const match of css.matchAll(/url\(\s*(?:"([^"]*)"|'([^']*)'|([^\s)'";]+))\s*\)/giu)) {
    references.push(match[1] ?? match[2] ?? match[3] ?? "");
  }
  for (const match of css.matchAll(/@import\s+(?!url\()(?:"([^"]*)"|'([^']*)')/giu)) {
    references.push(match[1] ?? match[2] ?? "");
  }
  for (const rawValue of references) {
    const raw = rawValue.trim();
    if (!raw || /^(?:data:|#)/iu.test(raw)) continue;
    if (isExternal(raw) || /^javascript:/iu.test(raw)) {
      errors.push(`${displayPath}: remote CSS runtime resource ${raw}`);
      continue;
    }
    const target = localOutputPath(outputRoot, contextFile, raw, siteUrl);
    if (target === null) errors.push(`${displayPath}: CSS resource escapes site ${raw}`);
    else if (!(await exists(target))) errors.push(`${displayPath}: missing CSS resource ${raw}`);
  }
  return errors;
}

function runtimeResourceValues(html) {
  const values = [];
  const allowedAttributes = new Map([
    ["audio", new Set(["src"])],
    ["embed", new Set(["src"])],
    ["iframe", new Set(["src"])],
    ["image", new Set(["href", "xlink:href"])],
    ["img", new Set(["src", "srcset"])],
    ["input", new Set(["src"])],
    ["link", new Set(["href"])],
    ["object", new Set(["data"])],
    ["script", new Set(["src"])],
    ["source", new Set(["src", "srcset"])],
    ["track", new Set(["src"])],
    ["use", new Set(["href", "xlink:href"])],
    ["video", new Set(["src", "poster"])],
  ]);
  for (const tag of html.matchAll(/<(audio|embed|iframe|image|img|input|link|object|script|source|track|use|video)\b([^>]*)>/giu)) {
    const tagName = tag[1].toLocaleLowerCase("en-US");
    for (const attribute of tag[2].matchAll(/\b(srcset|xlink:href|src|href|poster|data)\s*=\s*(?:"([^"]*)"|'([^']*)'|([^\s"'=<>`]+))/giu)) {
      const name = attribute[1].toLocaleLowerCase("en-US");
      if (!allowedAttributes.get(tagName)?.has(name)) continue;
      const raw = (attribute[2] ?? attribute[3] ?? attribute[4] ?? "").trim();
      if (name !== "srcset") {
        values.push(raw);
      } else {
        values.push(...srcsetCandidates(raw));
      }
    }
  }
  return values;
}

function srcsetCandidates(srcset) {
  const candidates = [];
  let cursor = 0;
  while (cursor < srcset.length) {
    while (/[\s,]/u.test(srcset[cursor] ?? "")) cursor += 1;
    if (cursor >= srcset.length) break;
    const start = cursor;
    while (cursor < srcset.length && !/\s/u.test(srcset[cursor])) cursor += 1;
    let candidate = srcset.slice(start, cursor);
    const trailingCommas = candidate.match(/,+$/u)?.[0].length ?? 0;
    if (trailingCommas > 0) candidate = candidate.slice(0, -trailingCommas);
    if (candidate) candidates.push(candidate);
    if (trailingCommas === 0) {
      let parentheses = 0;
      while (cursor < srcset.length) {
        const character = srcset[cursor];
        if (character === "(") parentheses += 1;
        else if (character === ")" && parentheses > 0) parentheses -= 1;
        else if (character === "," && parentheses === 0) {
          cursor += 1;
          break;
        }
        cursor += 1;
      }
    }
  }
  return candidates;
}

export function validateBrandCss(css) {
  const errors = [];
  for (const token of ["--eso-ink", "--eso-gold", "--eso-teal"]) {
    if (!css.includes(token)) errors.push(`brand CSS missing ${token}`);
  }
  if (!css.includes(":focus-visible")) errors.push("brand CSS missing focus-visible styling");
  if (!css.includes("prefers-reduced-motion: reduce")) errors.push("brand CSS missing reduced-motion override");
  if (!css.includes("max-width: 40rem")) errors.push("brand CSS missing narrow reflow rule");
  if (!/min-(?:height|width):\s*44px/gu.test(css)) errors.push("brand CSS missing 44px mobile target");

  const root = css.match(/:root\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  const light = css.match(/\.light,[\s\S]*?\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  const property = (block, name) =>
    block.match(new RegExp(`--${name}:\\s*(#[0-9a-f]{6})`, "iu"))?.[1];
  const contrastPairs = [
    ["dark text", property(root, "eso-text"), property(root, "eso-ink")],
    ["dark links", property(root, "eso-teal"), property(root, "eso-ink")],
    ["light text", property(light, "fg"), property(light, "bg")],
    ["light links", property(light, "links"), property(light, "bg")],
  ];
  for (const [label, foreground, background] of contrastPairs) {
    if (!foreground || !background) {
      errors.push(`brand CSS cannot resolve ${label} contrast colors`);
    } else if (contrastRatio(foreground, background) < 4.5) {
      errors.push(`brand CSS ${label} contrast is below 4.5:1`);
    }
  }
  const lightFocus = property(light, "eso-focus");
  for (const [label, background] of [
    ["light page", property(light, "bg")],
    ["light sidebar", property(light, "sidebar-bg")],
    ["light raised surface", property(light, "table-header-bg")],
  ]) {
    if (!lightFocus || !background) {
      errors.push(`brand CSS cannot resolve ${label} focus contrast colors`);
    } else if (contrastRatio(lightFocus, background) < 3) {
      errors.push(`brand CSS ${label} focus contrast is below 3:1`);
    }
  }
  if (!/@font-face[\s\S]*font-family:\s*"Inter"[\s\S]*Inter-Regular\.ttf/iu.test(css)) {
    errors.push("brand CSS missing local Inter Regular font face");
  }
  if (!/@font-face[\s\S]*font-family:\s*"Inter"[\s\S]*Inter-SemiBold\.ttf/iu.test(css)) {
    errors.push("brand CSS missing local Inter SemiBold font face");
  }
  return errors;
}

export function validateBrandJavascript(script) {
  const errors = [];
  if (!/querySelector\(["']main["']\)/u.test(script) || !/main\.id\s*=\s*["']main-content["']/u.test(script)) {
    errors.push("brand JavaScript must establish the main-content target");
  }
  if (!/main\.tabIndex\s*=\s*-1/u.test(script)) errors.push("main-content target must be programmatically focusable");
  if (!/href\s*=\s*["']#main-content["']/u.test(script) || !/textContent\s*=\s*["']Skip to main content["']/u.test(script)) {
    errors.push("brand JavaScript must create the skip-to-content link");
  }
  if (!/document\.body\.prepend\(/u.test(script)) errors.push("skip-to-content link must be the first body control");
  if (!/addEventListener\(["']click["'][\s\S]*main\.focus\(/u.test(script)) {
    errors.push("skip-to-content activation must transfer focus to main");
  }
  return errors;
}

export function contrastRatio(foreground, background) {
  const luminance = (hex) => {
    const channels = hex
      .slice(1)
      .match(/../gu)
      .map((channel) => Number.parseInt(channel, 16) / 255)
      .map((channel) => (channel <= 0.04045 ? channel / 12.92 : ((channel + 0.055) / 1.055) ** 2.4));
    return 0.2126 * channels[0] + 0.7152 * channels[1] + 0.0722 * channels[2];
  };
  const first = luminance(foreground);
  const second = luminance(background);
  return (Math.max(first, second) + 0.05) / (Math.min(first, second) + 0.05);
}

export function validateWorkflowText(workflow) {
  const errors = [];
  const withoutComments = workflow.replace(/\s+#.*$/gmu, "");
  if (/pull_request_target/gu.test(withoutComments)) errors.push("workflow must not use pull_request_target");
  const topLevel = withoutComments.split(/^jobs:/mu)[0] ?? withoutComments;
  if (/^\s+(?:pages|id-token):\s*write\s*$/gmu.test(topLevel)) errors.push("top-level Pages write permission is forbidden");
  if (/permissions:\s*write-all\b/iu.test(withoutComments)) errors.push("workflow must not use write-all permissions");
  if (!hasExactPermissions(topLevel, 0, new Map([["contents", "read"]]))) {
    errors.push("top-level permissions must contain only read-only contents");
  }
  for (const match of withoutComments.matchAll(/\buses:\s*[^\s@]+@([^\s#]+)/gu)) {
    if (!/^[0-9a-f]{40}$/u.test(match[1])) errors.push(`action must use an immutable commit SHA: ${match[0]}`);
  }

  const jobs = yamlBlocks(withoutComments, 2);
  const deploy = jobs.get("deploy") ?? "";
  const build = jobs.get("build") ?? "";
  const guard = "github.ref == 'refs/heads/main' && github.event_name != 'pull_request'";
  if (!deploy) errors.push("workflow requires a deploy job");
  if (!build) errors.push("workflow requires a build job");
  if (!new RegExp(`^\\s{4}if:\\s*${escapeRegExp(guard)}\\s*$`, "mu").test(deploy)) {
    errors.push("deploy job requires a main-only guard");
  }
  if (!hasExactPermissions(build, 4, new Map([["contents", "read"]]))) {
    errors.push("build job permissions must contain only read-only contents");
  }
  if (!hasExactPermissions(deploy, 4, new Map([["pages", "write"], ["id-token", "write"]]))) {
    errors.push("deploy job permissions must contain only Pages and OIDC writes");
  }
  if (!/^\s{4}needs:\s*build\s*$/mu.test(deploy)) errors.push("deploy job must consume the checked build job");
  if (!/^\s{6}name:\s*github-pages\s*$/mu.test(deploy)) errors.push("deploy job requires github-pages environment");
  for (const [name, block] of jobs) {
    if (name !== "deploy" && /^\s+(?:pages|id-token):\s*write\s*$/gmu.test(block)) {
      errors.push(`${name} job must not receive Pages or OIDC write permissions`);
    }
  }
  for (const stepName of ["Configure GitHub Pages", "Upload checked Pages artifact"]) {
    const step = yamlListBlock(build, stepName);
    if (!step || !new RegExp(`^\\s+if:\\s*${escapeRegExp(guard)}\\s*$`, "mu").test(step)) {
      errors.push(`${stepName} step requires a main-only guard`);
    }
  }
  return errors;
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
}

function yamlBlocks(text, indent) {
  const lines = text.split(/\r?\n/gu);
  const marker = new RegExp(`^ {${indent}}([A-Za-z0-9_-]+):\\s*$`, "u");
  const blocks = new Map();
  let name;
  let collected = [];
  for (const line of lines) {
    const match = line.match(marker);
    if (match) {
      if (name) blocks.set(name, collected.join("\n"));
      name = match[1];
      collected = [line];
    } else if (name) {
      collected.push(line);
    }
  }
  if (name) blocks.set(name, collected.join("\n"));
  return blocks;
}

function yamlListBlock(job, stepName) {
  const lines = job.split(/\r?\n/gu);
  const start = lines.findIndex((line) => line.trim() === `- name: ${stepName}`);
  if (start < 0) return "";
  const endOffset = lines.slice(start + 1).findIndex((line) => /^\s+- name:/u.test(line));
  const end = endOffset < 0 ? lines.length : start + 1 + endOffset;
  return lines.slice(start, end).join("\n");
}

function hasExactPermissions(text, indent, expected) {
  const lines = text.split(/\r?\n/gu);
  const header = `${" ".repeat(indent)}permissions:`;
  const start = lines.findIndex((line) => line.trimEnd() === header);
  if (start < 0) return false;
  const entries = new Map();
  const entry = new RegExp(`^ {${indent + 2}}([A-Za-z0-9_-]+):\\s*(read|write|none)\\s*$`, "u");
  for (const line of lines.slice(start + 1)) {
    if (line.trim() && line.length - line.trimStart().length <= indent) break;
    const match = line.match(entry);
    if (match) entries.set(match[1], match[2]);
  }
  return entries.size === expected.size && [...expected].every(([name, value]) => entries.get(name) === value);
}

async function run() {
  const docsRoot = path.resolve(process.argv[2] ?? "docs");
  const outputRoot = path.resolve(process.argv[3] ?? path.join("target", "docs-site", "html"));
  const workflowPath = path.resolve(".github", "workflows", "docs.yml");
  const cssPath = path.join(docsRoot, "theme", "eso-weave.css");
  const errors = [
    ...(await validateSourceTree(docsRoot)),
    ...(await validateGeneratedSite(outputRoot)),
    ...validateBrandCss(await readFile(cssPath, "utf8")),
    ...validateWorkflowText(await readFile(workflowPath, "utf8")),
  ];
  if (errors.length > 0) {
    for (const error of errors) console.error(`docs policy: ${error}`);
    process.exitCode = 1;
  } else {
    console.log("documentation site policy passed");
  }
}

const invokedPath = process.argv[1] ? pathToFileURL(process.argv[1]).href : "";
if (import.meta.url === invokedPath) await run();
