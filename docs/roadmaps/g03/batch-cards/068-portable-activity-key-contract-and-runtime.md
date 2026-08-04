# 068 Portable Activity Key Contract And Runtime

Status: completed
Owner: Tom
Created: 2026-08-04
Milestone: `../026-portable-activity-key-and-cross-operation-isolation.md`
Depends on: card 067

## Goal

Realize Contracts 009 and 044's composite durable activity identity without
changing provider-native references.

## Scope

1. Add a public `ActivityKey` containing operation owner and activity id.
2. Expose the key from every `ActivityObservation`.
3. Preserve equality, hashing, ordering, cloning, and redacted formatting.
4. Prove repeated activity/provider references under different operations do
   not collide.

## Out Of Scope

- provider id rewriting or consumer transcript identity
- database schema, serialization codec, migration, or global registry
- adapter changes beyond deterministic acceptance

## Acceptance Criteria

- [x] the public key preserves both exact identity dimensions
- [x] `ActivityObservation::key()` cannot omit operation ownership
- [x] equal local/provider ids under two operations produce unequal keys
- [x] key debug formatting remains redacted
- [x] focused runtime validation passes

## Validation

- `effigy validate:focused swallowtail-runtime`
- `git diff --check`

## Auto-Continuation

Completed. Continued to card 069 after the common runtime surface passed.
