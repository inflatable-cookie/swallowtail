# 149 Fail-Closed Exhaustiveness And Dead-Match Removal

Status: completed
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

- [x] unknown shared `Payload` variants fail closed with an error
- [x] the Anthropic dead arm is gone with no behavior change
- [x] every remaining guarded unreachable has a local comment stating its
      guard

## Stop Conditions

- stop if removing an arm changes qualified turn behavior

## Auto-Continuation

Yes, to card 150 after acceptance.

## Validation

- `effigy validate:focused swallowtail-adapter-kimi-platform swallowtail-adapter-anthropic`
- `effigy check:examples`

## Completion Evidence

- Kimi Platform event decoding now uses a plain exhaustive two-arm match on
  the shared compatible-chat `Payload` enum; the `Payload` enum is not
  `#[non_exhaustive]`, so adding a third shared variant fails the build at
  compile time instead of becoming a provider-triggered panic
  (`adapter-kimi-platform/src/protocol/events.rs`)
- the Anthropic turn handling owns one conversion location: the first
  match no longer pre-converts the tool outcome, the terminal match now
  converts `Ok(AttemptOutcome::Tool(_))` directly to the same
  `ProviderFailed` outcome, and `submitter.abandon()` still fires for the
  tool-failure case through an explicit condition; the dead `unreachable!()`
  arm is gone with identical terminal and exchange behavior
  (`adapter-anthropic/src/driver/session/turn.rs`)
- all five ACP activity `unreachable!` sites (kimi, claude-agent, gemini,
  grok, cursor) were verified: each is guarded by the early
  `role == AcpMessageRole::User` return and already carries the guard
  comment "user messages returned above"; none needed conversion
- focused rounds for both adapters, workspace nextest (1,495 passed),
  examples, format, and warnings-denied clippy all pass
