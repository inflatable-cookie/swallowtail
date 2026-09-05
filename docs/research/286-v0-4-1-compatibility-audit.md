# Research 286 — v0.4.0 To v0.4.1 Compatibility Audit
Status: complete evidence; patch-compatible candidate
Date: 2026-09-05
Card: g05.030 / Card 090
Audited tree: `2187bbecb9e24425f574f4c6c434fd8f16fe0300`

## Decision

v0.4.0 through this head is patch-compatible for the frozen 0.4.1 candidate.
All source changes are additive or widening; the only qualified-version
movement is the Contract 029 OpenCode HTTP ceiling. No breaking public API or
guaranteed-behaviour change was found. This worker did not edit Rust, Cargo,
changelog, contracts, releases, or release-baseline paths.

## Completion Protocol evidence

| Check | Result |
| --- | --- |
| remote / branch | `git@github.com:inflatable-cookie/swallowtail.git` / `worker/g05-card090-v0-4-1-compatibility-audit` |
| audited `HEAD` | `2187bbecb9e24425f574f4c6c434fd8f16fe0300` |
| `HEAD == origin/main` | yes after bounded fetch and fast-forward |
| planning base | `55d66a0e`; ancestor of `HEAD` |
| handoff blob | tracked handoff byte-identical to canonical absolute dispatch file |
| workspace | dedicated non-main worker; no review workspace |
| v0.4.0 tag object / peel | `6f398b9f0fedae4215ea7f58fdf04f888871e540` / `56f3913ac99af44b6ff45384cfc53a0adea587ba` |
| merge base | `56f3913ac99af44b6ff45384cfc53a0adea587ba` |
| range | 120 commits; 361 changed paths; `A 216`, `D 1`, `M 144` |
| inventory digests | name/status `2bc01f3cd7dd0414655521485b418d8b480fbc550cf44e3f17de1094ff1ad877`; commits `82b12f8418defe1120ae45b68e9f409a6698d3fa89f18e2529c22bae6f0ac482` |
| open PRs | `[]` for open PRs targeting `main` at preflight and pre-handoff |

`origin/main` advanced once during preparation with planning-only commit
`2187bbec` (prepare authorization and Bovine smoke acceptance). The worker
fast-forwarded before auditing; that commit changed four roadmap/log documents
only and did not change source, packages, routes, baselines, or owned paths.

## Package, dependency, and route census

- `cargo metadata --no-deps`: 40 unique packages; all versions `0.4.0`; all
  Rust requirements `1.95.0`.
- Package list: 40 lines, exact tag match; SHA-256
  `b6abf1f9218871ac15a6c4c9c057d4dc5518437109f4f06490ca45d13885a623`.
- Internal dependency ledger: 88 edges, exact tag/baseline match; SHA-256
  `eb3fa31c006b558d9f7c4d9c30c8469819c00c02495b84cc4477b74794ba2044`.
- Production route ledger: 49 rows, exact tag/baseline match; SHA-256
  `d6cd548f938ef9edcecb912ea173df593843f8d8abe0faed22184242be879f69`.
- Route additions/removals: 0/0. Route matrix and lifecycle rows: 49/49.
- Feature matrix: 41 rows and 49 fields; only `opencode.http`
  `guaranteed_version_posture` changed. Route/lifecycle row deltas are only
  `claude-agent.sdk` and `opencode.http`.
- Only manifest diffs are three added projection test targets in Antigravity,
  Cursor, and Gemini. No dependency, package, workspace, or lockfile delta.

### Package-by-package semantic API ledger

Contract 036 generation: `cargo-public-api 0.52.0`,
`nightly-2026-08-05` (`rustc 1.99.0-nightly (1ed2df61 2026-08-04)`),
`--all-features --simplified`, temporary output. All 40 generated outputs
match current references. Tag comparison: 244 added reference lines, 0
removed, across 14 files; 26 packages are byte-identical to tag.

| Package | Semantic API result |
| --- | --- |
| swallowtail-adapter-alibaba-model-studio | unchanged; generated API equals tag; no additions or removals |
| swallowtail-adapter-anthropic | unchanged; generated API equals tag; no additions or removals |
| swallowtail-adapter-antigravity | +6; 3 prepared contribution methods |
| swallowtail-adapter-bedrock | +4; 2 prepared contribution methods |
| swallowtail-adapter-claude-agent | +55; permission modes/tools/profiles/session open and handle APIs |
| swallowtail-adapter-cline | unchanged; generated API equals tag; no additions or removals |
| swallowtail-adapter-codex | unchanged; generated API equals tag; no additions or removals |
| swallowtail-adapter-command-code | unchanged; generated API equals tag; no additions or removals |
| swallowtail-adapter-copilot-cli | unchanged; generated API equals tag; no additions or removals |
| swallowtail-adapter-cursor | +6; 3 prepared contribution methods |
| swallowtail-adapter-deepagents | unchanged; generated API equals tag; no additions or removals |
| swallowtail-adapter-deepseek | +6; 3 prepared contribution methods |
| swallowtail-adapter-deepseek-harness | +14; contributions plus catalogue/history observations |
| swallowtail-adapter-gemini | +18; contributions plus projected-open APIs |
| swallowtail-adapter-goose | unchanged; generated API equals tag; no additions or removals |
| swallowtail-adapter-grok | +16; contributions plus projected-open APIs |
| swallowtail-adapter-kimi | +44; projection failure/outcome/future, provider value, contributions, observation, projected-open |
| swallowtail-adapter-kimi-platform | +4; 2 prepared contribution methods |
| swallowtail-adapter-kiro | unchanged; generated API equals tag; no additions or removals |
| swallowtail-adapter-llama-cpp | +6; 3 prepared contribution methods |
| swallowtail-adapter-mistral-vibe | unchanged; generated API equals tag; no additions or removals |
| swallowtail-adapter-muse | unchanged; generated API equals tag; no additions or removals |
| swallowtail-adapter-oh-my-pi | unchanged; generated API equals tag; no additions or removals |
| swallowtail-adapter-ollama | +6; 3 prepared contribution methods |
| swallowtail-adapter-openai | unchanged; generated API equals tag; no additions or removals |
| swallowtail-adapter-opencode | unchanged; generated API equals tag; no additions or removals |
| swallowtail-adapter-openhands | unchanged; generated API equals tag; no additions or removals |
| swallowtail-adapter-pi | unchanged; generated API equals tag; no additions or removals |
| swallowtail-adapter-qoder | unchanged; generated API equals tag; no additions or removals |
| swallowtail-adapter-qwen | unchanged; generated API equals tag; no additions or removals |
| swallowtail-adapter-xai | unchanged; generated API equals tag; no additions or removals |
| swallowtail-adapter-zcode | unchanged; generated API equals tag; no additions or removals |
| swallowtail-core | unchanged; generated API equals tag; no additions or removals |
| swallowtail-host-local | unchanged; generated API equals tag; no additions or removals |
| swallowtail-idioms | unchanged; generated API equals tag; no additions or removals |
| swallowtail-protocol-acp | unchanged; generated API equals tag; no additions or removals |
| swallowtail-protocol-openai-chat | unchanged; generated API equals tag; no additions or removals |
| swallowtail-runtime | +47; acknowledgement, provider-operation, compound-ack, projection, and evidence APIs |
| swallowtail-testkit | +12; 4 fixture cases and 8 assertions |
| swallowtail-transport-acp-remote | unchanged; generated API equals tag; no additions or removals |

Exact extracted direct public-item additions follow. The reference diffs are
244 lines: 177 direct `pub` lines plus implementation/context lines; no
removal exists.

```text
diff --git a/release-baselines/public-api-0.4.0/swallowtail-adapter-antigravity.txt b/release-baselines/public-api-0.4.0/swallowtail-adapter-antigravity.txt
+pub fn swallowtail_adapter_antigravity::AntigravityPreparedCatalogue::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_antigravity::AntigravityPreparedContinuation::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_antigravity::AntigravityPreparedHeadlessRun::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
diff --git a/release-baselines/public-api-0.4.0/swallowtail-adapter-bedrock.txt b/release-baselines/public-api-0.4.0/swallowtail-adapter-bedrock.txt
+pub fn swallowtail_adapter_bedrock::BedrockPreparedCatalogue::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_bedrock::BedrockPreparedInferenceAttempt::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
diff --git a/release-baselines/public-api-0.4.0/swallowtail-adapter-claude-agent.txt b/release-baselines/public-api-0.4.0/swallowtail-adapter-claude-agent.txt
+pub enum swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkPermissionMode
+pub swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkPermissionMode::AcceptEdits
+pub swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkPermissionMode::Default
+pub swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkPermissionMode::Plan
+pub const fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkPermissionMode::as_str(self) -> &'static str
+pub fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkPermissionMode::parse(&str) -> core::result::Result<Self, swallowtail_runtime::preparation::PreparationFailure>
+pub const fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkPermissionMode::skips_edit_admission(self) -> bool
+pub enum swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkTool
+pub swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkTool::Edit
+pub swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkTool::Glob
+pub swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkTool::Grep
+pub swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkTool::MultiEdit
+pub swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkTool::Read
+pub swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkTool::Write
+pub const fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkTool::as_str(self) -> &'static str
+pub const fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkTool::mutates_working_resource(self) -> bool
+pub fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkTool::parse(&str) -> core::option::Option<Self>
+pub const fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkDriver::session_profile(&self) -> swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionProfile
+pub const fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkDriver::with_session_profile(self, swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionProfile) -> Self
+pub fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkDriver::open_route_session(&self, swallowtail_core::preflight::PreflightPlan, swallowtail_runtime::roles::OpenSessionRequest, swallowtail_runtime::host_registry::HostServices) -> swallowtail_runtime::async_types::BoxFuture<'_, core::result::Result<swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle, swallowtail_runtime::failure::RuntimeFailure>>
+pub fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkPreparedSession::open_route_session(&self, swallowtail_runtime::host_registry::HostServices) -> swallowtail_runtime::async_types::BoxFuture<'static, core::result::Result<swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle, swallowtail_runtime::failure::RuntimeFailure>>
+pub const fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkPreparedSession::session_profile(&self) -> swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionProfile
+pub struct swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle
+pub const fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle::permission_mode(&self) -> swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkPermissionMode
+pub const fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle::session_profile(&self) -> swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionProfile
+pub fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle::set_permission_mode<'a>(&'a mut self, swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkPermissionMode, swallowtail_runtime::host_registry::HostServices, swallowtail_runtime::time::Deadline) -> swallowtail_runtime::async_types::BoxFuture<'a, core::result::Result<swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkPermissionMode, swallowtail_runtime::failure::RuntimeFailure>>
+pub fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle::drop(&mut self)
+pub fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle::cancellation(&self) -> &dyn swallowtail_runtime::cancellation::CancellationControl
+pub fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle::close(alloc::boxed::Box<Self>, swallowtail_runtime::session_cleanup::SessionCleanupRequest, swallowtail_runtime::host_registry::HostServices) -> swallowtail_runtime::async_types::BoxFuture<'static, swallowtail_runtime::outcome::CleanupOutcome>
+pub fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle::provider_session_ref(&self) -> core::option::Option<&swallowtail_core::provider_reference::SessionRef>
+pub fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle::request_id(&self) -> &swallowtail_runtime::identity::RequestId
+pub fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle::resume_binding(&self) -> core::option::Option<&swallowtail_runtime::session_binding::SessionResumeBinding>
+pub fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle::session_id(&self) -> &swallowtail_runtime::identity::RuntimeSessionId
+pub fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle::start_turn<'a>(&'a mut self, swallowtail_runtime::roles::TurnRequest, swallowtail_runtime::host_registry::HostServices) -> swallowtail_runtime::async_types::BoxFuture<'a, core::result::Result<alloc::boxed::Box<dyn swallowtail_runtime::handles::TurnHandle>, swallowtail_runtime::failure::RuntimeFailure>>
+pub const fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionPreparation::with_session_profile(self, swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionProfile) -> Self
+pub struct swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionProfile
+pub const fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionProfile::admits(&self, swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkTool) -> bool
+pub const fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionProfile::admits_writes(&self) -> bool
+pub fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionProfile::from_names<'a>(impl core::iter::traits::collect::IntoIterator<Item = &'a str>, &str) -> core::result::Result<Self, swallowtail_runtime::preparation::PreparationFailure>
+pub fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionProfile::new(impl core::iter::traits::collect::IntoIterator<Item = swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkTool>, swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkPermissionMode) -> core::result::Result<Self, swallowtail_runtime::preparation::PreparationFailure>
+pub const fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionProfile::permission_mode(&self) -> swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkPermissionMode
+pub const fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionProfile::read_only() -> Self
+pub const fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionProfile::read_write(swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkPermissionMode) -> Self
+pub const fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionProfile::resource_access(&self) -> swallowtail_core::session_access::ResourceAccess
+pub fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionProfile::tools(&self) -> impl core::iter::traits::iterator::Iterator<Item = swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkTool> + '_
+pub const fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionProfile::with_permission_mode(self, swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkPermissionMode) -> Self
+pub fn swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionProfile::default() -> Self
diff --git a/release-baselines/public-api-0.4.0/swallowtail-adapter-cursor.txt b/release-baselines/public-api-0.4.0/swallowtail-adapter-cursor.txt
+pub fn swallowtail_adapter_cursor::CursorPreparedAcpSession::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_cursor::CursorPreparedCatalogue::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_cursor::CursorPreparedHeadlessRun::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
diff --git a/release-baselines/public-api-0.4.0/swallowtail-adapter-deepseek-harness.txt b/release-baselines/public-api-0.4.0/swallowtail-adapter-deepseek-harness.txt
+pub fn swallowtail_adapter_deepseek_harness::DeepSeekHarnessPreparedRun::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_deepseek_harness::DeepSeekHarnessWebPreparedArchive::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_deepseek_harness::DeepSeekHarnessWebPreparedFork::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_deepseek_harness::DeepSeekHarnessWebPreparedRun::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_deepseek_harness::DeepSeekHarnessWebPreparedSessionCatalogue::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_deepseek_harness::DeepSeekHarnessWebPreparedSessionCatalogue::consumer_route_provider_operation_observation(&self, &swallowtail_runtime::provider_session_import::outcome::ProviderSessionCatalogueOutcome, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::provider_operation_observation::ConsumerRouteProviderOperationObservation, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_deepseek_harness::DeepSeekHarnessWebPreparedSessionHistory::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_deepseek_harness::DeepSeekHarnessWebPreparedSessionHistory::consumer_route_provider_operation_observation(&self, &swallowtail_runtime::provider_session_history::page::ProviderSessionHistoryPage, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::provider_operation_observation::ConsumerRouteProviderOperationObservation, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
diff --git a/release-baselines/public-api-0.4.0/swallowtail-adapter-deepseek.txt b/release-baselines/public-api-0.4.0/swallowtail-adapter-deepseek.txt
+pub fn swallowtail_adapter_deepseek::DeepSeekPreparedCatalogue::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_deepseek::DeepSeekPreparedRun::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_deepseek::DeepSeekPreparedSession::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
diff --git a/release-baselines/public-api-0.4.0/swallowtail-adapter-gemini.txt b/release-baselines/public-api-0.4.0/swallowtail-adapter-gemini.txt
+pub enum swallowtail_adapter_gemini::GeminiProjectionOpenFailure
+pub swallowtail_adapter_gemini::GeminiProjectionOpenFailure::Runtime(swallowtail_runtime::failure::RuntimeFailure)
+pub const fn swallowtail_adapter_gemini::GeminiProjectionOpenFailure::failure(&self) -> &swallowtail_runtime::failure::RuntimeFailure
+pub fn swallowtail_adapter_gemini::GeminiHeadlessPreparedRun::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_gemini::GeminiPreparedLiveSession::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_gemini::GeminiPreparedSession::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_gemini::GeminiPreparedSession::open_session_with_projection(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId, swallowtail_runtime::session_cleanup::SessionCleanupRequest, swallowtail_runtime::host_registry::HostServices) -> swallowtail_adapter_gemini::GeminiProjectionOpenFuture
+pub struct swallowtail_adapter_gemini::GeminiProjectionOpenOutcome
+pub const fn swallowtail_adapter_gemini::GeminiProjectionOpenOutcome::contribution(&self) -> &swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution
+pub fn swallowtail_adapter_gemini::GeminiProjectionOpenOutcome::into_parts(self) -> (alloc::boxed::Box<dyn swallowtail_runtime::handles::InteractiveSessionHandle>, swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution)
+pub fn swallowtail_adapter_gemini::GeminiProjectionOpenOutcome::negotiated_model_options(&self) -> core::option::Option<&swallowtail_runtime::negotiated_session_options::NegotiatedSessionModelOptions>
+pub fn swallowtail_adapter_gemini::GeminiProjectionOpenOutcome::session(&self) -> &dyn swallowtail_runtime::handles::InteractiveSessionHandle
+pub type swallowtail_adapter_gemini::GeminiProjectionOpenFuture = swallowtail_runtime::async_types::BoxFuture<'static, core::result::Result<swallowtail_adapter_gemini::GeminiProjectionOpenOutcome, swallowtail_adapter_gemini::GeminiProjectionOpenFailure>>
diff --git a/release-baselines/public-api-0.4.0/swallowtail-adapter-grok.txt b/release-baselines/public-api-0.4.0/swallowtail-adapter-grok.txt
+pub enum swallowtail_adapter_grok::GrokProjectionOpenFailure
+pub swallowtail_adapter_grok::GrokProjectionOpenFailure::Runtime(swallowtail_runtime::failure::RuntimeFailure)
+pub const fn swallowtail_adapter_grok::GrokProjectionOpenFailure::failure(&self) -> &swallowtail_runtime::failure::RuntimeFailure
+pub fn swallowtail_adapter_grok::GrokPreparedRun::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_grok::GrokPreparedSession::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_grok::GrokPreparedSession::open_session_with_projection(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId, swallowtail_runtime::session_cleanup::SessionCleanupRequest, swallowtail_runtime::host_registry::HostServices) -> swallowtail_adapter_grok::GrokProjectionOpenFuture
+pub struct swallowtail_adapter_grok::GrokProjectionOpenOutcome
+pub const fn swallowtail_adapter_grok::GrokProjectionOpenOutcome::contribution(&self) -> &swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution
+pub fn swallowtail_adapter_grok::GrokProjectionOpenOutcome::into_parts(self) -> (alloc::boxed::Box<dyn swallowtail_runtime::handles::InteractiveSessionHandle>, swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution)
+pub fn swallowtail_adapter_grok::GrokProjectionOpenOutcome::negotiated_model_options(&self) -> core::option::Option<&swallowtail_runtime::negotiated_session_options::NegotiatedSessionModelOptions>
+pub fn swallowtail_adapter_grok::GrokProjectionOpenOutcome::session(&self) -> &dyn swallowtail_runtime::handles::InteractiveSessionHandle
+pub type swallowtail_adapter_grok::GrokProjectionOpenFuture = swallowtail_runtime::async_types::BoxFuture<'static, core::result::Result<swallowtail_adapter_grok::GrokProjectionOpenOutcome, swallowtail_adapter_grok::GrokProjectionOpenFailure>>
diff --git a/release-baselines/public-api-0.4.0/swallowtail-adapter-kimi-platform.txt b/release-baselines/public-api-0.4.0/swallowtail-adapter-kimi-platform.txt
+pub fn swallowtail_adapter_kimi_platform::KimiPlatformPreparedCatalogue::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_kimi_platform::KimiPlatformPreparedInferenceAttempt::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
diff --git a/release-baselines/public-api-0.4.0/swallowtail-adapter-kimi.txt b/release-baselines/public-api-0.4.0/swallowtail-adapter-kimi.txt
+pub enum swallowtail_adapter_kimi::KimiProjectionOpenFailure
+pub swallowtail_adapter_kimi::KimiProjectionOpenFailure::Rejected
+pub swallowtail_adapter_kimi::KimiProjectionOpenFailure::Rejected::contribution: swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution
+pub swallowtail_adapter_kimi::KimiProjectionOpenFailure::Rejected::failure: swallowtail_runtime::failure::RuntimeFailure
+pub swallowtail_adapter_kimi::KimiProjectionOpenFailure::Runtime(swallowtail_runtime::failure::RuntimeFailure)
+pub const fn swallowtail_adapter_kimi::KimiProjectionOpenFailure::failure(&self) -> &swallowtail_runtime::failure::RuntimeFailure
+pub fn swallowtail_adapter_kimi::KimiProjectionOpenFailure::into_parts(self) -> (swallowtail_runtime::failure::RuntimeFailure, core::option::Option<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution>)
+pub const fn swallowtail_adapter_kimi::KimiProjectionOpenFailure::rejected_contribution(&self) -> core::option::Option<&swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution>
+pub fn swallowtail_adapter_kimi::KimiHeadlessPreparedRun::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_kimi::KimiLocalServerPreparedArchive::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_kimi::KimiLocalServerPreparedBindingImport::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_kimi::KimiLocalServerPreparedCatalogue::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_kimi::KimiLocalServerPreparedReconciliation::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_kimi::KimiLocalServerPreparedRestore::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_kimi::KimiLocalServerPreparedRun::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_kimi::KimiLocalServerPreparedSession::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_kimi::KimiPreparedSession::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_kimi::KimiPreparedSession::open_session_with_projection(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId, swallowtail_runtime::host_registry::HostServices) -> swallowtail_adapter_kimi::KimiProjectionOpenFuture
+pub fn swallowtail_adapter_kimi::KimiPreparedSessionCatalogue::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_kimi::KimiPreparedSessionCatalogue::consumer_route_provider_operation_observation(&self, &swallowtail_runtime::provider_session_import::outcome::ProviderSessionCatalogueOutcome, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::provider_operation_observation::ConsumerRouteProviderOperationObservation, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_kimi::KimiPreparedSessionImport::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub struct swallowtail_adapter_kimi::KimiProjectionOpenOutcome
+pub const fn swallowtail_adapter_kimi::KimiProjectionOpenOutcome::contribution(&self) -> &swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution
+pub fn swallowtail_adapter_kimi::KimiProjectionOpenOutcome::into_parts(self) -> (alloc::boxed::Box<dyn swallowtail_runtime::handles::InteractiveSessionHandle>, swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution)
+pub fn swallowtail_adapter_kimi::KimiProjectionOpenOutcome::negotiated_model_options(&self) -> core::option::Option<&swallowtail_runtime::negotiated_session_options::NegotiatedSessionModelOptions>
+pub fn swallowtail_adapter_kimi::KimiProjectionOpenOutcome::session(&self) -> &dyn swallowtail_runtime::handles::InteractiveSessionHandle
+pub struct swallowtail_adapter_kimi::KimiProviderValue(_)
+pub fn swallowtail_adapter_kimi::KimiProviderValue::as_str(&self) -> &str
+pub type swallowtail_adapter_kimi::KimiProjectionOpenFuture = swallowtail_runtime::async_types::BoxFuture<'static, core::result::Result<swallowtail_adapter_kimi::KimiProjectionOpenOutcome, swallowtail_adapter_kimi::KimiProjectionOpenFailure>>
diff --git a/release-baselines/public-api-0.4.0/swallowtail-adapter-llama-cpp.txt b/release-baselines/public-api-0.4.0/swallowtail-adapter-llama-cpp.txt
+pub fn swallowtail_adapter_llama_cpp::LlamaCppPreparedCatalogue::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_llama_cpp::LlamaCppPreparedInferenceAttempt::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_llama_cpp::LlamaCppPreparedServingStart::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
diff --git a/release-baselines/public-api-0.4.0/swallowtail-adapter-ollama.txt b/release-baselines/public-api-0.4.0/swallowtail-adapter-ollama.txt
+pub fn swallowtail_adapter_ollama::OllamaPreparedInferenceAttempt::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_ollama::OllamaPreparedInventory::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
+pub fn swallowtail_adapter_ollama::OllamaPreparedSession::consumer_route_projection_contribution(&self, swallowtail_runtime::consumer_route_projection::identity::ConsumerRouteProjectionSourceId) -> core::result::Result<swallowtail_runtime::consumer_route_projection::contribution::ConsumerRouteProjectionContribution, swallowtail_runtime::consumer_route_projection::failure::ConsumerRouteProjectionFailure>
diff --git a/release-baselines/public-api-0.4.0/swallowtail-runtime.txt b/release-baselines/public-api-0.4.0/swallowtail-runtime.txt
+pub enum swallowtail_runtime::ConsumerRouteAcknowledgementState
+pub swallowtail_runtime::ConsumerRouteAcknowledgementState::Absent
+pub swallowtail_runtime::ConsumerRouteAcknowledgementState::Effective(swallowtail_runtime::ConsumerRouteEnumerableValue)
+pub swallowtail_runtime::ConsumerRouteAcknowledgementState::Rejected(swallowtail_runtime::ConsumerRouteEnumerableValue)
+pub swallowtail_runtime::ConsumerRouteAcknowledgementState::RequestedNotDispatched
+pub const fn swallowtail_runtime::ConsumerRouteAcknowledgementState::absent() -> Self
+pub fn swallowtail_runtime::ConsumerRouteAcknowledgementState::effective(swallowtail_runtime::ConsumerRouteEnumerableValue) -> Self
+pub fn swallowtail_runtime::ConsumerRouteAcknowledgementState::rejected(swallowtail_runtime::ConsumerRouteEnumerableValue) -> Self
+pub const fn swallowtail_runtime::ConsumerRouteAcknowledgementState::requested_not_dispatched() -> Self
+pub swallowtail_runtime::ConsumerRouteEvidenceStrength::CompletedProviderOperation
+pub swallowtail_runtime::ConsumerRouteLifecycle::PostOperationObservationOnly
+pub swallowtail_runtime::ConsumerRouteProjectionFailureKind::ProviderOperationObservationInvalid
+pub swallowtail_runtime::ConsumerRouteProjectionSourceKind::ProviderOperationObservation
+pub enum swallowtail_runtime::ConsumerRouteProviderOperationOutcome<'a>
+pub swallowtail_runtime::ConsumerRouteProviderOperationOutcome::ProviderSessionCatalogue(&'a swallowtail_runtime::ProviderSessionCatalogueOutcome)
+pub swallowtail_runtime::ConsumerRouteProviderOperationOutcome::ProviderSessionHistory(&'a swallowtail_runtime::ProviderSessionHistoryPage)
+pub swallowtail_runtime::ConsumerRouteSourceClass::ProviderOperationOutcome
+pub struct swallowtail_runtime::ConsumerRouteCompoundAcknowledgement
+pub fn swallowtail_runtime::ConsumerRouteCompoundAcknowledgement::new(swallowtail_runtime::ConsumerRouteAcknowledgementState, swallowtail_runtime::ConsumerRouteAcknowledgementState) -> core::result::Result<Self, swallowtail_runtime::ConsumerRouteProjectionFailure>
+pub const fn swallowtail_runtime::ConsumerRouteCompoundAcknowledgement::plan(&self) -> &swallowtail_runtime::ConsumerRouteAcknowledgementState
+pub const fn swallowtail_runtime::ConsumerRouteCompoundAcknowledgement::reasoning(&self) -> &swallowtail_runtime::ConsumerRouteAcknowledgementState
+pub const fn swallowtail_runtime::ConsumerRouteProjection::provider_operation_state(&self) -> &swallowtail_runtime::ConsumerRouteProviderOperationState
+pub fn swallowtail_runtime::ConsumerRouteProjectionInput<'a>::with_provider_operation_observations(self, impl core::iter::traits::collect::IntoIterator<Item = &'a swallowtail_runtime::ConsumerRouteProviderOperationObservation>) -> Self
+pub const fn swallowtail_runtime::ConsumerRouteProjectionRow::compound_acknowledgement(&self) -> core::option::Option<&swallowtail_runtime::ConsumerRouteCompoundAcknowledgement>
+pub fn swallowtail_runtime::ConsumerRouteProjectionRow::with_compound_acknowledgement(self, swallowtail_runtime::ConsumerRouteCompoundAcknowledgement) -> Self
+pub struct swallowtail_runtime::ConsumerRouteProviderOperationObservation
+pub const fn swallowtail_runtime::ConsumerRouteProviderOperationObservation::applicability(&self) -> &swallowtail_runtime::ConsumerRouteApplicability
+pub fn swallowtail_runtime::ConsumerRouteProviderOperationObservation::new(&swallowtail_runtime::PreparedOperationEvidence, swallowtail_runtime::ConsumerRouteProviderOperationOutcome<'_>, swallowtail_runtime::ConsumerRouteProjectionSourceIdentity, impl core::iter::traits::collect::IntoIterator<Item = swallowtail_runtime::ConsumerRouteProjectionRow>) -> core::result::Result<Self, swallowtail_runtime::ConsumerRouteProjectionFailure>
+pub fn swallowtail_runtime::ConsumerRouteProviderOperationObservation::rows(&self) -> impl core::iter::traits::exact_size::ExactSizeIterator<Item = &swallowtail_runtime::ConsumerRouteProjectionRow>
+pub const fn swallowtail_runtime::ConsumerRouteProviderOperationObservation::source(&self) -> &swallowtail_runtime::ConsumerRouteProjectionSourceIdentity
+pub struct swallowtail_runtime::ConsumerRouteProviderOperationState
+pub fn swallowtail_runtime::ConsumerRouteProviderOperationState::rows(&self) -> impl core::iter::traits::exact_size::ExactSizeIterator<Item = &swallowtail_runtime::ConsumerRouteProjectionRow>
+pub fn swallowtail_runtime::ProviderSessionCatalogueOutcome::eq(&self, &Self) -> bool
+pub fn swallowtail_runtime::ProviderSessionCatalogueOutcome::fmt(&self, &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn swallowtail_runtime::ProviderSessionHistoryPage::eq(&self, &Self) -> bool
+pub fn swallowtail_runtime::ProviderSessionHistoryPage::fmt(&self, &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub const swallowtail_runtime::MAX_CONSUMER_ROUTE_PROVIDER_OPERATION_ROWS: usize
diff --git a/release-baselines/public-api-0.4.0/swallowtail-testkit.txt b/release-baselines/public-api-0.4.0/swallowtail-testkit.txt
+pub swallowtail_testkit::SessionAccessFixtureCase::AmbientMediatedReadWriteWithToolCalls
+pub swallowtail_testkit::SessionAccessFixtureCase::AmbientReadWriteWithToolCalls
+pub swallowtail_testkit::SessionAccessFixtureCase::BoundedWorkspaceWithToolCalls
+pub swallowtail_testkit::SessionAccessFixtureCase::ReadOnlyWithToolCalls
+pub fn swallowtail_testkit::assert_compound_acknowledgement_associates_each_half_state()
+pub fn swallowtail_testkit::assert_compound_acknowledgement_preserves_exact_provider_values()
+pub fn swallowtail_testkit::assert_compound_acknowledgement_preserves_reasoning_first_order()
+pub fn swallowtail_testkit::assert_compound_acknowledgement_rejects_impossible_half_combinations()
+pub fn swallowtail_testkit::assert_compound_acknowledgement_requires_observation_source()
+pub fn swallowtail_testkit::assert_compound_acknowledgement_terminal_not_dispatched_is_distinct()
+pub fn swallowtail_testkit::assert_consumer_route_provider_operation_observation_contract()
+pub fn swallowtail_testkit::assert_consumer_tool_exclusion_keys_on_boundary_claim()
```

Current additive API reference files, owned by earlier cards and not written
by this worker:
- `release-baselines/public-api-0.4.0/swallowtail-adapter-antigravity.txt (+6)`
- `release-baselines/public-api-0.4.0/swallowtail-adapter-bedrock.txt (+4)`
- `release-baselines/public-api-0.4.0/swallowtail-adapter-claude-agent.txt (+55)`
- `release-baselines/public-api-0.4.0/swallowtail-adapter-cursor.txt (+6)`
- `release-baselines/public-api-0.4.0/swallowtail-adapter-deepseek-harness.txt (+14)`
- `release-baselines/public-api-0.4.0/swallowtail-adapter-deepseek.txt (+6)`
- `release-baselines/public-api-0.4.0/swallowtail-adapter-gemini.txt (+18)`
- `release-baselines/public-api-0.4.0/swallowtail-adapter-grok.txt (+16)`
- `release-baselines/public-api-0.4.0/swallowtail-adapter-kimi-platform.txt (+4)`
- `release-baselines/public-api-0.4.0/swallowtail-adapter-kimi.txt (+44)`
- `release-baselines/public-api-0.4.0/swallowtail-adapter-llama-cpp.txt (+6)`
- `release-baselines/public-api-0.4.0/swallowtail-adapter-ollama.txt (+6)`
- `release-baselines/public-api-0.4.0/swallowtail-runtime.txt (+47)`
- `release-baselines/public-api-0.4.0/swallowtail-testkit.txt (+12)`

### Route ledger

| Route | Delta classification | Evidence |
| --- | --- | --- |
| alibaba.conversations | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| anthropic.managed-agent | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| anthropic.messages | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| antigravity.catalogue | C contribution | additive/qualification; existing route retained |
| antigravity.headless | C contribution | additive/qualification; existing route retained |
| bedrock.catalogue | C contribution | additive/qualification; existing route retained |
| bedrock.runtime | C contribution | additive/qualification; existing route retained |
| claude-agent.acp | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| claude-agent.sdk | Claude explicit profile; default unchanged | additive/qualification; existing route retained |
| claude-code.headless | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| claude-code.response-only | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| cline.acp | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| cline.headless | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| codex.app-server | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| codex.exec | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| command-code.headless | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| copilot-cli.acp | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| cursor-agent.acp | C contribution | additive/qualification; existing route retained |
| cursor-agent.catalogue | C contribution | additive/qualification; existing route retained |
| cursor-agent.headless | C contribution | additive/qualification; existing route retained |
| deepagents.acp | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| deepseek-harness.jsonrpc | I contribution | additive/qualification; existing route retained |
| deepseek-harness.local-server | I contribution | additive/qualification; existing route retained |
| deepseek.continuation | I contribution | additive/qualification; existing route retained |
| gemini-cli.acp | E contribution | additive/qualification; existing route retained |
| gemini-cli.headless | E contribution | additive/qualification; existing route retained |
| gemini.live | E contribution | additive/qualification; existing route retained |
| goose.acp | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| grok-build.acp | E contribution | additive/qualification; existing route retained |
| kimi-code.acp | F contribution | additive/qualification; existing route retained |
| kimi-code.headless | F contribution | additive/qualification; existing route retained |
| kimi-code.local-server | F contribution | additive/qualification; existing route retained |
| kimi-platform.chat | F contribution | additive/qualification; existing route retained |
| kiro.acp | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| llama-cpp.attached | J contribution | additive/qualification; existing route retained |
| llama-cpp.owned | J contribution | additive/qualification; existing route retained |
| mistral-vibe.headless | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| muse-code.headless | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| oh-my-pi.rpc | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| ollama.attached | J contribution | additive/qualification; existing route retained |
| openai.background | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| openai.realtime | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| opencode.http | Contract 029 ceiling 1.18.20 → 1.18.28 | additive/qualification; existing route retained |
| pi.rpc | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| pi.sdk-sidecar | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| qoder.headless | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| qwen.headless | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| xai.responses-websocket | unchanged | route, feature, lifecycle, and guaranteed values equal tag |
| zcode.app-server | unchanged | route, feature, lifecycle, and guaranteed values equal tag |

## Guaranteed-behaviour ledger

| Item | Audited guarantee and class | Consumer/release effect |
| --- | --- | --- |
| Candidate C: 7 routes | 94 tuples: 51 emitted, 43 withheld; prepared contribution seams are route-qualified and additive | Existing behavior retained; current Unreleased Candidate C coverage |
| Candidate E: 4 routes | 56 tuples: 39 emitted, 17 withheld; adapter-local projected-open preserves existing open | Additive; no separate Unreleased entry, so Card 091 must consolidate |
| Candidate F: 4 routes | 89 tuples: 75 emitted, 14 withheld; Kimi observation/projected-open preserves exact reasoning/Plan and model evidence | Additive; current Unreleased coverage |
| Candidate I: 3 routes | 47 tuples: 41 emitted, 6 withheld; completed catalogue/history observations only; execution/paging/cleanup unchanged | Additive; no separate Unreleased entry, so Card 091 must consolidate |
| Candidate J: 3 routes | 35 tuples: 32 emitted, 3 withheld; contribution seams only | Additive; current Unreleased coverage |
| provider-operation observation | Runtime/testkit view, max four rows, completed catalogue/history only; no active control/request authority | Additive observation-only; adapter methods are opt-in |
| compound acknowledgement | Independent Plan/reasoning states; exact provider values only effective/rejected; `RequestedNotDispatched` distinct from pending | Additive observation-only; no request/mutation/routing authority |
| Card 089 preflight | Bounded writable rejection remains; ambient and ambient-mediated read-write tool calls now pass; no previously passing plan fails | Additive widening; current Unreleased Changed entry |
| `claude-agent.sdk` default | `Read`/`Glob`/`Grep`, `default`, access policy, and instance policy id are behaviorally identical; explicit profiles are opt-in | Additive; current Unreleased records parity/write profile; no existing default break |
| `opencode.http` | Qualified `1.14.48..=1.18.20` → `..=1.18.28`; hops `1.18.21`–`1.18.28` retain surface-19 operations/lifecycle/cleanup | Contract 029 qualification; no package/API change; Card 091 must promote release-note coverage |
| currentness stops | Kimi local-server cap `0.38.0`, Antigravity cap `1.1.17`, Gemini deferred; newer observations unverified | Unchanged negative evidence; no unsupported claim |

Negative audit: no route was renamed/removed; no control authority, early
provider work, cleanup guarantee, provider-token flattening, or consumer-facing
permission was introduced.

## Immutable-baseline proof

`v0.4.0` has 217 release-baseline paths. Byte comparison of the 203 paths other
than the 14 current additive API reference files: 0 mismatches. Package list,
dependency ledger, and route ledger are exact tag matches. The 14 current API
reference files retain their historical tag bytes plus 244 authorized additive
lines and 0 removals; historical evidence remains available with
`git show v0.4.0:<path>`. No baseline was regenerated in place.

## Release posture

`effigy --json release status`: schema `effigy.release.status.v1`; `ok: true`,
`ready: true`, `blockers: []`, `current_version: 0.4.0`,
`next_version: 0.4.1`, `suggested_bump: patch`, `tag: v0.4.1`, valid changelog,
and seven unreleased entries (`Added: 6`, `Changed: 1`). This satisfies the
required inferred patch release. Census is 40 packages, 49 routes, Rust
`1.95.0`, Apple Silicon macOS, source-only annotated-tag intent. No prepare,
tag, publication, provider call, consumer mutation, or merge occurred.

## Review oracle

Invariant: one exact tree supports every compatibility statement. The smallest
counterexample is a changed public item or guaranteed value absent from the
ledger, or a break classified compatible. The package table, exact API block,
route table, behavior ledger, tag comparisons, and full inventory are the
review evidence.

## Full v0.4.0..HEAD name/status inventory

Digest: `2bc01f3cd7dd0414655521485b418d8b480fbc550cf44e3f17de1094ff1ad877`.

```text
M	.cursor/skills/version-currentness/SKILL.md
M	.cursor/skills/version-currentness/reference.md
M	AGENTS.md
M	CHANGELOG.md
A	CLAUDE.md
M	PAPERCUTS.md
M	README.md
M	crates/swallowtail-adapter-antigravity/Cargo.toml
A	crates/swallowtail-adapter-antigravity/src/consumer_route_projection.rs
M	crates/swallowtail-adapter-antigravity/src/lib.rs
A	crates/swallowtail-adapter-antigravity/tests/antigravity_1_1_26_delta_ledger.rs
M	crates/swallowtail-adapter-antigravity/tests/antigravity_release_identity.rs
A	crates/swallowtail-adapter-antigravity/tests/consumer_route_projection.rs
A	crates/swallowtail-adapter-antigravity/tests/consumer_route_projection/claims.rs
A	crates/swallowtail-adapter-antigravity/tests/consumer_route_projection/fixtures.rs
A	crates/swallowtail-adapter-antigravity/tests/consumer_route_projection/ledger.rs
A	crates/swallowtail-adapter-antigravity/tests/consumer_route_projection/ledger/catalogue.rs
A	crates/swallowtail-adapter-antigravity/tests/consumer_route_projection/ledger/headless.rs
A	crates/swallowtail-adapter-antigravity/tests/consumer_route_projection/naming.rs
A	crates/swallowtail-adapter-antigravity/tests/consumer_route_projection/proof.rs
A	crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.1.26/README.md
A	crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.1.26/dist-inventory.json
A	crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.1.26/identity.json
A	crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.1.26/protocol.json
A	crates/swallowtail-adapter-bedrock/src/consumer_route_projection.rs
M	crates/swallowtail-adapter-bedrock/src/lib.rs
A	crates/swallowtail-adapter-bedrock/tests/consumer_route_projection.rs
A	crates/swallowtail-adapter-bedrock/tests/consumer_route_projection/fixtures.rs
A	crates/swallowtail-adapter-bedrock/tests/consumer_route_projection/ledger.rs
M	crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs
M	crates/swallowtail-adapter-claude-agent/src/sdk.rs
M	crates/swallowtail-adapter-claude-agent/src/sdk/driver.rs
M	crates/swallowtail-adapter-claude-agent/src/sdk/driver/launch.rs
M	crates/swallowtail-adapter-claude-agent/src/sdk/driver/session.rs
M	crates/swallowtail-adapter-claude-agent/src/sdk/driver/startup.rs
M	crates/swallowtail-adapter-claude-agent/src/sdk/driver/validation.rs
M	crates/swallowtail-adapter-claude-agent/src/sdk/prepared.rs
M	crates/swallowtail-adapter-claude-agent/src/sdk/prepared/build.rs
A	crates/swallowtail-adapter-claude-agent/src/sdk/profile.rs
A	crates/swallowtail-adapter-claude-agent/src/sdk/profile/profile_tests.rs
M	crates/swallowtail-adapter-claude-agent/src/sdk/turn.rs
M	crates/swallowtail-adapter-claude-agent/src/sdk/wire.rs
M	crates/swallowtail-adapter-claude-agent/tests/claude_agent_sdk_driver.rs
A	crates/swallowtail-adapter-claude-agent/tests/claude_agent_sdk_driver/permission.rs
M	crates/swallowtail-adapter-claude-agent/tests/claude_agent_sdk_driver/readiness.rs
M	crates/swallowtail-adapter-claude-agent/tests/claude_agent_sdk_identity.rs
M	crates/swallowtail-adapter-claude-agent/tests/claude_agent_sdk_sidecar_asset.rs
M	crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-v1/commands.jsonl
M	crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-v1/protocol.json
M	crates/swallowtail-adapter-claude-agent/tests/sdk_support/host.rs
M	crates/swallowtail-adapter-claude-agent/tests/sdk_support/host/authority.rs
M	crates/swallowtail-adapter-claude-agent/tests/sdk_support/host/scenario.rs
M	crates/swallowtail-adapter-claude-agent/tests/sdk_support/host/script.rs
M	crates/swallowtail-adapter-claude-agent/tests/sdk_support/mod.rs
M	crates/swallowtail-adapter-claude-agent/tests/sdk_support/selection.rs
M	crates/swallowtail-adapter-claude-agent/tests/sidecar_asset_support/fake-sdk.mjs
M	crates/swallowtail-adapter-claude-agent/tests/sidecar_asset_support/mod.rs
M	crates/swallowtail-adapter-cursor/Cargo.toml
A	crates/swallowtail-adapter-cursor/src/consumer_route_projection.rs
A	crates/swallowtail-adapter-cursor/src/consumer_route_projection/builder.rs
M	crates/swallowtail-adapter-cursor/src/headless_model_parameters.rs
M	crates/swallowtail-adapter-cursor/src/lib.rs
A	crates/swallowtail-adapter-cursor/tests/consumer_route_projection.rs
A	crates/swallowtail-adapter-cursor/tests/consumer_route_projection/claims.rs
A	crates/swallowtail-adapter-cursor/tests/consumer_route_projection/fixtures.rs
A	crates/swallowtail-adapter-cursor/tests/consumer_route_projection/ledger.rs
A	crates/swallowtail-adapter-cursor/tests/consumer_route_projection/ledger/acp.rs
A	crates/swallowtail-adapter-cursor/tests/consumer_route_projection/ledger/catalogue.rs
A	crates/swallowtail-adapter-cursor/tests/consumer_route_projection/ledger/headless.rs
A	crates/swallowtail-adapter-cursor/tests/consumer_route_projection/naming.rs
A	crates/swallowtail-adapter-cursor/tests/consumer_route_projection/proof.rs
A	crates/swallowtail-adapter-deepseek-harness/src/consumer_route_projection.rs
A	crates/swallowtail-adapter-deepseek-harness/src/consumer_route_projection/builder.rs
A	crates/swallowtail-adapter-deepseek-harness/src/consumer_route_projection/tests.rs
A	crates/swallowtail-adapter-deepseek-harness/src/consumer_route_projection/tests/support.rs
M	crates/swallowtail-adapter-deepseek-harness/src/lib.rs
M	crates/swallowtail-adapter-deepseek-harness/src/prepared.rs
M	crates/swallowtail-adapter-deepseek-harness/src/web_prepared.rs
A	crates/swallowtail-adapter-deepseek/src/consumer_route_projection.rs
A	crates/swallowtail-adapter-deepseek/src/consumer_route_projection/builder.rs
M	crates/swallowtail-adapter-deepseek/src/lib.rs
A	crates/swallowtail-adapter-deepseek/tests/consumer_route_projection.rs
M	crates/swallowtail-adapter-gemini/Cargo.toml
A	crates/swallowtail-adapter-gemini/src/consumer_route_projection.rs
M	crates/swallowtail-adapter-gemini/src/lib.rs
A	crates/swallowtail-adapter-gemini/tests/consumer_route_projection.rs
M	crates/swallowtail-adapter-gemini/tests/prepared_facade.rs
A	crates/swallowtail-adapter-grok/src/consumer_route_projection.rs
M	crates/swallowtail-adapter-grok/src/lib.rs
A	crates/swallowtail-adapter-grok/tests/acp/consumer_route_projection.rs
M	crates/swallowtail-adapter-grok/tests/acp/support.rs
M	crates/swallowtail-adapter-grok/tests/installed_probe.rs
A	crates/swallowtail-adapter-kimi-platform/src/consumer_route_projection.rs
M	crates/swallowtail-adapter-kimi-platform/src/lib.rs
M	crates/swallowtail-adapter-kimi-platform/tests/prepared_facade.rs
A	crates/swallowtail-adapter-kimi/src/consumer_route_projection.rs
A	crates/swallowtail-adapter-kimi/src/consumer_route_projection/builder.rs
A	crates/swallowtail-adapter-kimi/src/consumer_route_projection/builder/active.rs
A	crates/swallowtail-adapter-kimi/src/consumer_route_projection/builder/capability.rs
A	crates/swallowtail-adapter-kimi/src/consumer_route_projection/builder/control.rs
A	crates/swallowtail-adapter-kimi/src/consumer_route_projection/contribution.rs
A	crates/swallowtail-adapter-kimi/src/consumer_route_projection/contribution/local.rs
A	crates/swallowtail-adapter-kimi/src/consumer_route_projection/open.rs
A	crates/swallowtail-adapter-kimi/src/consumer_route_projection/open/admission.rs
A	crates/swallowtail-adapter-kimi/src/consumer_route_projection/open/types.rs
M	crates/swallowtail-adapter-kimi/src/driver.rs
M	crates/swallowtail-adapter-kimi/src/driver/mode.rs
A	crates/swallowtail-adapter-kimi/src/driver/open.rs
M	crates/swallowtail-adapter-kimi/src/driver/reasoning.rs
M	crates/swallowtail-adapter-kimi/src/lib.rs
M	crates/swallowtail-adapter-kimi/src/local_server/interactive.rs
M	crates/swallowtail-adapter-kimi/src/local_server/prepared/import.rs
M	crates/swallowtail-adapter-kimi/src/local_server/prepared/import/validation.rs
M	crates/swallowtail-adapter-kimi/tests/acp_suite.rs
A	crates/swallowtail-adapter-kimi/tests/consumer_route_projection.rs
A	crates/swallowtail-adapter-kimi/tests/consumer_route_projection/catalogue.rs
A	crates/swallowtail-adapter-kimi/tests/consumer_route_projection/foreign.rs
A	crates/swallowtail-adapter-kimi/tests/consumer_route_projection/ledger.rs
A	crates/swallowtail-adapter-kimi/tests/fixtures/kimi-local-server-0.41.0/README.md
A	crates/swallowtail-adapter-kimi/tests/fixtures/kimi-local-server-0.41.0/identity.json
A	crates/swallowtail-adapter-kimi/tests/fixtures/kimi-local-server-0.41.0/protocol.json
A	crates/swallowtail-adapter-kimi/tests/headless_projection.rs
M	crates/swallowtail-adapter-kimi/tests/headless_suite.rs
M	crates/swallowtail-adapter-kimi/tests/identity_suite.rs
A	crates/swallowtail-adapter-kimi/tests/kimi_local_server_0_41_0_identity.rs
A	crates/swallowtail-adapter-kimi/tests/kimi_local_server_0_41_0_identity/artifacts.rs
A	crates/swallowtail-adapter-kimi/tests/kimi_local_server_0_41_0_identity/authority.rs
A	crates/swallowtail-adapter-kimi/tests/kimi_local_server_0_41_0_identity/claims.rs
A	crates/swallowtail-adapter-kimi/tests/kimi_local_server_0_41_0_identity/protocol.rs
A	crates/swallowtail-adapter-kimi/tests/kimi_local_server_0_41_0_identity/support.rs
A	crates/swallowtail-adapter-kimi/tests/local_server_projection.rs
M	crates/swallowtail-adapter-kimi/tests/local_suite.rs
M	crates/swallowtail-adapter-kimi/tests/support/agent.rs
M	crates/swallowtail-adapter-kimi/tests/support/agent/config.rs
M	crates/swallowtail-adapter-kimi/tests/support/agent/reasoning.rs
A	crates/swallowtail-adapter-llama-cpp/src/consumer_route_projection.rs
A	crates/swallowtail-adapter-llama-cpp/src/consumer_route_projection/builder.rs
M	crates/swallowtail-adapter-llama-cpp/src/lib.rs
A	crates/swallowtail-adapter-llama-cpp/tests/consumer_route_projection.rs
A	crates/swallowtail-adapter-llama-cpp/tests/consumer_route_projection/assembly.rs
A	crates/swallowtail-adapter-llama-cpp/tests/consumer_route_projection/claims.rs
A	crates/swallowtail-adapter-llama-cpp/tests/consumer_route_projection/controls.rs
A	crates/swallowtail-adapter-llama-cpp/tests/consumer_route_projection/fixtures.rs
A	crates/swallowtail-adapter-llama-cpp/tests/consumer_route_projection/ledger.rs
A	crates/swallowtail-adapter-llama-cpp/tests/consumer_route_projection/mixture.rs
A	crates/swallowtail-adapter-llama-cpp/tests/consumer_route_projection/naming.rs
A	crates/swallowtail-adapter-llama-cpp/tests/consumer_route_projection/posture.rs
A	crates/swallowtail-adapter-llama-cpp/tests/consumer_route_projection/proof.rs
A	crates/swallowtail-adapter-ollama/src/consumer_route_projection.rs
A	crates/swallowtail-adapter-ollama/src/consumer_route_projection/builder.rs
M	crates/swallowtail-adapter-ollama/src/lib.rs
A	crates/swallowtail-adapter-ollama/tests/consumer_route_projection.rs
A	crates/swallowtail-adapter-ollama/tests/consumer_route_projection/assembly.rs
A	crates/swallowtail-adapter-ollama/tests/consumer_route_projection/claims.rs
A	crates/swallowtail-adapter-ollama/tests/consumer_route_projection/controls.rs
A	crates/swallowtail-adapter-ollama/tests/consumer_route_projection/fixtures.rs
A	crates/swallowtail-adapter-ollama/tests/consumer_route_projection/ledger.rs
A	crates/swallowtail-adapter-ollama/tests/consumer_route_projection/mixture.rs
A	crates/swallowtail-adapter-ollama/tests/consumer_route_projection/naming.rs
A	crates/swallowtail-adapter-ollama/tests/consumer_route_projection/posture.rs
A	crates/swallowtail-adapter-ollama/tests/consumer_route_projection/proof.rs
A	crates/swallowtail-adapter-ollama/tests/consumer_route_projection/shapes.rs
M	crates/swallowtail-adapter-opencode/src/driver/tests.rs
M	crates/swallowtail-adapter-opencode/src/protocol/tests/catalogue_and_health.rs
M	crates/swallowtail-adapter-opencode/src/selection.rs
M	crates/swallowtail-adapter-opencode/src/selection/tests.rs
M	crates/swallowtail-adapter-opencode/tests/conformance.rs
M	crates/swallowtail-adapter-opencode/tests/deletion_range.rs
A	crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.18.28/README.md
A	crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.18.28/claim.json
A	crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.18.28/dist-inventory.json
A	crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.18.28/identity.json
A	crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.18.28/protocol.json
M	crates/swallowtail-adapter-opencode/tests/http_driver/lifecycle.rs
M	crates/swallowtail-adapter-opencode/tests/http_driver/version_range.rs
M	crates/swallowtail-adapter-opencode/tests/installed_probe.rs
A	crates/swallowtail-adapter-opencode/tests/opencode_http_1_18_28_delta_ledger.rs
A	crates/swallowtail-adapter-opencode/tests/opencode_http_1_18_28_delta_ledger/identity.rs
A	crates/swallowtail-adapter-opencode/tests/opencode_http_1_18_28_delta_ledger/inventory.rs
A	crates/swallowtail-adapter-opencode/tests/opencode_http_1_18_28_delta_ledger/protocol.rs
M	crates/swallowtail-adapter-opencode/tests/prepared_facade/cases/catalogue_and_session.rs
M	crates/swallowtail-adapter-opencode/tests/prepared_facade/cases/preparation_and_deletion.rs
M	crates/swallowtail-adapter-opencode/tests/prepared_facade/cases/structured_runs.rs
M	crates/swallowtail-adapter-opencode/tests/prepared_facade/deletion_conformance.rs
M	crates/swallowtail-adapter-opencode/tests/prepared_facade/session_history.rs
M	crates/swallowtail-adapter-opencode/tests/prepared_facade/session_import.rs
M	crates/swallowtail-adapter-opencode/tests/protocol_suite.rs
M	crates/swallowtail-core/src/preflight/session_access.rs
M	crates/swallowtail-runtime/src/consumer_route_projection.rs
A	crates/swallowtail-runtime/src/consumer_route_projection/acknowledgement.rs
M	crates/swallowtail-runtime/src/consumer_route_projection/admission.rs
M	crates/swallowtail-runtime/src/consumer_route_projection/compose.rs
M	crates/swallowtail-runtime/src/consumer_route_projection/contribution.rs
M	crates/swallowtail-runtime/src/consumer_route_projection/failure.rs
M	crates/swallowtail-runtime/src/consumer_route_projection/identity.rs
A	crates/swallowtail-runtime/src/consumer_route_projection/provider_operation_observation.rs
M	crates/swallowtail-runtime/src/consumer_route_projection/row.rs
M	crates/swallowtail-runtime/src/consumer_route_projection/semantics/authority.rs
M	crates/swallowtail-runtime/src/consumer_route_projection/semantics/posture.rs
M	crates/swallowtail-runtime/src/consumer_route_projection/view.rs
M	crates/swallowtail-runtime/src/consumer_route_projection/views.rs
M	crates/swallowtail-runtime/src/lib.rs
M	crates/swallowtail-runtime/src/provider_session_history/page.rs
M	crates/swallowtail-runtime/src/provider_session_import/outcome.rs
M	crates/swallowtail-testkit/src/consumer_route_projection_assertions.rs
A	crates/swallowtail-testkit/src/consumer_route_projection_assertions/compound_acknowledgement/admission.rs
A	crates/swallowtail-testkit/src/consumer_route_projection_assertions/compound_acknowledgement/mod.rs
A	crates/swallowtail-testkit/src/consumer_route_projection_assertions/compound_acknowledgement/state.rs
A	crates/swallowtail-testkit/src/consumer_route_projection_assertions/compound_acknowledgement/support.rs
A	crates/swallowtail-testkit/src/consumer_route_projection_assertions/provider_operation.rs
A	crates/swallowtail-testkit/src/consumer_route_projection_assertions/provider_operation/admission.rs
A	crates/swallowtail-testkit/src/consumer_route_projection_assertions/provider_operation/composition.rs
A	crates/swallowtail-testkit/src/consumer_route_projection_assertions/provider_operation/state.rs
A	crates/swallowtail-testkit/src/consumer_route_projection_assertions/provider_operation/support.rs
A	crates/swallowtail-testkit/src/consumer_route_projection_assertions/provider_operation/support/fixture.rs
A	crates/swallowtail-testkit/src/consumer_route_projection_assertions/provider_operation/support/outcomes.rs
A	crates/swallowtail-testkit/src/consumer_route_projection_assertions/provider_operation/support/plan.rs
A	crates/swallowtail-testkit/src/consumer_route_projection_assertions/provider_operation/support/rows.rs
M	crates/swallowtail-testkit/src/lib.rs
A	crates/swallowtail-testkit/src/session_access_assertions.rs
M	crates/swallowtail-testkit/src/session_access_fixture.rs
M	crates/swallowtail-testkit/tests/session_access_policy.rs
M	docs/README.md
M	docs/architecture/system-architecture.md
M	docs/contracts/001-working-rules.md
M	docs/contracts/013-interactive-session-access-policy.md
M	docs/contracts/029-interface-version-qualification-and-compatibility.md
M	docs/contracts/036-crate-release-and-compatibility-boundary.md
M	docs/contracts/061-consumer-route-feature-and-control-projection.md
M	docs/guides/claude-agent-sdk-prepared-integration.md
M	docs/guides/opencode-attached-prepared-integration.md
M	docs/guides/provider-route-matrix.md
M	docs/guides/provider-solution-feature-matrix.csv
M	docs/guides/version-currentness-checkpoint.md
A	docs/handoffs/20260904-132656-g05-card062-kimi-local-server-identity.md
A	docs/handoffs/20260904-133530-g05-card064-candidate-c-breadth-audit.md
A	docs/handoffs/20260904-133530-g05-card065-candidate-e-breadth-audit.md
A	docs/handoffs/20260904-133530-g05-card066-candidate-i-breadth-audit.md
A	docs/handoffs/20260904-133530-g05-card067-candidate-j-breadth-audit.md
A	docs/handoffs/20260904-141500-g05-card068-llama-cpp-ollama-package-completion.md
A	docs/handoffs/20260904-142000-g05-card069-antigravity-bedrock-cursor-package-completion.md
A	docs/handoffs/20260904-150000-g05-card070-provider-operation-observation-gate.md
A	docs/handoffs/20260904-170000-g05-card073-provider-operation-observation-runtime-baseline.md
A	docs/handoffs/20260904-180000-g05-card074-candidate-i-completion.md
A	docs/handoffs/20260904-180001-g05-card075-candidate-e-completion.md
A	docs/handoffs/20260904-180002-g05-card076-kimi-compound-acknowledgement-gate.md
A	docs/handoffs/20260904-180003-g05-card077-opencode-http-identity.md
A	docs/handoffs/20260904-190000-g05-card078-opencode-http-claim.md
A	docs/handoffs/20260904-200500-g05-card079-compound-ack-baseline.md
A	docs/handoffs/20260904-210500-g05-card034-kimi-package-completion.md
A	docs/handoffs/20260904-211500-g05-card089-core-preflight-tool-exclusion.md
A	docs/handoffs/20260905-022300-g05-card090-v0-4-1-compatibility-audit.md
A	docs/handoffs/README.md
M	docs/logs/2026-09-03-g05-021-card-051-lock-sync-prerequisite.md
A	docs/logs/2026-09-04-card-034-ready.md
A	docs/logs/2026-09-04-claude-sdk-parity-and-compound-acknowledgement.md
A	docs/logs/2026-09-04-contract-029-in-run-latest-movement.md
A	docs/logs/2026-09-04-contract-061-and-opencode-frontier-promoted.md
A	docs/logs/2026-09-04-contract-061-candidate-c-promoted-observation-gap-recurs.md
A	docs/logs/2026-09-04-contract-061-candidate-j-promoted.md
A	docs/logs/2026-09-04-contract-061-observation-deferral-and-breadth-audits.md
A	docs/logs/2026-09-04-g05-009-card-034-closeout.md
A	docs/logs/2026-09-04-g05-009-card-064-candidate-c-audit.md
A	docs/logs/2026-09-04-g05-009-card-065-candidate-e-breadth-audit.md
A	docs/logs/2026-09-04-g05-009-card-066-candidate-i-audit.md
A	docs/logs/2026-09-04-g05-009-card-067-candidate-j-audit.md
A	docs/logs/2026-09-04-g05-009-card-068-candidate-j-package-completion.md
A	docs/logs/2026-09-04-g05-009-card-069-candidate-c-package-completion.md
A	docs/logs/2026-09-04-g05-009-card-070-provider-operation-observation-gate.md
A	docs/logs/2026-09-04-g05-009-card-073-provider-operation-observation-runtime-baseline.md
A	docs/logs/2026-09-04-g05-009-card-074-deepseek-closeout.md
A	docs/logs/2026-09-04-g05-009-card-075-gemini-grok-closeout.md
A	docs/logs/2026-09-04-g05-009-card-076-kimi-compound-acknowledgement-closeout.md
A	docs/logs/2026-09-04-g05-009-card-079-compound-ack-closeout.md
A	docs/logs/2026-09-04-g05-021-card-051-v0-4-0-candidate-closeout.md
A	docs/logs/2026-09-04-g05-021-card-052-v0-4-0-consumer-proof.md
A	docs/logs/2026-09-04-g05-026-card-062-kimi-0-41-0-identity-stop.md
A	docs/logs/2026-09-04-g05-027-antigravity-1-1-26-compiled.md
A	docs/logs/2026-09-04-g05-027-card-071-antigravity-1-1-26-identity-stop.md
A	docs/logs/2026-09-04-g05-028-card-077-opencode-http-identity-closeout.md
A	docs/logs/2026-09-04-g05-028-card-078-opencode-http-claim.md
A	docs/logs/2026-09-04-g05-029-card-089-closeout.md
A	docs/logs/2026-09-04-observation-gate-chosen-and-card-062-retargeted.md
A	docs/logs/2026-09-04-preflight-tool-exclusion-ruling.md
A	docs/logs/2026-09-04-provider-operation-observation-promoted.md
A	docs/logs/2026-09-04-research-284-all-route-currentness-checkpoint.md
A	docs/logs/2026-09-04-v0-4-0-annotated-tag.md
A	docs/logs/2026-09-05-g05-029-card-080-closeout.md
A	docs/logs/2026-09-05-g05-030-v0-4-1-release-readiness-compiled.md
M	docs/logs/README.md
M	docs/releases/0.4.0.md
M	docs/releases/README.md
A	docs/research/282-kimi-code-local-server-0-41-0-identity.md
A	docs/research/283-antigravity-1-1-26-identity.md
A	docs/research/284-all-route-version-currentness-checkpoint.md
A	docs/research/285-opencode-http-1-18-28-identity.md
M	docs/research/README.md
M	docs/roadmaps/README.md
M	docs/roadmaps/g05/009-contract-061-consumer-projection-realization.md
M	docs/roadmaps/g05/021-v0-4-0-release-readiness.md
A	docs/roadmaps/g05/026-kimi-code-local-server-0-40-1-useful-newer.md
A	docs/roadmaps/g05/027-antigravity-1-1-26-useful-newer.md
A	docs/roadmaps/g05/028-opencode-http-1-18-28-useful-newer.md
A	docs/roadmaps/g05/029-claude-sdk-interactive-parity.md
A	docs/roadmaps/g05/030-v0-4-1-release-readiness.md
M	docs/roadmaps/g05/README.md
M	docs/roadmaps/g05/batch-cards/034-contract-061-kimi-package-completion.md
M	docs/roadmaps/g05/batch-cards/051-v0-4-0-candidate-preparation-and-exact-sha-ci.md
M	docs/roadmaps/g05/batch-cards/052-v0-4-0-consumer-proof-and-operator-tag-gate.md
A	docs/roadmaps/g05/batch-cards/062-kimi-code-local-server-0-40-1-identity.md
A	docs/roadmaps/g05/batch-cards/063-kimi-code-local-server-0-40-1-claim.md
A	docs/roadmaps/g05/batch-cards/064-contract-061-candidate-c-breadth-audit.md
A	docs/roadmaps/g05/batch-cards/065-contract-061-candidate-e-breadth-audit.md
A	docs/roadmaps/g05/batch-cards/066-contract-061-candidate-i-breadth-audit.md
A	docs/roadmaps/g05/batch-cards/067-contract-061-candidate-j-breadth-audit.md
A	docs/roadmaps/g05/batch-cards/068-contract-061-llama-cpp-ollama-package-completion.md
A	docs/roadmaps/g05/batch-cards/069-contract-061-antigravity-bedrock-cursor-package-completion.md
A	docs/roadmaps/g05/batch-cards/070-contract-061-provider-operation-observation-gate.md
A	docs/roadmaps/g05/batch-cards/071-antigravity-1-1-26-identity.md
A	docs/roadmaps/g05/batch-cards/072-antigravity-1-1-26-claim.md
A	docs/roadmaps/g05/batch-cards/073-contract-061-provider-operation-observation-baseline.md
A	docs/roadmaps/g05/batch-cards/074-contract-061-deepseek-package-completion.md
A	docs/roadmaps/g05/batch-cards/075-contract-061-gemini-grok-package-completion.md
A	docs/roadmaps/g05/batch-cards/076-contract-061-kimi-compound-acknowledgement-gate.md
A	docs/roadmaps/g05/batch-cards/077-opencode-http-1-18-28-identity.md
A	docs/roadmaps/g05/batch-cards/078-opencode-http-1-18-28-claim.md
A	docs/roadmaps/g05/batch-cards/079-contract-061-compound-acknowledgement-baseline.md
A	docs/roadmaps/g05/batch-cards/080-claude-sdk-read-write-session-and-permission-policy.md
A	docs/roadmaps/g05/batch-cards/081-claude-sdk-bash-under-mediation.md
A	docs/roadmaps/g05/batch-cards/082-claude-sdk-mid-session-model-and-effort.md
A	docs/roadmaps/g05/batch-cards/083-claude-sdk-resume-and-session-listing.md
A	docs/roadmaps/g05/batch-cards/084-claude-sdk-client-mcp-servers.md
A	docs/roadmaps/g05/batch-cards/085-grok-acp-answerable-permissions.md
A	docs/roadmaps/g05/batch-cards/086-claude-sdk-discovery-identity.md
A	docs/roadmaps/g05/batch-cards/087-claude-sdk-qualified-ranges.md
A	docs/roadmaps/g05/batch-cards/088-harness-install-guidance-diagnostics.md
A	docs/roadmaps/g05/batch-cards/089-core-preflight-tool-exclusion-scoped-to-bounded-profiles.md
A	docs/roadmaps/g05/batch-cards/090-v0-4-0-to-candidate-compatibility-audit.md
A	docs/roadmaps/g05/batch-cards/091-v0-4-1-candidate-preparation-and-exact-sha-ci.md
A	docs/roadmaps/g05/batch-cards/092-v0-4-1-consumer-proof-and-operator-tag-gate.md
M	docs/roadmaps/g05/batch-cards/README.md
M	docs/roadmaps/generation-index.md
M	docs/roadmaps/standing-lanes.md
M	docs/triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv
M	docs/triage/2026-09-01-contract-061-kimi-active-observation-public-baseline-gate.md
D	docs/triage/2026-09-02-claude-sdk-shared-lifecycle-prerequisites.md
A	docs/triage/README.md
M	release-baselines/public-api-0.4.0/swallowtail-adapter-antigravity.txt
M	release-baselines/public-api-0.4.0/swallowtail-adapter-bedrock.txt
M	release-baselines/public-api-0.4.0/swallowtail-adapter-claude-agent.txt
M	release-baselines/public-api-0.4.0/swallowtail-adapter-cursor.txt
M	release-baselines/public-api-0.4.0/swallowtail-adapter-deepseek-harness.txt
M	release-baselines/public-api-0.4.0/swallowtail-adapter-deepseek.txt
M	release-baselines/public-api-0.4.0/swallowtail-adapter-gemini.txt
M	release-baselines/public-api-0.4.0/swallowtail-adapter-grok.txt
M	release-baselines/public-api-0.4.0/swallowtail-adapter-kimi-platform.txt
M	release-baselines/public-api-0.4.0/swallowtail-adapter-kimi.txt
M	release-baselines/public-api-0.4.0/swallowtail-adapter-llama-cpp.txt
M	release-baselines/public-api-0.4.0/swallowtail-adapter-ollama.txt
M	release-baselines/public-api-0.4.0/swallowtail-runtime.txt
M	release-baselines/public-api-0.4.0/swallowtail-testkit.txt
```

## Closeout

Named validation passed: `effigy package:api`, `effigy qa:routes`,
`effigy qa:docs`, `effigy qa:northstar`, and `git diff --check`. The
untracked Research file also passed the whitespace check. One commit containing
only the three owned paths and one exact-head PR remain. Merge and Card 091
prepare remain outside worker scope.
