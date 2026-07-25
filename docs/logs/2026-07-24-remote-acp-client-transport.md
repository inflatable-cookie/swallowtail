# Remote ACP Client Transport

Date: 2026-07-24
Card: `../roadmaps/g01/batch-cards/134-remote-acp-client-transport.md`

## Outcome

The reusable operation-scoped remote ACP client is realized as
`swallowtail-transport-acp-remote`. It is a transport, not a provider or
configured instance. Consumers supply an exact preflight plan, scope,
host-approved endpoint grant, host services, and optional deadline.

The public surface covers explicit `http`/`https` and `ws`/`wss` endpoints.
Scheme and preflight transport must agree. Redirect widening, endpoint probes,
authentication, retries, reconnect, replay, resumption, pooling,
multiplexing, and transport fallback remain absent.

## SDK Audit

Exact source inspection found three maintained-client gaps against Contract
035:

- core and HTTP client channels are unbounded
- the HTTP crate's reqwest feature set does not enable HTTP/2
- the WebSocket client does not retain upgrade-response cookie state

Swallowtail therefore uses `agent-client-protocol = 2.0.0` privately to
cross-check schema while owning bounded HTTP/2 SSE and WebSocket actors. The
exact `agent-client-protocol-http = 2.0.0` package remains a test and server
oracle. No SDK type crosses the crate boundary.

## Realized Lifecycle

Both actors enforce first-message initialization and ACP wire version 1.
Connection-private cookie state uses standard domain, path, and secure rules
with count and byte caps. Frames, stream events, outgoing requests, incoming
callbacks, connection ids, and session ids are bounded.

HTTP uses HTTP/2, separate connection and session SSE streams, `202`
acceptance, correlated callback responses, and explicit `DELETE`. WebSocket
retains upgrade cookies, rejects non-text data frames, handles ping/pong, and
drives explicit close. Cancellation, deadline, and disconnect invalidate the
operation without recovery.

A private current-thread Tokio runtime runs only through host
`BlockingWork`. One scoped host task owns that future. Explicit close joins
the connection task and optional deadline task. Deterministic fixture evidence
confirms blocking work completes and the owning task joins.

## Corpus Evidence

Raw card-133 corpus records supply every client and server ACP message used by
the loopback tests. Hyper drives the HTTP/2 fixture connection; a bounded
channel drives SSE bodies. Async Tungstenite drives the WebSocket fixture.
Neither fixture uses the maintained client as its oracle.

The lifecycle pack covers:

- HTTP/2 initialize, affinity, connection and session streams, callbacks,
  cancellation notification, `DELETE`, and incomplete-SSE disconnect
- WebSocket initialize, affinity capture, sessions, callbacks, cancellation,
  deadline, peer disconnect, and close
- host-owned private runtime completion and join

## Validation

- `cargo test -p swallowtail-transport-acp-remote` — 5 passed
- repeated focused transport lifecycle run — 10 consecutive passes
- `cargo test -p swallowtail-core` — 47 passed
- `cargo test -p swallowtail-protocol-acp --test remote_transport_fixtures` —
  8 passed
- `cargo test -p swallowtail-testkit --test conformance_profiles` — 14 passed
- `cargo clippy -p swallowtail-transport-acp-remote --all-targets -- -D warnings`
- `effigy check:rust`
- `effigy lint:rust`
- `effigy qa:docs`
- `effigy doctor`
- `git diff --check`

Doctor remains at the inherited 19 oversized-file findings: 12 warnings and
seven errors. The generated Cargo cache was cleared during validation after
585,800 stale build files caused filesystem enumeration stalls; no source or
user data was removed.

## Continuation

Card 135 is ready. It adds public portability conformance under local and
remote-authoritative host identities, cross-checks the maintained server
without replacing the raw corpus, runs full QA, and closes roadmap 045 with
the required g01 generation-boundary checkpoint.
