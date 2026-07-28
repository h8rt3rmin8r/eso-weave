# Quickstart: Auto-Potion

**Feature**: [spec.md](spec.md) | **Date**: 2026-07-27

## Turning it on

Auto-potion ships **off**, and every resource inside it ships off. Nothing happens
until you enable both the feature and at least one resource.

1. Open Settings and find the Auto-potion group.
2. Tick the resource or resources you want watched and set a threshold for each.
   The rule is an **OR**: it fires when *any* enabled resource is at or below
   *its own* threshold.
3. Set the quickslot key if you rebound it in game (default `Q`).
4. Close Settings and press **F3**, or use the Auto-potion toggle in the app.

It also starts off after every restart, even if it was on when you closed the
app. That is deliberate and is not a bug; see plan.md D5.

## Confirming it without the game

The whole trigger rule is a pure function with a mock sink and a virtual clock,
so it is fully exercised with no game running:

```bash
cargo test --test potion
```

The tests worth reading are the truth-table ones in
[contracts/trigger-rule.md](contracts/trigger-rule.md): each of the seven
conditions failing in isolation with all others satisfied, and each asserting
*which* condition blocked rather than only that nothing fired.

## Confirming it with the game

Stand somewhere safe with a potion in your active quickslot, enable health at a
high threshold (say 90), and take a little damage. The key fires once; the app
logs it at DEBUG.

Things worth checking deliberately, because they are the ones that matter:

- **Open a menu or start typing in chat while a resource is low.** Nothing fires.
- **Press F1 to suspend while a resource is low.** Nothing fires. F3 still
  registers while suspended, but the feature does not act.
- **Reload the UI in game** (`/reloadui`) so the beacon signal drops. Auto-potion
  returns to off rather than firing blind.
- **Empty the quickslot, or put food in it.** Nothing fires.

## What it will not do

- It does not know what your potion restores. The game does not expose that as
  data (only as localized tooltip text), which is why you choose the stats to
  watch instead. Slotting a tri-restoration potion and enabling all three is the
  equivalent.
- It does not pick between potions, queue them, or swap your quickslot.
- It does not fire on a resource it cannot read. An unreadable resource is never
  treated as low, so a beacon outage makes the feature do nothing rather than
  something.

## If it never fires

In order:

- Is the feature on (F3), **and** is at least one resource ticked? Both are
  needed, and both default to off.
- Is a potion actually in the *active* quickslot, and is it a potion rather than
  food?
- Is the app suspended (F1)?
- Is the Quickslot readout in the Status region showing a cooldown and an item, or
  a dash? A dash means the app cannot read the quickslot, and it will never fire
  on an unreadable one.
- Is the addon at version 12 or later? The quickslot blocks this depends on
  arrived with that version.

## If it fires too often

Raise the retry interval. It is the floor on the attempt rate, and it exists
because the quickslot cooldown is read from the screen and does not update until
at least one sampling interval after the key is pressed.
