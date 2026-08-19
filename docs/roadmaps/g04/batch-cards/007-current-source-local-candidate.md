# 007 Current Source Local Candidate

Status: planned
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

- [ ] all current-source packages share the selected version
- [ ] existing-package APIs stay compatible or the break is reclassified
- [ ] all configured credential-free gates pass

## Validation

Named by Contract 036 and the release-prepare selector. No live provider work.

## Auto-Continuation

No. Operator accepts the local candidate before card 008.
