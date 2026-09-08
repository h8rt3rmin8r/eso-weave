# Contract: Runtime Behavior Copy

## Menu evidence

Unavailable evidence, including startup, decode failure, and signal loss, means generated input is gated. Only explicit valid gameplay evidence opens the menu gate.

## Adapt to Latency

The control adds a scaled latency allowance to Light Attack and Bash delays. The allowance is bounded to 0 through 300 ms. Higher scaling adds more delay within that cap.

## Live Log level

The selector changes the saved global captured level. Events at or above the level feed the bounded Live Log ring and, when enabled, the file sink. It is not a panel-only presentation filter.
