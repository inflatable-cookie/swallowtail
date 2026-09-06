# 014 Shared Harness Capability And Producer Boundary

Status: draft; operator-confirmed direction, independent review required before promotion
Owner: Tom
Updated: 2026-09-06
Evidence: Research 288; Contracts 012, 017, 019, 028, 029, 037, 041, 047, 051, 057, 058, 060-062

## Purpose

Define one reusable producer boundary for tool and MCP registration, exact
schema discovery, host-mediated execution, provider attachment, contextual
instructions, repository skills and references, and qualified scheduling
across Claude, Codex, and Grok. Preserve exact route behavior and consumer
authority.

This spec is provisional. It creates no capability, public API, provider
claim, implementation card, release, or consumer obligation until its durable
rules are reviewed and promoted.

## Ownership

| Owner | Owns | Does not own |
| --- | --- | --- |
| Swallowtail | portable registration records; server leases; transport and version negotiation; prepared-plan binding; callback/result correlation; safe lifecycle and diagnostics; route adapters; conformance | tool business logic, product task policy, UX, durable product state, provider fallback |
| Longhorn | server implementation, namespaced tool semantics, input/output schemas, business validation, execution, domain errors | provider session selection, Desktop context policy, Swallowtail transport lifecycle |
| Desktop | repository and skill discovery choice, required-reference selection, app-context assembly, task/session persistence, queue policy and UX, final Allow/Deny policy, consumer receipts | provider wire, reusable transport, server process supervision inside Swallowtail leases |

Other consumers may reuse the Swallowtail boundary. They do not inherit
Desktop or Longhorn semantics.

## Proposed Architecture

```text
Desktop task/context/skill selection
  -> immutable consumer request and exact registered capability selection
  -> adapter prepared plan
  -> Swallowtail registered-server lease on the selected execution host
  -> route-specific native tool callback or private MCP attachment
  -> exact Longhorn server method
  -> typed result or safe failure through the same operation binding
  -> provider turn terminal
  -> callback/server/transport/process/resource teardown and consumer receipt
```

### Registry And Schema Discovery

Add one provider-neutral registered-capability family. It composes with the
existing configured-instance and prepared-operation surfaces; it is not an
adapter registry or global service locator.

The immutable registration snapshot must contain:

- stable server identity and revision;
- stable namespaced tool identity: producer namespace plus local tool name;
- schema namespace, media type, dialect, revision, bounded bytes, and digest;
- native client-tool, MCP, app-tool, or provider-owned execution kind;
- supported transports and exact protocol/version ranges;
- required host services and execution-host identity;
- credential-reference requirements without credential material;
- process/environment recipe references without raw executable paths or env;
- positive call, byte, concurrency, deadline, and result bounds;
- mutability, replay, retry, reconnect, and teardown posture; and
- source identity and freshness suitable for Contract 061 projection.

Discovery returns descriptions only. Selection binds one exact registration
revision and schema digest into a prepared plan. Registration cannot choose a
provider, model, route, tool, credential, or server for the consumer.

### Host Service And Server Lease

Extend `HostServices` with one service kind for registered tool/server
execution. Reuse existing `ProcessService`, `CredentialService`,
`WorkingResourceService`, `SchemaService`, `TimeService`, and scoped-task
ownership. Do not encode credentials, executable paths, complete environment,
or server endpoints in public requests.

Opening a server lease binds:

- execution host;
- configured instance and prepared plan;
- consumer task and runtime session;
- runtime turn and attempt when one exists;
- exact registration revision and selected tools;
- transport generation and negotiated protocol version; and
- deadline, cancellation, and maximum concurrency.

The lease reaches ready before provider dispatch. Close freezes admission,
abandons or completes issued calls, joins callbacks and transport readers,
stops owned server processes, releases resources, then releases credential
leases. Drop is defensive cleanup, never success evidence.

### Transport And Reconnect

The first transport set is bounded stdio JSON-RPC and private loopback
HTTP/SSE MCP. Unix sockets, WebSocket, remote public HTTP, and attached ambient
servers remain out until separately qualified.

Initialization negotiates one exact protocol version and server identity.
Unsupported or substituted values fail before provider work. Reconnect may
replace transport only while retaining the same registration, host, task,
session, turn/attempt, and server generation. It never replays a mutating call.
The consumer must explicitly retry a call whose execution outcome is unknown.

Every call has a caller identity, server/tool identity, call id, task, session,
turn, attempt, deadline, cancellation state, and transport generation. Late,
foreign, duplicate, stale-generation, post-terminal, and post-close results
fail without reaching the provider.

### Results, Errors, Concurrency, And Teardown

One result is accepted exactly once. Result media, bytes, schema, and status
are bounded before provider translation. Safe failures distinguish:

- unsupported registration, tool, schema, transport, or version;
- host-service, credential-reference, process, environment, or readiness
  failure;
- consumer Deny, provider rejection, cancellation, timeout, and abandonment;
- server execution failure, invalid result, transport loss, unknown execution
  outcome, and teardown failure; and
- stale, duplicate, foreign, or post-terminal correlation.

The prepared plan fixes maximum concurrent calls. The first tranche permits
one active provider turn and one outstanding call per server lease. A later
bound may widen concurrency after deterministic race proof. No unbounded
queue, detached callback, automatic mutating retry, or silent result drop is
allowed.

### Tool Kinds And Permission Paths

Tool kind survives registration, events, callbacks, results, and receipts:

| Kind | Executor | Swallowtail role |
| --- | --- | --- |
| native client tool | Longhorn or another consumer-selected executor | declare through provider wire; correlate call/result |
| MCP tool | Longhorn MCP server | own private server lease and route attachment or mediate the call |
| app tool | Desktop/consumer service | transport through exact callback; no MCP identity inferred |
| provider-owned tool | provider/harness | observe only unless an exact permission request is exposed |

`Allow` means one exact call may proceed. `Deny` returns one exact route-supported
denial. Neither creates a persistent policy. When a provider cannot represent
the consumer's Deny, the adapter cancels or fails the turn under its qualified
route contract; it never simulates approval or reports a denial the provider
did not receive.

### Provider-Direct MCP Versus Host-Mediated Dispatch

| Route | First placement | Reason and gate |
| --- | --- | --- |
| `claude-agent.sdk` | provider-direct private MCP attachment behind the common server lease | upstream client MCP is the selected native seam; registration, process/env, credential references, per-call admission, status, and teardown remain host-owned; no support claim until a real disposable server returns a real result |
| `claude-agent.acp` | host-mediated native callbacks only; MCP withheld | current ACP sends an empty server list and has no MCP input facade; add direct MCP only after exact bridge schema, version, permission, and lifecycle evidence |
| `codex.app-server` | host-mediated dynamic native tools | this is already the qualified execution seam; observed MCP activity is not registration or result authority; provider-direct MCP needs a separate app-server surface and corpus |
| `grok-build.acp` | host-mediated permission and provider-tool observation; MCP withheld | current route sends an empty server list and exposes no consumer tools; direct MCP waits for exact ACP and Grok version evidence |

The common API must not promise that every route accepts MCP. A route publishes
the exact attachment or mediation kind through Contract 061.

### Context, Instructions, Skills, And References

Keep four inputs distinct:

1. session developer instructions, fixed at open or validated resume;
2. per-turn user text in `TurnRequest`;
3. an immutable selected-skill bundle derived from Contract 062 inventory; and
4. bounded required references resolved by opaque host references.

Desktop chooses the skill and references. Longhorn may define a tool whose
business semantics consume them. Swallowtail validates identity, bounds,
digest, lifecycle, and transport; it does not discover repositories, decide
relevance, generate product prompts, or claim the provider followed the skill.

A skill bundle contains stable skill identity/revision, content digest,
declared required-reference descriptors, provenance, and bounded resolved
content or opaque references. Missing, changed, oversized, inaccessible, or
foreign references fail before provider work. Raw client paths and unrelated
repository content never cross the public boundary.

### Scheduling And Continuation

Ordinary Desktop queueing remains consumer policy: submit the next bounded
`TurnRequest` only after the current turn reaches terminal and closes. Stored
pending text is not provider queue state.

Mid-turn steering, provider queueing, and follow-up scheduling require:

- an exact route capability and version segment;
- prepared-plan bounds;
- callable adapter implementation;
- correlated accepted/rejected acknowledgement;
- pending and terminal state observation;
- cancellation and overflow behavior; and
- deterministic plus real provider evidence.

Until all pass, `schedule_harness_message` stays unsupported or rejected.
Load, resume, reconnect, replay, and steering remain separate. Reconnect never
replays a mutating tool call or message.

## Capability Matrix And Readiness

Legend: `R` released at the audited `v0.4.3` source tag; `P` proposed here;
`W` withheld pending route evidence. An upstream feature without an adapter
plan, facade, acknowledgement, and proof is `W`.

| Capability | Claude SDK | Claude ACP | Codex app-server | Grok ACP | Common producer gate |
| --- | --- | --- | --- | --- | --- |
| bounded per-turn text | R | R | R | R | unchanged `TurnRequest` |
| session instructions | W | R subset | R | W | exact setup/resume lifecycle |
| durable session | R | R | R | R provider-owned | exact binding and cleanup |
| load/resume | route-qualified R | R | R | W | no raw id; no mutating replay |
| native consumer tools | route-local R | W | R | W | registration, schema, exact result |
| one-shot Allow/Deny | route-local R | R subset | tool result/question; approval W | R subset | exact provider response or typed cancel/fail |
| persistent permission | W | W | W | W | separate explicit contract |
| provider-direct MCP | P | W | W | W | private lease plus real server/result |
| host-mediated tool server | P | P | P | P after consumer-tool evidence | common host service and callback binding |
| skill/reference bundle | P | P | P | P | Contract 062 selection plus opaque refs |
| consumer app context | P | P | P | P | Desktop-owned composition, bounded transport |
| between-turn queue | Desktop-owned | Desktop-owned | Desktop-owned | Desktop-owned | next turn after terminal close |
| mid-turn steering | W | W | W | W | exact route contract and real acknowledgement |
| reconnect | P transport-only | P transport-only | P transport-only | P transport-only | same binding/generation; no mutating replay |

## Contract Promotion Set

Independent review must settle and promote these durable surfaces before any
runtime card becomes ready:

1. **Registered tool and server capability contract** — immutable namespaced
   registration, schema discovery, kinds, selection, bounds, projection, and
   no execution authority from discovery.
2. **Registered server host and transport contract** — host-service API,
   process/env/credential references, leases, stdio and private HTTP/SSE,
   negotiation, reconnect, correlation, cancellation, concurrency, and joined
   teardown.
3. **Tool execution and permission amendment** to Contract 041 — native/MCP/
   app/provider kinds, exact Allow/Deny paths, unknown execution outcome, no
   mutating replay, and route-specific refusal behavior.
4. **Skill/reference transport amendment** to Contracts 058 and 062, or one
   dedicated contract if transport would overload inventory — selected bundle,
   required references, opaque resolution, digest, failure, and no compliance
   claim.
5. **Scheduling amendment** only for routes that pass Contract 028's full
   acknowledgement and state gate. No Claude/Codex/Grok row is promoted now.
6. Contract 061 projection additions for registration kind, transport,
   permission strength, skill/reference support, and scheduling lifecycle.

## Readiness Gates

No implementation batch is ready until gates 1-5 pass. Route batches also
require their route gate.

1. **Authority:** architecture and repository authority map name Swallowtail,
   Longhorn, and Desktop ownership without importing product types.
2. **API:** exact public records, service traits, lease handles, requests,
   outcomes, safe diagnostics, and compatibility strategy are reviewed.
3. **Lifecycle:** state machine covers registration, prepare, ready, call,
   result, cancellation, reconnect, terminal, close, and stale callbacks.
4. **Security:** no raw credential, executable path, environment, endpoint,
   shell, outside-path, prompt, skill body, reference body, arguments, or result
   enters diagnostics or unapproved public records.
5. **Conformance:** provider-neutral fixtures falsify foreign identity,
   duplicate/stale result, deadline races, disconnect, unknown execution,
   concurrency overflow, cancellation, and teardown.
6. **Claude:** exact upstream SDK and ACP surfaces, version points, Allow/Deny,
   MCP, fresh-session options, and continuation constraints are frozen.
7. **Codex:** dynamic-tool host mediation remains the baseline; any direct MCP
   surface is independently frozen and does not borrow activity evidence.
8. **Grok:** one-shot permission response remains exact; consumer tool/MCP
   support waits for a frozen provider surface and cannot be inferred from
   provider-owned tools.
9. **Consumer:** Desktop and Longhorn compile against one reviewed candidate;
   product context and tool semantics stay outside Swallowtail.
10. **Release:** public API baseline, route matrices, guide, source consumer,
    exact-SHA CI, and a separately authorized tag follow Contract 036.

## Real Acceptance

Each Claude, Codex, and Grok acceptance lane must use a disposable workspace
and real provider route. Text-only fixtures or simulated approval cannot close
these criteria.

For each route:

- edit one in-workspace file, observe the result, and reconcile final content;
- refuse one outside-workspace mutation without broad shell authority;
- prove the supported Allow path, supported Deny path, and cancellation while
  a call or permission is pending;
- run one repository-skill-guided task with every declared required reference
  resolved and bound;
- attach or mediate one disposable MCP server, execute one real tool, and
  return its real result, or record the route as unsupported after its evidence
  gate rather than claiming parity;
- bind one Desktop app-context record without sending unrelated client
  content;
- disconnect the transport or server and observe typed failure, no mutating
  replay, stale-callback rejection, and joined teardown; and
- retain exact route, version, model, host, session, turn, attempt, tool,
  registration, and result identities in sanitized evidence.

No test may silently enable ambient shell, outside-path write, persistent
permission, provider fallback, or client-content authority.

## Public API And Compatibility

The public API is additive and provider-neutral. Existing `SessionOptions`,
`TurnRequest`, `CallbackExchange`, prepared facades, and low-level drivers stay
valid. New behavior is opt-in and absent by default. Adapter-specific methods
may expose exact direct-MCP or permission details, but the common registration
and host lifecycle types remain in `swallowtail-runtime`; capability and
preflight vocabulary remains in `swallowtail-core`; reusable local process and
transport realization belongs in `swallowtail-host-local`.

Any default change, new ambient authority, changed denial behavior, automatic
retry, or resume semantic is breaking before 1.0 and requires an explicit
operator decision. Version qualification remains one exact interface family
at a time under Contract 029.

## Cross-Repo Decisions Still Open

Only these require Desktop Chatterbox rather than Swallowtail repository
evidence:

1. the stable Desktop task/session/attempt identifiers that Longhorn must echo
   at its API boundary, and whether Desktop already has a single canonical
   attempt identity;
2. the exact Longhorn server distribution and startup authority: Desktop-owned
   process recipe, separately installed service, or an already-running
   endpoint; and
3. the Desktop app-context disclosure policy and maximum bounded payload the
   operator accepts for provider-visible context.

Everything else above is a producer contract, route evidence, or implementation
decision owned by Swallowtail planning.

## Promotion Targets

- `docs/architecture/system-architecture.md`
- `docs/architecture/repository-authority-map.md`
- new contracts and amendments named above
- Contract 061 projection vocabulary and adapter rows
- `docs/roadmaps/g05/035-shared-harness-capability-and-producer-boundary.md`
- route guides, matrices, release notes, and consumer handoffs only after
  implementation and evidence

## Spec Exit

Archive this spec only after independent review accepts the architecture,
every implementation-governing rule is promoted, the three cross-repo answers
are recorded, and the roadmap is recompiled with a ready first implementation
batch. Until then g05.035 remains planned and blocked from runtime work.
