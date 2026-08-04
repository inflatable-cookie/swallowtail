# 065 Configured Provider Instance Catalogue Handoff

Status: completed
Owner: Tom
Created: 2026-08-04
Milestone: `../024-configured-provider-instance-catalogue.md`
Depends on: card 064

## Goal

Accept the portable catalogue boundary and publish the exact Nucleus g05.073
assembly path without adding consumer policy to Swallowtail.

## Scope

1. Reconcile Contract 047, architecture, public exports, and deterministic
   evidence.
2. Run focused runtime and affected-package validation.
3. Record the public types and prepare-list-admit-assemble consumer flow.
4. Return the sole Next Task to the g03 evidence gate.

## Out Of Scope

- Nucleus implementation or UI
- default provider, model, or reasoning policy
- provider login, catalogue probes, or authenticated acceptance
- publication or broad workspace validation

## Acceptance Criteria

- [x] focused and affected-package validation pass
- [x] the handoff names the exact public types and binding rules
- [x] consumer-owned selection and session defaults remain explicit
- [x] no authenticated provider work ran
- [x] the sole Next Task returns to the g03 evidence gate

## Validation

- `effigy validate:focused swallowtail-runtime`
- `effigy package:verify-affected swallowtail-runtime`
- `effigy qa:docs`
- `git diff --check`

## Auto-Continuation

No. Return to the g03 evidence gate after the handoff.
