# 038 Canonical Main Candidate

Status: in progress
Owner: Tom
Created: 2026-07-25
Milestone: `../013-canonical-source-provenance-and-final-candidate.md`

## Objective

Commit the accepted Swallowtail source tree locally on `main`, then build and
verify a candidate from that exact normal-history commit.

## Governing Refs

- Contract 036
- card 037
- retained synthetic candidate

## Scope

1. Confirm the live source differs from the frozen candidate only by accepted
   closeout and provenance-repair files.
2. Commit all accepted source files except generated candidate evidence.
3. Stage a new candidate without replacing the active candidate.
4. Verify source ancestry, package checksums, file lists, and extraction.
5. Promote only after verification; retain the prior candidate as superseded.
6. Do not push `main`, create a tag, upload a crate, or create a release.

## Acceptance Criteria

- [ ] local `main` advances by one normal commit from the prior base
- [ ] the candidate source commit equals local `main`
- [ ] the candidate source parent equals the prior base
- [ ] all 23 packages and audited file lists reproduce
- [ ] the old candidate remains immutable and indexed

## Validation

- `effigy package:check`
- isolated candidate preparation
- `effigy package:candidate:verify`
- source-tree and checksum comparison

## Stop Conditions

- unrelated working-tree content is detected
- the source commit is root, detached, dirty, or not based on current `main`
- candidate evidence differs after regeneration
- any external mutation becomes necessary

## Auto-Continuation

Yes. Continue to card 039 after atomic candidate promotion.
