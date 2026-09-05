# 092 v0.4.1 Consumer Proof And Operator Tag Gate

Status: planned; serial after card 091's candidate merges with green exact-SHA CI; reduced by the operator's 2026-09-05 compression decision to the external source consumer; the recorded v0.4.0 Nucleus smoke satisfies Contract 036; Bovine's editing session is post-tag adoption evidence
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

## Auto-Continuation

No.
