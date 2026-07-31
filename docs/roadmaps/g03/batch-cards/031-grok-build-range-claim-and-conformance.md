# 031 Grok Build Range Claim And Conformance

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../012-grok-build-0-2-117-range-maintenance.md`
Depends on: card 030

## Goal

Extend Grok Build ACP's maintained claim through `0.2.117` using the frozen
exact corpus and one explicit task-control behavior milestone.

## Scope

1. Extend the executable claim without moving baseline `0.2.114`.
2. Map `0.2.114..=0.2.116` to the existing behavior revision and exact
   `0.2.117` to the task-control revision.
3. Bind discovery, preflight, initialization, run, cancellation, and activity
   evidence to the exact planned version.
4. Cover boundaries, interiors, prereleases, revision mismatches, malformed
   observations, and one later stable unverified point.

## Acceptance Criteria

- [x] `0.2.114..=0.2.117` membership matches the corpus exactly
- [x] each segment reports its exact private behavior revision
- [x] every qualified version requires its exact executable source revision
- [x] later stable versions remain permitted but unverified
- [x] unsupported and malformed versions fail before provider work
- [x] no direct task-control or subagent-control authority is added
- [x] focused Grok and extracted-package validation pass
- [x] card 032 becomes sole ready and next

## Validation

- `effigy validate:focused swallowtail-adapter-grok`
- `effigy package:verify-affected swallowtail-adapter-grok`
- `git diff --check`
- no broad workspace suite

## Auto-Continuation

Yes. Continue to card 032 after focused and extracted-package proof.

## Evidence

- 24 focused Grok tests passed in one second
- extracted 44-file package compiled in two seconds
- exact source revisions are required for all four guaranteed releases
- both private behavior segments complete deterministic structured runs
- no authentication or provider prompt ran
