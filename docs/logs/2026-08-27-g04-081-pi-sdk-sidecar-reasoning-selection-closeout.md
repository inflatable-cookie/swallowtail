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
- `prepared/build.rs` extraction from `prepared.rs` to clear god-file threshold
- driver startup/validation/continuity: reasoning propagation, bootstrap
  dispatch, and fail-closed mismatch handling
- sidecar driver reasoning lifecycle tests with split fixture scenarios
  (`ThinkingBootstrapMismatch`, `ThinkingStateMismatch`, `ThinkingStateMissing`)
- `evidence/npm-shrinkwrap.json`: published `@earendil-works/pi-coding-agent@0.84.2`
  tarball shrinkwrap (not synthetic)
- Research 228, cards 225-227, g04.081, programme/triage/indexes, guide, matrix,
  route-matrix inventory count, this closeout

## Validation

- `effigy validate:focused swallowtail-adapter-pi` — pass (101 tests)
- `effigy package:verify-affected swallowtail-adapter-pi` — pass
- `effigy check:examples` — pass
- `effigy package:api` — pass (baseline updated for `prepare_pi_sdk_sidecar_session`)
- `effigy qa:northstar` — pass
- `effigy qa:docs:index:*` and `effigy qa:docs:next-action:roadmaps` — pass
- `effigy doctor` — inherited baseline only: 333 warnings / 47 errors (no new
  `prepared.rs` god-file after build extraction)
- `cargo fmt -p swallowtail-adapter-pi` — pass
- `git diff --check` — pass

## Continuation

Keep g04 open. Reassess the remaining per-route feature inventory for the next
serial lane unless the operator supplies a different direction. Contract 029
currentness remains standing.
