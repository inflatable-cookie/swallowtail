# Research 427: DeepSeek Harness Web Browser-Session and Typed-Stream Boundary

Date: 2026-10-09

## Scope

Prepare the separately reviewed, provider-free design for adapting
`deepseek-harness.local-server` from its exact supported Web points through
`0.1.1-rc.2` toward `0.2.0-rc.2`. This study covers the DSH-owned browser
session credential, authenticated local process boundary, typed Session
controller, and Remote stream carrier. It does not change claims, API code,
route matrices, or release baselines.

[Research 411](./411-deepseek-harness-web-rc2-authentication-stop.md) remains
the immutable 51-artifact and 1,320-file route inventory. This record freezes
the missing selected route-module artifacts and source files separately from
CLI and runtime-bin axes. [Research 412](./412-deepseek-harness-sdk-jsonrpc-profile-mapping-study.md)
is independent and supplies no local-server evidence or authority.

## Exact selected artifacts

The [source identity ledger](./deepseek-harness-web-browser-session-stream-source-ledger.tsv)
records 104 exact package artifacts across these eight release points:
`0.1.2-rc.1`, `0.1.5-rc.1`, `0.1.5-rc.2`, `0.1.5-rc.3`,
`0.1.7-rc.1`, `0.1.7-rc.2`, `0.2.0-rc.1`, and `0.2.0-rc.2`. It covers
13 route-related packages: Web app, client connection, host webserver, API
gateway/remotes, session and workspace controllers, Typert protocol,
credential API and local store, plus the pinned Session, Agent, and LLM type
packages. Each row binds the official registry tarball URL, registry SHA-512
integrity, recomputed tarball SHA-256, selected-source manifest SHA-256, and
per-file SHA-256 values for 429 selected source files. The tarballs were
verified from public registry bytes and inspected without installation or
execution. The tarball is the source identity; no Git revision is inferred.

The exact target sources used for control-flow review include:

- `dsh-client-connection@0.2.0-rc.2`: `lib/index.js`
  `3950f524aed44e28240140a254cd928ea40eba8e2d04cc8fd04f92a687f5c91d` and
  `lib/client.js`
  `0fbb93fa2382bb66919767f69c909585c6255486f9536f9232231edaeac8c4e6`.
- `dsh-web-app@0.2.0-rc.2`: `lib/index.js`
  `50c7ce5b93e8a7ac0117e066699cd31b553a5a135d38b7051d6a8364ac691af4` and
  `lib/startup.js`
  `95a47053483fbe8ec86711369b8791bccb592c303fcc89300458a4bd07c6c252`.
- `dsh-api-gateway@0.2.0-rc.2`: `lib/client.js`
  `e09e26fcd4849cd55683b9cc702dca896efce93a5af3968431c1c8a62147b9cf`,
  stream protocol and server sources recorded in the ledger.
- `dsh-api-session-controller@0.2.0-rc.2`: `lib/index.js`
  `fb0f7b96130f595db20809eeb77a195b05f4a039f931d1e67692f1f74a4dd269`,
  `lib/typert.remote-client.js`
  `117d6192f45ce9447cd01002e61a01388c97d4799b35328264d13b699d11e880`,
  and `lib/types/types.d.ts`
  `1b4d81ed8f89ccd5d3c477a38fe2ab8c993f24af0e21539d0e0b7d22d7c2e7b8`.
- `dsh-session@0.2.0-rc.2`: `lib/types/known-event-types.js`
  `8f38e6fc9439bb4e47f2bd3aa21c86d5141ef9679f41181c9d9a48765c67d3be`.
- `dsh-credentials-local@0.2.0-rc.2`: `lib/index.js`
  `1688f17801d5809abace4ef6228b771625a0153c043d7d4dba21b398ec4056eb`.

Research 411 continues to own the previously accepted route inventory and
supported points. The target remains unqualified.

## Credential ownership and process effects

At the first stop, `dsh-client-connection@0.1.2-rc.1` adds `BrowserAuth`.
Its `create` path calls `credentials.modifyRecord` for the fixed logical key
`client-connection/browser-session`. If the record exists, the callback
validates and returns the current signing secret without replacing it. If it
is absent, the callback writes a `kind: grant` record with version `1` and a
new 32-byte random secret. `BrowserAuth` retains that secret in memory. It has
no direct delete or rotation call.

If the selected provider is `dsh-credentials-local`, its provider implementation
stores records in the configured credentials file under the selected DSH home.
A configured provider path may override the home-derived location. `modifyRecord`
serializes local writes, takes a cross-process file lock, rereads current data,
and atomically writes changes. The provider also exposes deletion. The source
does not identify which provider or DSH home the host's opaque Cordis
`EnvironmentRef` selects, who owns that profile, or whether the local provider
is active. No actual path, secret, or principal can be asserted from this
study.

The cookie is HMAC-SHA256 signed with the persisted secret. Its payload binds
the canonical Host authority and issued/expiry times; it does not bind an OS
user or named account. The default lifetime is 30 days. The cookie uses
`Path=/`, `HttpOnly`, `SameSite=Strict`, `Max-Age`, and `Expires`; it has no
`Domain` or `Secure` attribute on the HTTP loopback route. A valid launch token
is accepted once on `GET /`, sets the cookie, and redirects to a clean URL.
Invalid or absent cookies receive `401` for `/api`; there is no anonymous
fallback. The Host Connection attaches a process-wide `OperatorPeer` to each
authenticated request, not a per-user principal. Configured `trustedHosts` can
admit additional authorities, so the selected environment must prove the
resolved value and loopback binding.

The Web app logs an authenticated URL containing the process launch token when
it announces readiness. `--no-open` disables opening the operating-system
browser; it does not disable this URL output. The composition may also announce
a LAN URL when a LAN address is configured. The URL token is the one-time
input for cookie exchange and must never enter public diagnostics. The
persisted signing record survives the process. A cookie can remain valid for
up to 30 days and can be accepted by a later process using the same authority
and store. A run lease expiring does not revoke it.

This creates a concrete authority gap. The current prepared route accepts only
`CredentialMechanism::LocalUnauthenticated`, no `CredentialRef`, and
`CredentialState::NotRequired`. `AccessProfile` has no host-principal,
credential-store owner, record-operation, or persistence field. The current
operation-scoped `CredentialLease` binds a secret or delegated credential to
a scope, reference, and endpoint audience; it does not authorize DSH to create,
read, or revoke its persistent browser-signing record. The process API can
read child stdout, but its opaque environment reference does not prove which
credential provider or home the DSH child will use.

Token capture, cookie transport, and typed-frame parsing can be adapter-private
mechanics after the host authority is explicit. The credential authority
cannot be inferred privately from the current input types. A private-only
mapping is not established.

## Typed controller and stream lifecycle

The selected controller is a typed Remote API, not the old JSON-RPC method
map. Its `session` namespace includes `attachment`, `cancel`, `canOpenWorkspacePath`,
`control`, `create`, `follow`, `fork`, `initializeDefaultModel`, `list`,
`modelCatalog`, `openWorkspacePath`, `page`, `projections`, `prompt`, `rename`,
`search`, `selectModel`, `updateQueue`, and `workspacePathApplications`.
The current adapter's old contract remains an explicit 11-method allowlist:
`session.list`, `session.search`, `session.create`, `session.history`,
`session.models`, `session.prompt`, `session.cancel`, `session.fork`,
`workspace.list`, `workspace.archiveSession`, and `host.describe`. Do not
forward the whole typed controller or change the old allowlist.

For one structured run, the minimum candidate typed path is `session.create`,
`session.prompt`, `session.follow`, and `session.cancel`, plus a separately
reviewed workspace lookup only if required to bind the exact working resource.
`session.create` accepts `workspaceId` or `cwd`, never both. Its optional
`sessionId` is an explicit adoption identity, not proof that a lost response
can safely be retried. The adapter must supply only the prepared resource,
never the DSH default workspace.

`session.prompt` carries a client-minted durable `requestId`, `sessionId`,
`queue` or `steer` mode, and typed content. Its `accepted: true` result means
the prompt entered the Agent inbox; it does not mean the turn completed. The
controller source returns `accepted: true` for an already observed prompt with
the same request ID, which supports an exact reconciliation path. Keep one
stable request ID and never issue a fresh prompt after an ambiguous result.

`session.follow` opens with a complete snapshot: header, cursor, bounded
history records, projections, and an optional assistant-stream baseline. It
then yields ordered durable events and, when requested, assistant-stream
frames. The durable path carries sequence numbers and rejects a skipped
sequence. Assistant frames use `start`, `chunk`, and `end` with attempt ID,
revision, dense chunk index, and a terminal outcome; they are cursorless and
may be rebaselined after reconnect. The controller attaches event listeners,
uses an abort signal, and removes the listener, follower, and abort handler in
its `finally` path.

The browser Remote stream carrier uses `/api/remote.mux`. Client messages are
`open`, `item`, `end`, or `cancel` with a logical `streamId`; server messages
are `item`, `error`, or `end`. A physical socket is scoped to the authenticated
Peer. The Gateway has a configured per-stream uplink byte budget and returns
`gateway/uplink-overflow` when it is exceeded. Its server closes sockets and
waits for active iterators. The browser `RemoteStream` can reopen one logical
stream across connection generations; the consumer must accept a validated
opening snapshot/cursor. Its disposal waits for the consumer iterator, while
the mux client closes the carrier and fails active streams.

Keep the three cancellation layers distinct:

- `session.cancel` returns `accepted: true` when cancellation is admitted to
the live Agent. It is not completion.
- A Remote stream `cancel` frame terminates that logical stream.
- `ProcessHandle` stop/force-stop and `wait` own the local child lifecycle.

A successful run therefore needs a terminal durable turn outcome, not only a
prompt or cancel receipt. Close the stream and join its reader before releasing
the child. If cancellation is requested, send `session.cancel` once, drain the
terminal event when available, cancel/close the logical stream, join stream and
output readers, then stop and wait for the child. Use a bounded force-stop
fallback and report cleanup failure if a join fails.

The old route bounds are 256 KiB for HTTP bodies and WebSocket frames, 128
bytes for RPC IDs, 16,384 bytes for text, 64 history entries, 512 listed
sessions, 8,192 live events, and 256 KiB total structured output. The typed
API validates `maxMessages` as a positive safe integer and supports a
`turnWindow`; these fields do not bound frame bytes, live event count, or
output bytes. The Gateway's `streamInboxBytes` is configuration-dependent.
A new adapter must impose and test its own explicit frame, total-byte,
request, history, event, and output limits. Do not assume the old values bind
the new carrier without fake proof.

`SessionFollowFrame` has no separate usage message. The selected `dsh-session`
types put optional `TokenUsage` on durable `assistant/message` event data when
the model adapter reports it; there is no usage field on assistant chunk
frames. Parse usage only from that exact event schema and emit it once per
durable event sequence. The wire `SessionWireEvent` keeps `type` as a string
and `data` as JSON, so the adapter must validate known payloads and reject an
unknown required event instead of silently dropping it.

The selected `dsh-session` event vocabulary includes `approval/asked` and
`approval/decided`; the Session Remote namespace has no approval-response
method. The current Swallowtail route has no approval capability either. Do
not approve or fabricate a permission outcome. Fail closed and request
cancellation if an approval event appears until a separately approved
permission boundary exists.

## Proposed opt-in contract

Introduce a separate public, versioned prepared facade for authenticated Web
sessions. Candidate names such as `deepseek-harness.web-browser-session-v1`
and `DeepSeekHarnessWebBrowserSessionInput` are design labels only. Keep the
current `deepseek-harness.apiproxy-v1` input, defaults, allowlist, and supported
RC points unchanged. No caller opts in through a changed default or an
unauthenticated fallback.

The new facade needs a host-issued, operation-scoped authority object binding:

- execution host and process principal;
- exact executable/version, Cordis environment, DSH home/profile reference,
  and credential-provider identity;
- the fixed `client-connection/browser-session` record and the allowed
  create-if-absent, read, and revoke/delete operations;
- exact loopback Host authority and endpoint audience;
- exact working resource and session identity;
- grant expiry, cookie/redaction policy, and cleanup owner.

Do not expose the signing secret to Swallowtail. The host must map the child to
a dedicated store/profile containing only the authorized record, or prove an
equivalent scoped provider. A persistent shared user profile is not acceptable
for a run-scoped lease unless the host can revoke the record after process
exit and prove that an old cookie fails on the next activation. If neither
isolated storage nor tested revocation is available, preflight fails before
`ProcessService::start`.

Before child start, require proof of the exact selected DSH profile/store,
principal, loopback authority, `trustedHosts`, cookie lifetime, URL-printing
behavior, browser-open setting, and host log handling. The process must use
`--no-open`; capture the one-time token URL only through the host-owned output
channel, keep it in memory for one `GET /` exchange, retain the resulting cookie
only in memory, and redact both before diagnostics. If the resolved composition
cannot suppress or safely redact token-bearing output, fail before sending any
prompt. Send API and WebSocket requests only to the exact loopback authority,
with the authority-bound cookie and matching Origin. An invalid token, cookie,
Origin, Host, or endpoint is a terminal authentication error. Do not retry
anonymously, follow a redirect to another authority, or launch a browser.

The typed run adapter should expose only its reviewed method subset. Use a
separate `streamId` for the logical Remote stream and stable `SessionRequestId`
for prompt correlation. Reconnect only the read-only follow stream after a
validated opening snapshot; deduplicate durable events by sequence and
assistant frames by attempt/revision/index. Never replay `session.create` or
`session.prompt` after an ambiguous carrier failure. Treat model/provider
binding as a preflight prerequisite: if the selected new composition has no
reviewed read-only observation equivalent to the old `host.describe`, do not
start the prompt.

## Compatibility, rollback, and future proof

The additive facade must not reinterpret the old route. Replacing the old
facade identity or changing its access, lifecycle, or cleanup guarantees is a
breaking public behavior change under Contract 036 and requires the coordinated
pre-1.0 minor classification. A separate additive API may be classified
independently after review. This study makes no release or version decision.

Rollback is a disabled-by-default route selection. On preflight, authentication,
stream, or cleanup failure, do not fall back to the old anonymous route. Stop
the child, join the stream and output readers, and revoke the dedicated
credential record or task-owned profile under its explicit host grant. Keep
the existing supported facade available independently.

A future deterministic fake suite should cover:

- denied store owner, process principal, host, resource, endpoint, expiry, and
  record operations; no child start or credential write on denied preflight;
- missing-record create, existing-record read without replacement, explicit
  revoke/rotation, and rejection of a prior cookie after restart;
- one-time URL exchange, no browser call, token redaction, same-origin/Host
  checks, invalid-cookie `401`, and no anonymous retry;
- exact typed request serialization, stable prompt request ID, ambiguous
  response reconciliation, snapshot/cursor validation, durable sequence gaps,
  reconnect deduplication, assistant rebaseline, bounded output, and usage once;
- `approval/asked` without approval, cancel receipt before terminal event,
  cancel races, stream close, reader/task joins, process stop/wait, and cleanup
  failure reporting.

Implementation selectors for the later fake-backed change are
`effigy validate:current-deepseek-harness-local-server`,
`effigy check:current-deepseek-harness-local-server`, and
`effigy package:verify-affected swallowtail-adapter-deepseek-harness`. Add the
new fake test targets to the focused validation selector. Run
`effigy qa:docs` for contract/spec edits and `effigy qa:routes` if route or
matrix truth changes. The qualification brief must prove the exact shipped
path against the fakes; fake success alone does not qualify `0.2.0-rc.2`.

## Gate owners and next briefs

| Owner | Required evidence or decision before implementation |
| --- | --- |
| Execution-host and Cordis-environment owner | Secret-free binding from `EnvironmentRef` to DSH profile/home, credential provider, process principal, effective web config, endpoint/`trustedHosts`, cookie lifetime, URL output, browser setting, and host diagnostic path. Do not provide a machine path or secret. |
| DSH credential-store owner | Confirm the isolated store or scoped record operations and who can delete/revoke the exact record after the child exits; demonstrate old-cookie rejection after restart. |
| Tom, operator and contract owner | Rule on persistent-record creation/revocation, isolated DSH profile semantics, access lifetime, and the new public facade/route identity after independent review. Any public lifecycle/access change receives a Contract 036 classification. |
| Swallowtail implementation and qualification owners | Prepare separate Queue briefs: one fake-backed opt-in implementation with rollback and the selectors above; one exact `0.2.0-rc.2` qualification after implementation review. Do not claim qualification from this study or from fakes. |

These are named finite gates. The authentication stop is not closed, and no
implementation or qualification is authorized by this proposal.

## Verification boundary

Only public npm metadata and tarballs were read. No DSH package was installed
or executed. No credential store, real DSH home, process, server, browser,
provider, or consumer was accessed. No route claim, facade, capability, API,
serialization contract, release baseline, or qualification changed. The exact
supported Web points remain through `0.1.1-rc.2`; `0.2.0-rc.2` remains
unqualified.

## Sources

- [DeepSeek npm registry metadata](https://registry.npmjs.org/@deepseek-ai/dsh)
  and the package metadata/tarball URLs frozen in the source identity ledger.
- [DeepSeek Harness source repository](https://github.com/deepseek-ai/deepseek-harness).
  The package tarballs, not an inferred repository revision, identify the
  inspected code.
- [Research 411: Web RC2 authentication stop](./411-deepseek-harness-web-rc2-authentication-stop.md).
- [Contract 023: Harness operation isolation and native boundary](../knowledge/contracts/023-harness-operation-isolation-and-native-boundary.md).
- [Contract 036: Crate release and compatibility boundary](../knowledge/contracts/036-crate-release-and-compatibility-boundary.md).
