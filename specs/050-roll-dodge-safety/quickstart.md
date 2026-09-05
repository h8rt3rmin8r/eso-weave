# Quickstart: Roll-Dodge Safety

1. Build and run the S050 and full validation suites.
2. Install or update managed PixelBeacon to version 17.
3. Enter a character and confirm Roll dodge reads Inactive.
4. Perform a valid roll and confirm it changes promptly to Active, then Inactive.
5. Hold sprint and attempt a rejected roll; confirm Active clears within 1.5 seconds.
6. Start a weave, roll during its delay, and confirm no later generated skill down
   event occurs while generated input already held is released.
7. Press a bound skill during Active and confirm the physical key is not swallowed
   and no delayed weave fires after recovery.
8. Reload, zone, die, resurrect in place, and stop ESO; confirm death clears stale
   Active, a late fade cannot reopen the gate, and resurrection restores Inactive
   without requiring a zone.
9. Set the fast reader interval above 1,500 ms and confirm interception still
   samples Roll dodge at the safety-capped 375 ms cadence.
10. Temporarily use a version-16 addon and confirm earlier readings work while Roll
   dodge stays Not detected and generated weaves fail closed.

The implementation pull request closes #57 and #60. Sprint detection and
auto-potion sprint deferral remain separate follow-on work.
