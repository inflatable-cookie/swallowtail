# 049 Hang And Deadline Closure

Status: planned
Owner: Tom
Created: 2026-08-08
Generation: g03
Depends on: g03.048
Vision tags: correctness, deadlines, safe termination
Contract refs: 009, 010, 035, 051
Planning state: cards 144-147; card 144 ready

## Problem

A verified deep audit found one hang class with independent triggers in
host-local process supervision, runtime coordination, and the remote ACP
transport:

- process supervision joins the stdout and stderr reader threads with no
  timeout, so a child that spawns a descendant inheriting the pipes stalls
  `wait()` and `read_output()` forever and leaks the supervisor thread
  (`host-local/src/process_exit.rs:111-115`, `output.rs:93`)
- a `ForceStop` racing natural exit treats the `kill()` ESRCH on an
  already-exited child as `force_stop_failed`, misreporting a clean exit
  (`host-local/src/process_exit.rs:88-98`)
- `LocalJoinedTask::drop` blocks the consumer thread while joining its worker
  (`host-local/src/task.rs:59-65`)
- two concurrent waiters on `ImmediateCancellation` share one stored waker
  slot, so the first waiter can never wake
  (`runtime/src/cancellation.rs:63-79`)
- dropping `RuntimeEventSender` or `TerminalOutcomeSender` without the explicit
  close call stalls consumers forever (`runtime/src/event_channel.rs:53-58`,
  `runtime/src/outcome.rs`)
- the remote ACP transport sets no client, connect, or body-read timeout, and
  the deadline branch cannot fire while the body read suspends the select
  loop, so a hanging peer defeats the transport's own advertised deadline
  (`transport-acp-remote/src/http.rs:55-62`, `http/io.rs:38-52`,
  `websocket.rs:30`, `lib.rs:156-182`)

## Goals

- [ ] bound process supervision so a descendant pipe holder cannot stall
      `wait()`, `read_output()`, or the supervisor thread
- [ ] report a clean natural exit as success even when a force-stop races it
- [ ] remove the permanent-stall classes in waiter slots and dropped senders
- [ ] make every remote ACP deadline admit a timeout that actually fires

## Execution Plan

- [ ] Execute card 144 (process supervision reader-join bound).
- [ ] Execute card 145 (force-stop truth and task drop disposition).
- [ ] Execute card 146 (waiter-slot and sender-close standardization).
- [ ] Execute card 147 (remote ACP deadline closure).

## Boundaries

- no public API, diagnostic-code, or guaranteed-behavior change
- no provider, route, version-range, transport-selection, or consumer change
- no tag, release, registry publication, or live provider work
- ordinary well-behaved processes observe identical behavior

## Acceptance Criteria

- [ ] a fixture child that spawns a pipe-inheriting descendant cannot stall
      `wait()`, `read_output()`, or a drop
- [ ] a force-stop racing a clean exit reports the natural exit
- [ ] concurrent waiters on one cancellation signal all wake exactly once
- [ ] a dropped sender resolves the pending stream instead of stalling
- [ ] a non-responding remote ACP peer fails within the configured deadline
- [ ] focused and workspace test rounds pass

## Next Planning Checkpoint

The suite planning checkpoint after g03.051: reassess evidence-gate posture
before the scaffolding extraction tranches.
