# 027 Cross-Process Active Operation Reconciliation

Status: completed
Owner: Tom
Created: 2026-08-04
Depends on: g03.025
Vision tags: provider continuity, consumer recovery, lifecycle truth
Contract refs: 009, 017, 021, 022, 042, 046, 048
Planning state: card 070 completed

## Problem

Consumers can persist exact provider-session attachment authority but cannot
reconcile a locally active turn after process loss. UI state can remain stuck,
and consumers may either retry or cancel without provider truth.

## Goals

- [x] separate reconciliation from import, resume, retry, and cancellation
- [x] add portable state, attribution, bounds, plan, request, outcome, and role
- [x] realize Codex exact-turn status and retained-history observation
- [x] realize OpenCode session-scoped status and retained-history observation
- [x] classify every production route by evidence and promotion gate
- [x] close focused and extracted-package validation

## Boundaries

- no prompt replay, implicit retry, or session import
- no cancellation, callback answer, provider-request, or subagent authority
- no terminal claim from session-scoped status
- no consumer persistence, routing, or UI policy
- no authenticated provider work
- no controlled-shutdown detach semantics in this tranche

## Acceptance Criteria

- [x] exact durable binding and runtime turn identify the observation
- [x] terminal state requires exact provider-turn attribution
- [x] replay is bounded, ordered, replacement-shaped, and completeness-labelled
- [x] Codex exact active and terminal outcomes correlate to the requested turn
- [x] OpenCode active and idle outcomes issue no prompt, abort, or delete
- [x] unsupported routes have a concrete promotion gate
- [x] focused core/runtime/Codex/OpenCode and affected-package validation pass

## Next Planning Checkpoint

The lane has returned to the g03 evidence gate. The next recovery expansion is
Kimi local-server cursor-checkpoint qualification or controlled-shutdown
detach semantics, selected only with route evidence and explicit sequencing.
