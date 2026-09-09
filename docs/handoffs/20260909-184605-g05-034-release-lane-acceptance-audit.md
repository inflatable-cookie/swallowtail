---
title: g05.034 release-lane acceptance audit
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom said 'Continue' on 2026-09-09 while docs/roadmaps/README.md named g05.034 as the sole Next Task; the repository continuation rule authorizes this bounded acceptance audit and closeout."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Close `g05.034 Release Lane Simplification` by measuring the completed
`v0.4.4` lane against every target it set. Publish one exact, source-linked
event ledger and an honest pass/fail result. Do not repair the lane.

## Why It Matters

The release changes delivered real improvements, but green CI alone does not
prove the end-to-end target. The record must distinguish candidate work, PR
critical path, repeated hosted runs, merge-to-main qualification, exceptional
Desktop acceptance, unrelated planning migrations, tag-request latency, and
tag execution so the next repair is aimed at measured residue.

## Current State

- Canonical task: `docs/roadmaps/g05/034-release-lane-simplification.md`.
- `v0.4.4` is tagged at `49c9e3b2`; g05.036 and the tag task are complete.
- Card 154 task `d3390475-778e-4850-bcee-7b2ff91e487a` was submitted at
  `2026-09-09T11:08:09.240Z` and records about 47 minutes from 12:08 to
  12:55 BST through candidate handoff.
- Qualifying workflow-dispatch run `34345060452` ran at candidate head
  `1fb5b16c`, created `11:20:50Z`, and completed green around `11:23:19Z`.
- PR run `34348370780` ran `11:58:24Z` to `12:00:56Z`; its critical path was
  2m32s and all required PR jobs passed.
- Repeated workflow-dispatch run `34348374964` was created `11:58:27Z`, ended
  `12:05:06Z`, and failed only Pinned MSRV floor tests on the unchanged tree.
- PR #300 merged at `12:17:52Z`; main-SHA push run `34350208617` ran from
  `12:17:55Z` to `12:21:26Z` and passed all 11 jobs.
- The tag-authorization planning commit was recorded at `15:46:22Z`; tag task
  submission followed at `15:47:25.593Z`. Treat those as durable bounds around
  the operator request/authorization, not as a fabricated exact chat time.

## Boundaries

Documentation and read-only evidence only. Follow the task dispatch table.
Do not edit Rust, workflows, Effigy configuration, release scripts, contracts,
release notes, tags, consumers, providers, or other task bodies. Do not rerun
CI or release gates. Do not create a repair task or change release policy.

## Important Context

Apply the original target literally and score each clause separately:
under 30 minutes from gates green on `main` to tag request presented; one
prepare attempt; one hosted run; PR gate under three minutes; no
rerun-to-green. Define start/end semantics from Contract 036 and the release
playbook. When the exact Chatterbox message time is unavailable, publish the
tightest durable bound and show whether that bound alone proves pass or fail.

The special pre-tag Desktop exact-tree qualification was operator-required for
`v0.4.4`; unrelated flattened-task and backlog migrations also occurred before
the tag request. Separate those contributors, but do not subtract them from
end-to-end elapsed time. A repeated failed run was not repaired into green;
state that precisely rather than calling either “no rerun” or “rerun-to-green”
without qualification.

## Suggested Next Move

Build the UTC ledger from queue detail, GitHub Actions/PR events, Git commit
times, and the two release logs. Calculate durations mechanically, record
source confidence for each boundary, then write the audit log and task result.

## Completion Protocol

Update only the owned worker paths, run `effigy qa:docs`, `effigy
qa:northstar`, and `git diff --check`, then open one PR for independent
exact-head review. The queue owns merge and closeout. Unless the complete
evidence proves every original target, close g05.034 as `stopped` with the
passed and failed clauses explicit; do not mark it complete merely because the
audit itself completed. The coordinator advances the Next Task to the next
already-ready canonical task without inventing repair scope. Return event
sources, durations/bounds, verdicts, review comment, PR/merge, closeout commit,
and the resulting pointer.
