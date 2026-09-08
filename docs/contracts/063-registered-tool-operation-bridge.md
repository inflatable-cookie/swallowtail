# 063 Registered Tool Operation Bridge

Status: active; implementation pending cards 114-117
Owner: Tom
Created: 2026-09-07

This is implementation authority, not a claim of realized runtime support.
Promoted from independently reviewed Spec014 and the bilateral ownership
decision at Desktop `30a338f2`, Longhorn `edc21078`, Swallowtail `6fa6266b`.

## Ownership

| Owner | Owns | Does not own | Governing evidence |
| --- | --- | --- | --- |
| Swallowtail | sole namespaced registration snapshot; operation bridge listener/transport, lease, correlation, generation, lifecycle, prepared-plan binding, route adapters, safe diagnostics, and conformance | domain tool names or schemas, business/effect policy, durable product identity, admission issuance, packaged host startup | [Contract 060](../contracts/060-operation-scoped-watcher-http-bridge.md), amended by this promotion |
| Longhorn | transport-neutral typed host dispatch/validation library and generic safe result/error envelopes | registry authority, listener, lease, correlation kernel, admission identity, domain names/schemas/policy, standalone daemon in slice 1 | [PR 22](https://github.com/inflatable-cookie/longhorn/pull/22), whose aligned [`Contract 023`](https://github.com/inflatable-cookie/longhorn/blob/6ce4aa1beafad6748af238d19fb47ffaa1ad342f/docs/contracts/023-production-contextual-agent-tool-boundary.md) and [`Spec 002`](https://github.com/inflatable-cookie/longhorn/blob/6ce4aa1beafad6748af238d19fb47ffaa1ad342f/docs/specs/002-production-contextual-agent-tool-boundary.md) independently passed at `6ce4aa1b` |
| Desktop | domain tool names and input/output schemas; effects/business/tool policy; bounded app-context disclosure; durable task/session/attempt admission issuance; final Allow/Deny policy; packaging, distribution, startup, queue/UX, and receipts | provider wire, registration snapshot ownership, bridge transport/listener/lease/correlation lifetime | [`Spec 010` at `30a338f2`](https://github.com/acowtancy/bovine-accelerator-desktop/blob/30a338f2/docs/specs/010-contextual-chat-and-task-queue.md) |

Other consumers may reuse the Swallowtail boundary. They do not inherit
Desktop or Longhorn semantics.

### Architecture Boundary

`docs/architecture/system-architecture.md` must record one operation-bridge
kernel with closed profiles, not independent watcher and registered-server
stacks. `docs/architecture/repository-authority-map.md` must keep bridge
lifecycle in Swallowtail, watcher semantics in the existing watcher service,
Longhorn generic dispatch outside the bridge, and Desktop domain semantics,
schema, admission, disclosure, packaging, selection, and policy outside all
producer services. The architecture promotion must point to
Contract 060 for the realized watcher profile and to the new registered-server
contract for the additional profile. Neither spec text nor a new service name
may silently replace that dependency direction.

## Contracted Architecture

```text
Desktop task/context/skill selection
  -> Desktop-issued admission plus domain tool/schema declarations
  -> Swallowtail immutable registration snapshot and capability selection
  -> adapter prepared plan
  -> Swallowtail operation bridge lease inside the Desktop host
  -> route-specific native tool callback or private MCP attachment
  -> linked Longhorn typed validation/dispatch
  -> exact Desktop tool implementation and policy
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
- stable namespaced tool identity supplied by Desktop: producer namespace plus
  local tool name;
- schema namespace, media type, dialect, revision, bounded bytes, and digest;
- native client-tool, MCP, app-tool, or provider-owned execution kind;
- supported transports and exact protocol/version ranges;
- required host services and execution-host identity;
- credential-reference requirements without credential material;
- process/environment recipe references without raw executable paths or env;
- positive call, byte, concurrency, deadline, and result bounds;
- Desktop-declared effect/retry posture plus Swallowtail-enforced no-replay,
  reconnect, and teardown posture; and
- source identity and freshness suitable for Contract 061 projection.

Within one registration snapshot, one namespaced tool identity binds exactly
one execution kind. The same namespaced identity cannot appear as native,
MCP, app, or provider-owned through another registration or alias. A producer
that intentionally exposes equivalent business behavior through two kinds
must assign distinct namespaced identities and declare their relationship.
Kind uniqueness is validated before prepare because it selects permission,
dispatch, event, result, and receipt paths.

Discovery returns descriptions only. Selection binds one exact registration
revision and schema digest into a prepared plan. Registration cannot choose a
provider, model, route, tool, credential, or server for the consumer.
Desktop owns the domain name and schema content; Swallowtail alone owns the
immutable snapshot that carries their bounded revision/digest to a prepared
operation. Longhorn consumes that snapshot for generic validation and dispatch
and cannot mint, mutate, or shadow it with a callable catalogue.

### Contract 060 Bridge Kernel And Server Lease

Contract 060 is live prior art, not a pattern to copy. The implementation must factor its existing operation-scoped lease, private bearer, execution-host
binding, ready-before-provider ordering, correlation, admission freeze, and
joined listener/task/resource teardown into one reusable private-operation
bridge kernel. `WatcherBridge` remains the source-compatible closed watcher
profile over that kernel. Its reserved methods, watcher authority, completion
barrier, safe records, omission behavior, and released `claude-code` behavior
do not widen.

Extend `HostServices` with one registered tool bridge profile backed by that
same kernel, not a sibling lease manager or loopback runtime. Desktop mounts a
linked Longhorn dispatch/validation implementation behind the typed host seam.
Reuse existing `WatcherBridge`, `CredentialService`,
`WorkingResourceService`, `SchemaService`, `TimeService`, and scoped-task
ownership. The registered-tool profile has a separate protocol namespace,
Swallowtail registration authority, Longhorn generic dispatch, and safe result
vocabulary. It cannot call Contract 059 watcher methods unless selected through
the existing watcher profile. Slice 1 starts no Longhorn process or daemon.
Do not encode credentials, executable paths, complete environment, or server
endpoints in public requests.

Opening a server lease binds:

- execution host;
- configured instance and prepared plan;
- Desktop process incarnation and opaque admitted task/session/attempt;
- admitted workspace generation, task generation, and revocation state;
- the one-to-one Swallowtail operation/turn attempt and lease generation;
- exact registration revision and selected tools;
- transport generation and negotiated protocol version; and
- deadline, cancellation, and maximum concurrency.

The authenticated Desktop host binding supplies identity; model arguments,
provider session IDs, tool arguments, and PID never do. PID is diagnostic only.
The Desktop process incarnation plus Swallowtail lease generation is instance
authority. Before host work, Swallowtail validates that incarnation, workspace
and task generations, revocation state, admitted task/session/attempt binding,
and lease generation. The lease reaches ready before provider dispatch. Close
freezes admission, abandons or completes issued calls, joins callbacks and transport readers,
releases the linked dispatch binding and resources, then releases credential
leases. Desktop owns application startup/shutdown; Swallowtail owns bridge
lifetime inside that host. Drop is defensive cleanup, never success evidence.
Production and development control profiles remain distinct. The Longhorn
Contract 022 development loopback is not a registered-tool profile and cannot
enter a release build through this bridge, host binding, or feature selection.

### Transport And Reconnect

The slice-1 transport is Contract 060's existing private loopback HTTP carrier
extended with the minimum registered-tool MCP
profile and, only if required by frozen protocol evidence, bounded SSE. The
loopback listener, bearer, ready barrier, generation checks, admission freeze,
and joined tasks remain one implementation. SSE must not create another
listener or lifecycle. Unix sockets, WebSocket, remote public HTTP, and
attached ambient servers remain out until separately qualified.
Remote hosts and standalone server distribution are later profiles; the
linked Longhorn slice cannot infer them. Stdio is not a carrier: it is an
attachment shape over this same loopback carrier, admitted only as the
Mediated Stdio Proxy Attachment below.

Initialization negotiates one exact protocol version and server identity.
Unsupported or substituted values fail before provider work. Reconnect may
replace transport only while retaining the same registration, host, task,
session, turn/attempt, and server generation. It never replays a mutating call.
The consumer must explicitly retry a call whose execution outcome is unknown.
Every retry that may dispatch provider work uses a fresh Desktop attempt and a
fresh Swallowtail operation binding. Mutating or indeterminate calls never
replay.
Recovery never revives an old lease. Every Desktop startup creates a fresh,
non-reused process incarnation, and private lease authority cannot survive a
restart.

Every call has a caller identity, server/tool identity, call id, task, session,
turn, attempt, deadline, cancellation state, and transport generation. Late,
foreign, duplicate, stale-generation, post-terminal, and post-close results
fail without reaching the provider.

### Progress And Partial Notifications

Progress is non-terminal and never satisfies the exactly-once result rule.
Each notification is positively bounded and correlated to the same server,
namespaced tool identity and kind, call id, task, session, turn, attempt, and
transport generation as its call. A per-call monotonic sequence admits
notifications in order. Duplicate, regressive, foreign, stale-generation,
post-cancel, post-terminal, and post-close notifications fail before consumer
or provider delivery. Cancellation freezes new progress admission before the
terminal cancellation result; reconnect does not replay missed progress.

The first common tranche permits bounded replacement snapshots or deltas only
when the selected route can preserve their declared semantics. Codex
app-server already projects `item/mcpToolCall/progress` as provider-owned tool
display progress; that released observation remains provider activity, not
evidence of consumer MCP registration or result authority. Claude Code's
watcher bridge exposes only its closed watcher protocol and completion state.
Claude SDK and Claude ACP gain no common tool-progress claim until an exact
route mapping and evidence gate passes. Grok's accepted live gate proved the
opposite disposition: the route delivers no consumer tool progress at all, and
the qualified projection records exactly that (`NoProgress`), never a partial
or implied progress channel. Unsupported progress may be
dropped only before prepare by selecting a no-progress profile, never silently
after an operation begins.

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
| native client tool | linked Longhorn dispatcher to Desktop implementation | declare through provider wire; correlate call/result |
| MCP tool | linked Longhorn dispatcher to Desktop implementation | own private bridge lease and route attachment or mediate the call |
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
| `claude-code` | existing provider-direct private watcher MCP through Contract 060 | released `--mcp-config`/`--strict-mcp-config` carries only `swallowtail-watchers`; reuse proves the common bridge kernel while its closed protocol, operation scope, and omission behavior remain unchanged; it is not generic tool registration |
| `claude-agent.sdk` | merged consumer-declared stdio MCP attachment, then common-bridge integration | PR 255 proves strict declared stdio configuration, explicit child env, per-call mediation, bounded status, typed required-connect failure, and unchanged omission; it does not prove SSE/HTTP/in-process/managed MCP, a real provider turn, the centralized snapshot, or Contract 060 integration |
| `claude-agent.acp` | host-mediated native callbacks only; MCP withheld | current ACP sends an empty server list and has no MCP input facade; add direct MCP only after exact bridge schema, version, permission, and lifecycle evidence |
| `codex.app-server` | host-mediated dynamic native tools | this is already the qualified execution seam; observed MCP activity is not registration or result authority; provider-direct MCP needs a separate app-server surface and corpus |
| `grok-build.acp` | route-local ACP client-MCP courier through the Contract 063 mediated-stdio kernel, scoped to exact maintained `1.0.4..=1.0.5` | the accepted Card 128 live gate ran exact Grok Build `1.0.4` and `1.0.5` each admitting the Swallowtail-owned courier, listing its tools, and completing one registered call (Research 295); the qualification is version-scoped — other executable versions project unqualified rows and a registered open refuses with `version_not_admitted` before any host, lease, or provider work — the consumer Deny stays unrepresented, progress stays undelivered, selected skills stay `NotCarried` for want of any ACP skill input, and omission still sends `mcpServers: []` |

The common API must not promise that every route accepts MCP. A route publishes
the exact attachment or mediation kind through Contract 061.

Card 084 merged through PR 255 at `8a377c1b` after independent exact-head
review at `f52c48c6`. It is merged but untagged evidence, not released `R`
behavior and not a reviewed centralized producer contract. The preserved
`g05-card084-claude-client-mcp` branch at `7cb08b1f` remains archival history,
not a second lane. Batch C consumes PR 255 rather than recreating it.

### Context, Instructions, Skills, And References

Keep four inputs distinct:

1. session developer instructions, fixed at open or validated resume;
2. per-turn user text in `TurnRequest`;
3. an immutable selected-skill bundle derived from Contract 062 inventory; and
4. bounded required references resolved by opaque host references.

Desktop chooses the skill and references and defines any tool whose business
semantics consume them. Swallowtail validates identity, bounds,
digest, lifecycle, and transport; it does not discover repositories, decide
relevance, generate product prompts, or claim the provider followed the skill.

A skill bundle contains stable skill identity/revision, content digest,
declared required-reference descriptors, provenance, and bounded resolved
content or opaque references. Missing, changed, oversized, inaccessible, or
foreign references fail before provider work. Raw client paths and unrelated
repository content never cross the public boundary.

Desktop app context is deny-by-default and limited to the admitted workspace:
stable entity IDs, relevant hierarchy, selected reader/passage and filters,
source revision, and separately labelled live-viewing versus immutable task
scope. It excludes unrelated workspaces, other-thread transcripts/drafts,
hidden settings, credentials, endpoint/environment material, and arbitrary
filesystem inventory. Model-visible paths are workspace-relative. Every field,
record, page, and aggregate payload has a positive Desktop-declared bound that
Swallowtail enforces; concrete numeric limits are producer-settleable protocol
evidence, not a generic operator gate.
Content is untrusted data. Its text, metadata, links, and embedded instructions
cannot broaden registered tool authority, admission, or app-context disclosure.

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


## Compatibility

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


## Additive API And State Machine

The following new public types belong in swallowtail-runtime. Constructors
validate positive bounds and identity; fields are private with typed accessors.
Schema/body wrappers expose contents only through explicit execution accessors,
never Debug/Display/diagnostics. No new required field is added to existing
SessionOptions, TurnRequest, watcher records or public constructible structs.

| Type | Required fields or semantics |
| --- | --- |
| RegisteredToolSnapshot | Immutable server ID/revision; unique namespaced tool IDs/kinds; schemas/digests; effect declarations; limits; no runtime resource |
| RegisteredToolSelection | Snapshot reference, selected IDs, exact transport/protocol, effective bounds; immutable after prepare |
| ConsumerAdmissionBinding | Trusted opaque incarnation, workspace/task generations, session/task/attempt IDs and live revocation source; no serialized bearer |
| RegisteredToolOpenRequest | Configured/prepared identity, operation scope/turn, selection, admission binding, deadline |
| RegisteredToolBridgeLease | Non-Clone scoped kernel token, safe binding accessors, driver-private endpoint/auth; nonserializable; defensive Drop |
| RegisteredToolCall | Validated binding, unique call ID, selected tool, bounded arguments and deadline |
| RegisteredToolProgress | Same binding/call plus monotonic sequence and bounded payload |
| RegisteredToolOutcome | Exactly one correlated bounded result or typed failure, with known/unknown execution disposition |
| RegisteredToolCompletionState | Admission state and bounded outstanding-call/cleanup truth |
| RegisteredToolCleanupCause | Completion, cancellation, deadline, provider failure, transport failure or explicit close |

The new optional port follows existing object-safe BoxFuture conventions:

```rust
trait RegisteredToolBridgeHostService: Send + Sync {
    fn open(&self, request: RegisteredToolOpenRequest)
        -> BoxFuture<'_, Result<RegisteredToolBridgeLease, RuntimeFailure>>;
    fn completion_gate(&self, lease: &RegisteredToolBridgeLease)
        -> BoxFuture<'_, Result<RegisteredToolCompletionState, RuntimeFailure>>;
    fn close(&self, lease: RegisteredToolBridgeLease, cause: RegisteredToolCleanupCause)
        -> BoxFuture<'_, Result<CleanupOutcome, RuntimeFailure>>;
}
```

HostServices adds a private optional Arc port, with_registered_tool_bridge and
registered_tool_bridge getter. HostServiceKind is currently exhaustive: do NOT
add a variant in this slice. A separate typed registered-profile preflight
checks port availability and selected topology before provider work. Existing
HostServiceKind projections stay unchanged. A later enum change requires an
explicit compatibility decision, not a claim that adding a variant is harmless.

A RegisteredToolDispatcher object-safe host port provides dispatch(call, context) ->
BoxFuture<Result<RegisteredToolOutcome, RuntimeFailure>>. The local composition
builder receives this Arc port; portable crates never depend on Longhorn.
Only the kernel can mint the live validated binding passed to dispatch. The
host admission port revalidates revocation immediately before dispatch and
result delivery; an open-time snapshot is insufficient. Schema and consumer
policy checks are distinct from transport authentication, not a second issuer.
Progress is a bounded lease-associated channel and must obey the same live check.

Lifecycle: registered (no resources) -> prepared -> opening -> ready ->
call-pending -> ready -> frozen -> closing -> closed. Any opening failure joins
partial resources. Cancellation/deadline/terminal freeze admission before
settling or abandoning calls; completion_gate observes and freezes when clear,
never silently waits or turns provider-terminal into success. Reconnect is
allowed only within the same live bound attempt and cannot unfreeze a lease.
Dispatch/result/progress/revocation races have one serialized admission point.
Duplicate results are rejected; exactly-once acceptance does not claim exactly-
once remote execution. Cleanup failure is visible and never a successful close.

## First Implementation Limits And Protocol Qualification

Maximum one outstanding call per lease; 64 selected tools; 64 KiB per schema;
1 MiB aggregate schema bytes; 256 KiB arguments or result; 64 KiB progress item;
32 queued progress items; 64 KiB total selected skill/reference content.
Consumer limits may narrow these ceilings. Calls expire at the earlier of their
operation deadline or 60 seconds; opening and joined cleanup are bounded by
10 seconds each and the parent lifecycle budget. Reject overflow before work,
never silently truncate required content. Tests use virtual clocks where feasible.

Protocol versions are an explicit nonempty subset of frozen qualified route
artifacts; no latest/default negotiation or production route is enabled by
card114. Provider-free fixtures negotiate an explicitly named test version.
HTTP carries the registered profile only when its route corpus qualifies it;
SSE remains withheld until such evidence exists. Native callback mediation can
consume the same binding without a listener. Card084's stdio SDK support is
retained separately; this contract does not pretend it proves HTTP support.

## Implementation Conformance Oracle

Falsify kind/schema substitution, missing service, stale incarnation/generation,
revocation racing dispatch/result, post-cancel/reordered progress, duplicate
result, deadline/terminal races, disconnect with unknown execution, concurrency
and byte overflow, and partial-open/close failures. Count listener and resource
owners when both profiles are selected; retain all existing watcher fixtures
unchanged. No Longhorn dependency or consumer domain type enters portable crates.
No scheduling amendment is promoted: Contract028 gates remain unchanged for all
three target routes. Descriptive Contract061 fields grant no runtime support.

## Trusted Admission Port

ConsumerAdmissionHostService is a Send + Sync object-safe port with
validate(binding, phase) -> BoxFuture<Result<AdmissionVerdict, RuntimeFailure>>.
AdmissionPhase is BeforeDispatch or BeforeDelivery; AdmissionVerdict is Current
or Revoked with a bounded reason code. The immutable binding carries a private
Arc to this consumer-supplied port. Constructors require the trusted host to
supply it; provider input cannot create the binding. The kernel checks fixed
identity equality and the live verdict at its serialized admission point.
Revocation freezes future dispatch/delivery; an already executing side effect
may finish indeterminately and is reported honestly rather than rolled back by
claim. Cancellation handles remain bound to that exact call and lease.

A consumer does not reissue identity during validation. A new generation needs
a new admitted attempt/binding. Longhorn's dispatcher validates schema and
invokes consumer policy using the already validated call token; it never checks
an independent bearer or constructs a second admission model.

## Selection And Dispatch Context

RegisteredToolPreparation::new(snapshot, selection, admission, limits) creates
an immutable opt-in request without opening resources. Its prepare(hosts,
configured_identity, operation_scope, turn, deadline) returns
Result<PreparedRegisteredToolBinding, RuntimeFailure> after identity, limits,
port and topology checks. The prepared binding exposes safe descriptions only;
its open() delegates to the bound RegisteredToolBridgeHostService and returns
the scoped lease. Construction cannot choose or start a provider. It composes
beside existing prepared facades rather than wrapping them in a generic runner.
Route cards add an optional with_registered_tools(preparation) builder to the
exact route preparation object; absence preserves all previous behavior.
Card114 proves this public compositional path with a consumer fixture. Adapter
builder wiring follows in116/117 only; no current provider support is inferred.

SelectedSkillBundle::new(identity, revision, digest, references, bounds) and
RequiredReferenceDescriptor identify immutable selected content or opaque host
references. SelectedSkillBundle::resolve(host_resources) returns
Result<ResolvedSkillBundle, RuntimeFailure>; all required digests and aggregate
bounds validate before prepare. Swallowtail is the digest authority for
transported selected content: it computes the canonical digest (`sha256:` plus
lowercase hex over the exact bounded transported bytes, after bounds checks)
and fails on mismatch with the declared digest before provider work; a
host-reported digest is an input claim, never the verification (ruled
2026-09-07, card 115 PR 261). Route preparation's optional
with_selected_skill_bundle(resolved_bundle) carries it separately from session
instructions and verbatim TurnRequest. Card115 implements producer vocabulary;
route builders and exact transport are the adapter cards' responsibility.

RegisteredToolDispatcher::dispatch receives both RegisteredToolCall and
RegisteredToolDispatchContext, returning BoxFuture<Result<RegisteredToolOutcome,
RuntimeFailure>>. DispatchContext owns a call-bound cancellation observation
handle and a bounded RegisteredToolProgressSink. The sink's
publish(sequence, bounded_payload) returns BoxFuture<Result<(), RuntimeFailure>>
resolved after the kernel's identity/generation checks and the live
BeforeDelivery admission verdict at the serialized admission point; revoked,
out-of-order or full queues fail explicitly. Publish is asynchronous because
the live verdict comes from ConsumerAdmissionHostService::validate, itself a
future; a synchronous publish could only consult a snapshot, which this
contract rejects (ruled 2026-09-07, card 114 PR 260). The handle cannot cancel another call or extend the
lease. The kernel owns cancellation signaling and progress queue lifetime;
terminal close joins dispatch futures and drops the sink only after admission
is frozen. A dispatcher cannot mint validated tokens or publish on another call.

## Admission Linearization And Failed Cleanup

A successful live validation verdict is committed at the kernel's serialized
admission point before dispatch. Revocation committed earlier prevents dispatch;
revocation after admission may encounter executing work and freezes subsequent
calls/progress/results. This is not a claim to undo an already-started effect.

Dispatcher implementations must cooperate with cancellation and settle within
the declared deadline. The ten-second cleanup budget bounds the normal join
attempt, not a license to detach an uncooperative future. Timeout reports failed
cleanup; the lease stays frozen and its resources remain under the existing
host shutdown/reap owner until joined or explicitly accounted as unreaped under
the host contract. It never reports Closed/Clean, returns credentials for reuse,
or allows a new attempt to inherit the failed lease. Tests include a deliberately
uncooperative dispatcher and prove retained ownership plus visible failure.

## Mediated Stdio Proxy Attachment — 2026-09-07

Promoted from card 116 ATTACH-01 (provider-free planning capsule). Applies to
providers that consume registered tools only through an MCP stdio server they
spawn themselves (`claude-agent.sdk` on SDK `0.3.259`).

**Shape.** Stdio is an attachment shape, not a carrier. The admitted attachment
is `MediatedStdioProxy` over the existing `RegisteredToolTransport::PrivateLoopbackHttp`.
A Swallowtail-authored courier process, spawned by the provider as its declared
stdio MCP server, speaks MCP stdio toward the provider and the registered-tool
loopback profile toward the kernel. It mints nothing. One listener, one lease,
one generation space, one secret source, and one admission authority remain
the kernel's. Card 084 consumer-declared servers stay a separate unchanged
path. SSE/HTTP provider transports, in-process, managed, remote, and
standalone distribution remain withheld.

**Additive vocabulary.** `RegisteredToolAttachment { HostMediated,
MediatedStdioProxy }`; `RegisteredToolProxyRecipe` (executable reference,
environment reference, fixed wire tag; no raw values); a private, non-`Clone`,
non-serializable, zeroizing `RegisteredToolProxyRendezvous`; and a safe
`RegisteredToolAttachmentDescriptor`. Selection, topology, and readiness bind
the exact attachment and require recipe resolution before open.

**Secret handoff.** Endpoint, bearer, and generations reach the courier only
through an operation-scoped one-shot rendezvous file: created `0600` and
exclusive under a host-private directory (never the working resource,
workspace, or any durable configuration), unlinked by its first reader, and
expired at the ready barrier whether or not read. The provider's argument
vector carries only the fixed wire tag and the non-authoritative rendezvous
path; the child environment is the card 084 allowlisted environment with no
`SWALLOWTAIL_*` authority and no runtime-added values. Tool credentials stay
host-side `CredentialRef`s resolved in the dispatcher. This is the same handoff
class Contract 060 already uses for the watcher MCP configuration file and is
recorded there.

**Lifecycle.** Prepare references and readiness → bind the existing listener →
mint one lease and transport generation and the zeroizing secret →
materialize the rendezvous → provider spawns the courier → courier reads and
unlinks, connects, authenticates, negotiates → only then Ready → every call
passes provider-side `canUseTool` mediation and the kernel's `BeforeDispatch`
and `BeforeDelivery` verdicts → freeze or revoke → settle or abandon → close
joins listener tasks → release the resource and zeroize. A teardown timeout is
`TeardownFailed`: the lease stays frozen, resources are retained under the
host reap owner, and it is never reported clean.

**Slice 1 bounds.** One courier, one connection, one call in flight; no
reconnect and no progress; connect bounded by the lesser of the open
remainder and ten seconds; existing 256 KiB payload ceilings; the reserved
server name `swallowtail-registered-tools`; omission byte-identical to today.

**Failed-open evidence (claude-agent.sdk).** A rejected Claude SDK prepared
open — registered or plain — keeps its exact stable route failure and gains
one additive observation-derived receipt beside it: the underlying route
code, the open stage, the exact bounded sidecar subcode when the sidecar
rejected the open command, whether provider readiness was reached, and the
observed cleanup disposition. The cleanup disposition is three-way and
mutually exclusive: a failure before anything was acquired reports
`NotAcquired`; a completed ordered continuation reports `Confirmed` with its
staged observations (resource and credential release, the owned-tree
survivor posture, and, when a registered lease was opened, its bridge close
— admission frozen, joined calls, listener and registry release); and an
unconfirmed cleanup reports `Unconfirmed` with every staged observation
absent. When the deadline or an unconfirmed cleanup replaces the returned
error, the receipt still carries the underlying failure's route code and
facts. The receipt carries no paths, bearer material, credentials,
environment values, provider content, or sidecar stderr, and it is never a
support, qualification, availability, or release claim; both Contract 061
registered-tool cells stay owned by the producer gap until a separately
authorized passing live capsule exists.

**Provider-free falsifiers.** The F1–F18 set in the capsule is the conformance
oracle: omission, ready ordering, one-shot expiry, redaction, argv and
environment, exact kind and name mapping, correlation, deny, concurrency and
bounds, revocation races, cancel and deadline, transport loss without
re-authentication, stale, foreign, and late frames, exactly-once settle,
uncooperative teardown, the shared watcher and registered listener and
generation space, and every existing fixture unchanged.

**Live gate (later, separate authority).** The exact tuple is frozen before
any turn: `claude-agent.sdk`; SDK `0.3.259` at its recorded digest; native
`2.1.259`; the pinned Node; JSONL v1; proxy wire `swallowtail-registered-tool-mcp-v1`;
the exact MCP protocol version; `private-loopback-http` plus
`mediated-stdio-proxy`; `strictMcpConfig` true; `settingSources` empty;
`allowedTools` omitted; one disposable tool. Stop conditions: authority
leakage, ambient MCP, lazy attach, automatic respawn or retry, `canUseTool`
bypass, ambiguous identities, incomplete cleanup.

**Ownership.** Swallowtail owns the wire specification and a feature-gated
reference courier binary inside `swallowtail-host-local`; no new crate in
slice 1, on the condition that the wire specification is a distinct module
with no dependency on host-local runtime internals, so that it can move to
its own crate without a wire change. Desktop owns packaging and placement
and resolves the executable reference. A conforming consumer-built courier is
admitted, bound by the wire specification and its falsifiers, never by the
artifact. Any claim that an external courier conforms requires the wire
module to have been promoted to its own crate with a frozen version.
