# 025 Durable Session Resume-Binding Persistence

Status: completed
Owner: Tom
Created: 2026-08-04
Depends on: g03.024
Vision tags: exact lifecycle, consumer integration, persistent sessions
Contract refs: 002, 017, 029, 037, 046
Planning state: cards 066-067 completed

## Problem

A confirmed T3 Code failure shows one consumer thread creating fresh OpenCode
root sessions because it did not persist the exact provider session identity.
Swallowtail keeps exact identity while a handle or binding remains in process,
but exposes no stable restart form for `SessionResumeBinding`.

## Goals

- [x] add one versioned opaque persistence record for ordinary resume bindings
- [x] reject corruption, unsupported versions, bounds, and attachment drift
- [x] prove OpenCode resumes and prompts the original session after simulated
  consumer restart
- [x] keep provider compaction separate from explicit session replacement
- [x] preserve consumer-owned storage, mapping, synchronization, and UI

## Execution Plan

### Batch 25.1 — Portable Codec

- [x] Execute card 066.
- [x] extend Contract 017 and architecture
- [x] implement bounded export and exact-plan restore in `swallowtail-runtime`
- [x] cover round trip, redaction, corruption, version, and drift

### Batch 25.2 — OpenCode Acceptance

- [x] Execute card 067 after the runtime surface settles.
- [x] freeze exact compaction and foreign-session posture
- [x] prove open, persist, restore, resume, and prompt use one provider id
- [x] reconcile public guidance and closeout evidence

## Boundaries

- no consumer database, thread mapping, synchronization, or UI
- no raw-id attachment, title lookup, automatic import, or new-session fallback
- no provider-planned rollover claim
- no provider-session management-binding persistence
- no OpenCode range extension or authenticated provider work
- no consumer repository edits or generation rollover

## Acceptance Criteria

- [x] persistence record is bounded, versioned, opaque, and redacted
- [x] exact adapter, route, host, target, interface, resource, access, model,
  and policy drift rejects before provider work
- [x] corrupted and unsupported records issue no binding
- [x] OpenCode restart proof contains one create and exact-id continuation
- [x] same-session compaction is accepted and foreign identity is not adopted
- [x] focused runtime/OpenCode and affected-package validation pass

## Next Planning Checkpoint

Complete. The sole Next Task has returned to the g03 evidence gate. Management-
binding persistence remains separately deferred.
