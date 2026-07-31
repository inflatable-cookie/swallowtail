# 030 Grok Build 0.2.114 To 0.2.117 Range Corpus

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../012-grok-build-0-2-117-range-maintenance.md`
Depends on: card 029

## Goal

Freeze every stable Grok Build point and selected ACP behavior milestone before
widening the production claim.

## Scope

1. Record exact `0.2.114`, `0.2.115`, `0.2.116`, and `0.2.117` launcher,
   platform, executable, and source identities.
2. Record selected ACP initialization facts and relevant official changelog
   deltas.
3. Name the existing and `0.2.117` behavior revisions without changing public
   operations or authority.
4. Add deterministic corpus assertions while exact `0.2.114` remains the
   production boundary.

## Acceptance Criteria

- [x] every stable point has exact package and executable identity
- [x] selected initialization, access, catalogue, lifecycle, and task-control
  evidence is explicit
- [x] corpus data contains no credential, host path, token, or provider payload
- [x] exact `0.2.114` remains the production boundary during corpus acceptance
- [x] focused Grok tests pass
- [x] card 031 becomes sole ready and next

## Validation

- `effigy validate:focused swallowtail-adapter-grok`
- `git diff --check`
- no live provider prompt

## Auto-Continuation

Yes. Continue to card 031 when the corpus settles both behavior segments.

## Evidence

- Research 085
- four exact stable artifact and executable identities
- unchanged selected ACP initialization across the interval
- one task-control behavior milestone at exact `0.2.117`
- 23 focused Grok tests passed in three seconds
- no authentication, provider prompt, or durable provider mutation ran
