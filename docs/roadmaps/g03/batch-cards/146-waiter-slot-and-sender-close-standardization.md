# 146 Waiter-Slot And Sender-Close Standardization

Status: planned
Owner: Tom
Created: 2026-08-08
Milestone: `../049-hang-and-deadline-closure.md`
Depends on: card 145

## Goal

Remove the permanent-stall classes in runtime coordination: shared waker
slots and senders that stall consumers when dropped.

## Scope

1. Convert the single-waker slots in `ImmediateCancellation::wait_requested`
   (`runtime/src/cancellation.rs:63-79`) and
   `InstalledExecutable` waiters (`runtime/src/installed_executable.rs:72-90`)
   to the `Vec<Waker>` plus `will_wake` dedup pattern already used elsewhere
   in the crate (`runtime/src/outcome.rs:317-323`,
   `runtime/src/output.rs:96-102`, `process_exit.rs:69-75`).
2. Add concurrent-waiter tests: two tasks waiting on one signal must both
   wake exactly once.
3. Give `RuntimeEventSender` and `TerminalOutcomeSender` a `Drop` that marks
   the channel closed and wakes pending consumers, so a lost sender resolves
   the pending stream instead of returning `Pending` forever
   (`runtime/src/event_channel.rs:53-58`, `runtime/src/outcome.rs`).

## Out Of Scope

- public API, diagnostic-code, or event-shape changes
- provider, transport, or route behavior

## Acceptance

- [ ] two concurrent waiters on one cancellation signal both wake exactly
      once
- [ ] a dropped sender resolves the pending stream with terminal truth or a
      failure
- [ ] no consumer can rely on a dropped sender stalling (document the change)

## Stop Conditions

- stop if waking on drop changes ordinary drain semantics for in-flight
  consumers

## Auto-Continuation

Yes, to card 147 after acceptance and a focused runtime round.

## Validation

- `effigy validate:focused swallowtail-runtime`
- `effigy test:rust`
