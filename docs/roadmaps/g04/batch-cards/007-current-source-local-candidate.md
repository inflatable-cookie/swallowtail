# 007 Current Source Local Candidate

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../003-current-source-tag-before-readiness.md`
Depends on: card 006

## Goal

Prepare the exact local source-tag candidate and pass credential-free gates.

## Scope

1. Coordinated version, changelog promotion, release notes, and metadata.
2. First-release API baselines for additive packages.
3. Isolated source-consumer proof from the candidate.

## Out Of Scope

- commit, push, or tag
- GitHub Release or crates.io
- readiness facade work

## Acceptance Criteria

- [x] all current-source packages share the selected version
- [x] existing-package APIs stay compatible or the break is reclassified
- [x] all configured credential-free gates pass

## Evidence

- coordinated version `0.3.3` on all 40 packages
- `effigy release prepare --yes --check-gates --version 0.3.3` prepared the
  local candidate and passed all 11 configured gates
- isolated source consumer passed; `.release-prepared.json` is local state
  and is not a source identity
- no annotated tag, GitHub Release, or crates.io mutation

## Validation

Named by Contract 036 and the release-prepare selector. No live provider work.

## Auto-Continuation

No. Operator accepts the local candidate before card 008.
