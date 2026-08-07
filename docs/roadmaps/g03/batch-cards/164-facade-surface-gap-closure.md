# 164 Facade-Surface Gap Closure

Status: planned
Owner: Tom
Created: 2026-08-08
Milestone: `../053-claim-and-surface-consistency.md`
Depends on: card 163

## Goal

Close the facade-surface gaps found by the audit, or record an explicit
disposition for each.

## Scope

1. Muse: add `prepare_working_state_restoration` to the prepared facade to
   match its headless peers (claude-agent, cursor, gemini, kimi, qwen), or
   record a disposition in the route matrix and muse guide explaining why the
   route stays replacement-only.
2. Kimi: decide the frozen `#[allow(dead_code)]` REST/WS corpus in
   `adapter-kimi/src/local_server/protocol.rs:3-8` — activate it, gate it
   behind a feature, or delete it with the corpus disposition recorded.
3. Event vocabulary: consolidate `core::EventEnvelope`/`EventKind` (now
   testkit-only) with `runtime::RuntimeEvent`/`RuntimeEventKind`, or record
   the two-vocabulary posture as intentional in architecture.
4. Verify the 18-adapter `into_parts` and `low_level_driver` surface is
   consistent with the authoring guide (antigravity, cursor, and muse remain
   permitted omissions).

## Out Of Scope

- public API additions beyond the chosen dispositions
- provider, route, or behavior changes

## Acceptance

- [ ] every gap is closed or has an explicit disposition in architecture or
      the route matrix
- [ ] the authoring guide and realized facade surface agree

## Stop Conditions

- stop if a chosen disposition changes a qualified route behavior without a
  separate qualification

## Auto-Continuation

Yes, to the suite planning checkpoint after acceptance.

## Validation

- `effigy qa:docs`, `effigy qa:routes`, `effigy check:examples`
- focused validation per touched adapter
