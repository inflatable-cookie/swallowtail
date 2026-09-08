---
title: g05 Card 149 Desktop Claude zero-credit first-turn packet
kind: northstar-handoff
handoff_mode: external-evidence-packet
status: ready
owner: Swallowtail Chatterbox
created: 2026-09-08
updated: 2026-09-08
tags: [claude-agent-sdk, live-evidence, quota, first-turn]
---

## What This Thread Was Doing

Closing the last recoverable zero-credit evidence gap before Claude credit is
restored: the SDK does not initialize its query generator until the first turn.

## Why It Matters

Card 147 proved open succeeds at zero credit but sent no prompt. Card 148/313
then repaired the cleanup oracle provider-free. The actual exhausted-account
behavior can only be observed by one first query while the balance stays zero.

## Current State

Swallowtail Card 148 is complete from Desktop PR 175. The operator confirmed
the balance remains zero and authorized the fourth, final diagnostic attempt.
Exact source-linked route remains `0d120067`; `claude-sonnet-5` is the cheapest
model in Desktop's admitted Sonnet/Opus inventory.

## Boundaries

Prepare provider-free fixtures first. Then run one open and exactly one prompt,
`Reply with exactly: ready`. No retry, reconnect, respawn, second turn, tool
invocation, permission response, control attempt, top-up, qualification,
candidate, tag, or release. Preserve only bounded codes and terminal metadata;
never persist provider error text or account/billing data.

## Important Context

The SDK's first query may reject during initialization or start successfully
and later terminate with `swallowtail.claude-agent.sdk.provider_failed`. A
valid rate-limit event projects only generic progress; it cannot by itself
prove billing cause. Both branches must close immediately through Card 148's
exact cleanup oracle.

## Suggested Next Move

Add a separate one-turn diagnostic mode and capsule schema, prove every branch
offline, run it once while the balance is still zero, and preserve the result
through independent review and merge.

## Completion Protocol

Return capsule SHA-256, exact source and artifact tuple, prompt digest, bounded
start/terminal result, action counts, cleanup evidence, accepted head, review,
merge, and canonical Desktop closeout. No automatic second attempt.
