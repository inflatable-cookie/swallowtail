# 068 Selected Route Contract And Fixtures

Status: blocked
Owner: Tom
Updated: 2026-07-21
Milestone: `../020-post-portability-coverage-expansion.md`

## Objective

Promote the smallest shared boundary required by card 067's selected route and
freeze its deterministic protocol or SDK corpus.

## Scope

- provider-neutral contract delta only where current contracts are incomplete
- exact version, transport, endpoint, credential, support, and capability pins
- bounded success, failure, cancellation, drift, redaction, and cleanup
  fixtures
- explicit exclusions and live-test gate

## Acceptance Criteria

- [ ] the selected route and authority are fixed by card 067
- [ ] shared rules contain no provider identity branch
- [ ] deterministic fixtures fail closed on version and capability drift
- [ ] live credentials and external inference remain outside default QA

## Validation

- focused fixture tests
- `effigy qa`
- `effigy doctor`
- `git diff --check`

## Stop Conditions

- card 067 is incomplete
- the selected route needs unresolved product policy
- stable fixtures require live credentials or paid inference

## Auto-Continuation

Continue to card 069 only when the production boundary is unambiguous.
