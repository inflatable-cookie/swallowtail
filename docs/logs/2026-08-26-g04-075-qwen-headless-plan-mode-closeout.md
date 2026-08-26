# 2026-08-26 g04.075 Qwen Headless Plan Mode Closeout

Status: complete
Owner: Tom
Milestone: g04.075
Cards: 207-209
Research: 222
PR: https://github.com/inflatable-cookie/swallowtail/pull/74
Head: `fd750c5aa3797ffa75a5955a875c327d20319ff4`

## Outcome

Delivered portable `HarnessMode::Plan` on route `qwen.headless` for exact
`0.21.15`, `0.22.0`, and `0.22.1`. No behavior revision:
`qwen-code.headless.v0.21.15-reasoning-control` is unchanged. Selection is
optional on `QwenRunProfileInput` and `QwenSessionProfileInput`. Canonical
argv replaces only `--approval-mode default` with `plan`. Omission keeps
`--approval-mode default` and is not implicit Plan.

Provider `auto-edit|auto|yolo` stay unselected. Text-input children do not
register `exit_plan_mode`. Stream-json reasoning children cannot complete
plan-exit without a host `can_use_tool` allow this route does not send. Plan
is scheduler-blocked provider behavior, not filesystem, network, shell,
process, sandbox, or descendant containment. Isolation stays `AmbientHost`.
`--safe-mode` and `--exclude-tools` stay independent. Applied
`session_start.permission_mode` is observed as `"plan"` or `"default"`.

## Evidence

- Research 222 promoted one deliver-now row on exact packages `0.21.15`,
  `0.22.0`, and `0.22.1`, gitHeads `5dce2515`, `1c3a385d`, and `2755dbe1`
- command, driver, and prepared-facade fixtures cover omit argv, canonical
  `plan` replacement, unqualified-version rejection before spawn, resume
  reapplication, fresh replacement, and Plan-plus-reasoning composition on
  exact `0.21.15`
- Qwen headless guide, route matrix, feature-matrix notes, architecture, and
  triage distinguish Plan dispatch from isolation and other approval values

## Validation

Named closeout suite passed: `cargo fmt -p swallowtail-adapter-qwen`,
`effigy validate:focused swallowtail-adapter-qwen`,
`effigy package:verify-affected swallowtail-adapter-qwen`,
`effigy check:examples`, `effigy package:api`, `effigy qa:northstar`,
research/logs/roadmap/g04/batch-card/next-action index checks, and
`git diff --check`. Additive Qwen public API is recorded in
`public-api-unreleased`; `public-api-0.3.3` stays immutable.

Doctor remains an inherited error: `scan.god-files` plus generated-in-src.
This run records 380 god-file findings (334 warnings, 46 errors) against the
inherited 379 (333 warnings, 46 errors). The extra warning is
`crates/swallowtail-adapter-qwen/tests/prepared_facade/plan.rs`. This lane
does not repair god-files.

## Generation Boundary

g04.075 closes only this route-local family. g04 remains open for the next
per-route inventory reassessment unless the operator supplies a different
direction. Contract 029 currentness stays standing. Do not merge from the
worker thread.
