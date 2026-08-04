# 083 Gemini Stored Transcript Management Truth Repair

Status: ready
Owner: Tom
Created: 2026-08-04
Milestone: `../033-anthropic-managed-run-reconciliation-and-recovered-cleanup.md`
Depends on: card 082

## Goal

Stop claiming confirmed Gemini transcript deletion through a
`--list-sessions` operation which may perform inference and mutate history.

## Scope

1. Freeze exact `0.51.0..=0.52.0` summary-before-list evidence.
2. Remove the stateful post-delete list from the claimed read-only confirmation path.
3. Downgrade or remove `HistoryRemoved` capability and prepared evidence until
   exact side-effect-free confirmation exists.
4. Preserve explicit destructive uncertainty; do not infer deletion from exit
   status, success prose, or direct filesystem inspection.
5. Reconcile public route and feature truth without affecting Gemini ACP or
   headless run support.

## Validation

- `effigy validate:focused swallowtail-adapter-gemini`
- `effigy package:verify-affected swallowtail-adapter-gemini`

## Stop Conditions

- stop rather than introduce provider-private filesystem authority
- stop rather than expose transcript content or issue a second provider request

## Auto-Continuation

Continue to card 084 after runtime and public deletion truth agree.
