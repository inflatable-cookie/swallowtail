# 101 Retained Session Recovery Acceptance

Status: complete
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

## Completion

- [x] common retained-conversation conformance passes for the selected Alibaba
      route; Pi remains blocked rather than partially promoted
- [x] Alibaba retained conversation maps to the common one-shot restoration
      facade as `ProviderSessionContinuationRecovery`
- [x] the mapping returns exact interrupted-turn identity, complete bounded
      replay, one live session, and no terminal-state inference
- [x] route, lifecycle, feature, restoration, and prepared-integration guidance
      distinguishes retained preservation from operation-owned deletion
- [x] Alibaba load and explicit management deletion are public `Yes` values;
      archive, restore, replay-free resume, and native close remain unsupported
- [x] Gemini ACP replay readiness and private headless continuation remain
      outside production recovery
- [x] focused validation passed: 216 tests across runtime, Pi, and Alibaba
- [x] extracted Alibaba package proof, docs validation, and route validation
      passed without authenticated provider work

Roadmap g03.037 is complete. The sole Next Task returns to the g03 evidence
gate.
