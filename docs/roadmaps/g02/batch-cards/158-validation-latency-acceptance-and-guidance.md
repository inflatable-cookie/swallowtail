# 158 Validation Latency Acceptance And Guidance

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../046-validation-latency-and-proof-routing.md`

## Goal

Close validation-latency work through measured normal-path improvement and
unchanged milestone, package, and release proof.

## Scope

1. Compare representative before and after focused validation paths.
2. Confirm broad workspace, candidate, and release gates remain independently
   available and unchanged.
3. Confirm selector failure propagation and package isolation.
4. Publish concise contributor and agent guidance.
5. Return to the active g02 product checkpoint.

## Acceptance Criteria

- [x] representative normal-path validation meets the selected budget
- [x] affected package proof avoids unnecessary repeated shared compilation
- [x] no test, lint, route, package, or release evidence is silently removed
- [x] contributor guidance distinguishes focused and milestone gates
- [x] no provider effect, consumer edit, candidate replacement, or publication
  occurs
- [x] one clear next task remains

## Validation

- measured focused selector path
- affected extracted-package proof
- workspace check
- package metadata and public-API gates
- route and docs QA
- doctor
- `git diff --check`

## Stop Conditions

- Stop if measured gains depend on weakening evidence.
- Do not run the full workspace test suite more than once.
- Do not continue into warning-only decomposition.

## Auto-Continuation

No. Return to the g02 stabilization checkpoint.

## Evidence

- four independently assembled adapter archives compiled through one shared
  extracted target in five seconds; the comparable card-155 path took 22.4
  seconds with separate targets
- Pi and xAI focused validation ran 64 tests plus warnings-denied clippy in
  four seconds
- deterministic scope failures and Effigy non-zero propagation passed
- workspace check, package metadata, public-API, route, docs, and diff gates
  passed
- candidate and release task definitions were unchanged
- doctor reported 143 warnings and one error from concurrent Kimi
  local-server activity work; card 155's accepted zero-error baseline predates
  that work
- no workspace test suite, live provider call, consumer edit, candidate
  replacement, or publication ran
