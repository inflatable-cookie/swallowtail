---
title: g06.020 — Review external Grok Build ACP 1.0.40 window
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/020-grok-build-acp-1-0-40-review-entry-pilot.md
queue_dispatch: northstar-queue
queue_approval: "Tom accepted the Chatterbox recommendation on 2026-09-24 to admit PR #358 as the review-entry pilot for #354-#359, and attested the same day that Grok 4.7 (xai/grok-4.7) wrote it."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Take over external PR #358 for g06.020 through Queue review entry. The first
agent is the independent reviewer. This handoff supplies canonical scope and
the implementation contract only if review requests revisions.

## Why It Matters

#358 is the bounded pilot for six open Grok Bot currentness PRs. It raises the
`grok-build.acp` `AllowUnverified` window from `1.0.30` to `1.0.40`. The rest of
the suite is sequenced on a fresh base after it lands.

## Current State

Repository: `/Users/tom/Dev/projects/swallowtail`; integration branch: `main`.
Task: `docs/roadmaps/g06/020-grok-build-acp-1-0-40-review-entry-pilot.md`.
PR: https://github.com/inflatable-cookie/swallowtail/pull/358.
Existing branch: `cursor/grok-build-acp-qualify-c135`.
External head inspected before planning:
`2eda6ab0e529e0d9eddb51ff81ff2035501a4284`.
Paseo workspace: `wks_084763b09f69d2cc` (created for this intake).
Workspace directory:
`/Users/tom/.paseo/worktrees/2ee7rnl8/swallowtail-pr358-review-entry`.
No worker exists. Runtime pins the actual head and published planning commit
after planning integration.

All CI checks passed at the external head. Known integration defects: the PR
names its record Research 331, which `main` already uses for Claude Code
`2.1.278`; and it conflicts with `main` in `docs/research/README.md`,
`docs/logs/README.md`, and `docs/guides/provider-solution-feature-matrix.csv`.
Official npm `latest` moved to `1.0.41` after the PR.

## Boundaries

Only the Grok Build ACP executable window. Owned revision surfaces: the PR's
existing changed paths, the renumbered research record, the three conflicted
files, and fixtures and identity tests for an added hop. No catalogue pin,
`1.0.4`/`1.0.5` registered-tool courier, contract, other suite PR, Next Task or
generation-runway change. Do not edit the task card's prose or this handoff —
the lifecycle block is hook-owned. No login, install, binary execution, prompt,
live session, release or tag. Preserve the branch and external commits; no new
PR or workspace; never merge from a worker or reviewer.

## Important Context

Operator-declared external author: `xai/grok-4.7`; GitHub `betterthanclay`;
Cursor cloud agent per the PR footer. That is Tom's attestation, not
runtime-observed identity.

Extend to `1.0.41` only as a compatible extension proven per hop. A
selected-surface change is recorded exactly and lands as the compatible prefix;
Contract 029 No Terminal Stop then makes Chatterbox compile the adaptation.
Conflict resolution keeps every `main` row.

## Suggested Next Move

Reviewer: check the identity chain and compatible-extension claim against
Contract 029 and the task card, then require the research renumber and the
conflict resolution if planning integration left any. Report through the
authenticated Queue review run. If changes are requested, Queue creates a
worker in this workspace to repair only those findings.

## Completion Protocol

Retain the same branch and PR. Exact-head independent review is mandatory.
Required evidence: `effigy validate:focused swallowtail-adapter-grok`,
`effigy package:verify-affected swallowtail-adapter-grok`, `effigy qa:docs`,
`effigy qa:routes`, and `git diff --check`. No workspace `qa` or live probes.

Queue owns verification, prospective integration, merge, sync and hook-owned
closeout. Revision workers stop after the authenticated ready-for-review
receipt; reviewers stop after reporting their exact-head verdict.
