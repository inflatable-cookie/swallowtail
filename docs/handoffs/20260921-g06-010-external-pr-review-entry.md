---
title: g06.010 — Review external OpenCode HTTP 1.18.31 PR
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/010-opencode-http-1-18-31-useful-newer.md
queue_dispatch: northstar-queue
queue_approval: "Tom authorized the Queue Oracle on 2026-09-21 to prepare and take over the remaining externally authored currentness PRs through review entry, accepted the pilot-first sequence, and attested Grok 4.6 as the author model for the cloud sessions. OpenCode HTTP is the bounded pilot."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Take over existing external PR #351 for g06.010 through Queue review entry. The
first agent is the independent reviewer. This handoff supplies canonical scope
and the implementation contract only if review requests revisions.

## Why It Matters

Review externally authored qualification through the ordinary exact-head,
verification, prospective-merge, sync and lifecycle closeout gates, preserving
its work instead of recreating it. It is the bounded pilot for admitting the
remaining external currentness PRs.

## Current State

Repository: `/Users/tom/Dev/projects/swallowtail`; integration branch: `main`.
Task: [g06.010](../roadmaps/g06/010-opencode-http-1-18-31-useful-newer.md).
PR: https://github.com/inflatable-cookie/swallowtail/pull/351.
Existing branch: `cursor/opencode-http-1-18-31-9a56`.
External head inspected before planning:
`567b4c13742939fda2bb7ecdf4bc4f9d5a56da9f`.
Paseo workspace: `wks_e9ea59af935d98d0`.
Workspace directory:
`/Users/tom/.paseo/worktrees/2ee7rnl8/swallowtail-pr351-review-entry`.
No worker exists. Runtime pins the actual head and published planning commit
after planning integration. Do not use the old external head as a
post-preparation review pin.

## Boundaries

Only the OpenCode `opencode.server` / `opencode.http` qualification to
`1.18.31`. Owned revision surfaces are this PR's existing OpenCode selection
source, OpenCode identity and delta-ledger tests and fixtures in
`crates/swallowtail-adapter-opencode/`, the OpenCode attached prepared guide,
affected provider matrix rows, the CHANGELOG entry, the lane's research record
and its indexes/logs. No other route claim, OpenCode ACP or web-search surface,
workflow, dependency, release or provider mutation. Do not edit the task card
outside its generated lifecycle block or rewrite this handoff. Queue hooks own
lifecycle records and canonical closeout. No login, install, downloaded binary
execution, provider prompt or live probe. Preserve the existing branch,
external commits and workspace. Do not create another PR or workspace, and do
not merge from a worker or reviewer.

## Important Context

Operator-declared external author: `xai/grok-4.6`; GitHub `betterthanclay`;
Cursor cloud driven by Grok Bot. This is Tom's attestation, not runtime-observed
author identity. Planning preparation adds no product implementation and
renumbers the lane's research record, which collided on `331` with the pilot
lane that retained it.

Only this PR is authorized. The remaining external PRs, including the Claude
Agent ACP lane whose contract edits need explicit scope attention and the Qwen
lane with a failing check, are admitted separately and serially after fresh
base reconciliation. External validation claims must be checked against
exact-head evidence; a failing check is evidence for review or revision, never
a reason to bypass a merge gate.

## Suggested Next Move

Reviewer: inspect the complete PR against g06.010 and Contract 029, including
identity-before-claim history, the deterministic per-hop inventory, the raised
published window, the unchanged baseline and gaps, and documentation truth.
Report through the authenticated Queue review run. If changes are requested,
Queue creates a worker in this workspace to repair only those findings.

## Completion Protocol

Retain the same branch and PR. Exact-head independent review is mandatory.
Required evidence: `effigy validate:focused swallowtail-adapter-opencode`,
`effigy package:verify-affected swallowtail-adapter-opencode`,
`effigy qa:docs`, `effigy qa:routes`, and `git diff --check`. Use existing
exact-head evidence where sufficient; rerun focused checks when changes
invalidate it. No workspace `qa` or unrequested live probes.

Queue owns verification, prospective integration, merge, sync and hook-owned
closeout. Revision workers stop after the authenticated ready-for-review
receipt; reviewers stop after reporting their exact-head verdict. Neither
consumes the handoff or rewrites planning to bypass a gate.
