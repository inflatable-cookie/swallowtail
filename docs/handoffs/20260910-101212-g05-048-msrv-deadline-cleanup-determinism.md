---
title: g05.048 MSRV deadline-cleanup determinism
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom said Continue on 2026-09-10 after Chatterbox recommended g05.048 to repair the recurring pinned-MSRV deadline test and obtain a qualifying all-green run. No tag or release-prepare authority was granted."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g05.048: replace the recurring scheduler-sensitive Claude ACP
structured-run deadline/cleanup test with deterministic, separately controlled
proof, then restore the `v0.5.0` qualifying hosted gate.

## Why It Matters

The exact test failed in both the `v0.4.4` release lane and `v0.5.0` merge-SHA
run 34456800235. A blind rerun cannot support the irreversible tag decision.
The current fixture conflates operation timeout with the independent cleanup
deadline and lets thread scheduling decide the asserted cleanup diagnostic.

## Current State

g05.047 is complete. PR #310 merged as
`31375966bb4bcb2b62d6c5396490355ba41955c8`, tree
`4336c36236d865129638a8129914fab89fbe448c`, after review `5615719889`.
All cheap gates passed and version is `0.5.0`; merge-SHA push run 34456800235
failed only `Pinned MSRV floor tests` with 167 passed and the one recurring
deadline/cleanup assertion failed. `v0.5.0` is absent; `v0.4.4` is immutable.

## Boundaries

Follow `docs/roadmaps/g05/048-msrv-deadline-cleanup-determinism.md`. Own only
the exact structured-run test and narrow fixture controls plus closeout. Do
not edit production source, workflows, Cargo/version/release surfaces,
historical evidence, or the existing prepare receipt. Do not rerun release
prepare, contact providers, mutate Desktop, create or push a tag, or publish.

## Important Context

Prove operation timeout, cleanup completed before its boundary, and cleanup
deadline expiry as separate deterministic cases. No sleeps, timing margins,
ignored assertions, compiler branches, or retry-until-green. If production
behavior is wrong, stop with the counterexample instead of changing it under
this handoff. Run the repeated exact pinned-Rust proof before one new hosted
gate.

## Suggested Next Move

Instrument the fixture's deadline observations and write the scheduler-order
counterexample first. Split the old compound assertion into exact independent
oracles, then run the bounded Rust 1.95.0 repetition and focused package gate.

## Completion Protocol

Open one PR for independent exact-head review. Obtain a new qualifying hosted
run with every job green, including pinned-MSRV tests, before merge; transfer
it only across an identical merge tree. Return the final repair head, merge
SHA/tree, review, hosted run, local repetition proof, unchanged release
receipt/surfaces, zero provider contact, and absent `v0.5.0`. The queue owns
merge and closeout; Tom owns any exact-SHA tag decision.
