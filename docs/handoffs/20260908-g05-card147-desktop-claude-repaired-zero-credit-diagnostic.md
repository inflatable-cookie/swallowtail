---
title: g05 Card 147 Desktop Claude repaired zero-credit diagnostic packet
kind: northstar-handoff
handoff_mode: external-evidence-packet
status: ready
owner: Swallowtail Chatterbox
created: 2026-09-08
updated: 2026-09-08
tags: [claude-agent-sdk, live-evidence, quota, diagnostics]
---

## What This Thread Was Doing

Preparing the one authorized Desktop diagnostic open after Card 146 repaired
the pre-readiness MCP-status projection mismatch.

## Why It Matters

Card 145 consumed one open but failed at `mcp_status_invalid` before the route
could observe the operator-confirmed exhausted-credit state. Card 147 checks
the repaired route while that state still exists.

## Current State

Card 146 merged through PR 296 as
`13dee542e5ef4ab967cb4f0934cc11c35a8768c2`; canonical closeout is exact
Swallowtail `0d120067cd260b1f5127835cab0b1a3ad020a29d`. The operator confirmed
credit remains zero and explicitly authorized one repaired diagnostic open.

## Boundaries

Run exactly one open with `claude-sonnet-5`. No prompt, turn, tool dispatch,
permission callback, control attempt, retry, reconnect, or respawn. If open
succeeds, close immediately. Do not top up or mutate the account. Do not
qualify cells, change matrix availability, prepare a candidate, tag, or
release.

## Important Context

Preserve the Card 145 frozen tuple: SDK `0.3.259`, native `2.1.259`, Node
`22.23.2`, source-tagged sidecar `0.4.4`, mediated stdio/private loopback, MCP
`2025-11-25`, default permission, persistence false. Link the complete Desktop
development graph to exact Swallowtail `0d120067` without committing its
machine-local lock. Preserve Card 144's bounded receipt and Card 145's
redaction oracle.

## Suggested Next Move

Reuse the accepted Card 145 runner, add only the source/receipt changes needed
for the repaired route, prove one-open and zero-action bounds provider-free,
then consume the single authorized live open and freeze the redacted capsule.

## Completion Protocol

Commit the immutable capsule, focused runner changes if needed, tests, and a
factual Desktop log in one PR. Independent review verifies source linkage,
attempt counts, receipt fidelity, cleanup, and redaction. Return exact capsule,
PR, review, merge, and closeout identities to Swallowtail Chatterbox. No
automatic continuation follows.
