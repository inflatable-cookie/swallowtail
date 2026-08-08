# 053 Opt-In Debug Observation

Status: accepted
Owner: Tom
Updated: 2026-08-08

## Purpose

Give every route one opt-in, host-owned debug observation channel for
restricted communication and lifecycle context without weakening safe
diagnostics, public events, or portable failure classification.

## Boundary

`SafeDiagnostic` remains the stable public code and message. Portable
classification under Contract 051 remains evidence-bounded and independent.
Public run and turn event streams under Contracts 009 and 044 remain redacted
and are not a debug bus.

Restricted debug context travels only through the host
`DiagnosticObserver` registered on the execution-host service set under
Contract 010. Absence of an observer means no debug emission. Missing
observation never fails preparation, start, terminal settlement, or cleanup.

## Observation Record

Runtime owns one structured `DebugObservation` vocabulary used across routes.
Each observation carries:

- optional operation correlation (`request`, `scope`, run, turn, or session ids
  when already established)
- optional route or adapter identity string owned by the emitting surface
- one observation kind
- optional stage or boundary label
- optional correlated exact safe diagnostic code
- one bounded restricted detail body

Observation kinds cover at least:

- lifecycle or preparation stage
- interface-version or qualification evidence
- host process start, exit, or supervision boundary
- wire inbound
- wire outbound
- protocol parse or map failure
- classification evidence
- stderr-ring snapshot
- cleanup boundary

Adapters may omit kinds they never observe. Unknown future kinds require a
contract amendment before they become portable vocabulary.

## Emission Rules

- emission is best-effort and fail-soft
- adapters and runtime helpers call the observer only when registered
- observer panic, blocking, or sink failure must not change terminal status,
  classification, cleanup outcome, cancellation, timeout, detachment, route
  selection, or retry policy
- debug emission is not a substitute for safe diagnostics; failures still
  carry `SafeDiagnostic`
- correlated safe codes, when present, must match an exact code already used
  or about to be used on the public failure path
- observation order is informational; it is not a sequenced public event
  stream and carries no delivery, replay, or completeness guarantee

## Bounds And Redaction

Debug detail is restricted, not raw-by-default:

- each detail body is bounded; truncation is explicit
- secrets, tokens, credential-store paths, API keys, and private continuation
  bytes never enter debug observations
- prompt bodies, tool argument bodies, and assistant output bodies stay out
  unless a later contract qualifies a narrower host-private exception
- host paths and endpoints remain redacted in default detail text
- wire observations may carry protocol method, direction, structural fields,
  and a bounded sanitized excerpt sufficient to diagnose drift
- stderr-ring observations reuse the same sanitizer family as safe bounded
  excerpts

Exact numeric bounds live with the runtime implementation and conformance
fixtures. They may exceed the safe-diagnostic excerpt bound because the host
opted into restricted detail, but they remain finite and testable.

Hosts may apply stricter redaction or discard. Swallowtail does not persist
debug observations and does not own product log retention.

## Relationship To Existing Surfaces

| Surface | Role |
| --- | --- |
| `SafeDiagnostic` | public stable code and message |
| `Diagnostic.internal_detail` | optional restricted detail attached to one diagnostic record |
| `DebugObservation` | structured timeline/context for the host observer |
| Public events / activity | consumer-visible operation progress; stays redacted |
| Host observer | sole opt-in sink for restricted debug context |

`Diagnostic::with_internal_detail` remains valid for simple one-shot restricted
detail. Structured multi-boundary debugging uses `DebugObservation`. Both may
reach the same observer; neither may leak into default formatting or public
events.

## Compatibility

Adding or omitting debug observations must not change:

- safe diagnostic code or message
- portable failure classification
- terminal status or source
- preparation stage
- cleanup outcome
- callback, cancellation, timeout, or detachment behavior
- route, model, credential, access, or authority selection
- public event vocabulary or activity identity

Public API additions for observation records and observer methods are additive
for callers that ignore them. Existing observer implementors must keep
compiling through a defaulted or compatibility-preserving trait shape.

## Acceptance

- runtime fixtures prove no-op emission without an observer
- runtime fixtures prove bounded truncation and redacted default formatting
- runtime fixtures prove observer failure cannot alter terminal or cleanup
  truth
- at least one production adapter emits correlated debug observations on a
  known failure fixture while preserving its exact safe diagnostic
- guides state host opt-in wiring without requiring observer registration for
  ordinary integration
- focused validation passes without live provider work
