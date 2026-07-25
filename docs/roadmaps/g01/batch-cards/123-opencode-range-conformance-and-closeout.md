# 123 OpenCode Range Conformance And Closeout

Status: completed
Owner: Tom
Updated: 2026-07-23
Milestone: `../040-cross-harness-compatibility-range-expansion.md`

## Objective

Prove the OpenCode HTTP range and compare its portability evidence with the
closed Codex six-month window.

## Scope

- every frozen OpenCode boundary, milestone, exclusion, and representative
  interior point
- authoritative endpoint-health observation under both host topologies
- configuration, retention, capability, failure, and cleanup behavior
- unchanged attached HTTP/SSE provider-neutral profile
- full repository QA, doctor delta, currentness, and closeout log
- no live authentication in default QA

## Acceptance Criteria

- [x] OpenCode publishes one honest closed compatibility posture
- [x] every mismatch rejects after health observation and before authenticated
      catalogue or session work
- [x] current behavior remains regression-covered
- [x] Codex-specific behavior did not leak into shared records
- [x] full validation passes apart from documented structural debt
- [x] the next compatibility or coverage checkpoint is explicit

## Validation

- focused selected-harness and conformance tests
- workspace all-target check
- workspace warnings-denied clippy
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Auto-Continuation

No. Return to the provider-coverage checkpoint after the second range closes.

## Outcome

OpenCode HTTP now publishes one exact closed range from `1.14.48` through
`1.18.4`. All 45 stable releases, 18 selected surfaces, 20 published segments,
semantic gaps, outside neighbors, malformed values, health drift, and session
drift are deterministic.

The unchanged attached-network harness profile covers preflight, lifecycle,
delegated authentication, redaction, cleanup, external ownership, and no
fallback. Baseline and latest health checks pass under local and
remote-authoritative host identities. Current `1.14.48` behavior remains
covered.

Full repository QA inventories 577 tests: 573 pass and four separately gated
live or installed probes remain ignored. Workspace all-target check,
warnings-denied clippy, documentation checks, formatting, and
`git diff --check` pass. Doctor remains at the inherited 19 oversized-file
findings: 12 warnings and seven errors. Two temporary warnings introduced by
the batch were removed by splitting health and selection tests.

Roadmap 040 is complete. Card 124 owns the next provider-coverage checkpoint.
