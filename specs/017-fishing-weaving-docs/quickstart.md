# Quickstart: Validating the README Documentation

Documentation-only feature; validation is review plus hygiene checks.

## Steps

1. Render `README.md` and confirm the section order top to bottom is: banner and
   badges, Installation, Fishing, Weaving, Disclaimer, License (SC-003).
2. Read the Fishing section and confirm it states the hotkey casts for the player
   (do not cast first), lists the prerequisites (addon installed and enabled and
   not out of date, beacon visible, window focused), shows the status progression,
   names the interact key and configurable timings, and gives troubleshooting for
   the early-stop symptom (SC-001).
3. Read the Weaving section and confirm it states the default global cooldown
   (500 ms), light (50 ms), heavy (1000 ms), and bash (125 ms) delays, that F1
   suspends and resumes and F2 toggles fishing, and that multi-bar weaving is out
   of scope with no dual-bar mechanics documented (SC-002, SC-004).
4. Confirm the documented defaults match the code (fishing arm timeout 8000 ms,
   reel 100 ms, recast 3000 ms, interact key E; weave timings above).
5. Hygiene: confirm the file is UTF-8 without BOM with LF endings, contains no em-
   or en-dashes, and that all links resolve.
