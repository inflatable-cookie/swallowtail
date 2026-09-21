---
title: g06.011 — Review external Codex 0.155.1 PR
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/011-codex-0-155-1-useful-newer.md
queue_dispatch: northstar-queue
queue_approval: "Tom authorized the Queue Oracle on 2026-09-21 to prepare and take over the remaining externally authored currentness PRs through review entry, accepted the pilot-first sequence, and attested Grok 4.6 as the author model for the cloud sessions. The OpenCode HTTP pilot is complete and merged; Codex is the next admission."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Take over existing external PR #353 for g06.011 through Queue review entry. The
first agent is the independent reviewer. This handoff supplies canonical scope
and the implementation contract only if review requests revisions.

## Why It Matters

Review externally authored qualification through the ordinary exact-head,
verification, prospective-merge, sync and lifecycle closeout gates, preserving
its work instead of recreating it. The OpenCode HTTP pilot proved the route;
this is the first sequenced admission after it.

## Current State

Repository: `/Users/tom/Dev/projects/swallowtail`; integration branch: `main`.
Task: [g06.011](../roadmaps/g06/011-codex-0-155-1-useful-newer.md).
PR: https://github.com/inflatable-cookie/swallowtail/pull/353.
Existing branch: `cursor/codex-0-155-1-currentness-aa1d`.
External head inspected before planning:
`e064c892c41ac1f1a28ec020b2bc2d00e4c106f9`.
Paseo workspace: `wks_6fc7f04d88cc3ced`.
Workspace directory:
`/Users/tom/.paseo/worktrees/2ee7rnl8/swallowtail-pr353-review-entry`.
No worker exists. Runtime pins the actual head and published planning commit
after planning integration. Do not use the old external head as a
post-preparation review pin. The branch was failing three checks at first
inspection and is green at this head; a failing check would be evidence for
review or revision, never a reason to bypass a merge gate.

## Boundaries

Only the Codex exec and app-server qualification to `0.155.1`. Owned revision
surfaces are this PR's existing Codex selection source and lifecycle selection,
Codex identity and compatibility-corpus tests and fixtures in
`crates/swallowtail-adapter-codex/`, the Codex prepared guide, affected provider
and activity matrix rows, the CHANGELOG entry, the lane's research record and
its indexes/logs. No other route claim, SDK or ACP native pin, workflow,
dependency, release or provider mutation. Do not edit the task card outside its
generated lifecycle block or rewrite this handoff. Queue hooks own lifecycle
records and canonical closeout. No login, install, downloaded binary execution,
provider prompt or live probe. Preserve the existing branch, external commits
and workspace. Do not create another PR or workspace, and do not merge from a
worker or reviewer.

## Important Context

Operator-declared external author: `xai/grok-4.6`; GitHub `betterthanclay`;
Cursor cloud driven by Grok Bot. This is Tom's attestation, not runtime-observed
author identity. Planning preparation adds no product implementation and
renumbers the lane's research record, which collided on `331` with the pilot
lane that retained it.

Only this PR is authorized at this point. The remaining external PRs — Qwen,
whose failing check is repaired through the ordinary revision path, and Claude
Agent ACP last, whose Contracts 015 and 038 edits need explicit scope attention
— are admitted separately and serially after fresh base reconciliation.
External validation claims must be checked against exact-head evidence.

## Suggested Next Move

Reviewer: inspect the complete PR against g06.011 and Contract 029, including
identity-before-claim history, the per-hop artifact and inventory evidence, the
raised ceilings on both axes, the new interior gap `0.154.1`, the unchanged
baseline and historical gaps, and documentation truth. Report through the
authenticated Queue review run. If changes are requested, Queue creates a
worker in this workspace to repair only those findings.

## Completion Protocol

Retain the same branch and PR. Exact-head independent review is mandatory.
Required evidence: `effigy validate:focused swallowtail-adapter-codex`,
`effigy package:verify-affected swallowtail-adapter-codex`, `effigy qa:docs`,
`effigy qa:routes`, and `git diff --check`. Use existing exact-head evidence
where sufficient; rerun focused checks when changes invalidate it. No workspace
`qa` or unrequested live probes.

Queue owns verification, prospective integration, merge, sync and hook-owned
closeout. Revision workers stop after the authenticated ready-for-review
receipt; reviewers stop after reporting their exact-head verdict. Neither
consumes the handoff or rewrites planning to bypass a gate.
