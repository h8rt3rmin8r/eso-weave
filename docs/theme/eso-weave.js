"use strict";

const main = document.querySelector("main");
if (main) {
  main.id = "main-content";
  main.tabIndex = -1;
  const skipLink = document.createElement("a");
  skipLink.className = "eso-skip-link";
  skipLink.href = "#main-content";
  skipLink.textContent = "Skip to main content";
  skipLink.addEventListener("click", () => main.focus({ preventScroll: false }));
  document.body.prepend(skipLink);
}

for (const image of document.querySelectorAll(".docs-flow-diagram .img-wrapper > img")) {
  image.alt = "";
  image.setAttribute("aria-hidden", "true");
}
