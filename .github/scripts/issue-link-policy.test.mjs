import assert from "node:assert/strict";
import test from "node:test";

import { evaluateIssueLinkPolicy, findClosingIssueNumbers } from "./issue-link-policy.mjs";

test("accepts every documented GitHub closing keyword variant", () => {
  const body = [
    "close #1",
    "Closes: #2",
    "CLOSED #3",
    "fix #4",
    "Fixes: #5",
    "FIXED #6",
    "resolve #7",
    "Resolves: #8",
    "RESOLVED #9",
  ].join("\n");

  assert.deepEqual(findClosingIssueNumbers(body), [1, 2, 3, 4, 5, 6, 7, 8, 9]);
});

test("returns unique issue numbers in first-seen order", () => {
  assert.deepEqual(
    findClosingIssueNumbers("Closes #45\nFixes #46\nResolved #45\nCloses #47"),
    [45, 46, 47],
  );
});

test("rejects descriptive references and incomplete multi-issue references", () => {
  assert.deepEqual(findClosingIssueNumbers("Related to #45\nCloses #46, #47"), [46]);
});

test("ignores closing references inside pull request template comments", () => {
  const body = [
    "<!--",
    "Example only:",
    "Closes #123",
    "Closes #124",
    "-->",
    "Closes #45",
    "<!-- Closes #46 -->",
  ].join("\n");

  assert.deepEqual(findClosingIssueNumbers(body), [45]);
  assert.deepEqual(findClosingIssueNumbers("<!-- Closes #47"), []);
});

test("rejects zero, negative, cross-repository, and malformed issue references", () => {
  const body = [
    "Closes #0",
    "Closes #-1",
    "Closes owner/repository#2",
    "Closes GH-3",
    "Closes #not-a-number",
    "Closes\n#4",
  ].join("\n");

  assert.deepEqual(findClosingIssueNumbers(body), []);
});

test("passes when at least one closing reference is present", () => {
  assert.deepEqual(
    evaluateIssueLinkPolicy({
      author: "maintainer",
      body: "Closes #45\nCloses #46",
      labels: [],
    }),
    {
      closingIssueNumbers: [45, 46],
      exempt: false,
      passed: true,
      reason: "Found closing references for issues #45 and #46.",
    },
  );
});

test("fails with guidance when no closing reference or exemption exists", () => {
  const result = evaluateIssueLinkPolicy({
    author: "maintainer",
    body: "Related to #45",
    labels: [],
  });

  assert.equal(result.passed, false);
  assert.equal(result.exempt, false);
  assert.deepEqual(result.closingIssueNumbers, []);
  assert.match(result.reason, /Closes #123/);
});

test("exempts Dependabot regardless of body", () => {
  const result = evaluateIssueLinkPolicy({
    author: "dependabot[bot]",
    body: "",
    labels: [],
  });

  assert.equal(result.passed, true);
  assert.equal(result.exempt, true);
  assert.match(result.reason, /Dependabot/);
});

test("exempts dependency-labeled pull requests case-insensitively", () => {
  const result = evaluateIssueLinkPolicy({
    author: "renovate[bot]",
    body: "",
    labels: ["Dependencies"],
  });

  assert.equal(result.passed, true);
  assert.equal(result.exempt, true);
  assert.match(result.reason, /dependencies/);
});

test("exempts explicitly labeled repository administration", () => {
  const result = evaluateIssueLinkPolicy({
    author: "maintainer",
    body: "No atomic issue applies. This rotates repository administration metadata.",
    labels: ["skip: issue-link"],
  });

  assert.equal(result.passed, true);
  assert.equal(result.exempt, true);
  assert.match(result.reason, /skip: issue-link/);
});

test("normalizes missing event values without throwing", () => {
  assert.equal(evaluateIssueLinkPolicy({}).passed, false);
  assert.deepEqual(findClosingIssueNumbers(), []);
});
