# 008 Current Source Canonical CI

Status: planned
Owner: Tom
Created: 2026-08-19
Milestone: `../003-current-source-tag-before-readiness.md`
Depends on: card 007; operator acceptance of the local candidate

## Goal

Commit and push the accepted candidate and require canonical CI at the exact
SHA.

## Scope

1. Commit the candidate on the authorized branch.
2. Push and wait for canonical CI.
3. Do not create the tag.

## Out Of Scope

- annotated tag
- GitHub Release or registry
- candidate mutation after push without a new card

## Acceptance Criteria

- [ ] canonical CI passes at the exact candidate commit
- [ ] `HEAD` matches the recorded SHA

## Validation

Canonical CI jobs at the exact SHA.

## Auto-Continuation

No. Card 009 requires separate exact tag authorization.
