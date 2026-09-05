# 092 v0.4.1 Consumer Proof And Operator Tag Gate

Status: planned; serial after card 091's candidate merges with green exact-SHA CI; needs the operator's smoke authority packet
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

The operator names: the working application and exact checkout; the
authenticated route and the exact command or test; the retry budget; and
consumer-repo mutation permission. Chatterbox recommends a Bovine Desktop
multi-turn editing session on `claude-agent.sdk` with per-call admission
visible, because that is the requirement's own acceptance.

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
