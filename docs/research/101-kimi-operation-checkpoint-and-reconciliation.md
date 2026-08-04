# 101 Kimi Operation Checkpoint And Reconciliation

Status: promoted
Owner: Tom
Updated: 2026-08-04

## Trigger

Research 099 and 100 left `kimi-code.local-server` behind one explicit gate:
persist the exact live `{seq, epoch}` position and prove a restart can observe
the same provider turn without replaying a prompt or acquiring control.

## Finding

The qualified WebSocket v2 surface closes the gate.

- durable events carry ordered `seq`, `epoch`, session, and exact turn ids
- subscribe accepts one prior cursor
- subscribe acknowledgement returns the server's current cursor
- accepted replay begins strictly after the supplied cursor
- `resync_required`, epoch change, gaps, foreign sessions, and foreign turns
  are explicit failure evidence
- session lookup supplies cwd, archive, busy, and current sequence corroboration

The acknowledgement cursor bounds a finite read-only snapshot. Reconciliation
can stop after that sequence. It does not need to attach to the continuing
live turn, submit a prompt, answer a callback, abort work, or infer terminal
state from session idleness.

## Portable Checkpoint

The checkpoint is not a raw Kimi cursor handed to consumers. One portable
record binds:

- exact provider session
- consumer runtime turn
- exact provider turn
- adapter-owned opaque cursor bytes
- the same route, instance, host, model, resource, access, and provider-state
  attachment fingerprint as the durable session binding

Accepted Kimi runtime events carry the newest checkpoint after the provider
turn is known. The consumer persists the opaque versioned record. Restoration
rejects corruption, unknown versions, oversized fields, and attachment drift.

## Detachment Qualification

An externally attached qualified local server owns the provider session and
turn independently of one WebSocket observer. Closing only that observer sends
no `abort`. The persisted checkpoint then reconciles the same exact turn.

The proof excludes host-owned foreground servers, manual permission mode,
callback exchange, unverified-newer versions, structured runs, and ordinary
close without an explicit detach request.

## Promoted Decisions

- Contract 048 owns the route-bound operation checkpoint and finite Kimi
  reconciliation snapshot.
- Contract 049 admits Kimi as the second active-turn detachment mapping.
- A Kimi terminal state requires the exact retained `turn.ended` event.
- Same-cursor busy state may report exact `Active`; idle without a terminal
  event remains `InactiveUnresolved`.
- Reconciliation grants no abort, callback, prompt, resume, import, or
  subagent-control authority.

## Sources

- qualified Kimi local-server REST and WebSocket v2 corpus
- `kimi-code-0.29.1-0.29.2/retained-execution.json`
- Contracts 017, 042, 048, and 049
- Research 099 and 100
