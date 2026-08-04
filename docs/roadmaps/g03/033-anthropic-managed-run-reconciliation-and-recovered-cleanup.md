# 033 Anthropic Managed Run Reconciliation And Recovered Cleanup

Status: active
Owner: Tom
Created: 2026-08-04
Depends on: g03.032
Vision tags: provider continuity, exact recovery, managed resource cleanup
Contract refs: 021-022, 038, 042, 048
Planning state: card 083 ready; cards 084-086 planned

## Problem

Anthropic Managed Agents retains exact sessions and authoritative events after
local attachment loss, but the current driver exposes no durable checkpoint
before work starts and always deletes resources during ordinary close. The
preceding qualification also found Gemini's management confirmation relies on
a stateful list operation incorrectly described as read-only.

## Generation Runway Goal

Advance g03's exact cross-process recovery lane without combining read-only
observation, callback authority, interruption, or destructive cleanup.

## Goals

- [ ] remove the unsupported Gemini `HistoryRemoved` confirmation claim
- [ ] add portable waiting-state and exact recovered-resource cleanup records
- [ ] reconcile one exact Anthropic run through bounded session/event reads
- [ ] clean exact inactive recovered resources without implicit interruption
- [ ] preserve ordinary Managed Agents delete-on-close behavior

## Execution Plan

- [ ] card 083: repair Gemini stored-transcript management truth across runtime,
  prepared evidence, tests, and public route claims
- [ ] card 084: add portable run waiting state plus bounded persisted
  owned-resource cleanup binding, role, outcome, and conformance
- [ ] card 085: emit Anthropic checkpoints and cleanup bindings before work can
  be lost; realize exact session/event reconciliation and recovered cleanup
- [ ] card 086: close prepared profiles, deterministic corpus, public guidance,
  focused validation, and extracted-package proof

## Boundaries

- no authenticated provider work
- no raw provider id, path, or payload as admission authority
- no message, retry, resume, stream attachment, callback answer, or provider
  request during reconciliation
- no cleanup authority from a checkpoint
- no implicit interrupt or deletion of active, ambiguous, operator-owned,
  foreign, stale, or cross-operation resources
- no generic provider router or capability inheritance

## Acceptance Criteria

- [ ] Gemini no longer reports deletion truth from a stateful list operation
- [ ] checkpoint and cleanup records restore only against the exact prepared route
- [ ] Anthropic active, waiting, completed, failed, cancelled, and unknown
  mappings follow exact ordered provider evidence
- [ ] incomplete, foreign, stale, oversized, or contradictory history fails closed
- [ ] recovered cleanup deletes only an exact inactive session then environment
- [ ] ordinary run cleanup and credential-release ordering remain unchanged
- [ ] focused and affected-package validation pass

## Lane Runway

Card 083 removes the discovered false claim. Cards 084-086 realize the selected
Anthropic mapping as one implementation batch. The next planning checkpoint
returns to g03 compatibility evidence after package acceptance.
