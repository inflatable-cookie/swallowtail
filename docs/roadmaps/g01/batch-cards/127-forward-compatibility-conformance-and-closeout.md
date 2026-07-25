# 127 Forward Compatibility Conformance And Closeout

Status: completed
Owner: Tom
Updated: 2026-07-24
Milestone: `../041-qualified-support-and-newer-version-execution.md`

## Objective

Prove guaranteed qualified windows and permitted newer execution remain
distinct across discovery, preflight, runtime, topology, and diagnostics.

## Scope

- provider-neutral three-way classification assertions
- Codex installed discovery under both host topologies
- OpenCode health and session matching under both host topologies
- qualified, unverified-newer, exclusion, prerelease, gap, and below-baseline
  cases
- unchanged common harness profiles
- full repository QA and doctor delta
- provider-coverage checkpoint recompiled after closeout

## Acceptance Criteria

- [x] no test counts unverified execution as guaranteed support
- [x] newer releases are not rejected solely for exceeding the ceiling
- [x] known-incompatible points still fail before unsafe work
- [x] runtime drift remains explicit and redacted
- [x] one provider-coverage next task remains

## Validation

- focused compatibility and adapter conformance
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Auto-Continuation

No. Return to provider-coverage selection after the policy correction closes.

## Outcome

Reusable conformance distinguishes qualified support from permitted
unverified execution. Codex discovery proves both postures under local and
remote-authoritative process hosts. OpenCode proves exact newer health,
catalogue, and session work under both network host identities. Full QA passes
with 583 inventoried tests: 579 pass and four live probes remain gated.
Doctor returned to the inherited 19 findings after splitting three files that
crossed warning thresholds during the batch.
