---
title: g05.084 Roll over to g06 and compact g05
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g05/084-roll-over-to-g06-and-compact-g05.md
queue_dispatch: northstar-queue
queue_approval: "Tom authorized the structural rollover on 2026-09-17 with 'roll over and compact'. Documentation and planning surfaces only; no product or release authority."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Deliver g05.084: close g05, open g06, and compact g05 into a non-procedural
archive roll-up without losing an open commitment.

## Why It Matters

g05 is at 84 numbered tasks, past its 30–50 range, and the operator authorized
the structural rollover. Swallowtail already compacted `g01`–`g04` this way in
g05.038, so the pattern, the preservation oracle, and the roll-up shape all
exist and must be followed rather than reinvented.

## Current State

`docs/roadmaps/g05/` holds the task set. `.northstar/lifecycle/v1/` holds 22
task fragments and `projection-targets.json`, which declares `active_generation:
g05` and three projection targets. Swallowtail has no lifecycle tooling wired —
`effigy lifecycle:run` is not defined in its catalog — so this compaction is
hand-executed against the installed procedure. There is no `generations/`
directory yet. The four live lanes this task depends on are dispatched.

## Boundaries

Follow `docs/roadmaps/g05/084-roll-over-to-g06-and-compact-g05.md`. Planning,
instruction, checker, and lifecycle surfaces only. Do not change product or
runtime code, contract or architecture semantics, release state, tags, or any
consumer repository. Do not invent a new g06 product goal: carry the unclosed
outcomes forward and leave g06's wider focus to Chatterbox and the operator.

## Important Context

The carry-forward set and the required destination map are named in the task.
Run the preservation oracle before deleting anything: no live rule may exist
only inside the removed tree, and every open commitment needs an active home or
a recorded reason. Update the front door, generation index, status grammar,
standing lanes, the lifecycle projection targets, the projection blocks and
their digests, and the `effigy.toml` index policy. Delete
`docs/roadmaps/g05/**` last, and fold this task's own record into
`archive/g05.md` and the closeout log before its file is removed.

## Suggested Next Move

Start with the classification and destination map, then write the roll-up and
the g06 README before touching any index or deleting any file.

## Completion Protocol

Run `effigy qa:docs` and `effigy qa:northstar` in full, confirm no reference
resolves into `docs/roadmaps/g05/`, and open one PR for independent exact-head
review. Return head, validation, review, merge, and closeout.
