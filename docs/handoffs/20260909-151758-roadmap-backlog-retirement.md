---
title: Swallowtail roadmap backlog retirement
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom supplied the Northstar roadmap-backlog retirement prompt on 2026-09-09; it explicitly authorizes bounded documentation, planning, instruction, template, fixture, and local-checker repair plus deletion after truthful disposition."
queue:
  dependsOn: [960585b1-6e7f-4a96-9427-411c00bb2776]
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Run the one-time Swallowtail roadmap-backlog retirement under canonical task
`g05.043`. Apply the frozen nine-item disposition manifest, remove the duplicate
intake surface, and align current doctrine and local checks with flattened
Northstar tasks.

## Why It Matters

The backlog duplicates triage and blurs candidate intake with approved
execution. Northstar now uses roadmaps only for promoted executable tasks and
triage for unresolved or deferred candidates.

## Current State

Swallowtail `main` was clean and synchronized at planning start at
`d85bb989be046b636337794fe7c3a71ba174822c`. The flattened-task migration queue
task `960585b1-6e7f-4a96-9427-411c00bb2776` is done. No unfinished Swallowtail
queue task, worker, reviewer, or PR remains. The backlog holds nine files plus
its index; one item needs a new triage destination and the rest have frozen
delivered, parked, duplicate, or declined dispositions in `g05.043`.

Canonical task:
`docs/roadmaps/g05/043-roadmap-backlog-retirement.md`.

## Boundaries

Edit only the documentation, planning, instruction, template, fixture, and
local-checker surfaces owned by `g05.043`. Do not edit Rust/product/runtime
code, make provider calls, create product tasks, change queued-work disposition,
roll a generation, alter candidate `49c9e3b2` or tree `1a9db127`, tag or publish
`v0.4.4`, repin a consumer, or modernize historical evidence broadly. Do not
alter any pre-existing Paseo workspace or agent.

## Important Context

Read and follow the installed Northstar skill plus
`/Users/tom/Dev/projects/northstar/bundle-docs/operators/retire-roadmap-backlog-prompt.md`.
Recheck queue/worktree ownership before mutation. Freeze and audit the exact
manifest already recorded in `g05.043`; migration does not approve execution.
Create the exact unique persistence note named there, prune the resolved Pi
duplicate only after preserving its canonical meaning, and repair current
inbound links before deleting the backlog tree. Historical references may
remain as provenance when they are not live authority or broken links.

## Suggested Next Move

Run the ownership preflight, materialize the one deferred triage destination,
then apply the deletion and doctrine/checker cleanup as one semantic batch.
Validate only after the coherent cutover is assembled.

## Completion Protocol

Meet every `g05.043` acceptance row. Run repository-native docs checks,
`effigy qa:docs`, `effigy qa:northstar`, `git diff --check`, the exact backlog
directory/string inventories, and any affected deterministic checker fixtures.
Commit and push one reviewable cleanup, open one PR, obtain independent
exact-head review, merge through Northstar Queue, and synchronize `main`.
Return the manifest, exact path inventory, retained historical exceptions,
checks, review/PR/merge identities, and unchanged frontier. No tag or product
continuation follows automatically.
