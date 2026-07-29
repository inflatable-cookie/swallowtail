# 119 Observable Activity Runtime Records

Status: ready
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

- [ ] activity identity is exact within one run or turn
- [ ] complete and completion-only lifecycles are representable
- [ ] content deltas name one activity and one stream
- [ ] reasoning uses summary terminology only
- [ ] callback and direct-tool correlation does not duplicate opaque bodies
- [ ] semantic activity is never coalescible
- [ ] malformed lifecycle transitions fail with safe diagnostics
- [ ] `Debug` and `Display` reveal no activity content or provider reference
- [ ] existing runtime lifecycle tests remain green

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

Continue to card 120 only after focused core and runtime validation passes.

