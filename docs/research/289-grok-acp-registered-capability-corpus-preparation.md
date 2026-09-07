# 289 Grok ACP Registered-Capability Corpus Preparation

Status: preparation artifact; no production claim
Owner: Tom
Date: 2026-09-07
Card: g05.035 / 118
Baseline: main `70909172` (Cards114-115 merged; Card117 not present)

## Question

Does the exact `grok-build.acp` route expose enough provider and ACP surface
evidence to map the common registered-tool/server capability to Grok, while
preserving provider-owned tool identity, one-shot permission semantics, bounded
lifecycle, and no mutating replay?

## Method And Boundary

This is read-only corpus preparation. Evidence was collected on 2026-09-07
from public primary ACP and xAI documentation, the public filtered Grok Build
source tree, and checked-in Swallowtail source and exact-version fixtures. No
Grok process, provider endpoint, account, login, credential, live client,
external MCP server, prompt, tool call, or paid inference was used.

The exact local route corpus remains `grok-build.acp` over ACP v1 stdio on the
`grok-build.executable` axis. Existing exact fixtures cover maintained `1.0.4`
and `1.0.5` identity and handshake shape. Current main also contains the
provider-neutral Contract063 registration kernel, selected-bundle transport,
and descriptive projection from Cards114-115. No Grok adapter consumes those
types. The current public xAI source tree is later than those exact package
commits and is corroboration only; it does not extend a compatibility claim.

No runtime, adapter, fixture, public API baseline, route matrix, contract,
roadmap status, or claim was changed by this preparation. The research front
door was additively updated during coordinator closeout to index this artifact.

## Corpus Index

| Source | Exact use | Digest or identity |
| --- | --- | --- |
| [ACP v1 session setup](https://agentclientprotocol.com/protocol/v1/session-setup) | `session/new`, `session/load`, and `session/resume` accept client-supplied `mcpServers`; stdio is required, HTTP/SSE are capability-gated | Retrieved 2026-09-07; SHA-256 `bab5140924d6f07aa318055d2f3a90aacaa0faa72dcdb5c924d16bd031217cca` |
| [ACP v1 tool calls](https://agentclientprotocol.com/protocol/v1/tool-calls) | `session/update` tool-call reporting and `session/request_permission` option/response shape | Retrieved 2026-09-07; SHA-256 `8a16c3f750cb1d11aeea4908622df3be20ee96839af2b90c9696608f6888d007` |
| [xAI Headless & Scripting](https://docs.x.ai/build/cli/headless-scripting.md) | official ACP launch and `session/new` example; example sends `mcpServers: []` | Retrieved 2026-09-07; SHA-256 `a4f39daf25f81aba5dba79265d12d0e4ec444e6b28dcbe3c2335c05401052097` |
| [xAI MCP Servers](https://docs.x.ai/build/features/mcp-servers.md) | Grok-native MCP configuration, project scope, namespaced `search_tool`/`use_tool`, and credential-bearing transports | Retrieved 2026-09-07; SHA-256 `e0b9ce6bf03bf2979b1bbfa7d8ae54ecc1d38b32cdeb5c5dbfdb103cea9dabf1` |
| [Grok Build MCP guide](https://raw.githubusercontent.com/xai-org/grok-build/main/crates/codegen/xai-grok-pager/docs/user-guide/07-mcp-servers.md) | later public source description of native MCP discovery, server config, and tool naming | `SOURCE_REV` `a549186d9d39311f2d3ee4208db62af8c65aa476`; guide SHA-256 `b6d5c2a0a96d8349d074e2cfc9f0ed817f892d24f2891f74b3827d721b02c2eb` |
| [Grok shell README](https://raw.githubusercontent.com/xai-org/grok-build/main/crates/codegen/xai-grok-shell/README.md) | later public source ACP examples and architecture diagram; all shown ACP session setup lists use `mcpServers: []` | README SHA-256 `b1be9ca13c67d83b39a61c8813339aaa8a8fda983eb6967a329007471aa76988` |
| `crates/swallowtail-adapter-grok/tests/fixtures/grok-1-0-5/compatibility.json` | exact `1.0.5` route identity, ACP v1, selected command, successful session-new fixture, empty-MCP route posture, and unmapped ACP MCP metadata | SHA-256 `948c1589afe6a037f7dc484d88e6ceb273995025ff05e81de729f5d13415651c` |
| `crates/swallowtail-adapter-grok/tests/fixtures/grok-1-0-5-identity.json` | exact `1.0.5` package/source identity and no-prompt observation boundary | SHA-256 `c0acd39c3fddafdd57874ef3c006ad07c6310b3d1bff0d3b6cbebcf33297bbba` |
| `docs/research/288-shared-harness-capability-placement-audit.md` | promoted planning evidence for the shared registration boundary and released Grok gap | SHA-256 `38d18686524eb971d8ef24022079a79af2ea35ba95b8a4bb22a34c8764c7f214` |
| `crates/swallowtail-runtime/src/registered_tool/` | current-main Contract063 snapshot, selection, binding, call, result, progress, admission, and lease vocabulary | main `70909172`; provider-free only |
| `crates/swallowtail-runtime/src/consumer_route_projection/registered_capability.rs` | current-main Contract061 rows for registration, kind, transport, permission, progress, skill/reference delivery, and qualification posture | main `70909172`; provider-free only |
| `crates/swallowtail-testkit/src/registered_tool_assertions/` and `crates/swallowtail-host-local/src/registered_tool/` | mounted provider-free registration, call/result, progress, race, revocation, deadline, and joined-cleanup probes | main `70909172`; provider-free only |

The source and adapter anchors used below were inspected from the clean
baseline. Their hashes are recorded to make the preparation reproducible:

| Local anchor | SHA-256 |
| --- | --- |
| `crates/swallowtail-adapter-grok/src/driver.rs` | `6c982a8666325ac453f64fb634d6de2ec6bf861cf14cd5fe1d9469b03a856c01` |
| `crates/swallowtail-adapter-grok/src/connection.rs` | `ee2704f24e9dc6710b134721cd76cc9e9f008577c303c2db524d6a0b3a2c1deb` |
| `crates/swallowtail-adapter-grok/src/connection/dispatch.rs` | `7a1df431ff24dcdd8647b55a1394c2dcffa8e5a72e027a93dad50f1fa80155d2` |
| `crates/swallowtail-adapter-grok/src/consumer_route_projection.rs` | `c3876746711066b4f46a2e97d70f6d94e48f15b260dc1f17729ccbcf64b39618` |
| `crates/swallowtail-adapter-grok/tests/acp/consumer_route_projection.rs` | `cfb8de961b9cbb0c1f312bdb3ff13c59683fcf218d13cfc1aa9d28932b6da350` |

## Evidence Matrix

| Surface | Evidence collected | Boundary of the evidence |
| --- | --- | --- |
| Grok-native MCP | Official xAI docs describe configured stdio/HTTP/SSE servers, project-scoped config, namespaced server tools, and built-in `search_tool`/`use_tool`. | This proves a Grok-native MCP subsystem. It does not prove that the ACP client may inject a server per session, that a non-empty ACP `mcpServers` list is accepted on exact `1.0.4`/`1.0.5`, or that Swallowtail can bind the server lifecycle. |
| ACP client-supplied MCP | ACP v1 specifies `mcpServers` on session setup, absolute stdio command shape, optional HTTP/SSE capability gates, and the expectation that an Agent connects to requested servers. | ACP protocol allowance is not Grok implementation evidence. No Grok exact-version transcript in this corpus proves connection readiness, tool discovery, or result routing for a client-supplied server. |
| Grok official ACP examples | xAI’s official ACP examples launch `grok agent stdio` and send `mcpServers: []` on `session/new`; the later public README repeats the empty list for `session/new` and `session/load`. | The examples establish the documented path and its empty-MCP posture only. They do not show that non-empty input is rejected, ignored, or supported. |
| Exact `1.0.5` fixture | The checked-in fixture records ACP v1, `session_new_ok`, `provider_prompt_sent: false`, `mcpCapabilities` as an unmapped initialize key, and vendor MCP notifications as unmapped observations. | The fixture has no consumer server, `tools/list`, tool-result, or per-session MCP readiness transcript. It cannot qualify registration or result dispatch. |
| Current Swallowtail open path | `driver.rs:218-227` opens ACP and sends `session/new` with `mcpServers: []`; `connection.rs:150-155` sends the same empty list during `session/load`. | There is no prepared consumer-server input or route-local registration path in the current adapter. Changing the empty list would be runtime work and is outside this preparation. |
| Current Swallowtail callback path | `connection/dispatch.rs:49-75` handles `session/update` and vendor metadata; `:77-118` accepts only filesystem reads and `session/request_permission` as agent-to-client callbacks. | No client-to-agent registered-tool result path, server lease, schema binding, or result correlation is present in the Grok adapter. The existing permission exchange remains a separate one-shot route-local mechanism. |
| Current projection | `consumer_route_projection.rs:198-277` emits prepared/session/model/session-option/activity rows only. The checked-in projection test expects ten emitted rows and withholds model catalogue, persistent-session posture, and negotiated-model observation. | No consumer-tool or MCP row is emitted. This is existing route truth, not a new claim or a reason to change the matrix during preparation. |

## Contract063 Gap Capsule — Current Main

This is a field-level gap ledger, not a support matrix. `Missing` means that
the exact Grok route has no frozen evidence or adapter path for the field.
Provider-neutral types and provider-free testkit proof establish the required
shape only; they do not qualify Grok. A Grok `session/update` tool activity is
provider-owned display activity. A Grok `session/request_permission` exchange
is provider-owned permission mediation. Neither may be relabeled as a
consumer-registered MCP call or result.

### Registration and prepared selection

| Contract063 field or invariant | Exact Grok evidence | Gap | Authoritative source / provider-free route | Smallest evidence action under current authority |
| --- | --- | --- | --- | --- |
| Stable server identity and registration revision | No ACP server identity or revision appears in the exact `1.0.4`/`1.0.5` fixtures; both route wires send `mcpServers: []`. Native xAI configuration names servers, but that is a different authority. | Missing. No Swallowtail `server_id`/`revision` can be bound. | Contract063, **Registry And Schema Discovery**; `RegisteredToolSnapshotInput` plus `fixture_snapshot_input` and `RegisteredToolSnapshot::new`. | Inspect frozen binary/source strings for a server identity and revision; record absent if none. Do not derive one from a native config name. |
| Stable producer namespace plus local tool name | No client-supplied server or tool list; no `tools/list`/equivalent specimen. | Missing. | Contract063 registry identity and kind-uniqueness rule; `RegisteredToolId::new` in `fixture_tool_id` and registration assertions. | Static-search the exact artifacts for a client-visible tool-list schema. A name found in native MCP docs is not an ACP registration. |
| One execution kind per namespaced identity | Exact ACP `tool_call`/`tool_call_update` fixtures are provider-owned activity and carry no consumer registration kind. | Missing for `Mcp`; provider-owned activity cannot satisfy it. | Contract063 **Tool Kinds And Permission Paths**; `snapshot_binds_one_kind_per_identity` and `unsupported_tools_fail_before_dispatch`. | Preserve the provider-owned classification; use the provider-free kind-substitution assertion only to keep the future mapping fail-closed. |
| Input/output schema namespace, media type, dialect, revision, bounded bytes, digest | No consumer tool schema or `tools/list` result is frozen. | Missing in full. | Contract063 registry/schema requirements; `fixture_schema`, `RegisteredToolSchema::new`, and `snapshot_rejects_oversized_schema`. | Search frozen artifacts/docs for an exact schema response. If absent, leave schema and digest unknown; do not hash provider display content. |
| Supported carrier and exact protocol/version subset | Installed route identity is `acp-v1-stdio`; exact fixtures record ACP `protocolVersion: 1`. They do not prove a registered-tool carrier or non-empty MCP acceptance. | Partial for base ACP only; registered-tool transport/version is missing. | Contract063 **Transport And Reconnect**; `RegisteredToolTransportSupport::new`, `fixture_protocol_version`, and `selection_rejects_undeclared_transport_and_version`. | Record only `acp-v1-stdio` / ACP v1 for the existing route. Do not introduce a registered-tool version from `latest`, `mcpCapabilities`, or a later source tree. |
| Required host services and execution-host identity | Existing Grok descriptor names process, credential, working-resource, task/time services for its current roles. No registered snapshot binds a consumer host. | Missing for the registered profile. | Contract063 server lease/open binding; `fixture_snapshot_input` and `ready_registry_reports_every_dimension`. | Reuse only the provider-free host identity/readiness probe. A real Grok value needs a future adapter mapping and exact host-bound specimen. |
| Credential-reference requirements without credential material | Exact artifacts document `cached_token`/`grok.com` access for the ACP route. No consumer-server credential reference is present. | Missing for a registered server. | Contract063 registry ownership and credential-material prohibition; snapshot credential-reference fields and `schema_and_snapshot_hold_no_runtime_resource`. | Record the existing ACP access mechanism as route access only. Do not inspect, add, or transmit credentials. |
| Process/environment recipe references without raw paths or environment | Exact launch is `--no-auto-update agent stdio`; no consumer server recipe is frozen. | Missing. | Contract063 registry recipe-reference rule; opaque `ExecutableRef`/`EnvironmentRef` fields in `RegisteredToolSnapshotInput`. | Keep route launch identity separate from a server recipe. Static inspection may record an opaque recipe identity only if a frozen source declares one. |
| Positive call, byte, concurrency, deadline, and result bounds | Grok has adapter limits for ACP output, framing, recovery, and permission callbacks, not Contract063 registered-tool bounds. | Missing for the registered profile. | Contract063 **First Implementation Limits**; `RegisteredToolBounds::ceiling`, `fixture_snapshot_input`, and `ready_registry_reports_every_dimension`. | Use the existing provider-free bounds fixture to validate shape. No Grok bound is acquired until a registered route declares it. |
| Consumer effect/retry posture plus enforced no-replay, reconnect, teardown posture | Grok fixtures prove joined ACP process cleanup and current session cancellation. They do not bind an effect declaration or registered-call retry posture. | Missing for consumer tools; current ACP process cleanup is not registered-tool teardown proof. | Contract063 declaration posture and **Transport And Reconnect**; `RegisteredToolEnforcedPosture` and `assert_registered_tool_conformance`. | Preserve the existing no-replay statement as a Contract063 requirement. Acquire no provider value under this read-only pass. |
| Source identity/freshness for Contract061 projection | Current Grok projection sources are adapter/prepared and active-session sources for existing rows. No registered capability source exists. | Missing. | Contract061 **Snapshot Identity And Replacement** and Contract063 projection amendment; `RegisteredToolSource` plus `an_unqualified_registered_capability_is_never_supported_by_inference`. | Keep the registered-capability projection unqualified/unavailable. Do not add a Grok row or source identity. |
| Exact selection binding: snapshot reference, selected IDs, registration revision, schema digests, carrier, protocol, effective bounds | No Grok selection or prepared facade accepts a registered-tool preparation. `grok` has no `RegisteredTool*` reference. | Missing. | Contract063 **Selection And Dispatch Context**; `fixture_selection`, `RegisteredToolSelection::new`, and `project_registered_capability`. | Use the provider-free selection fixture as the future adapter handoff shape; no route wiring is authorized here. |

### Consumer call

| Contract063 field or invariant | Exact Grok evidence | Gap | Authoritative source / provider-free route | Smallest evidence action under current authority |
| --- | --- | --- | --- | --- |
| Trusted caller/admission identity: process incarnation, workspace/task generations, admitted task/session/attempt, live revocation source | Current Grok `SessionResumeBinding` binds provider/session and route context, not a Desktop admission binding. | Missing. | Contract063 **Trusted Admission Port**; `ConsumerAdmissionBinding`, `fixture_admission`, and `revocation_before_dispatch_never_reaches_the_dispatcher`. | Run/retain only the provider-free admission assertions. Do not create a provider or consumer admission identity from a model or provider session id. |
| Execution host, configured instance, operation scope, and turn binding | Current Grok plan/session binds an execution host, instance, route, model, resource, and runtime turn, but no registered lease. | Missing as a registered call binding. | Contract063 **Contracted Architecture** and `RegisteredToolOpenRequest`; `RegisteredToolHarness::open`. | Record existing ACP plan identity as non-transferable. A future adapter specimen must show one exact registered open request. |
| Server identity/revision, namespaced tool identity, execution kind, and selected schema digests | No consumer server/tool identity reaches `session/new`, `session/prompt`, or Grok dispatch. | Missing. | Contract063 call binding; `ValidatedRegisteredToolBinding` and `RegisteredToolCallRequest` in the provider-free lifecycle assertions. | No exact-route action is permitted now. Static evidence can only close an absence; it cannot create a call. |
| Unique call id and per-call deadline | Grok ACP JSON-RPC ids correlate ACP requests; `toolCallId` appears only in provider-owned activity/permission fixtures. | Missing as a consumer call id plus deadline bound. | Contract063 **Transport And Reconnect** and **Additive API And State Machine**; `request("call-1", ...)` and `one_call_settles_exactly_once`. | Keep ACP request ids and provider `toolCallId` separate. Use the provider-free call fixture for the required correlation shape. |
| Cancellation state bound to that exact call and lease | Current Grok cancellation is session/turn scoped and uses `session/cancel`; no consumer-call cancellation handle exists. | Missing. | Contract063 dispatch context; `RegisteredToolCancellation` and `a_closed_lease_is_not_reusable`. | No live cancellation probe. The provider-free lease/cancellation assertions are the smallest safe check. |
| Transport generation and exact registered-tool protocol version | No registered lease or transport generation is created by Grok; ACP v1 is only the route protocol. | Missing. | Contract063 **Transport And Reconnect**; `ValidatedRegisteredToolBinding` and `selection_rejects_undeclared_transport_and_version`. | Record route ACP v1 only. Defer generation evidence to a future registered adapter fixture. |
| Bounded argument payload and first-tranche concurrency | Grok bounds current prompt/output/permission payloads, not consumer tool arguments; current adapter has no registered dispatcher. | Missing. | Contract063 **Results, Errors, Concurrency, And Teardown**; `oversized_argument_fails_before_dispatch` and `one_outstanding_call_per_lease`. | Use the fake dispatcher/virtual clock route only; no provider arguments or live turn. |

### Consumer result and terminal truth

| Contract063 field or invariant | Exact Grok evidence | Gap | Authoritative source / provider-free route | Smallest evidence action under current authority |
| --- | --- | --- | --- | --- |
| Exactly one result correlated to the same server/tool/kind/call/operation binding | Grok `tool_call_update` is a notification with provider display content; no consumer result response is captured. | Missing. | Contract063 **Results, Errors, Concurrency, And Teardown**; `one_call_settles_exactly_once` and duplicate-correlation assertion. | Keep provider activity out of result evidence. The provider-free echo dispatcher is sufficient to check the shared exactly-once rule. |
| Result media type, bounded bytes, output schema revision, and schema digest | No registered tool result or output schema is in any exact Grok fixture. | Missing. | Contract063 result bounds and schema authority; `RegisteredToolResult::new`, `fixture_media_type`, `digest`, and `oversized_result_is_rejected`. | Static-search for a frozen result schema only. Do not use ACP display text or hash it as a result schema. |
| Result status and typed known/unknown execution disposition | Grok has ACP stop reasons and runtime failures, but none are tied to a consumer tool outcome. | Missing. | Contract063 `RegisteredToolOutcome` and execution disposition; `server_failure_stays_typed_and_correlated` and `a_call_that_outruns_its_deadline_is_unknown`. | Provider-free dispatcher cases can prove the disposition mapping. No Grok status is inferred from `end_turn`, `cancelled`, or process disconnect. |
| Consumer Deny, provider rejection, cancellation, timeout, abandonment, server failure, invalid result, transport loss, unknown outcome, and stale/duplicate/foreign/post-terminal failures | Current Grok fixtures cover ACP permission rejection/cancellation and transport/malformed failures, but not registered-tool outcomes. | Missing as a complete consumer-result vocabulary. | Contract063 safe-failure list; `RegisteredToolFailureKind` plus lifecycle/adversarial/integrity assertions. | Preserve current provider failure classifications; run no provider action. The shared testkit is the smallest route for the typed failure set. |
| Live `BeforeDelivery` admission verdict and honest result after revocation | No registered result-delivery point exists in Grok. | Missing. | Contract063 **Trusted Admission Port**; `revocation_before_delivery_reports_honest_execution` and `progress_rechecks_live_revocation`. | Use the scripted admission port with a fake dispatcher; do not probe a real Grok operation. |
| Completion gate, outstanding-call count, cleanup cause, joined teardown, and visible cleanup failure | Grok has route-local session/process cleanup and `CleanupOutcome`; no registered bridge lease or completion gate. | Missing for the consumer-tool profile. | Contract063 lifecycle and `RegisteredToolCompletionState`; `terminal_barrier_and_close_are_joined` and `failed_cleanup_retains_ownership`. | Provider-free local host tests are sufficient for this shared field set. No Grok process or server is started. |

### Consumer progress

| Contract063 field or invariant | Exact Grok evidence | Gap | Authoritative source / provider-free route | Smallest evidence action under current authority |
| --- | --- | --- | --- | --- |
| Non-terminal progress payload, same server/tool/kind/call/task/session/turn/attempt/transport binding | ACP `session/update` carries provider thought/plan/tool display activity. No consumer progress channel exists. | Missing. | Contract063 **Progress And Partial Notifications**; `RegisteredToolProgress`, `RegisteredToolProgressSink`, and `progress_admits_only_forward_sequences`. | Keep `ProviderToolDisplay` activity separate. The provider-free progress sink is the only permitted current probe. |
| Per-call monotonic sequence and positive payload/queue bounds | Grok event sequencing and permission callback capacity are not Contract063 progress sequencing. | Missing. | Contract063 first-tranche progress limits; `progress_queue_is_positively_bounded` and `concurrent_publishes_keep_exact_ordering`. | Use the fake clock/host queue assertions; acquire no provider progress. |
| Live admission, duplicate/regressive/foreign/stale-generation rejection | No Grok consumer progress admission point. | Missing. | Contract063 progress admission rule; `revocation_between_dispatch_and_progress_is_linearized`, `a_retained_sink_fails_after_the_call_settles`, and race assertions. | Provider-free race tests are the smallest action; no real callback or transport probe. |
| Post-cancel/post-terminal/post-close refusal and reconnect no-replay of missed progress | Grok cancels the ACP session/turn and drops route-local callbacks, but no consumer progress is admitted or replayed. | Missing. | Contract063 **Progress And Partial Notifications** and **Transport And Reconnect**; lifecycle/integrity assertions. | Record the current absence. A future exact route specimen must show refusal and no replay under the selected generation. |

### Permission

| Contract063 field or invariant | Exact Grok evidence | Gap | Authoritative source / provider-free route | Smallest evidence action under current authority |
| --- | --- | --- | --- | --- |
| One exact Allow and one exact route-supported Deny for a registered call | Current fixture `consumer_permission_exchange_answers_each_one_shot_option_and_hides_persistent_choices` proves `allow_once`/`reject_once` for a provider-owned ACP permission request. | Partial only. The options are not attached to a registered server/tool/call. | Contract063 **Tool Kinds And Permission Paths**; Grok `Scenario::Permission` fixture and the named permission exchange test. | Retain this as provider-owned one-shot evidence. Do not promote it to registered-tool permission strength. |
| Permission identity bound to server/tool/kind/call, lease generation, task/session/turn/attempt, deadline, and live admission | Current callback carries provider request id, `toolCallId`, callback id, turn, and deadline; it lacks registered identity, schema/revision, lease, and admission binding. | Missing. | Contract063 dispatch context/admission; `RegisteredToolCall`, `ConsumerAdmissionBinding`, and provider-free revocation assertions. | No provider permission probe. The future fake route must issue permission against a kernel-issued call token. |
| Provider-visible Allow/Deny result without persistent policy | Current `respond_permission` sends one selected ACP option; persistent `allow_always`/`reject_always` choices are hidden, and consumer failure maps to `reject_once`. | Existing provider-owned behavior; missing consumer-call mapping. | Contract063 **Allow/Deny** rule; `consumer_permission_exchange_answers_each_one_shot_option_and_hides_persistent_choices` and `consumer_permission_abandonment_closes_the_exchange_without_granting_access`. | Leave current one-shot behavior unchanged. No flag, default, or provider permission mode change. |
| Cancellation/timeout while permission is pending and no simulated approval | Current fixture proves provider permission observation/cancel, callback abandonment, timeout, and malformed rejection. | Missing only for a registered call; no consumer result is settled. | Contract063 permission and lifecycle rules; `consumer_permission_timeout_abandons_the_bounded_exchange`, `consumer_permission_malformed_request_fails_closed`, and `permission_is_observed_and_cancelled_without_ambient_approval`. | Use existing provider-free permission cases as a negative boundary. Do not start a live turn. |
| Permission result separate from registered-tool result | Current permission response can complete an ACP prompt; it never yields a `RegisteredToolOutcome`. | Missing. | Contract063 **Tool Kinds And Permission Paths** and **Results**; `RegisteredToolOutcome` plus the Grok provider-permission tests. | Keep the two channels separate in the capsule and future adapter. |

### Version and qualification fields

| Contract063 version field | Exact Grok evidence | Gap | Authoritative source / provider-free route | Smallest evidence action under current authority |
| --- | --- | --- | --- | --- |
| Exact executable/agent version and behavior revision | Frozen installed identity/compatibility records qualify `0.2.114..0.2.117` and `1.0.4..1.0.5` for existing ACP behavior revisions. | No consumer-tool behavior revision is qualified for any segment. | Contract029 version rule; exact Grok fixtures and `qualified_behavior_segments_execute`. | Keep the current route segments. Do not extend their behavior revision to MCP registration. |
| ACP wire protocol version | Exact `1.0.4`/`1.0.5` fixtures and route code use ACP `protocolVersion: 1`; `initialize` rejects another value. | Present for base ACP, not proof of a registered-tool profile. | Exact fixture plus `AcpConnection::initialize`; provider-free `qualified_behavior_segments_execute`. | No new action; retain ACP v1 as a route fact only. |
| Registered-tool protocol version | No `RegisteredToolProtocolVersion` or registered-tool protocol namespace occurs in the Grok adapter or fixtures. | Missing. | Contract063 explicit nonempty subset/no `latest`; `fixture_protocol_version` and `selection_rejects_undeclared_transport_and_version`. | Do not invent a version. A future route specimen must name one exact version string in both selection and evidence. |
| Server revision and schema revision/digest | No registered server/tool/schema records are frozen. | Missing. | Contract063 registry schema fields; provider-free snapshot/declaration fixtures. | Static artifact search only; no digest of unbounded provider data. |
| Transport profile and transport generation | `acp-v1-stdio` is the current Grok route transport. No private MCP attachment or registered callback generation is observed. | Registered profile missing. | Contract063 transport/reconnect; provider-free `RegisteredToolTransportSupport` and `ValidatedRegisteredToolBinding`. | Keep private MCP and host-mediated registered profiles withheld for Grok. |
| Contract061 qualified version-segment identity for registered-capability rows | Current main projection requires route qualification and exact protocol version; Grok supplies neither. | Missing; rows must stay unqualified/unavailable. | Contract061 **Registered Capability Projection**; `every_row_names_its_exact_route_and_qualified_version_segment` and `an_unqualified_registered_capability_is_never_supported_by_inference`. | Do not emit a Grok registered-capability projection row. |

### Acquisition boundary

The smallest action permitted by this preparation authority is a no-credential,
no-authentication static comparison of the frozen `1.0.4`/`1.0.5` artifacts,
the already captured empty-MCP ACP handshakes, the vendor docs, and the
provider-free Contract063 routes named above. That action can establish
absence or leave a field unknown. It cannot establish that a non-empty
`mcpServers` list is accepted, connected, discovered, called, or returned.

The smallest future exact-route acquisition, if the operator separately
authorizes credential use and a provider turn, is one exact installed version
at a time with one disposable non-mutating stdio MCP server: initialize and
negotiate the named registered-tool protocol, open with one non-empty server,
capture readiness and one tool schema, issue one bounded call, capture one
result plus progress/permission/cancellation behavior, then disconnect and
prove stale-callback rejection and joined cleanup. `1.0.4` and `1.0.5` must
remain separate evidence segments because their package identities and
behavior revisions are distinct. That probe is outside this preparation and
was not run.

## Exact Findings

1. The common protocol surface exists in ACP v1: a client can describe an
   MCP server at session setup, and an Agent can report tool calls and request
   one-shot permission. This is a protocol shape, not a Grok qualification.
2. Grok has a native, configuration-owned MCP subsystem with tool discovery
   and invocation. Its authority is Grok configuration and/or plugin
   discovery, not a proven Swallowtail operation-scoped registration lease.
3. The official Grok ACP corpus shows only empty `mcpServers` setup. The exact
   Swallowtail route corpus also records empty-MCP setup and leaves
   `mcpCapabilities` unmapped.
4. No permitted source proves all required Card118 stages for Grok consumer
   tools: server admission, namespaced schema/digest binding, exact operation
   and turn binding, bounded tool-call/result correlation, Allow/Deny outcome,
   cancellation and unknown-outcome handling, reconnect behavior, and joined
   teardown.
5. Provider-owned tool activity and the optional one-shot Grok permission
   exchange must not be relabeled as consumer-tool registration or MCP result
   dispatch.

## Proposed Adapter Mapping For A Future Qualified Surface

This is a preparation note, not an implementation authorization. If a later
exact-route corpus proves client-supplied Grok ACP MCP, the route-local adapter
mapping should consume Batch B’s shared registration snapshot and bridge
lease, then preserve these distinctions:

- Swallowtail owns the namespaced registration, schema revision/digest,
  operation/session/turn/attempt binding, deadline, cancellation, result
  correlation, and teardown evidence.
- Grok owns ACP translation and provider-native tool identity. Native Grok
  tools, consumer-registered MCP tools, and provider-owned tool activity remain
  separate identities.
- The existing one-shot `allow_once` / `reject_once` permission exchange is
  not a registered-tool result channel. Its default reject/cancel behavior
  remains unchanged.
- The current empty `mcpServers` emission remains unchanged until the exact
  surface gate is accepted. Cards114-115 are already in the current baseline.
  No empty-list-to-support flag, public API, route matrix, or baseline change
  is justified by this corpus.

## Conditional Alternative If ACP Consumer Tools Are Impossible

The frozen artifacts do not yet prove impossibility: ACP permits a client-
supplied `mcpServers` list, while the exact Grok corpus shows only the empty
case. Therefore no alternative is currently qualified or supported.

If an exact installed-version probe later proves that Grok rejects or ignores
client-supplied ACP MCP servers, the exact supported alternative to evaluate is
a separately qualified, configuration-owned native Grok MCP mediation route.
That is a different route authority from `grok-build.acp`; the current ACP
route remains withheld and is not silently repurposed. The minimum mapping is:

- Contract061 emits a registered-capability row only for the new exact native
  route and its qualified version segment; otherwise the row stays
  `Unqualified`/unavailable.
- Contract062 continues to transport the selected bundle and opaque bounded
  route/config references; native Grok configuration does not become
  registration authority by inference.
- Contract063 receives a native-route snapshot with exact server/revision,
  namespaced declarations and schema digests, transport/protocol subset,
  service and reference requirements, bounds, permission/progress posture,
  and reconnect/teardown evidence. Swallowtail still owns the admission,
  bridge lease, call/result correlation, and safe failure boundary.
- A new g05.035 follow-up card acquires that exact native-route corpus and
  implements the adapter only after the same provider-free and exact-route
  gates pass. Card118 itself records the ACP unsupported disposition and does
  not change runtime.

### Single Chatterbox decision question

If the exact route probe proves ACP consumer-tool registration impossible,
should g05.035 accept a separately qualified, configuration-owned native Grok
MCP mediation route with a new exact route/version segment and follow-up card,
or close the full-MCP goal as unsupported for `grok-build.acp`?

## Disposition

Evidence stop for Card118 preparation. The corpus is sufficient to preserve
the current withheld posture and to define the missing proof, but insufficient
to qualify Grok consumer-tool/MCP registration or to authorize runtime edits.
Cards114-115 are merged in the current baseline; runtime remains gated on the
exact Grok route evidence and the route gates named by g05.035.

The next evidence tranche would need a deterministic exact-version, provider-
free protocol specimen or an explicitly authorized exact-route probe that
proves non-empty session MCP admission, server readiness, tool discovery,
tool-call/result exchange, permission/cancellation outcomes, and joined
cleanup. Absence of that specimen supports an explicit unsupported disposition
later; it does not change the released claim in this preparation.

## Validation And Non-Actions

- `git diff --check` is the applicable formatting check for this document.
- No provider/live credentials, runtime edits, public API baseline update,
  route/feature claim change, tag, release, consumer mutation, or competing
  producer dispatch was performed.
- The research front door was additively updated during coordinator closeout to
  link Research 289; roadmap/card status remains untouched because the
  manifest reserves that shared surface.
