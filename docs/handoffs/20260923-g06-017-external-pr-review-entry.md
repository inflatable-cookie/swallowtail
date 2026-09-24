---
title: g06.017 — Review external Claude Code 2.1.280 identity stop
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/017-claude-code-2-1-280-identity-stop.md
queue_dispatch: northstar-queue
queue_approval: "Tom identified PR #363 as Grok Bot external work on 2026-09-23 and directed its integration, extending his 2026-09-22 cloud-session attestation of xai/grok-4.6 to this PR."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Take over existing external PR #363 for g06.017 through Queue review entry. The
first agent is the independent reviewer. This handoff supplies canonical scope
and the implementation contract only if review requests revisions.

## Why It Matters

Review a standing-lane currentness record through the ordinary exact-head,
verification and hook-owned closeout gates. The record is a stop rather than an
extension, and an unrecorded stop is what lets an unverified point look
maintained.

## Current State

Repository: `/Users/tom/Dev/projects/swallowtail`; integration branch: `main`.
Task: [g06.017](../roadmaps/g06/017-claude-code-2-1-280-identity-stop.md).
PR: https://github.com/inflatable-cookie/swallowtail/pull/363.
Existing branch: `cursor/claude-code-2-1-280-identity-stop-5aa6`.
External head inspected before planning:
`f657151bbd2ceab373f3db009ff4237a573fd35e`.
Paseo workspace: `wks_1690b3319ad37b43`.
Workspace directory:
`/Users/tom/.paseo/worktrees/2ee7rnl8/swallowtail-pr363-review-entry`.
No worker exists. Runtime pins the actual head and published planning commit
after planning integration. Do not use the old external head as a
post-preparation review pin.

The PR is evidence-only, eight files: Research 338, fixtures for
`claude-code-2.1.280`, one identity test, the integration registration, and one
research-index line. It carries no task card and no handoff. Its
`Stable format and lint` check is failing.

## Boundaries

Only the Claude Code `2.1.280` identity record. Owned revision surfaces are this
PR's changed paths: the identity fixtures, the identity test, the integration
registration, Research 338, and its index line. No claim, matrix, admission,
contract, or guide change; no Claude Agent ACP or SDK surface; no watcher;
no workflow, dependency, release, or provider mutation; no Next Task or
generation-runway edit. Do not edit the task card's prose or this handoff — the
generated lifecycle block is hook-owned. No login, install, downloaded binary
execution, provider prompt or live session. Preserve the existing branch,
external commits and workspace; do not create another PR or workspace, and do
not merge from a worker or reviewer.

## Important Context

Operator-declared external author: `xai/grok-4.6`; GitHub `betterthanclay`;
Cursor cloud driven by Grok Bot. That is Tom's attestation, not runtime-observed
author identity.

The record's decision is a **stop**: keep both baselines, claim ids, behavior
revisions and `AllowUnverified`; leave headless and response-only
`latest_qualified` at `2.1.278`; keep `2.1.279` as the synthetic later
`UnverifiedNewer` point and make `2.1.280` `UnverifiedNewer`; keep every
historical unpublished gap, watcher exact `2.1.251`, and every feature-specific
exact set; and do not flatten onto Claude Agent ACP or the Claude Agent SDK pin.

The selected-surface change is what makes this a stop, so the reviewer must
confirm it is named from frozen platform artifacts rather than inferred from a
version bump or changelog. The failing format check is evidence for review or
revision, never a bypass.

## Suggested Next Move

Reviewer: inspect the record against Contract 029 and g06.017, including the
reproduced ceiling, the two-hop identity, the unpublished-neighbour proof, and
the exact `--safe-mode` delta on the selected headless surface; confirm no claim
moves and no neighbouring family is flattened. Report through the authenticated
Queue review run, including the failing check. If changes are requested, Queue
creates a worker in this workspace to repair only those findings.

## Completion Protocol

Retain the same branch and PR. Exact-head independent review is mandatory.
Required evidence: `effigy validate:focused swallowtail-adapter-claude-agent`,
`effigy package:verify-affected swallowtail-adapter-claude-agent`,
`effigy qa:docs`, `effigy qa:routes`, and `git diff --check`. The failing
`Stable format and lint` job must pass on the final head. No workspace `qa` or
unrequested live probes.

Queue owns verification, prospective integration, merge, sync and hook-owned
closeout. Revision workers stop after the authenticated ready-for-review
receipt; reviewers stop after reporting their exact-head verdict.
