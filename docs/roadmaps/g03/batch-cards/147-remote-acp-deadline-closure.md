# 147 Remote ACP Deadline Closure

Status: planned
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

- [ ] a non-responding HTTP peer fails within the configured deadline
- [ ] a non-responding WebSocket peer fails within the configured deadline
- [ ] a hanging initial connect is interruptible
- [ ] transport tests pass under `effigy validate:focused
      swallowtail-transport-acp-remote`

## Stop Conditions

- stop if timeout behavior changes qualified transport guarantees for
  well-behaved peers

## Auto-Continuation

Yes, to card 148 after acceptance and a focused transport round.

## Validation

- `effigy validate:focused swallowtail-transport-acp-remote`
- `effigy test:rust`
