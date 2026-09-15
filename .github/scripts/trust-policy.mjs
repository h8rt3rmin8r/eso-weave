import fs from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

const EXACT_COMMIT = /^[0-9a-f]{40}$/u;
const VERSION_COMMENT = /^v\d+(?:\.\d+){0,2}(?:[-+][0-9A-Za-z.-]+)?$/u;
const PROHIBITED_TRIGGERS = [
  "pull_request_target",
  "issue_comment",
  "workflow_run",
  "repository_dispatch",
];
const ALLOWED_WRITES = new Map([
  [".github/workflows/codeql.yml", new Map([["security-events", "analyze"]])],
  [".github/workflows/docs.yml", new Map([["id-token", "deploy"], ["pages", "deploy"]])],
  [".github/workflows/release.yml", new Map([["contents", "release"]])],
]);
const AUTHORITY_CHANNELS = new Map([
  ["direct-operator", { isAuthority: true, mayExpandScope: true }],
  ["protected-base-policy", { isAuthority: true, mayExpandScope: false }],
]);

function normalizePath(filePath) {
  return filePath.replaceAll("\\", "/");
}

function leadingSpaces(line) {
  return line.length - line.trimStart().length;
}

function checkoutStep(lines, index) {
  const indent = leadingSpaces(lines[index]);
  let end = lines.length;

  for (let next = index + 1; next < lines.length; next += 1) {
    if (/^\s*-\s+(?:name|uses):/u.test(lines[next]) && leadingSpaces(lines[next]) <= indent) {
      end = next;
      break;
    }
  }

  return lines.slice(index, end).join("\n");
}

function owningJob(lines, index) {
  const jobsIndex = lines.findIndex((line) => line === "jobs:");
  if (jobsIndex < 0 || index <= jobsIndex) {
    return null;
  }

  for (let candidate = index; candidate > jobsIndex; candidate -= 1) {
    const match = lines[candidate].match(/^ {2}([A-Za-z0-9_-]+):\s*$/u);
    if (match) {
      return match[1];
    }
  }
  return null;
}

function validateReleaseWorkflow(text, errors) {
  const digestMatch = text.match(/^\s*APPIMAGETOOL_SHA256:\s*([0-9a-f]+)\s*$/mu);
  if (!digestMatch || digestMatch[1].length !== 64) {
    errors.push("release workflow requires one fixed 64-character AppImage tool digest");
  }

  const downloadIndex = text.indexOf("curl -fsSL -o appimagetool");
  const verifyIndex = text.indexOf("sha256sum --check");
  const executeIndex = text.indexOf("chmod +x appimagetool");
  if (downloadIndex < 0 || verifyIndex < downloadIndex || (executeIndex >= 0 && verifyIndex > executeIndex)) {
    errors.push("release workflow requires digest verification after download and before execution");
  }

  if (!/cargo install cargo-wix --version '=\d+\.\d+\.\d+' --locked/u.test(text)) {
    errors.push("release workflow requires a cargo-wix exact version");
  }
  if (!/cargo install cargo-deb --version '=\d+\.\d+\.\d+' --locked/u.test(text)) {
    errors.push("release workflow requires a cargo-deb exact version");
  }
}

export function classifyAuthorityChannel(channel) {
  return AUTHORITY_CHANNELS.get(channel) ?? { isAuthority: false, mayExpandScope: false };
}

export function validateWorkflow(filePath, text) {
  const normalized = normalizePath(filePath);
  const errors = [];
  const lines = text.split(/\r?\n/u);

  const topPermissionsIndex = lines.findIndex((line) => line === "permissions:");
  const topPermissionLines = [];
  if (topPermissionsIndex >= 0) {
    for (let index = topPermissionsIndex + 1; index < lines.length; index += 1) {
      if (!/^ {2}\S/u.test(lines[index])) {
        break;
      }
      topPermissionLines.push(lines[index]);
    }
  }
  if (!topPermissionLines.some((line) => /^ {2}contents:\s*read\s*$/u.test(line))) {
    errors.push(`${normalized}: workflow requires top-level contents: read`);
  }
  if (topPermissionLines.some((line) => /:\s*write\s*$/u.test(line))) {
    errors.push(`${normalized}: top-level workflow permissions must remain read-only`);
  }

  for (const trigger of PROHIBITED_TRIGGERS) {
    const triggerPattern = new RegExp(`(?:^|[\\s[,] )${trigger}(?=[:\\s,\\]])`, "mu");
    if (triggerPattern.test(text)) {
      errors.push(`${normalized}: prohibited trigger ${trigger}`);
    }
  }

  for (const [index, line] of lines.entries()) {
    const actionMatch = line.match(/^\s*-?\s*uses:\s*([^\s@]+)@([^\s#]+)(?:\s+#\s*(\S.*))?\s*$/u);
    if (!actionMatch) {
      continue;
    }

    const [, action, revision, comment = ""] = actionMatch;
    if (action.startsWith("./") || action.startsWith("docker://")) {
      continue;
    }
    if (!EXACT_COMMIT.test(revision)) {
      errors.push(`${normalized}:${index + 1}: remote Action requires an exact 40-character commit SHA`);
    }
    if (!VERSION_COMMENT.test(comment.trim())) {
      errors.push(`${normalized}:${index + 1}: remote Action requires a version comment such as # v1.2.3`);
    }
    if (action.toLocaleLowerCase("en-US") === "actions/checkout") {
      const step = checkoutStep(lines, index);
      if (!/^\s*persist-credentials:\s*false\s*$/mu.test(step)) {
        errors.push(`${normalized}:${index + 1}: checkout requires persist-credentials: false`);
      }
    }
  }

  const allowedWrites = ALLOWED_WRITES.get(normalized) ?? new Map();
  for (const [index, line] of lines.entries()) {
    const writeMatch = line.match(/^\s+([a-z-]+):\s*write\s*$/u);
    if (writeMatch) {
      const requiredJob = allowedWrites.get(writeMatch[1]);
      const actualJob = owningJob(lines, index);
      if (!requiredJob || actualJob !== requiredJob) {
        errors.push(`${normalized}:${index + 1}: unexpected ${writeMatch[1]} write permission`);
      }
    }
  }

  for (const [index, line] of lines.entries()) {
    for (const match of line.matchAll(/secrets\.([A-Za-z0-9_]+)/gu)) {
      const allowedReleaseToken = normalized === ".github/workflows/release.yml"
        && match[1] === "GITHUB_TOKEN"
        && owningJob(lines, index) === "release";
      if (!allowedReleaseToken) {
        errors.push(`${normalized}:${index + 1}: unexpected secret reference ${match[1]}`);
      }
    }
  }

  if (normalized === ".github/workflows/release.yml") {
    validateReleaseWorkflow(text, errors);
  }

  return errors;
}

export function validateSkillMetadata(filePath, text) {
  const normalized = normalizePath(filePath);
  const frontMatterEnd = text.startsWith("---") ? text.indexOf("\n---", 3) : -1;
  const metadata = frontMatterEnd >= 0 ? text.slice(0, frontMatterEnd + 4) : text.slice(0, 1200);
  const highRiskPattern = /\b(?:sandbox escape|unrestricted code execution|av\/?edr evasion|amsi bypass|process injection|credential theft)\b/iu;

  return highRiskPattern.test(metadata)
    ? [`${normalized}: project-local skill metadata declares unrelated high-risk guidance`]
    : [];
}

export function validateAgentGuidance(filePath, text) {
  const normalized = normalizePath(filePath);
  const requirements = [
    ["protected main", /protected `?main`?/iu],
    ["untrusted data", /untrusted data/iu],
    ["cannot authorize", /cannot authorize/iu],
  ];

  return requirements
    .filter(([, pattern]) => !pattern.test(text))
    .map(([description]) => `${normalized}: guidance must state ${description}`);
}

function walkFiles(root, predicate) {
  if (!fs.existsSync(root)) {
    return [];
  }

  const results = [];
  const entries = fs.readdirSync(root, { withFileTypes: true });
  for (const entry of entries) {
    const child = path.join(root, entry.name);
    if (entry.isDirectory()) {
      results.push(...walkFiles(child, predicate));
    } else if (predicate(child)) {
      results.push(child);
    }
  }
  return results;
}

function runRepositoryScan(repositoryRoot) {
  const errors = [];
  const workflowRoot = path.join(repositoryRoot, ".github", "workflows");
  const workflows = walkFiles(workflowRoot, (filePath) => /\.ya?ml$/u.test(filePath)).sort();
  for (const workflowPath of workflows) {
    const relative = normalizePath(path.relative(repositoryRoot, workflowPath));
    errors.push(...validateWorkflow(relative, fs.readFileSync(workflowPath, "utf8")));
  }

  const skillRoot = path.join(repositoryRoot, ".claude", "skills");
  const skills = walkFiles(skillRoot, (filePath) => path.basename(filePath) === "SKILL.md").sort();
  for (const skillPath of skills) {
    const relative = normalizePath(path.relative(repositoryRoot, skillPath));
    errors.push(...validateSkillMetadata(relative, fs.readFileSync(skillPath, "utf8")));
  }

  for (const relative of ["CLAUDE.md", "CONTRIBUTING.md", "docs/project/build-autopilot.md"]) {
    const guidancePath = path.join(repositoryRoot, relative);
    if (!fs.existsSync(guidancePath)) {
      errors.push(`${relative}: required agent guidance is missing`);
    } else {
      errors.push(...validateAgentGuidance(relative, fs.readFileSync(guidancePath, "utf8")));
    }
  }

  if (errors.length > 0) {
    console.error(["Repository trust policy failed:", ...errors.map((error) => `- ${error}`)].join("\n"));
    process.exitCode = 1;
    return;
  }

  console.log(`Repository trust policy passed: ${workflows.length} workflows and ${skills.length} local skills checked.`);
}

const invokedPath = process.argv[1] ? pathToFileURL(process.argv[1]).href : "";
if (import.meta.url === invokedPath) {
  runRepositoryScan(path.resolve(process.argv[2] ?? "."));
}
