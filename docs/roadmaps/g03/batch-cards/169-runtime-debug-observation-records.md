# 169 Runtime Debug Observation Records

Status: done
Closeout: 2026-08-08
Owner: Tom
Created: 2026-08-08
Milestone: `../055-opt-in-debug-observation-seam.md`
Depends on: Contract 053

## Goal

Land the structured debug observation vocabulary, fail-soft emit helpers, and
observer API shape in runtime so adapters can opt into host debug sinks
without touching public events or safe diagnostics.

## Scope

1. Add `DebugObservation` / kind / builder types in runtime with operation
   correlation fields, stage label, optional correlated safe code, and bounded
   restricted detail.
2. Preserve `DiagnosticObserver::observe(&Diagnostic)`; add a compatibility-
   preserving path for structured observations (defaulted method or equivalent
   so existing implementors keep compiling).
3. Add a host-services helper that no-ops when no observer is registered and
   never propagates sink failure into operation lifecycle.
4. Prove no-op emission, truncation, redacted default formatting, and
   non-interference with terminal/cleanup fixtures in runtime/testkit.

## Out Of Scope

- adapter emission sites beyond fixtures
- guide or consumer wiring
- public event / activity vocabulary changes
- live provider work

## Acceptance

- [x] Contract 053 runtime acceptance items for records, no-op emit, bounds,
      redaction formatting, and non-interference are covered by tests
- [x] existing `DiagnosticObserver` implementors in-repo still compile
- [x] focused runtime validation passes
- [x] `effigy package:api` disposition matches the active candidate baseline
      policy for additive observer/debug surface

## Closeout

Runtime now owns `DebugObservation` / `DebugObservationKind` with 4096-char
bounded detail, redacted `Display`/`Debug`, and correlation builders.
`DiagnosticObserver::observe_debug` defaults to a no-op; `HostServices`
adds fail-soft `emit_diagnostic` / `emit_debug_observation` that swallow
observer panics. Testkit records `DebugObserve`. The v0.3.0 candidate
semantic API baseline was regenerated for the additive surface.

## Stop Conditions

- stop if the trait shape would force a silent behavior change on ordinary
  hosts that ignore debug observations
- stop if bounds or redaction rules need a contract amendment beyond 053

## Auto-Continuation

Yes, to card 170.

## Validation

```sh
effigy validate:focused swallowtail-runtime
effigy package:verify-affected swallowtail-runtime
effigy package:api
```

- `effigy validate:focused swallowtail-runtime`: 166 passed
- `effigy validate:focused swallowtail-testkit`: 83 passed
- `effigy package:verify-affected swallowtail-runtime`: extracted package proof passed
- `effigy package:api`: 28 packages at the regenerated v0.3.0 candidate baseline
