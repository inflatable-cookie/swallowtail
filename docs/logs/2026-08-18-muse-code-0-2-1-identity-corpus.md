# 2026-08-18 Muse Code 0.2.1 Host Payload Identity

## Result

Card 235 ranked Muse first among exact-pin host-drift families and froze
local `muse-bin-0.2.1-R1215.1`. Deterministic echo JSONL still uses schema
`1` with the same 23-record payload-type sequence as `0.1.0-R708.1`. Opaque
claims cannot keep both pins. Decision for card 236: move the QualifiedOnly
pin to exact `0.2.1-R1215.1` and reuse `muse-code.events-v1`. Production
claim stayed on `0.1.0-R708.1`.

## Next

Move the Muse opaque pin on card 236.
