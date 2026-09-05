import { pathToFileURL } from "node:url";

const CLOSING_REFERENCE =
  /\b(?:close[sd]?|fix(?:es|ed)?|resolve[sd]?)[ \t]*:?[ \t]+#([1-9]\d*)\b/giu;

const EXEMPT_LABELS = new Set(["dependencies", "skip: issue-link"]);

function normalizedLabels(labels) {
  if (!Array.isArray(labels)) {
    return [];
  }

  return labels
    .filter((label) => typeof label === "string")
    .map((label) => label.trim().toLocaleLowerCase("en-US"));
}

function issueList(issueNumbers) {
  const references = issueNumbers.map((issueNumber) => `#${issueNumber}`);

  if (references.length === 1) {
    return references[0];
  }

  if (references.length === 2) {
    return `${references[0]} and ${references[1]}`;
  }

  return `${references.slice(0, -1).join(", ")}, and ${references.at(-1)}`;
}

export function findClosingIssueNumbers(body = "") {
  if (typeof body !== "string") {
    return [];
  }

  const visibleBody = body.replace(/<!--[\s\S]*?(?:-->|$)/gu, "");
  const issueNumbers = [];
  const seen = new Set();

  for (const match of visibleBody.matchAll(CLOSING_REFERENCE)) {
    const issueNumber = Number.parseInt(match[1], 10);
    if (!seen.has(issueNumber)) {
      seen.add(issueNumber);
      issueNumbers.push(issueNumber);
    }
  }

  return issueNumbers;
}

export function evaluateIssueLinkPolicy({ author = "", body = "", labels = [] } = {}) {
  if (typeof author === "string" && author.toLocaleLowerCase("en-US") === "dependabot[bot]") {
    return {
      closingIssueNumbers: [],
      exempt: true,
      passed: true,
      reason: "Dependabot pull requests are exempt from issue-link enforcement.",
    };
  }

  const exemptionLabel = normalizedLabels(labels).find((label) => EXEMPT_LABELS.has(label));
  if (exemptionLabel !== undefined) {
    return {
      closingIssueNumbers: [],
      exempt: true,
      passed: true,
      reason: `The ${exemptionLabel} label exempts this pull request from issue-link enforcement.`,
    };
  }

  const closingIssueNumbers = findClosingIssueNumbers(body);
  if (closingIssueNumbers.length > 0) {
    return {
      closingIssueNumbers,
      exempt: false,
      passed: true,
      reason: `Found closing references for issues ${issueList(closingIssueNumbers)}.`,
    };
  }

  return {
    closingIssueNumbers: [],
    exempt: false,
    passed: false,
    reason:
      "Add a complete GitHub closing reference such as `Closes #123` to the pull request body. " +
      "Repeat the complete keyword for every issue. If no atomic issue can apply, a maintainer may " +
      "use the documented `skip: issue-link` exemption.",
  };
}

function parseLabels(rawLabels) {
  try {
    const parsed = JSON.parse(rawLabels || "[]");
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}

function runFromEnvironment() {
  const result = evaluateIssueLinkPolicy({
    author: process.env.PR_AUTHOR,
    body: process.env.PR_BODY,
    labels: parseLabels(process.env.PR_LABELS_JSON),
  });

  const prefix = result.passed ? "Issue-link policy passed" : "Issue-link policy failed";
  console.log(`${prefix}: ${result.reason}`);

  if (!result.passed) {
    process.exitCode = 1;
  }
}

const invokedPath = process.argv[1] ? pathToFileURL(process.argv[1]).href : "";
if (import.meta.url === invokedPath) {
  runFromEnvironment();
}
