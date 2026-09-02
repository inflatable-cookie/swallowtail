# 279 Claude Agent ACP Capability Census And Tranche Selection

Status: complete; evidence-only; no production change
Owner: Tom
Date: 2026-09-02
Route: `claude-agent.acp`
Card: g05.022 card 054

## Question

What does the qualified Claude Agent ACP bridge expose across protocol,
admission, effective state, observation, lifecycle, authority, facade, and
consumer projection, and what is the largest adapter-only expansion that can
be delivered without a shared contract or public-API decision?

## Method And Boundary

The census was re-derived from current production source, adapter tests, the
two ACP corpus generations, and the current route matrices at worker HEAD
`998807d7eeefcd8f91d4b272b8421f8b573d5423`. No provider process, prompt,
login, authenticated session, live probe, host mutation, or consumer
repository was used.

The version boundary is the already qualified semantic range
`0.53.0..=0.73.0`, excluding unpublished `0.58.0`. `0.74.0` remains
unverified newer. Native `claude-agent.sdk`, Claude Code headless, and
Claude Code response-only are separate routes and do not contribute evidence
to this census.

Primary evidence:

- qualified 0.73 corpus: `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.73.0/protocol.json:2-127`
- qualified portable corpus: `crates/swallowtail-protocol-acp/tests/fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/protocol.json:1-112`
- lifecycle corpus: `.../lifecycle-corpus.json:14-46`
- continuity corpus: `.../session-continuity-corpus.json:10-48`
- route claim: `crates/swallowtail-adapter-claude-agent/src/selection.rs:11-147`
- prepared capabilities: `src/prepared/instance.rs:45-119`
- prepared plans: `src/prepared_profile/plan.rs:79-164`
- open/configuration path: `src/driver/access/open.rs:8-165`, `src/driver/config.rs:37-213`
- ACP wire setup and methods: `src/connection.rs:94-211`
- callbacks and update admission: `src/connection/dispatch.rs:57-190,192-267,283-353`
- active session facade and serialized turns: `src/driver/session.rs:84-215`
- activity projection: `src/acp_activity.rs:48-178,247-268`
- existing route projection and exact effort acknowledgement:
  `src/consumer_route_projection/open.rs:90-173`,
  `src/consumer_route_projection/builder/control.rs:73-112`
- provider-free projection oracles:
  `tests/consumer_route_projection/acknowledgement.rs:14-250`

The current route matrix agrees with this source result. The shared matrix is
evidence only and was not edited.

## State Vocabulary

Each row has the same ten dimensions. `Yes` means the exact current bridge
and adapter path proves the dimension. `Partial` means only a bounded subset
is implemented. `No` means no supported path. `Advertised` means the bridge
advertises a capability that this adapter does not select or expose.

| Dimension | Meaning |
| --- | --- |
| `P` | Protocol or qualified bridge presence |
| `Parse` | Swallowtail wire parsing or dispatch |
| `Plan` | Prepared-plan admission |
| `Ack` | Effective provider acknowledgement, not transport success |
| `Obs` | Active-session or turn observation |
| `Life` | Session/turn/process lifecycle ownership |
| `Auth` | Host/process or callback authority bound to the operation |
| `Facade` | Current public prepared or active facade |
| `Projection` | Current consumer-facing row/activity/callback projection |
| `Withheld` | Exact reason a broader claim is not emitted |

## Complete Capability Census

This table covers every capability named by card 054. The rows are the whole
partition; there is no filter or exception list hidden outside it.

| Capability | `P` | `Parse` | `Plan` | `Ack` | `Obs` | `Life` | `Auth` | `Facade` | `Projection` | `Withheld` |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ACP v1 initialize and identity | Yes: `initialize`, protocol 1 | Yes: protocol, exact agent name/version, auth shape, lifecycle capabilities | Yes: exact interface binding | Protocol version exact; feature advertisements are not effect acknowledgements | No | Owned child process begins here | `Process` plus explicit environment; `AmbientHost` | `ClaudeAgentAcpDriver`, prepared evidence | Prepared operation evidence | None |
| New session and text prompt | Yes: `session/new`, `session/prompt` | Yes: session id, text content, response | Yes: `InteractiveSession` or `StructuredRun`, exact model route and resource | Model is confirmed after `session/set_config_option` on 0.54+; prompt response is terminal truth | Yes: ordered activity and terminal streams | One provider session per owned process; one active turn | Cwd/resource lease and provider process are bound, not sandboxed | `prepare_session`, `prepare_run`, `open_session`, `start_turn` | Interactive/structured activity, provider tool lifecycle, final outcome | None |
| Read-only interactive tools | Yes: `Read`, `Glob`, `Grep`, `fs/read_text_file` | Yes: text read callback, locator and byte/line bounds | Yes: `ResourceAccess::Read`, filesystem representation, `WorkingResourceIo` | Not applicable | Yes: provider tool updates and callback requests | Turn and session cleanup joined | Bounded read callback only; provider process remains ambient | Interactive session `start_turn` | Provider-display activity and read callback exchange | No broader filesystem claim |
| Read-write interactive tools | Upstream `Edit`, `Write`, `acceptEdits` remain advertised | Partial: connection can construct RW tool set and dispatch no write callback | No: interactive plan fixes `ResourceAccess::Read`; attachment validation rejects RW | `session/set_mode` checks an empty response only; no effective mode value | Provider tool activity exists only on admitted operation shapes | Structured run owns its internal session; interactive RW is absent | No interactive RW admission or host write mediation | RW exists on `prepare_run`, not `prepare_session` | Structured-run bounded write row only; no RW interactive row | New access policy/input and effective mode boundary require integration review |
| One-shot permission requests | Yes: `session/request_permission` | Yes: tool id, bounded options, exact callback correlation | Conditional: `ConsumerMediated` opt-in; default reject path always admitted | Consumer selection is exact; default selects `reject_once` then cancels | Yes: callback or provider-request observation | Bound to one active turn and abandoned on close | Consumer callback or fail-closed rejection; no global authority | `ClaudeAgentPermissionHandling`, `CallbackExchange` | Permission exchange row/callback; provider request is separate from tool completion | None within one-shot subset |
| Persistent permission choices | Yes: `allow_always`, `reject_always` are in bridge source | Skipped by `permission.rs`; never accepted into callback options | No | No | No | No | No persistent consumer or provider authority is bound | None | No persistent-choice row | Contract 015 forbids inferring persistence from a one-shot choice |
| `session/set_mode` / `acceptEdits` | Yes and used by RW structured setup | Yes: request and empty-result shape | Conditional: structured run with RW resource | Transport response only; empty object is not effective mode observation | No current-mode retention from this response | Session-start setup; later close/join is owned | Provider mode mutation is performed by the bridge, not consumer control | Private open path only | No active mode acknowledgement row | Effective mode and reusable session control are unproved |
| `session/set_config_option` / model | Yes: `configId = model` | Yes: option shape and exact current value | Yes on 0.54+; baseline uses legacy exact model response | Exact selected model is confirmed; current code discards option domain | No negotiated model-options snapshot on Claude handle | Session-start only | Provider config response, no model catalogue authority | Caller model selection in session/run input | Prepared `control.model-selection`; no active model observation | Candidate C1 below retains only this exact bounded observation |
| `session/set_config_option` / effort | Yes: `configId = effort`, `thought_level` | Yes: bounded values and exact current value | Yes: six qualified reasoning values on supported revisions | Exact match is effective; exact admitted difference is rejected | Yes only through existing projected open acknowledgement | Session-start only | Provider acknowledgement is observation, not reusable mutation authority | `SessionOptions`/run reasoning input and additive projected open | `feature.active-session-reasoning-ack` with effective/rejected state | Mid-session effort control is not exposed |
| `session/set_config_option` / Plan mode | Yes: `configId = mode`, `mode`, `plan` | Yes: option presence and exact current value | Yes on supported revisions | Exact setup confirmation is checked internally but not retained | Plan activity is observed; mode acknowledgement is not | Session-start only | Provider config mutation, not consumer runtime control | `SessionOptions::harness_mode` | Prepared pending Plan control; provider Plan activity | Active effective/rejected Plan state is withheld |
| Bash and terminal methods | Provider-adjacent upstream behavior is not a selected portable subset | No terminal callback, terminal capability, or Bash dispatch | No | No | No | Owned process lifecycle only | `Process` ownership is not terminal mediation or containment | None | No Bash/terminal row | Ambient cwd/process authority is not bounded; Contract 017/023 require a separate host proof |
| Client MCP servers and MCP identity | ACP wire has `mcpServers`; adapter always sends `[]` on new/load/resume | Empty list only; no server descriptors or callbacks | No configurable MCP input | No | No client-MCP observation | Binding remains exact for the selected empty value | No MCP server process or callback authority | No MCP input facade | No client-MCP row | Client MCP identity/control needs explicit binding, lifecycle, and process authority |
| Load session | Yes: `session/load`, `loadSession` | Yes: session id, bounded replay, response ordering | Yes: replay max 64 items/256 KiB | Exact bound/session response; no new config acknowledgement | Yes: replay before ready | New owned process, loaded provider session, joined cleanup | Exact resource/model/access binding; read-only session | `load_request`, `load_session` | Load lifecycle and replay activity | Negotiated model observation on load is not retained |
| Resume session | Yes: `session/resume`, `resume` | Yes: bound session and no-replay phase | Yes | Exact bound/session response; no config acknowledgement | Partial: post-response metadata only, no historical replay | New owned process attached to retained session; joined cleanup | Exact session/resource/access binding; empty MCP list | `resume_request`, `resume_session` | Resume lifecycle; no replay row | No re-declaration of reasoning/options; no cross-process reattachment claim |
| Session list | Advertised in `sessionCapabilities` | No list request or response parser | No | No | No | No | No catalogue/import authority | None | No catalogue row | Advertisement is not a prepared operation or import authority |
| Session fork | Advertised in `sessionCapabilities` | No fork request or response parser | No | No | No | No | No descendant-session authority | None | No fork row | Provider capability is not selected or exposed |
| Session close | Yes: `session/close` | Yes: empty-result validation | Yes on negotiated qualified lifecycle | Empty response is transport success only | No | Native provider close, stdin close, process/task join | Provider close plus owned runtime cleanup; not secure deletion | Handle `close`, management capability | Native-close lifecycle capability | None |
| Session delete | Yes: `session/delete` | Yes: empty-result validation | Yes: separate management plan, inactive asserted | Empty response plus cleanup; deletion strength is fixed by plan | No | Provider deletion and owned cleanup are separate | Provider-data deletion over exact binding; no secure-erasure authority | `prepare_delete_session` | Provider-session-delete management row | No archive/restore/list/import or stronger deletion claim |
| Images | ACP ecosystem supports richer prompt content, but current initialize/prompt path selects text only | No image block parser or transport | No | No | No | Turn lifecycle only | No attachment or media authority | `TurnRequest` text only | No image row | Attachments are rejected before provider work |
| Embedded context | ACP ecosystem names the content kind; current prompt is text-only | No embedded-context parser | No | No | No | Turn lifecycle only | No external context resource authority | No input facade | No embedded-context row | No bounded attachment contract was selected |
| `@` mentions | Provider UX concept, not selected ACP content in this bridge | No mention parser | No | No | No | Turn lifecycle only | No mention/resource resolution authority | No input facade | No mention row | Cannot infer from text or provider display |
| Slash commands and available commands | Yes: `available_commands_update` | Recognized as configuration/replay kind, content discarded | No command admission | No | No command observation | Session metadata only | No command execution authority | No command facade | No command projection | Command discovery and command execution are separate surfaces |
| Edit review | `Edit`/`Write` tools and `acceptEdits` mode are upstream evidence | Provider tool display is parsed; no review decision parser | Structured RW setup only | Mode response is transport-only | Tool lifecycle/display observed | Turn/session cleanup | Provider-owned tool plus bounded adapter write path; no consumer review authority | Structured run activity only | Diff/text provider-display activity | Public review/approve/reject control is absent |
| Prompt queueing | 0.72 source adds queued/cancelled ownership internals | Adapter admits one active turn and rejects another | No queue plan capability | No queue acknowledgement | No queue state | Serialized turn cleanup | No queued-turn consumer authority | `start_turn` only | No queue row | Provider internal queue behavior is not a consumer control |
| Steering | Steering metadata is present in later bridge source and deferred-input behavior is recorded | `_meta` is ignored; no scheduling implementation | No | No | No steering observation | Turn cancellation only | No provider message injection authority | `TurnHandle::schedule_harness_message` remains unsupported | No steering row | Metadata and deferral are not a callable steering contract |
| Usage | Yes: input/output/cache/total usage fields | Yes: exact cumulative invariant | Yes: `UsageReporting` | Terminal usage is provider evidence | Yes: usage observation | Turn terminal lifecycle | Provider-reported evidence only; no billed-cost authority | Terminal outcome | Usage evidence row | No billing or cost claim |
| Auth and subscription/login | Empty `authMethods` is required; API-key/local access profiles exist | Yes: access/profile and credential handling; no auth RPC | Yes: API-key credential or local unauthenticated posture | No login/authentication acknowledgement | No account observation | Credential lease is awaited and released | Credential service or inherited process environment; no token custody | Access profile and prepared evidence | Access posture only | No ACP authenticate, login/logout, subscription entitlement, gateway, or provider-switch claim |
| Subagent and async-task metadata | 0.71 adds capability and update names | Unknown active updates become bounded namespaced unknown activity; replay rejects unsupported kinds | No | No | Partial identity/lifecycle-only unknown observation | No child-session lifecycle | No child process/control authority | Activity stream only | Unknown identity/lifecycle activity; no topology row | Stable child identity, parentage, lifecycle, and control are not qualified |
| Provider tool/activity updates | Yes: message, thought, tool, plan, usage, metadata updates | Yes for selected stable kinds; raw fields excluded | Yes through activity capability | Prompt stop remains separate from tool status | Yes bounded ordered activity | Activity closes at terminal outcome | Provider-owned tool activity, not consumer tool execution | Event stream and terminal outcome | Provider-display assistant/reasoning/plan/tool lifecycle and unknown safe identity | No raw payload or consumer tool exchange |
| Plan display and replacement | Yes: ACP `plan` update | Yes: replacement snapshot and typed task entries | Conditional Plan mode setup | No provider mode acknowledgement in activity stream | Yes provider-display Plan activity | Turn-scoped activity | Display only, no Plan control | Event stream | Plan provider-display activity/task list | Display is not effective mode state |
| Form elicitation | Yes: `elicitation/create` form | Yes exact 1–4 question choice-and-Other subset | Active-turn callback path | Exact callback response | Yes callback request/response | Abandoned with turn/session close | Consumer chooses typed answer; richer forms fail closed | Callback exchange/user-input facade | Question exchange | Rich schemas, previews, arbitrary metadata, and non-choice forms are declined |

## Current State Transitions And Counterexamples

The current open path is:

```text
prepare exact version/model/resource/access
  -> start owned ACP process
  -> initialize and validate identity/auth/capabilities
  -> session/new
  -> confirm model
  -> optionally set+confirm effort and Plan config
  -> if structured RW, set_mode acceptEdits
  -> return bound session/run handle
  -> serialize one turn, dispatch callbacks/activity, await terminal
  -> close provider session, join process/tasks, release resource/credential
```

Load inserts bounded replay before the load response marks the handle ready.
Resume has no replay. Delete is a separate inactive-session management
operation. All paths retain exact instance, host, model, resource, access, and
provider-session binding.

The following counterexamples are load-bearing:

| Counterexample | Correct result |
| --- | --- |
| `set_mode` returns `{}` | Transport success only; do not publish effective mode |
| `set_config_option(model)` returns a valid model option list | It confirms the selected model; it is not a model catalogue |
| Requested effort differs from exact bounded current value | `Rejected` only through the existing projected open for an admitted known value; malformed/ambiguous values are `Runtime` with no contribution |
| Permission offers `allow_always` | Skip it; expose only one-shot choices, or default to `reject_once` and cancel |
| Same prepared and active projection source id | Fail before process/resource/credential work |
| Resume uses another session id or non-empty/unmatched binding | Fail without a usable handle; never mix session identity |
| Provider Bash uses the leased cwd | `AmbientHost` remains uncontained; cwd, tool policy, MCP empty list, and process ownership do not prove sandboxing |
| Provider emits a subagent update | Preserve only bounded unknown identity/lifecycle if the active decoder can do so; do not synthesize child topology or control |

## Delivery Selection

### Candidate C1 — Negotiated Model-Options Observation

This is the largest candidate that fits the existing adapter-only seam.

The orchestrator may compile one later implementation card for
`swallowtail-adapter-claude-agent` with this exact boundary:

1. After the existing `session/new` and exact model confirmation, parse one
   `configOptions` row with `id = "model"`, `type = "select"`,
   `category = "model"`, a bounded non-empty `currentValue`, and bounded
   unique option values whose set contains the current value. Optional display
   names remain bounded. The existing runtime type and maxima are
   `NegotiatedSessionModelOptions` in
   `crates/swallowtail-runtime/src/negotiated_session_options.rs:13-94`.
2. Retain the exact snapshot on the Claude interactive session handle through
   the existing `InteractiveSessionHandle::negotiated_model_options` seam
   (`crates/swallowtail-runtime/src/handles.rs:96-113`). Keep the existing
   `open_session` signature and lifecycle. The additive projected-open path
   may publish the snapshot using the established
   `feature.negotiated-model-options-observation` identity and
   `ActiveSessionObservation` source already proved by the Cline route.
3. Mark the row observation-only, post-open, not selectable, and tied to the
   active source. It is negotiated session evidence, not a model catalogue,
   selection authority, provider registry, or mid-session mutation handle.
4. Required missing `configOptions[id=model]` fails both public opens through
   the existing confirmation path. That is not a successful-open Absent
   snapshot: confirmation already requires the model entry. On the preserved
   open path, snapshot-detail malformation that still confirms `currentValue`
   (wrong category, duplicate option values, unbounded option, current not in
   advertised options) may remain absent to preserve existing behavior. On the
   projected path, that invalid snapshot-detail evidence must close the opened
   session and return the bounded runtime diagnostic without a contribution.
   Load and resume do not emit this row until their attachment responses have
   an equally exact parser and lifecycle proof.
5. The provider-free oracle must cover exact, missing required model entry,
   snapshot-detail malformation, duplicate option values, unbounded,
   current-not-in-options, source-id disagreement, projected cleanup,
   preserved-open compatibility, and the negative assertion that a negotiated
   snapshot is not a catalogue.

This proposal uses an existing runtime type, source kind, lifecycle, and
projection identity. It does not select a shared contract amendment or a new
shared public API. The worker does not implement it; implementation-card
compilation belongs to the orchestrator after the SDK lane and joint review.

### Deferred And Distinct Tranches

| Candidate | Why it is not C1 |
| --- | --- |
| Read-write interactive session | Current interactive preparation binds `ResourceAccess::Read` and `AmbientHost`; RW is realized only by the structured one-prompt path. A new interactive access input, write authority, mode acknowledgement, and consumer projection need joint review. |
| Session-scoped permission and mode control | Permission mediation is already a session policy applied to every turn, but there is no public control for changing it. `set_mode` is private setup and returns no effective mode value. |
| Mid-session model/effort controls | `set_config_option` is private setup; the active session facade exposes turns, cancellation, and close, not config mutation. A new control needs serialization, effective acknowledgement, failure, and lifecycle semantics. |
| Client MCP servers | The bridge wire accepts `mcpServers`, but this adapter fixes `[]`. Adding servers requires exact identity across new/load/resume, process ownership, callback boundaries, and consumer authority. |
| Bash/terminal | No selected terminal callback or host containment proof exists. Process ownership and cwd are lifecycle facts, not mediation or sandboxing. |
| Auth readiness and packaging | Auth methods, login/logout, subscription entitlement, gateways, and package-install policy are separate authority and release surfaces. |
| List/fork/archive/restore/import | Advertised capabilities do not supply prepared operations, binding, lifecycle, or deletion semantics. |
| Images/embedded context/@mentions/commands | Each needs a distinct typed input or command projection contract; text-only prompt and discarded metadata cannot stand in for it. |
| Queueing/steering/subagents | Current source records provider-internal deltas but exposes no bounded consumer control or stable child topology. |

No exact load-bearing blocker makes the tranche set empty. C1 is a useful,
bounded observation expansion; the larger preferred controls remain explicitly
deferred rather than being flattened into it.

## Result

Card 054 has a complete route/bridge partition. Current ACP support is
featureful but deliberately bounded: text interactive sessions, read-only
callbacks, structured RW one-shot runs, exact setup model/effort/Plan handling,
one-shot permission mediation, usage/activity, load/resume, close/delete, and
typed form elicitation. The only independent next expansion selected here is
exact negotiated model-options observation on the existing projected-open
seam. No claim, matrix, fixture, package pin, contract, public API, or
production behavior changed.

## Authority

- [Research 277](277-claude-subscription-dual-route-direction.md)
- [Research 272](272-claude-agent-acp-0-73-0-identity.md)
- [Contract 015](../contracts/015-acp-v1-negotiation-and-client-callbacks.md)
- [Contract 017](../contracts/017-provider-owned-session-load-replay-and-host-containment.md)
- [Contract 029](../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 038](../contracts/038-provider-session-management-and-consumer-thread-boundary.md)
- [Contract 041](../contracts/041-input-callback-and-provider-tool-admission.md)
- [Contract 047](../contracts/047-configured-provider-instance-catalogue.md)
- [g05.022](../roadmaps/g05/022-claude-agent-dual-route-parity.md)
- [card 054](../roadmaps/g05/batch-cards/054-claude-agent-acp-parity-census-and-delivery-gate.md)
