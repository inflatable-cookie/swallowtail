# Integration Guide Map

This is the documentation front door for agents and operators. It maps
realized route and feature truth to usable guidance. The
[provider route matrix](provider-route-matrix.md) remains authoritative for
route selection and capability posture.

Coverage states:

- **partial** — useful guidance exists but has not passed the full Contract
  052 route or feature checklist
- **missing** — no canonical task-oriented guide exists
- **complete** — guide, example, and coverage validation meet Contract 052

All 36 production route rows and every portable feature family have completed
the checklist. Coverage state describes documentation evidence, not the
underlying production capability.

## Where Facts Live

Each artifact owns one kind of fact. When the same fact appears in two places,
treat the authority below as the source:

| Artifact | Owns | Read it for |
| --- | --- | --- |
| [Provider route matrix](provider-route-matrix.md) | route identity, driver, transport, access, version axes, lifecycle posture | which route exists and what it is capable of |
| [Feature matrix CSV](provider-solution-feature-matrix.csv) | portable capability cells per solution | whether a feature is supported on a route |
| [Activity matrix](provider-solution-activity-matrix.md) | observable-activity fidelity per route and operation | what agent work is visible, and how faithful it is |
| Route guides | how to use one route: prerequisites, normal flow, failures | the exact preparation and execution sequence |
| [Key Concepts](key-concepts.md) | plain-English definitions of shared terms | what the vocabulary means before you read a guide |

Guides restate matrix facts for convenience. Prefer the matrix when a fact
conflicts or when you are qualifying a new claim.

## Route Guides

| Route | Canonical guide | Normal-path example | Coverage |
| --- | --- | --- | --- |
| `antigravity.catalogue` | [Antigravity](antigravity-prepared-integration.md) | [prepared catalogue](../../crates/swallowtail-adapter-antigravity/examples/prepared_antigravity_catalogue.rs) | complete |
| `antigravity.headless` | [Antigravity](antigravity-prepared-integration.md) | [prepared headless and continuation](../../crates/swallowtail-adapter-antigravity/examples/prepared_antigravity_headless.rs) | complete |
| `codex.exec` | [Codex](codex-prepared-integration.md) | [prepared discovery](../../crates/swallowtail-adapter-codex/examples/prepared_discovery.rs) | complete |
| `codex.app-server` | [Codex](codex-prepared-integration.md) | [prepared discovery](../../crates/swallowtail-adapter-codex/examples/prepared_discovery.rs) | complete |
| `claude-agent.acp` | [Claude Agent](claude-agent-prepared-integration.md) | [prepared ACP](../../crates/swallowtail-adapter-claude-agent/examples/prepared_claude_agent_acp.rs) | complete |
| `claude-code.headless` | [Claude Agent](claude-agent-prepared-integration.md) | [prepared headless](../../crates/swallowtail-adapter-claude-agent/examples/prepared_claude_code_headless.rs) | complete |
| `claude-code.response-only` | [Claude Agent](claude-agent-prepared-integration.md) | [prepared response only](../../crates/swallowtail-adapter-claude-agent/examples/prepared_claude_code_response_only.rs) | complete |
| `cursor-agent.catalogue` | [Cursor](cursor-prepared-integration.md) | [prepared catalogue](../../crates/swallowtail-adapter-cursor/examples/prepared_cursor_catalogue.rs) | complete |
| `cursor-agent.acp` | [Cursor](cursor-prepared-integration.md) | [prepared ACP](../../crates/swallowtail-adapter-cursor/examples/prepared_cursor_acp.rs) | complete |
| `cursor-agent.headless` | [Cursor](cursor-prepared-integration.md) | [prepared headless](../../crates/swallowtail-adapter-cursor/examples/prepared_cursor_headless.rs) | complete |
| `gemini-cli.acp` | [Gemini CLI](gemini-cli-prepared-integration.md) | [prepared ACP](../../crates/swallowtail-adapter-gemini/examples/prepared_gemini_acp.rs) | complete |
| `gemini-cli.headless` | [Gemini CLI](gemini-cli-prepared-integration.md) | [prepared headless](../../crates/swallowtail-adapter-gemini/examples/prepared_gemini_headless.rs) | complete |
| `grok-build.acp` | [Grok Build](grok-build-prepared-integration.md) | [prepared run and session](../../crates/swallowtail-adapter-grok/examples/prepared_grok_build_acp.rs) | complete |
| `kimi-code.acp` | [Kimi Code](kimi-prepared-integration.md) | [prepared ACP](../../crates/swallowtail-adapter-kimi/examples/prepared_acp.rs) | complete |
| `kimi-code.headless` | [Kimi Code](kimi-prepared-integration.md) | [prepared headless](../../crates/swallowtail-adapter-kimi/examples/prepared_headless.rs) | complete |
| `muse-code.headless` | [Muse Code](muse-code-prepared-integration.md) | [prepared headless](../../crates/swallowtail-adapter-muse/examples/prepared_muse_headless.rs) | complete |
| `command-code.headless` | [Command Code](command-code-prepared-integration.md) | [prepared headless](../../crates/swallowtail-adapter-command-code/examples/prepared_command_code_headless.rs) | complete |
| `oh-my-pi.rpc` | [Oh My Pi](oh-my-pi-prepared-integration.md) | [prepared RPC](../../crates/swallowtail-adapter-oh-my-pi/examples/prepared_oh_my_pi_rpc.rs) | complete |
| `pi.rpc` | [Pi RPC](pi-rpc-prepared-integration.md) | [prepared RPC](../../crates/swallowtail-adapter-pi/examples/prepared_pi_rpc.rs) | complete |
| `qwen.headless` | [Qwen](qwen-headless-prepared-integration.md) | [prepared headless](../../crates/swallowtail-adapter-qwen/examples/prepared_qwen_headless.rs) | complete |
| `kimi-code.local-server` | [Kimi Local Server](kimi-local-server-prepared-integration.md) | [prepared attached](../../crates/swallowtail-adapter-kimi/examples/prepared_local_server_attached.rs) | complete |
| `opencode.http` | [OpenCode](opencode-attached-prepared-integration.md) | [prepared attached](../../crates/swallowtail-adapter-opencode/examples/prepared_opencode_attached.rs) | complete |
| `anthropic.messages` | [Anthropic Direct](anthropic-direct-prepared-integration.md) | [prepared direct](../../crates/swallowtail-adapter-anthropic/examples/prepared_direct.rs) | complete |
| `kimi-platform.chat` | [Kimi Platform](kimi-platform-prepared-integration.md) | [prepared direct](../../crates/swallowtail-adapter-kimi-platform/examples/prepared_kimi_platform_direct.rs) | complete |
| `deepseek.continuation` | [DeepSeek](deepseek-prepared-integration.md) | [prepared continuation](../../crates/swallowtail-adapter-deepseek/examples/prepared_direct_continuation.rs) | complete |
| `alibaba.conversations` | [Alibaba Model Studio](alibaba-model-studio-prepared-integration.md) | [prepared conversation](../../crates/swallowtail-adapter-alibaba-model-studio/examples/prepared_provider_conversation.rs) | complete |
| `openai.background` | [OpenAI Background](openai-background-prepared-integration.md) | [prepared response](../../crates/swallowtail-adapter-openai/examples/prepared_background_response.rs) | complete |
| `anthropic.managed-agent` | [Anthropic Managed Agent](anthropic-managed-agent-prepared-integration.md) | [prepared managed agent](../../crates/swallowtail-adapter-anthropic/examples/prepared_managed_agent.rs) | complete |
| `xai.responses-websocket` | [Realtime routes](realtime-prepared-integration.md) | [prepared WebSocket](../../crates/swallowtail-adapter-xai/examples/prepared_responses_websocket.rs) | complete |
| `openai.realtime` | [Realtime routes](realtime-prepared-integration.md) | [prepared realtime](../../crates/swallowtail-adapter-openai/examples/prepared_realtime_session.rs) | complete |
| `gemini.live` | [Realtime routes](realtime-prepared-integration.md) | [prepared live](../../crates/swallowtail-adapter-gemini/examples/prepared_live_session.rs) | complete |
| `bedrock.runtime` | [Bedrock SDK](bedrock-sdk-prepared-integration.md) | [prepared runtime](../../crates/swallowtail-adapter-bedrock/examples/prepared_runtime.rs) | complete |
| `bedrock.catalogue` | [Bedrock SDK](bedrock-sdk-prepared-integration.md) | [prepared catalogue](../../crates/swallowtail-adapter-bedrock/examples/prepared_catalogue.rs) | complete |
| `ollama.attached` | [Ollama](ollama-attached-prepared-integration.md) | [prepared attached](../../crates/swallowtail-adapter-ollama/examples/prepared_attached.rs) | complete |
| `llama-cpp.attached` | [llama.cpp](llama-cpp-prepared-integration.md) | [prepared attached](../../crates/swallowtail-adapter-llama-cpp/examples/prepared_llama_cpp_attached.rs) | complete |
| `llama-cpp.owned` | [llama.cpp](llama-cpp-prepared-integration.md) | [prepared owned](../../crates/swallowtail-adapter-llama-cpp/examples/prepared_llama_cpp_owned.rs) | complete |

## Feature Guide Families

| Feature family | Matrix columns and portable surfaces | Current guide | Coverage |
| --- | --- | --- | --- |
| route and configured-instance selection | `model_catalog`, `prepared_facade`, `configured_provider_instance_catalogue` | [selection and preparation](provider-selection-and-preparation.md) | complete |
| ordinary operation shapes | `structured_run`, `interactive_session` | [ordinary lifecycle](ordinary-operation-lifecycle.md) | complete |
| realtime sessions | `realtime_media_session`, `planned_connection_rollover` | [realtime routes](realtime-prepared-integration.md) | complete |
| events and accounting | `streaming_events`, `usage_evidence`, `billed_cost_evidence` | [ordinary lifecycle](ordinary-operation-lifecycle.md), [observable activity](observable-activity.md) | complete |
| generation controls | `output_token_limit`, `reasoning_selection`, `structured_output` | [generation controls and input authority](generation-controls-and-input-authority.md) | complete |
| inputs and authority | `attachments`, `consumer_tool_exchange`, `permission_exchange`, `question_exchange`, `working_resource`, `bounded_workspace_text_write`, `external_search` | [generation controls and input authority](generation-controls-and-input-authority.md) | complete |
| operation control | `cancellation_or_interruption`, terminal status, cleanup | [ordinary lifecycle](ordinary-operation-lifecycle.md) | complete |
| session continuation | `load_session`, `resume_session`, `persistent_session_posture`, native close | [provider state and resources](provider-state-and-resource-lifecycle.md) | complete |
| provider history browse | `provider_session_history` | [session history pages](provider-session-history.md) | complete |
| external session discovery | `provider_session_catalogue`, `provider_session_import` | [session import](provider-session-import.md) | complete |
| retained work and restart | `retained_background_execution`, `stream_reattachment`, `provider_managed_recovery`, `working_state_restoration` | [provider state and resources](provider-state-and-resource-lifecycle.md), [restoration](working-state-restoration.md) | complete |
| provider management and cleanup | `provider_session_archive`, `provider_session_restore`, `provider_session_delete`, `native_session_close`, `owned_remote_resource_cleanup`, `owned_runtime_lifecycle` | [provider state and resources](provider-state-and-resource-lifecycle.md) | complete |
| activity, plans, tasks, and child work | `observable_activity`, `plan_mode`, `task_lists`, `subagent_topology`, `subagent_control` | [observable activity](observable-activity.md), [activity matrix](provider-solution-activity-matrix.md) | complete |
| failures | `failure_classification`; exact diagnostics, preparation, terminal, activity, and cleanup failures | [portable failure handling](portable-failure-handling.md) | complete |
| operator validation | `operator_validation`; deterministic tiers, package proof, optional live probes | [validation tiers](validation-tiers.md) | complete |

## Maintainer Guidance

- [Prepared Facade Authoring](prepared-facade-authoring.md)
- [Validation Tiers](validation-tiers.md)

These do not replace consumer route and feature instructions.
