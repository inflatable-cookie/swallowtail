# 164 Facade-Surface Gap Closure

Status: done
Closeout: 2026-08-08
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

- [x] every gap is closed or has an explicit disposition in architecture or
      the route matrix
- [x] the authoring guide and realized facade surface agree

## Closeout

Every audit gap received a disposition (two via operator decision):

1. Muse `prepare_working_state_restoration`: recorded as replacement-only in
   the muse guide and route matrix. Muse exposes no interactive session,
   continuation, load, or resume route, so there is no interrupted working
   state to restore; a restoration surface would be a new route
   qualification, not a facade gap.
2. Kimi frozen REST/WS corpus: kept frozen with the disposition recorded
   (operator decision). The protocol module comment now records that the
   corpus stays as card-061 evidence, the interactive lifecycle subset is
   consumed, and the remaining decoders wait for a future interactive
   activation card as a separate qualification.
3. Event vocabulary: recorded as intentional in system architecture
   (operator decision). `core::EventEnvelope` is the portable consumer-facing
   envelope with extension governance; `runtime::RuntimeEvent` is the
   runtime-internal sequenced event with typed evidence and delivery policy.
   Consolidation in either direction would lose one responsibility.
4. `into_parts`/`low_level_driver` surface: verified against the authoring
   guide. Antigravity (neither), cursor (low-level only), and muse
   (low-level only) are now recorded as permitted omissions in the guide and
   route matrix; all other adapters expose the full surface.

No route, provider, or public API change.

### Validation

- `effigy qa:routes` passed (route, lifecycle, 27-solution/34-route feature,
  and activity matrices)
- `effigy qa:docs` passed; `effigy check:examples` clean
- kimi crate check clean (comment-only change)

## Stop Conditions

- stop if a chosen disposition changes a qualified route behavior without a
  separate qualification

## Auto-Continuation

Yes, to the suite planning checkpoint after acceptance.

## Validation

- `effigy qa:docs`, `effigy qa:routes`, `effigy check:examples`
- focused validation per touched adapter
