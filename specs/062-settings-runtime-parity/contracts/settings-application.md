# Contract: Settings Application

| Setting | Persistence | Runtime application | Safety behavior |
| --- | --- | --- | --- |
| Fishing Arm Timeout | Coalesced JSON save | Immediate controller update | Changed requested work stops without input |
| Fishing Reel Delay | Coalesced JSON save | Immediate controller update | Changed requested work stops without input |
| Fishing Recast Delay | Coalesced JSON save | Immediate controller update | Changed requested work stops without input |
| Fishing Interact Key | Coalesced JSON save | Immediate controller update | Changed requested work stops without input |
| Pixel Bus Color Tolerance | Coalesced JSON save | Next worker iteration after wake | Prior safety observations close before resampling |
| Pixel Bus Fishing Sample Interval | Coalesced JSON save | Next worker iteration after wake | Latest complete update wins |
| Pixel Bus Idle Sample Interval | Coalesced JSON save | Next worker iteration after wake | Latest complete update wins |
| Pixel Bus Block Size | Coalesced JSON save and managed redeploy | Next ESO reload or relog plus app restart | Running geometry remains unchanged |

The saved confirmation means the store write succeeded. It does not claim that staged geometry is active.
