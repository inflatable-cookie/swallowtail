---
title: g06.012 — Review external Qwen Code 0.24.2 PR
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/012-qwen-code-0-24-2-useful-newer.md
queue_dispatch: northstar-queue
queue_approval: "Tom authorized the Queue Oracle on 2026-09-21 to prepare and take over the remaining externally authored currentness PRs through review entry, accepted the pilot-first sequence, attested Grok 4.6 as the author model for the cloud sessions, and explicitly directed that the failing Qwen lane be admitted and repaired through the ordinary revision path."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Take over existing external PR #352 for g06.012 through Queue review entry. The
first agent is the independent reviewer. This handoff supplies canonical scope
and the implementation contract only if review requests revisions.

## Why It Matters

Review externally authored qualification through the ordinary exact-head,
verification, prospective-merge, sync and lifecycle closeout gates, preserving
its work instead of recreating it. This lane is admitted with a known failing
check, so it is also the first exercise of that path on non-approval evidence.

## Current State

Repository: `/Users/tom/Dev/projects/swallowtail`; integration branch: `main`.
Task: [g06.012](../roadmaps/g06/012-qwen-code-0-24-2-useful-newer.md).
PR: https://github.com/inflatable-cookie/swallowtail/pull/352.
Existing branch: `cursor/qwen-code-0-24-2-2299`.
External head inspected before planning:
`e09ebb3fefaffb8e15a611ec7be74f56a96c48c5`.
Paseo workspace: `wks_7f594596f0cd3e7b`.
Workspace directory:
`/Users/tom/.paseo/worktrees/2ee7rnl8/swallowtail-pr352-review-entry`.
No worker exists. Runtime pins the actual head and published planning commit
after planning integration. Do not use the old external head as a
post-preparation review pin. `Stable nextest` is failing at this head; Tom
directed that the lane be admitted and repaired through the ordinary revision
path, and a failing check is evidence for review or revision, never a reason to
bypass a merge gate.

## Boundaries

Only the `qwen-code.package` and `qwen.headless` qualification to `0.24.2`.
Owned revision surfaces are this PR's existing Qwen selection source, Qwen
identity and prepared-facade tests and fixtures in
`crates/swallowtail-adapter-qwen/`, the Qwen headless prepared guide, affected
provider and activity matrix rows, the CHANGELOG entry, the lane's research
record and its indexes/logs. No other route claim, Alibaba or Qwen hosted
family, workflow, dependency, release or provider mutation. Do not edit the
task card outside its generated lifecycle block or rewrite this handoff. Queue
hooks own lifecycle records and canonical closeout. No login, install,
downloaded binary execution, provider prompt or live probe. Preserve the
existing branch, external commits and workspace. Do not create another PR or
workspace, and do not merge from a worker or reviewer.

## Important Context

Operator-declared external author: `xai/grok-4.6`; GitHub `betterthanclay`;
Cursor cloud driven by Grok Bot. This is Tom's attestation, not runtime-observed
author identity. Planning preparation adds no product implementation and
renumbers the lane's research record, which collided on `331` with earlier
lanes.

The failing nextest check is inside this lane's changed surfaces and is expected
to be repaired honestly rather than skipped or weakened. The remaining external
PR after this one is Claude Agent ACP, held until last with its Contracts 015
and 038 edits needing explicit scope attention. External validation claims must
be checked against exact-head evidence.

## Suggested Next Move

Reviewer: inspect the complete PR against g06.012 and Contract 029, including
identity-before-claim history, the per-hop inventory and fixtures, the raised
ceiling, the truthful gap set, the unchanged baseline and probed exact sets, and
documentation truth. Report through the authenticated Queue review run,
including the failing check. If changes are requested, Queue creates a worker in
this workspace to repair only those findings.

## Completion Protocol

Retain the same branch and PR. Exact-head independent review is mandatory.
Required evidence: `effigy validate:focused swallowtail-adapter-qwen`,
`effigy package:verify-affected swallowtail-adapter-qwen`, `effigy qa:docs`,
`effigy qa:routes`, and `git diff --check`. The failing `Stable nextest` job
must pass on the final head. No workspace `qa` or unrequested live probes.

Queue owns verification, prospective integration, merge, sync and hook-owned
closeout. Revision workers stop after the authenticated ready-for-review
receipt; reviewers stop after reporting their exact-head verdict. Neither
consumes the handoff or rewrites planning to bypass a gate.
