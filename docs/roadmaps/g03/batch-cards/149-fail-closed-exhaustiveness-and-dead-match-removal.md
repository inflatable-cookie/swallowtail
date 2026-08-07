# 149 Fail-Closed Exhaustiveness And Dead-Match Removal

Status: planned
Owner: Tom
Created: 2026-08-08
Milestone: `../050-provider-reachable-panic-closure.md`
Depends on: card 148

## Goal

Remove panic traps that couple adapters to shared-enum growth and dead
unreachable arms that must stay in sync by hand.

## Scope

1. Replace the `unreachable!()` fallthrough in Kimi Platform event decoding
   with a fail-closed error for unknown `Payload` variants
   (`adapter-kimi-platform/src/protocol/events.rs:53-55`), so a future third
   shared variant in `swallowtail-protocol-openai-chat` cannot panic.
2. Remove the dead `Ok(AttemptOutcome::Tool(_)) => unreachable!()` arm in
   Anthropic turn handling by converting the first match to its final error
   directly (`adapter-anthropic/src/driver/session/turn.rs:202-243`).
3. Sweep the other five ACP activity `unreachable!()` sites to confirm each is
   guarded by an early return; convert any that are not.

## Out Of Scope

- event-shape, payload, or wire changes
- public API or diagnostic-code changes

## Acceptance

- [ ] unknown shared `Payload` variants fail closed with an error
- [ ] the Anthropic dead arm is gone with no behavior change
- [ ] every remaining guarded unreachable has a local comment stating its
      guard

## Stop Conditions

- stop if removing an arm changes qualified turn behavior

## Auto-Continuation

Yes, to card 150 after acceptance.

## Validation

- `effigy validate:focused swallowtail-adapter-kimi-platform swallowtail-adapter-anthropic`
- `effigy check:examples`
