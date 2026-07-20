# Nucleus Adoption Handoff

Status: prepared
Owner: Nucleus downstream
Source milestone: 008 Nucleus Interactive-Session Readiness
Updated: 2026-07-19

## Outcome

Nucleus can replace the live Codex process and JSONL-RPC mechanics behind its
existing `AgentSessionRuntime` facade without moving product authority,
persistence, or workflow state into Swallowtail. This is a downstream plan. It
does not claim adoption has happened.

The first slice is deliberately local and embedded. A project resource whose
authoritative execution host is not the Nucleus process host must fail before
provider work until Nucleus can invoke the integration on that host. Giving a
local process service a remote host id is not remote execution.

## Replacement Boundary

Add one Nucleus-owned `swallowtail_codex` module in
`nucleus-agent-adapters`. It implements the existing
`nucleus_agent_protocol::AgentSessionRuntime` and `AgentLiveSession` traits,
then registers under the existing `codex-app-server` adapter id. The server and
desktop continue to call the current facade.

Replace only these mechanics from
`nucleus-agent-adapters/src/codex_runtime.rs`:

- Codex executable launch and app-server initialization
- model-list RPC and provider metadata parsing
- session open, turn start, streamed output, callback transport, interruption,
  terminal outcome, and child cleanup
- provider session and turn reference handling

Keep these Nucleus-owned:

- `local_codex_chat` request, reply, history, and stored product records
- project and resource selection, authority-host policy, and transient-project
  fallback
- developer instructions and the `task_ledger` and `task_workflow` schemas
- task/goal inspection, mutation, execution admission, review, and receipts
- canonical conversation and turn ids, transcript persistence, and UI DTOs
- `codex_supervision`, command policy, host authority, and all task, goal,
  memory, SCM, and orchestration records

Do not replace `nucleus-agent-protocol` or make Swallowtail depend on it. The
Nucleus integration module is the translation layer between the two APIs.

## Current-Seam Mapping

| Current Nucleus mechanism | Swallowtail mechanism | Remaining owner |
| --- | --- | --- |
| `AgentAdapterRegistry::with_builtin_adapters` | select the legacy or Swallowtail implementation for the same adapter id at compile time | Nucleus registry |
| `AgentSessionRuntime::model_catalog` | preflight plus `CodexAppServerDriver::list_models` | Nucleus maps `ModelCatalogEntry` into `AgentModelOption` |
| `AgentSessionRuntime::start_session` | `preflight`, `OpenSessionRequest`, and `InteractiveSessionDriver::open_session` | Nucleus facade and session cache |
| `AgentLiveSession::send_turn` | `TurnRequest` and `InteractiveSessionHandle::start_turn` | Nucleus blocking facade waits and projects one reply |
| `AgentSessionStartRequest::working_directory` | server-selected `WorkingResourceRef` resolved by a host service | Nucleus project/resource authority; no renderer path crosses the boundary |
| model and reasoning strings | exact configured instance, model route, model id, and `SessionOptions::reasoning_mode` | Nucleus selection and persistence |
| developer instructions | redacted `OperationContent` in `SessionOptions` | Nucleus authors the instructions |
| dynamic JSON tool specs | bounded `ToolDeclaration` values | Nucleus owns names, descriptions, schemas, and meaning |
| adapter tool-call handler | `CallbackExchange`, `CallbackRequest`, and `CallbackResponse` | Nucleus executes the declared portal and creates receipts |
| provider message deltas | ordered `RuntimeEvent` values | Nucleus aggregates the assistant reply and may later project progress |
| provider thread id | opaque `SessionRef` and `SessionResumeBinding` | Nucleus stores only the provider value it already owns |
| provider turn id | opaque `TurnRef` | Nucleus keeps it separate from its canonical turn id |
| drop-time child kill | explicit session/turn close and joined cleanup | Swallowtail mechanism; Nucleus owns when to close |

The existing `send_agent_chat_message` Tauri command, `LocalCodexChatService`,
and `LocalCodexChatSession` shapes do not need to change for the first slice.
The integration may use a private `futures` executor inside the existing
blocking worker to drive the async handle, callback, event, and terminal
futures concurrently. Swallowtail does not provide or choose that executor.

## Identity Mapping

Keep every identity namespace explicit. Do not reuse one string merely because
the current facade exposes strings.

| Identity | Source and projection |
| --- | --- |
| Nucleus conversation id | stays the product conversation key |
| Nucleus canonical turn id | stays `turn:chat:<conversation>:<ordinal>` |
| Swallowtail request id | integration-local id derived from the operation, not persisted as the canonical turn |
| Swallowtail runtime session id | remains inside the live wrapper |
| provider session ref | projects to the existing stored `provider_thread_id` |
| Swallowtail runtime turn id | integration-local turn identity |
| provider turn ref | projects to `AgentTurnReply.turn_id` and existing `provider_turn_id` storage |
| callback id | projects to `AgentToolCall.call_id`; it is not a task or receipt id |
| Nucleus task and goal ids | remain inside tool arguments, results, and product receipts |
| Nucleus receipt ids | remain Nucleus records and never become runtime or provider refs |

Default diagnostics must keep `OperationContent`, callback payloads, provider
refs, instructions, and schemas redacted. Nucleus may persist its existing
sanitized messages, product receipts, and provider refs; it must not persist
raw Swallowtail event or provider envelopes.

## Host Wiring

The Nucleus integration should receive a Nucleus-owned host context rather than
constructing ambient authority inside the driver. For the initial desktop
slice it should:

1. take the selected resource id and authority-host ref from
   `project_resource_target`
2. reject any target not owned by the embedded desktop process host
3. map the product resource id to an opaque `WorkingResourceRef`; keep the
   canonical path inside the local `WorkingResourceService`
4. resolve the Codex executable and minimal saved-login environment under
   Nucleus host authority
5. construct `HostServices` with the exact `ExecutionHostId` used by the
   configured instance and preflight plan
6. register scoped task, time, process, working-resource, and diagnostic
   services; add only capabilities required by the selected operation
7. preflight the exact configured-instance revision, app-server role, model
   route, model, and session capabilities before launch

Normally allowlist `HOME`, `PATH`, optional `CODEX_HOME`, and the platform
temporary directory. Proxy or certificate variables require explicit Nucleus
configuration. Do not inherit the renderer environment or accept arbitrary
paths from a client request.

Resource-free chat may map its current host-home fallback to a dedicated
host-approved working-resource reference. That remains a Nucleus policy. It
does not make the home path part of the portable request.

Remote-authoritative projects are a later Nucleus host-routing slice. The same
preflight plan, configured instance, working-resource reference, and host
services must be constructed and executed on the authoritative worker or
server. Until that route exists, report an unsupported host placement rather
than falling back locally.

## Tool Callback Bridge

Convert the two current dynamic tool specs into `ToolDeclaration` values when
the session opens. When a turn yields a callback request:

1. require a declared `ToolCall`; reject extensions and unknown declarations
2. decode the bounded argument payload as JSON
3. call the existing `AgentToolCallHandler` with the callback id, runtime turn
   identity, tool name, and arguments
4. let `local_codex_chat` execute `task_ledger` or `task_workflow` under its
   existing policy and collect its existing receipts
5. return the handler text as a successful callback response, or a typed
   consumer failure without granting alternate execution authority
6. continue polling callbacks, ordered events, cancellation, and terminal
   outcome until exactly one terminal result exists

Swallowtail never calls Nucleus control APIs directly and never understands a
task, Goal, review note, mandate, or workflow receipt. Cancellation or deadline
abandonment must reject the waiting provider callback and prevent a late
response from executing product work.

## Session And Turn Projection

For each turn, aggregate safe output deltas in sequence order. On success,
project the provider turn ref and final assistant content into the current
`AgentTurnReply`. Map cancelled, timed-out, provider-failed, host-failed, and
runtime-failed terminal states to distinct internal diagnostics even though the
current facade returns `Result<_, String>`.

Do not collapse a terminal failure into an empty assistant response. Always
close the turn handle. Close the session handle when the Nucleus live wrapper
drops, and require joined cleanup rather than relying on process destruction.

The current facade has no progress or cancellation methods. The first slice may
consume events internally and retain current behavior. A later Nucleus-owned
facade extension can expose progressive events and explicit interruption; it
is not required to replace the Codex transport.

## Resume Constraint

Nucleus currently persists `provider_thread_id`, adapter id, provider-instance
id, resource id, model, reasoning, and turn count. Swallowtail additionally
binds resume to the exact configured instance, execution host, model route, and
model.

More importantly, current Codex app-server schema evidence does not permit
dynamic-tool redeclaration on `thread/resume`. Every Nucleus chat session opens
with tools. Therefore the first Swallowtail path must not resume a stored
tool-enabled provider thread. It should use Nucleus's existing transcript
migration context and open a fresh provider session.

This requires no durable schema change and keeps rollback trivial. Enable
Swallowtail resume later only when both conditions hold:

- provider schema evidence supports the required tool continuity safely
- Nucleus can reconstruct or store the complete `SessionResumeBinding` without
  confusing current resource placement with the host that created the session

Never resume from the provider thread id alone.

## Development Dependency And Versioning Gate

Sibling path dependencies are valid while Swallowtail and Nucleus are developed
together locally. Pin a version or immutable revision when either consumer
enters versioned distribution; that packaging step is not a development gate.

```toml
[dependencies]
swallowtail-adapter-codex = { path = "../../../swallowtail/crates/swallowtail-adapter-codex" }
swallowtail-core = { path = "../../../swallowtail/crates/swallowtail-core" }
swallowtail-host-local = { path = "../../../swallowtail/crates/swallowtail-host-local" }
swallowtail-runtime = { path = "../../../swallowtail/crates/swallowtail-runtime" }
```

Compile exactly one implementation for `codex-app-server`:

- register `SwallowtailCodexSessionRuntime` under the existing adapter id
- remove the superseded direct app-server wire implementation in the same lane

Do not register both under different ids and expose a permanent product
backend choice. The gate is temporary migration scaffolding.

## Downstream Sequence

1. Pin a verified Swallowtail revision and add the optional feature.
2. Add `swallowtail_codex` beside the current Codex runtime without changing
   `nucleus-agent-protocol`.
3. Add Nucleus-owned configured-instance, preflight, resource, process, task,
   time, and diagnostic adapters for the embedded host.
4. Route model catalog through the Swallowtail implementation under the
   feature; keep the returned Nucleus DTO unchanged.
5. Route session open and turn send through the wrapper. Use transcript
   migration rather than tool-enabled resume.
6. Bridge declared callbacks into the existing tool handler and prove existing
   task/workflow receipts are unchanged.
7. Run legacy and Swallowtail builds separately, then enable the feature for
   desktop manual acceptance.
8. Make the Swallowtail path the normal build only after the checklist passes.
9. Remove the legacy Codex launch, RPC, parsing, timeout, and cleanup code plus
   the temporary feature gate in one Nucleus-owned lane.
10. Plan remote host routing and progressive facade changes separately; neither
    blocks removal of the local legacy transport.

## Nucleus-Owned Validation

- the registry exposes exactly one `codex-app-server` implementation per build
- model ids, display names, descriptions, provider default, and reasoning
  choices remain unchanged at the desktop boundary
- saved ChatGPT/Codex login works with the allowlisted host environment
- resource-free chat still uses the current Nucleus host-home policy
- an attached local folder or repository runs without a renderer-supplied path
- a remote-authoritative resource fails before provider work in the local slice
- developer instructions and exactly `task_ledger` and `task_workflow` reach a
  newly opened session
- task creation, update, goal grouping, workflow inspection, authorized run,
  review feedback, and rework keep their current Nucleus receipts and state
- multiple callbacks in one turn remain ordered and exactly once
- callback failure, cancellation, deadline, and late response cannot duplicate
  product effects
- provider session, provider turn, runtime session, runtime turn, callback,
  task, and receipt identities stay distinct
- assistant output remains ordered; sentence spacing and UI rendering do not
  regress
- switching panels during a turn remains responsive
- success, provider failure, host failure, cancellation, and timeout each close
  the turn and join process work
- restarting with stored tool-enabled history opens a fresh provider session
  with transcript context instead of unsafe resume
- stored Nucleus session, turn, message, task, Goal, review, and receipt schemas
  do not change
- default logs and errors expose no prompt, callback payload, tool schema,
  provider envelope, credential, or raw filesystem path
- focused adapter, local-chat, persistence, desktop command, Effigy, and manual
  chat checks pass in both feature configurations

## Rollback

Before legacy removal, disable `swallowtail-codex` and rebuild. Existing
conversation history, provider refs, model settings, tasks, Goals, reviews, and
receipts require no migration or rollback.

After legacy removal, revert the downstream transport-switch/removal commit and
restore the last verified Swallowtail revision. If only a Swallowtail
regression is involved, pin the previous verified revision first.

No rollback path may silently resume a provider thread on another configured
instance or execution host.

## Removal Gate

Delete `nucleus-agent-adapters/src/codex_runtime.rs` process launch, RPC loop,
model parsing, callback envelope, timeout, and drop cleanup only after both
feature configurations compile and the Nucleus-owned checklist passes with the
Swallowtail feature enabled.

Do not delete `nucleus-agent-protocol`, the live registry, local-chat tools,
product persistence, `codex_supervision`, host authority, task/Goal/memory
logic, UI commands, or review workflows. Adoption replaces transport
mechanics, not Nucleus.
