# Quickstart: World Transition State

1. Build and run the test suite for S049.
2. Install or update the managed PixelBeacon to version 16.
3. Enter a character and confirm World state becomes Active after the dashboard
   observations appear.
4. Use a wayshrine or door and confirm World state becomes Transitioning before
   the loading interval, then Active after the new scene baseline.
5. Reload the UI and confirm the initial state is not reported as Active before
   player activation.
6. Stop ESO and confirm the row becomes Game not active.
7. Temporarily use a version-15 addon or corrupt B22 in a test sampler and confirm
   the row becomes Not detected without disturbing earlier payloads.

The implementation pull request closes #56 only. Pending recall protection in
#59 remains blocked on its live travel-path matrix.
