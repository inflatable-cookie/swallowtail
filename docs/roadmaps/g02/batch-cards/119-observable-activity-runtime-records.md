# 119 Observable Activity Runtime Records

Status: completed
Owner: Tom
Created: 2026-07-29
Milestone: `../035-observable-agent-activity-kernel.md`
Depends on: card 118

## Goal

Add the provider-neutral activity record model and carry it through the
existing runtime event stream.

## Scope

1. Add bounded operation-local activity and opaque provider-item identities.
2. Add activity kinds, lifecycle phases, status, assistant phase, content
   stream kinds, disclosure strength, and correlation records.
3. Add one semantic runtime-event variant for activity observations.
4. Keep activity content bounded and redacted from default formatting.
5. Enforce per-activity ordering, non-regressing status, one completion, and
   no post-completion delta.
6. Preserve existing callback, direct-tool, output, provider-observation,
   terminal, cancellation, deadline, and cleanup APIs.
7. Add focused core and runtime unit tests.

## Out Of Scope

- capability and prepared profile records
- provider adapter mappings
- durable storage, UI grouping, or consumer edits
- raw provider extensions or hidden reasoning
- compatibility aliases for the pre-1.0 event model

## Acceptance Criteria

- [x] activity identity is exact within one run or turn
- [x] complete and completion-only lifecycles are representable
- [x] content deltas name one activity and one stream
- [x] reasoning uses summary terminology only
- [x] callback and direct-tool correlation does not duplicate opaque bodies
- [x] semantic activity is never coalescible
- [x] malformed lifecycle transitions fail with safe diagnostics
- [x] `Debug` and `Display` reveal no activity content or provider reference
- [x] existing runtime lifecycle tests remain green

## Result

- Added bounded runtime activity ids, namespaced unknown kinds, and opaque
  provider activity references.
- Added exact run or turn ownership, lifecycle phase, status, assistant phase,
  disclosure, content stream, delta or replacement-snapshot, and correlation
  records.
- Added one semantic `RuntimeEventKind::Activity` variant on the existing
  ordered event stream.
- The ordered buffer now rejects identity drift, repeated starts, status
  regression, repeated completion, post-completion observations, and legacy
  content duplication with safe diagnostics.
- Existing callback, direct-tool, output, provider-observation, terminal,
  cancellation, deadline, and cleanup surfaces are unchanged.
- Core and runtime focused tests pass. The complete Rust workspace compiles
  against the new event variant.

## Validation

- `effigy format:check`
- `cargo test -p swallowtail-core`
- `cargo test -p swallowtail-runtime`
- `effigy check:rust`

## Stop Conditions

- Stop if activity content cannot remain outside safe diagnostics.
- Stop if operation ownership requires consumer thread identity.
- Stop if existing event transport cannot preserve semantic backpressure.

## Auto-Continuation

Continue to card 120. Focused validation passes.
