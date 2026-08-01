# 019 Provider Session Catalogue And Import Foundation

Status: active
Owner: Tom
Created: 2026-08-01
Depends on: g03.018
Vision tags: consumer continuity, harness interoperability, explicit authority
Contract refs: 005, 008-011, 017, 029, 037, 046
Planning state: card 049 ready; cards 050-051 planned

## Problem

Swallowtail can load and resume a provider session only after a consumer holds
an exact binding. It cannot discover harness-origin sessions or turn an
explicitly selected discovery result into that binding. Consumers therefore
see only sessions whose ids they already recorded.

## Goals

- [ ] add separate provider-session catalogue and import roles
- [ ] keep candidates non-authoritative and content bounded
- [ ] bind discovery scope, pagination, cancellation, and cleanup
- [ ] revalidate exact attachment dimensions before issuing a binding
- [ ] reuse existing load, replay, and resume contracts after import
- [ ] provide deterministic provider-neutral conformance and prepared evidence

## Execution Plan

### Batch 19.1 — Records, Capabilities, And Plans

- [ ] Execute card 049.
- [ ] add provider-neutral candidate, cursor, scope, availability, and content
  records
- [ ] add independent catalogue and import capabilities, roles, requirements,
  plans, and request validation
- [ ] prove raw ids and candidates cannot enter existing load or resume paths

### Batch 19.2 — Runtime Roles And Import Outcome

- [ ] Execute card 050 after card 049 passes.
- [ ] add object-safe catalogue and import drivers
- [ ] return the ordinary `SessionResumeBinding` only after import revalidation
- [ ] preserve cancellation, deadline, failure, and cleanup truth

### Batch 19.3 — Common Conformance And Prepared Evidence

- [ ] Execute card 051 after card 050 passes.
- [ ] add local and remote-authoritative synthetic fixtures
- [ ] cover bounds, redaction, cursor drift, candidate drift, stale import,
  replay ordering, and no provider mutation
- [ ] expose common prepared evidence without a provider router

## Boundaries

- no provider adapter implementation in this roadmap
- no consumer database, thread type, prompt, workflow, or UI
- no automatic import, polling, synchronization, merge, or deduplication
- no raw path, prompt, transcript, provider payload, or credential diagnostic
- no management-binding persistence or public serialization codec
- no archive, restore, delete, fork, rename, export, or active-handle control
- no default provider, route, resource, model, access profile, or scope

## Acceptance Criteria

- [ ] catalogue observation and import authority are distinct types and roles
- [ ] one exact prepared scope bounds every candidate and cursor
- [ ] provider content is bounded and absent from stable diagnostics
- [ ] import validates route, host, access, version, model, resource, and policy
- [ ] missing, stale, malformed, or mismatched candidates issue no binding
- [ ] imported bindings use unchanged load/replay/resume semantics
- [ ] synthetic conformance passes under both host identities
- [ ] focused and affected-package validation passes without a broad suite

## Next Planning Checkpoint

After card 051, start g03.020 with the exact Codex catalogue range corpus. Do
not begin a provider mapping before the common negative conformance passes.
