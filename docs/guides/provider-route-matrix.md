# Provider Route Matrix

This is the integration front door for Swallowtail's 32 production routes.
Choose one row explicitly. Swallowtail does not select a provider, driver,
model, target, endpoint, credential, billing arrangement, execution host, or
fallback route.

The companion
[provider and feature CSV](provider-solution-feature-matrix.csv) groups
complementary routes where one public solution facade exists, then compares
runtime posture, version posture, and qualified feature coverage across all 32
routes. Composite-row notes name the branch that owns each route-specific
capability; the cells remain a solution-conversion scoreboard. Permission
exchange and question exchange are separate columns: authorization stays
provider-specific while losslessly representable questions use one typed
runtime contract.

Every row has two public paths:

1. the adapter-local prepared constructor and typed bound operation for normal
   integration
2. the listed low-level driver and its provider-neutral runtime role for
   advanced composition

Remote ACP is not an additional production route. It is an explicit transport that compatible
ACP adapters may compose instead of their stdio transport.

## Installed Harnesses

| Route | Crate and driver | Role and transport | Explicit target and access | Version axis | Prepared path | Low-level escape hatch |
| --- | --- | --- | --- | --- | --- | --- |
| `antigravity.catalogue` | `swallowtail-adapter-antigravity`; `swallowtail.antigravity.catalogue` | authenticated model catalogue; Antigravity models stdio | approved executable and environment; provider-supported personal Google subscription login owned by Antigravity | `antigravity-cli.release`; exact `1.1.9` plus permitted visible unverified-newer stable points | `prepare_antigravity(Catalogue)` → `prepare_catalogue` → `list_models` | `AntigravityCatalogueDriver`; `ModelCatalogDriver` |
| `antigravity.headless` | `swallowtail-adapter-antigravity`; `swallowtail.antigravity.headless` | explicit-model structured runs and exact-id restarted turn-scoped continuation; Antigravity stream JSON over one joined stdio child per run or turn | approved executable and environment; provider-supported personal Google subscription login; explicit ambient read or read-write authority for runs, optional provider sandbox, and ambient read-intent plan mode for continuation | `antigravity-cli.release`; exact `1.1.9` plus permitted visible unverified-newer stable points | `prepare_antigravity(Headless)` → `prepare_run` → `start_run`, or `prepare_antigravity(Continuation)` → `prepare_session` → `open_session`; later turns privately use only the exact observed conversation id | `AntigravityHeadlessDriver`; `StructuredRunDriver` and `InteractiveSessionDriver` |
| `codex.exec` | `swallowtail-adapter-codex`; `swallowtail.codex.exec` | structured run; structured CLI | approved executable and environment; caller-selected Codex profile plus matching evidence | `codex.cli`; ordered maintained/deprecated range with permitted unverified-newer stable points | `prepare_codex(StructuredExec)` → `prepare_structured_exec` → `start_run` | `CodexExecDriver`; `StructuredRunDriver` |
| `codex.app-server` | `swallowtail-adapter-codex`; `swallowtail.codex.app-server` | catalogue, interactive session with optional exact plan-mode selection, exact or session-scoped interrupted-turn reconciliation, optional settled reconcile then bounded load, and inactive provider-thread management; JSONL RPC stdio | approved executable and environment; caller-selected Codex profile plus matching evidence | `codex.cli`; independent app-server, thread-read, and lifecycle behavior segments with permitted visible unverified-newer points | `prepare_codex(AppServer)` → catalogue, session, reconciliation, archive, restore, or delete profile → its typed bound operation; a prepared reconciliation and session may compose through `prepare_settled_session_restoration` | `CodexAppServerDriver`; `ModelCatalogDriver`, `InteractiveSessionDriver`, `ProviderSessionReconciliationDriver`, and `ProviderSessionManagementDriver` |
| `claude-agent.acp` | `swallowtail-adapter-claude-agent`; `swallowtail.claude-agent.acp` | ambient read-write one-prompt structured run and read-only interactive session, both with optional consumer-mediated one-shot permissions; reasoning and plan-mode selection plus inactive provider-session delete; ACP v1 stdio | approved executable and environment; maintainer-supported local Claude subscription auth by default, or explicit Anthropic public API-key pay-as-you-go access | `claude-agent.acp-adapter`; maintained `0.53.0..=0.64.0` excluding `0.58.0`, plus permitted unverified-newer stable points | `prepare_claude_agent` → run, session, or delete profile → `start_run`, `open_session`, or `execute` | `ClaudeAgentAcpDriver`; `StructuredRunDriver`, `InteractiveSessionDriver`, and `ProviderSessionManagementDriver` |
| `claude-code.headless` | `swallowtail-adapter-claude-agent`; `swallowtail.claude-code.headless` | read-only plan-mode one-prompt structured run with usage and reasoning selection; Claude Code stream JSON over stdio | approved `claude` executable and environment; provider-supported local Claude subscription access; explicit ambient harness configuration with no session persistence | `claude-code.headless-stream-json`; exact `2.1.220` plus permitted unverified-newer stable points | `prepare_claude_code_headless` → `prepare_run` → `start_run` | `ClaudeCodeHeadlessDriver`; `StructuredRunDriver` |
| `cursor-agent.catalogue` | `swallowtail-adapter-cursor`; `swallowtail.cursor-agent.catalogue` | authenticated model catalogue; Cursor models stdio | approved executable and environment; provider-supported delegated Cursor subscription login | `cursor-agent.release-date`; exact `2026.07.01-41b2de7` and `2026.07.23-e383d2b`; no inferred gap; later dates permitted as visible unverified newer | `prepare_cursor(Catalogue)` → `prepare_catalogue` → `list_models` | `CursorCatalogueDriver`; `ModelCatalogDriver` |
| `cursor-agent.acp` | `swallowtail-adapter-cursor`; `swallowtail.cursor-agent.acp` | durable interactive session; ACP v1 stdio | approved executable and environment; provider-supported delegated Cursor subscription login; explicit ambient read-write workspace authority | `cursor-agent.release-date`; exact `2026.07.01-41b2de7` and `2026.07.23-e383d2b`; no inferred gap; later dates permitted as visible unverified newer | `prepare_cursor(Acp)` → `prepare_session` → `open_session` | `CursorAcpDriver`; `InteractiveSessionDriver` |
| `cursor-agent.headless` | `swallowtail-adapter-cursor`; `swallowtail.cursor-agent.headless` | explicit-model one-prompt structured run with assistant, thinking, tool, result, and usage events; Cursor stream JSON over stdio | approved executable and environment; provider-supported delegated Cursor subscription login; explicit read-only plan mode or read-write authority; optional sandbox not selected | `cursor-agent.release-date`; exact `2026.07.01-41b2de7` and `2026.07.23-e383d2b`; no inferred gap; later dates permitted as visible unverified newer | `prepare_cursor(Headless)` → `prepare_run` → `start_run` | `CursorHeadlessDriver`; `StructuredRunDriver` |
| `gemini-cli.acp` | `swallowtail-adapter-gemini`; `swallowtail.gemini.acp` | interactive session with negotiated model options and optional exact plan-mode selection on the read-only profile; ACP v1 stdio | approved executable; provider-supported Gemini Developer API-key profile | `gemini-cli.acp-agent`; maintained release plus permitted unverified-newer stable points | `prepare_gemini_acp` → `prepare_session` → `open_session`; read options from the authorized handle | `GeminiAcpDriver`; `InteractiveSessionDriver` |
| `gemini-cli.headless` | `swallowtail-adapter-gemini`; `swallowtail.gemini.headless` | one-prompt structured run with usage; stream-json stdio | approved executable and environment; provider-supported Gemini Developer API-key profile; explicit ambient harness configuration and durable transcript retention | `gemini-cli.headless-stream-json`; exact `0.51.0..=0.52.0` plus permitted unverified-newer stable points | `prepare_gemini_cli(Headless)` → `prepare_run` → `start_run` | `GeminiHeadlessDriver`; `StructuredRunDriver` |
| `grok-build.acp` | `swallowtail-adapter-grok`; `swallowtail.grok-build.acp` | operation-private structured run and interactive session with session-negotiated model options; ACP v1 stdio | approved executable and environment; provider-supported delegated Grok subscription OAuth activated only through the advertised `cached_token` method | `grok-build.executable`; maintained `0.2.114..=0.2.117` across activation and task-control behavior segments, then permitted unverified-newer stable points | `prepare_grok_build` → run or session profile → `start_run` or `open_session`; read authorized model options from the session handle | `GrokAcpDriver`; `StructuredRunDriver` and `InteractiveSessionDriver` |
| `kimi-code.acp` | `swallowtail-adapter-kimi`; `swallowtail.kimi.acp` | resource-scoped provider-session catalogue/import plus persistent interactive session with negotiated model options; ACP v1 stdio | approved executable; delegated Kimi membership OAuth reference; explicit opaque state-root identity for catalogue/import | `kimi-code.executable`; exact `0.28.1` plus qualified `0.29.0..=0.31.1`, then permitted unverified-newer stable points; unverified-newer does not inherit import | `prepare_kimi_code(Acp)` → `prepare_session_catalogue`/`prepare_session_import` or `prepare_session` → typed operation; imported bindings then use ordinary `load_session` or `resume_session` | `KimiAcpDriver`; `ProviderSessionCatalogueDriver`, `ProviderSessionImportDriver`, and `InteractiveSessionDriver` |
| `kimi-code.headless` | `swallowtail-adapter-kimi`; `swallowtail.kimi.headless` | one-prompt structured run; stream-json stdio | approved executable and audited default-engine environment; delegated Kimi membership OAuth reference; explicit ambient harness configuration and durable provider retention | `kimi-code.executable`; exact `0.29.0..=0.31.1`, then permitted unverified-newer stable points | `prepare_kimi_code(Headless)` → `prepare_run` → `start_run` | `KimiHeadlessDriver`; `StructuredRunDriver` |
| `pi.rpc` | `swallowtail-adapter-pi`; `swallowtail.pi.rpc` | catalogue, one-prompt structured run, and interactive session with optional bounded PNG input; strict LF JSONL RPC stdio | approved executable; maintainer-supported `pi/delegated-harness-auth` profile | `pi.package`; maintained range plus permitted unverified-newer stable points | `prepare_pi_rpc` → catalogue, run, or session profile → `list_models`, `start_run`, or `open_session`; run inputs carry attachments and session inputs opt into image-bearing turns | `PiRpcDriver`; `ModelCatalogDriver`, `StructuredRunDriver`, and `InteractiveSessionDriver` |
| `qwen.headless` | `swallowtail-adapter-qwen`; `swallowtail.qwen.headless` | catalogue, structured run, and turn-scoped interactive continuation; structured CLI stream JSON with one joined child per run or turn | approved executable; maintainer-supported `qwen-code/delegated-harness-auth` profile | `qwen-code.package`; maintained `0.19.11..=0.20.1` plus `0.21.0..=0.21.2`, then permitted unverified-newer stable points | `prepare_qwen_catalogue` → `list_models`, or `prepare_qwen_headless` → run or session profile → `start_run` or `open_session`; later turns privately use the exact observed provider session ID | `QwenHeadlessDriver`; `ModelCatalogDriver`, `StructuredRunDriver`, and `InteractiveSessionDriver` |

## Attached Harness Network

| Route | Crate and driver | Role and transport | Explicit target and access | Version axis | Prepared path | Low-level escape hatch |
| --- | --- | --- | --- | --- | --- | --- |
| `kimi-code.local-server` | `swallowtail-adapter-kimi`; `swallowtail.kimi.local-server` | catalogue, retained one-prompt structured run, interactive session, exact-turn reconciliation, optional settled reconcile then replay-free resume, controlled attached-turn detachment, and inactive provider-session archive/restore; local REST and WebSocket v2 | approved loopback server endpoint and opaque server-bearer lease; Kimi retains its separate harness account and configuration | `kimi-code.executable`; exact `0.28.1`, exact `0.29.0`, qualified `0.29.1..=0.30.0`, exact `0.31.0`, and exact `0.31.1` behavior milestones, then permitted visible unverified-newer points | `prepare_kimi_local_server_attached` or `start_kimi_local_server_owned` → catalogue, run, session, reconciliation, archive, restore, or ACP-binding-import profile → its typed operation; a prepared reconciliation and session may compose through `prepare_settled_session_restoration` | `KimiLocalServerDriver`; `ModelCatalogDriver`, `StructuredRunDriver`, `InteractiveSessionDriver`, `ProviderSessionReconciliationDriver`, and `ProviderSessionManagementDriver` |
| `opencode.http` | `swallowtail-adapter-opencode`; `swallowtail.opencode.http` | resource-scoped provider-session catalogue/import and read-only interrupted-turn reconciliation with optional settled bounded load; model catalogue; operation-private structured run with catalogue-gated reasoning, zero-retry harness-validated JSON Schema, optional bounded PNG input, and optional one-shot permission or ordered-question callbacks; interactive session with the same opt-in input/callback subset; opt-in callback-free active-turn detachment with durable binding and later reconciliation; inactive provider-session delete; HTTP/SSE | approved attached-server endpoint; maintainer-supported delegated-auth credential profile | `opencode.server`; qualified published segments `1.14.48..=1.18.10`; later stable points may be visible unverified newer but do not inherit import, reconciliation, or detachment | `prepare_opencode_attached` → session catalogue/import/reconciliation, model catalogue, run, session, or delete profile → typed operation; imported bindings then use ordinary `load_session` or `resume_session`; a prepared reconciliation and session may compose through `prepare_settled_session_restoration`; selected read-only sessions use `with_active_turn_detachment` before dispatch | `OpenCodeHttpDriver`; `ProviderSessionCatalogueDriver`, `ProviderSessionImportDriver`, `ProviderSessionReconciliationDriver`, `ModelCatalogDriver`, `StructuredRunDriver`, `InteractiveSessionDriver`, and `ProviderSessionManagementDriver` |

## External Provider-Session Import Classification

The counts below split combined solution rows by transport. Support requires a
qualified catalogue, exact revalidation, bounded replay, public continuation,
resource binding, activity truth, and exact version evidence.

| Classification | Routes | Promotion boundary |
| --- | --- | --- |
| supported (3) | `codex.app-server`; `kimi-code.acp`; `opencode.http` | complete production profile |
| attachment-only (2) | `claude-agent.acp`; `kimi-code.local-server` | qualified catalogue plus exact list-to-attachment revalidation; Kimi also needs bounded transcript replay |
| blocked (3) | `cursor-agent.acp`; `gemini-cli.headless`; `pi.rpc` | Cursor needs an exact list/load/resume corpus; Gemini needs a side-effect-free list or export before lookup/history qualification; Pi needs provable stored-cwd equality with the host lease |
| not applicable (11) | `codex.exec`; `kimi-code.headless`; `claude-code.headless`; `cursor-agent.catalogue`; `cursor-agent.headless`; `qwen.headless`; `antigravity.catalogue`; `antigravity.headless`; `gemini-cli.acp`; `grok-build.acp`; `anthropic.managed-agent` | selected route exposes no reusable external provider-session identity |

No row inherits capability from another transport or provider family. See
[the import guide](provider-session-import.md) and
[Research 096](../research/096-provider-session-import-route-classification.md)
for the operation-level audit.

Interrupted-turn recovery is classified separately because session-scoped
observation, exact-turn terminal proof, history-only recovery, and live stream
reattachment are not equivalent. See the
[reconciliation guide](provider-operation-reconciliation.md) and
[working-state restoration guide](working-state-restoration.md). Every prepared
interactive harness route now exposes one static post-restart action: Codex,
OpenCode, and Kimi local reconcile; Claude Agent ACP and Kimi ACP recover with
complete bounded replay; Cursor and Grok attach to the exact provider session
while discarding non-authoritative replay; Antigravity continuation, Gemini
ACP, Pi RPC, and Qwen continuation create an explicitly context-losing
replacement. One-prompt routes never auto-retry provider work.

## Hosted Direct And Provider-Owned State

| Route | Crate and driver | Role and transport | Explicit target and access | Version axis | Prepared path | Low-level escape hatch |
| --- | --- | --- | --- | --- | --- | --- |
| `anthropic.messages` | `swallowtail-adapter-anthropic`; `swallowtail.anthropic.direct` | catalogue; one-attempt structured inference with optional bounded PNG input or provider-owned web search; resource-free consumer-tool continuation session; HTTP/SSE | approved `api.anthropic.com` endpoint; provider-supported public API key and pay-as-you-go billing | no ordered axis; exact `anthropic-2023-06-01` facade | `prepare_anthropic_direct` → catalogue, inference-attempt, or session profile → `list_models`, `start_run`, or `open_session`; image and search remain structured-run options, while tools use the separate direct-continuation role | `AnthropicDirectDriver`; `ModelCatalogDriver`, `StructuredRunDriver`, and `InteractiveSessionDriver` |
| `kimi-platform.chat` | `swallowtail-adapter-kimi-platform`; `swallowtail.kimi-platform.direct-chat` | catalogue and structured run; HTTP/SSE | approved `api.moonshot.ai` endpoint; Platform API key and pay-as-you-go billing | no ordered axis; exact `kimi-platform-chat-2026-07-21` facade | `prepare_kimi_platform_direct` → `prepare_catalogue` or `prepare_inference_attempt` → `list_models` or `start_run` | `KimiPlatformDirectDriver`; `ModelCatalogDriver` and `StructuredRunDriver` |
| `deepseek.continuation` | `swallowtail-adapter-deepseek`; `swallowtail.deepseek.direct` | catalogue, one-request structured run, and interactive direct continuation; OpenAI-compatible HTTP/SSE | exact `https://api.deepseek.com` target; Open Platform API key | `deepseek.openai-chat-facade`; exact opaque revision | `prepare_deepseek_direct` → catalogue, run, or session profile → `list_models`, `start_run`, or `open_session` | `DeepSeekDirectDriver`; `ModelCatalogDriver`, `StructuredRunDriver`, and `InteractiveSessionDriver` |
| `alibaba.conversations` | `swallowtail-adapter-alibaba-model-studio`; `swallowtail.alibaba-model-studio.conversations-responses` | resource-free structured run and interactive provider conversation; HTTPS/SSE | approved Singapore workspace endpoint; general API key and pay-as-you-go billing | no ordered axis; exact `openai-conversations-responses` facade | `prepare_alibaba_model_studio` → run or conversation profile → `start_run` or `open_session` | `AlibabaModelStudioDriver`; `StructuredRunDriver` and `InteractiveSessionDriver` |
| `openai.background` | `swallowtail-adapter-openai`; `swallowtail.openai.background` | retained structured run with exact reasoning, provider-native JSON Schema, optional controlled detachment, and exact read-only run reconciliation; HTTP/SSE background Responses | exact public API endpoint; public API key and pay-as-you-go billing | `openai.responses-background-facade`; exact opaque revision | `prepare_openai_background` → `prepare_background_run` → `start_run`, or `prepare_run_reconciliation` → `reconcile`; detachable run profiles opt in before dispatch | `OpenAiBackgroundDriver`; `StructuredRunDriver` and `ProviderRunReconciliationDriver` |
| `anthropic.managed-agent` | `swallowtail-adapter-anthropic`; `swallowtail.anthropic.managed-agent` | provider-hosted harness structured run with opt-in exact-run reconciliation and separate inactive recovered-resource cleanup; managed-agent HTTPS/SSE | approved first-party endpoint; public API key, pay-as-you-go billing, and operator-owned agent version | `anthropic.managed-agents-facade`; exact opaque revision | `prepare_anthropic_managed_agent` → ordinary or recoverable `prepare_managed_run` → `start_run`; later `prepare_run_reconciliation` → `reconcile`, or `prepare_recovered_cleanup` → `cleanup` | `AnthropicManagedAgentDriver`; `StructuredRunDriver`, `ProviderRunReconciliationDriver`, and `ProviderRecoveredResourceCleanupDriver` |

## Realtime Connections

| Route | Crate and driver | Role and transport | Explicit target and access | Version axis | Prepared path | Low-level escape hatch |
| --- | --- | --- | --- | --- | --- | --- |
| `xai.responses-websocket` | `swallowtail-adapter-xai`; `swallowtail.xai.websocket` | one-response structured run and connection-scoped interactive session; Responses WebSocket | approved `/v1/responses` endpoint; xAI public API key and pay-as-you-go billing | `xai.responses-websocket-facade`; exact opaque revision | `prepare_xai_responses_websocket` → run or session profile → `start_run` or `open_session` | `XaiWebSocketDriver`; `StructuredRunDriver` and `InteractiveSessionDriver` |
| `openai.realtime` | `swallowtail-adapter-openai`; `swallowtail.openai.realtime` | realtime media session with exact output-token maximum; WebSocket | approved public Realtime endpoint; OpenAI public API key and pay-as-you-go billing | `openai.realtime-facade`; exact opaque revision | `prepare_openai_realtime` → `prepare_realtime_session` → `open_session` | `OpenAiRealtimeDriver`; `RealtimeMediaSessionDriver` |
| `gemini.live` | `swallowtail-adapter-gemini`; `swallowtail.gemini.live` | realtime media session; Gemini Live raw WebSocket | approved Gemini Live endpoint; project authorization API key | `gemini.live-facade`; exact opaque revision | `prepare_gemini_live` → `prepare_live_session` → `open_session` | `GeminiLiveDriver`; `RealtimeMediaSessionDriver` |

## Auxiliary Hosted Catalogue Branches

These branches report provider or control-plane inventory for existing
solutions. They are not extra inference routes, and their results do not claim
compatibility with the solution transport beside them.

| Provider scope | Driver | Exact source and access | Prepared path |
| --- | --- | --- | --- |
| Alibaba Model Studio | `swallowtail.alibaba-model-studio.deployable-models` | international deployable-model control plane; general API key | `prepare_alibaba_deployable_models` → `prepare_catalogue` → `list_models` |
| Gemini | `swallowtail.gemini.models` | Gemini Developer API `models.list`; project API key | `prepare_gemini_models` → `prepare_catalogue` → `list_models` |
| OpenAI | `swallowtail.openai.models` | public Models API; OpenAI public API key | `prepare_openai_models` → `prepare_catalogue` → `list_models` |
| xAI | `swallowtail.xai.models` | public language-models API; xAI public API key | `prepare_xai_models` → `prepare_catalogue` → `list_models` |

## Embedded SDK

| Route | Crate and driver | Role and transport | Explicit target and access | Version axis | Prepared path | Low-level escape hatch |
| --- | --- | --- | --- | --- | --- | --- |
| `bedrock.runtime` | `swallowtail-adapter-bedrock`; `swallowtail.amazon-bedrock.direct` | structured run; Rust SDK EventStream | approved regional Runtime target and explicit `BedrockCloudClientConfig`; delegated cloud-provider identity | `amazon-bedrock.runtime-rust-sdk` plus `amazon-bedrock.runtime-service-api`; exact opaque revisions | `prepare_bedrock` → `runtime` → `prepare_inference_attempt` → `start_run` | `BedrockDirectDriver`; `StructuredRunDriver` |
| `bedrock.catalogue` | `swallowtail-adapter-bedrock`; `swallowtail.amazon-bedrock.catalogue` | model catalogue; Rust SDK control plane | approved regional control-plane target and explicit `BedrockCloudClientConfig`; delegated cloud-provider identity | `amazon-bedrock.control-plane-rust-sdk` plus `amazon-bedrock.control-plane-service-api`; exact opaque revisions | `prepare_bedrock` → `catalogue` → `prepare_catalogue` → `list_models` | `BedrockCatalogueDriver`; `ModelCatalogDriver` |

`prepare_bedrock` binds only the shared execution host, region, and explicit
credential provider. Its typed `runtime` and `catalogue` branches still require
separate configured-instance identity, target, access profile, evidence,
descriptor, version axes, preflight plan, and low-level driver. The direct
route-specific constructors remain available.

## Local Model Runtimes

| Route | Crate and driver | Role and transport | Explicit target and access | Version axis | Prepared path | Low-level escape hatch |
| --- | --- | --- | --- | --- | --- | --- |
| `ollama.attached` | `swallowtail-adapter-ollama`; `swallowtail.ollama.native-attached` | inventory, structured run, and bounded interactive transcript replay with selected-model reasoning evidence and provider-native JSON Schema on structured runs; native HTTP/NDJSON | approved attached-runtime endpoint; local unauthenticated compute | `ollama.runtime`; guaranteed semantic range, exact exclusion, and permitted unverified-newer stable points | `prepare_ollama_attached` → inventory, inference-attempt, or session profile → `observe_inventory`, `start_run`, or `open_session`; session history commits only clean terminal turns | `OllamaNativeAttachedDriver`; `ModelCatalogDriver`, `StructuredRunDriver`, and `InteractiveSessionDriver` |
| `llama-cpp.attached` | `swallowtail-adapter-llama-cpp`; `swallowtail.llama-cpp.attached-openai-chat` | catalogue and structured run; HTTP/SSE | approved external server endpoint; local unauthenticated compute | `llama.cpp.attached-runtime`; exact opaque b9910/f5525f7e7 revision | `prepare_llama_cpp_attached` → `prepare_catalogue` or `prepare_inference_attempt` → `list_models` or `start_run` | `LlamaCppAttachedDriver`; `ModelCatalogDriver` and `StructuredRunDriver` |
| `llama-cpp.owned` | `swallowtail-adapter-llama-cpp`; `swallowtail.llama-cpp.owned-b10069-openai-chat` | owned ephemeral serving lifecycle; process plus HTTP/SSE | approved `llama-server` executable and exact GGUF artifact; local unauthenticated compute | `llama.cpp.owned-runtime`; exact opaque b10069/178a6c449 revision | `prepare_llama_cpp_owned` → `prepare_serving_start` → `start` and returned-handle `stop` | `LlamaCppOwnedDriver`; `ServingInstanceDriver` |

## Provider Session Lifecycle

This matrix classifies the selected production driver and transport only.
`unsupported` means the route has persistent provider sessions but has not
qualified the requested management action. `not-applicable` means the route's
current operation shape has no user-managed persistent provider session.

Another CLI, SDK, REST route, filesystem path, application UI, or private
provider surface cannot substitute for the selected driver. Driver-owned
cleanup releases resources created or attached by an operation. It never
creates a management binding or satisfies user-directed archive, restore, or
delete.

<!-- provider-session-lifecycle-matrix:start -->
| Route | Persistent-session posture | Management binding | Archive | Restore | Delete | Deletion strength | Version posture | Driver-owned cleanup |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `antigravity.catalogue` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | exact qualified `1.1.9`; later stable points remain visible `UnverifiedNewer`; catalogue has no provider session | process and task cleanup only |
| `antigravity.headless` | `unsupported` | `no` | `unsupported` | `unsupported` | `unsupported` | `unsupported` | exact qualified `1.1.9`; structured runs and exact-id continuation preserve provider state without exposing public session identity | each run or turn joins its process and task; provider state is preserved |
| `codex.exec` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | accepted `codex.cli` exec segments; no persistent-session claim | run and process cleanup only |
| `codex.app-server` | `supported` | `yes` | `supported` | `supported` | `supported` | `ProviderHardDeleted` | qualified segments inside `0.80.0..=0.146.0`, excluding `0.82.0..=0.83.0` and `0.108.0..=0.109.0`: archive from `0.80.0`, restore from `0.92.0`, delete from `0.140.0`; later stable points may be visible `UnverifiedNewer` | attachment and process cleanup only; provider state is preserved |
| `claude-agent.acp` | `supported` | `yes` | `unsupported` | `unsupported` | `supported` | `ProviderDataDeleted` | `0.53.0..=0.64.0`, excluding unpublished `0.58.0`; later stable releases are visible `UnverifiedNewer` | durable structured runs preserve history; the opt-in temporary profile closes then deletes its operation-private session |
| `claude-code.headless` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | exact qualified `2.1.220`; later stable releases may be visible `UnverifiedNewer`; every run disables session persistence | process and task cleanup only; no provider session is retained |
| `cursor-agent.catalogue` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | exact qualified `2026.07.01-41b2de7` and `2026.07.23-e383d2b`; no inferred gap; later dates remain visible `UnverifiedNewer`; catalogue has no provider session | process cleanup only |
| `cursor-agent.acp` | `unsupported` | `no` | `unsupported` | `unsupported` | `unsupported` | `unsupported` | exact qualified `2026.07.01-41b2de7` and `2026.07.23-e383d2b`; no inferred gap; later dates remain visible `UnverifiedNewer`; load suppresses history-read and per-turn replay failures, so advertisement cannot qualify continuation recovery or lifecycle management | connection, resource, process, and task cleanup only; provider session state is preserved |
| `cursor-agent.headless` | `unsupported` | `no` | `unsupported` | `unsupported` | `unsupported` | `unsupported` | exact qualified `2026.07.01-41b2de7` and `2026.07.23-e383d2b`; no inferred gap; later dates remain visible `UnverifiedNewer`; durable provider state exposes no public session identity | process and task cleanup only; provider state is preserved |
| `gemini-cli.acp` | `unsupported` | `no` | `unsupported` | `unsupported` | `unsupported` | `unsupported` | exact qualified `0.51.0`; later stable points may be visible `UnverifiedNewer`; selected ACP route advertises no close or delete | connection and process cleanup only; provider state is preserved |
| `gemini-cli.headless` | `unsupported` | `no` | `unsupported` | `unsupported` | `unsupported` | `unsupported` | exact qualified `0.51.0..=0.52.0`; delete exit and provider prose cannot confirm removal, while `--list-sessions` may invoke summary inference and mutate history | durable runs preserve the transcript; the opt-in owned cleanup profile issues one joined delete attempt and reports removal unconfirmed without listing |
| `kimi-code.acp` | `unsupported` | `no` | `unsupported` | `unsupported` | `unsupported` | `unsupported` | exact qualified `0.28.1` plus `0.29.0..=0.31.1`; later stable points may be visible `UnverifiedNewer`; selected ACP route supports separate catalogue/import but advertises no close or delete | catalogue/import joins its process and releases leases; interactive connection cleanup preserves provider state |
| `kimi-code.headless` | `unsupported` | `no` | `unsupported` | `unsupported` | `unsupported` | `unsupported` | exact qualified `0.29.0..=0.31.1`; later stable points may be visible `UnverifiedNewer`; each run may retain provider-owned session state without exposing its identity | process and task cleanup only; provider state is preserved |
| `kimi-code.local-server` | `supported` | `yes` | `supported` | `supported` | `unsupported` | `unsupported` | exact qualified `0.28.1` plus `0.29.0..=0.31.1`; profile and disabled-tool controls require `0.29.0`; `0.31.0` has a distinct subagent-status revision and `0.31.1` a refresh-stable revision; later stable points may be visible `UnverifiedNewer` | joins WebSocket and task work, releases the server-bearer lease, preserves provider state, and stops only an owned foreground child |
| `pi.rpc` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | accepted `pi.package` segments; current RPC operation has no management binding | attachment and process cleanup only |
| `qwen.headless` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | maintained `0.19.11..=0.20.1` plus `0.21.0..=0.21.2`; current structured-run operation has no provider session | run and process cleanup only |
| `opencode.http` | `supported` | `yes` | `unsupported` | `unsupported` | `supported` | `ProviderDataDeleted` | `1.14.48..=1.18.10` across exact published segments; later stable points may be visible `UnverifiedNewer` | structured runs delete their operation-private session; interactive attachment cleanup preserves the external server |
| `anthropic.messages` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | exact `anthropic-2023-06-01` facade; one-attempt inference has no provider session | request and stream cleanup only |
| `kimi-platform.chat` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | exact `kimi-platform-chat-2026-07-21` facade; one-attempt inference has no provider session | request and stream cleanup only |
| `deepseek.continuation` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | exact opaque `deepseek.openai-chat-facade` revision; continuation is consumer-owned | request and stream cleanup only |
| `alibaba.conversations` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | exact `openai-conversations-responses` facade; the conversation is operation-owned | deletes driver-created items, then its conversation; never user-directed management |
| `openai.background` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | exact opaque `openai.responses-background-facade` revision; retained run is not a reusable thread | ordinary runs use bounded retrieval and native cancellation, then terminal response deletion; selected detachment preserves the exact checkpointed response for read-only run reconciliation |
| `anthropic.managed-agent` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | exact opaque `anthropic.managed-agents-facade` revision; session and environment are operation-owned | ordinary close deletes its created session, then environment; the opt-in recoverable profile emits separate exact-run observation and inactive-resource cleanup authority, never user-directed session management |
| `grok-build.acp` | `unsupported` | `no` | `unsupported` | `unsupported` | `unsupported` | `unsupported` | maintained `0.2.114..=0.2.117`; later stable points may be visible `UnverifiedNewer`; stripped artifacts and no deterministic load transcript cannot prove complete client-visible replay, so no continuation-recovery, lifecycle, or management mapping is qualified | attachment, process, credential, and working-resource cleanup only; provider-owned session state is preserved |
| `xai.responses-websocket` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | exact opaque `xai.responses-websocket-facade` revision; continuation is connection-local | connection and response cleanup only |
| `openai.realtime` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | exact opaque `openai.realtime-facade` revision; media session is connection-scoped | response and connection cleanup only |
| `gemini.live` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | exact opaque `gemini.live-facade` revision; media session is connection-scoped | provider rollover and connection cleanup only |
| `bedrock.runtime` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | exact SDK and Runtime service revisions; inference attempt has no provider session | request and EventStream cleanup only |
| `bedrock.catalogue` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | exact SDK and control-plane service revisions; catalogue has no provider session | request cleanup only |
| `ollama.attached` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | accepted `ollama.runtime` segments; attached inference has no provider session | request cleanup only; the external runtime is preserved |
| `llama-cpp.attached` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | exact opaque b9910/f5525f7e7 revision; attached inference has no provider session | request cleanup only; the external server is preserved |
| `llama-cpp.owned` | `not-applicable` | `no` | `not-applicable` | `not-applicable` | `not-applicable` | `not-applicable` | exact opaque b10069/178a6c449 revision; serving lifecycle is not session management | stops and joins the owned server; model and provider state are not managed |
<!-- provider-session-lifecycle-matrix:end -->

## Kimi Code Route Selection

Kimi Code has three independent harness routes:

| Choice | Use it for | Transport and topology | Management |
| --- | --- | --- | --- |
| `kimi-code.acp` | the smallest installed-harness path, including resource-scoped discovery/import, provider load replay, and resume | ACP v1 over an attached or owned stdio process | imported binding only; no archive, restore, or delete |
| `kimi-code.headless` | one bounded prompt with the smallest consumer-facing operation shape | stream-JSON over one owned stdio process | durable provider state is explicit; no reusable session or management binding |
| `kimi-code.local-server` | one retained structured prompt or a reusable Web-style session with explicit approvals and questions | REST plus WebSocket v2 against an attached loopback server or a Swallowtail-owned foreground child | archive and restore after interactive handle close; no delete |

The installed `prepare_kimi_code` facade requires explicit `Acp` or `Headless`
selection; it never infers one from the requested operation. Local-server
topology and access are different enough to retain their own facade. No route
is an authority fallback for another. Local-server preparation
requires a host-approved loopback endpoint and opaque bearer lease. That bearer
authenticates the local server; it is not a Kimi account credential, Platform
API key, or transferable membership token. Kimi retains its own account and
configuration authority.

Owned topology starts `kimi web --no-open` directly and joins that child. It
does not require a container and does not claim a sandbox. Attached topology
preserves the external server. Permission mode and any provider or host
isolation remain explicit.

An ACP session can receive local-server archive/restore authority only through
the typed binding-import operation. Source authority, execution host,
executable version, state root, endpoint target, access profile, and observed
session must agree. A raw session id is insufficient.

Close the interactive handle before archive or restore. Handle close preserves
provider state; archive and restore are separate consumer-directed effects.
Kimi exposes no qualified hard-delete operation on either route.

Codex archive guarantees only `TargetOnly`. Its qualified delete segment
reports `ProviderHardDeleted` with `ProviderDefinedDescendants`. Claude Agent
and OpenCode report `ProviderDataDeleted` with
`ProviderDefinedDescendants`; neither claims secure erasure. Version-qualified
action absence still prepares the route for its older supported operations,
but it does not advertise the absent management action.

## Version Posture

Ordered installed-harness and native-runtime claims distinguish:

- maintained points inside the guaranteed range
- deprecated points still covered by an explicit behavior segment
- known exclusions and prereleases, which do not prepare
- exact later stable observations admitted as `UnverifiedNewer`

`UnverifiedNewer` means execution is allowed with the latest qualified behavior
when the claim permits it. It is mileage-may-vary, does not extend the
guaranteed range, and remains visible in prepared evidence. Swallowtail does
not hard-deny a later stable release solely for exceeding the maintained upper
milestone.

Opaque hosted facades, SDK packages, service revisions, and exact llama.cpp
runtime revisions are qualified only at named points. They do not infer an
ordered range or an unverified-newer attempt.

The active [installed-harness maintenance inventory](../research/074-installed-harness-and-protocol-maintenance-inventory.md)
records evidence cutoffs, corpus and live-probe posture, shared ACP axes, and
the next bounded currentness source set. It does not extend any compatibility
claim by itself.

## Examples

All examples compile from their adapter crate's public API under
`effigy check:examples`.

- Installed harnesses:
  [Codex](../../crates/swallowtail-adapter-codex/examples/prepared_discovery.rs),
  [Claude Agent](../../crates/swallowtail-adapter-claude-agent/examples/prepared_claude_agent_acp.rs),
  [Claude Code headless](../../crates/swallowtail-adapter-claude-agent/examples/prepared_claude_code_headless.rs),
  [Gemini CLI ACP](../../crates/swallowtail-adapter-gemini/examples/prepared_gemini_acp.rs),
  [Gemini CLI headless](../../crates/swallowtail-adapter-gemini/examples/prepared_gemini_headless.rs),
  [Kimi Code ACP](../../crates/swallowtail-adapter-kimi/examples/prepared_acp.rs),
  [Kimi Code headless](../../crates/swallowtail-adapter-kimi/examples/prepared_headless.rs),
  [Pi](../../crates/swallowtail-adapter-pi/examples/prepared_pi_rpc.rs), and
  [Qwen](../../crates/swallowtail-adapter-qwen/examples/prepared_qwen_headless.rs)
- Attached harnesses:
  [OpenCode](../../crates/swallowtail-adapter-opencode/examples/prepared_opencode_attached.rs),
  [Kimi local server](../../crates/swallowtail-adapter-kimi/examples/prepared_local_server_attached.rs),
  [Kimi retained structured run](../../crates/swallowtail-adapter-kimi/examples/prepared_local_server_structured.rs),
  [Kimi interactive session](../../crates/swallowtail-adapter-kimi/examples/prepared_local_server_interactive.rs),
  [Kimi owned lifecycle](../../crates/swallowtail-adapter-kimi/examples/prepared_local_server_owned_lifecycle.rs),
  and [Kimi ACP binding import](../../crates/swallowtail-adapter-kimi/examples/prepared_local_server_imported_management.rs)
- Hosted and provider-owned state:
  [Anthropic direct](../../crates/swallowtail-adapter-anthropic/examples/prepared_direct.rs),
  [Kimi Platform](../../crates/swallowtail-adapter-kimi-platform/examples/prepared_kimi_platform_direct.rs),
  [DeepSeek](../../crates/swallowtail-adapter-deepseek/examples/prepared_direct_continuation.rs),
  [Alibaba](../../crates/swallowtail-adapter-alibaba-model-studio/examples/prepared_provider_conversation.rs),
  [OpenAI background](../../crates/swallowtail-adapter-openai/examples/prepared_background_response.rs), and
  [Anthropic managed](../../crates/swallowtail-adapter-anthropic/examples/prepared_managed_agent.rs)
- Realtime:
  [xAI](../../crates/swallowtail-adapter-xai/examples/prepared_responses_websocket.rs),
  [OpenAI](../../crates/swallowtail-adapter-openai/examples/prepared_realtime_session.rs), and
  [Gemini Live](../../crates/swallowtail-adapter-gemini/examples/prepared_live_session.rs)
- Embedded SDK:
  [combined Bedrock facade](../../crates/swallowtail-adapter-bedrock/examples/prepared_bedrock.rs),
  [Bedrock Runtime](../../crates/swallowtail-adapter-bedrock/examples/prepared_runtime.rs), and
  [Bedrock catalogue](../../crates/swallowtail-adapter-bedrock/examples/prepared_catalogue.rs)
- Local runtimes:
  [Ollama](../../crates/swallowtail-adapter-ollama/examples/prepared_attached.rs),
  [llama.cpp attached](../../crates/swallowtail-adapter-llama-cpp/examples/prepared_llama_cpp_attached.rs), and
  [llama.cpp owned](../../crates/swallowtail-adapter-llama-cpp/examples/prepared_llama_cpp_owned.rs)

## Consumer-Owned Inputs

The prepared path still requires the consumer to select or authorize every
material choice relevant to the route:

- adapter route, configured-instance identity and revision
- execution host and host-approved target
- access profile, credential reference where required, and access evidence
- model and route where the operation uses one
- prompt or media, output bound, tools, schema, attachments, working resource,
  network, search, retention, and deadline where supported

Consumers retain prompts, authorization policy, workflows, retry policy,
persistence, memory, routing preferences, and UI.

## Non-Goals

There is no umbrella provider facade, universal prompt method, automatic
provider or model router, ambient credential discovery, implicit sign-in,
installation or update flow, endpoint fallback, billing fallback, mandatory
sandbox, or conversion between harness access and public API access.

Preparation failure is failure for the selected row. It does not authorize
another route.
