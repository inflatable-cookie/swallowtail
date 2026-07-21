# 015 ACP v1 Negotiation And Client Callbacks

Status: active
Owner: Tom
Updated: 2026-07-20

## Purpose

Define the first reusable ACP wire and callback boundary without treating ACP
compatibility as a universal harness capability or moving filesystem,
permission, terminal, authentication, or configuration authority into an
adapter.

## Version And Artifact Identity

ACP wire compatibility is the integer `protocolVersion` negotiated during
`initialize`. SDK, crate, package, and JSON Schema release versions are
separate artifact identities.

The first proof binds:

- ACP wire version `1`
- stable schema release `schema-v1.19.0`
- Gemini CLI `0.51.0`
- Gemini CLI's `@agentclientprotocol/sdk` `0.16.1`

Only the stable schema belongs to the portable baseline. Unstable schema
fields, Gemini model selectors, and provider `_meta` values remain
adapter-private evidence. A different wire version fails initialization,
closes the connection, and reports protocol incompatibility.

The second-agent Kimi proof independently binds stable schema release
`schema-v1.19.1`, Kimi's exact TypeScript SDK lock `0.23.0`, and Kimi Code
`0.28.1`. Those artifact changes do not alter the negotiated wire version or
the Gemini proof's historical pins. Contract 017 governs the later stable
load, resume, replay, and write subset.

## Transport And Framing

The first transport is UTF-8 newline-delimited JSON-RPC 2.0 over stdio:

- the client launches one host-approved agent process
- one complete message occupies one line and contains no embedded newline
- stdout contains ACP messages only
- stderr is provider logging, not protocol or a safe public diagnostic
- frame size, frame count, ingress, and pending-request count are bounded
- malformed JSON, invalid JSON-RPC version, duplicate correlation, overflow,
  and a partial final frame fail the owning scope

The driver owns framing and correlation. The host owns executable, arguments,
environment, working resource, stdio limits, stop, exit, and join. EOF, process
exit, or transport failure cannot leave a detached reader or callback wait.

## Initialization And Capabilities

`initialize` precedes authentication or session work. The client offers its
latest supported wire version and exact callback capabilities. The agent
returns one version, agent capabilities, authentication methods, and optional
implementation evidence.

- an omitted or null capability is unsupported
- optional methods are callable only after their advertised capability
- advertised authentication methods do not select an access route
- advertised content, MCP, mode, model, or extension support does not enlarge
  Swallowtail's selected operation requirements
- unknown stable core requests receive method-not-found and fail the scope
- unknown extension requests receive method-not-found
- unknown extension notifications are ignored
- `_meta` stays bounded and adapter-private unless a namespaced extension is
  explicitly preflight-bound

The ACP v1 baseline requires `session/new`, `session/prompt`,
`session/cancel`, and `session/update`. Load, resume, list, delete, close,
configuration, logout, richer prompt content, filesystem, terminal, and custom
methods remain independently optional.

## Session And Turn Lifecycle

The first Gemini proof claims one new read-only interactive session and one
active text prompt at a time:

- the process starts with `--acp --approval-mode plan`
- `session/new` carries one host-resolved working directory and no MCP servers
- the returned current mode must remain `plan`; another mode fails and closes
- returned models and modes are observations only
- the driver never calls `session/set_mode`, `session/set_config_option`, or
  unstable model-selection methods
- ordered `session/update` notifications belong to the exact active session
- the `session/prompt` response supplies the one terminal stop reason
- native active-turn cancellation uses `session/cancel`
- cancellation remains pending until the prompt returns `cancelled` or the
  process is stopped and joined

Gemini `0.51.0` advertises load support, but its implementation starts history
replay without awaiting completion before returning the load response. The
first driver therefore does not claim `Resume` or call `session/load`.

Gemini `0.51.0` does not advertise stable `session/close`. Session close ends
stdin, stops the owned process when needed, observes exit, joins all protocol
work, and releases the resource lease. It does not claim native session close.

## Authentication And Provider State

ACP authentication negotiation is not credential authority. The first Gemini
driver does not call `authenticate` because the current implementation may
change user settings, clear cached credentials, or launch provider-owned login
flows.

The configured instance instead binds one exact access profile and one
host-approved isolated provider-state and process-environment reference.
Gemini Developer API-key access is the first proof profile. Consumer
membership, interactive Google login, Vertex AI, gateway access, and enterprise
entitlements remain distinct later profiles.

No ambient home, provider configuration, extension, MCP server, credential
store, proxy, model, or policy fallback is allowed. Missing or rejected access
fails the session; advertised authentication methods do not trigger retry or
route selection.

Gemini CLI Plan Mode alone is not sufficient access policy because it permits
some search and can inherit higher-tier policies. The first configured
instance requires a host-approved isolated Gemini state root whose policy
denies mutation, external search, MCP servers, extensions, and mode widening.
The driver verifies the returned current mode but does not interpret provider
policy files.

## Permission Requests

`session/request_permission` is an ACP client request, not a declared consumer
tool callback under Contract 012.

The read-only profile never selects an allow option. When a permission request
arrives, the driver:

1. bounds and correlates it to the active session, turn, and tool call
2. exposes only a safe provider-request observation
3. sends the ACP `cancelled` outcome for a pending permission wait
4. sends `session/cancel`
5. closes and joins if the prompt does not terminate

No permission option persists provider configuration. Missing reject or cancel
semantics cannot be replaced with fabricated approval.

## Filesystem And Terminal Callbacks

Client filesystem and terminal methods are execution-host callbacks. They are
not consumer product tools and do not execute through the Contract 012 callback
exchange.

The first proof advertises only `fs.readTextFile`. It requires a separate
`WorkingResourceIo` host service bound to the same scope, execution host,
read-only resource lease, and provider session. A read request:

- accepts only the provider locator needed by the adapter in a redacted wrapper
- resolves canonically under the one authorized working root
- rejects absolute-root mismatch, traversal, symlink escape, wrong scope,
  wrong session, and non-file targets before content access
- applies explicit line and byte limits
- returns bounded text only through the private protocol response
- never places the locator or content in public events or default diagnostics

The first client advertises filesystem write as false and omits terminal
capability. Any write or terminal request is unsupported and stops the scope.
Filesystem write support is governed by Contract 017 and still needs an exact
write-capable host service. Terminal support needs a later contract. Process
ownership implies neither callback authority nor filesystem containment.

## Extensions And Failure

Raw JSON-RPC envelopes, prompts, filesystem content, permission bodies,
authentication metadata, model ids, provider logs, and Gemini `_meta` do not
enter stable diagnostics.

Provider errors, protocol failures, consumer cancellation, host deadline,
process failure, and cleanup failure remain distinct. Unknown stop reasons or
session-update semantics fail closed; known additive fields on a known message
may be retained privately without changing the normalized result.

## Acceptance

- wire, schema artifact, SDK, and agent versions remain distinct
- initialization and capability omission fail closed
- baseline and optional ACP methods are not flattened
- the first Gemini route cannot mutate authentication or ambient configuration
- permission, filesystem, terminal, and consumer tool callbacks remain separate
- read callbacks cannot escape one read-only host resource
- load/resume and native close are not claimed from incomplete evidence
- cancellation produces one terminal outcome and joined cleanup
- provider and extension payloads stay out of public diagnostics
