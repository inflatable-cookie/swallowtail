# 016 Replacement Release Candidate And Handoffs

Status: superseded
Owner: Tom
Created: 2026-07-24
Milestone: `../006-consumer-runtime-proof-and-candidate-replacement.md`
Superseded by: `036-replacement-release-candidate-and-handoffs.md`

## Objective

Replace the unpublished `0.1.0` candidate with one built from the prepared
facade and proven by deterministic consumer runtime evidence.

This card was not executed. Contract 037 now requires provider-wide prepared
facade evidence before candidate freeze. Card 036 owns the replacement after
that work.

## Governing Refs

- Contracts 029 and 036-037
- completed card 015
- release and package topology architecture
- existing candidate evidence and release authority

## Scope

1. Classify the final public API delta against the retained unreleased
   candidate.
2. Re-run package metadata, dependency, API, docs, MSRV, content, and checksum
   gates from one clean commit.
3. Replace candidate archives and evidence atomically.
4. Update release notes and Nucleus/Soundcheck handoffs to the prepared facade,
   deterministic runtime proof, exact provider ranges, and rollback.
5. Retain old candidate history as superseded evidence rather than presenting
   two active candidates.
6. Stop before registry, owner, credential, upload, tag, push, workflow, or
   release mutation.

## Acceptance Criteria

- [ ] one exact active `0.1.0` candidate remains
- [ ] every archive and evidence checksum matches the clean source commit
- [ ] packaged consumer runtime proof passes
- [ ] public API, MSRV, docs, content, and provider-range evidence passes
- [ ] handoffs contain no stale manual setup
- [ ] superseded evidence is labelled honestly
- [ ] one explicit publication decision remains

## Validation

- `effigy package:check`
- `effigy package:candidate:prepare`
- `effigy package:candidate:verify`
- `effigy package:candidate:consumers`
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Evidence Required

- candidate source commit and complete archive/evidence hashes
- public API classification
- packaged consumer runtime results
- updated release notes and handoffs
- explicit no-external-mutation statement
- roadmap g02.006 closeout log

## Stop Conditions

- any candidate artifact differs from its recorded source
- a consumer runtime proof fails
- release credentials or external mutation become necessary
- a provider range or package boundary changes without contract review
- two active candidates would remain ambiguous

## Auto-Continuation

No. Return the exact replacement candidate to the sole publication decision in
`docs/roadmaps/README.md`.
