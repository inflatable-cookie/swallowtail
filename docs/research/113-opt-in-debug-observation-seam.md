# 113 Opt-In Debug Observation Seam

Status: promoted
Owner: Tom
Date: 2026-08-08

## Question

How should Swallowtail expose enough cross-route context to debug harness and
provider communication failures without weakening the safe-diagnostic boundary
or inventing a second consumer event bus?

## Method

Evidence inspected on 2026-08-08:

- Contracts 003, 004, 009, 010, 044, 051 and product guardrails
- `Diagnostic` / `SafeDiagnostic` and `DiagnosticObserver` runtime skeleton
- Codex malformed-inbound closeout (Research-adjacent log 2026-08-07;
  Roadmap g03.047)
- Research 001 diagnostics row: structured safe diagnostics with opt-in
  internal detail under host redaction policy
- Spec 002 event-delivery and diagnostics section

No live provider, credential, or paid operation was used.

## Problem

Consumers often receive only a stable safe code and redacted message when a
harness drifts, misbehaves, or violates an adapter assumption. Bounded safe
excerpts (g03.047) help for some Codex failures, but they cannot carry a
timeline of wire frames, prep stages, process exits, or classification
evidence without contaminating the public diagnostic surface.

The seam already named in authority is unfinished:

| Surface | Realized today |
| --- | --- |
| `SafeDiagnostic` + portable classification | live on failure paths |
| Bounded safe excerpts | live on selected Codex protocol failures |
| `Diagnostic { safe, internal_detail }` | kernel type; rarely populated |
| `HostServices::with_diagnostic_observer` | registry + trait only |
| Adapter calls to `DiagnosticObserver::observe` | effectively unused |

## Recommendation

Realize one opt-in, operation-scoped **debug observation** channel on the
existing host diagnostic observer. Do not grow `SafeDiagnostic` into a wire
dump and do not add a second public event stream.

### Shape

1. Hosts register `DiagnosticObserver` when they want deep context.
2. Runtime exposes structured `DebugObservation` records and a fail-soft emit
   helper that no-ops when no observer is registered.
3. Adapters emit at interesting boundaries: preparation stages, version
   qualification, process start/exit, bounded wire in/out, parse/map failures,
   classification evidence, stderr-ring snapshots, cleanup.
4. Public events, `SafeDiagnostic`, and default formatting stay redacted.
5. Debug emission never changes terminal truth, classification, cleanup, route
   selection, or retry policy.
6. Host owns retention, further redaction, UI, and discard.

### Why this shape

- Contract 010 already authorizes restricted internal details under explicit
  host policy.
- Contract 044 forbids a second observable operation / global bus as the
  activity delivery model.
- Contract 003 already separates safe public message from internal detail.
- Consumer apps (Nucleus, Soundcheck, future Monkey) can opt in per host
  composition without Swallowtail owning product logging.

## Tradeoffs

| Choice | Cost | Benefit |
| --- | --- | --- |
| Structured observations vs free-text only | more vocabulary | cross-route filtering and correlation |
| Opt-in observer vs always-on capture | apps must wire | safe default; no silent payload retention |
| Larger debug bounds than safe excerpts | more host responsibility | useful wire context without poisoning safe messages |
| Trait evolution for `observe_debug` | implementor touch | keeps `observe(Diagnostic)` for simple cases |

## Open Questions Settled For Promotion

- Observation vocabulary lives in runtime beside `DiagnosticObserver`, not in
  the portable public event stream.
- Secrets, tokens, credential paths, and prompt/tool bodies remain excluded
  even from debug observations unless a later contract explicitly qualifies a
  narrower host-private exception. Default debug detail is restricted and
  redacted, not raw.
- Missing observer is success with no emission, never a preparation failure.
- Observer panics or host sink errors must not alter operation lifecycle.

## Remaining Planning Gaps

- Exact first emission sites beyond the Codex proof adapter remain an
  implementation sequencing choice, not a contract gap.
- Consumer-repo host wiring (Nucleus / Soundcheck) is outside Swallowtail
  authority; Swallowtail ships the seam and a guide/example only.

## Promotion Targets

- Contract 053 (new)
- Contract 010 amendment (diagnostic observer duties)
- Architecture system note
- Spec 003 (planning history)
- Roadmap g03.055 and ready implementation cards

## Validation Needs After Implementation

- focused runtime tests for no-op emit, bounded truncation, redacted default
  formatting, and lifecycle non-interference
- Codex fixture proving a malformed-inbound failure still keeps its safe code
  while also emitting at least one correlated debug observation when an
  observer is registered
- `effigy package:api` against the active candidate baseline once the public
  surface lands
