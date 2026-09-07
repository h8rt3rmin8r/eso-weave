---
title: "Ultimate at a Glance: Exact Charge and Bar-Aware Readiness"
description: "ESO Weave now shows exact Ultimate charge, active-bar cast cost, and readiness without a skill catalogue."
date: "2026-09-07"
author: "The Bushido Collective"
tags: ["eso", "pixelbeacon", "accessibility", "ui"]
category: "Feature"
---

Ultimate is one of the most important combat resources in The Elder Scrolls
Online, but it does not behave like Health, Stamina, or Magicka. It has a stored
point total, a game-reported maximum, and a cast cost that depends on the ability
slotted on each weapon bar. A useful display therefore needs more than one
percentage.

## The Problem

A simple fill bar can show roughly how much Ultimate is stored, but it cannot say
whether the currently slotted ability is castable. Assuming one cost is also
wrong because front and back bars can hold different Ultimate abilities. Keeping
a table of ability IDs and costs would age poorly as the game changes and would
miss live cost modifiers.

Rounded percentage telemetry creates another subtle failure. Two different point
values can round to the same percentage, which can make a threshold comparison
claim Ready too early or too late. It also cannot reproduce an exact readout such
as `185/500`.

## The Solution

PixelBeacon now asks ESO directly for current and maximum Ultimate and for the
effective Ultimate cost on both primary and backup hotbars. Protocol version 5
publishes those four exact values. Each value keeps the pixel bus marker and
checksum protections, and older protocol layouts remain bounded to their original
payload lengths.

ESO Weave stores the resulting aggregate separately from Health, Stamina, and
Magicka. That boundary is deliberate: Ultimate is display-only and cannot affect
auto-potion, weaving, or any input gate.

### Key Capabilities

- A fourth purple Live HUD meter shows exact current and maximum Ultimate.
- Every resource meter includes subtle 25, 50, and 75 percent landmarks.
- A stronger tick shows the active bar's exact cast threshold and protrudes below
  the track so it remains distinct when it overlaps a quarter mark.
- Green Ready text appears at or above the exact cost.
- The numeric and Ready regions are always reserved, so state changes do not move
  nearby content.
- Screen-reader information includes exact charge, maximum, active bar, cost, and
  readiness without relying on color.

## Getting Started

Install or update PixelBeacon from System and State, then use `/reloadui` in ESO
and restart ESO Weave so both sides negotiate protocol version 5. The Ultimate row
appears under Magicka. Swap weapon bars to see the threshold follow each slotted
ability. If a bar has no Ultimate or the active bar cannot be identified, the
threshold and Ready state stay hidden instead of guessing.

## What's Next

Repository tests cover encoding, corruption, loss, bar selection, layout, color,
and accessibility. A separate release-verification pass will exercise the feature
inside ESO with unequal bar costs, live modifiers, loading transitions, and both
themes before the containing release is considered field-verified.
