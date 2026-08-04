# 066 Session Resume-Binding Persistence Record

Status: completed
Owner: Tom
Created: 2026-08-04
Milestone: `../025-durable-session-resume-binding-persistence.md`
Depends on: card 065

## Goal

Implement Contract 017's versioned, bounded, provider-neutral persistence form
for one exact `SessionResumeBinding`.

## Scope

1. Add an opaque redacted persisted record in `swallowtail-runtime`.
2. Bind export to the exact issuing or accepting preflight plan.
3. Restore only under the exact current plan, resource, and access policy.
4. Reject malformed, oversized, unsupported, corrupted, and drifted records.
5. Cover stable round trip, bounds, redaction, corruption, and mismatch cases.

## Out Of Scope

- consumer storage, mapping, transactions, synchronization, or UI
- provider session lookup, prompt, import, or creation
- provider-session management bindings
- cryptographic authority against the consumer which owns the bytes

## Acceptance Criteria

- [x] one stable versioned record round-trips the exact binding
- [x] default formatting exposes no record or provider value
- [x] every material attachment drift rejects safely
- [x] malformed, oversized, unsupported, and corrupted bytes reject
- [x] no credential, target, resource, prompt, or transcript enters the record
- [x] focused runtime validation passes

## Validation

- `effigy validate:focused swallowtail-runtime`
- `git diff --check`

## Auto-Continuation

Completed. Continue to card 067.
