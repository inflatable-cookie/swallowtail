# 092 v0.4.1 Consumer Proof And Operator Tag Gate

Status: complete; source consumer passed at `c3cce750`; operator-authorized annotated tag `v0.4.1` pushed
Owner: Tom
Created: 2026-09-05
Updated: 2026-09-05
Milestone: `../030-v0-4-1-release-readiness.md`
Depends on: completed card 091 at the immutable candidate SHA; card 052 as precedent

## Goal

Prove the frozen candidate through the external source consumer and one
operator-authorized authenticated working-application session, compile final
evidence, and stop for the operator's exact tag decision.

## Operator Authority Packet (required before ready)

Accepted on 2026-09-05: the working application is Bovine Desktop and the
smoke is a multi-turn editing session on `claude-agent.sdk` with per-call
admission visible, the requirement's own acceptance. Still required before
this card is ready, supplied by the operator or the Acowtancy Chatterbox
thread: the exact Bovine checkout and dependency revision that consumes the
candidate; the exact command or test; the retry budget (the `v0.4.0`
precedent was one attempt plus one authorized retry); and consumer-repo
mutation permission. No provider call happens until that packet is
complete.

## Scope

1. `effigy package:source-consumer` from a clean detached candidate checkout.
2. One authenticated application smoke against the exact candidate, within
   the packet's retry budget; sanitized evidence only.
3. Compile the tag decision request: source commit, canonical branch and
   remote, exact tag `v0.4.1`, annotated message, and confirmation that no
   publication, GitHub Release, binary, sidecar, or installer is included.
4. Stop. No card authorizes tag creation or push.

## Result

`effigy package:source-consumer` passed from a clean detached checkout at
`c3cce7504ffd5eae138a0190f1cd81332db68c3c`. No application had driven the
candidate before the tag. The operator authorized creation and push of the
annotated `v0.4.1` tag; its object is
`c888b2dc1a968d8dda66a99da1bb5fd51067df58`, and its local and remote peels
resolve to the merged SHA. No provider call, consumer-repository mutation,
publication, GitHub Release, binary, sidecar, or installer was performed.

## Auto-Continuation

No.
