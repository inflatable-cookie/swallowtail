# 001 Two-Consumer Runtime Requirements

Status: promoted
Owner: Tom
Updated: 2026-07-19

## Question

What runtime mechanisms are genuinely shared by Nucleus interactive sessions
and Soundcheck structured runs, and which behavior must remain in each
consumer?

This inventory describes the consumer working trees inspected on 2026-07-19.
It is source evidence, not a dependency on either repository.

## Evidence Basis

Nucleus:

- `crates/nucleus-agent-protocol/src/live_runtime.rs`
- `crates/nucleus-agent-adapters/src/codex_runtime.rs`
- `crates/nucleus-agent-adapters/src/live_registry.rs`
- `crates/nucleus-server/src/local_codex_chat/runtime.rs`
- `docs/contracts/017-engine-host-authority-contract.md`

Soundcheck:

- `src-tauri/src/assistant_tagging.rs`
- `src-tauri/src/app_settings.rs`
- `src/lib/api.ts`

## Shared Requirement Matrix

| Concern | Nucleus evidence | Soundcheck evidence | Swallowtail finding |
| --- | --- | --- | --- |
| Adapter discovery and readiness | `CodexSessionRuntime::start_session` currently assumes `codex`; registry resolves a built-in adapter id. | `find_codex` searches configured and host paths; `detect_assistant_status` probes version and login. | Adapters expose discovery, version, auth/readiness, and failure evidence. The host supplies configuration and owns credential UX. |
| Model catalog | `AgentSessionRuntime::model_catalog` drives Codex `model/list` and normalizes reasoning choices. | `discover_codex_models` independently starts app-server and drives the same model list flow. | Model discovery is shared adapter mechanism. Host policy still selects defaults and permitted routes. |
| Model and reasoning selection | `AgentSessionStartRequest` and `AgentTurnRequest` carry model and effort, allowing per-turn change. | `run_codex_turn` receives model and effort for each bounded call. | Both execution shapes carry an explicit model route and provider-supported options. |
| Execution context | `LocalCodexChatSession::start` resolves project root and resource before passing a working directory. | `run_codex` creates a temporary scratch directory and may attach a screenshot path. | The execution host resolves working resources, scratch space, and attachments. A client path is not authority. |
| Process or connection lifecycle | `CodexLiveSession` owns a long-lived app-server child and kills it on drop. | `run_codex_turn` owns an ephemeral child, monitors it, and always joins or kills it. | Provider adapters own connection/process mechanics under execution-host authority and declare owned versus external runtime mode. |
| Deadlines | Codex requests and turns use separate fixed deadlines. | Research and repair/ranking/companion calls have separate fixed deadlines. | Request deadline is shared mechanism and must be explicit per operation; hard-coded provider defaults are adapter policy, not universal semantics. |
| Cancellation and interruption | The current live trait has no interrupt operation; dropping the session kills the whole process. Nucleus's harness contract requires explicit active-turn interruption. | Request-scoped cancellation uses an atomic flag, kills the active child, waits, and returns a distinct cancellation result. | Cancellation is scoped to run, turn, or session and is capability-led. Cleanup and terminal outcome must be observable. |
| Progress and events | The adapter parses assistant deltas, tool requests, and turn completion but the current host boundary returns only the final reply. | JSONL is translated into correlated safe progress phases and token usage; final structured payloads are deliberately omitted from progress. | Both shapes need ordered correlated events distinct from terminal outcomes. Raw provider payload exposure is never the default. |
| Tool and callback exchange | `AgentToolCallHandler` returns provider-visible text while Nucleus records tool receipts and retains authority. | No tool callback path is used. | Tool exchange is an interactive-session capability. Swallowtail carries declarations, calls, and responses; the host owns execution, authorization, and receipts. |
| Structured output | Not used by the current chat boundary. | `--output-schema` constrains provider output; Soundcheck then decodes, validates, repairs once, ranks, and applies domain rules. | Schema transport and structured result bytes are shared mechanisms. Schema content, domain validation, repair orchestration, and application remain consumer-owned. |
| Attachments | Current chat turns are text-only. | An optional screenshot is passed as an image input. | Attachments are capability-led host-resolved inputs, not assumed local filesystem paths. |
| Provider references and recovery | Nucleus persists provider thread id, resumes only when resource/toolset invariants match, and rejects a mismatched resumed id. | Every Codex call is ephemeral and retains no provider session. | Interactive sessions return opaque refs and explicit recovery outcomes. Structured runs need no resume unless an adapter declares it. Consumer stores decide persistence. |
| Diagnostics | Current adapter errors are provider-derived strings. | Failures include bounded stderr tails and product-facing progress errors. | Adapters produce structured safe diagnostics with opt-in internal detail. Raw stderr and provider errors require host redaction policy. |
| Correlation and concurrency | Conversation, session, thread, turn, tool-call, and receipt ids remain distinct. | A request id scopes cancellation and filters progress events. | Operations require stable host correlation plus opaque provider refs; ids from different scopes are never interchangeable. |
| Topology | Nucleus permits embedded, sidecar, remote authoritative, and remote worker hosts with separate execution/source/credential authority. | Current execution runs in the Tauri host via `spawn_blocking`, not in the renderer. | Swallowtail runs on the authorized execution host. It does not assume client-local binaries, credentials, or files. |

## Shape-Specific Requirements

### Interactive Session

- start a new session or resume an opaque provider session
- retain a live connection or owned process across turns
- send multiple turns with per-turn model configuration where supported
- emit deltas, item events, tool calls, usage, warnings, and terminal turn state
- exchange host-authorized tool or callback responses
- interrupt an active turn without necessarily destroying the session
- close the session and report cleanup outcome
- expose explicit recovery after disconnect, process exit, or host restart

Nucleus evidence: `AgentSessionRuntime`, `AgentLiveSession`,
`CodexLiveSession::send_turn`, `CodexAppServerRpc::wait_for_turn`, and
`LocalCodexChatSession::stored_session`.

### Structured Run

- accept one bounded request with model route, prompt/input, optional schema,
  and optional attachments
- emit safe correlated progress while work runs
- enforce an explicit deadline and request-scoped cancellation
- terminate and reap owned processes on completion, timeout, or cancellation
- return one structured terminal outcome with output and usage when available
- avoid session persistence unless the adapter declares a resumable run

Soundcheck evidence: `propose_assistant_taxonomy`, `run_codex`,
`run_codex_turn`, `parse_codex_progress_line`, and the request-id/AbortSignal
bridge in `src/lib/api.ts`.

## Consumer-Owned Policy

Nucleus keeps:

- conversation, project, resource, task, goal, memory, and review records
- developer instructions and projected tool declarations
- tool authorization, execution, receipts, and product consequences
- session persistence rules and UI behavior

Soundcheck keeps:

- evidence collection and tagging prompts
- taxonomy and schema content
- proposal decoding, domain validation, repair, ranking, and companion logic
- proposal application and review UX
- product-specific progress wording

## Gaps For Card 007

1. Public async posture: both consumers offload blocking provider work, but the
   evidence does not choose async traits, pollable handles, or a sync core.
2. Host ports: process spawning, clock/deadline, temporary storage,
   attachments, credentials, redaction, and event delivery need exact ownership
   and test seams.
3. Event delivery: ordering, buffering, backpressure, replay, and terminal
   outcome separation remain unsettled.
4. Cancellation: run, turn, and session cancellation need distinct semantics
   and cleanup guarantees.
5. Structured output: Swallowtail can carry schemas and result bytes, but
   generic schema validation ownership remains open.
6. Attachments: portable handles versus host paths or streams remain open.
7. Runtime instances: adapter registry, configured instance identity, owned
   versus external process mode, and per-instance capabilities need a contract.
8. Diagnostics: internal-detail access and redaction hooks need an explicit
   host policy boundary.

## Promotion

- realized consumer shapes: `docs/architecture/consumer-runtime-evidence.md`
- settled ownership rules: Contract 004 Runtime Ownership Boundary
- remaining decisions: g01 cards 007 and 008
