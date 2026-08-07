# 145 Force-Stop Truth And Task Drop Disposition

Status: planned
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

- [ ] a force-stop racing a clean exit reports the natural exit
- [ ] the task-drop disposition is explicit and tested where it changes
      behavior

## Stop Conditions

- stop if the natural-exit fix changes guaranteed cancellation semantics

## Auto-Continuation

Yes, to card 146 after acceptance and a focused host-local round.

## Validation

- `effigy validate:focused swallowtail-host-local`
- `effigy test:rust`
