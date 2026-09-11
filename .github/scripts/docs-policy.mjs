import { createHash } from "node:crypto";
import { readdir, readFile, stat } from "node:fs/promises";
import path from "node:path";
import { pathToFileURL } from "node:url";
import { gzipSync } from "node:zlib";

const HEADING = /^#{1,6}\s+(.+?)\s*#*\s*$/gmu;
const HTML_LINK = /<a\b[^>]*?\bhref=["']([^"']+)["']/giu;

const MIGRATION_BASELINE = "bdc7b228b78ef535d07357e7b33a36bbade917cc";
const BASELINE_ARTIFACTS = new Map([
  ["docs/ESO-Weave-Specification.md", "e7fb461cc1492d98fef62eab728dbaf8dcb9af4d"],
  ["docs/architecture-decisions/0001-mdbook-documentation-site.md", "0a33f2681c0403c3a8abc2a6ed30b37211f5c2bd"],
  ["docs/book.toml", "ca64b8faccf576678c0ba4d07daa18ae286cad5c"],
  ["docs/brand/ESO-Weave-Brand-v1.md", "80bcf6487ff47c0c82ad467ae7148d8b6483bebf"],
  ["docs/build-autopilot.md", "96d94fd8312e5a182a0bab5f396164559f42c639"],
  ["docs/plans/README.md", "04734e75e4340197ca105f44adf7ca7f5a3d35dc"],
  ["docs/plans/plan-001.md", "a11fce81b081d79cf81ea4ae5fa72f78157b602b"],
  ["docs/plans/plan-002.md", "1e153b471eb230d2af888e13bcd518e49de3a21c"],
  ["docs/plans/plan-003.md", "e05ae22a4b5a35370ef553a1a1fc82dcc6a992e2"],
  ["docs/plans/plan-004.md", "4706ffbcea4b51d3a1426c4839481674be262c4b"],
  ["docs/plans/plan-005.md", "f68a1d8064304c212fa79d65a746c4cb0d21e261"],
  ["docs/plans/plan-006.md", "d51418161ffd0c5b28e060efafb85560d6cc825c"],
  ["docs/plans/plan-007.md", "af9d082769238e47086d1d3e91ca603ce78db36a"],
  ["docs/plans/plan-008.md", "3cb1791b0ea28b26e91e61f05fadb06bec4b0e8f"],
  ["docs/plans/plan-009.md", "efc5882c8524bb0d2ce79f7c44df4be995bbac75"],
  ["docs/plans/plan-010.md", "1a4619a961c4422c76cdab7f001414e9fcabe055"],
  ["docs/plans/plan-011.md", "70fb3bfcb2cf2426bfd5c670bdccffa018d8bfbe"],
  ["docs/plans/plan-012.md", "d6297bc8951f43bae2c0e6bde150c95ec0b56eee"],
  ["docs/plans/plan-013.md", "5397e1eacb1757ea254b99208216f6393e12e2b0"],
  ["docs/plans/plan-014.md", "e37c7cfd15665e580440c75db8eb19fe05e7afe7"],
  ["docs/plans/plan-015.md", "cfe1e7fa6ce279e5457136c10466bacec3777e24"],
  ["docs/plans/plan-016.md", "4cc55998cdd69515c89cac1aec0b091fcef2dc0c"],
  ["docs/plans/plan-017.md", "cb487f47d70a7ba98c5c1843943ef620c0bbb951"],
  ["docs/plans/plan-018.md", "8545690bc42ead247be04f9cbf60533a94c2bf7e"],
  ["docs/plans/plan-019.md", "84242a8aceb9ab87869863c13f8988581c4a1533"],
  ["docs/plans/plan-020.md", "6951ab1dec2dc2388bfebaf89a0a12f94e7dee59"],
  ["docs/plans/plan-021.md", "c1cd1c2b4500153a90174e0713ce1f4877b2f60e"],
  ["docs/plans/plan-022.md", "1f77ec256e32291b7c6b3125cb6df92cc3ae2863"],
  ["docs/plans/plan-023.md", "86d834d999a63f891c57fa0737b2971d78fe72c0"],
  ["docs/plans/plan-024.md", "e468a9039d6d72c97d55658cdd0347ffa369d12f"],
  ["docs/plans/plan-025.md", "7e72815b837bc292a430a9bd9ec77ee89ff5ae5d"],
  ["docs/plans/plan-026.md", "1e5eca82ae31be31ff2a7435e67cd3bc89664a96"],
  ["docs/plans/plan-027.md", "518a01fa2fecedca26fcd7ec054e514a43e0ba5b"],
  ["docs/project-governance.md", "67c895f427ce5380ec5c03ee3364a73bfa43c716"],
  ["docs/releasing.md", "5f5f33351f93eb11eb8c78c46fd265c1b16c4f3c"],
  ["docs/src/404.md", "27385f2f0abb01be2f8317524804d71a969e530c"],
  ["docs/src/README.md", "07f1f638a2f17a498370a11f77dceb5b823f6341"],
  ["docs/src/SUMMARY.md", "c62ee5d6d56490d223fd63d52da6800a5f8bfb4f"],
  ["docs/src/assets/brand/eso-weave-mark.svg", "9f102a3e1fa285ac05659ce3d1b62af151e75812"],
  ["docs/src/assets/brand/fonts/Inter-Regular.ttf", "012d1b470d92db48d6f45478f9711f088a6c7359"],
  ["docs/src/assets/brand/fonts/Inter-SemiBold.ttf", "4be54399d679a0cc0ed46526f79d8e53cec2a1a5"],
  ["docs/src/assets/brand/fonts/OFL.txt", "ff80f8c615684e796c37ab5ff82a9b31d7390d6e"],
  ["docs/src/concepts/README.md", "c5fdeb855f644a58810b8b6db99f7484d91dca26"],
  ["docs/src/development/README.md", "c14e20138797031814858c92d92dcd7ad8b209e6"],
  ["docs/src/features/README.md", "2af6689aa984b015515331c5f4f34085adfca391"],
  ["docs/src/getting-started/README.md", "e4e3add4cfad1abe7e76142bea428b7b0ee0c58e"],
  ["docs/src/reference/README.md", "c07fb227c9ab6539b6b99dcf1d8c4a3ba731b087"],
  ["docs/theme/eso-weave.css", "2e46acafe395a981462b740516bdd660b9f8bb9c"],
  ["docs/theme/eso-weave.js", "b89c06f742105b3e9ca1e203d86a7e461f387de8"],
  ["website/content/blog/ultimate-resource-meter.md", "7c6628bc3595c2bf003ca5b65dc56eb2a68a213c"],
]);
const SPECIFICATION_HEADINGS = [
  "Table of Contents",
  "1. Overview",
  "2. Terminology",
  "3. Scope",
  "4. Platform Support",
  "5. System Architecture",
  "6. Concurrency and Ownership",
  "7. Input Engine",
  "8. Weave Engine",
  "9. Fishing Automation",
  "10. PixelBeacon Companion Addon",
  "11. Auto-Potion",
  "12. Graphical User Interface",
  "13. Configuration and Session State",
  "14. Logging",
  "15. Packaging and Distribution",
  "16. Repository Conventions",
  "17. README Disclaimer Text",
  "Disclaimer",
  "Appendix A. Weave Delay Defaults",
];
const SAFETY_INVARIANTS = new Set([
  "injected-input-recursion-breaking",
  "focused-window-only-suppression",
  "non-blocking-hook-callbacks",
  "managed-marker-gated-uninstall",
  "addons-directory-containment",
  "fishing-signal-loss-fail-closed",
]);
const LEGACY_LITERALS = new Set([
  "docs/ESO-Weave-Specification.md",
  "docs/build-autopilot.md",
  "docs/project-governance.md",
  "docs/releasing.md",
  "docs/plans/",
  "website/content/blog/ultimate-resource-meter.md",
]);
const HISTORICAL_EXCEPTIONS = new Map([
  ["CHANGELOG.md\u0000docs/ESO-Weave-Specification.md", 4],
  ["CHANGELOG.md\u0000docs/releasing.md", 2],
  ["CHANGELOG.md\u0000docs/plans/", 2],
]);
const NON_PUBLISHED_SEARCH_SENTINELS = [
  "Migration Ledger",
  "Current Build Plans",
  "Archived Build Plans",
];
const PRESERVATION_MANIFEST_SHA256 = "5067105e76147adced7e4a94b3afc5e57bad7dc013059c44eb5663e397a0542e";
const DELIVERY_EVIDENCE = /(?:\bPR #\d+\b|\bcommit [0-9a-f]{7,40}\b|\bv\d+\.\d+\.\d+ release\b|\bissue #\d+ closed\b)/iu;
const CONTENT_COVERAGE_BASELINE = "bc2b8356f3bec7b5349be5dbe616e1a10c93f244";
const CONTENT_OBLIGATION_IDS = new Set([
  ...Array.from({ length: 25 }, (_, index) => `LOG-${String(index + 1).padStart(3, "0")}`),
  ...Array.from({ length: 7 }, (_, index) => `CFG-${String(index + 1).padStart(3, "0")}`),
  "PLT-001", "PLT-002",
  ...Array.from({ length: 6 }, (_, index) => `REL-${String(index + 1).padStart(3, "0")}`),
  ...Array.from({ length: 7 }, (_, index) => `DEF-${String(index + 1).padStart(3, "0")}`),
]);
const DEFERRED_ISSUES = new Set();
const COVERAGE_LABELS = new Set(["Guarantee", "Implementation", "Diagnostic", "VersionSensitive"]);
const DIAGRAM_IDS = new Set(["DIA-001", "DIA-002", "DIA-003", "DIA-004", "DIA-005", "DIA-006"]);
const CONTENT_CONTRACT_SHA256 = "8493d71d66dd81874967f91e34c41a82692136c022762476bd44d6e7244a9db0";
const CONTENT_PAGE_PATHS = new Set([
  "docs/src/README.md",
  "docs/src/getting-started/installation.md",
  "docs/src/getting-started/first-launch.md",
  "docs/src/getting-started/troubleshooting.md",
  "docs/src/features/weaving.md",
  "docs/src/features/fishing.md",
  "docs/src/features/auto-potion.md",
  "docs/src/features/pixelbeacon.md",
  "docs/src/concepts/action-authorization.md",
  "docs/src/concepts/game-observation.md",
  "docs/src/reference/settings.md",
  "docs/src/reference/configuration.md",
  "docs/src/development/test-strategy.md",
  "docs/src/development/release-and-packaging.md",
]);
const SEARCH_TARGETS = new Map([
  ["ESO Weave", "docs/src/README.md"], ["Weaving", "docs/src/features/weaving.md"],
  ["Light Attack", "docs/src/features/weaving.md"], ["Heavy Attack", "docs/src/features/weaving.md"],
  ["Bash Attack", "docs/src/features/weaving.md"], ["Block Casting", "docs/src/features/weaving.md"],
  ["Global Cooldown", "docs/src/features/weaving.md"], ["Weave Delay", "docs/src/features/weaving.md"],
  ["Weapon Bar", "docs/src/features/weaving.md"], ["Latency Adaptation", "docs/src/features/weaving.md"],
  ["Input Safety", "docs/src/concepts/input-safety.md"], ["Synthesized Input", "docs/src/concepts/input-safety.md"],
  ["Suspension", "docs/src/concepts/input-safety.md"], ["Menu Gate", "docs/src/concepts/action-authorization.md"],
  ["Game Context", "docs/src/concepts/game-observation.md"], ["Life State", "docs/src/concepts/game-observation.md"],
  ["World State", "docs/src/concepts/game-observation.md"], ["Travel", "docs/src/concepts/game-observation.md"],
  ["Roll Dodge", "docs/src/concepts/game-observation.md"], ["Sprinting", "docs/src/concepts/game-observation.md"],
  ["Unknown", "docs/src/concepts/action-authorization.md"], ["Fishing", "docs/src/features/fishing.md"],
  ["Interact Key", "docs/src/features/fishing.md"], ["No Cast Detected", "docs/src/features/fishing.md"],
  ["Signal Lost", "docs/src/getting-started/troubleshooting.md"], ["Auto Potion", "docs/src/features/auto-potion.md"],
  ["Resource Watch", "docs/src/features/auto-potion.md"], ["Quickslot", "docs/src/features/auto-potion.md"],
  ["Retry Interval", "docs/src/features/auto-potion.md"], ["PixelBeacon", "docs/src/features/pixelbeacon.md"],
  ["Pixel Bus", "docs/src/reference/pixel-bus-protocol.md"], ["Layout Header", "docs/src/reference/pixel-bus-protocol.md"],
  ["Payload Block", "docs/src/reference/pixel-bus-protocol.md"], ["Heartbeat", "docs/src/reference/pixel-bus-protocol.md"],
  ["Capture Tolerance", "docs/src/reference/pixel-bus-protocol.md"], ["Ultimate", "docs/src/features/ultimate-resource.md"],
  ["Ultimate Cost", "docs/src/features/ultimate-resource.md"], ["Ready", "docs/src/features/ultimate-resource.md"],
  ["Configuration", "docs/src/reference/configuration.md"], ["Session State", "docs/src/reference/configuration.md"],
  ["Invalid Configuration", "docs/src/reference/configuration.md"], ["Live Log", "docs/src/reference/logging.md"],
  ["File Logging", "docs/src/reference/logging.md"], ["Windows Input", "docs/src/concepts/scope-and-platform.md"],
  ["Linux Input", "docs/src/concepts/scope-and-platform.md"], ["XWayland", "docs/src/concepts/scope-and-platform.md"],
  ["PixelBeacon Status", "docs/src/features/pixelbeacon.md"], ["API Version", "docs/src/features/pixelbeacon.md"],
  ["Release Package", "docs/src/getting-started/installation.md"], ["Checksum", "docs/src/getting-started/installation.md"],
  ["Troubleshooting", "docs/src/getting-started/troubleshooting.md"], ["Startup Failure", "docs/src/getting-started/troubleshooting.md"],
  ["Test Strategy", "docs/src/development/test-strategy.md"], ["Release Pipeline", "docs/src/development/release-and-packaging.md"],
]);

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

function maskCharacters(characters, start, end) {
  for (let index = start; index < end; index += 1) {
    if (characters[index] !== "\r" && characters[index] !== "\n") characters[index] = " ";
  }
}

function maskHtmlTags(characters) {
  for (let cursor = 0; cursor < characters.length - 1; cursor += 1) {
    if (characters[cursor] !== "<" || !/[A-Za-z/!?]/u.test(characters[cursor + 1])) continue;
    let quote = "";
    let tagEnd = cursor + 1;
    for (; tagEnd < characters.length; tagEnd += 1) {
      const character = characters[tagEnd];
      if (quote) {
        if (character === quote) quote = "";
        continue;
      }
      if (character === '"' || character === "'") {
        quote = character;
      } else if (character === ">") {
        break;
      }
    }
    if (characters[tagEnd] !== ">") continue;
    maskCharacters(characters, cursor, tagEnd + 1);
    cursor = tagEnd;
  }
}

function decodeVisibleEntities(text) {
  const named = new Map([
    ["amp", "&"], ["bsol", "\\"], ["colon", ":"], ["hyphen", "-"],
    ["apos", "'"], ["gt", ">"], ["lowbar", "_"], ["lt", "<"],
    ["nbsp", " "], ["num", "#"], ["quot", "\""], ["sol", "/"], ["tab", "\t"],
  ]);
  return text.replace(/&(?:#(?<decimal>\d+)|#x(?<hex>[0-9a-f]+)|(?<name>[A-Za-z]+));/giu, (entity, ...args) => {
    const groups = args.at(-1);
    if (groups.decimal) {
      const value = Number.parseInt(groups.decimal, 10);
      return value <= 0x10ffff ? String.fromCodePoint(value) : entity;
    }
    if (groups.hex) {
      const value = Number.parseInt(groups.hex, 16);
      return value <= 0x10ffff ? String.fromCodePoint(value) : entity;
    }
    return named.get(groups.name) ?? named.get(groups.name.toLowerCase()) ?? entity;
  });
}

function maskMarkdownReferenceDefinitions(characters) {
  const text = characters.join("");
  const lines = [];
  let offset = 0;
  for (const value of text.match(/.*(?:\r?\n|$)/gu) ?? []) {
    const content = value.replace(/\r?\n$/u, "");
    lines.push({ content, offset, length: value.length });
    offset += value.length;
  }

  for (let index = 0; index < lines.length; index += 1) {
    const line = lines[index];
    const definition = line.content.match(/^ {0,3}\[[^\]\r\n]+\]:[ \t]*(?<destination>.*)$/u);
    if (!definition) continue;
    maskCharacters(characters, line.offset, line.offset + line.length);

    let destinationLine = index;
    if (!definition.groups.destination.trim()) {
      const continuation = lines[index + 1];
      if (!continuation || !/^ {0,3}(?:<[^>\r\n]+>|\S+)/u.test(continuation.content)) continue;
      maskCharacters(characters, continuation.offset, continuation.offset + continuation.length);
      destinationLine = index + 1;
    }

    const title = lines[destinationLine + 1];
    if (title && /^ {0,3}(?:"[^"\r\n]*"|'[^'\r\n]*'|\([^\)\r\n]*\))[ \t]*$/u.test(title.content)) {
      maskCharacters(characters, title.offset, title.offset + title.length);
      index = destinationLine + 1;
    } else {
      index = destinationLine;
    }
  }
}

function markdownContainerContent(line) {
  let content = line;
  while (true) {
    const blockQuote = content.match(/^ {0,3}>[ \t]?/u);
    if (blockQuote) {
      content = content.slice(blockQuote[0].length);
      continue;
    }
    const listItem = content.match(/^ {0,3}(?:[-+*]|\d{1,9}[\.\)])[ \t]+/u);
    if (listItem) {
      content = content.slice(listItem[0].length);
      continue;
    }
    return content;
  }
}

const DOCUMENTATION_FENCE_LANGUAGES = new Set(["bash", "json", "powershell", "text"]);
const DOCUMENTATION_FENCE_COUNTS = new Map([["bash", 12], ["powershell", 3], ["text", 8]]);
const PLAIN_CODE_BLOCKS = [
  { page: "development/catalog-updates.md", sha256: "394477b8bebb95ddb31e3dbf93c78908468e2f69d91e6992f84b27d916886883", rationale: "A path layout, not an executable command." },
  { page: "development/discovery-collector.md", sha256: "26982824fe45ffb5c1224ed89d9f04d0585059214f5431d5bd2b2065b25bbad3", rationale: "ESO slash commands shown as an in-game sequence, not shell syntax." },
  { page: "development/repository-conventions.md", sha256: "ba72b7779b972c573ded96d80a6d6d21e9416720419c1ddc287adbe3bab0e227", rationale: "A repository directory map, not executable syntax." },
  { page: "features/encounter-capture.md", sha256: "7670ddaeb06a92493c7156e32a8eb3bf286fb146597d34530d28b7486ec1eb94", rationale: "ESO slash commands shown as an in-game sequence, not shell syntax." },
  { page: "features/weaving.md", sha256: "74576ddc34a330808aa09f8bbc76348f573f3686a2eec2555060dc5651c3bee5", rationale: "A mathematical formula, not executable syntax." },
  { page: "features/weaving.md", sha256: "4fa825b4a104e43a5c6c4fff848dfd7f8192486b146e1ce02ee9f551694e62b1", rationale: "An authorization process flow, not executable syntax." },
  { page: "getting-started/troubleshooting.md", sha256: "37c611fcdadd2bf1f87561e8fa4d110d1d1fcab662a40d3771d0cd1a3e0ee04a", rationale: "A diagnostic decision tree, not executable syntax." },
  { page: "reference/pixel-bus-protocol.md", sha256: "e159118f0007ffa610a1afedeb56051affb6758336b096c99e6456a0b828e3e4", rationale: "Protocol geometry formulas, not executable syntax." },
];

function fenceDigest(content) {
  return createHash("sha256").update(content, "utf8").digest("hex");
}

export function parseDocumentationFences(markdown, relative) {
  const fences = [];
  const errors = [];
  const lines = markdown.split(/\r?\n/gu);
  let active = null;
  for (let index = 0; index < lines.length; index += 1) {
    const rawLine = lines[index];
    const content = markdownContainerContent(rawLine);
    if (active) {
      const closing = content.match(new RegExp(`^ {0,3}${active.character}{${active.length},}[ \\t]*$`, "u"));
      if (closing) {
        const blockContent = active.content.join("\n");
        fences.push({
          page: relative,
          line: active.line,
          language: active.language,
          content: blockContent,
          sha256: fenceDigest(blockContent),
        });
        active = null;
      } else {
        let codeLine = rawLine;
        if (active.containerPrefix && codeLine.startsWith(active.containerPrefix)) {
          codeLine = codeLine.slice(active.containerPrefix.length);
        }
        if (active.indent && codeLine.startsWith(active.indent)) codeLine = codeLine.slice(active.indent.length);
        active.content.push(codeLine);
      }
      continue;
    }
    const opening = content.match(/^ {0,3}(`{3,}|~{3,})(?<info>.*)$/u);
    if (!opening) continue;
    const info = opening.groups.info.trim();
    const identifier = info.match(/^[A-Za-z0-9_-]+$/u)?.[0] ?? "";
    if (!identifier) {
      errors.push(`${relative}:${index + 1}: S087 fenced code block requires one language identifier`);
    }
    active = {
      character: opening[1][0],
      length: opening[1].length,
      language: identifier.toLocaleLowerCase("en-US"),
      line: index + 1,
      content: [],
      containerPrefix: rawLine.slice(0, rawLine.length - content.length),
      indent: content.match(/^ {0,3}/u)?.[0] ?? "",
    };
  }
  if (active) errors.push(`${relative}:${active.line}: S087 fenced code block is unclosed`);
  return { fences, errors };
}

export function validateDocumentationCodeFences(pages, options = {}) {
  const complete = options.complete ?? true;
  const plainExceptions = options.plainExceptions ?? PLAIN_CODE_BLOCKS;
  const errors = [];
  const fences = [];
  for (const [page, markdown] of pages) {
    const parsed = parseDocumentationFences(markdown, page);
    errors.push(...parsed.errors);
    fences.push(...parsed.fences);
  }
  const seenPlain = new Set();
  for (const fence of fences) {
    if (["console", "sh"].includes(fence.language)) {
      errors.push(`${fence.page}:${fence.line}: S087 ${fence.language} is an obsolete documentation fence identifier`);
      continue;
    }
    if (!DOCUMENTATION_FENCE_LANGUAGES.has(fence.language)) {
      errors.push(`${fence.page}:${fence.line}: S087 unknown documentation fence identifier ${fence.language || "(missing)"}`);
      continue;
    }
    if (fence.language !== "text") continue;
    const exception = plainExceptions.find((candidate) => candidate.page === fence.page && candidate.sha256 === fence.sha256);
    if (!exception) {
      errors.push(`${fence.page}:${fence.line}: S087 plain block is not in the exact content allowlist`);
      continue;
    }
    if (typeof exception.rationale !== "string" || exception.rationale.trim().length < 20) {
      errors.push(`${fence.page}:${fence.line}: S087 plain block requires a substantive rationale`);
    }
    seenPlain.add(`${exception.page}\0${exception.sha256}`);
  }
  if (complete) {
    if (fences.length !== 23) errors.push(`S087 documentation corpus requires exactly 23 fenced blocks, found ${fences.length}`);
    for (const [language, count] of DOCUMENTATION_FENCE_COUNTS) {
      const actual = fences.filter((fence) => fence.language === language).length;
      if (actual !== count) errors.push(`S087 documentation corpus requires ${count} ${language} blocks, found ${actual}`);
    }
    for (const exception of plainExceptions) {
      if (!seenPlain.has(`${exception.page}\0${exception.sha256}`)) {
        errors.push(`S087 plain block allowlist entry is missing from the corpus: ${exception.page} ${exception.sha256}`);
      }
    }
  }
  return [...new Set(errors)];
}

function generatedPageForMarkdown(relative) {
  if (relative === "README.md") return "index.html";
  if (relative.endsWith("/README.md")) return `${relative.slice(0, -"README.md".length)}index.html`;
  return relative.replace(/\.md$/u, ".html");
}

function generatedCodeBlocks(html) {
  const blocks = [];
  for (const match of html.matchAll(/<pre><code\b(?<attributes>[^>]*)>(?<content>[\s\S]*?)<\/code><\/pre>/giu)) {
    const classes = match.groups.attributes.match(/\bclass\s*=\s*["']([^"']*)["']/iu)?.[1]?.split(/\s+/gu) ?? [];
    const language = classes.find((name) => name.startsWith("language-"));
    const content = decodeVisibleEntities(match.groups.content.replace(/<[^>]*>/gu, "")).replace(/\r?\n$/u, "");
    blocks.push({ language: language?.slice("language-".length) ?? "", content });
  }
  return blocks;
}

export function validateDocumentationCodeBlocksGenerated(sourcePages, generatedPages, outputPaths) {
  const errors = [];
  for (const [sourcePage, markdown] of sourcePages) {
    const source = parseDocumentationFences(markdown, sourcePage);
    if (source.fences.length === 0) continue;
    const outputPage = generatedPageForMarkdown(sourcePage);
    const html = generatedPages.get(outputPage);
    if (typeof html !== "string") {
      errors.push(`S087 generated code page is missing: ${outputPage}`);
      continue;
    }
    const expected = source.fences.map((fence) => ({ language: fence.language, content: fence.content }));
    const actual = generatedCodeBlocks(html);
    if (expected.length !== actual.length || expected.some((block, index) => block.language !== actual[index]?.language)) {
      errors.push(`S087 generated code language order differs for ${outputPage}: expected ${expected.map((block) => block.language).join(",")}, found ${actual.map((block) => block.language).join(",")}`);
    }
    if (expected.length === actual.length && expected.some((block, index) => block.content !== actual[index].content)) {
      errors.push(`S087 generated code text differs from its Markdown source for ${outputPage}`);
    }
  }
  for (const [label, pattern] of [
    ["Highlight.js runtime", /^highlight-[0-9a-z]+\.js$/u],
    ["light highlight palette", /^highlight-[0-9a-z]+\.css$/u],
    ["dark highlight palette", /^tomorrow-night-[0-9a-z]+\.css$/u],
    ["ayu highlight palette", /^ayu-highlight-[0-9a-z]+\.css$/u],
    ["ESO Weave highlighting extension", /^theme\/eso-weave-[0-9a-z]+\.js$/u],
    ["ESO Weave highlighting palette", /^theme\/eso-weave-[0-9a-z]+\.css$/u],
  ]) {
    const matches = [...outputPaths].filter((file) => pattern.test(file));
    if (matches.length !== 1) errors.push(`S087 generated output requires exactly one local ${label}`);
  }
  return errors;
}

export function validateHighlightingExtension(script) {
  const errors = [];
  if (!/hljs\.registerLanguage\(["']eso-command["']/u.test(script)) errors.push("S087 local command grammar registration is missing");
  if (!/hljs\.registerLanguage\(["']eso-powershell["']/u.test(script)) errors.push("S087 local PowerShell grammar registration is missing");
  if (!/querySelectorAll\(["']code\.language-bash, code\.language-powershell["']\)/u.test(script)) errors.push("S087 re-highlighting selector must remain bounded to Bash and PowerShell blocks");
  if (!/dataset\.esoHighlighted/u.test(script)) errors.push("S087 re-highlighting requires an idempotent data marker");
  if (!/hljs\.highlightBlock\(/u.test(script)) errors.push("S087 local grammars must reuse the bundled Highlight.js runtime");
  if (/\b(?:fetch|XMLHttpRequest|import\s*\(|https?:\/\/)/u.test(script)) errors.push("S087 highlighting extension must remain local and offline");
  return errors;
}

function cssVariables(block) {
  return new Map([...block.matchAll(/--([a-z0-9-]+):\s*(#[0-9a-f]{6})\s*;/giu)].map((match) => [match[1], match[2]]));
}

export function validateSyntaxHighlightingCss(css) {
  const errors = [];
  const darkBlock = css.match(/\.navy\s*,\s*\.coal\s*,\s*\.ayu\s*\{(?<body>[\s\S]*?)\}/u)?.groups.body ?? "";
  const lightBlock = css.match(/\.light\s*,\s*\.rust\s*\{(?<body>[\s\S]*?)\}/u)?.groups.body ?? "";
  const roles = ["comment", "keyword", "number", "string", "title", "variable", "punctuation"];
  for (const [family, block] of [["dark", darkBlock], ["light", lightBlock]]) {
    const variables = cssVariables(block);
    const background = variables.get("eso-code-background");
    if (!background) {
      errors.push(`S087 ${family} syntax palette requires --eso-code-background`);
      continue;
    }
    for (const role of roles) {
      const color = variables.get(`eso-code-${role}`);
      if (!color) errors.push(`S087 ${family} syntax palette requires ${role}`);
      else if (contrastRatio(color, background) < 4.5) errors.push(`S087 ${family} ${role} token contrast must be at least 4.5:1`);
    }
    const distinctColors = new Set(roles.map((role) => variables.get(`eso-code-${role}`)).filter(Boolean));
    if (distinctColors.size < 5) errors.push(`S087 ${family} syntax palette requires at least five distinct token colors`);
  }
  if (!/pre\s*>\s*code\.hljs\s*\{[^}]*background:\s*var\(--eso-code-background\)/iu.test(css)) errors.push("S087 code blocks must use the project-owned syntax background");
  for (const role of roles) {
    if (!new RegExp(`\\.hljs-${role}\\b[^{}]*\\{[^}]*color:\\s*var\\(--eso-code-${role}\\)`, "iu").test(css)) {
      errors.push(`S087 syntax palette must map the ${role} token class`);
    }
  }
  return errors;
}

function maskMarkdownWorkSliceExceptions(markdown) {
  const characters = markdown.split("");
  const lines = markdown.match(/.*(?:\r?\n|$)/gu) ?? [];
  let offset = 0;
  let fence = null;

  for (const line of lines) {
    const containerContent = markdownContainerContent(line);
    const opening = containerContent.match(/^ {0,3}(`{3,}|~{3,})/u);
    const closing = fence
      ? containerContent.match(new RegExp(`^ {0,3}${fence.character}{${fence.length},}\\s*$`, "u"))
      : null;
    if (fence || opening) {
      maskCharacters(characters, offset, offset + line.length);
      if (closing) fence = null;
      else if (!fence) fence = { character: opening[1][0], length: opening[1].length };
    }
    offset += line.length;
  }

  const masked = characters.join("");
  for (const match of masked.matchAll(/<!--[\s\S]*?-->/gu)) {
    maskCharacters(characters, match.index, match.index + match[0].length);
  }
  maskHtmlTags(characters);

  maskMarkdownReferenceDefinitions(characters);

  const linkSyntax = maskMarkdownCode(characters.join(""));
  for (let cursor = 0; cursor < linkSyntax.length; cursor += 1) {
    const image = linkSyntax[cursor] === "!" && linkSyntax[cursor + 1] === "[";
    const labelStart = image ? cursor + 1 : cursor;
    if (linkSyntax[labelStart] !== "[") continue;
    let labelDepth = 0;
    let labelEnd = labelStart + 1;
    for (; labelEnd < linkSyntax.length; labelEnd += 1) {
      if (linkSyntax[labelEnd] === "\\") labelEnd += 1;
      else if (linkSyntax[labelEnd] === "[") labelDepth += 1;
      else if (linkSyntax[labelEnd] === "]" && labelDepth > 0) labelDepth -= 1;
      else if (linkSyntax[labelEnd] === "]") break;
    }
    if (linkSyntax[labelEnd] !== "]" || linkSyntax[labelEnd + 1] !== "(") continue;
    let depth = 0;
    let quote = "";
    let destinationEnd = labelEnd + 2;
    for (; destinationEnd < linkSyntax.length; destinationEnd += 1) {
      const character = linkSyntax[destinationEnd];
      if (character === "\\") {
        destinationEnd += 1;
        continue;
      }
      if (quote) {
        if (character === quote) quote = "";
        continue;
      }
      if ((character === '"' || character === "'") && /\s/u.test(linkSyntax[destinationEnd - 1] ?? "")) {
        quote = character;
        continue;
      }
      if (character === "(") depth += 1;
      else if (character === ")" && depth > 0) depth -= 1;
      else if (character === ")") break;
    }
    if (linkSyntax[destinationEnd] !== ")") continue;
    maskCharacters(characters, labelEnd + 2, destinationEnd);
    cursor = destinationEnd;
  }
  return characters.join("");
}

const WORK_SLICE_ALGORITHM_PAGES = new Set([
  "development/architecture.md",
  "development/architecture.html",
  "print.html",
  "reference/encounter-data-and-metrics.md",
  "reference/encounter-data-and-metrics.html",
]);

function validateVisibleWorkSliceText(text, relative) {
  const errors = [];
  text = decodeVisibleEntities(text);
  const visible = WORK_SLICE_ALGORITHM_PAGES.has(relative)
    ? text.replace(/(?<![A-Za-z0-9_-])s069-v1(?![A-Za-z0-9_-])/gu, "       ")
    : text;
  const findings = [
    /\bslice(?:[ \t_-]+|[ \t]*[:#][ \t]*)(?:[Ss][ \t_:#-]*\d+|\d+)\b/giu,
    /\bspecs[\\/]\d+-[a-z0-9][a-z0-9-]*(?:[\\/][^\s<]*)?/giu,
    /\bs\d+(?:[_-][a-z0-9][a-z0-9_-]*)+\b/giu,
    /\b[sS]\d+[A-Za-z][A-Za-z0-9_-]*\b/gu,
    /\b[sS][ \t_:#-]+\d+\b/gu,
  ];
  for (const pattern of findings) {
    for (const match of visible.matchAll(pattern)) {
      const line = visible.slice(0, match.index).split("\n").length;
      errors.push(`${relative}:${line}: S085 work-slice reference must use S###; found ${match[0]}`);
    }
  }
  for (const match of visible.matchAll(/(?<![A-Za-z0-9_])[sS]\d+(?![A-Za-z0-9_-])/gu)) {
    if (/^S\d{3}$/u.test(match[0])) continue;
    const line = visible.slice(0, match.index).split("\n").length;
    errors.push(`${relative}:${line}: S085 work-slice reference must use S###; found ${match[0]}`);
  }
  return [...new Set(errors)];
}

export function validateWorkSliceMarkdown(markdown, relative) {
  return validateVisibleWorkSliceText(maskMarkdownWorkSliceExceptions(markdown), relative);
}

export function validateWorkSliceHtml(html, relative) {
  const withoutBlocks = html
    .replace(/<!--[\s\S]*?-->/gu, " ")
    .replace(/<(pre|script|style)\b[^>]*>[\s\S]*?<\/\1>/giu, " ");
  const characters = withoutBlocks.split("");
  maskHtmlTags(characters);
  return validateVisibleWorkSliceText(characters.join(""), relative);
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

export async function validateSourceTree(docsRoot, options = {}) {
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
  const markdownPages = new Map();
  for (const file of markdownFiles) {
    const relative = slash(path.relative(sourceRoot, file));
    if (!["SUMMARY.md", "404.md"].includes(relative) && !counts.has(relative)) {
      errors.push(`${relative} is not listed in SUMMARY.md`);
    }
    const contents = await readFile(file, "utf8");
    markdownPages.set(relative, contents);
    if (relative !== "SUMMARY.md" && !/^#\s+\S+/mu.test(contents)) {
      errors.push(`${relative} has no non-empty level-one heading`);
    }
    errors.push(...(await validateMarkdownLinks(sourceRoot, file, contents)));
    errors.push(
      ...validateSettingsRuntimeClaims(contents).map(
        (error) => `${relative}: ${error}`,
      ),
    );
    errors.push(...validateWorkSliceMarkdown(contents, relative));
  }
  errors.push(...validateDocumentationCodeFences(markdownPages, { complete: options.completeFenceInventory ?? false }));
  return errors;
}

/// Rejects settings claims superseded by S062's per-setting runtime contract.
export function validateSettingsRuntimeClaims(markdown) {
  const obsolete = [
    [
      "blanket immediate-application claim",
      /\b(?:all settings apply immediately|any change to the draft is applied live)\b/iu,
    ],
    [
      "live-reader restart claim",
      /\b(?:live reader fields require restart|some Fishing and Pixel Bus settings currently require an application restart|Color Tolerance and sampling intervals are (?:also )?saved for the next application start|Fishing and Pixel Bus changes are saved but are not propagated to their running components, so restart ESO Weave after changing them)\b/iu,
    ],
    [
      "missing Fishing Interact Key claim",
      /\b(?:Fishing Interact Key is not exposed|current modal has no editor for (?:the stored interact key|it)|current modal does not expose Fishing's stored interact key|Settings modal does not currently expose that Interact Key|modal exposes Arm Timeout, Reel Delay, and Recast Delay, but no interact-key control|there is no supported in-app editor for it|modal does not expose the Fishing Interact Key)\b/iu,
    ],
  ];
  return obsolete
    .filter(([, pattern]) => pattern.test(markdown))
    .map(([label]) => `obsolete ${label}`);
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
  const searchIndexes = outputFiles.filter((file) => /^searchindex-[0-9a-z]+\.js$/u.test(file));
  if (searchIndexes.length === 0) {
    errors.push("missing generated search index");
  } else {
    for (const searchIndex of searchIndexes) {
      const indexContents = await readFile(path.join(outputRoot, searchIndex), "utf8");
      const normalizedIndex = indexContents.toLocaleLowerCase("en-US");
      if (/\bdoc_urls\s*:\s*\[[^\]]*docs\/(?:project|archive)\//iu.test(indexContents)) {
        errors.push("generated search index publishes a project or archive document URL");
      }
      for (const sentinel of NON_PUBLISHED_SEARCH_SENTINELS) {
        if (normalizedIndex.includes(sentinel.toLocaleLowerCase("en-US"))) {
          errors.push(`generated search index contains non-published content sentinel: ${sentinel}`);
        }
      }
    }
  }
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
    errors.push(...validateWorkSliceHtml(contents, relative));
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

function normalizedText(value) {
  return value.normalize("NFKC").toLocaleLowerCase("en-US").replace(/\s+/gu, " ").trim();
}

function hasVisiblePhrase(markdown, phrase) {
  return normalizedText(visibleMarkdown(markdown)).includes(normalizedText(phrase));
}

function validEvidenceRecord(record, expectedKind) {
  if (!record || typeof record !== "object") return false;
  if (record.kind === "TestGap") {
    return expectedKind === "test" && typeof record.claim === "string" && record.claim.trim().length >= 20 &&
      Number.isInteger(record.issue) && record.issue > 0;
  }
  const allowed = expectedKind === "test" ? new Set(["Test", "Workflow"]) : new Set(["Source", "Protocol", "Packaging", "Workflow"]);
  return allowed.has(record.kind) && typeof record.path === "string" && record.path.trim() !== "" &&
    typeof record.anchor === "string" && record.anchor.trim() !== "" &&
    typeof record.claim === "string" && record.claim.trim().length >= 12;
}

export function contentContractDigest(manifest) {
  const projection = {
    evidence: Object.fromEntries(Object.entries(manifest?.evidence ?? {})
      .sort(([left], [right]) => left.localeCompare(right, "en-US"))
      .map(([id, record]) => [id, {
        kind: record.kind,
        path: record.path ?? null,
        anchor: record.anchor ?? null,
        claim: record.claim,
        issue: record.issue ?? null,
      }])),
    obligations: (manifest?.obligations ?? []).map((row) => ({
      id: row.id,
      area: row.area,
      audience: row.audience,
      statement: row.statement,
      destination: row.destination,
      source_evidence: row.source_evidence,
      test_evidence: row.test_evidence,
      coverage: row.coverage,
      labels: row.labels,
      content_anchors: row.content_anchors,
      follow_up: row.follow_up ?? null,
    })),
    pages: (manifest?.pages ?? []).map((page) => ({
      path: page.path,
      title: page.title,
      page_type: page.page_type,
      audiences: page.audiences,
      required_anchors: page.required_anchors,
      related_pages: page.related_pages,
    })),
    search_map: (manifest?.search_map ?? []).map((entry) => ({
      canonical: entry.canonical,
      aliases: entry.aliases,
      target: entry.target,
    })),
    diagrams: (manifest?.diagrams ?? []).map((diagram) => ({
      id: diagram.id,
      kind: diagram.kind,
      destination: diagram.destination,
      title: diagram.title,
      relationships: diagram.relationships,
      alt_text: diagram.alt_text,
      text_equivalent: diagram.text_equivalent,
      content_anchors: diagram.content_anchors,
    })),
  };
  return createHash("sha256").update(JSON.stringify(projection), "utf8").digest("hex");
}

function glossaryExplainsAlias(markdown, alias, target) {
  const formalEntries = markdown.split(/^###\s+/gmu).slice(1);
  const blocks = formalEntries.length > 0 ? formalEntries : markdown.split(/\n\s*\n/gu);
  return blocks.some((block) => {
    if (/^ {0,3}(?:`{3,}|~{3,})/mu.test(block)) return false;
    const prose = block.replace(/<!--[\s\S]*?-->/gu, "").replace(/`+/gu, "");
    return normalizedText(prose).includes(normalizedText(alias)) &&
      hasMarkdownLinkTo(block, "docs/src/reference/glossary.md", target);
  });
}

function sameBytes(left, right) {
  if (!(left instanceof Uint8Array) || !(right instanceof Uint8Array) || left.byteLength !== right.byteLength) return false;
  return left.every((value, index) => value === right[index]);
}

export function validateLandingIdentity({ landingMarkdown, cargoToml, changelog, approvedBanner, publishedBanner }) {
  const errors = [];
  const packageName = tomlStringValue(cargoToml, "package", "name");
  const packageVersion = tomlStringValue(cargoToml, "package", "version");
  const repository = tomlStringValue(cargoToml, "package", "repository");

  if (!packageName) errors.push("S080 cannot resolve the package handle authority");
  if (!packageVersion) errors.push("S080 cannot resolve the package version authority");
  if (!repository) errors.push("S080 cannot resolve the package repository authority");

  const releaseMatch = packageVersion
    ? changelog.match(new RegExp(`^## \\[${escapeRegExp(packageVersion)}\\] - (\\d{4}-\\d{2}-\\d{2})\\s*$`, "mu"))
    : null;
  const releaseDate = releaseMatch?.[1];
  if (!releaseDate) errors.push("S080 cannot resolve the matching changelog release date authority");

  const bannerPattern = /<p\s+class=["']landing-wordmark["']>\s*<img\s+src=["']assets\/brand\/eso-weave-banner\.png["']\s+alt=["']["']\s*\/?>\s*<\/p>/iu;
  if (!bannerPattern.test(landingMarkdown)) errors.push("S080 landing identity requires the local decorative full-color banner");
  const headings = [...landingMarkdown.matchAll(/^#\s+(.+?)\s*$/gmu)];
  if (headings.length !== 1 || !/^<span class=["']visually-hidden["']>ESO Weave<\/span>\s+Documentation$/u.test(headings[0]?.[1] ?? "")) {
    errors.push("S080 landing heading requires visually hidden ESO Weave and visible Documentation text");
  }

  const metadataPattern = /<dl\s+class=["']project-metadata["']\s+aria-label=["']Documentation snapshot["']>[\s\S]*?<\/dl>/iu;
  const metadata = landingMarkdown.match(metadataPattern)?.[0] ?? "";
  if (!metadata) errors.push("S080 landing page requires semantic Documentation snapshot metadata");
  const fieldPresent = (term, valuePattern) => new RegExp(`<dt>${term}<\\/dt>\\s*<dd>${valuePattern}<\\/dd>`, "u").test(metadata);
  if (packageName && !fieldPresent("Handle", `<code>${escapeRegExp(packageName)}<\\/code>`)) {
    errors.push("S080 landing metadata handle does not match Cargo.toml");
  }
  if (packageVersion && !fieldPresent("Applies to", `v${escapeRegExp(packageVersion)}`)) {
    errors.push("S080 landing metadata Applies to version does not match Cargo.toml");
  }
  if (releaseDate && !fieldPresent("Released", `<time datetime=["']${releaseDate}["']>${releaseDate}<\\/time>`)) {
    errors.push("S080 landing metadata release date does not match CHANGELOG.md");
  }
  if (repository) {
    const repositoryLabel = repository.replace(/^https?:\/\//u, "");
    const repositoryValue = `<a href=["']${escapeRegExp(repository)}["']>${escapeRegExp(repositoryLabel)}<\\/a>`;
    if (!fieldPresent("Repository", repositoryValue)) {
      errors.push("S080 landing metadata repository link does not match Cargo.toml");
    }
  }

  if (!hasVisiblePhrase(landingMarkdown, "build-time documentation snapshot") ||
      !landingMarkdown.includes("`Cargo.toml`") || !landingMarkdown.includes("`CHANGELOG.md`")) {
    errors.push("S080 landing page must disclose the build-time snapshot and its update authorities");
  }
  const bannerIndex = landingMarkdown.search(bannerPattern);
  const headingIndex = landingMarkdown.search(/^#\s+/mu);
  const metadataIndex = landingMarkdown.search(metadataPattern);
  const introIndex = landingMarkdown.indexOf("ESO Weave is an offline-first desktop companion");
  if (!(bannerIndex >= 0 && headingIndex > bannerIndex && metadataIndex > headingIndex && introIndex > metadataIndex)) {
    errors.push("S080 landing identity and metadata must precede the introductory prose in order");
  }
  if (!sameBytes(approvedBanner, publishedBanner)) {
    errors.push("S080 published banner bytes must match the approved source asset");
  }
  return errors;
}

export function validateLandingGenerated(indexHtml, outputPaths) {
  const errors = [];
  if (!/<p\s+class=["']landing-wordmark["']>[\s\S]*?<img\s+src=["']assets\/brand\/eso-weave-banner\.png["']\s+alt=["']["']/iu.test(indexHtml)) {
    errors.push("S080 generated landing identity is missing the local decorative banner");
  }
  if (!/<h1[^>]*>[\s\S]*?<span\s+class=["']visually-hidden["']>ESO Weave<\/span>\s+Documentation[\s\S]*?<\/h1>/iu.test(indexHtml)) {
    errors.push("S080 generated landing heading lost its accessible identity");
  }
  if (!/<dl\s+class=["']project-metadata["']\s+aria-label=["']Documentation snapshot["']>/iu.test(indexHtml)) {
    errors.push("S080 generated landing metadata lost its semantic definition list");
  }
  if (!hasVisiblePhrase(indexHtml, "build-time documentation snapshot")) {
    errors.push("S080 generated landing page lost its snapshot disclosure");
  }
  if (!(outputPaths instanceof Set) || !outputPaths.has("assets/brand/eso-weave-banner.png")) {
    errors.push("S080 generated banner output is missing");
  }
  return errors;
}

export function validateLandingCss(css) {
  const errors = [];
  const wordmark = css.match(/\.landing-wordmark\s+img\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  if (!/display:\s*block/iu.test(wordmark) || !/height:\s*auto/iu.test(wordmark) ||
      !/max-width:\s*min\(100%,\s*38rem\)/iu.test(wordmark) || !/width:\s*100%/iu.test(wordmark)) {
    errors.push("S080 wordmark CSS must bound the responsive banner");
  }
  const hiddenRules = [...css.matchAll(/(?<selectors>[^{}]+)\{(?<body>[^{}]*)\}/gu)]
    .filter((match) => match.groups.selectors.split(",").some((selector) => selector.trim() === ".visually-hidden"))
    .map((match) => match.groups.body);
  const hidden = hiddenRules.join("\n");
  if (!/clip-path:\s*inset\(50%\)/iu.test(hidden) || !/height:\s*1px/iu.test(hidden) ||
      !/overflow:\s*hidden/iu.test(hidden) || !/position:\s*absolute/iu.test(hidden) ||
      !/white-space:\s*nowrap/iu.test(hidden) || !/width:\s*1px/iu.test(hidden)) {
    errors.push("S080 visually hidden CSS must preserve accessible text");
  }
  if (hiddenRules.some((rule) => /\b(?:display:\s*none|visibility:\s*hidden)\b/iu.test(rule))) {
    errors.push("S080 visually hidden CSS must not remove text from assistive technology");
  }
  if (!/\.project-metadata\s*\{[\s\S]*?display:\s*grid/iu.test(css)) {
    errors.push("S080 metadata CSS requires a grid layout");
  }
  const labels = css.match(/\.project-metadata\s+dt\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  if (!/color:\s*var\(--eso-muted\)/iu.test(labels)) {
    errors.push("S080 metadata labels must use the theme-aware muted color");
  }
  const root = css.match(/:root\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  const light = css.match(/\.light,[\s\S]*?\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  const property = (block, name) =>
    block.match(new RegExp(`--${name}:\\s*(#[0-9a-f]{6})`, "iu"))?.[1];
  for (const [label, foreground, background] of [
    ["dark", property(root, "eso-muted"), property(root, "eso-panel")],
    ["light", property(light, "eso-muted"), property(light, "sidebar-bg")],
  ]) {
    if (!foreground || !background) {
      errors.push(`S080 cannot resolve ${label} metadata label contrast colors`);
    } else if (contrastRatio(foreground, background) < 4.5) {
      errors.push(`S080 ${label} metadata label contrast is below 4.5:1`);
    }
  }
  if (!/@media\s*\(max-width:\s*40rem\)[\s\S]*?\.project-metadata\s*\{[\s\S]*?grid-template-columns:\s*1fr/iu.test(css)) {
    errors.push("S080 metadata CSS requires a one-column narrow layout");
  }
  return errors;
}

const BRAND_STANDARD_TOKENS = [
  ["Ink base", "#0E1116"], ["Panel", "#151B23"], ["Elevated", "#1C2530"],
  ["Stroke", "#2A3340"], ["Gold (action)", "#F2B03C"], ["Gold hover", "#FBCB6B"],
  ["Gold deep", "#D18F22"], ["Teal (support)", "#2DD4BF"], ["Text", "#E6EDF3"],
  ["Muted", "#8B97A7"], ["Status ok", "#34D399"], ["Status warn", "#FB9E3C"],
  ["Status err", "#F87171"], ["Base", "#F7F5F0"], ["Panel", "#FFFFFF"],
  ["Elevated", "#ECE8DE"], ["Stroke", "#DCD9D0"], ["Gold (action)", "#E7A42C"],
  ["Gold deep", "#C6871F"], ["Teal (support)", "#0D9488"], ["Text", "#14110B"],
  ["Muted", "#6B6455"], ["Status ok", "#059669"], ["Status warn", "#B45309"],
  ["Status err", "#DC2626"],
];

const BRAND_STANDARD_ASSETS = [
  ["banner", "eso-weave-banner.png"],
  ["mark", "eso-weave-mark.svg"],
  ["glyph", "eso-weave-glyph.svg"],
];

export function validateBrandStandard({
  markdown,
  approvedBanner,
  publishedBanner,
  approvedMark,
  publishedMark,
  approvedGlyph,
  publishedGlyph,
}) {
  const errors = [];
  for (const [label, filename] of BRAND_STANDARD_ASSETS) {
    const source = `../assets/brand/${filename}`;
    const imagePattern = new RegExp(`<img\\b(?=[^>]*\\bsrc=["']${escapeRegExp(source)}["'])(?=[^>]*\\balt=["'][^"']+["'])[^>]*>`, "iu");
    if (!imagePattern.test(markdown)) errors.push(`S081 Brand Standard requires the local approved ${label} image`);
    const linkPattern = new RegExp(`\\[[^\\]]+\\]\\(${escapeRegExp(source)}\\)`, "iu");
    if (!linkPattern.test(markdown)) errors.push(`S081 Brand Standard requires a local ${label} download link`);
  }

  if (!/<div\s+class=["']brand-asset-gallery["']>/iu.test(markdown)) errors.push("S081 Brand Standard requires the approved asset gallery");
  const darkSurface = markdown.match(/<figure\s+class=["']brand-surface brand-surface--dark["']\s+aria-label=["']Dark ink surface["']>[\s\S]*?<\/figure>/iu)?.[0] ?? "";
  const lightSurface = markdown.match(/<figure\s+class=["']brand-surface brand-surface--light["']\s+aria-label=["']Light surface["']>[\s\S]*?<\/figure>/iu)?.[0] ?? "";
  if (!darkSurface || !lightSurface) errors.push("S081 Brand Standard requires labeled dark and light surface examples");
  if (!darkSurface.includes("eso-weave-glyph.svg")) errors.push("S081 glyph requires a dark ink surface example");
  if (lightSurface.includes("eso-weave-glyph.svg")) errors.push("S081 glyph must not appear on the light surface");

  for (const phrase of ["Full-color banner", "Badged mark", "Badge-less glyph", "generated compatibility outputs, not masters", "Preserve aspect ratio", "clear space", "Minimum sizes", "Do not recolor"]) {
    if (!markdown.toLowerCase().includes(phrase.toLowerCase())) errors.push(`S081 Brand Standard is missing guidance: ${phrase}`);
  }
  for (const filename of ["eso-weave-logo-clear.png", "eso-weave-logo-white.png"]) {
    if (!markdown.includes(`\`${filename}\``)) errors.push(`S081 compatibility status must name ${filename}`);
  }

  for (const [role, hex] of BRAND_STANDARD_TOKENS) {
    const expected = `<span class="brand-swatch" role="img" aria-label="${role}, ${hex} color swatch" style="--swatch-color: ${hex}"></span> \`${hex}\``;
    if (!markdown.includes(`| ${role} | ${expected} |`)) {
      errors.push(`S081 palette token ${role} ${hex} requires the exact fill and accessible chip label`);
    }
  }
  const chipCount = [...markdown.matchAll(/class=["']brand-swatch["']/giu)].length;
  if (chipCount !== BRAND_STANDARD_TOKENS.length) errors.push(`S081 palette requires exactly ${BRAND_STANDARD_TOKENS.length} chips`);

  for (const [label, approved, published] of [
    ["banner", approvedBanner, publishedBanner],
    ["mark", approvedMark, publishedMark],
    ["glyph", approvedGlyph, publishedGlyph],
  ]) {
    if (!sameBytes(approved, published)) errors.push(`S081 published ${label} bytes must match the approved source asset`);
  }
  return errors;
}

export function validateBrandStandardGenerated(html, outputPaths) {
  const errors = [];
  if (!/class=["']brand-asset-gallery["']/iu.test(html)) errors.push("S081 generated Brand Standard is missing the approved asset gallery");
  if (!/aria-label=["']Dark ink surface["']/iu.test(html) || !/aria-label=["']Light surface["']/iu.test(html)) {
    errors.push("S081 generated Brand Standard is missing labeled surface examples");
  }
  for (const [role, hex] of BRAND_STANDARD_TOKENS) {
    const label = `aria-label="${role}, ${hex} color swatch"`;
    if (!html.includes(label)) errors.push(`S081 generated Brand Standard lost the ${role} ${hex} accessible swatch`);
  }
  for (const [, filename] of BRAND_STANDARD_ASSETS) {
    if (!(outputPaths instanceof Set) || !outputPaths.has(`assets/brand/${filename}`)) {
      errors.push(`S081 generated asset output is missing: ${filename}`);
    }
  }
  return errors;
}

export function validateBrandStandardVisualCss(css) {
  const errors = [];
  const gallery = css.match(/\.brand-asset-gallery\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  if (!/display:\s*grid/iu.test(gallery) || !/grid-template-columns:\s*repeat\(2,\s*minmax\(0,\s*1fr\)\)/iu.test(gallery)) {
    errors.push("S081 asset gallery CSS requires a bounded two-column grid");
  }
  const surface = css.match(/\.brand-surface\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  if (!/border:\s*1px\s+solid/iu.test(surface)) errors.push("S081 asset surfaces require a visible boundary");
  if (!/\.brand-surface--dark\s*\{[\s\S]*?background:\s*#0e1116/iu.test(css) ||
      !/\.brand-surface--light\s*\{[\s\S]*?background:\s*#f7f5f0/iu.test(css)) {
    errors.push("S081 asset gallery requires explicit dark and light backgrounds");
  }
  const image = css.match(/\.brand-surface\s+img\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  if (!/display:\s*block/iu.test(image) || !/height:\s*auto/iu.test(image) || !/max-width:\s*100%/iu.test(image)) {
    errors.push("S081 asset image CSS must preserve aspect ratio and card containment");
  }
  const swatch = css.match(/\.brand-swatch\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  if (!/background:\s*var\(--swatch-color\)/iu.test(swatch) || !/display:\s*inline-block/iu.test(swatch) ||
      !/height:\s*1\.1rem/iu.test(swatch) || !/width:\s*1\.1rem/iu.test(swatch)) {
    errors.push("S081 swatch CSS requires a fixed visible chip using the authored token");
  }
  if (!/border:\s*1px\s+solid/iu.test(swatch) || !/box-shadow:\s*inset/iu.test(swatch)) {
    errors.push("S081 swatch boundary must remain visible across fills and themes");
  }
  if (!/\.brand-surface--light\s+\.brand-asset--mark\s*\{[\s\S]*?grid-column:\s*1\s*\/\s*-1/iu.test(css)) {
    errors.push("S081 light-surface mark must span the grid to remain centered");
  }
  if (!/@media\s*\(max-width:\s*40rem\)[\s\S]*?\.brand-asset-gallery\s*\{[\s\S]*?grid-template-columns:\s*1fr/iu.test(css)) {
    errors.push("S081 asset gallery CSS requires a one-column narrow layout");
  }
  return errors;
}

const DOCUMENTATION_DIAGRAMS = [
  {
    id: "S082-D01",
    label: "architecture ownership",
    page: "development/architecture.md",
    outputPage: "development/architecture.html",
    asset: "architecture-ownership.svg",
    width: 400,
    height: 650,
    alt: "Architecture ownership flow keeps physical input and observed game evidence separate until named consumers",
    heading: "Ownership flow text equivalent",
    anchors: [
      "Physical input remains on the input path",
      "Observed game evidence remains on the observation path",
      "Named engines and controllers consume only their owned inputs",
    ],
    svgAnchors: ["Physical input", "Observed evidence", "Named consumers"],
  },
  {
    id: "S082-D02",
    label: "action authorization",
    page: "concepts/action-authorization.md",
    outputPage: "concepts/action-authorization.html",
    asset: "action-authorization.svg",
    width: 400,
    height: 700,
    alt: "Action authorization flow requires every positive gate or fails closed without generated input",
    heading: "Authorization flow text equivalent",
    anchors: [
      "A physical event first reaches the focus-scoped decision",
      "Every generated action requires positive current evidence",
      "Unsafe or unavailable evidence fails closed",
    ],
    svgAnchors: ["Physical event", "Positive gates", "Authorized", "Fails closed"],
  },
  {
    id: "S082-D03",
    label: "safety recovery",
    page: "development/state-machines.md",
    outputPage: "development/state-machines.html",
    asset: "safety-recovery.svg",
    width: 400,
    height: 690,
    alt: "Safety recovery flow closes gates before synchronization and reopens only after a coherent baseline",
    heading: "Safety recovery text equivalent",
    anchors: [
      "Unsafe or unavailable evidence closes shared gates first",
      "Consumers synchronize while authorization remains closed",
      "A complete positive baseline reopens the gates",
    ],
    svgAnchors: ["Unsafe evidence", "Close gates", "Synchronize", "Republish baseline", "Reopen"],
  },
  {
    id: "S082-D04",
    label: "Pixel Bus validation",
    page: "reference/pixel-bus-protocol.md",
    outputPage: "reference/pixel-bus-protocol.html",
    asset: "pixel-bus-validation.svg",
    width: 400,
    height: 820,
    alt: "Pixel Bus validation flow rejects invalid headers and layouts before independently decoding and publishing payload signals",
    heading: "Pixel Bus validation text equivalent",
    anchors: [
      "Capture the header from one displayed frame",
      "Header or layout corruption suppresses all payload sampling",
      "Each payload block then validates independently",
    ],
    svgAnchors: ["Capture one frame", "Validate header", "Require B0 heartbeat", "Decode blocks independently", "Signal-specific", "unavailable or hold", "Route consumers"],
  },
];

export function validateDocumentationDiagrams({ pages, svgs }) {
  const errors = [];
  if (!(pages instanceof Map) || !(svgs instanceof Map)) return ["S082 diagram validation requires page and SVG maps"];
  for (const record of DOCUMENTATION_DIAGRAMS) {
    const markdown = pages.get(record.page) ?? "";
    const source = `../assets/diagrams/${record.asset}`;
    const reference = `![${record.alt}](${source})`;
    if (!markdown.includes(source)) errors.push(`S082 ${record.label} page requires its local diagram reference`);
    if (!markdown.includes(reference)) errors.push(`S082 ${record.label} diagram requires the exact meaningful alternative`);
    if (!new RegExp(`<figure\\s+class=["']docs-flow-diagram["']>[\\s\\S]*?${escapeRegExp(reference)}[\\s\\S]*?<\\/figure>`, "iu").test(markdown)) {
      errors.push(`S082 ${record.label} diagram requires the responsive figure wrapper`);
    }
    if (!markdown.includes(`### ${record.heading}`)) errors.push(`S082 ${record.label} page requires its text equivalent heading`);
    for (const anchor of record.anchors) {
      if (!hasVisiblePhrase(markdown, anchor)) errors.push(`S082 ${record.label} text equivalent is missing: ${anchor}`);
    }

    const svg = svgs.get(record.asset) ?? "";
    if (!svg) {
      errors.push(`S082 ${record.label} SVG source is missing`);
      continue;
    }
    if (!/<svg\b(?=[^>]*\bxmlns=["']http:\/\/www\.w3\.org\/2000\/svg["'])(?=[^>]*\bviewBox=["']0 0 \d+ \d+["'])(?=[^>]*\brole=["']img["'])(?=[^>]*\bfocusable=["']false["'])(?=[^>]*\baria-labelledby=["'][^"']+-title [^"']+-desc["'])(?=[^>]*\bdata-flow-direction=["']top-down["'])[^>]*>/iu.test(svg)) {
      errors.push(`S082 ${record.label} SVG requires a top-down accessible root`);
    }
    const root = svg.match(/<svg\b[^>]*>/iu)?.[0] ?? "";
    const width = Number(root.match(/\swidth=["'](\d+(?:\.\d+)?)["']/iu)?.[1]);
    const height = Number(root.match(/\sheight=["'](\d+(?:\.\d+)?)["']/iu)?.[1]);
    const viewBox = root.match(/\bviewBox=["']0 0 (\d+(?:\.\d+)?) (\d+(?:\.\d+)?)["']/iu)?.slice(1).map(Number) ?? [];
    if (width !== record.width || height !== record.height || viewBox[0] !== record.width || viewBox[1] !== record.height) {
      errors.push(`S086 ${record.label} SVG intrinsic geometry must exactly match its viewBox`);
    }
    const titleId = svg.match(/<title\s+id=["']([^"']+-title)["']>/iu)?.[1];
    const descId = svg.match(/<desc\s+id=["']([^"']+-desc)["']>/iu)?.[1];
    const labelled = svg.match(/aria-labelledby=["']([^"']+)["']/iu)?.[1]?.split(/\s+/u) ?? [];
    if (!titleId || !descId || !labelled.includes(titleId) || !labelled.includes(descId)) {
      errors.push(`S082 ${record.label} SVG title and description must match aria-labelledby`);
    }
    const hasNonFragmentCssUrl = [...svg.matchAll(/\burl\s*\(\s*([^)]+?)\s*\)/giu)].some((match) => {
      let target = match[1].trim();
      if ((target.startsWith('"') && target.endsWith('"')) || (target.startsWith("'") && target.endsWith("'"))) {
        target = target.slice(1, -1).trim();
      }
      return !target.startsWith("#");
    });
    const hasCssImport = /@import\b/iu.test(svg);
    if (hasNonFragmentCssUrl || hasCssImport || /<(?:script|foreignObject|animate|set|image|iframe)\b|\bon[a-z]+\s*=|\b(?:href|xlink:href)\s*=\s*["'](?!#)|https?:\/\//iu.test(svg.replace('xmlns="http://www.w3.org/2000/svg"', ""))) {
      errors.push(`S082 ${record.label} SVG contains active or external content`);
    }
    if (!new RegExp(`<rect\\b(?=[^>]*\\bwidth=["']${record.width}["'])(?=[^>]*\\bheight=["']${record.height}["'])(?=[^>]*\\bfill=["']#0e1116["'])[^>]*>`, "iu").test(svg)) {
      errors.push(`S082 ${record.label} SVG requires the opaque ink canvas`);
    }
    const paintedColors = new Set([...svg.matchAll(/\b(?:fill|stroke)=["'](#[0-9a-f]{6})["']/giu)].map((match) => match[1].toLowerCase()));
    if (paintedColors.size < 4 || !/<path\b/iu.test(svg)) {
      errors.push(`S086 ${record.label} SVG requires multicolor painted labels and connectors`);
    }
    for (const size of [...svg.matchAll(/font-size=["'](\d+(?:\.\d+)?)["']/giu)].map((match) => Number(match[1]))) {
      if (size < 14) errors.push(`S082 ${record.label} SVG label size must be at least 14`);
    }
    if (![...svg.matchAll(/<text\b/giu)].length) errors.push(`S082 ${record.label} SVG requires visible labels`);
    for (const anchor of record.svgAnchors) {
      if (!svg.includes(anchor)) errors.push(`S082 ${record.label} SVG requires the ${anchor} label`);
    }
  }
  return [...new Set(errors)];
}

export function validateDocumentationDiagramsGenerated(pages, outputPaths, sourceSvgs, generatedSvgs, script = "") {
  const errors = [];
  if (!(pages instanceof Map)) return ["S082 generated diagram validation requires a page map"];
  for (const record of DOCUMENTATION_DIAGRAMS) {
    const html = pages.get(record.outputPage) ?? "";
    if (!html.includes('class="docs-flow-diagram"') || !html.includes(`src="../assets/diagrams/${record.asset}"`) || !html.includes(`alt="${record.alt}"`)) {
      errors.push(`S082 generated ${record.label} page lost its local diagram or meaningful alternative`);
    }
    const figure = html.match(/<figure\s+class=["']docs-flow-diagram["']>[\s\S]*?<\/figure>/iu)?.[0] ?? "";
    const assetReferences = [...figure.matchAll(new RegExp(`src=["']\\.\\.\/assets\/diagrams\/${escapeRegExp(record.asset)}["']`, "giu"))].length;
    if (!figure.includes('class="checkbox-label"') || !figure.includes('class="checkbox-img"') ||
        !figure.includes('class="img-wrapper"') || assetReferences !== 2) {
      errors.push(`S086 generated ${record.label} page requires the expected zoom DOM`);
    }
    if (!html.includes(record.heading)) errors.push(`S082 generated ${record.label} page lost its text equivalent`);
    if (!(outputPaths instanceof Set) || !outputPaths.has(`assets/diagrams/${record.asset}`)) {
      errors.push(`S082 generated asset is missing: ${record.asset}`);
    }
    if (!(sourceSvgs instanceof Map) || !(generatedSvgs instanceof Map) || sourceSvgs.get(record.asset) !== generatedSvgs.get(record.asset)) {
      errors.push(`S086 generated ${record.label} SVG must be byte-identical to source`);
    }
    if (/\b(?:mermaid|https?:\/\/)/iu.test(html.match(/<figure\s+class=["']docs-flow-diagram["']>[\s\S]*?<\/figure>/iu)?.[0] ?? "")) {
      errors.push(`S082 generated ${record.label} diagram must not require a remote renderer`);
    }
  }
  if (!/querySelectorAll\(["']\.docs-flow-diagram \.img-wrapper > img["']\)/u.test(script) ||
      !/(?:\.alt\s*=\s*["']["']|setAttribute\(["']alt["'],\s*["']["']\))/u.test(script) ||
      !/setAttribute\(["']aria-hidden["'],\s*["']true["']\)/u.test(script)) {
    errors.push("S086 generated expanded diagram clones must become decorative");
  }
  return [...new Set(errors)];
}

export function validateDocumentationDiagramCss(css) {
  const errors = [];
  const figure = css.match(/\.docs-flow-diagram\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  const image = css.match(/\.docs-flow-diagram\s+img\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  const primary = css.match(/\.docs-flow-diagram\s+\.checkbox-img\s*\+\s*img\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  const expanded = css.match(/\.docs-flow-diagram\s+\.img-wrapper\s*>\s*img\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  if (!/border:\s*1px\s+solid/iu.test(figure) || !/max-width:\s*40rem/iu.test(figure) || !/width:\s*100%/iu.test(figure)) {
    errors.push("S082 diagram wrapper requires a bounded visible frame");
  }
  if (!/overflow:\s*hidden/iu.test(figure)) errors.push("S082 diagram wrapper must contain overflow");
  if (!/display:\s*block/iu.test(image) || !/height:\s*auto/iu.test(image) || !/max-width:\s*100%/iu.test(image)) {
    errors.push("S082 diagram image must preserve aspect ratio and content containment");
  }
  if (!/width:\s*100%/iu.test(primary)) errors.push("S086 primary diagram image must fill its bounded wrapper");
  if (!/height:\s*auto/iu.test(expanded) || !/width:\s*auto/iu.test(expanded) ||
      !/max-height:\s*100vh/iu.test(expanded) || !/max-width:\s*100vw/iu.test(expanded)) {
    errors.push("S086 expanded diagram image must preserve intrinsic sizing and viewport containment");
  }
  if (!/@media\s*\(max-width:\s*40rem\)[\s\S]*?\.docs-flow-diagram\s*\{[\s\S]*?max-width:\s*100%/iu.test(css)) {
    errors.push("S082 diagram CSS requires a full-width narrow layout");
  }
  return errors;
}

const DOCUMENTATION_SCREENSHOT_IDS = new Set([
  "windows-msi-properties-unblock",
  "first-launch",
  "healthy-system-state",
  "pixelbeacon-lost",
  "pixelbeacon-unmanaged",
  "weaving-configuration",
  "auto-potion-ready",
  "auto-potion-blocked",
  "pixelbeacon-overlay-example",
]);

const DETERMINISTIC_SCREENSHOT_SOURCES = new Map([
  ["first-launch", ["first-launch", "first-launch--dark--wide.png"]],
  ["healthy-system-state", ["healthy-system-state", "healthy-system-state--dark--wide.png"]],
  ["pixelbeacon-lost", ["pixelbeacon-lost", "pixelbeacon-lost--dark--wide.png"]],
  ["pixelbeacon-unmanaged", ["pixelbeacon-unmanaged", "pixelbeacon-unmanaged--dark--wide.png"]],
  ["weaving-configuration", ["weaving-configuration", "weaving-configuration--dark--wide.png"]],
  ["auto-potion-ready", ["auto-potion-ready", "auto-potion-ready--dark--wide.png"]],
  ["auto-potion-blocked", ["auto-potion-blocked", "auto-potion-blocked--dark--wide.png"]],
]);

function pngCrc32(bytes, start, end) {
  let crc = 0xffffffff;
  for (let index = start; index < end; index += 1) {
    crc ^= bytes[index];
    for (let bit = 0; bit < 8; bit += 1) {
      crc = (crc >>> 1) ^ (0xedb88320 & -(crc & 1));
    }
  }
  return (crc ^ 0xffffffff) >>> 0;
}

function pngDimensions(bytes) {
  const signature = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
  if (bytes.length < 45 || signature.some((value, index) => bytes[index] !== value)) return null;
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  let offset = signature.length;
  let dimensions = null;
  let sawImageData = false;
  let sawEnd = false;
  let chunkIndex = 0;
  while (offset < bytes.length) {
    if (bytes.length - offset < 12) return null;
    const length = view.getUint32(offset);
    const typeOffset = offset + 4;
    const dataOffset = typeOffset + 4;
    const crcOffset = dataOffset + length;
    const nextOffset = crcOffset + 4;
    if (nextOffset > bytes.length) return null;
    const type = String.fromCharCode(...bytes.slice(typeOffset, dataOffset));
    if (!/^[A-Za-z]{4}$/u.test(type)
        || view.getUint32(crcOffset) !== pngCrc32(bytes, typeOffset, crcOffset)) return null;
    if (chunkIndex === 0 && (type !== "IHDR" || length !== 13)) return null;
    if (type === "IHDR") {
      if (dimensions || length !== 13) return null;
      dimensions = [view.getUint32(dataOffset), view.getUint32(dataOffset + 4)];
      if (dimensions[0] === 0 || dimensions[1] === 0) return null;
    } else if (type === "IDAT") {
      sawImageData = true;
    } else if (type === "IEND") {
      if (length !== 0 || nextOffset !== bytes.length) return null;
      sawEnd = true;
    }
    offset = nextOffset;
    chunkIndex += 1;
    if (sawEnd) break;
  }
  return dimensions && sawImageData && sawEnd ? dimensions : null;
}

function screenshotFigureBlocks(content) {
  return content.match(/<figure\b[^>]*\bclass=["'][^"']*\bdocs-screenshot\b[^"']*["'][^>]*>[\s\S]*?<\/figure>/giu) ?? [];
}

export function validateDocumentationScreenshots({ manifest, assets, pages }) {
  const errors = [];
  if (!manifest || !Array.isArray(manifest.assets) || !(assets instanceof Map) || !(pages instanceof Map)) {
    return ["S084 screenshot validation requires a manifest, asset map, and page map"];
  }
  if (manifest.schema_version !== 1 || manifest.generator !== "S084 guided documentation screenshots") {
    errors.push("S084 screenshot manifest requires the supported schema and generator");
  }
  if (manifest.assets.length !== DOCUMENTATION_SCREENSHOT_IDS.size) {
    errors.push(`S084 screenshot manifest requires exactly ${DOCUMENTATION_SCREENSHOT_IDS.size} assets`);
  }

  const ids = new Set();
  const destinations = new Set();
  const kinds = new Map();
  for (const record of manifest.assets) {
    if (!DOCUMENTATION_SCREENSHOT_IDS.has(record.id) || ids.has(record.id)) {
      errors.push(`S084 screenshot has an unknown or duplicate id: ${record.id ?? "missing"}`);
    }
    ids.add(record.id);
    if (!/^docs\/src\/assets\/(?:screenshots\/[a-z0-9-]+\.png|illustrations\/[a-z0-9-]+\.svg)$/u.test(record.destination ?? "") || destinations.has(record.destination)) {
      errors.push(`S084 screenshot has an invalid or duplicate destination: ${record.destination ?? "missing"}`);
    }
    destinations.add(record.destination);
    kinds.set(record.kind, (kinds.get(record.kind) ?? 0) + 1);
    for (const field of ["reader_question", "alt", "caption", "update_trigger"]) {
      if (typeof record[field] !== "string" || !record[field].trim()) errors.push(`S084 ${record.id} requires ${field}`);
    }
    if (!Array.isArray(record.pages) || record.pages.length === 0) {
      errors.push(`S084 ${record.id} requires at least one published page`);
    }

    const bytes = assets.get(record.destination);
    if (!(bytes instanceof Uint8Array) || bytes.length === 0) {
      errors.push(`S084 screenshot asset is missing: ${record.destination}`);
      continue;
    }
    if (record.bytes !== bytes.length) errors.push(`S084 ${record.id} byte size does not match its asset`);
    const digest = createHash("sha256").update(bytes).digest("hex");
    if (record.sha256 !== digest) errors.push(`S084 ${record.id} digest does not match its asset`);

    if (record.destination.endsWith(".png")) {
      const dimensions = pngDimensions(bytes);
      if (!dimensions) errors.push(`S084 ${record.id} is not a complete valid PNG`);
      else if (dimensions[0] !== record.width || dimensions[1] !== record.height) {
        errors.push(`S084 ${record.id} dimensions do not match its PNG`);
      }
    } else {
      const svg = new TextDecoder("utf8", { fatal: true }).decode(bytes);
      if (!/<svg\b(?=[^>]*\brole=["']img["'])(?=[^>]*\baria-labelledby=["'][^"']+["'])[^>]*>/iu.test(svg)
          || !/<title\s+id=["'][^"']+["']>/iu.test(svg)
          || !/<desc\s+id=["'][^"']+["']>/iu.test(svg)
          || !svg.includes("SYNTHETIC EXAMPLE")) {
        errors.push(`S084 ${record.id} requires an accessible visible synthetic label`);
      }
      if (/<(?:script|foreignObject|image|iframe)\b|\bon[a-z]+\s*=|\b(?:href|xlink:href)\s*=/iu.test(svg)) {
        errors.push(`S084 ${record.id} synthetic SVG contains active or external content`);
      }
    }

    if (record.kind === "deterministic-app") {
      const source = record.source ?? {};
      const crop = source.crop;
      const expectedSource = DETERMINISTIC_SCREENSHOT_SOURCES.get(record.id);
      if (!expectedSource || source.scene !== expectedSource[0] || source.file !== expectedSource[1]
          || source.generator !== "S083 deterministic screenshot sandbox"
          || source.theme !== "dark" || source.viewport !== "wide"
          || source.width !== 1280 || source.height !== 900
          || !/^[a-f0-9]{64}$/u.test(source.sha256 ?? "")
          || !Array.isArray(crop) || crop.length !== 4
          || crop[0] !== 0 || crop[1] !== 0 || crop[2] !== record.width || crop[3] !== record.height) {
        errors.push(`S084 ${record.id} requires complete deterministic source and crop provenance`);
      }
    } else if (record.kind === "maintainer-supplied") {
      if (record.source?.original_sha256 !== record.sha256) {
        errors.push(`S084 ${record.id} must remain byte-identical to the supplied source`);
      }
    } else if (record.kind === "synthetic-illustration") {
      if (record.source?.synthetic !== true) errors.push(`S084 ${record.id} must declare its synthetic source`);
    } else {
      errors.push(`S084 ${record.id} has an unsupported source kind`);
    }

    for (const page of record.pages ?? []) {
      const markdown = pages.get(page) ?? "";
      const relative = path.posix.relative(path.posix.dirname(page), record.destination);
      if (!markdown.includes(`src="${relative}"`) || !markdown.includes(`alt="${record.alt}"`)) {
        errors.push(`S084 ${record.id} page ${page} lost its local image or exact alternative text`);
      }
      if (!markdown.includes(`<figcaption>${record.caption}</figcaption>`)) {
        errors.push(`S084 ${record.id} page ${page} lost its exact adjacent caption`);
      }
      const associatedFigure = screenshotFigureBlocks(markdown).some((figure) => (
        figure.includes(`src="${relative}"`)
        && figure.includes(`alt="${record.alt}"`)
        && figure.includes(`<figcaption>${record.caption}</figcaption>`)
      ));
      if (!associatedFigure) {
        errors.push(`S084 ${record.id} page ${page} requires its own screenshot figure`);
      }
    }
  }

  for (const id of DOCUMENTATION_SCREENSHOT_IDS) {
    if (!ids.has(id)) errors.push(`S084 screenshot manifest is missing ${id}`);
  }
  if (kinds.get("deterministic-app") !== 7 || kinds.get("maintainer-supplied") !== 1 || kinds.get("synthetic-illustration") !== 1) {
    errors.push("S084 screenshot manifest requires seven deterministic, one supplied, and one synthetic asset");
  }
  for (const destination of assets.keys()) {
    if (!destinations.has(destination)) errors.push(`S084 screenshot asset is not inventoried: ${destination}`);
  }
  return [...new Set(errors)];
}

export function validateDocumentationScreenshotsGenerated({ manifest, pages, outputPaths }) {
  const errors = [];
  if (!manifest || !Array.isArray(manifest.assets) || !(pages instanceof Map) || !(outputPaths instanceof Set)) {
    return ["S084 generated screenshot validation requires a manifest, page map, and output paths"];
  }
  for (const record of manifest.assets) {
    const asset = record.destination.replace("docs/src/", "");
    if (!outputPaths.has(asset)) errors.push(`S084 generated asset is missing: ${asset}`);
    for (const page of record.pages) {
      const outputPage = page.replace("docs/src/", "").replace(/\.md$/u, ".html");
      const html = pages.get(outputPage) ?? "";
      const relative = path.posix.relative(path.posix.dirname(page), record.destination);
      const associatedFigure = screenshotFigureBlocks(html).some((figure) => (
        figure.includes(`src="${relative}"`)
        && figure.includes(`alt="${record.alt}"`)
        && figure.includes(`<figcaption>${record.caption}</figcaption>`)
      ));
      if (!associatedFigure) {
        errors.push(`S084 generated page lost ${record.id} figure semantics: ${outputPage}`);
      }
    }
  }
  return [...new Set(errors)];
}

export function validateDocumentationScreenshotCss(css) {
  const errors = [];
  const figure = css.match(/\.docs-screenshot\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  const image = css.match(/\.docs-screenshot\s+img\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  const caption = css.match(/\.docs-screenshot\s+figcaption\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  const grid = css.match(/\.docs-screenshot-grid\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  if (!/max-width:\s*64rem/iu.test(figure) || !/overflow:\s*hidden/iu.test(figure) || !/width:\s*100%/iu.test(figure)) {
    errors.push("S084 screenshot wrapper requires bounded contained width");
  }
  if (!/display:\s*block/iu.test(image) || !/height:\s*auto/iu.test(image) || !/max-width:\s*100%/iu.test(image) || !/width:\s*100%/iu.test(image)) {
    errors.push("S084 screenshot image must preserve aspect ratio and content containment");
  }
  if (!/border-top:\s*1px\s+solid/iu.test(caption)) errors.push("S084 screenshot requires a visibly adjacent caption");
  if (!/display:\s*grid/iu.test(grid) || !/repeat\(2,\s*minmax\(0,\s*1fr\)\)/iu.test(grid)) {
    errors.push("S084 screenshot comparison requires a bounded two-column grid");
  }
  if (!/@media\s*\(max-width:\s*40rem\)[\s\S]*?\.docs-screenshot-grid\s*\{[\s\S]*?grid-template-columns:\s*1fr/iu.test(css)) {
    errors.push("S084 screenshot grid requires one-column narrow reflow");
  }
  return errors;
}

function figureBlocks(content, className) {
  const pattern = new RegExp(`<figure\\b[^>]*\\bclass=["'][^"']*\\b${className}\\b[^"']*["'][^>]*>[\\s\\S]*?<\\/figure>`, "giu");
  return content.match(pattern) ?? [];
}

function rawImages(content) {
  return content.match(/<img\b[^>]*>/giu) ?? [];
}

function markdownImages(content) {
  return [...content.matchAll(/!\[([^\]]*)\]\([^\r\n)]+\)/gu)];
}

export function validateDocumentationFigureInventory(pages) {
  if (!(pages instanceof Map)) return ["S088 figure inventory requires a source page map"];
  const errors = [];
  let allRaw = 0;
  let allMarkdown = 0;
  let screenshots = 0;
  let diagrams = 0;
  let brandImages = 0;
  let screenshotCaptions = 0;
  let brandCaptions = 0;
  let decorativeWordmarks = 0;

  for (const [page, content] of pages) {
    allRaw += rawImages(content).length;
    allMarkdown += markdownImages(content).length;

    for (const block of figureBlocks(content, "docs-screenshot")) {
      const images = rawImages(block);
      screenshots += images.length;
      screenshotCaptions += (block.match(/<figcaption\b/giu) ?? []).length;
      for (const image of images) {
        if (!/\balt=["'][^"']+["']/iu.test(image)) errors.push(`S088 ${page} screenshot requires meaningful alternative text`);
      }
    }

    for (const block of figureBlocks(content, "docs-flow-diagram")) {
      const images = markdownImages(block);
      diagrams += images.length;
      for (const image of images) {
        if (!image[1].trim()) errors.push(`S088 ${page} diagram requires meaningful alternative text`);
      }
    }

    for (const block of figureBlocks(content, "brand-surface")) {
      const images = rawImages(block);
      brandImages += images.length;
      brandCaptions += (block.match(/<figcaption\b/giu) ?? []).length;
      for (const image of images) {
        if (!/\balt=["'][^"']+["']/iu.test(image)) errors.push(`S088 ${page} brand example requires meaningful alternative text`);
      }
    }

    decorativeWordmarks += (content.match(/<p\s+class=["']landing-wordmark["']>\s*<img\b(?=[^>]*\balt=["']["'])[^>]*>\s*<\/p>/giu) ?? []).length;
  }

  const meaningful = screenshots + diagrams + brandImages;
  if (screenshots !== 11 || diagrams !== 4 || brandImages !== 5 || meaningful !== 20) {
    errors.push(`S088 figure inventory requires 20 meaningful placements (11 screenshot or illustration, 4 diagram, 5 brand); found ${meaningful} (${screenshots}, ${diagrams}, ${brandImages})`);
  }
  if (decorativeWordmarks !== 1) errors.push("S088 figure inventory requires exactly one empty-alt decorative wordmark");
  if (screenshotCaptions !== 11 || brandCaptions !== 2) {
    errors.push(`S088 figure inventory requires 13 captions (11 screenshot or illustration, 2 brand); found ${screenshotCaptions + brandCaptions}`);
  }
  const classified = meaningful + decorativeWordmarks;
  if (allRaw + allMarkdown !== classified) {
    errors.push(`S088 figure inventory found ${allRaw + allMarkdown - classified} unclassified image placement(s)`);
  }
  return [...new Set(errors)];
}

export function validateDocumentationFigureJavascript(script) {
  const errors = [];
  if (!/document\.createElement\(["']dialog["']\)/u.test(script)
      || !/data-docs-figure-dialog/u.test(script)
      || !/document\.body\.append\(dialog\)/u.test(script)) {
    errors.push("S088 figure system requires one native dialog appended to the document");
  }
  if (!/dataset\.docsFigureSystem/u.test(script) || !/data-docs-figure-trigger/u.test(script)) {
    errors.push("S088 figure system requires idempotent trigger enhancement");
  }
  for (const selector of [".docs-screenshot > img", ".docs-flow-diagram .checkbox-img + img", ".brand-surface__assets > img"]) {
    if (!script.includes(selector)) errors.push(`S088 figure system requires the ${selector} trigger inventory`);
  }
  if (!/closest\(["']\.docs-flow-diagram["']\)/u.test(script)
      || !/closest\(["']\.checkbox-label["']\)/u.test(script)
      || !/generatedLabel\.replaceWith\(trigger\)/u.test(script)) {
    errors.push("S088 figure system must replace the generated mdBook checkbox modal with the common trigger");
  }
  if (!/dialog\.showModal\(\)/u.test(script) || !/closeButton\.focus\(/u.test(script)) {
    errors.push("S088 figure system requires native modal opening and initial close-control focus");
  }
  if (!/globalThis\.visualViewport/u.test(script)
      || !/--docs-figure-viewport-width/u.test(script)
      || !/--docs-figure-viewport-height/u.test(script)) {
    errors.push("S088 figure dialog requires visual-viewport synchronization for browser zoom");
  }
  if (!/closeButton\.addEventListener\(["']click["']/u.test(script)
      || !/dialog\.addEventListener\(["']click["']/u.test(script)
      || !/event\.target\s*!==\s*dialog/u.test(script)) {
    errors.push("S088 figure dialog requires close-button and backdrop pointer closure");
  }
  if (!/trigger\.addEventListener\(["']keydown["'][\s\S]*event\.key\s*!==\s*["']Enter["'][\s\S]*event\.key\s*!==\s*["'] ["']/u.test(script)
      || !/dialog\.addEventListener\(["']keydown["'][\s\S]*event\.key\s*===\s*["']Tab["'][\s\S]*closeButton\.focus\([\s\S]*event\.key\s*!==\s*["']Escape["']/u.test(script)) {
    errors.push("S088 figure system requires explicit Enter, Space, Tab containment, and Escape keyboard behavior");
  }
  if (!/dialog\.addEventListener\(["']close["'][\s\S]*trigger\?\.focus\(/u.test(script)) {
    errors.push("S088 figure dialog requires exact invoking-trigger focus return");
  }
  if (!/aria-label/u.test(script) || !/aria-describedby/u.test(script)
      || !/sourceCaption\.childNodes/u.test(script)
      || !/expandedImage\.alt\s*=\s*["']["']/u.test(script)) {
    errors.push("S088 figure dialog requires one accessible name and optional caption description");
  }
  if (!/sourceImage\.currentSrc\s*\|\|\s*sourceImage\.src/u.test(script)) {
    errors.push("S088 figure dialog must reuse the selected local image source");
  }
  return [...new Set(errors)];
}

export function validateDocumentationFigureCss(css) {
  const errors = [];
  const trigger = css.match(/\.docs-figure-trigger\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  const affordance = css.match(/\.docs-figure-trigger::after\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  const dialog = css.match(/\.docs-figure-dialog\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  const dialogImage = css.match(/\.docs-figure-dialog__image\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  const caption = css.match(/\.docs-screenshot figcaption,\s*\.brand-surface figcaption\s*\{(?<body>[\s\S]*?)\}/u)?.groups?.body ?? "";
  const print = css.match(/@media\s+print\s*\{(?<body>[\s\S]*)\}\s*$/u)?.groups?.body ?? "";

  if (!/appearance:\s*none/iu.test(trigger) || !/cursor:\s*zoom-in/iu.test(trigger)
      || !/position:\s*relative/iu.test(trigger) || !/display:\s*block/iu.test(trigger)) {
    errors.push("S088 figure trigger requires a bounded semantic-button presentation");
  }
  if (!/content:\s*["']Expand image["']/iu.test(affordance)
      || !/position:\s*absolute/iu.test(affordance)
      || !/pointer-events:\s*none/iu.test(affordance)) {
    errors.push("S088 figure trigger requires a persistent visible expansion affordance");
  }
  if (!/\.docs-figure-trigger:hover::after,[\s\S]*\.docs-figure-trigger:focus-visible::after/iu.test(css)) {
    errors.push("S088 figure affordance requires hover and focus-visible treatment");
  }
  if (!/height:\s*var\(--docs-figure-viewport-height,\s*100vh\)/iu.test(dialog)
      || !/width:\s*var\(--docs-figure-viewport-width,\s*100vw\)/iu.test(dialog)
      || !/\.docs-figure-dialog::backdrop\s*\{/u.test(css)) {
    errors.push("S088 native dialog requires a full-viewport modal and backdrop");
  }
  if (!/display:\s*block/iu.test(dialogImage) || !/height:\s*auto/iu.test(dialogImage)
      || !/width:\s*auto/iu.test(dialogImage)
      || !/max-width:\s*calc\(var\(--docs-figure-viewport-width,\s*100vw\)/iu.test(dialogImage)
      || !/max-height:\s*calc\(var\(--docs-figure-viewport-height,\s*100vh\)/iu.test(dialogImage)
      || !/object-fit:\s*contain/iu.test(dialogImage)) {
    errors.push("S088 expanded figure must preserve intrinsic geometry without upscaling and remain viewport-contained");
  }
  const fontSize = Number(caption.match(/font-size:\s*(\d+(?:\.\d+)?)em/iu)?.[1]);
  const lineHeight = Number(caption.match(/line-height:\s*(\d+(?:\.\d+)?)/iu)?.[1]);
  if (!(fontSize >= 0.875 && fontSize < 1)) errors.push("S088 caption size must be subordinate and compute to at least 14 CSS pixels");
  if (!(lineHeight >= 1.5)) errors.push("S088 caption line height must be at least 1.5");
  if (!/\.brand-surface figcaption strong\s*\{[\s\S]*?font-weight:\s*600/iu.test(css)) {
    errors.push("S088 brand caption strong lead-ins must retain emphasis");
  }
  if (!/\.docs-figure-trigger::after,[\s\S]*\.docs-figure-dialog[\s\S]*display:\s*none\s*!important/iu.test(print)) {
    errors.push("S088 print CSS must hide figure affordances and dialog chrome");
  }
  if (!/\.docs-flow-diagram \.checkbox-img,\s*\.docs-flow-diagram \.img-wrapper\s*\{[\s\S]*?display:\s*none/iu.test(css)) {
    errors.push("S088 static and no-JavaScript rendering must hide legacy mdBook modal chrome");
  }
  return [...new Set(errors)];
}

const GLOSSARY_LEGACY_ENTRIES = [
  {
    canonical: "Managed Marker",
    aliases: ["X-ESO-Weave-Managed", "managed addon ownership", "Unmanaged"],
    target: "docs/src/features/pixelbeacon.md",
  },
  {
    canonical: "Skill Slot",
    aliases: ["ability slot", "skills 1 through 5", "Ultimate slot", "Synergy slot"],
    target: "docs/src/features/weaving.md",
  },
  {
    canonical: "Weave Type",
    aliases: ["LA", "HA", "BA", "BL", "attack pattern"],
    target: "docs/src/features/weaving.md",
  },
];

function compareGlossaryText(left, right) {
  const normalizedLeft = left.toLowerCase();
  const normalizedRight = right.toLowerCase();
  return normalizedLeft < normalizedRight ? -1 : normalizedLeft > normalizedRight ? 1 : 0;
}

export function validateFormalGlossary(markdown, searchMap) {
  const errors = [];
  if (/^##\s+Search vocabulary\s*$/imu.test(markdown)) {
    errors.push("S079 glossary must not retain a separate Search vocabulary section");
  }
  if (/^\s*[-*+]\s+\*\*[^*]+:\*\*/mu.test(markdown)) {
    errors.push("S079 glossary canonical definitions must not use the former bullet structure");
  }

  const navMatch = markdown.match(/<nav\s+class=["']glossary-index["']\s+aria-label=["']Glossary alphabet["']>([\s\S]*?)<\/nav>/iu);
  if (!navMatch) errors.push("S079 glossary requires labeled Glossary alphabet navigation");
  const navLetters = navMatch ? [...navMatch[1].matchAll(/<a\s+href=["']#([a-z])["']>\s*([A-Z])\s*<\/a>/gu)] : [];
  if (navLetters.some((match) => match[1].toUpperCase() !== match[2])) {
    errors.push("S079 glossary navigation labels must match their letter targets");
  }

  const lines = markdown.split(/\r?\n/gu);
  const groups = [];
  const entries = [];
  let currentGroup = null;
  for (let index = 0; index < lines.length; index += 1) {
    const groupMatch = lines[index].match(/^##\s+(.+?)\s*$/u);
    if (groupMatch) {
      if (!/^[A-Z]$/u.test(groupMatch[1])) {
        errors.push(`S079 glossary letter group must be one uppercase letter: ${groupMatch[1]}`);
        currentGroup = null;
      } else {
        currentGroup = groupMatch[1];
        groups.push(currentGroup);
      }
      continue;
    }
    const entryMatch = lines[index].match(/^###\s+(.+?)\s*$/u);
    if (!entryMatch) continue;
    const canonical = entryMatch[1];
    let end = index + 1;
    while (end < lines.length && !/^#{2,3}\s+/u.test(lines[end])) end += 1;
    entries.push({ canonical, group: currentGroup, block: lines.slice(index + 1, end).join("\n") });
  }

  const sortedGroups = [...groups].sort(compareGlossaryText);
  if (new Set(groups).size !== groups.length) errors.push("S079 glossary contains a duplicate letter group");
  if (groups.join("|") !== sortedGroups.join("|")) errors.push("S079 glossary letter groups are not alphabetical");
  const navValues = navLetters.map((match) => match[2]);
  if (navValues.join("|") !== groups.join("|")) {
    errors.push("S079 glossary navigation must exactly match populated letter groups");
  }

  const seenTerms = new Set();
  for (const entry of entries) {
    const folded = entry.canonical.toLowerCase();
    if (seenTerms.has(folded)) errors.push(`S079 glossary contains duplicate canonical term: ${entry.canonical}`);
    seenTerms.add(folded);
    if (!entry.group || entry.canonical[0].toUpperCase() !== entry.group) {
      errors.push(`S079 glossary term is in the wrong letter group: ${entry.canonical}`);
    }
    const aliases = entry.block.match(/^\*\*Aliases:\*\*\s+(.+)$/mu);
    if (!aliases) errors.push(`S079 glossary entry requires an Aliases field: ${entry.canonical}`);
    const related = entry.block.match(/^\*\*Related:\*\*\s+(.+)$/mu);
    if (!related) errors.push(`S079 glossary entry requires a Related field: ${entry.canonical}`);
    const definition = visibleMarkdown(entry.block)
      .replace(/^\*\*Aliases:\*\*.*$/gmu, "")
      .replace(/^\*\*Related:\*\*.*$/gmu, "")
      .replace(/\s+/gu, " ")
      .trim();
    if (definition.length < 40) errors.push(`S079 glossary entry requires a substantive definition: ${entry.canonical}`);
  }

  const groupedEntries = new Map();
  for (const entry of entries) groupedEntries.set(entry.group, [...(groupedEntries.get(entry.group) ?? []), entry.canonical]);
  for (const [letter, terms] of groupedEntries) {
    const sorted = [...terms].sort(compareGlossaryText);
    if (terms.join("|") !== sorted.join("|")) errors.push(`S079 glossary terms are not alphabetical in group ${letter}`);
  }

  const expectedEntries = [...(Array.isArray(searchMap) ? searchMap : []), ...GLOSSARY_LEGACY_ENTRIES];
  const entryByTerm = new Map(entries.map((entry) => [entry.canonical.toLowerCase(), entry]));
  for (const expected of expectedEntries) {
    const entry = entryByTerm.get(expected.canonical.toLowerCase());
    if (!entry) {
      errors.push(`S079 glossary is missing canonical term: ${expected.canonical}`);
      continue;
    }
    const aliasLine = entry.block.match(/^\*\*Aliases:\*\*\s+(.+)$/mu)?.[1] ?? "";
    const aliasItems = aliasLine.split(",").map(normalizedText);
    for (const alias of expected.aliases) {
      if (!aliasItems.includes(normalizedText(alias))) {
        errors.push(`S079 glossary ${expected.canonical} entry is missing alias: ${alias}`);
      }
    }
    const relatedLine = entry.block.match(/^\*\*Related:\*\*\s+(.+)$/mu)?.[1] ?? "";
    const hasDirectoryReadmeLink = expected.target === "docs/src/README.md" && /\]\(\.\.\/\)/u.test(relatedLine);
    if (!hasMarkdownLinkTo(relatedLine, "docs/src/reference/glossary.md", expected.target) && !hasDirectoryReadmeLink) {
      errors.push(`S079 glossary ${expected.canonical} entry is missing its related target: ${expected.target}`);
    }
  }
  if (entries.length !== expectedEntries.length) {
    errors.push(`S079 glossary requires exactly ${expectedEntries.length} canonical entries, found ${entries.length}`);
  }
  return errors;
}

const GLOSSARY_SEARCH_EVIDENCE = [
  "animation cancel",
  "key interception",
  "telemetry overlay",
  "screen telemetry",
  "config.json",
  "ring buffer",
  "WH_KEYBOARD_LL",
  "publish release",
  "mock backend",
  "X-ESO-Weave-Managed",
];

export function validateGlossarySearchIndex(searchIndex) {
  const errors = [];
  const normalized = searchIndex.toLowerCase();
  for (const phrase of GLOSSARY_SEARCH_EVIDENCE) {
    if (!normalized.includes(phrase.toLowerCase())) {
      errors.push(`S079 generated search index is missing glossary phrase: ${phrase}`);
    }
  }
  return errors;
}

export function validateContentCoverage(manifest, snapshot) {
  const errors = [];
  if (contentContractDigest(manifest) !== CONTENT_CONTRACT_SHA256) {
    errors.push("S059 content coverage semantic projection does not match the frozen contract");
  }
  if (!manifest || manifest.schema_version !== 1) errors.push("S059 content coverage requires schema_version 1");
  if (manifest?.baseline !== CONTENT_COVERAGE_BASELINE) errors.push("S059 content coverage baseline does not match the frozen commit");
  const obligations = Array.isArray(manifest?.obligations) ? manifest.obligations : [];
  const ids = obligations.map((row) => row?.id);
  const uniqueIds = new Set(ids);
  if (uniqueIds.size !== ids.length) errors.push("S059 content coverage contains a duplicate obligation identifier");
  if (uniqueIds.size !== CONTENT_OBLIGATION_IDS.size ||
      [...CONTENT_OBLIGATION_IDS].some((id) => !uniqueIds.has(id))) {
    errors.push("S059 content coverage must contain the exact obligation identifiers");
  }
  const evidenceCatalog = manifest?.evidence && typeof manifest.evidence === "object" ? manifest.evidence : {};
  const deferredSeen = new Set();
  for (const row of obligations) {
    const prefix = row?.id ?? "unknown obligation";
    if (typeof row?.area !== "string" || row.area.trim() === "" ||
        !Array.isArray(row.audience) || row.audience.length === 0 ||
        typeof row.statement !== "string" || row.statement.trim().length < 20 ||
        !Array.isArray(row.labels) || row.labels.length === 0 || row.labels.some((label) => !COVERAGE_LABELS.has(label))) {
      errors.push(`${prefix}: required obligation fields are invalid`);
    }
    if (!["Covered", "Deferred"].includes(row?.coverage)) {
      errors.push(`${prefix}: coverage may not remain ${row?.coverage ?? "missing"}`);
    }
    if (typeof row?.destination !== "string" || !/^docs\/src\/.+\.md$/u.test(row.destination)) {
      errors.push(`${prefix}: destination must be published docs/src Markdown`);
    } else {
      if (!snapshot.existingPaths.has(row.destination)) errors.push(`${prefix}: destination does not exist: ${row.destination}`);
      if (!snapshot.summaryEntries.has(row.destination)) errors.push(`${prefix}: destination is absent from SUMMARY.md`);
    }
    if (!Array.isArray(row?.content_anchors) || row.content_anchors.length === 0 ||
        row.content_anchors.some((anchor) => typeof anchor !== "string" || anchor.trim().length < 8)) {
      errors.push(`${prefix}: substantive content anchors are required`);
    } else {
      const prose = snapshot.textFiles.get(row.destination) ?? "";
      for (const anchor of row.content_anchors) {
        if (!hasVisiblePhrase(prose, anchor)) errors.push(`${prefix}: substantive anchor is missing from ${row.destination}: ${anchor}`);
      }
    }
    for (const [field, expectedKind] of [["source_evidence", "source"], ["test_evidence", "test"]]) {
      const references = Array.isArray(row?.[field]) ? row[field] : [];
      if (references.length === 0) errors.push(`${prefix}: ${field === "test_evidence" ? "test evidence or a TestGap" : "source evidence"} is required`);
      for (const reference of references) {
        const record = typeof reference === "string" ? evidenceCatalog[reference] : null;
        if (!validEvidenceRecord(record, expectedKind)) {
          errors.push(`${prefix}: invalid ${expectedKind} evidence reference`);
          continue;
        }
        if (record.kind === "TestGap") continue;
        if (!snapshot.existingPaths.has(record.path)) {
          errors.push(`${prefix}: evidence path does not exist: ${record.path}`);
        } else if (!(snapshot.textFiles.get(record.path) ?? "").includes(record.anchor)) {
          errors.push(`${prefix}: evidence anchor is missing from ${record.path}: ${record.anchor}`);
        }
      }
    }
    if (row?.coverage === "Deferred") {
      const followUp = row.follow_up;
      if (!followUp || !DEFERRED_ISSUES.has(followUp.issue) ||
          followUp.url !== `https://github.com/h8rt3rmin8r/eso-weave/issues/${followUp.issue}` ||
          typeof followUp.disposition !== "string" || followUp.disposition.trim().length < 20) {
        errors.push(`${prefix}: Deferred coverage requires an exact follow-up issue and truthful disposition`);
      } else {
        deferredSeen.add(followUp.issue);
      }
    } else if (row?.follow_up) {
      errors.push(`${prefix}: Covered obligation may not carry a follow-up disposition`);
    }
  }
  for (const issue of DEFERRED_ISSUES) {
    if (!deferredSeen.has(issue)) errors.push(`S059 deferred coverage must include follow-up issue #${issue}`);
  }

  const pages = Array.isArray(manifest?.pages) ? manifest.pages : [];
  const pagePaths = new Set();
  for (const page of pages) {
    if (!page || pagePaths.has(page.path) || !/^docs\/src\/.+\.md$/u.test(page.path) ||
        !["Landing", "Task", "Feature", "Concept", "Reference", "Development"].includes(page.page_type) ||
        !Array.isArray(page.audiences) || page.audiences.length === 0 ||
        !Array.isArray(page.required_anchors) || page.required_anchors.length < 2 ||
        !Array.isArray(page.related_pages) || page.related_pages.length === 0) {
      errors.push(`S059 exact page profile is invalid: ${page?.path ?? "missing path"}`);
      continue;
    }
    pagePaths.add(page.path);
    const prose = snapshot.textFiles.get(page.path) ?? "";
    for (const anchor of page.required_anchors) {
      if (!hasVisiblePhrase(prose, anchor)) errors.push(`${page.path}: required page anchor is missing: ${anchor}`);
    }
  }
  if (pagePaths.size !== CONTENT_PAGE_PATHS.size || [...CONTENT_PAGE_PATHS].some((item) => !pagePaths.has(item))) {
    errors.push("S059 exact page profiles are required");
  }

  const searchMap = Array.isArray(manifest?.search_map) ? manifest.search_map : [];
  const canonicalTerms = new Set();
  for (const entry of searchMap) {
    if (!entry || canonicalTerms.has(entry.canonical) || SEARCH_TARGETS.get(entry.canonical) !== entry.target ||
        !/^docs\/src\/.+\.md$/u.test(entry.target)) {
      errors.push(`S059 search target must be published and canonical: ${entry?.canonical ?? "missing term"}`);
      continue;
    }
    canonicalTerms.add(entry.canonical);
    const prose = snapshot.textFiles.get(entry.target) ?? "";
    if (!hasVisiblePhrase(prose, entry.canonical)) errors.push(`${entry.canonical}: canonical search term is absent from ${entry.target}`);
    const glossary = snapshot.textFiles.get("docs/src/reference/glossary.md") ?? "";
    if (!Array.isArray(entry.aliases) || entry.aliases.length === 0) {
      errors.push(`${entry.canonical}: required search aliases are missing`);
    } else for (const alias of entry.aliases) {
      if (!hasVisiblePhrase(prose, alias) && !glossaryExplainsAlias(glossary, alias, entry.target)) {
        errors.push(`${entry.canonical}: required search alias is absent: ${alias}`);
      }
    }
  }
  if (canonicalTerms.size !== SEARCH_TARGETS.size || [...SEARCH_TARGETS].some(([term]) => !canonicalTerms.has(term))) {
    errors.push("S059 search map must retain every contracted canonical term and target");
  }

  const diagrams = Array.isArray(manifest?.diagrams) ? manifest.diagrams : [];
  const diagramIds = new Set(diagrams.map((diagram) => diagram?.id));
  if (diagramIds.size !== DIAGRAM_IDS.size || [...DIAGRAM_IDS].some((id) => !diagramIds.has(id))) {
    errors.push("S059 diagrams must contain the exact contracted records");
  }
  for (const diagram of diagrams) {
    if (!diagram || !/^docs\/src\/.+\.md$/u.test(diagram.destination) ||
        typeof diagram.title !== "string" || diagram.title.trim().length < 8 ||
        typeof diagram.alt_text !== "string" || diagram.alt_text.trim().length < 20 || /^diagram\.?$/iu.test(diagram.alt_text.trim()) ||
        typeof diagram.text_equivalent !== "string" || diagram.text_equivalent.trim().length < 30 ||
        /^(?:see|follow) (?:the )?(?:colors?|diagram)/iu.test(diagram.text_equivalent.trim())) {
      errors.push(`${diagram?.id ?? "unknown diagram"}: diagram requires a useful non-color text equivalent and alternative`);
      continue;
    }
    if (!snapshot.existingPaths.has(diagram.destination)) {
      errors.push(`${diagram.id}: diagram destination does not exist: ${diagram.destination}`);
      continue;
    }
    if (!snapshot.summaryEntries.has(diagram.destination)) {
      errors.push(`${diagram.id}: diagram destination is absent from SUMMARY.md: ${diagram.destination}`);
    }
    const prose = snapshot.textFiles.get(diagram.destination) ?? "";
    if (!hasVisiblePhrase(prose, diagram.title)) {
      errors.push(`${diagram.id}: diagram title is absent from visible destination content: ${diagram.title}`);
    }
    if (!Array.isArray(diagram.content_anchors) || diagram.content_anchors.length === 0) {
      errors.push(`${diagram.id}: diagram content anchors are required`);
    } else for (const anchor of diagram.content_anchors) {
      if (!hasVisiblePhrase(prose, anchor)) {
        errors.push(`${diagram.id}: diagram content anchor is missing from ${diagram.destination}: ${anchor}`);
      }
    }
  }
  return errors;
}

export async function validateContentCoverageRepository(repositoryRoot, manifest) {
  const sourceRoot = path.join(repositoryRoot, "docs", "src");
  const summary = await readFile(path.join(sourceRoot, "SUMMARY.md"), "utf8");
  const summaryEntries = new Set();
  for (const line of summary.split(/\r?\n/gu)) {
    if (!/^\s*[-*+]\s+/u.test(line)) continue;
    for (const link of markdownLinks(line)) {
      const { pathname } = splitTarget(link.destination);
      if (!pathname.endsWith(".md")) continue;
      summaryEntries.add(`docs/src/${slash(path.normalize(pathname))}`);
    }
  }
  const repositoryErrors = [];
  const referencedPaths = new Set([...(manifest.obligations ?? []).map((row) => row.destination), ...(manifest.pages ?? []).map((page) => page.path), ...(manifest.search_map ?? []).map((entry) => entry.target), ...(manifest.diagrams ?? []).map((diagram) => diagram.destination), "docs/src/reference/glossary.md"]);
  for (const record of Object.values(manifest.evidence ?? {})) {
    if (!record?.path) continue;
    const relative = record.path;
    const normalized = typeof relative === "string" ? slash(path.normalize(relative)) : "";
    const resolved = typeof relative === "string" ? path.resolve(repositoryRoot, relative) : repositoryRoot;
    const fromRoot = path.relative(repositoryRoot, resolved);
    if (typeof relative !== "string" || path.isAbsolute(relative) || normalized !== relative ||
        slash(relative).split("/").some((segment) => segment === "." || segment === "..") ||
        fromRoot.startsWith("..") || path.isAbsolute(fromRoot)) {
      repositoryErrors.push(`evidence path must be normalized and repository-relative: ${relative}`);
      continue;
    }
    const casing = await exactCase(repositoryRoot, relative);
    if (casing.exists && !casing.exact) {
      repositoryErrors.push(`evidence path must use exact case: ${relative}`);
      continue;
    }
    referencedPaths.add(relative);
  }
  const existingPaths = new Set();
  const textFiles = new Map();
  for (const relative of referencedPaths) {
    const absolute = path.join(repositoryRoot, relative);
    try {
      const contents = await readFile(absolute, "utf8");
      existingPaths.add(relative);
      textFiles.set(relative, contents);
    } catch {
      // The pure validator reports the exact missing reference.
    }
  }
  return [...repositoryErrors, ...validateContentCoverage(manifest, { existingPaths, summaryEntries, textFiles })];
}

export function validateCatalogSourceContract(contract) {
  const errors = [];
  const requiredCategories = new Set([
    "player-skills", "crafted-abilities", "ability-metadata", "effects-and-status",
    "items-and-gear", "item-sets", "champion-skills", "consumables", "mundus-effects",
    "companions-races-classes", "combat-statistics", "constants", "localized-text",
    "icon-references", "icon-bytes",
  ]);
  const completenessValues = new Set(["exhaustive", "bounded", "opportunistic", "unknown"]);
  const redistributionValues = new Set([
    "allowed", "attribution-required", "user-generated-only", "prohibited", "unresolved",
  ]);
  const methods = new Set([
    "api-iterator", "known-id", "observed-event", "item-link", "constant-file", "local-file", "none",
  ]);
  const enumerableMethods = new Set(["api-iterator", "constant-file", "local-file"]);
  const sourceFamilies = new Set(["zos-api", "stock-ui", "collector", "community-code", "archive", "remote"]);
  const sourceChannels = new Set(["live", "pts", "not-applicable"]);
  if (!contract || typeof contract !== "object") return ["catalog contract must be an object"];
  if (contract.schema_version !== 1) errors.push("catalog contract schema_version must be 1");
  if (!/^\d{4}-\d{2}-\d{2}$/u.test(contract.as_of ?? "")) errors.push("catalog contract as_of must be an ISO date");
  if (!contract.project_facts?.placeholders_allowed) errors.push("catalog contract must retain project-created placeholders");
  if (contract.project_facts?.telemetry !== false) errors.push("catalog contract must record telemetry as disabled");
  if (!Array.isArray(contract.completeness_values) ||
      completenessValues.size !== contract.completeness_values.length ||
      [...completenessValues].some((value) => !contract.completeness_values.includes(value))) {
    errors.push("catalog contract completeness_values must declare the exact supported vocabulary");
  }
  if (!Array.isArray(contract.redistribution_values) ||
      redistributionValues.size !== contract.redistribution_values.length ||
      [...redistributionValues].some((value) => !contract.redistribution_values.includes(value))) {
    errors.push("catalog contract redistribution_values must declare the exact supported vocabulary");
  }

  const snapshots = Array.isArray(contract.source_snapshots) ? contract.source_snapshots : [];
  const snapshotIds = new Set();
  for (const snapshot of snapshots) {
    if (!snapshot?.id || snapshotIds.has(snapshot.id)) errors.push(`catalog contract has invalid or duplicate source id ${snapshot?.id ?? "<missing>"}`);
    else snapshotIds.add(snapshot.id);
    const sourceLabel = snapshot?.id ?? "<missing>";
    for (const key of ["family", "locale", "revision", "uri", "license_scope"]) {
      if (typeof snapshot?.[key] !== "string" || snapshot[key].trim() === "") {
        errors.push(`source ${sourceLabel} requires ${key}`);
      }
    }
    if (!sourceFamilies.has(snapshot?.family)) errors.push(`source ${sourceLabel} requires a supported family`);
    if (!sourceChannels.has(snapshot?.channel)) errors.push(`source ${sourceLabel} requires a supported channel`);
    if (!/^\d{4}-\d{2}-\d{2}$/u.test(snapshot?.acquired_at ?? "")) {
      errors.push(`source ${sourceLabel} requires acquired_at as an ISO date`);
    }
    if (snapshot?.recommended) {
      if (!/^[0-9a-f]{40}$/u.test(snapshot.revision ?? "")) {
        errors.push(`recommended source ${snapshot.id} requires an immutable 40-character revision`);
      }
      if (!/^[0-9a-f]{64}$/u.test(snapshot.sha256 ?? "")) {
        errors.push(`recommended source ${snapshot.id} requires a lowercase SHA-256`);
      }
    }
    if (["live", "pts"].includes(snapshot?.channel) &&
        (!Number.isInteger(snapshot.api_version) || !snapshot.game_version)) {
      errors.push(`channel source ${snapshot.id} requires game and API versions`);
    }
  }
  const live = snapshots.find((snapshot) => snapshot.channel === "live" && snapshot.recommended);
  const pts = snapshots.find((snapshot) => snapshot.channel === "pts" && snapshot.recommended);
  if (!live || !pts) errors.push("catalog contract requires recommended live and PTS snapshots");
  else if (live.id === pts.id || live.revision === pts.revision || live.api_version === pts.api_version) {
    errors.push("catalog contract live and PTS snapshots must remain distinct");
  }

  const categories = Array.isArray(contract.categories) ? contract.categories : [];
  const categoryIds = new Set();
  for (const category of categories) {
    if (!category?.id || categoryIds.has(category.id)) errors.push(`catalog contract has invalid or duplicate category ${category?.id ?? "<missing>"}`);
    else categoryIds.add(category.id);
    for (const key of ["category", "stable_key", "enumeration_method", "completeness", "redistribution", "field_verification"]) {
      if (typeof category?.[key] !== "string" || category[key].trim() === "") {
        errors.push(`catalog category ${category?.id ?? "<missing>"} requires ${key}`);
      }
    }
    for (const key of ["visibility", "source_ids", "failure_modes", "validation"]) {
      if (!Array.isArray(category?.[key]) || category[key].length === 0) {
        errors.push(`catalog category ${category?.id ?? "<missing>"} requires non-empty ${key}`);
      }
    }
    const stableKeyTerms = String(category?.stable_key ?? "").toLowerCase().split(/[^a-z0-9]+/u);
    const transientKeyTerms = /^(?:(?:array|iterator|lua|mutable|bag|slot|ordinal|sequence|list)?(?:index|position|offset))$/u;
    if (stableKeyTerms.some((term) => transientKeyTerms.test(term))) {
      errors.push(`catalog category ${category.id} has a transient stable_key`);
    }
    if (!methods.has(category?.enumeration_method)) errors.push(`catalog category ${category?.id} has invalid enumeration method`);
    if (!completenessValues.has(category?.completeness)) errors.push(`catalog category ${category?.id} has invalid completeness`);
    if (!redistributionValues.has(category?.redistribution)) errors.push(`catalog category ${category?.id} has invalid redistribution`);
    if (category?.completeness === "exhaustive" && category.field_verification !== "none") {
      errors.push(`catalog category ${category.id} cannot be exhaustive while field verification is pending`);
    }
    if (category?.completeness === "exhaustive" && !enumerableMethods.has(category.enumeration_method)) {
      errors.push(`catalog category ${category.id} requires an enumerable method for exhaustive coverage`);
    }
    if (category?.completeness === "exhaustive" &&
        !(category.validation ?? []).some((check) => /\b(?:count|relationship)s?\b/iu.test(check))) {
      errors.push(`catalog category ${category.id} requires count or relationship validation for exhaustive coverage`);
    }
    for (const sourceId of category?.source_ids ?? []) {
      if (!snapshotIds.has(sourceId)) errors.push(`catalog category ${category?.id} references unknown source ${sourceId}`);
    }
  }
  for (const id of requiredCategories) {
    if (!categoryIds.has(id)) errors.push(`catalog contract is missing required category ${id}`);
  }
  const iconBytes = categories.find((category) => category.id === "icon-bytes");
  if (iconBytes?.redistribution !== "prohibited") {
    errors.push("catalog contract must prohibit redistribution of game icon bytes");
  }
  for (const decision of ["zenimax_icon_bytes", "prebuilt_extracted_icon_pack"]) {
    if (contract.redistribution_decisions?.[decision] !== "prohibited") {
      errors.push(`catalog contract must prohibit ${decision}`);
    }
  }

  if (contract.promotion_policy?.automatic !== false) errors.push("catalog contract must forbid automatic PTS promotion");
  if (contract.promotion_policy?.preserve_original_channel !== true) {
    errors.push("catalog promotion policy must preserve original channel provenance");
  }
  for (const key of ["requires_live_api_version", "requires_live_revision", "requires_content_hash", "requires_reviewer"]) {
    if (contract.promotion_policy?.[key] !== true) errors.push(`catalog promotion policy requires ${key}`);
  }
  const collector = contract.collector_policy ?? {};
  if (collector.execute_lua !== false) errors.push("catalog collector must never execute Lua");
  if (collector.byte_limit_required !== true) errors.push("catalog collector requires a byte limit");
  if (collector.record_limit_required !== true) errors.push("catalog collector requires a record limit");
  if (collector.atomic_import !== true) errors.push("catalog collector import must be atomic");
  if (collector.upload_default !== false) errors.push("catalog collector data must not be uploaded by default");
  if (collector.user_data_separate !== true) errors.push("catalog collector must keep user data separate");
  for (const key of ["provisional_max_snapshot_bytes", "provisional_max_records", "provisional_max_string_bytes"]) {
    if (!Number.isInteger(collector[key]) || collector[key] <= 0) {
      errors.push(`catalog collector requires positive integer ${key}`);
    }
  }
  const iconCache = contract.icon_cache_policy ?? {};
  if (iconCache.source_selection !== "explicit-user-directory") {
    errors.push("catalog icon cache requires an explicit user directory");
  }
  for (const key of ["network_access", "archive_extraction", "installed_client_discovery", "source_mutation", "third_party_bytes_distributable"]) {
    if (iconCache[key] !== false) errors.push(`catalog icon cache must disable ${key}`);
  }
  for (const key of ["manifest_required", "immutable_generations"]) {
    if (iconCache[key] !== true) errors.push(`catalog icon cache requires ${key}`);
  }
  if (iconCache.fallback !== "project-created-placeholder") {
    errors.push("catalog icon cache requires the project-created placeholder");
  }
  if (!Array.isArray(iconCache.allowed_input) ||
      iconCache.allowed_input.length !== 2 ||
      !iconCache.allowed_input.includes("png") ||
      !iconCache.allowed_input.includes("dds")) {
    errors.push("catalog icon cache allows only PNG and DDS input");
  }
  for (const key of ["max_source_bytes", "max_dimension", "max_pixels", "max_references", "max_manifest_bytes"]) {
    if (!Number.isInteger(iconCache[key]) || iconCache[key] <= 0) {
      errors.push(`catalog icon cache requires positive integer ${key}`);
    }
  }
  return [...new Set(errors)];
}

const REQUIRED_ENCOUNTER_KINDS = new Set([
  "encounter-start", "encounter-end", "damage", "healing", "effect", "resource", "cast",
  "bar-change", "death", "resurrection", "boss-health", "performance", "quickslot", "discontinuity",
]);
const REQUIRED_ENCOUNTER_METRICS = new Set([
  "observed-dps", "observed-hps", "ability-damage-share", "effect-uptime", "ordered-cast-sequence",
]);

function canonicalEncounterEvents(events) {
  return JSON.stringify([...events].sort((left, right) => left.sequence - right.sequence));
}

export function validateEncounterModelContract(contract) {
  const errors = [];
  if (!contract || typeof contract !== "object") return ["encounter model contract must be an object"];
  for (const field of [
    "source_snapshots", "storage_planes", "capture_envelope", "ordering_policy", "loss_policy",
    "privacy_policy", "integrity_policy", "catalog_join_policy", "actor_policy", "build_snapshot_policy",
    "retention_policy", "recommendation_policy", "catalog_schema_requirements", "transport_policy",
    "event_kinds", "metrics", "parity_roadmap", "follow_up_order", "follow_up_issues", "synthetic_fixture",
  ]) if (!(field in contract)) errors.push(`encounter model requires ${field}`);
  if (contract.schema_version !== 1) errors.push("encounter model schema_version must be 1");
  if (!/^\d{4}-\d{2}-\d{2}$/u.test(contract.as_of ?? "")) errors.push("encounter model as_of must be an ISO date");

  const snapshots = Array.isArray(contract.source_snapshots) ? contract.source_snapshots : [];
  const snapshotIds = new Set();
  for (const snapshot of snapshots) {
    if (!snapshot?.id || snapshotIds.has(snapshot.id)) errors.push("encounter source IDs must be present and unique");
    else snapshotIds.add(snapshot.id);
    if (!/^[0-9a-f]{40}$/u.test(snapshot?.revision ?? "")) errors.push(`encounter source ${snapshot?.id ?? "<missing>"} requires an immutable revision`);
    for (const field of ["channel", "license", "uri"]) {
      if (typeof snapshot?.[field] !== "string" || snapshot[field].trim() === "") errors.push(`encounter source ${snapshot?.id ?? "<missing>"} requires ${field}`);
    }
  }
  for (const required of ["eso-api-live", "libcombat", "combat-metrics"]) {
    if (!snapshotIds.has(required)) errors.push(`encounter model is missing required source ${required}`);
  }

  const planes = contract.storage_planes ?? {};
  if (!planes.catalog || !planes.raw || !planes.derived || planes.raw === planes.catalog || planes.derived === planes.catalog) {
    errors.push("raw observations and derived analysis must remain separate from catalog storage");
  }
  if (planes.catalog !== "catalog.sqlite") errors.push("catalog storage must remain catalog.sqlite");

  const envelope = contract.capture_envelope ?? {};
  if (JSON.stringify(envelope.identity) !== JSON.stringify(["session_id", "sequence"])) errors.push("encounter event identity must be session_id plus sequence");
  if (envelope.duration_clock !== "monotonic_ms") errors.push("encounter durations require monotonic_ms");
  if (envelope.actor_identity !== "encounter-local-opaque") errors.push("encounter actors must use encounter-local opaque identity");

  const ordering = contract.ordering_policy ?? {};
  if (ordering.authority !== "sequence" || ordering.reject_duplicates !== true || ordering.reject_undeclared_gaps !== true || ordering.reject_backward_monotonic_time !== true) {
    errors.push("encounter ordering must reject duplicate, undeclared-gap, and backward-time input");
  }
  if (contract.loss_policy?.marker !== "discontinuity" || contract.loss_policy?.degrade_spanning_metrics !== true || contract.loss_policy?.expose_ranges !== true) {
    errors.push("encounter loss must use exposed discontinuities and degrade spanning metrics");
  }

  const privacy = contract.privacy_policy ?? {};
  if (privacy.local_only_default !== true || privacy.upload_default !== false) errors.push("encounter data must be local-only and never uploaded by default");
  for (const omitted of ["account-name", "character-name", "chat", "guild", "location"]) {
    if (!privacy.omitted_by_default?.includes(omitted)) errors.push(`encounter privacy must omit ${omitted} by default`);
  }

  const integrity = contract.integrity_policy ?? {};
  if (integrity.raw_immutable !== true) errors.push("raw observations must be immutable");
  if (integrity.derived_rebuildable !== true) errors.push("derived analysis must be rebuildable");
  if (integrity.execute_input !== false || integrity.bounded_import !== true || integrity.atomic_import !== true) errors.push("encounter imports must be bounded, atomic, and non-executing");
  const join = contract.catalog_join_policy ?? {};
  if (join.retain_unknown_ids !== true || join.rejoin_without_raw_mutation !== true || join.preserve_channel !== true) errors.push("catalog joins must preserve unknown IDs, raw content, and channel provenance");
  const transport = contract.transport_policy ?? {};
  if (transport.pixel_bus_bulk_transport !== false) errors.push("Pixel Bus cannot be the bulk encounter transport");
  if (transport.automation_independent !== true) errors.push("encounter observation and calculation must remain independent of automation");
  if (transport.future_transport !== "bounded-saved-variables-import") errors.push("future encounter transport must use a bounded SavedVariables import");
  if (contract.actor_policy?.identity !== "encounter-local-opaque" || !contract.actor_policy?.roles?.includes("pet") || !contract.actor_policy?.pet_owner_relationship || !contract.actor_policy?.ability_aliases) errors.push("encounter actor policy must model opaque actors, pets, owners, and aliases");
  if (contract.build_snapshot_policy?.retention !== "derived-versioned" || contract.build_snapshot_policy?.catalog_version_required !== true || contract.build_snapshot_policy?.consent_required_for_personal_identity !== true) errors.push("build snapshots must be versioned, catalog-bound, and consent personal identity");
  const retention = contract.retention_policy ?? {};
  for (const field of ["export", "delete", "backup", "corruption_recovery", "compression"]) if (!retention[field]) errors.push(`encounter retention policy requires ${field}`);
  if (retention.production_budget !== "verification-required") errors.push("encounter production storage budget must remain verification-required");
  const recommendations = contract.recommendation_policy ?? {};
  if (recommendations.requires_encounter_version !== true || recommendations.requires_catalog_version !== true || recommendations.requires_metric_quality !== true || recommendations.correlation_is_not_causation !== true || recommendations.action_automation_coupling !== false) errors.push("recommendations must be versioned, quality-scoped, non-causal, and automation-independent");
  const catalogRequirements = contract.catalog_schema_requirements ?? {};
  if (!Array.isArray(catalogRequirements.entities) || catalogRequirements.entities.length === 0 || !Array.isArray(catalogRequirements.relationships) || catalogRequirements.relationships.length === 0 || catalogRequirements.unknown_id_supported !== true) errors.push("encounter model requires concrete catalog entities, relationships, and unknown-ID support");

  const eventIds = new Set((Array.isArray(contract.event_kinds) ? contract.event_kinds : []).map((row) => row?.id));
  for (const id of REQUIRED_ENCOUNTER_KINDS) if (!eventIds.has(id)) errors.push(`encounter model is missing required event kind ${id}`);
  const metrics = Array.isArray(contract.metrics) ? contract.metrics : [];
  const metricIds = new Set(metrics.map((row) => row?.id));
  for (const id of REQUIRED_ENCOUNTER_METRICS) if (!metricIds.has(id)) errors.push(`encounter model is missing required metric ${id}`);
  for (const metric of metrics) {
    if (!metric?.algorithm_version) errors.push(`metric ${metric?.id ?? "<missing>"} requires an algorithm version`);
    if (metric?.source_range_required !== true) errors.push(`metric ${metric?.id ?? "<missing>"} requires a source range`);
    if (metric?.quality_required !== true) errors.push(`metric ${metric?.id ?? "<missing>"} requires quality disclosure`);
  }
  for (const row of Array.isArray(contract.parity_roadmap) ? contract.parity_roadmap : []) {
    for (const field of ["capability", "source_event", "calculation", "confidence", "privacy_impact", "state", "target_phase", "acceptance", "owner", "risk"]) {
      if (typeof row?.[field] !== "string" || row[field].trim() === "") errors.push(`parity row ${row?.capability ?? "<missing>"} requires ${field}`);
    }
    if (row?.state !== "fixture-proved" && !/^issue #\d+$/u.test(row?.owner ?? "")) errors.push(`parity row ${row?.capability ?? "<missing>"} requires a concrete owner issue`);
  }
  if (JSON.stringify(contract.follow_up_order) !== JSON.stringify(["capture", "import", "calculation", "ui", "recommendations"])) {
    errors.push("encounter follow-up order must be capture, import, calculation, UI, recommendations");
  }
  for (const phase of ["capture", "import", "calculation", "ui", "recommendations", "live_parity_verification"]) {
    if (!Number.isInteger(contract.follow_up_issues?.[phase]) || contract.follow_up_issues[phase] <= 0) errors.push(`encounter follow-up ${phase} requires an issue number`);
  }
  if (contract.synthetic_fixture?.proves_live_parity !== false) errors.push("synthetic evidence cannot prove live parity");
  for (const field of ["encounter", "projection"]) if (!contract.synthetic_fixture?.[field]) errors.push(`synthetic fixture requires ${field}`);
  return [...new Set(errors)];
}

export function validateEncounterFixture(fixture) {
  const errors = [];
  if (!fixture?.envelope || !Array.isArray(fixture?.events)) return ["encounter fixture requires an envelope and events"];
  const events = [...fixture.events].sort((left, right) => left.sequence - right.sequence);
  const seen = new Set();
  let previous;
  const prohibitedPrivateFields = new Set([
    "accountid", "accountname", "characterid", "charactername", "chat", "chattext",
    "guild", "guildid", "guildname", "location", "locationname",
  ]);
  const scanPrivateFields = (value, location) => {
    if (!value || typeof value !== "object") return;
    for (const [key, nested] of Object.entries(value)) {
      const normalized = key.toLowerCase().replace(/[^a-z0-9]/gu, "");
      if (prohibitedPrivateFields.has(normalized)) errors.push(`${location} contains prohibited private field ${key}`);
      scanPrivateFields(nested, location);
    }
  };
  scanPrivateFields(fixture.envelope, "encounter envelope");
  for (const event of events) {
    if (!Number.isInteger(event.sequence) || seen.has(event.sequence)) errors.push(`duplicate sequence ${event.sequence}`);
    seen.add(event.sequence);
    if (event.session_id !== fixture.envelope.session_id || event.encounter_id !== fixture.envelope.encounter_id) errors.push(`event ${event.sequence} does not match its envelope identity`);
    if (!Number.isFinite(event.monotonic_ms)) errors.push(`event ${event.sequence} requires monotonic_ms`);
    if (previous) {
      if (event.monotonic_ms < previous.monotonic_ms) errors.push(`backward monotonic time at sequence ${event.sequence}`);
      const hasGap = event.sequence > previous.sequence + 1;
      const declaresGap = event.kind === "discontinuity" &&
        Number.isInteger(event.payload?.missing_sequence_from) && Number.isInteger(event.payload?.missing_sequence_to) &&
        event.payload.missing_sequence_from === previous.sequence + 1 &&
        event.payload.missing_sequence_to === event.sequence - 1;
      if (hasGap && !declaresGap) errors.push(`undeclared sequence gap after ${previous.sequence}`);
      if (event.kind === "discontinuity" && !declaresGap) errors.push(`discontinuity ${event.sequence} must exactly describe the preceding missing range`);
    }
    if (!previous && event.kind === "discontinuity") errors.push(`discontinuity ${event.sequence} cannot be the first event`);
    if (event.kind === "discontinuity" && (typeof event.payload?.reason !== "string" || event.payload.reason.trim() === "")) errors.push(`discontinuity ${event.sequence} requires a non-empty reason`);
    scanPrivateFields(event, `event ${event.sequence}`);
    previous = event;
  }
  const kinds = new Set(events.map((event) => event.kind));
  for (const id of REQUIRED_ENCOUNTER_KINDS) if (!kinds.has(id)) errors.push(`encounter fixture is missing required event kind ${id}`);
  if (events[0]?.sequence !== fixture.envelope.first_sequence || events.at(-1)?.sequence !== fixture.envelope.last_sequence) errors.push("encounter fixture sequence range does not match its envelope");
  if (events[0]?.monotonic_ms !== fixture.envelope.started_monotonic_ms || events.at(-1)?.monotonic_ms !== fixture.envelope.ended_monotonic_ms) errors.push("encounter fixture duration range does not match its envelope");
  const contentSha256 = createHash("sha256").update(canonicalEncounterEvents(events), "utf8").digest("hex");
  if (fixture.envelope.content_sha256 && fixture.envelope.content_sha256 !== contentSha256) errors.push("encounter fixture content_sha256 does not match canonical events");
  return [...new Set(errors)];
}

export function projectEncounterMetrics(fixture, knownCatalogIds = new Set()) {
  const errors = validateEncounterFixture(fixture);
  if (errors.length > 0) throw new Error(errors.join("\n"));
  const events = [...fixture.events].sort((left, right) => left.sequence - right.sequence);
  const durationMs = fixture.envelope.ended_monotonic_ms - fixture.envelope.started_monotonic_ms;
  const damage = events.filter((event) => event.kind === "damage" && event.payload.direction === "outgoing");
  const healing = events.filter((event) => event.kind === "healing" && event.payload.direction === "outgoing");
  const totalDamage = damage.reduce((sum, event) => sum + event.payload.amount, 0);
  const totalHealing = healing.reduce((sum, event) => sum + event.payload.effective_amount, 0);
  const damageByAbility = {};
  for (const event of damage) damageByAbility[event.payload.ability_id] = (damageByAbility[event.payload.ability_id] ?? 0) + event.payload.amount;
  const effectStarts = new Map();
  const effectIntervals = new Map();
  for (const event of events.filter((candidate) => candidate.kind === "effect")) {
    const id = event.payload.ability_id;
    const instance = event.payload.effect_instance_id ?? event.payload.effect_slot ?? "default";
    const instanceKey = `${event.payload.target_actor_id ?? "unknown"}:${id}:${instance}`;
    if (event.payload.change === "gained" && !effectStarts.has(instanceKey)) effectStarts.set(instanceKey, { id, started: event.monotonic_ms });
    else if (event.payload.change === "faded" && effectStarts.has(instanceKey)) {
      const started = effectStarts.get(instanceKey).started;
      if (!effectIntervals.has(id)) effectIntervals.set(id, []);
      effectIntervals.get(id).push([started, event.monotonic_ms]);
      effectStarts.delete(instanceKey);
    }
  }
  for (const { id, started } of effectStarts.values()) {
    if (!effectIntervals.has(id)) effectIntervals.set(id, []);
    effectIntervals.get(id).push([started, fixture.envelope.ended_monotonic_ms]);
  }
  const effectDurations = {};
  for (const [id, intervals] of effectIntervals) {
    const sorted = intervals.map(([start, end]) => [
      Math.max(start, fixture.envelope.started_monotonic_ms),
      Math.min(end, fixture.envelope.ended_monotonic_ms),
    ]).filter(([start, end]) => end > start).sort((left, right) => left[0] - right[0]);
    let total = 0;
    let current;
    for (const interval of sorted) {
      if (!current || interval[0] > current[1]) {
        if (current) total += current[1] - current[0];
        current = [...interval];
      } else current[1] = Math.max(current[1], interval[1]);
    }
    if (current) total += current[1] - current[0];
    effectDurations[id] = total;
  }
  const abilityIds = [...new Set(events.map((event) => event.payload?.ability_id).filter(Number.isInteger))].sort((left, right) => left - right);
  const lossRanges = events.filter((event) => event.kind === "discontinuity").map((event) => ({ from: event.payload.missing_sequence_from, to: event.payload.missing_sequence_to, reason: event.payload.reason }));
  const quality = lossRanges.length > 0 ? "degraded" : "complete";
  const decorate = (value) => ({ ...value, algorithm_version: "s069-v1", first_sequence: fixture.envelope.first_sequence, last_sequence: fixture.envelope.last_sequence, quality, loss_ranges: lossRanges });
  const rawContentSha256 = createHash("sha256").update(canonicalEncounterEvents(events), "utf8").digest("hex");
  return {
    raw_content_sha256: rawContentSha256,
    metrics: {
      "observed-dps": decorate({ value: totalDamage / (durationMs / 1000), unit: "damage-per-second" }),
      "observed-hps": decorate({ value: totalHealing / (durationMs / 1000), unit: "effective-healing-per-second" }),
      "ability-damage-share": decorate({ values: Object.fromEntries(Object.entries(damageByAbility).map(([id, amount]) => [id, amount / totalDamage])), unit: "ratio" }),
      "effect-uptime": decorate({ values: Object.fromEntries(Object.entries(effectDurations).map(([id, amount]) => [id, amount / durationMs])), unit: "ratio" }),
      "ordered-cast-sequence": decorate({ values: events.filter((event) => event.kind === "cast").map((event) => event.payload.ability_id), unit: "ability-id-sequence" }),
    },
    catalog_receipt: {
      known_ids: abilityIds.filter((id) => knownCatalogIds.has(id)),
      unknown_ids: abilityIds.filter((id) => !knownCatalogIds.has(id)),
      raw_content_sha256: rawContentSha256,
    },
  };
}

export function validateEncounterEvidence(fixture, expected, rawBytes) {
  const errors = validateEncounterFixture(fixture);
  if (errors.length > 0) return errors;
  const initial = projectEncounterMetrics(fixture, new Set([100, 200, 300]));
  const resolved = projectEncounterMetrics(fixture, new Set([100, 200, 300, 999999]));
  if (expected?.schema_version !== 1) errors.push("encounter projection schema_version must be 1");
  if (expected?.algorithm_version !== "s069-v1") errors.push("encounter projection algorithm_version must be s069-v1");
  if (expected?.raw_content_sha256 !== initial.raw_content_sha256) errors.push("encounter projection raw_content_sha256 is stale");
  if (fixture.envelope.content_sha256 !== initial.raw_content_sha256) errors.push("encounter envelope content_sha256 is stale");
  if (JSON.stringify(expected?.loss_ranges) !== JSON.stringify(initial.metrics["observed-dps"].loss_ranges)) errors.push("encounter projection loss_ranges are stale");
  for (const metricId of REQUIRED_ENCOUNTER_METRICS) {
    const actual = initial.metrics[metricId];
    const receipt = expected?.metrics?.[metricId];
    if (!receipt) errors.push(`encounter projection is missing metric ${metricId}`);
    else {
      const resultField = "value" in actual ? "value" : "values";
      for (const field of [resultField, "unit", "algorithm_version", "first_sequence", "last_sequence", "quality", "loss_ranges"]) {
        if (!(field in receipt)) errors.push(`encounter projection ${metricId} requires ${field}`);
        else if (JSON.stringify(receipt[field]) !== JSON.stringify(actual[field])) errors.push(`encounter projection ${metricId} has stale ${field}`);
      }
    }
  }
  const receipts = Array.isArray(expected?.catalog_receipts) ? expected.catalog_receipts : [];
  for (const [index, label, actual] of [[0, "initial", initial.catalog_receipt], [1, "resolved", resolved.catalog_receipt]]) {
    const receipt = receipts[index];
    if (!receipt) {
      errors.push(`${label} catalog receipt is missing`);
      continue;
    }
    if (typeof receipt.catalog_snapshot !== "string" || receipt.catalog_snapshot.trim() === "") errors.push(`${label} catalog receipt requires catalog_snapshot`);
    for (const field of ["known_ids", "unknown_ids"]) {
      if (!Array.isArray(receipt[field])) errors.push(`${label} catalog receipt requires ${field}`);
      else if (JSON.stringify(receipt[field]) !== JSON.stringify(actual[field])) errors.push(`${label} catalog receipt has stale ${field}`);
    }
    if (receipt.raw_content_sha256 !== initial.raw_content_sha256) errors.push(`${label} catalog receipt must preserve the raw-content hash`);
  }
  if (rawBytes) {
    if (expected?.storage?.exact?.raw_bytes !== rawBytes.byteLength) errors.push("encounter fixture exact raw byte receipt is stale");
    if (expected?.storage?.exact?.gzip_bytes !== gzipSync(rawBytes, { mtime: 0 }).byteLength) errors.push("encounter fixture exact gzip byte receipt is stale");
    const rawSize = rawBytes.byteLength;
    const gzipSize = gzipSync(rawBytes, { mtime: 0 }).byteLength;
    const estimates = expected?.storage?.linear_estimates ?? {};
    if (estimates.one_hour_at_fixture_rate?.raw_bytes !== rawSize * 360 || estimates.one_hour_at_fixture_rate?.gzip_bytes !== gzipSize * 360) errors.push("encounter fixture one-hour linear estimate is stale");
    if (estimates.one_hundred_fixture_encounters?.raw_bytes !== rawSize * 100 || estimates.one_hundred_fixture_encounters?.gzip_bytes !== gzipSize * 100) errors.push("encounter fixture 100-encounter linear estimate is stale");
  }
  if (expected?.storage?.production_retention_recommendation !== "verification-required") errors.push("production retention guidance must remain verification-required");
  if (expected?.parity_claim !== "synthetic-determinism-only") errors.push("synthetic evidence must remain a determinism-only parity claim");
  return [...new Set(errors)];
}

export function validateMigrationLedger(ledger, snapshot) {
  const errors = [];
  if (ledger.baselineCommit !== MIGRATION_BASELINE) {
    errors.push(`migration ledger baselineCommit must be ${MIGRATION_BASELINE}`);
  }
  const expectedSummary = {
    docsArtifacts: 49,
    websiteArtifacts: 1,
    totalArtifacts: 50,
    specificationH2Units: 20,
    legacyPlans: 27,
    safetyInvariants: 6,
  };
  if (JSON.stringify(ledger.baselineSummary) !== JSON.stringify(expectedSummary)) {
    errors.push("migration ledger baselineSummary does not match the frozen corpus");
  }

  const artifacts = Array.isArray(ledger.artifacts) ? ledger.artifacts : [];
  const bySource = new Map();
  for (const artifact of artifacts) {
    if (bySource.has(artifact.source)) errors.push(`duplicate baseline artifact: ${artifact.source}`);
    bySource.set(artifact.source, artifact);
  }
  const artifactPaths = new Set(bySource.keys());
  const missing = [...BASELINE_ARTIFACTS.keys()].filter((source) => !artifactPaths.has(source));
  const unexpected = [...artifactPaths].filter((source) => !BASELINE_ARTIFACTS.has(source));
  if (artifacts.length !== BASELINE_ARTIFACTS.size || missing.length > 0 || unexpected.length > 0) {
    errors.push(`baseline artifact coverage must contain exactly ${BASELINE_ARTIFACTS.size} frozen paths`);
  }

  const existingPaths = snapshot?.existingPaths ?? new Set();
  const currentPaths = snapshot?.currentPaths ?? existingPaths;
  const textFiles = snapshot?.textFiles ?? new Map();
  const preservationManifest = {
    units: (Array.isArray(ledger.specificationUnits) ? ledger.specificationUnits : []).map((unit) => ({
      order: unit.order,
      heading: unit.heading,
      destinations: unit.destinations,
    })),
    safety: (Array.isArray(ledger.safetyCrosswalk) ? ledger.safetyCrosswalk : []).map((invariant) => ({
      id: invariant.id,
      destination: invariant.destination,
      requiredExcerpt: invariant.requiredExcerpt,
    })),
  };
  const preservationHash = createHash("sha256").update(JSON.stringify(preservationManifest)).digest("hex");
  if (preservationHash !== PRESERVATION_MANIFEST_SHA256) {
    errors.push("migration preservation excerpts do not match the frozen manifest");
  }
  const dispositions = new Set(["Retain", "Move", "Split", "Archive", "Delete"]);
  const authorities = new Set(["canonical", "project", "historical", "infrastructure", "superseded"]);
  const kinds = new Set(["published", "project", "archive", "infrastructure", "website"]);
  for (const [source, expectedBlob] of BASELINE_ARTIFACTS) {
    const artifact = bySource.get(source);
    if (!artifact) continue;
    if (artifact.blob !== expectedBlob) errors.push(`${source}: frozen baseline blob does not match`);
    if (!dispositions.has(artifact.disposition)) errors.push(`${source}: unknown disposition ${artifact.disposition}`);
    if (!authorities.has(artifact.authority)) errors.push(`${source}: unknown authority ${artifact.authority}`);
    if (!kinds.has(artifact.kind)) errors.push(`${source}: unknown artifact kind ${artifact.kind}`);
    const hasEvidence = typeof artifact.evidence === "string" && artifact.evidence.trim() !== "";
    if (!hasEvidence) errors.push(`${source}: evidence is required`);
    const destinations = Array.isArray(artifact.destinations) ? artifact.destinations : [];
    if (artifact.disposition === "Retain" && (destinations.length !== 1 || destinations[0] !== source)) {
      errors.push(`${source}: Retain requires the unchanged path as its destination`);
    }
    if (["Move", "Archive"].includes(artifact.disposition) && destinations.length !== 1) {
      errors.push(`${source}: ${artifact.disposition} requires exactly one destination`);
    }
    if (artifact.disposition === "Split" && destinations.length < 1) errors.push(`${source}: Split requires destinations`);
    if (artifact.disposition === "Archive" && !destinations[0]?.startsWith("docs/archive/")) {
      errors.push(`${source}: Archive destination must be under docs/archive/`);
    }
    if (artifact.disposition === "Delete" &&
        (typeof artifact.replacement !== "string" || artifact.replacement === "" || !hasEvidence)) {
      errors.push(`${source}: Delete requires a replacement and evidence`);
    }
    const authorityRoot = new Map([
      ["canonical", "docs/src/"],
      ["project", "docs/project/"],
      ["historical", "docs/archive/"],
    ]).get(artifact.authority);
    if (authorityRoot && destinations.some((destination) => !destination.startsWith(authorityRoot))) {
      errors.push(`${source}: ${artifact.authority} destination must be under ${authorityRoot}`);
    }
    for (const destination of destinations) {
      if (!existingPaths.has(destination)) errors.push(`${source}: destination does not exist: ${destination}`);
    }
    if (artifact.replacement && !existingPaths.has(artifact.replacement)) {
      errors.push(`${source}: replacement does not exist: ${artifact.replacement}`);
    }
    if (artifact.disposition !== "Retain" && currentPaths.has(source)) {
      errors.push(`${source}: migrated source still exists`);
    }
  }

  const units = Array.isArray(ledger.specificationUnits) ? ledger.specificationUnits : [];
  const unitHeadings = new Set(units.map((unit) => unit.heading));
  if (units.length !== SPECIFICATION_HEADINGS.length || unitHeadings.size !== SPECIFICATION_HEADINGS.length) {
    errors.push(`specification unit coverage must contain exactly ${SPECIFICATION_HEADINGS.length} unique H2 units`);
  }
  for (const [index, heading] of SPECIFICATION_HEADINGS.entries()) {
    const unit = units[index];
    if (!unit || unit.order !== index + 1 || unit.heading !== heading) {
      errors.push(`specification unit ${index + 1} must be ${heading}`);
      continue;
    }
    const destinations = Array.isArray(unit.destinations) ? unit.destinations : [];
    if (destinations.length === 0) errors.push(`${heading}: at least one preservation destination is required`);
    if (heading === "Table of Contents" &&
        (destinations.length !== 1 || destinations[0]?.path !== "docs/src/SUMMARY.md")) {
      errors.push("Table of Contents must map to docs/src/SUMMARY.md");
    }
    for (const destination of destinations) {
      if (!existingPaths.has(destination.path)) {
        errors.push(`${heading}: destination does not exist: ${destination.path}`);
        continue;
      }
      if (typeof destination.requiredExcerpt !== "string" || destination.requiredExcerpt.trim() === "") {
        errors.push(`${heading}: normalized required excerpt is required for ${destination.path}`);
        continue;
      }
      const contents = textFiles.get(destination.path);
      if (typeof contents !== "string" || !normalizePreservedText(contents).includes(normalizePreservedText(destination.requiredExcerpt))) {
        errors.push(`${heading}: required excerpt is missing from ${destination.path}`);
      }
    }
    if (typeof unit.evidence !== "string" || unit.evidence.trim() === "") errors.push(`${heading}: preservation evidence is required`);
  }

  const plans = Array.isArray(ledger.plans) ? ledger.plans : [];
  const expectedPlanIds = Array.from({ length: 27 }, (_, index) => String(index + 1).padStart(3, "0"));
  if (plans.length !== expectedPlanIds.length || plans.some((plan, index) => plan.id !== expectedPlanIds[index])) {
    errors.push("plan IDs must be contiguous from 001 through 027");
  }
  for (const plan of plans) {
    if (plan.completion !== "Complete" || plan.lifecycle !== "Archived") {
      errors.push(`plan ${plan.id} must be Complete and Archived`);
    }
    if (plan.destination !== `docs/archive/build-plans/plan-${plan.id}.md`) {
      errors.push(`plan ${plan.id} has an invalid archive destination`);
    } else if (!existingPaths.has(plan.destination)) {
      errors.push(`plan ${plan.id}: destination does not exist`);
    }
    if (!Array.isArray(plan.specs) || plan.specs.length === 0 ||
        plan.specs.some((spec) => !/^specs\/\d{3}-[a-z0-9-]+$/u.test(spec) || !existingPaths.has(spec)) ||
        typeof plan.evidence !== "string" || plan.evidence.trim() === "" ||
        !DELIVERY_EVIDENCE.test(plan.evidence)) {
      errors.push(`plan ${plan.id} requires spec and delivery evidence`);
    }
  }

  const postBaselinePlans = Array.isArray(ledger.postBaselinePlans) ? ledger.postBaselinePlans : [];
  const postIds = new Set(postBaselinePlans.map((plan) => plan.id));
  if (postBaselinePlans.length === 0 || postIds.size !== postBaselinePlans.length ||
      postBaselinePlans.some((plan, index) => !/^\d{3}$/u.test(plan.id) || Number(plan.id) !== 28 + index)) {
    errors.push("postBaselinePlans must be unique and contiguous from 028");
  }
  if (postBaselinePlans.filter((plan) => plan.lifecycle === "Active").length > 1) {
    errors.push("postBaselinePlans may contain at most one Active plan");
  }
  const activePlanIndex = postBaselinePlans.findIndex((plan) => plan.lifecycle === "Active");
  if (activePlanIndex >= 0 &&
      (activePlanIndex !== postBaselinePlans.length - 1 ||
       postBaselinePlans.slice(0, activePlanIndex).some((plan) => plan.lifecycle !== "Archived"))) {
    errors.push("Active post-baseline plan must be the final entry after Archived predecessors");
  }
  for (const plan of postBaselinePlans) {
    const activePath = `docs/project/build-plans/plan-${plan.id}.md`;
    const archivePath = `docs/archive/build-plans/plan-${plan.id}.md`;
    const validSpecs = Array.isArray(plan.specs) && plan.specs.length > 0 &&
      plan.specs.every((spec) => /^specs\/\d{3}-[a-z0-9-]+$/u.test(spec) && existingPaths.has(spec));
    if (!validSpecs || typeof plan.evidence !== "string" || plan.evidence.trim() === "") {
      errors.push(`post-baseline plan ${plan.id} requires spec and evidence`);
    }
    if (plan.lifecycle === "Active") {
      if (plan.completion !== "In Progress" || plan.destination !== activePath ||
          !/\bissue #\d+\b/iu.test(plan.evidence) || !existingPaths.has(activePath) || currentPaths.has(archivePath)) {
        errors.push(`post-baseline plan ${plan.id} has an invalid Active lifecycle`);
      }
    } else if (plan.lifecycle === "Archived") {
      const contents = textFiles.get(archivePath) ?? "";
      if (plan.completion !== "Complete" || plan.destination !== archivePath || !DELIVERY_EVIDENCE.test(plan.evidence) ||
          !existingPaths.has(archivePath) || currentPaths.has(activePath) ||
          !/^#\s+\S/mu.test(contents) || contents.trim().split(/\r?\n/gu).length < 3) {
        errors.push(`post-baseline plan ${plan.id} has an invalid Archived lifecycle`);
      }
    } else {
      errors.push(`post-baseline plan ${plan.id} has an unknown lifecycle`);
    }
  }

  const invariants = Array.isArray(ledger.safetyCrosswalk) ? ledger.safetyCrosswalk : [];
  const invariantIds = new Set(invariants.map((invariant) => invariant.id));
  if (invariants.length !== SAFETY_INVARIANTS.size ||
      [...SAFETY_INVARIANTS].some((id) => !invariantIds.has(id))) {
    errors.push(`safety crosswalk must contain the six required invariants`);
  }
  for (const invariant of invariants) {
    if (!existingPaths.has(invariant.destination)) errors.push(`${invariant.id}: destination does not exist`);
    if (typeof invariant.evidence !== "string" || invariant.evidence.trim() === "") {
      errors.push(`${invariant.id}: preservation evidence is required`);
    }
    if (typeof invariant.requiredExcerpt !== "string" || invariant.requiredExcerpt.trim() === "") {
      errors.push(`${invariant.id}: normalized safety excerpt is required`);
    } else {
      const contents = textFiles.get(invariant.destination);
      if (typeof contents !== "string" || !normalizePreservedText(contents).includes(normalizePreservedText(invariant.requiredExcerpt))) {
        errors.push(`${invariant.id}: safety excerpt is missing from ${invariant.destination}`);
      }
    }
  }

  const staleLiterals = Array.isArray(ledger.staleLiterals) ? ledger.staleLiterals : [];
  if (staleLiterals.length !== LEGACY_LITERALS.size ||
      [...LEGACY_LITERALS].some((literal) => !staleLiterals.includes(literal))) {
    errors.push("staleLiterals must enumerate every retired live path exactly once");
  }
  const exceptionKeys = new Set();
  for (const exception of ledger.historicalExceptions ?? []) {
    const key = `${exception.file}\u0000${exception.literal}`;
    if (exceptionKeys.has(key)) errors.push(`duplicate historical exception: ${exception.file} ${exception.literal}`);
    exceptionKeys.add(key);
    if (!exception.file || !exception.literal || /[*?\[\]]/u.test(exception.file) || /[*?\[\]]/u.test(exception.literal) ||
        !LEGACY_LITERALS.has(exception.literal) || !Number.isInteger(exception.occurrences) || exception.occurrences < 1 ||
        typeof exception.reason !== "string" || exception.reason.trim() === "") {
      errors.push(`historical exception must be exact: ${exception.file ?? "<missing>"}`);
    }
  }
  if (exceptionKeys.size !== HISTORICAL_EXCEPTIONS.size ||
      [...HISTORICAL_EXCEPTIONS].some(([key, occurrences]) =>
        !exceptionKeys.has(key) || ledger.historicalExceptions.find((entry) => `${entry.file}\u0000${entry.literal}` === key)?.occurrences !== occurrences)) {
    errors.push("historicalExceptions must match the approved CHANGELOG exceptions");
  }
  return errors;
}

function normalizePreservedText(value) {
  return value.normalize("NFKC").toLocaleLowerCase("en-US").replace(/\s+/gu, " ").trim();
}

export function validateCorpusSnapshot(ledger, snapshot) {
  const errors = [];
  const textFiles = snapshot?.textFiles ?? new Map();
  const currentPaths = snapshot?.currentPaths ?? new Set();
  const readme = textFiles.get("README.md");
  if (typeof readme !== "string") {
    errors.push("README.md is missing from the corpus snapshot");
  } else {
    const lines = readme === "" ? 0 : readme.replace(/\n$/u, "").split("\n").length;
    if (lines > 120) errors.push(`README.md exceeds 120 lines (${lines})`);
    for (const link of markdownLinks(readme)) {
      if (!link.image && !isExternal(link.destination)) {
        errors.push(`README.md package copy requires an absolute link: ${link.destination}`);
      }
    }
  }

  const summary = textFiles.get("docs/src/SUMMARY.md") ?? "";
  for (const link of markdownLinks(summary)) {
    if (isExternal(link.destination)) continue;
    const { pathname } = splitTarget(link.destination);
    const resolved = path.posix.normalize(path.posix.join("docs/src", pathname));
    if (pathname && !resolved.startsWith("docs/src/")) {
      errors.push(`published navigation crosses the docs/src boundary: ${link.destination}`);
    }
  }

  const bookConfig = textFiles.get("docs/book.toml") ?? "";
  if (tomlStringValue(bookConfig, "book", "src") !== "src") {
    errors.push('docs/book.toml must keep [book] src = "src"');
  }

  const postBaselinePlans = Array.isArray(ledger.postBaselinePlans) ? ledger.postBaselinePlans : [];
  const archivedPlans = [...currentPaths].filter((name) => /^docs\/archive\/build-plans\/plan-\d{3}\.md$/u.test(name));
  const expectedArchivedPlans = new Set([
    ...(ledger.plans?.map((plan) => plan.destination) ?? []),
    ...postBaselinePlans.filter((plan) => plan.lifecycle === "Archived").map((plan) => plan.destination),
  ]);
  if (archivedPlans.length !== expectedArchivedPlans.size || archivedPlans.some((name) => !expectedArchivedPlans.has(name))) {
    errors.push("archive plan files must match the declared completed-plan lifecycle");
  }
  const activePlans = [...currentPaths].filter((name) => /^docs\/project\/build-plans\/plan-\d{3}\.md$/u.test(name));
  const expectedActivePlans = postBaselinePlans.filter((plan) => plan.lifecycle === "Active").map((plan) => plan.destination);
  if (activePlans.length !== expectedActivePlans.length || activePlans.some((name) => !expectedActivePlans.includes(name))) {
    errors.push("active project build plan must match the ledger lifecycle declaration");
  }
  const currentPlanIndex = textFiles.get("docs/project/build-plans/README.md") ?? "";
  const archivePlanIndex = textFiles.get("docs/archive/build-plans/README.md") ?? "";
  for (const plan of postBaselinePlans) {
    if (plan.lifecycle === "Active" && !hasPlanIndexRow(currentPlanIndex, plan.id, "Active")) {
      errors.push(`post-baseline plan ${plan.id} is missing its Active index row`);
    }
    if (plan.lifecycle === "Archived" &&
        (!hasPlanIndexRow(archivePlanIndex, plan.id, "Complete, Archived") ||
         !hasConcreteEvidenceRow(archivePlanIndex, plan.id))) {
      errors.push(`post-baseline plan ${plan.id} is missing its Complete, Archived evidence row`);
    }
  }

  const archiveUltimatePath = "docs/archive/website/ultimate-resource-meter.md";
  const archiveUltimate = textFiles.get(archiveUltimatePath) ?? "";
  if (!hasMarkdownLinkTo(archiveUltimate, archiveUltimatePath, "docs/src/features/ultimate-resource.md")) {
    errors.push("archived Ultimate article must link its canonical replacement");
  }
  const lifecycleMap = textFiles.get("docs/README.md") ?? "";
  for (const required of ["docs/project/migration-ledger.md", "docs/archive/website/ultimate-resource-meter.md"]) {
    if (!hasMarkdownLinkTo(lifecycleMap, "docs/README.md", required)) {
      errors.push(`docs/README.md must discover ${required.slice("docs/".length)}`);
    }
  }

  const exceptionMap = new Map((ledger.historicalExceptions ?? []).map((entry) => [`${entry.file}\u0000${entry.literal}`, entry]));
  for (const literal of ledger.staleLiterals ?? []) {
    for (const [file, contents] of textFiles) {
      if (file.startsWith("docs/archive/") || file.startsWith("specs/") ||
          [
            ".github/scripts/docs-policy.mjs",
            ".github/scripts/docs-policy.test.mjs",
            "docs/project/migration-ledger.json",
            "docs/project/migration-ledger.md",
          ].includes(file)) continue;
      const occurrences = contents.split(literal).length - 1;
      if (occurrences === 0) continue;
      const exception = exceptionMap.get(`${file}\u0000${literal}`);
      if (!exception) errors.push(`${file}: stale live documentation path ${literal}`);
      else if (occurrences !== exception.occurrences) {
        errors.push(`${file}: historical exception occurrence mismatch for ${literal}`);
      }
    }
  }
  for (const exception of ledger.historicalExceptions ?? []) {
    const contents = textFiles.get(exception.file);
    if (contents === undefined) continue;
    const occurrences = contents.split(exception.literal).length - 1;
    if (occurrences !== exception.occurrences) {
      errors.push(`${exception.file}: historical exception occurrence mismatch for ${exception.literal}`);
    }
  }
  return [...new Set(errors)];
}

function hasMarkdownLinkTo(contents, source, expectedTarget) {
  const withoutComments = contents.replace(/<!--[\s\S]*?-->/gu, "");
  return markdownLinks(withoutComments).some((link) => {
    if (link.image || isExternal(link.destination)) return false;
    const { pathname } = splitTarget(link.destination);
    const resolved = path.posix.normalize(path.posix.join(path.posix.dirname(source), pathname));
    return resolved === expectedTarget;
  });
}

function hasPlanIndexRow(contents, id, status) {
  contents = visibleMarkdown(contents);
  const row = new RegExp(
    `^\\|\\s*\\[(?:plan-)?${escapeRegExp(id)}(?:\\.md)?\\]\\(plan-${escapeRegExp(id)}\\.md\\)\\s*\\|\\s*${escapeRegExp(status)}\\s*\\|`,
    "mu",
  );
  return row.test(contents);
}

function hasConcreteEvidenceRow(contents, id) {
  const row = visibleMarkdown(contents).split(/\r?\n/gu).find((line) =>
    new RegExp(`^\\|\\s*\\[(?:plan-)?${escapeRegExp(id)}(?:\\.md)?\\]\\(plan-${escapeRegExp(id)}\\.md\\)`, "u").test(line));
  return typeof row === "string" && /(?:\/pull\/\d+|\/releases\/tag\/v\d+\.\d+\.\d+|\/issues\/\d+|\bcommit [0-9a-f]{7,40}\b)/iu.test(row);
}

function visibleMarkdown(contents) {
  return maskMarkdownCode(contents).replace(/<!--[\s\S]*?-->/gu, "");
}

function tomlStringValue(contents, requestedSection, requestedKey) {
  let section = "";
  for (const rawLine of contents.split(/\r?\n/gu)) {
    const line = rawLine.replace(/\s+#.*$/u, "").trim();
    const sectionMatch = line.match(/^\[([^\]]+)\]$/u);
    if (sectionMatch) {
      section = sectionMatch[1];
      continue;
    }
    if (section !== requestedSection) continue;
    const valueMatch = line.match(new RegExp(`^${escapeRegExp(requestedKey)}\\s*=\\s*["']([^"']*)["']$`, "u"));
    if (valueMatch) return valueMatch[1];
  }
  return undefined;
}

export async function validateCorpusRepository(repositoryRoot, ledger) {
  const errors = [];
  const existingPaths = new Set(["README.md"]);
  for (const rootName of ["docs", "website"]) {
    const root = path.join(repositoryRoot, rootName);
    try {
      for (const file of await walk(root)) existingPaths.add(slash(path.relative(repositoryRoot, file)));
    } catch {
      // A removed legacy root is a valid migration result.
    }
  }
  try {
    for (const entry of await readdir(path.join(repositoryRoot, "specs"), { withFileTypes: true })) {
      if (entry.isDirectory()) existingPaths.add(`specs/${entry.name}`);
    }
  } catch {
    errors.push("specs directory is missing");
  }

  const textFiles = new Map();
  const textExtensions = new Set([".c", ".css", ".h", ".html", ".js", ".json", ".lua", ".md", ".mjs", ".ps1", ".rs", ".sh", ".toml", ".txt", ".yml", ".yaml"]);
  const scanRoots = [".github", ".specify", "addon", "assets", "docs", "packaging", "scripts", "specs", "src", "tests", "website", "wix"];
  const candidates = [
    "Cargo.toml",
    "CHANGELOG.md",
    "CLAUDE.md",
    "CONTRIBUTING.md",
    "README.md",
    "release.toml",
    "rust-toolchain.toml",
  ].map((name) => path.join(repositoryRoot, name));
  for (const rootName of scanRoots) {
    try {
      candidates.push(...(await walk(path.join(repositoryRoot, rootName))));
    } catch {
      // Optional or retired roots do not invalidate the corpus by themselves.
    }
  }
  for (const file of candidates) {
    if (!textExtensions.has(path.extname(file).toLocaleLowerCase("en-US"))) continue;
    const relative = slash(path.relative(repositoryRoot, file));
    const bytes = await readFile(file);
    const hygieneErrors = validateTextHygiene(relative, bytes);
    errors.push(...hygieneErrors);
    if (hygieneErrors.some((error) => error.endsWith("text is not valid UTF-8"))) continue;
    const contents = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
    textFiles.set(relative, contents);
  }

  const snapshot = { existingPaths, currentPaths: existingPaths, textFiles };
  errors.push(...validateMigrationLedger(ledger, snapshot));
  errors.push(...validateCorpusSnapshot(ledger, snapshot));
  return errors;
}

export function validateTextHygiene(relative, bytes) {
  const errors = [];
  let contents;
  try {
    contents = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
  } catch {
    return [`${relative}: text is not valid UTF-8`];
  }
  if (bytes.length >= 3 && bytes[0] === 0xef && bytes[1] === 0xbb && bytes[2] === 0xbf) {
    errors.push(`${relative}: UTF-8 BOM is forbidden`);
  }
  if (contents.includes("\r")) errors.push(`${relative}: text must use LF line endings`);
  if (/(?:\uFFFD|\u00c3.|\u00e2\u20ac|\u00ef\u00bb\u00bf)/u.test(contents)) {
    errors.push(`${relative}: possible mojibake detected`);
  }
  if (/[\u2013\u2014]/u.test(contents)) errors.push(`${relative}: forbidden dash character`);
  return errors;
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
  const installStep = yamlListBlock(build, "Install pinned documentation tools");
  const spellingStep = yamlListBlock(build, "Check spelling");
  const typosInstall = /^\s*(?:run:\s*)?cargo install typos-cli --version ['"]?=1\.50\.1['"]? --locked\s*$/mu;
  const spellingGate = /^\s*run:\s*typos docs\/src docs\/README\.md README\.md\s*$/mu;
  if (!typosInstall.test(installStep)) {
    errors.push("build job must install pinned typos-cli 1.50.1 with --locked");
  }
  if (!spellingGate.test(spellingStep)) {
    errors.push("build job requires the exact canonical-documentation spelling gate");
  }
  if (installStep && spellingStep && build.indexOf(installStep) > build.indexOf(spellingStep)) {
    errors.push("build job must install pinned typos-cli before running the spelling gate");
  }
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

export function validateDocumentationAuthorityTriggers(workflow) {
  const errors = [];
  const triggerText = workflow.split(/^permissions:/mu)[0] ?? workflow;
  const triggers = yamlBlocks(triggerText, 2);
  for (const [slice, authority] of [
    ["S080", "Cargo.toml"],
    ["S080", "CHANGELOG.md"],
    ["S081", "assets/eso-weave-banner.png"],
    ["S081", "assets/brand/eso-weave-mark.svg"],
    ["S081", "assets/brand/eso-weave-glyph.svg"],
  ]) {
    const missing = [];
    for (const event of ["push", "pull_request"]) {
      const block = triggers.get(event) ?? "";
      const path = new RegExp(`^\\s+- ["']?${escapeRegExp(authority)}["']?\\s*$`, "mu");
      if (!path.test(block)) missing.push(event);
    }
    if (missing.length > 0) {
      errors.push(`${slice} documentation workflow must include ${authority} in ${missing.join(" and ")} paths`);
    }
  }
  return errors;
}

export function validateCatalogCandidateWorkflow(text) {
  const errors = [];
  if (!/^  workflow_dispatch:\s*$/mu.test(text) || !/^  schedule:\s*$/mu.test(text)) {
    errors.push("catalog candidate workflow requires manual and scheduled triggers");
  }
  if (!hasExactPermissions(text, 0, new Map([["contents", "read"]]))) {
    errors.push("catalog candidate workflow requires only contents: read");
  }
  if (!/^    timeout-minutes: 30\s*$/mu.test(text)) {
    errors.push("catalog candidate workflow requires a 30 minute job timeout");
  }
  for (const action of ["actions/checkout", "actions/upload-artifact"]) {
    const use = new RegExp(`uses:\\s*${escapeRegExp(action)}@([^\\s#]+)`, "gu");
    const matches = [...text.matchAll(use)];
    if (matches.length === 0 || matches.some((match) => !/^[0-9a-f]{40}$/u.test(match[1]))) {
      errors.push(`${action} must use an exact commit SHA`);
    }
  }
  for (const required of ["pipeline-build", "pipeline-verify", "if-no-files-found: error"]) {
    if (!text.includes(required)) errors.push(`catalog candidate workflow requires ${required}`);
  }
  for (const forbidden of [
    "contents: write", "pull-requests: write", "git push", "gh pr", "gh release",
    "cargo release", "pipeline-promote", "pipeline-install",
  ]) {
    if (text.includes(forbidden)) errors.push(`catalog candidate workflow forbids ${forbidden}`);
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
  const repositoryRoot = path.dirname(docsRoot);
  const outputRoot = path.resolve(process.argv[3] ?? path.join("target", "docs-site", "html"));
  const workflowPath = path.resolve(".github", "workflows", "docs.yml");
  const catalogWorkflowPath = path.resolve(".github", "workflows", "catalog-candidate.yml");
  const cssPath = path.join(docsRoot, "theme", "eso-weave.css");
  const ledgerPath = path.join(docsRoot, "project", "migration-ledger.json");
  const coveragePath = path.join(docsRoot, "project", "content-coverage.json");
  const catalogPath = path.join(docsRoot, "project", "catalog-sources.json");
  const encounterModelPath = path.join(docsRoot, "project", "encounter-model.json");
  const screenshotManifestPath = path.join(docsRoot, "project", "documentation-screenshots.json");
  const ledger = JSON.parse(await readFile(ledgerPath, "utf8"));
  const coverage = JSON.parse(await readFile(coveragePath, "utf8"));
  const catalog = JSON.parse(await readFile(catalogPath, "utf8"));
  const encounterModel = JSON.parse(await readFile(encounterModelPath, "utf8"));
  const screenshotManifest = JSON.parse(await readFile(screenshotManifestPath, "utf8"));
  const encounterFixturePath = path.join(repositoryRoot, encounterModel.synthetic_fixture.encounter);
  const encounterProjectionPath = path.join(repositoryRoot, encounterModel.synthetic_fixture.projection);
  const encounterFixtureBytes = await readFile(encounterFixturePath);
  const encounterFixture = JSON.parse(encounterFixtureBytes);
  const encounterProjection = JSON.parse(await readFile(encounterProjectionPath, "utf8"));
  const glossary = await readFile(path.join(docsRoot, "src", "reference", "glossary.md"), "utf8");
  const landingMarkdown = await readFile(path.join(docsRoot, "src", "README.md"), "utf8");
  const brandStandardMarkdown = await readFile(path.join(docsRoot, "src", "development", "brand-standard.md"), "utf8");
  const diagramPages = new Map(await Promise.all(DOCUMENTATION_DIAGRAMS.map(async (record) => [
    record.page,
    await readFile(path.join(docsRoot, "src", ...record.page.split("/")), "utf8"),
  ])));
  const diagramSvgs = new Map(await Promise.all(DOCUMENTATION_DIAGRAMS.map(async (record) => [
    record.asset,
    await readFile(path.join(docsRoot, "src", "assets", "diagrams", record.asset), "utf8"),
  ])));
  const screenshotPages = new Map(await Promise.all([...new Set(screenshotManifest.assets.flatMap((record) => record.pages))].map(async (page) => [
    page,
    await readFile(path.join(repositoryRoot, ...page.split("/")), "utf8"),
  ])));
  const screenshotFiles = await walk(path.join(docsRoot, "src", "assets", "screenshots"));
  const screenshotAssets = new Map(await Promise.all([
    ...screenshotFiles.map(async (file) => [slash(path.relative(repositoryRoot, file)), await readFile(file)]),
    ...screenshotManifest.assets.filter((record) => record.kind === "synthetic-illustration").map(async (record) => [
      record.destination,
      await readFile(path.join(repositoryRoot, ...record.destination.split("/"))),
    ]),
  ]));
  const cargoToml = await readFile(path.join(repositoryRoot, "Cargo.toml"), "utf8");
  const changelog = await readFile(path.join(repositoryRoot, "CHANGELOG.md"), "utf8");
  const approvedBanner = await readFile(path.join(repositoryRoot, "assets", "eso-weave-banner.png"));
  const publishedBanner = await readFile(path.join(docsRoot, "src", "assets", "brand", "eso-weave-banner.png"));
  const approvedMark = await readFile(path.join(repositoryRoot, "assets", "brand", "eso-weave-mark.svg"));
  const publishedMark = await readFile(path.join(docsRoot, "src", "assets", "brand", "eso-weave-mark.svg"));
  const approvedGlyph = await readFile(path.join(repositoryRoot, "assets", "brand", "eso-weave-glyph.svg"));
  const publishedGlyph = await readFile(path.join(docsRoot, "src", "assets", "brand", "eso-weave-glyph.svg"));
  const landingHtml = await readFile(path.join(outputRoot, "index.html"), "utf8");
  const brandStandardHtml = await readFile(path.join(outputRoot, "development", "brand-standard.html"), "utf8");
  const generatedDiagramPages = new Map(await Promise.all(DOCUMENTATION_DIAGRAMS.map(async (record) => [
    record.outputPage,
    await readFile(path.join(outputRoot, ...record.outputPage.split("/")), "utf8"),
  ])));
  const generatedDiagramSvgs = new Map(await Promise.all(DOCUMENTATION_DIAGRAMS.map(async (record) => [
    record.asset,
    await readFile(path.join(outputRoot, "assets", "diagrams", record.asset), "utf8"),
  ])));
  const generatedScreenshotPages = new Map(await Promise.all([...new Set(screenshotManifest.assets.flatMap((record) => record.pages))].map(async (page) => {
    const outputPage = page.replace("docs/src/", "").replace(/\.md$/u, ".html");
    return [outputPage, await readFile(path.join(outputRoot, ...outputPage.split("/")), "utf8")];
  })));
  const outputPaths = new Set((await walk(outputRoot)).map((file) => slash(path.relative(outputRoot, file))));
  const sourceMarkdownPages = new Map(await Promise.all((await walk(path.join(docsRoot, "src"), ".md")).map(async (file) => [
    slash(path.relative(path.join(docsRoot, "src"), file)),
    await readFile(file, "utf8"),
  ])));
  const generatedHtmlPages = new Map(await Promise.all((await walk(outputRoot, ".html")).map(async (file) => [
    slash(path.relative(outputRoot, file)),
    await readFile(file, "utf8"),
  ])));
  const css = await readFile(cssPath, "utf8");
  const themeScript = await readFile(path.join(docsRoot, "theme", "eso-weave.js"), "utf8");
  const workflow = await readFile(workflowPath, "utf8");
  const searchIndexFiles = (await readdir(outputRoot)).filter((name) => /^searchindex-[0-9a-f]+\.js$/u.test(name));
  const searchIndex = searchIndexFiles.length === 1
    ? await readFile(path.join(outputRoot, searchIndexFiles[0]), "utf8")
    : "";
  const errors = [
    ...(await validateSourceTree(docsRoot, { completeFenceInventory: true })),
    ...(await validateGeneratedSite(outputRoot)),
    ...validateBrandCss(css),
    ...validateLandingCss(css),
    ...validateBrandStandardVisualCss(css),
    ...validateSyntaxHighlightingCss(css),
    ...validateWorkflowText(workflow),
    ...validateDocumentationAuthorityTriggers(workflow),
    ...validateCatalogCandidateWorkflow(await readFile(catalogWorkflowPath, "utf8")),
    ...(await validateCorpusRepository(repositoryRoot, ledger)),
    ...(await validateContentCoverageRepository(repositoryRoot, coverage)),
    ...validateFormalGlossary(glossary, coverage.search_map),
    ...validateLandingIdentity({ landingMarkdown, cargoToml, changelog, approvedBanner, publishedBanner }),
    ...validateLandingGenerated(landingHtml, outputPaths),
    ...validateBrandStandard({
      markdown: brandStandardMarkdown,
      approvedBanner,
      publishedBanner,
      approvedMark,
      publishedMark,
      approvedGlyph,
      publishedGlyph,
    }),
    ...validateBrandStandardGenerated(brandStandardHtml, outputPaths),
    ...validateDocumentationDiagrams({ pages: diagramPages, svgs: diagramSvgs }),
    ...validateDocumentationDiagramsGenerated(generatedDiagramPages, outputPaths, diagramSvgs, generatedDiagramSvgs, themeScript),
    ...validateDocumentationDiagramCss(css),
    ...validateDocumentationScreenshots({ manifest: screenshotManifest, assets: screenshotAssets, pages: screenshotPages }),
    ...validateDocumentationScreenshotsGenerated({ manifest: screenshotManifest, pages: generatedScreenshotPages, outputPaths }),
    ...validateDocumentationScreenshotCss(css),
    ...validateDocumentationFigureInventory(sourceMarkdownPages),
    ...validateDocumentationFigureJavascript(themeScript),
    ...validateDocumentationFigureCss(css),
    ...validateDocumentationCodeBlocksGenerated(sourceMarkdownPages, generatedHtmlPages, outputPaths),
    ...validateHighlightingExtension(themeScript),
    ...(searchIndexFiles.length === 1 ? [] : ["S079 generated site requires exactly one hashed search index"]),
    ...validateGlossarySearchIndex(searchIndex),
    ...validateCatalogSourceContract(catalog),
    ...validateEncounterModelContract(encounterModel),
    ...validateEncounterEvidence(encounterFixture, encounterProjection, encounterFixtureBytes),
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
