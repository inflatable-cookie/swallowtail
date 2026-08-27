# 2026-08-27 g04.081 Pi SDK Sidecar Reasoning Selection Closeout

Status: complete and review-ready
Owner: Tom
Milestone: g04.081
Cards: 225, 226, 227
Branch: `t3code/pi-sdk-sidecar-selection`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-532827d3`
PR: https://github.com/inflatable-cookie/swallowtail/pull/80

## Result

Research 228 promotes one bounded deliver-now family on exact Pi `0.84.2`:
`anthropic` / `claude-opus-4-5` with `off`, `minimal`, `low`, `medium`, and
`high`. Cards 226-227 bind portable `ReasoningSelection` through preparation,
immutable plan/request agreement, canonical bootstrap `thinkingLevel`, and
bootstrap/state effective-level confirmation before readiness. Omission retains
exact prior bootstrap bytes and claims no selected mode.

## What Changed

- `crates/swallowtail-adapter-pi/src/sidecar/reasoning.rs`: closed admission
  table and option validation
- `prepared.rs`, driver startup/validation/continuity: reasoning propagation,
  bootstrap dispatch, and fail-closed mismatch handling
- sidecar driver tests plus fixture host `ThinkingMismatch` scenario
- `evidence/npm-shrinkwrap.json`: frozen npm integrity pin for pi-ai `0.84.2`
- Research 228, cards 225-227, g04.081, programme/triage/indexes, guide, matrix,
  this closeout

## Validation

- `effigy validate:focused swallowtail-adapter-pi` — pass (96 tests)
- `effigy package:verify-affected swallowtail-adapter-pi` — pass
- `effigy check:examples` — pass
- `effigy package:api` — pass (baseline updated for `prepare_pi_sdk_sidecar_session`)
- `effigy qa:northstar` — pass
- `effigy qa:docs:index:*` and `effigy qa:docs:next-action:roadmaps` — pass
- `cargo fmt -p swallowtail-adapter-pi` — pass
- `git diff --check` — pass

## Continuation

Keep g04 open. Reassess the remaining per-route feature inventory for the next
serial lane unless the operator supplies a different direction. Contract 029
currentness remains standing.
