# Retained Execution Audit And Corpus Gate

Date: 2026-07-28
Status: completed

## Changed

- classified all 59 retained-background-execution, stream-reattachment, and
  provider-managed-recovery `No` cells
- froze 32 operation-shape non-applicable cells, 22 exact selected-surface
  absences, two separate-route candidates, and three shared-contract
  candidates
- kept every matrix cell unchanged; no realized false negative exists
- selected Kimi headless and local-server recovery plus local-server
  reattachment
- promoted Contract 042
- froze the exact Kimi policy, retry, cursor, acknowledgement, failure,
  redaction, cancellation, topology, and cleanup corpus

## Decision

Kimi already emits exact harness retry records while its prepared operations
require prohibited provider recovery. That contradiction makes this tranche
an integrity repair, not a provider-priority choice.

Managed recovery now requires explicit caller acceptance. Swallowtail never
chooses or performs the retry.

Kimi local-server WebSocket v2 may reattach once to the same active turn from
the last accepted `{seq, epoch}` cursor. Reattachment sends no prompt,
callback, session-create, model, route, or credential request.

Kimi ACP inherits neither capability. Background execution remains
non-applicable to Kimi harness sessions.

## Evidence

- Research 056 records the complete currentness classification.
- Research 057 records contract fit and exact-range corpus.
- Contract 042 owns harness-managed recovery and active-turn reattachment.
- `retained-execution.json` freezes deterministic implementation cases.
- `effigy qa:routes --json` passed after machine classification was added.

## Current State

Cards 103-104 are complete. Card 105 is ready. Card 106 remains in bounds for
matrix closeout after focused production evidence.

## Next

Implement card 105 against the frozen Kimi corpus. Do not widen to Bedrock
asynchronous invocation or Anthropic Message Batches.
