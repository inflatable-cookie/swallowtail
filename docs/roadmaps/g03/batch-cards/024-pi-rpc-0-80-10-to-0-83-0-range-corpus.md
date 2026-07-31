# 024 Pi RPC 0.80.10 To 0.83.0 Range Corpus

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../010-pi-rpc-installed-range-closure.md`
Depends on: card 023

## Goal

Freeze every stable Pi package point and selected RPC behavior milestone before
widening the production claim.

## Scope

1. Classify exact `0.80.10`, `0.81.0`, `0.81.1`, `0.82.0`, `0.82.1`, and
   `0.83.0` package identities.
2. Compare strict-LF framing, commands, state, model catalogue, activity,
   usage, retries, direct-bash updates, extensions, and session-cwd evidence.
3. Name exact behavior revisions and selected absences.
4. Add deterministic corpus assertions without changing the production claim.

## Acceptance Criteria

- [x] every stable point has one exact package and source classification
- [x] selected command, stream, activity, and usage deltas are explicit
- [x] direct-bash and persisted-session changes remain outside selected authority
- [x] corpus data contains no host paths, credentials, or provider payloads
- [x] exact `0.80.10` remained the production boundary during corpus acceptance
- [x] focused Pi tests pass
- [x] card 025 becomes sole ready and next

## Validation

- `effigy validate:focused swallowtail-adapter-pi`
- `git diff --check`
- no live provider prompt

## Auto-Continuation

Yes. Continue to card 025 when the corpus settles every range segment.

## Evidence

- six stable package identities and source commits are exact
- `0.81.0`, `0.81.1`, `0.82.0`, and `0.83.0` are separate upstream milestones
- the session-cwd blob is unchanged across the interval
- 41 focused Pi tests passed in three seconds
- no provider prompt or attached state mutation ran
