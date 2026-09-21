---
title: g06.013 — Review external Claude Agent ACP 0.79.0 PR
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
roadmap: docs/roadmaps/g06/013-claude-agent-acp-0-79-0-useful-newer.md
queue_dispatch: northstar-queue
queue_approval: "Tom authorized the Queue Oracle on 2026-09-21 to prepare and take over the remaining externally authored currentness PRs through review entry, accepted the pilot-first sequence, attested Grok 4.6 as the author model for the cloud sessions, and directed that this Claude Agent ACP lane be held until last with its Contracts 015 and 038 edits receiving explicit scope and review attention."
queue:
  capability: general
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Take over existing external PR #350 for g06.013 through Queue review entry. The
first agent is the independent reviewer. This handoff supplies canonical scope
and the implementation contract only if review requests revisions.

## Why It Matters

Review externally authored qualification through the ordinary exact-head,
verification, prospective-merge, sync and lifecycle closeout gates. This is the
last admission of the operator-approved suite and the only one that amends
durable contract ranges, so its review is the suite's highest-risk item.

## Current State

Repository: `/Users/tom/Dev/projects/swallowtail`; integration branch: `main`.
Task: [g06.013](../roadmaps/g06/013-claude-agent-acp-0-79-0-useful-newer.md).
PR: https://github.com/inflatable-cookie/swallowtail/pull/350.
Existing branch: `cursor/claude-agent-acp-0-79-fdb5`.
External head inspected before planning:
`29288531a56a04db2fd11418c6eaa0748dde18ac`.
Paseo workspace: `wks_1c688f763523dec0`.
Workspace directory:
`/Users/tom/.paseo/worktrees/2ee7rnl8/swallowtail-pr350-review-entry`.
No worker exists. Runtime pins the actual head and published planning commit
after planning integration. Do not use the old external head as a
post-preparation review pin. All checks are green at this head.

## Boundaries

Only the `claude-agent.acp` qualification to `0.79.0`. Owned revision surfaces
are this PR's existing Claude Agent ACP selection source, its ACP identity and
delta-ledger tests and fixtures in `crates/swallowtail-adapter-claude-agent/`,
the Claude Agent prepared guide, affected provider and activity matrix rows,
Contracts 015 and 038 **only as the qualified-range record**, the CHANGELOG
entry, the lane's research record and its indexes/logs. No other route claim,
Claude Agent SDK sidecar, Claude Code, watcher, workflow, dependency, release or
provider mutation. Do not edit the task card outside its generated lifecycle
block or rewrite this handoff. Queue hooks own lifecycle records and canonical
closeout. No login, install, downloaded binary execution, provider prompt or
live probe. Preserve the existing branch, external commits and workspace. Do not
create another PR or workspace, and do not merge from a worker or reviewer.

## Important Context

Operator-declared external author: `xai/grok-4.6`; GitHub `betterthanclay`;
Cursor cloud driven by Grok Bot. This is Tom's attestation, not runtime-observed
author identity. Planning preparation adds no product implementation and
renumbers the lane's research record, which collided on `331` with earlier
lanes.

Contract attention, by explicit operator direction: Contracts 015 and 038 each
extend their statement of the qualified Claude Agent ACP range from
`0.53.0..=0.76.0` to `0.53.0..=0.79.0`. That is the honest record of a raised
claim, and it is in scope only that far. Any further contract change, or any
change that would make a contract express a rule it did not already hold, is
outside this lane and must be escalated rather than absorbed. The ACP SDK pin
`1.4.0` holds; the Agent SDK pin and capability-gated compaction stay unmapped.

## Suggested Next Move

Reviewer: inspect the complete PR against g06.013 and Contract 029, including
identity-before-claim history, the complete dist inventory, the
unpublished-interior proof for `0.77.1`, `0.78.1` and `0.80.0` where claimed,
the raised ceiling, the unchanged baseline and exclusion `0.58.0`, and both
contract diffs line by line. Report through the authenticated Queue review run.
If changes are requested, Queue creates a worker in this workspace to repair
only those findings.

## Completion Protocol

Retain the same branch and PR. Exact-head independent review is mandatory.
Required evidence: `effigy validate:focused swallowtail-adapter-claude-agent`,
`effigy package:verify-affected swallowtail-adapter-claude-agent`,
`effigy qa:docs`, `effigy qa:routes`, and `git diff --check`. Use existing
exact-head evidence where sufficient; rerun focused checks when changes
invalidate it. No workspace `qa` or unrequested live probes.

Queue owns verification, prospective integration, merge, sync and hook-owned
closeout. Revision workers stop after the authenticated ready-for-review
receipt; reviewers stop after reporting their exact-head verdict. Neither
consumes the handoff or rewrites planning to bypass a gate.
