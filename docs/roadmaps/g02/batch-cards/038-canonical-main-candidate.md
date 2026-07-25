# 038 Canonical Main Candidate

Status: completed
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

- [x] local `main` advances through normal commits from the prior base
- [x] the candidate source commit equalled local `main` at assembly
- [x] the candidate source records its exact normal-history parent
- [x] all 23 packages and audited file lists reproduce
- [x] the old candidate remains immutable and indexed

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

## Execution Evidence

The accepted source tree differed from the prior frozen package source only by
the provenance contract, architecture, tooling, roadmap, release-currentness,
and closeout files in this lane. Local `main` advanced normally from
`91a0774010ee83594a4565e1b4e2b0daa998db28` to the source recorded in active
candidate evidence.

Candidate format v2 records that exact commit, exact parent, and
`clean-head-excluding-generated-candidate-evidence` scope. Bundle verification
reports complete history. Regeneration reproduces all 23 archive and audited
file-list checksums. The final source is
`f142d927767f49fe86f2737d822fecf182f52591`; its parent is
`e9ead4d35fb7754962053417bf8328e646839b32`. Exact package and evidence values
are retained under `release-candidates/0.1.0/`.

The former parentless candidate moved intact to
`.effigy/release-candidates/superseded/0.1.0-73c7f5b5b561/`. No external state
changed. The first normal-history candidate at `e9ead4d35fb...` also moved
intact to `.effigy/release-candidates/superseded/0.1.0-e9ead4d35fb7/` after
packaged README inspection found stale release-currentness wording.
