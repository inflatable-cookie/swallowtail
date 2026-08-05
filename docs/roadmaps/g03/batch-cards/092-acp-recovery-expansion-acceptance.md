# 092 ACP Recovery Expansion Acceptance

Status: planned
Owner: Tom
Created: 2026-08-05
Milestone: `../035-acp-continuation-recovery-expansion.md`
Depends on: card 091

## Goal

Close deterministic, public, and package acceptance for the selected ACP
continuation-recovery mappings.

## Scope

1. Run shared local and remote-authoritative conformance where applicable.
2. Prove exact-once prepared execution, bounded replay, drift rejection,
   cancellation, and cleanup.
3. Update route truth and working-state restoration guidance.
4. Keep rejected candidates visible with their exact promotion gates.
5. Verify every affected extracted package without authenticated work.

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-protocol-acp swallowtail-adapter-cursor swallowtail-adapter-grok`
- `effigy package:verify-affected swallowtail-protocol-acp swallowtail-adapter-cursor swallowtail-adapter-grok`

## Stop Conditions

- stop if public truth implies all ACP agents inherit recovery
- stop if package proof requires an ambient executable or credential

## Auto-Continuation

Continue to card 093 after g03.035 closes.
