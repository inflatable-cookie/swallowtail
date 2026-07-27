# 016 Codex Thread Lifecycle Proof

Status: completed
Owner: Tom
Created: 2026-07-26
Depends on: g02.015
Vision tags: Codex app-server, archive, restore, hard deletion
Contract refs: 011, 017, 029, 032-034, 037-038
Planning state: cards 049-051 complete

## Problem

Current Codex app-server supports thread archive, unarchive, and hard delete,
but Swallowtail maps only start and resume. Lifecycle availability across the
qualified `0.80.0..=0.145.0` window is not yet frozen.

## Goals

- [x] Qualify lifecycle-method introduction and behavior across the maintained
      Codex range.
- [x] Add low-level and prepared archive, restore, and delete operations.
- [x] Preserve hard-delete, descendant, already-absent, notification, and
      uncertainty truth.
- [x] Keep Codex exec and consumer-local thread lifecycle unchanged.

## Execution Plan

### Batch 16.1 — Version Corpus

- [x] Execute card 049 after g02.015 closes.

### Batch 16.2 — Driver And Facade

- [x] Execute card 050 after the lifecycle range is qualified.

### Batch 16.3 — Production Conformance

- [x] Execute card 051 after the driver path is complete.

## Acceptance Criteria

- [x] every supported Codex segment has explicit lifecycle capabilities
- [x] legacy versions remain usable when management is unsupported
- [x] unverified-newer attempts remain visible and consumer-selectable
- [x] archive and restore are reversible and distinct from resume
- [x] delete reports only the exact qualified hard-delete and descendant scope
- [x] local and remote-authoritative host fixtures pass with joined cleanup

## Planning Gap

Research 037 and card 049 resolve the planning gap. Archive exists at the
`0.80.0` baseline, restore begins at `0.92.0`, lifecycle notifications at
`0.104.0`, best-effort descendant archive at `0.123.0`, and strict descendant
hard delete at `0.140.0`.

Unknown and repeatedly fully deleted targets fail. A missing rollout is
tolerated only after Codex otherwise knows the target. Card 050 must preserve
that distinction.

## Closeout

Codex app-server is the first production implementation of the shared
provider-session management role. Prepared session handles return opaque
management authority for applicable new and resumed threads. The caller
closes the handle before selecting one separate archive, restore, or delete
operation.

The production driver preserves response authority, notification
disagreement, provider rejection, after-dispatch uncertainty, exact deletion
strength, version posture, remote host authority, and joined cleanup without
touching rollout files or consumer thread state.
