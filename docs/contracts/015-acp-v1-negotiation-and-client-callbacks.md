# 015 ACP v1 Negotiation And Client Callbacks

Status: active
Owner: Tom
Updated: 2026-08-01

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

The lifecycle refresh independently binds stable schema release
`schema-v1.20.0`, source commit
`5e89c71497fe07dd4ae633c181a17224f4a8956d`, and stable schema SHA-256
`92c1dfcda10dd47e99127500a3763da2b471f9ac61e12b9bf0430c32cf953796`.
The selected close/delete shape is unchanged from stable schema `1.18.0`
through `1.20.0`. Historical adapter pins remain historical evidence. The
wire version remains `1`.

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

## Stable Session List

Stable ACP v1 exposes optional `session/list` through
`agentCapabilities.sessionCapabilities.list`.

- `{}` advertises the method; omission or null forbids dispatch
- optional `cwd` scopes discovery to one provider-understood working directory
- optional cursor pagination remains opaque and request-scoped
- results contain session id, cwd, optional title, optional updated time, and
  optional bounded agent metadata
- listing does not imply load, resume, close, delete, or complete history
- stable protocol support does not qualify an adapter that omits the capability

Contract 046 governs portable catalogue projection and explicit import. ACP
session ids, cwd strings, titles, `_meta`, and list results grant no attachment
or management authority by themselves.

## Stable Close And Delete

`session/close` and `session/delete` use separate fields under
`agentCapabilities.sessionCapabilities`.

- `{}` advertises the exact method
- omission or null means unsupported
- a client must not dispatch an unsupported method
- close support does not imply delete support
- delete support does not imply close support

Close cancels active work as if `session/cancel` had been called, frees the
agent's active resources, and returns an empty result on success. It preserves
durable history. The agent may reject a missing or inactive target.

Delete removes the session from future `session/list` results and returns an
empty result on success. ACP recommends silent success for an absent target,
but does not require it. Load after delete and deletion of an active session
remain implementation-defined. Portable deletion truth is `HistoryRemoved`;
soft deletion, data deletion, hard deletion, descendants, idempotency, and
missing-target behavior need exact adapter evidence under Contract 038.

The stdio and explicit remote transports carry the same bounded
`swallowtail_protocol_acp::Message` records. Physical framing, affinity,
recovery, and fallback remain transport concerns. No lifecycle method can
trigger implicit transport substitution.

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

### Delegated credential activation

An adapter may call ACP `authenticate` only to activate one already authorized
harness-owned credential when all of these are preflight-bound:

- exact driver behavior revision and executable version
- configured access profile, credential mechanism, entitlement, and endpoint
  audience
- one adapter-private authentication method
- whether provider-owned refresh of the same credential mechanism is allowed

Initialization must advertise the exact method for the current state. The
adapter sends it once, after initialization and before session allocation.
Missing, renamed, ambiguous, interactive, rejected, or mechanism-changing
behavior fails attachment and joins owned work.

Activation grants no login, logout, account switch, credential extraction,
browser or device flow, terminal action, external helper, API key, endpoint,
billing, or authentication-method fallback. Provider-private response metadata
is ignored. It cannot change public access evidence or enter stable
diagnostics.

The first qualified mapping is Grok Build `0.2.114` with pre-existing
subscription OAuth, adapter-private `cached_token`, and
`_meta.headless = true`. Provider-owned refresh of that same delegated OAuth
mechanism is allowed. `grok.com`, `xai.api_key`, and every other advertised or
configured mechanism remain excluded.

Gemini CLI Plan Mode alone is not sufficient access policy because it permits
some search and can inherit higher-tier policies. The first configured
instance requires a host-approved isolated Gemini state root whose policy
denies mutation, external search, MCP servers, extensions, and mode widening.
The driver verifies the returned current mode but does not interpret provider
policy files.

## Permission Requests

`session/request_permission` is an ACP client request, not a consumer tool
callback. It may cross Contract 012 only as the exact
`acp/session/request-permission` provider extension.

The default read-only and structured profiles never select an allow option.
When a permission request arrives, the driver:

1. bounds and correlates it to the active session, turn, and tool call
2. exposes only a safe provider-request observation
3. sends the ACP `cancelled` outcome for a pending permission wait
4. sends `session/cancel`
5. closes and joins if the prompt does not terminate

No permission option persists provider configuration. Missing reject or cancel
semantics cannot be replaced with fabricated approval.

Claude Agent structured-run or interactive-session preparation may instead opt
into one consumer-mediated profile. The immutable operation plan then declares
exactly `acp/session/request-permission`; absence of that namespace retains the
default reject-and-stop behavior. An interactive selection applies to the
whole prepared session, including load and resume, rather than changing per
turn. The opt-in driver:

1. validates session, turn, tool-call, provider-request, option, count, and
   payload bounds
2. exposes only offered `allow_once` and `reject_once` options through one
   correlated provider-extension callback
3. accepts exactly one consumer response naming an offered option
4. translates consumer failure to the offered one-shot rejection
5. abandons pending responses and applies native cancellation on turn
   cancellation, timeout, failure, or close

Persistent approval options are neither exposed nor selectable. Swallowtail
does not choose an allow response. The response port confirms ACP transport
acceptance, not tool execution or turn completion.

## Form Elicitation

ACP form elicitation is an unstable capability inside wire protocol `1`.
Support requires exact schema-artifact and agent evidence. The client
advertises `clientCapabilities.elicitation.form = {}` and handles
`elicitation/create`. This grants no URL, browser, credential, MCP,
provider-tool, model-switch, or persistent configuration authority.

Claude Agent ACP `0.53.0..=0.64.0`, excluding `0.58.0`, qualifies one common
typed subset. Stable newer wrappers inherit it only as unverified behavior
under Contract 029. The driver accepts only choice forms that map losslessly
to Contract 012:

- one to four ordered indexed questions
- single or multiple choice with two to four stable options
- qualified legacy or current option-description encoding
- one paired optional free-text `Other` field
- skipped answers

The client callback exchange exists whenever form capability is advertised.
The adapter validates session, active turn, JSON-RPC request, field identity,
schema, count, size, and response exactly once. `HarnessUserInputResponse`
becomes `accept` content using the original field ids. Consumer failure becomes
`decline`. Cancellation and terminal completion abandon the pending exchange
before joined cleanup.

Numeric, boolean, constrained-text, secret, URL, preview-bearing, unknown, or
otherwise richer forms do not enter the common callback type. They are
declined, not flattened. Advertising form support does not imply that an MCP
form or Claude refusal-fallback dialog is accepted.

ACP `_meta` remains private and exact. The Claude bridge does not preserve an
arbitrary request or question context. Swallowtail does not parse one from
prose or invent one. Recognizing the qualified option-description and
custom-answer markers grants no general metadata interpretation.

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
