# 2026-08-23 g04.051 Qwen Headless Budgets Closeout

Status: worker-complete; awaiting review
Owner: Tom
Milestone: g04.051

## Result

Research 198 was promoted with a non-empty deliver-now set: exact Qwen Code
`0.21.15` on route `qwen.headless` admits adapter-local caller-decreasing
`--max-session-turns` `1..=24` and `--max-tool-calls` `0..=16`. Independent
omission of either flag keeps the current argv byte `24` or `16`. Both omitted
keeps `--max-wall-time 60s --max-tool-calls 16 --max-session-turns 24`.

These are per-child process-local Qwen counters. They reset on every
structured-run, first, `--resume`, and fresh-replacement child. They do not
cap Swallowtail's separate interactive session bound of 24 host turns, prove
the provider completed less work, or become Contract 040 portable generation
controls.

Zero tools is useful: assistant text can succeed; the first tool tick aborts
before dispatch. Turn `0`, raised values, `-1`, negatives, and fractions stay
invalid or withheld. Selected values require exact package `0.21.15`.

Terminal truth stays process exit **53** (`native_turn_limit`) and **55**
(`native_budget`, shared with wall-time overrun) plus plain `stream-json`
stderr. No stronger semantic stream event is claimed.

Cards 142-144 bound that subset through `QwenSessionTurnBudget`,
`QwenToolCallBudget`, and `QwenHeadlessBudgets` on prepared input, immutable
evidence, driver, and every child command. Ordinary and reasoning-selected
transports are unchanged. No shared `Capability`, OperationPolicy field, or
Contract 029 edit landed here.

Worker validation passed: focused package validation (55 tests), affected-
package verification, examples, route QA, Northstar QA, research/log/roadmap
index QA, next-action QA, package API, and `git diff --check`. Review
requested a split of `tests/prepared_facade/budgets.rs`; run/version/terminal
proofs now live in `budget_runs.rs` and session/replacement proofs in
`budget_sessions.rs`. `effigy doctor` after that split reproduces the
inherited baseline: 376 god-file findings (330 warn / 46 err) plus
generated-in-src. No credentials, account state, live provider request, or
paid work was used.

PR: [#50](https://github.com/inflatable-cookie/swallowtail/pull/50).
Implementation commit: `87a59f8a879ebc92c66ce094332645da99308524`.
Worker branch: `t3code/follow-qwen-headless-budgets-handoff`.
No merge was performed.

## Shared Closeout Delta

Orchestrator after merge; this worker did not apply it.

- architecture: record adapter-local Qwen `0.21.15` caller-decreasing turn/tool
  budgets on `qwen.headless`
- Contract 029: currentness range unchanged; optional feature-local name
  `qwen-code.headless.v0.21.15-turn-tool-budgets` if the claim should record
  the exact-version gate
- route and feature matrices: selected turns `1..=24`, tools `0..=16`,
  omission `24` / `16`, wall time `60s`
- per-route feature programme: close this Qwen budget family
- research/log/roadmap/g04/batch-card indexes: Research 198 promoted; cards
  142-144 complete; g04.051 complete at the merged head
- `CHANGELOG.md`: unreleased `swallowtail-adapter-qwen` budget types
- `docs/roadmaps/README.md` Next Task: reassess remaining per-route inventory
  inside g04; keep g04 open until explicit operator direction
- do not close or roll g04
