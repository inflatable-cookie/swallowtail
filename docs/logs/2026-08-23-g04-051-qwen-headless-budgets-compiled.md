# 2026-08-23 g04.051 Qwen Headless Budgets Compiled

## Change

- recorded the operator's selection of exact `qwen.headless` `0.21.15`
  caller-decreasing turn/tool budgets after the post-g04.050 reassessment
- compiled g04.051 and cards 142-144 as one serial evidence-first worker lane
- reserved Research 198 and the route-local closeout before dispatch
- kept g04 active; no generation closure or rollover was authorized

## Decision

The current route already emits fixed `--max-session-turns 24` and
`--max-tool-calls 16` on every run and turn child. Current official docs and
exact `0.21.15` source expose real native enforcement, so a smaller typed
caller-selected envelope is plausible without adding a portable generation
control.

Card 142 must first close zero-tool usefulness, exact turn accounting, counter
lifetime, process/stream/terminal truth, and ordinary/reasoning child
composition. Cards 143-144 may continue only for a non-empty Research 198
deliver-now set. Omission stays `24` / `16`; the 60-second native wall bound,
host deadline, tool set, approval posture, model route, and credentials do not
change. No production claim or implementation was introduced during
compilation.

## Next

Execute g04.051 cards 142-144 serially in one isolated worker worktree and open
one PR. Stop honestly after card 142 if the exact deliver-now set is empty or
requires shared contract/currentness work. After merge, reconcile this
route-local milestone and reassess the remaining inventory. Keep g04 open until
the operator directs otherwise.

