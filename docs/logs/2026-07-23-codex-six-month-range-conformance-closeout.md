# Codex Six-Month Range Conformance Closeout

Date: 2026-07-23
Card: `../roadmaps/g01/batch-cards/119-codex-six-month-range-conformance-and-closeout.md`

## Outcome

Roadmap 039 is complete. One Swallowtail release now supports the qualified
Codex span from `0.80.0` through `0.145.0` through separate exec and app-server
claims, exact private behavior segments, explicit deprecated status, and
closed publication gaps.

The proof keeps exact executable version, protocol facade, behavior revision,
configuration posture, retention, capability, configured instance, and
adapter identity separate. No consumer operation shape changed.

## Conformance

- every frozen baseline, milestone, interior checkpoint, latest boundary, and
  rejection neighbor passes
- `0.82.0`, `0.83.0`, `0.108.0`, and `0.109.0` remain excluded
- prerelease, malformed, missing, and unknown versions fail closed
- deprecated and maintained support status stays observable
- ambient, provider-suppressed, durable-retention, and prohibited-retention
  mismatches reject before harness work
- legacy app-server exposes only stable read-only behavior
- current dynamic tools and workspace roots retain explicit experimental gates
- local and remote-authoritative topology, cancellation, deadline, failure,
  redaction, interruption, and joined cleanup remain covered
- provider-neutral conformance profiles remain unchanged

## Validation

- Codex adapter: 72 passed
- shared configuration boundary: eight passed
- final inventory: 563 tests; 559 passed and four live or installed probes
  remained ignored
- `effigy qa`: passed on the final run
- `effigy doctor`: inherited 19 findings, seven errors and twelve warnings
- `git diff --check`: passed

The first full QA run had one transient OpenCode loopback transport failure.
The exact OpenCode target then passed 5/5 in isolation, and the complete QA
rerun passed. No code was changed or failure waived.

## Risks

- app-server remains an experimental development surface
- legacy exec explicitly accepts ambient configuration
- releases before `0.99.0` explicitly accept durable local retention
- authentication, entitlement, catalogue freshness, and model availability
  remain outside version compatibility
- versions newer than `0.145.0` remain unqualified

## Continuation

Roadmap 040 and card 120 begin the second installed-harness range selection.
OpenCode leads on current evidence because it has a materially different
HTTP/SSE lifecycle and an exact health-version seam. The selection remains an
evidence task, not an implicit support commitment.
