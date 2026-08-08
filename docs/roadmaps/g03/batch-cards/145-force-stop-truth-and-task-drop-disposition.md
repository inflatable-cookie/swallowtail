# 145 Force-Stop Truth And Task Drop Disposition

Status: completed
Owner: Tom
Created: 2026-08-08
Milestone: `../049-hang-and-deadline-closure.md`
Depends on: card 144

## Goal

Report a clean natural exit as success when a force-stop races it, and
dispose of the consumer-thread-blocking task drop.

## Scope

1. Treat `kill()` failing on an already-exited child as already-exited, not
   `force_stop_failed` (`host-local/src/process_exit.rs:88-98`); the natural
   exit status wins.
2. Add a deterministic race fixture for force-stop versus natural exit.
3. Decide and record the intended disposition for
   `LocalJoinedTask::drop` blocking the consumer thread
   (`host-local/src/task.rs:59-65`): either bound the join with the same
   bounded-join mechanism from card 144, or document the blocking contract as
   intentional in the module docs.

## Out Of Scope

- public API or diagnostic-code changes
- provider, transport, or route behavior

## Acceptance

- [x] a force-stop racing a clean exit reports the natural exit
- [x] the task-drop disposition is explicit and tested where it changes
      behavior

## Stop Conditions

- stop if the natural-exit fix changes guaranteed cancellation semantics

## Auto-Continuation

Yes, to card 146 after acceptance and a focused host-local round.

## Validation

- `effigy validate:focused swallowtail-host-local`
- `effigy test:rust`

## Completion Evidence

- the supervision loop checks `try_wait` before killing, so a natural exit
  that raced the stop wins by construction; a stop is killed and retried
  every tick while the child stays listed, and a child that survives 100
  consecutive failed kills (one second) still reports
  `force_stop_failed`, preserving the old failure surface for the
  unkillable case (`process_exit.rs`)
- `LocalJoinedTask::drop` joins deliberately; the blocking contract is now
  documented on the service and the drop impl, with bounded-shutdown
  guidance (bound the task or join explicitly). Bounding the drop would
  detach running work and change drop semantics, so the disposition is
  document-as-intentional, consistent with the crate's deterministic-join
  posture (`task.rs`)
- race fixture added: an immediately-exiting child raced by `force_stop`,
  repeated twelve times; the invariant asserted is that `wait()` never
  reports `force_stop_failed` on a racing clean exit
- existing force-stop semantics unchanged: a genuinely running child is
  still SIGKILLed and reports an unsuccessful exit
- no public API or diagnostic-code change; focused host-local round,
  workspace nextest (1,485 passed), format, and warnings-denied clippy all
  pass
