# 051 Provider Session Import Common Conformance

Status: completed
Owner: Tom
Created: 2026-08-01
Milestone: `../019-provider-session-catalogue-and-import-foundation.md`
Depends on: card 050

## Goal

Prove the shared catalogue/import boundary under both host identities before a
provider adapter implements it.

## Scope

1. Add synthetic catalogue and import fixtures to `swallowtail-testkit`.
2. Cover scope, pagination, duplicate, bounds, redaction, stale target, drift,
   cancellation, deadline, disconnect, and cleanup.
3. Chain successful import into existing load replay and resume assertions.
4. Add prepared evidence assertions without a generic provider facade.
5. Compile affected common packages independently.

## Out Of Scope

- real provider wire data or live effects
- consumer persistence, replay merge, synchronization, or presentation
- provider-session lifecycle management

## Acceptance Criteria

- [x] local and remote-authoritative fixtures agree
- [x] raw ids, copied candidates, and cross-plan cursors fail before effects
- [x] provider content never enters diagnostics
- [x] successful import binds exact attachment dimensions
- [x] load replay completes before readiness and resume emits none
- [x] focused and affected-package validation pass
- [x] card 052 becomes the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-testkit`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-testkit`
- `git diff --check`
- no broad workspace suite

## Auto-Continuation

Yes. Continue to card 052 after common conformance passes.

## Evidence

- the public testkit fixture prepares the same catalogue/import chain for local
  and remote-authoritative host identities
- bounded pages reject cross-page duplicates, oversized content and provider
  references, copied candidates, cross-plan cursors, stale targets, and active
  unavailable targets without issuing bindings
- synthetic continuation proves ordered load replay before readiness and no
  replay on ordinary resume
- prepared evidence retains exact access, host, route, resource, policy, and
  interface-version bindings without adding a provider router
- testkit corpus snapshots are package-local, so extracted package validation
  no longer depends on sibling workspace crates
- `effigy validate:focused swallowtail-core swallowtail-runtime
  swallowtail-testkit` passed 248 tests
- `effigy package:verify-affected swallowtail-core swallowtail-runtime
  swallowtail-testkit` passed
- `git diff --check` passed
