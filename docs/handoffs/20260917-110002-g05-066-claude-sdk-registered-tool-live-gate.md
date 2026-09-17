---
title: g05.066 Claude Agent SDK 0.3.270 registered-tool live gate
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom re-confirmed on 2026-09-17 that the separately authorized Claude Agent SDK registered-tool live gate still stands. It covers one exact tuple and one attempt only."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g05.066: run the authorized registered-tool live requalification on the
exact rebound tuple and settle the two Contract 061 cells it feeds.

## Why It Matters

Research 301's accepted live evidence is bound to `0.3.259`/`2.1.259` and does
not transfer. g05.065 rebounded the tuple to `0.3.270`/`2.1.270` but left
`registered_tools` and `consumer_tool_exchange` unqualified behind
`live_tuple_not_compiled`, so the route advertises less truth than it can prove.

## Current State

The tuple, wrapper/native coupling, wire, Node `22.23.2`, sidecar source-tag
axes, claim ids, and `QualifiedOnly` posture are frozen. Tom re-confirmed the
provider-operation authority on 2026-09-17. The gate itself has not run.

## Boundaries

Follow `docs/roadmaps/g05/066-claude-agent-sdk-0-3-270-registered-tool-live-requalification.md`.
One exact tuple, one attempt, no fallback and no rerun. Use the cheapest
adequate model. No install, login, host update, release, tag, or consumer
mutation, and no compatibility transferred from Claude Code or Claude Agent ACP.

## Important Context

Reproduce Research 301's Desktop Card 318 capsule shape exactly: an exact Allow
dispatches once with an unchanged `{}` and its fixed result is correlated;
Deny, cancellation, and stale or foreign controls dispatch zero times; cleanup
is joined with no survivors. On acceptance, move the frozen live point and
re-open both matrix cells. On any other outcome, keep
`live_tuple_not_compiled` and record the typed stop.

## Suggested Next Move

Fix the exact executable, model, prompt, correlation, and cleanup evidence
before any provider operation, then run the single gate.

## Completion Protocol

Run the focused and affected-package gates for
`swallowtail-adapter-claude-agent`, route and matrix QA, and the named docs
gates. Open one PR for independent exact-head review. Return head, validation,
review, merge, and closeout, with the provider evidence attached. No provider
work beyond the one authorized gate.
