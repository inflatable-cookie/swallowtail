# 039 Final Candidate Packaged Acceptance

Status: ready
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

- [ ] all 22 production route proofs pass
- [ ] packaged consumer and Codex proofs pass
- [ ] exact source, parent, archive, file-list, provider, and consumer evidence
      is recorded
- [ ] only one active candidate remains
- [ ] one exact publication decision remains

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
