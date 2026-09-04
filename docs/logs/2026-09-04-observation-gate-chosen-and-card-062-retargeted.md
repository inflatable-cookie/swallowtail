# Observation Gate Chosen; Card 062 Retargeted To 0.41.0

Date: 2026-09-04
Roadmaps: `../roadmaps/g05/009-contract-061-consumer-projection-realization.md`,
`../roadmaps/g05/026-kimi-code-local-server-0-40-1-useful-newer.md`

## Decisions

The operator accepted both Chatterbox recommendations on 2026-09-04.

1. **Shared provider-operation observation gate.** The gap recurs on
   `kimi-code.acp`, `deepseek-harness.local-server`, and `opencode.http`.
   Card 070 is a planning-only public-baseline gate that proposes exact
   additive runtime names, admission, composition, maxima, a Contract 061
   amendment, and testkit assertions, with DeepSeek harness and Kimi ACP as
   the proving consumers. Chatterbox promotes its note; a runtime baseline
   card follows; candidate I completion and card 034 reopen after that
   baseline merges.
2. **Card 062 retarget.** Official Kimi Code latest moved from `0.40.1` to
   `0.41.0` during the first identity run and the worker stopped cleanly with
   its evidence uncommitted. The family is retargeted to `0.41.0`; `0.40.1`
   becomes published adjacency; the same worker resumes. A second latest
   move before push stops the lane and raises the freeze-at-dispatch policy
   question rather than another retarget.

## Frontier

Concurrent: card 069 (candidate C), card 070 (observation gate), audit 065
(candidate E), and card 062 (Kimi `0.41.0` identity).
