# 033 Anthropic Managed Run Reconciliation And Recovered Cleanup

Status: completed
Owner: Tom
Created: 2026-08-04
Depends on: g03.032
Vision tags: provider continuity, exact recovery, managed resource cleanup
Contract refs: 021-022, 038, 042, 048
Planning state: cards 083-086 completed

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

- [x] remove the unsupported Gemini `HistoryRemoved` confirmation claim
- [x] add portable waiting-state and exact recovered-resource cleanup records
- [x] reconcile one exact Anthropic run through bounded session/event reads
- [x] clean exact inactive recovered resources without implicit interruption
- [x] preserve ordinary Managed Agents delete-on-close behavior

## Execution Plan

- [x] card 083: repair Gemini stored-transcript management truth across runtime,
  prepared evidence, tests, and public route claims
- [x] card 084: add portable run waiting state plus bounded persisted
  owned-resource cleanup binding, role, outcome, and conformance
- [x] card 085: emit Anthropic checkpoints and cleanup bindings before work can
  be lost; realize exact session/event reconciliation and recovered cleanup
- [x] card 086: close prepared profiles, deterministic corpus, public guidance,
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

- [x] Gemini no longer reports deletion truth from a stateful list operation
- [x] checkpoint and cleanup records restore only against the exact prepared route
- [x] Anthropic active, waiting, completed, failed, cancelled, and unknown
  mappings follow exact ordered provider evidence
- [x] incomplete, foreign, stale, oversized, or contradictory history fails closed
- [x] recovered cleanup deletes only an exact inactive session then environment
- [x] ordinary run cleanup and credential-release ordering remain unchanged
- [x] focused and affected-package validation pass

## Lane Runway

Complete. Cards 083-084 removed the false Gemini claim and realized the
portable cleanup kernel. Cards 085-086 realize and accept the selected
Anthropic mapping. The next planning checkpoint is the g03 compatibility
evidence gate.
