# 050 Provider Session Catalogue And Import Runtime

Status: planned
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

- [ ] catalogue returns candidates and cursor evidence only
- [ ] import returns no usable handle and at most one resume binding
- [ ] failed, cancelled, stale, or cleanup-degraded import returns no binding
- [ ] object-safe roles preserve executor neutrality
- [ ] existing session roles and bindings remain source-compatible
- [ ] focused runtime validation passes
- [ ] card 051 becomes the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-runtime`
- `git diff --check`
- no broad workspace suite

## Auto-Continuation

Yes. Continue to card 051 after focused runtime acceptance.
