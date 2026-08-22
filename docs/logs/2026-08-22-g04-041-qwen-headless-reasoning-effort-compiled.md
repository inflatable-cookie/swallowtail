# 2026-08-22 g04.041 Qwen Headless Reasoning Effort Compiled

## Change

- selected Qwen headless reasoning effort from the promoted per-route feature
  inventory after the Copilot evidence stop
- compiled g04.041 and cards 113-115 as one serial evidence-first worker lane
- reserved Research 189 and the route-local closeout record before dispatch

## Decision

Qwen already binds an exact model across structured-run and turn-scoped
session paths. That removes Copilot's no-model gap, but does not qualify the
documented `model.reasoningEffort` setting.

Card 113 must prove exact package `0.21.15` transport, precedence, model/value
support, clamp/default behavior, and child-process lifetime. A global setting
or `/effort` path is insufficient. The lane stops if use requires user-config
mutation, ambient defaults, or a synthetic config root without the Contract
033 host-scoped lease.

Cards 114-115 may continue only for Research 189 deliver-now rows through one
typed, operation-private Contract 040 mapping. Current behavior remains intact
when reasoning is absent. No Qwen implementation or capability claim was made
during compilation.

## Next

Execute g04.041 cards 113-115 in one isolated worker worktree and open one PR.
