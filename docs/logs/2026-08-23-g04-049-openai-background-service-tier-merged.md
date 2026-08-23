# 2026-08-23 g04.049 OpenAI Background Service Tier Merged

## Change

- fast-forwarded `main` from `8d49f704` to reviewed PR 48 head `06c00e6c`
- verified GitHub records `06c00e6c` as the merge commit
- applied the reserved architecture, Contract 029, route-matrix, fixture,
  changelog, index, programme, triage, and roadmap closeout delta
- retained the earlier `openai-responses-background-2026-08-23` facade point as
  frozen, non-executable history

## Result

`openai.background` now exposes adapter-local
`OpenAiBackgroundServiceTier::standard()` for ordinary attached runs and one
in-process reattachment. It dispatches exact Responses
`service_tier: "default"` through preparation, evidence, driver, and create
encoding. Omission preserves prior request bytes. Active-run detachment rejects
before effects. Selected-tier checkpoints carry an adapter-owned
non-reconcilable marker and reject restart reconciliation before network work.
Returned tier, project settings, price, latency, capacity, entitlement,
fallback, and provider acceptance remain unclaimed.

All five required CI jobs passed on exact merged head `06c00e6c`. Independent
review validation passed 63 focused tests, affected-package proof, semantic API,
Northstar QA, and diff checks.

## Next

Compile g04.050 from the remaining promoted per-route feature inventory. Recheck
current production-route, contract, and official-source truth before selecting
one coherent route-local control family. Treat g04.050 as the final roadmap in
g04, then reassess the generation boundary.
