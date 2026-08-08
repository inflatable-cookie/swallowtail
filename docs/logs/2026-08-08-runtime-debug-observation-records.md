# Runtime Debug Observation Records

Date: 2026-08-08
Roadmap: g03.055
Card: 169

## Outcome

Runtime realizes Contract 053's observation vocabulary and emit path.

`DebugObservation` / `DebugObservationKind` carry optional request, scope,
run, turn, and session correlation, route and stage labels, an optional
correlated safe code, and a 4096-character bounded detail body with an
explicit truncation marker. Default `Display` and `Debug` redact the detail
body. `DiagnosticObserver::observe_debug` defaults to a no-op so existing
hosts keep compiling. `HostServices::emit_diagnostic` and
`emit_debug_observation` no-op when unregistered and swallow observer panics
so debug sinks cannot alter terminal or cleanup truth. Testkit records
`DebugObserve`.

The v0.3.0 candidate semantic API baseline was regenerated for the additive
runtime and testkit surface.

## Local Validation

- `effigy validate:focused swallowtail-runtime`: 166 passed
- `effigy validate:focused swallowtail-testkit`: 83 passed
- `effigy package:verify-affected swallowtail-runtime`: extracted package
  proof passed
- `effigy package:api`: 28 packages at the regenerated v0.3.0 candidate
  baseline

## Boundaries

No adapter emission sites, guide text, consumer-repo commits, tag, or
release. Card 170 owns the Codex proof.
