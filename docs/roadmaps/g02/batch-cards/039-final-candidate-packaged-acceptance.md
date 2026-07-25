# 039 Final Candidate Packaged Acceptance

Status: completed
Owner: Tom
Created: 2026-07-25
Milestone: `../013-canonical-source-provenance-and-final-candidate.md`

## Objective

Prove the canonical-history candidate across every packaged route and selected
consumer, then return it to the exact publication decision.

## Governing Refs

- Contracts 011, 036-037
- cards 035-038
- provider route matrix

## Scope

1. Run all packaged facade suites.
2. Run Nucleus, Soundcheck, and packaged Codex proofs.
3. Sync canonical textual evidence.
4. Update release, roadmap, card, index, and log currentness.
5. Run one repository validation round.
6. Stop before all external mutations.

## Acceptance Criteria

- [x] all 22 production route proofs pass
- [x] packaged consumer and Codex proofs pass
- [x] exact source, parent, archive, file-list, provider, and consumer evidence
      is recorded
- [x] only one active candidate remains
- [x] one exact publication decision remains

## Validation

- `effigy package:candidate:facades`
- `effigy package:candidate:consumers`
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- any packaged route or consumer proof fails
- active evidence differs from the retained candidate
- credentials, provider calls, registry access, or remote mutation become
  necessary

## Auto-Continuation

No. Return the exact candidate and request the crates.io owner identity plus
bounded publication authorization.

## Execution Evidence

The retained candidate passes 20 packaged prepared-facade suites across all 22
production routes. Exact provider evidence is retained under
`release-candidates/0.1.0/`.

The isolated consumer proof passes Nucleus with 14 tests and two live probes
ignored, Soundcheck with four tests and one installed probe ignored, and the
full packaged Codex suite with 89 tests. The retained evidence names the exact
consumer snapshots and digest used; Swallowtail made no consumer edits.

An initial parallel Soundcheck run exceeded its ten-second fake discovery
deadline while two complete package proofs contended for the machine. The
isolated rerun completed that test in 0.15 seconds. The direct retained-
candidate run completed it in 0.14 seconds. No provider credentials or calls
were used.
