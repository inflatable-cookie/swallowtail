# 008 Current Source Canonical CI

Status: completed
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

- [x] canonical CI passes at the exact candidate commit
- [x] `HEAD` matches the recorded SHA

## Validation

Canonical CI jobs at the exact SHA.

## Auto-Continuation

No. Card 009 requires separate exact tag authorization.

## Evidence

- operator accepted the local candidate and authorized merge
- PR 3 fast-forwarded onto canonical `main` at
  `51d186208e75dca4c04f077dd7179ec3c2fafae9`
- `CI` workflow dispatched against `main` because branch pushes do not
  trigger it; run
  https://github.com/inflatable-cookie/swallowtail/actions/runs/32308431817
- all five jobs passed at head SHA `51d186208e75dca4c04f077dd7179ec3c2fafae9`
- local `HEAD`, remote `main`, and the workflow head matched that SHA
- no local or remote `v0.3.3` tag exists
