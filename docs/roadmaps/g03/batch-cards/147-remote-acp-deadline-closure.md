# 147 Remote ACP Deadline Closure

Status: completed
Owner: Tom
Created: 2026-08-08
Milestone: `../049-hang-and-deadline-closure.md`
Depends on: card 146

## Goal

Make the remote ACP transport's advertised deadline admit a timeout that
actually fires against a hanging peer.

## Scope

1. Set client and connect timeouts on the reqwest builder
   (`transport-acp-remote/src/http.rs:55-62`).
2. Race the deadline against in-flight body reads so
   `WorkerCommand::Deadline` can fire while `bounded_body` awaits
   (`transport-acp-remote/src/http/io.rs:38-52`, `http.rs:85-120`).
3. Bound `connect_async` and close/pong sends in the WebSocket transport
   (`transport-acp-remote/src/websocket.rs:30,72,90-94`), and start the
   deadline task before the initial connect so a hanging connect is
   interruptible (`lib.rs:156-182`).
4. Add transport tests with a non-responding peer asserting deadline
   delivery.

## Out Of Scope

- wire framing, bounds, or codec changes
- provider, route, or consumer behavior changes

## Acceptance

- [x] a non-responding HTTP peer fails within the configured deadline
- [x] a non-responding WebSocket peer fails within the configured deadline
- [x] a hanging initial connect is interruptible
- [x] transport tests pass under `effigy validate:focused
      swallowtail-transport-acp-remote`

## Stop Conditions

- stop if timeout behavior changes qualified transport guarantees for
  well-behaved peers

## Auto-Continuation

Yes, to card 148 after acceptance and a focused transport round.

## Validation

- `effigy validate:focused swallowtail-transport-acp-remote`
- `effigy test:rust`

## Completion Evidence

- `worker::run` and both transport `run` functions take the connection
  deadline and host time service; `race_deadline` races every network await
  against the host's own `wait_until(deadline)`, so the crate never assumes
  a tick rate (`worker.rs`)
- HTTP: the initial connect, every send, body reads, the SSE stream wait,
  and the close DELETE are deadline-raced; a silent peer fails within the
  deadline instead of hanging at any await point (`http.rs`)
- WebSocket: `connect_async` is deadline-raced so a hanging initial connect
  is interruptible; sends, pongs, the incoming wait, and closes are raced
  too (`websocket.rs`)
- `connect_bound` starts the deadline task before the connect resolves and
  races the ready signal against the deadline; a deadline during connect
  returns `DeadlineExceeded` with joined cleanup instead of hanging
  (`lib.rs`)
- three new scenarios: non-responding HTTP peer, hanging WebSocket connect,
  and silent-after-handshake WebSocket peer, each asserting
  `DeadlineExceeded` within the deadline (with a nanosecond-tick test time
  service mirroring the host-local convention)
- well-behaved peers observe the same flow; the existing corpus, cancel,
  close, disconnect, and portability scenarios pass unchanged
- no public API or diagnostic-code change; focused transport round (8),
  workspace nextest (1,493 passed), examples, format, and warnings-denied
  clippy all pass
