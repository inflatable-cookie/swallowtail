# 080 Gemini Durable Transcript Reconciliation Qualification

Status: completed
Owner: Tom
Created: 2026-08-04
Milestone: `../032-retained-operation-reconciliation-candidate-gate.md`
Depends on: card 079

## Goal

Decide whether one qualified Gemini CLI headless transcript can provide exact,
read-only interrupted-run reconciliation without prompt replay or deletion.

## Scope

1. Recheck exact `0.51.0..=0.52.0` transcript creation, storage, list/read, and
   terminal-record evidence.
2. Correlate the driver-derived session id, runtime run, final stream event,
   transcript identity, model, resource, host, and prepared route.
3. Separate history snapshot truth from exact terminal operation truth.
4. Prove the observation path starts no Gemini run and performs no resume,
   prompt, deletion, callback, or tool action.
5. Promote the result into Research 099 and preserve all stop evidence.

## Validation

- deterministic source and corpus evidence only
- `effigy qa:docs`

## Stop Conditions

- stop if reading history requires `--resume`, prompt input, or transcript mutation
- stop if the stored transcript cannot identify the exact interrupted run and
  terminal state
- stop if a filesystem path rather than a prepared opaque binding would become authority

## Auto-Continuation

Continue to card 081 after the Gemini classification is promoted.

## Evidence

Exact `v0.51.0` and `v0.52.0` source calls summary generation before
`--list-sessions`; that path may invoke Gemini and append transcript metadata.
The list and transcript also lack an exact terminal record. Research 103 and
Contracts 038 and 048 preserve the blocked classification.
