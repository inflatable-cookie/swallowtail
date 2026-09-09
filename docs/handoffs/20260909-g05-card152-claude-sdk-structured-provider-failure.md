---
title: g05 Card 152 Claude SDK structured provider failure
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-09
updated: 2026-09-09
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom authorized the provider-free projection repair and one final zero-credit diagnostic by saying 'Do it' on 2026-09-09."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
tags: [coordination, handoff, worker, claude-agent-sdk, diagnostics, billing, provider-free]
---

## What This Thread Was Doing

Implement Swallowtail Card 152: retain the Claude Agent SDK's safe structured
provider-failure fields and classify only what they prove.

## Why It Matters

Card 316 reached the provider and exposed `api_error_status` plus
`terminal_reason`, but the sidecar kept only their presence. The result became
generic `provider_failed`, preventing a billing-specific return even when the
upstream numeric status may support one.

## Current State

Research 300 and Contract 019 fix the boundary. Exact SDK `0.3.259` exposes a
numeric API error status and terminal reason. The existing sidecar separately
validates `rate_limit_event.status` as `allowed`, `allowed_warning`, or
`rejected`, then collapses it to progress. Official provider documentation says
`402` is billing-specific, while `400` and `429` each mix spend-limit and
non-billing causes.

## Boundaries

Provider-free Swallowtail production repair only. Project the validated status,
bounded terminal reason, and fixed rate state through the private wire and
failed terminal diagnostic. Never forward or match raw result/error prose.
Preserve one-attempt, redaction, MCP, cleanup, and absent-field behavior. No
provider, Desktop, qualification, matrix, candidate, tag, or release work.

## Important Context

Use portable `FailureClassification` only for facts the exact status proves.
`402` may become `EntitlementUnavailable` with configuration change required.
`400` and `429` need distinct safe mixed route codes but cannot become
`QuotaExhausted`. A rate-limit update remains informational and creates no retry
authority. Idle or prior-turn rate state cannot attach to the next result.

## Suggested Next Move

Freeze the exact SDK field domains in fixtures, extend sidecar and strict Rust
wire together, add adverse redaction/reset/classification cases, then run the
two package-scoped Effigy selectors.

## Completion Protocol

Open one PR. Independent exact-head review challenges status ambiguity,
redaction, wire strictness, cross-turn state, portable classification, retry
non-authority, and unchanged MCP/success behavior. Return accepted head, review
comment, merge, exact structured mappings, validation counts, and canonical
closeout. No provider runs. The later Desktop one-shot is separate queue work
under the same operator authorization after this merge supplies an exact SHA.
