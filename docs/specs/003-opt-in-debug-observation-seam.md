# 003 Opt-In Debug Observation Seam

Status: promoted
Owner: Tom
Updated: 2026-08-08

## Purpose

Plan the unfinished host diagnostic observer into a cross-route, opt-in debug
observation channel that keeps safe diagnostics and public events redacted.

## Scope

In:

- structured debug observation records
- host `DiagnosticObserver` duties and fail-soft emission
- bounds, redaction, and non-interference rules
- Codex proof adoption as the first adapter evidence
- consumer/operator guidance for opt-in host wiring

Out:

- product logging UX, retention policy, or support workflows
- a second public event stream or global telemetry bus
- widening `SafeDiagnostic` messages into raw wire dumps
- live provider work or consumer-repo commits

## Decisions Needed

Settled by Research 113 and promoted into Contract 053:

1. observer sink, not public event stream
2. structured kinds with operation correlation
3. fail-soft when unregistered
4. restricted/redacted detail by default even on the debug path
5. emission never changes lifecycle or classification

## Acceptance Criteria

- [x] research memo records evidence and recommendation
- [x] Contract 053 and Contract 010 amendment govern the seam
- [x] architecture notes the realized-but-unfinished-to-live path
- [x] roadmap and ready cards sequence runtime, Codex proof, and guide work

## Promotion Targets

- `docs/contracts/053-opt-in-debug-observation.md`
- `docs/contracts/010-execution-host-services-and-inputs.md`
- `docs/architecture/system-architecture.md`
- `docs/roadmaps/g03/055-opt-in-debug-observation-seam.md`
