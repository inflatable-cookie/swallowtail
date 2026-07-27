# 056 OpenCode Session Deletion Driver And Facade

Status: completed
Owner: Tom
Created: 2026-07-26
Updated: 2026-07-27
Milestone: `../018-opencode-session-deletion-proof.md`

## Objective

Expose qualified OpenCode session deletion through the shared runtime role and
attached prepared facade.

## Governing Refs

- Contracts 014, 017, 029, 037-038
- cards 048 and 055
- existing OpenCode endpoint, health, access, and session binding

## Scope

1. Promote the existing persistent session reference into a management
   binding.
2. Add one typed prepared delete operation; keep archive and restore
   unsupported.
3. Reuse the exact approved endpoint, server version, delegated harness
   access, blocking-work service, deadline, and transport cleanup.
4. Map provider-declared data deletion, missing-target rejection, server
   rejection, and post-dispatch uncertainty from card 055.
5. Preserve attached external service ownership and no server stop authority.
6. Keep catalogue, prompt, SSE, abort, resume, and consumer persistence
   unchanged.

## Acceptance Criteria

- [x] only an exact inactive management binding can dispatch
- [x] endpoint, host, instance, access, and server-version drift stop first
- [x] successful HTTP deletion reports no stronger than provider data deletion
- [x] local cancellation after dispatch cannot confirm provider cancellation
- [x] no service lifecycle, archive, restore, retry, or fallback appears
- [x] prepared and low-level operations share one production path

## Validation

- focused OpenCode range, protocol, driver, and prepared tests
- shared management conformance
- `effigy check:rust`
- `effigy format:check`

## Stop Conditions

- delete requires taking ownership of the attached OpenCode server
- exact access cannot be retained through the operation
- missing-target behavior is too ambiguous for the planned outcome
- implementation would bypass the HTTP API through local files

## Auto-Continuation

Yes after card 055 acceptance. Continue to card 057.

## Outcome

The attached OpenCode HTTP driver now implements the shared
`ProviderSessionManagementDriver` deletion role. Prepared sessions promote
their exact provider reference, configured instance, access evidence, server
version, and working resource into one opaque management binding after open.

The prepared facade accepts only that binding and one explicit delete action.
It reports `ProviderDataDeleted` with `ProviderDefinedDescendants` after an
exact HTTP `true` response. Missing and other 4xx targets are provider
rejections before effect. Malformed success, 5xx, transport loss,
cancellation, or deadline after DELETE dispatch remain unconfirmed.

Deletion rechecks exact health before dispatch, reuses the approved endpoint,
delegated credential, and read-only resource leases, then joins blocking
transport work before cleanup. The attached server remains externally owned.
Archive, restore, retry, fallback, server stop, and local-file deletion remain
unsupported.

## Validation Evidence

- OpenCode protocol tests: 11 passed
- OpenCode prepared-facade tests: 10 passed
- OpenCode shared conformance tests: 3 passed
- `effigy check:rust`: passed
- `effigy format:check`: passed after formatting

Initial parallel OpenCode runs exposed fixture transport failures while the
focused production path passed. Card 057 identified inherited nonblocking
accepted sockets as the cause, repaired the fixture, and completed the full
range, topology, failure, and regression closeout.
