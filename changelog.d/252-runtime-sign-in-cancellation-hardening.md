# 252: Fix runtime sign-in async poll_now panic, DiscoveryCancellation deadlock, and waker leaks

### Fixes

- `swallowtail-runtime`: remove `poll_now()` panics from `connection_lifecycle/sign_in.rs`, add async sign-in entry points (`start_sign_in_async`, `poll_sign_in_async`, `complete_sign_in_async`, `cancel_sign_in_async`), preserve pending poll futures without panicking in synchronous `poll_sign_in`, and release `session.loopback` plus stop and reap `session.process` on timeout, `start_interactive` URL-open failure, `complete_sign_in` error paths, and `cancel_sign_in`.
- `swallowtail-runtime`: deduplicate `DiscoveryCancellation` over `ImmediateCancellation` so waiter notifications wake outside the lock and recover from poisoned locks.
- `swallowtail-runtime`: track `ImmediateCancellation::wait_requested()` registrations by waiter id and remove them on drop so abandoned wait futures do not accumulate wakers.
- `swallowtail-runtime`: signal `active.cancelled` on revocation in `RegisteredToolOperationKernel::require_current` and on `abandon_dispatched_call`.
- `swallowtail-runtime`: recover poisoned `WatcherRegistry` locks in `ModelWatcherRole` and `OperatorWatcherRole`, use a counting waker in `dropped_sender_wakes_a_pending_stream_consumer`, and implement `Display` and `std::error::Error` for `TerminalAlreadySet`.
