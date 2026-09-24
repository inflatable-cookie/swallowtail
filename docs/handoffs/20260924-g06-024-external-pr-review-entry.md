---
title: g06.024 — Review external Cursor Agent 2026.09.18
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/024-cursor-agent-2026-09-18-review-entry.md
queue_dispatch: northstar-queue
queue_approval: "Tom, 2026-09-24: approved the rest of the Grok Bot suite after the #358 pilot and attested 'All of the Grok Bot PRs will use Grok 4.6 or 4.7'."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Take over external PR #355 for g06.024 through Queue review entry. The first
agent is the independent reviewer. This handoff supplies canonical scope and
the implementation contract only if review requests revisions.

## Why It Matters

One of five remaining Grok Bot currentness PRs, admitted after the g06.020
pilot. It raises `cursor-agent.release-date` (catalogue, ACP and headless claims) from `2026.09.10-fd3934a` to `2026.09.18-9a7762b`.

## Current State

Repository: `/Users/tom/Dev/projects/swallowtail`; integration branch: `main`.
Task: `docs/roadmaps/g06/024-cursor-agent-2026-09-18-review-entry.md`.
PR: https://github.com/inflatable-cookie/swallowtail/pull/355.
Existing branch: `cursor/cursor-agent-acp-2026-09-18-3ff7`.
External head inspected before planning: `bce44fb7ffc8fc4e3364596fdf766da8ad285ccd`.
Paseo workspace: `wks_5c115e3fef915365` (created for this intake).
Workspace directory: `/Users/tom/.paseo/worktrees/2ee7rnl8/swallowtail-pr355-review-entry`.
No worker exists. Runtime pins the actual head and published planning commit
after planning integration.

The PR names its record Research 331, which `main` already uses; it is
reserved **Research 343**. Planning integration merged `main` into the branch
and resolved shared-file conflicts additively.

## Boundaries

Only this family's window. Owned revision surfaces: the PR's existing changed
paths, the renumbered record, shared-file conflict resolution, and fixtures
and identity tests for an added hop. No other family, suite PR, contract, Next
Task or generation-runway change. Do not edit the task card's prose or this
handoff. No login, install, binary execution, prompt, live session, release or
tag. Preserve the branch and external commits; no new PR or workspace; never
merge from a worker or reviewer.

## Important Context

Operator-declared external author set: `xai/grok-4.6`, `xai/grok-4.7`; GitHub
`betterthanclay`; Cursor cloud agent. Tom's attestation, not runtime-observed.

Four sibling suite PRs run in parallel and touch the same indexes and
`CHANGELOG.md`. A conflict from a sibling landing first is resolved
additively, never by dropping a row.

## Suggested Next Move

Reviewer: check identity and the compatible-extension claim against Contract
029 and the task card, then require the Research 343 renumber. Report through
the authenticated Queue review run.

## Completion Protocol

Retain the same branch and PR. Exact-head independent review is mandatory.
Required evidence: `effigy validate:focused swallowtail-adapter-cursor`,
`effigy package:verify-affected swallowtail-adapter-cursor`, `effigy qa:docs`,
`effigy qa:routes`, and `git diff --check`. No workspace `qa` or live probes.

Queue owns verification, prospective integration, merge, sync and hook-owned
closeout. Revision workers stop after the authenticated ready-for-review
receipt; reviewers stop after reporting their exact-head verdict.
