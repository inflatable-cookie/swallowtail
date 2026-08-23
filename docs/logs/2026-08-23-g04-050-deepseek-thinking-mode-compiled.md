# 2026-08-23 g04.050 DeepSeek Thinking Mode Compiled

## Change

- reassessed the remaining promoted per-route feature inventory after g04.049
- rejected Ollama attached `think=max` as the immediate lane because its
  selected-model catalogue advertises only generic thinking support and exact
  0.32.15 aliases `max` to `high` for Harmony/GPT-OSS
- selected DeepSeek V4 Pro explicit non-thinking mode for one-request
  structured runs
- compiled g04.050 and cards 139-141 as one serial evidence-first worker lane
- reserved Research 197 and the route-local closeout before dispatch
- fixed g04.050 as the final numbered roadmap in generation g04

## Decision

Current official DeepSeek Chat Completions and Thinking Mode material names
`thinking.type=enabled|disabled`, lists `deepseek-v4-pro` as supporting both
modes, and treats reasoning effort as a thinking-mode control. Existing
Research 186 already withheld disabled mode only because no exact typed
adapter-local control exists and direct continuation depends on private
reasoning replay.

Card 139 must freeze the current field combination, response behavior, cache
boundary, plan/evidence representation, and facade revision. Cards 140-141 may
continue only for a non-empty Research 197 structured-run deliver-now set.
Direct continuation remains enabled-only. No portable capability, production
claim, or implementation was introduced during compilation.

## Next

Execute g04.050 cards 139-141 serially in one isolated worker worktree and open
one PR. After merge closeout, reassess and close the g04 generation boundary.
