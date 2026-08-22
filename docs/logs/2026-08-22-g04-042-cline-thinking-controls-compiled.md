# 2026-08-22 g04.042 Cline Thinking Controls Compiled

## Change

- selected Cline thinking controls from the promoted per-route feature
  inventory after Qwen reasoning delivery
- compiled g04.042 and cards 116-118 as one serial evidence-first worker lane
- reserved Research 190 and the route-local closeout record before dispatch

## Decision

`cline.acp` and `cline.headless` share exact package `3.0.55`, but not an
operation shape or capability claim. Card 116 must classify the ACP spawn flag
and headless run flag independently.

Current official documentation names `none`, `low`, `medium`, `high`, and
`xhigh`, but its omission/default descriptions conflict. Exact package evidence
must settle parsing, normalization, provider/model dependence, persistence,
and lifetime. It must also prove whether upstream `none` is exact portable
`off`; label resemblance is insufficient.

Cards 117-118 may continue only for Research 190 deliver-now route/value rows.
An empty set or a one-transport result is valid. No Cline implementation,
compatibility-currentness change, or capability claim was made during
compilation.

## Next

Execute g04.042 cards 116-118 in one isolated worker worktree and open one PR.
