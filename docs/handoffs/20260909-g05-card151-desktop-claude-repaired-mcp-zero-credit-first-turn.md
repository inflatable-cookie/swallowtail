---
title: g05 Card 151 Desktop Claude repaired-MCP zero-credit first-turn packet
kind: northstar-handoff
handoff_mode: external-evidence-packet
status: ready
owner: Swallowtail Chatterbox
created: 2026-09-09
updated: 2026-09-09
tags: [claude-agent-sdk, registered-tools, mcp, live-evidence, quota, first-turn]
---

## What This Thread Was Doing

Running the final recoverable zero-credit first-turn observation after Card 315
proved and repaired the registered-MCP courier race.

## Why It Matters

Card 314 never reached provider readiness or submitted its prompt. Card 315
retained MCP and proved 24/24 registered opens under the same target churn that
previously produced 0/24. The account is still at zero, so this is the last
recoverable diagnostic before top-up.

## Current State

Desktop PR 178 merged at
`dfc9c6a6e6f29c56751278d39fd025febdd8d917`; canonical closeout
`d2b23bbca0c0c602a188478a722a3763cd0df656` records the reproduced cause,
immutable courier repair, 28/28 focused fixtures, and 24/24 churn proof. Exact
source-linked Swallowtail remains
`0d120067cd260b1f5127835cab0b1a3ad020a29d`.

## Boundaries

Run provider-free checks first. Then one fresh registered-MCP open and exactly
one prompt, `Reply with exactly: ready`, using `claude-sonnet-5`. Write a new
capsule; preserve Card 314 and all earlier capsules byte-identically. No retry,
second turn, tool invocation, permission response, control, reconnect, respawn,
top-up, qualification, matrix, candidate, tag, or release work.

## Important Context

The capsule must prove the immutable courier digest and authenticated MCP
connection before provider readiness. A provider failure is a valid bounded
zero-credit observation but does not prove billing cause unless a safe typed
field says so. An MCP startup failure is still a defect, not quota evidence.

## Suggested Next Move

Promote the Desktop worker card and queue handoff, run the complete provider-free
check, then consume the one-shot live authority and stop after its first result.

## Completion Protocol

Open one Desktop PR. Independent exact-head review checks the offline-before-live
gate, repaired courier identity, one-open/one-prompt counts, MCP admission,
redaction, immutable prior capsules, and cleanup. Return the new capsule digest,
exact tuple, prompt digest, bounded result, counts, cleanup, accepted head,
review comment, merge, and canonical closeout. No automatic top-up or credited
qualification follows.
