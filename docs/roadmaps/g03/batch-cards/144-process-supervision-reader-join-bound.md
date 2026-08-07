# 144 Process Supervision Reader-Join Bound

Status: ready
Owner: Tom
Created: 2026-08-08
Milestone: `../049-hang-and-deadline-closure.md`
Depends on: g03.048

## Goal

Bound host-local process supervision so a descendant holding the stdout or
stderr pipe can never stall `wait()`, `read_output()`, or the supervisor
thread.

## Scope

1. Restructure `supervise_child` (`host-local/src/process_exit.rs:81-124`) so
   the exit state completes and the child is reaped before the reader joins,
   or the joins are bounded.
2. Add a bounded reader-join timeout as a backstop; a timed-out join becomes a
   bounded cleanup outcome, not a hang.
3. Ensure `OutputFuture` resolves terminal truth even when one reader never
   reaches EOF (`host-local/src/output.rs:80-100`).
4. Add a deterministic fixture: a child that spawns a grandchild inheriting
   stdout and stderr, then exits.

## Out Of Scope

- public `ProcessService`/`ProcessHandle` API or diagnostic-code changes
- provider, transport, or route behavior
- force-stop classification (card 145)

## Acceptance

- [ ] the pipe-inheriting-descendant fixture terminates: `wait()` resolves,
      `read_output()` drains to terminal, no thread leak
- [ ] normal processes observe no behavior or timing change
- [ ] a genuinely hanging reader thread is bounded, not joined forever

## Stop Conditions

- stop if the fix requires an API, diagnostic-code, or guaranteed-behavior
  change
- stop if ordinary-process timing regresses materially

## Auto-Continuation

Yes, to card 145 after acceptance and a focused host-local round.

## Validation

- `effigy validate:focused swallowtail-host-local`
- `effigy test:rust` and `effigy check:examples`
