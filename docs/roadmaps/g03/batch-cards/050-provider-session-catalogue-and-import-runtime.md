# 050 Provider Session Catalogue And Import Runtime

Status: completed
Owner: Tom
Created: 2026-08-01
Milestone: `../019-provider-session-catalogue-and-import-foundation.md`
Depends on: card 049

## Goal

Add object-safe runtime roles whose only authoritative import result is the
ordinary exact `SessionResumeBinding`.

## Scope

1. Add catalogue and import driver traits and boxed futures.
2. Add bounded outcomes, failure stages, cancellation, deadline, and cleanup
   evidence.
3. Require read-only import revalidation before binding issue.
4. Mark imported binding origin without changing load or resume semantics.
5. Expose prepared operation evidence for adapter-local facades.

## Out Of Scope

- provider protocol mapping
- model prompts, history replay during import, or consumer thread creation
- global registry, active-session locking, retries, or fallback

## Acceptance Criteria

- [x] catalogue returns candidates and cursor evidence only
- [x] import returns no usable handle and at most one resume binding
- [x] failed, cancelled, stale, or cleanup-degraded import returns no binding
- [x] object-safe roles preserve executor neutrality
- [x] existing session roles and bindings remain source-compatible
- [x] focused runtime validation passes
- [x] card 051 becomes the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-runtime`
- `git diff --check`
- no broad workspace suite

## Auto-Continuation

Yes. Continue to card 051 after focused runtime acceptance.

## Evidence

- separate object-safe catalogue and import driver traits register only against
  their declared roles
- catalogue outcomes enforce page and traversal bounds, request-local cursor
  history, duplicate rejection, plan identity, and clean completion
- import outcomes require exact read-only revalidation before issuing one
  `ExplicitlyImported` resume binding
- typed failures retain before-dispatch, dispatch, projection, revalidation,
  binding, cancellation, deadline, and cleanup stages
- prepared evidence binds exact access provenance without adding a router
- `effigy validate:focused swallowtail-runtime` passed 107 tests
- `git diff --check` passed
