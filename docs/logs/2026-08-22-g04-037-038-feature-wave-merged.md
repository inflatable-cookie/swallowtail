# 2026-08-22 g04.037-038 Feature Wave Merged

## Change

- merged PR 37 fast-forward-only at `56a7b87b` after 5/5 exact-head checks
- retargeted PR 36 to the identical new `main` base, then merged it
  fast-forward-only at `badb400a` after 5/5 exact-head checks
- reconciled the deferred Anthropic and DeepSeek architecture, route matrix,
  feature matrix, changelog, programme, indexes, and roadmap state
- compiled g04.039 as the next serial per-route feature milestone

## Result

`anthropic.messages` now exposes exact `claude-opus-4-7` effort `low`,
`medium`, `high`, `xhigh`, and `max` through portable reasoning selection on
structured and fixed direct-continuation profiles. It does not add Messages
thinking or claim effective effort.

`deepseek.continuation` now exposes exact V4 Pro `low`, `high`, and `max`
reasoning selection while keeping thinking fixed to enabled and private
reasoning replay inside the adapter. It does not accept provider aliases or
claim effective reasoning depth.

The promoted advanced-feature triage remains represented by the per-route
programme. Contract 029 currentness remains standing.

## Next

Execute g04.039 cards 107-109 for xAI Responses WebSocket reasoning and output
bounds. Card 107 must recheck the exact current model/value/profile surface;
current official Grok 4.5 and 4.6 behavior supersedes the older inventory
assumptions.
