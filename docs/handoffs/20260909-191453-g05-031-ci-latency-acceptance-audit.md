---
title: g05.031 CI latency acceptance audit
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom said 'Continue' on 2026-09-09 while docs/roadmaps/README.md named g05.031 as the sole Next Task; this authorizes its bounded evidence-only acceptance audit and closeout. The earlier 2026-09-05 authority for workflow edits remains recorded but is not consumed by this audit."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Close `g05.031 CI Latency` by auditing the completed Card 095 implementation
against every original Acceptance clause. Publish one reproducible timing and
coverage ledger. Do not repair the workflow or reinterpret a failed target.

## Why It Matters

Card 095 made the PR gate visibly faster, but the parent task still has four
unchecked promises. One is potentially contradictory: the task says the full
pinned floor remains on macOS, while current CI places its clippy and test jobs
on Linux. Closing from measured evidence keeps speed, coverage, platform, and
merge-gate strength separate.

## Current State

- Canonical task: `docs/roadmaps/g05/031-ci-latency.md`.
- Card 095 merged through PR #230 as `ba8275eb`; current main is the planning
  commit recorded by queue submission.
- Current PR CI splits stable lint, two count-partitioned nextest shards, a
  dedicated macOS process-spawning shard, contracts, docs/API, pinned clippy,
  supply-chain, source-consumer, and roadmap checks.
- `Pinned MSRV floor tests` is skipped on pull requests and runs on
  workflow-dispatch, `main`, and tags. Both pinned jobs currently use Linux.
- The g05.034 ledger already records PR #300 run `34348370780` at 2m32s,
  workflow-dispatch runs `34345060452` and `34348374964`, and push-to-main run
  `34350208617`. Reuse the source events; do not rerun gates.

## Boundaries

Documentation and read-only evidence only. Follow the task Dispatch table.
Do not edit workflows, nextest configuration, Rust, manifests, Effigy, release
surfaces, contracts, tags, branch protection, or another task. Do not trigger
CI. A contradiction or failed clause is a result, not permission to repair.

## Important Context

For latency, take the latest 20 merged PRs through #305 that have a completed
pull-request `CI` run at the accepted head. Name exclusions and extend backward
only if fewer than 10 valid observations remain. Use run creation-to-completion
as the user-visible all-green clock and also report the critical-path job span.
Report median, p75, range, docs/code strata, cancellations, and reruns. Score
“typical ... about five minutes” from the median; do not hide the tail.

Reconstruct the pre-Card-095 PR command/test inventory from `ba8275eb^1`, then
map each item to exactly where it runs on current pull requests. A skipped
pinned duplicate may still preserve test coverage only if another current PR
path actually executes the same test population. Separately score the literal
full-*macOS* pinned-floor promise from current runner and trigger evidence. For
required checks, prefer repository/GitHub settings plus the PR #230 agreement;
if access cannot prove the configured set, mark that clause unproven.

## Suggested Next Move

Collect the bounded Actions population and PR metadata mechanically, freeze
the source identifiers, calculate timings, then build the command/test and
required-check maps before scoring the four clauses.

## Completion Protocol

Update only the owned worker paths. Run `effigy qa:docs`, `effigy
qa:northstar`, and `git diff --check`, then open one PR for independent
exact-head review. The queue owns merge and closeout. Mark g05.031 complete
only if all four clauses are proven; otherwise close it as `stopped` with each
pass, fail, or unproven clause explicit. The coordinator advances Next Task to
the next already-ready canonical task without inventing repair scope. Return
sample membership, run IDs, timing statistics, coverage and runner maps,
required-check evidence, verdicts, review comment, PR/merge, closeout commit,
and the resulting pointer.
