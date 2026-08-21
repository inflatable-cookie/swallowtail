# g04.021 Unmarked Overlay Worker Dispatch

Date: 2026-08-21
Roadmap: `../roadmaps/g04/021-unmarked-overlay-rows.md`
Handoff: `../handoffs/20260821-075348-g04-021-unmarked-overlay-rows.md`

## Result

PR 17 is on `main`. One serial worker lane is dispatched for unmarked
overlay rows. Cards 059-061 are the assigned runway. Planning base is
`3d7616555a94233b8d03a5f3f20382b6a62a084c`. Preferred direction:
instance-plus-model keying when `provider_id` is absent. Do not invent a
provider id. 022-023 stay planned. Hosted OAuth stays parked.

## Next

Operator starts one worker thread with only:

```text
Read and follow `docs/handoffs/20260821-075348-g04-021-unmarked-overlay-rows.md`.
```
