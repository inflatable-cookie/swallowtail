# 146 Waiter-Slot And Sender-Close Standardization

Status: completed
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

- [x] two concurrent waiters on one cancellation signal both wake exactly
      once
- [x] a dropped sender resolves the pending stream with terminal truth or a
      failure
- [x] no consumer can rely on a dropped sender stalling (document the change)

## Stop Conditions

- stop if waking on drop changes ordinary drain semantics for in-flight
  consumers

## Auto-Continuation

Yes, to card 147 after acceptance and a focused runtime round.

## Validation

- `effigy validate:focused swallowtail-runtime`
- `effigy test:rust`

## Completion Evidence

- `ImmediateCancellation` and `DiscoveryCancellation` now register
  `Vec<Waker>` with `will_wake` dedup and wake all waiters exactly once on
  the first request, closing the lost-wakeup stall for concurrent waiters
  (`cancellation.rs`, `installed_executable.rs`)
- `RuntimeEventSender` tracks a sender count with manual `Clone`/`Drop`; the
  stream closes when the last sender clone drops, exactly like an explicit
  `close()`, so a producer that dies without closing cannot stall a
  consumer; the stream's own waiter slot is also a multi-waker vec
  (`event_channel.rs`)
- `TerminalOutcomeSender` tracks a sender count; when the last clone drops
  without publishing, the pending future resolves to a `RuntimeFailed`
  outcome with code `swallowtail.terminal_sender_dropped` instead of hanging
  forever; a sender that completed first is untouched on drop
  (`outcome.rs`)
- eight new tests: concurrent multi-thread waiter wake-once, repeated-request
  wake-once, no-registration after request, repoll dedup, last-sender stream
  close, dropped-sender wake, terminal drop synthesis, and completion-wins
- in-flight consumer drain semantics are unchanged: senders only close when
  the last producer is gone, and pumps that complete before dropping behave
  identically
- no public API or diagnostic-code change; focused runtime round (150),
  workspace nextest (1,493 passed), examples, format, and warnings-denied
  clippy all pass
