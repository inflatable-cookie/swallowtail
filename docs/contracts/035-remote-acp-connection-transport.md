# 035 Remote ACP Connection Transport

Status: active
Owner: Tom
Updated: 2026-07-24

## Purpose

Permit provider adapters to compose one host-approved remote ACP connection
without treating a shared protocol as an integration family, hiding network
authority, or claiming reconnect and authentication behavior the current
transport does not define.

## Authority And Maturity

The first boundary binds:

- ACP wire version 1
- the Active Streamable HTTP and WebSocket transport RFD as observed
  2026-07-24
- `agent-client-protocol-http = 2.0.0`
- the matching `agent-client-protocol = 2.0.0` compile-time SDK
- `ExperimentalObserved` support authority

An official SDK release does not make every remote ACP deployment
provider-supported or wire-stable. Consumers must explicitly accept the
experimental support authority. A later provider adapter may publish stronger
authority only from its own endpoint, agent, version, and provider evidence.

## Identity And Placement

Remote ACP is a transport family, not an integration family or generic
provider driver. One provider-specific adapter still supplies:

- integration family and adapter driver identity
- configured instance and exact endpoint reference
- remote agent identity and version evidence
- model route and capability posture
- access profile, entitlement, and support authority
- session behavior and provider extensions

The reusable implementation belongs in
`swallowtail-transport-acp-remote`. It may depend privately on the maintained
Rust SDK, Tokio, HTTP, TLS, SSE, WebSocket, and cookie implementations.
`swallowtail-protocol-acp` retains provider-neutral ACP wire records and
fixtures without those runtime dependencies.

The maintained `agent-client-protocol-http = 2.0.0` client is not the
production transport actor for the first proof. Its HTTP/SSE and core channel
paths contain unbounded queues, its reqwest dependency does not enable HTTP/2,
and its WebSocket path does not retain upgrade-response cookie state. The
shared crate uses the exact maintained core schema privately, owns bounded
physical transport actors for both selected transports, and retains the HTTP
crate as an exact cross-check dependency and server oracle.

SDK types, executors, clients, errors, endpoint values, headers, cookies, raw
frames, and provider payloads do not enter core, runtime, testkit, or stable
consumer-facing records.

## Endpoint And Access

Preflight binds one exact configured instance, operation, execution host,
endpoint reference, endpoint audience, access profile, driver, and route before
network work.

Contract 014 supplies one scoped `NetworkGrant`. The transport uses its exact
host-approved endpoint and cannot alter scheme, authority, base path, proxy,
TLS policy, or host placement.

The endpoint scheme selects one transport:

- `http` or `https`: HTTP/2 Streamable HTTP with SSE response streams
- `ws` or `wss`: WebSocket

There is no probe, automatic upgrade, negotiation between transports,
redirect-based route change, or fallback.

The first proof is unauthenticated. It needs a portable unauthenticated
credential posture distinct from local-compute topology and no credential
lease. Authentication headers, query parameters, WebSocket subprotocol
credentials, interactive authentication, delegated login, token refresh, and
provider credential discovery remain excluded.

## Connection, Session, And Affinity

Swallowtail operation scope, remote ACP connection id, and ACP session id are
separate identities.

- one operation owns one remote transport connection
- the HTTP initialize response supplies the connection id
- subsequent HTTP requests carry that exact connection id
- an ACP session id appears only after session creation or load
- neither id is a credential, access profile, configured instance, model
  route, or durable consumer identity
- ids remain bounded, opaque, redacted, and scoped to their owning connection

Cookies sent during Streamable HTTP responses or the WebSocket upgrade are
retained only in one connection-scoped private cookie store for affinity. They
are sensitive transport state, not credentials or ACP session ids. They cannot
cross operation, endpoint, audience, host, or configured-instance boundaries
and are discarded after connection close.

WebSocket uses one full-duplex connection. The first proof must preserve
upgrade-response cookies even though it does not reconnect. It does not claim
custom authentication injection or cookie reuse across a reconnect.

## Lifecycle And Reliability

The normal lifecycle is:

1. authorize the exact endpoint
2. create one operation-scoped client and connection
3. initialize and verify ACP wire version and capabilities
4. open or load only the provider adapter's separately authorized session
5. exchange bounded ordered requests, responses, notifications, and callbacks
6. close explicitly
7. join readers, callbacks, connection work, and the operation-scoped executor
8. discard private ids and cookie state

The operation-scoped Tokio runtime runs only through the host's
`BlockingWork` service. A host task owns and joins that blocking job. Remote
ACP therefore requires `Task`, `BlockingWork`, `Time`, and `Network`; it does
not gain process or credential authority.

The first transport performs no automatic reconnect, retry, request replay,
stream resumption, liveness recovery, connection pooling, or multiplexing.
EOF, stream failure, protocol failure, cancellation, or deadline invalidates
the connection.

A later adapter may reconnect only under a new contract. Current ACP v1 cannot
replay in-flight requests or resume streams. A provider session that survives
a lost connection is not silently reattached; explicit `session/load` remains
a provider-adapter capability under Contract 017.

SDK drop behavior is not cleanup evidence. Swallowtail drives the SDK's
explicit close path during normal operation and joins all owned work under
Contracts 009 and 019. A cleanup failure remains visible beside the provider
outcome.

## Protocol And Callback Semantics

Contract 015 governs ACP initialization, correlation, capability omission,
session identity, updates, permission requests, filesystem callbacks,
extensions, cancellation, and provider-safe diagnostics.

Remote transport does not widen any advertised client or agent capability.
Filesystem, terminal, permission, consumer-tool, authentication, configuration,
load, resume, replay, and native-close behavior remain independently selected.

HTTP status success is not ACP success. `202 Accepted` means a request was
accepted for asynchronous response delivery; the correlated JSON-RPC response
still determines protocol success. Connection and session SSE streams preserve
provider order and remain bounded.

## Version Evidence

Keep these axes independent:

- ACP wire version
- transport RFD revision and status
- HTTP transport SDK version
- ACP core SDK version
- remote agent artifact and interface version
- configured-instance revision
- provider protocol or facade version

The compile-time SDK pin is not an installed interface-version support range.
The remote transport currently exposes no protocol-version response header
from which to infer one. Contract 029 range claims begin only when a
provider-specific adapter observes an exact version axis and freezes
behavioral milestones.

Known additive fields may be retained privately. Missing required headers,
invalid connection ids, wrong wire version, unknown semantic messages,
correlation drift, capability drift, or changed lifecycle behavior fail
closed. None trigger a fallback transport or behavior revision.

## Conformance

The thirteenth common profile is a remote ACP harness profile. It composes
shared assertions without pretending the existing process ACP or attached
network-harness profile already covers the shape.

Deterministic fixtures prove:

- no network work before exact preflight and endpoint authorization
- explicit HTTP/SSE versus WebSocket selection
- HTTP initialize, connection id, `202` request acceptance, connection and
  session SSE response delivery, and cookie affinity
- WebSocket upgrade cookies plus full-duplex request, response, notification,
  and callback order
- distinct operation, connection, and session identity
- bounded frames, streams, pending requests, callbacks, and private state
- wire-version and capability rejection
- no redirect, retry, reconnect, replay, resumption, pooling, multiplexing, or
  transport fallback
- cancellation, deadline, disconnect, protocol failure, explicit close, and
  joined cleanup
- endpoint, headers, cookies, ids, frames, payloads, and SDK errors stay out of
  stable diagnostics
- the same public seam under local and remote-authoritative execution-host
  identities

Default QA uses independent loopback fixtures and no live endpoint or
credential. A maintained-SDK test server may cross-check behavior but cannot be
the sole oracle for the maintained-SDK client.

## Acceptance

- remote ACP remains a reusable transport rather than a generic provider
- access, provider, model, agent, session, and transport identities stay
  separate
- HTTP/SSE and WebSocket differences remain visible
- unauthenticated experimental use is explicit and opt-in
- connection affinity state is scoped and redacted
- no implicit recovery or transport fallback exists
- explicit graceful close and joined cleanup are the normal path
- provider-specific authentication and support claims require later evidence
