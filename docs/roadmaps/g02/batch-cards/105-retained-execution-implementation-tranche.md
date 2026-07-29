# 105 Retained Execution Implementation Tranche

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../031-retained-execution-and-recovery-feature-closure.md`
Depends on: card 104

## Objective

Implement explicit Kimi managed recovery and maximum-one local-server
active-turn reattachment under Contract 042.

## Scope

1. Require explicit managed-recovery acceptance for Kimi headless and
   local-server structured runs.
2. Advertise the exact route-dependent and local-server recovery capabilities.
3. Add maximum-one local-server reattachment without prompt replay.
4. Preserve route, version, state, cursor, access, topology, and lifecycle
   truth.
5. Keep retrieval, stream reattachment, reconnect, and provider-managed
   recovery separate.
6. Join transport and host work before releasing access.
7. Change matrix cells only after focused conformance passes.

## Acceptance Criteria

- [x] every converted cell has a public prepared path
- [x] raw state ids or cursors cannot mint authority
- [x] no lifecycle or recovery outcome is strengthened
- [x] cancellation and uncertainty remain visible
- [x] focused exact-range conformance passes offline

## Auto-Continuation

Continue to card 106 only after every selected cell has deterministic
production evidence.

## Outcome

Kimi headless and local-server run inputs now require explicit
managed-recovery acceptance. Exact retry records remain redacted progress
evidence; malformed or contradictory attempt ordering fails closed.

Local-server structured runs may opt into one cursor reattachment. The failed
socket joins, the replacement uses the same session, prompt, turn, access,
model, deadline, and cursor, cancellation follows the replacement control,
and no prompt is replayed.

Focused headless, local-server structured, and corpus suites pass. The matrix
now records route-dependent installed recovery plus local-server recovery and
reattachment.
