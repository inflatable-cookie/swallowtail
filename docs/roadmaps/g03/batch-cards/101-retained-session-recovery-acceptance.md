# 101 Retained Session Recovery Acceptance

Status: planned
Owner: Tom
Created: 2026-08-05
Milestone: `../037-retained-session-recovery-promotion.md`
Depends on: card 097 and every selected implementation card

## Goal

Close route, facade, package, and remaining-gate truth for retained-session
recovery candidates.

## Scope

1. Run common conformance for every selected route.
2. Update restoration guidance and the exact route matrix.
3. Preserve unsupported candidates and promotion gates explicitly.
4. Keep Gemini ACP's replay-readiness blocker and private headless continuation
   outside production recovery.
5. Record the next compatibility evidence gate.

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-adapter-pi swallowtail-adapter-alibaba-model-studio`
- affected-package verification for every changed exact package
- `effigy qa:docs`
- `effigy qa:routes`

## Stop Conditions

- stop if public counts exceed packaged prepared mappings
- stop if a blocked candidate is reported as partial recovery

## Auto-Continuation

No. Close g03.037 and return to the generation evidence gate.
