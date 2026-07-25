# 134 Remote ACP Client Transport

Status: completed
Owner: Tom
Updated: 2026-07-24
Milestone: `../045-remote-acp-transport-proof.md`

## Objective

Implement the reusable operation-scoped remote ACP client against the exact
maintained Rust SDK schema and independent corpus.

## Governing Refs

- Research 029
- Contracts 009, 010, 014, 015, 019, and 035
- roadmap g01.045
- card 133 corpus and records

## Scope

1. Add workspace crate `swallowtail-transport-acp-remote`.
2. Keep SDK, Tokio, HTTP, TLS, SSE, WebSocket, cookie, and error types private.
3. Consume only the exact operation plan and host-approved endpoint grant.
4. Select HTTP/SSE or WebSocket from the exact endpoint scheme with no probe,
   redirect widening, negotiation, or fallback.
5. Use connection-scoped cookie state for Streamable HTTP and the WebSocket
   upgrade.
6. Do not instantiate the maintained client's unbounded channel paths. Use the
   exact maintained core schema privately with bounded HTTP/SSE and WebSocket
   physical transport actors; retain the HTTP crate as an exact cross-check
   dependency and later server oracle.
7. Bound frames, streams, pending correlation, callback work, and private ids.
8. Map the shared ACP protocol boundary without a provider branch.
9. Run the private Tokio runtime through host `BlockingWork`; a scoped host
   task owns and joins that job.
10. Drive explicit graceful close and join the reader, callback, connection,
   and private executor work on success, failure, cancellation, and deadline.
11. Sanitize endpoint, header, cookie, frame, payload, SDK, and transport errors.

## Boundaries

- no provider adapter or registered configured instance
- no authentication header, query credential, subprotocol credential, or
  credential lease
- no global client, global executor, detached task, pool, or multiplexing
- no automatic reconnect, retry, replay, resumption, or failover
- no live endpoint in default tests
- no consumer edit

## Acceptance Criteria

- [x] one public reusable transport surface covers both explicit endpoint
      schemes without flattening their lifecycle
- [x] every SDK/runtime dependency stays behind the crate boundary
- [x] HTTP cookie affinity is connection-scoped and redacted
- [x] explicit close, cancellation, deadline, disconnect, and cleanup pass the
      independent corpus
- [x] no provider, model, route, credential, or transport fallback exists
- [x] card 135 can run portability conformance without design changes

## Validation

- focused transport, protocol, runtime, and testkit tests
- workspace all-target check
- workspace warnings-denied clippy
- `git diff --check`

## Evidence Required

- exact dependency lock evidence
- lifecycle and cleanup test outcomes
- proof that the raw corpus, not an SDK server alone, drives expectations
- public dependency-direction review

## Stop Conditions

- normal SDK use requires detached cleanup
- the maintained core schema cannot be used without an SDK type leak or an
  unbounded runtime actor
- WebSocket use requires an authentication or ambient configuration source
- WebSocket upgrade cookies cannot be retained without detached or ambient
  state
- SDK types must leak into core, runtime, testkit, or provider adapters

## Auto-Continuation

Yes, after both transports pass the independent corpus and all owned work
joins.

## Outcome

Completed 2026-07-24.

- Added `swallowtail-transport-acp-remote` as the twenty-second workspace
  crate with one preflight-bound public client and distinct HTTP/2 SSE and
  WebSocket private actors.
- Exact `agent-client-protocol = 2.0.0` validates private wire schema.
  `agent-client-protocol-http = 2.0.0` remains an exact test oracle rather
  than the production actor because its client paths are unbounded, do not
  enable HTTP/2, and discard WebSocket upgrade state.
- Connection-private cookies, frames, streams, outgoing requests, incoming
  callbacks, connection ids, and session ids are bounded. Initialization must
  be first and must confirm ACP wire version 1.
- Raw corpus-driven loopback fixtures prove callbacks, affinity,
  cancellation, deadline, disconnect invalidation, explicit close, and
  host-owned joined private runtime work without authentication or recovery.
- Focused transport tests and warnings-denied Clippy pass. Workspace
  validation and the unchanged doctor baseline are recorded in the closeout
  log.

Card 135 is ready without a design change.
