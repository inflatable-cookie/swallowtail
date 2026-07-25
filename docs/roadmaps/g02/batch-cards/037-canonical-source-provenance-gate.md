# 037 Canonical Source Provenance Gate

Status: completed
Owner: Tom
Created: 2026-07-25
Milestone: `../013-canonical-source-provenance-and-final-candidate.md`

## Objective

Make final-candidate preparation retain a clean normal-history commit while
preserving dirty-worktree package verification as a separate local check.

## Governing Refs

- Contract 036
- release and package topology architecture
- publication authorization gate log

## Scope

1. Add clean-source mode to the candidate builder.
2. Keep ordinary local package verification behavior unchanged.
3. Record exact source parent and scope.
4. Make candidate verification reject synthetic-root or dirty-source drift.
5. Do not create, move, or push a Git ref.

## Acceptance Criteria

- [x] local verification still accepts the current working tree
- [x] final candidate preparation fails while source changes remain
- [x] a clean non-root candidate retains its exact `HEAD`
- [x] regeneration preserves commit and archive checksums

## Validation

- `effigy package:verify-local`
- focused temporary-repository provenance checks
- `git diff --check`

## Stop Conditions

- local verification loses working-tree coverage
- final preparation can still create a synthetic source commit
- verification accepts a root source commit
- any external mutation becomes necessary

## Auto-Continuation

Yes. Continue to card 038 when all gates pass.

## Execution Evidence

Ordinary `package:verify-local` still assembles and validates all 23 packages
from the live tracked-plus-untracked source snapshot. Candidate mode now
rejects the same dirty source before package work.

Candidate format v2 records clean source scope, exact commit, and exact parent.
The verifier rejects root history, clones the retained bundle, regenerates the
candidate from its clean `HEAD`, and compares commit, parent, archive, and
file-list evidence.
