# Changelog

All notable Swallowtail changes are recorded here. Releases are source-only
annotated Git tags from the canonical repository.

## [Unreleased]

### Added

- add typed OpenAI Background adapter-local standard service-tier selection:
  `OpenAiBackgroundServiceTier::standard()` dispatches exact Responses
  `service_tier: "default"` on ordinary attached runs and one in-process
  reattachment; omission preserves prior create bytes, active-run detachment
  rejects before effects, selected-tier checkpoints reject restart
  reconciliation before network work, and returned tier, price, latency,
  capacity, entitlement, and provider acceptance remain unclaimed; the exact
  facade advances to
  `openai-responses-background-2026-08-23-service-tier`; Research 196,
  g04.049
- add typed Gemini Live adapter-local default-only context-window compression
  for `gemini-3.1-flash-live-preview`: selected
  `GeminiLiveContextWindowCompression::sliding_window()` dispatches exact
  `contextWindowCompression.slidingWindow = {}` on initial setup, one planned
  rollover, and fresh restoration; omission preserves prior setup bytes,
  explicit trigger and target token forms remain withheld, no portable
  capability or shared request field is added, and dispatch does not claim
  provider acceptance or effective compression; Research 195, g04.048
- add exact Gemini Live caller-selected output-token maximum for
  `gemini-3.1-flash-live-preview`: positive values through `65_536` dispatch as
  `generationConfig.maxOutputTokens` on initial setup, one planned rollover,
  and fresh restoration; omission preserves the existing setup bytes, the
  maximum composes with every admitted thinking level, unsupported values fail
  before effects, and dispatch does not claim provider acceptance or effective
  generated length; Research 194, g04.047
- add exact Gemini Live reasoning selection for
  `gemini-3.1-flash-live-preview` through portable `minimal`, `low`, `medium`,
  and `high` values on initial setup, one planned rollover, and fresh
  restoration; omission keeps the existing `MINIMAL` bytes without claiming a
  caller selection, unsupported values and sibling OpenAI Realtime use fail
  before effects, and dispatch does not claim provider acceptance or effective
  reasoning depth; Research 193, g04.046
- add exact Qwen Code `0.21.15` reasoning selection for `qwen3.8-max` and
  `qwen3.8-max-preview` through portable `low`, `medium`, `high`, `xhigh`, and
  `max` values on structured runs, first and resumed turns, and fresh
  replacement; a bounded private stream-JSON control exchange rejects ambient
  override or substitution before the user message without claiming provider-
  effective reasoning depth; Research 189, g04.041
- add exact xAI Responses WebSocket reasoning selection for `grok-4.5`
  (`low`, `medium`, `high`) and `grok-4.6` (`low`, `medium`, `high`,
  `xhigh`), plus positive `max_output_tokens` through `2_147_483_647`, on
  structured runs and serial connection-local sessions; controls remain fixed
  through continuation and fresh replacement, and dispatch does not claim
  provider acceptance, effective reasoning depth, or exact generated length;
  Research 187, g04.039
- add exact Anthropic Messages effort selection for `claude-opus-4-7` through
  portable `ReasoningSelection` values `low`, `medium`, `high`, `xhigh`, and
  `max` on structured runs and fixed direct-continuation sessions; dispatches
  `output_config.effort` without adding Messages thinking or claiming effective
  effort; Research 185, g04.037
- add exact DeepSeek V4 Pro reasoning selection for `low`, `high`, and `max`
  across structured runs, tool continuation, later turns, and fresh local
  restoration; `thinking.type` remains fixed to `enabled`, private reasoning
  replay remains adapter-held, and dispatch does not claim provider acceptance
  or effective depth; Research 186, g04.038
- add typed Ollama adapter-local context-window selection (`OllamaContextWindow`,
  `with_context_window`) on structured inference and interactive session
  profiles, with Research 184 positive-domain dispatch of `options.num_ctx`
  beside `num_predict`; dispatch does not prove provider acceptance or
  effective allocation; g04.036
- add typed Cursor headless model parameters (`CursorHeadlessFast`,
  `CursorHeadlessContext`, `with_fast`, `with_context`, `with_effort`) with
  Research 183 deliver-now tuples, canonical single-argument `--model`
  dispatch, and qualified `ReasoningSelection` for high effort on
  `claude-opus-4-8` and `claude-opus-5` only; dispatch does not prove provider
  acceptance; g04.035

### Changed

- Correct the exact GPT-5.6 `openai.background` reasoning vocabulary to
  `none`, `low`, `medium`, `high`, `xhigh`, and `max`; the previously
  guaranteed but unqualified `minimal` value now fails before endpoint,
  credential, request, or provider work. The corrected mapping uses opaque
  facade point `openai-responses-background-2026-08-23` and private behavior
  revision `openai.responses-background-v2`. This guaranteed-behavior shrink
  requires the next source release to advance the pre-1.0 minor; g04.044.
- Raised qualified Gemini CLI ACP and headless ceilings from their previous
  `0.51.0` and `0.51.0..=0.52.0` bounds to maintained
  `0.51.0..=0.56.0` on their separate version axes. Official published
  intermediates are qualified; `0.56.1` remains visible `UnverifiedNewer`.
  Selected lifecycle and stream shapes remain compatible, provider-private
  invalid-stream additions stay unmapped, and transcript management remains
  unsupported. Research 182, g04.034.
- Raised qualified Kimi Code ACP, headless, and local-server ceilings from
  `0.37.2` to official `@moonshot-ai/kimi-code` `0.38.0` on the
  `kimi-code.executable` axis. Compatible-extension: selected ACP,
  headless, and local-server protocol blobs byte-identical; `acp --region`,
  WaitFor, advertised close/delete, and watch-fs `runtime_id` stay
  unmapped. Research 179, g04.032.
- Raised qualified OpenCode HTTP ceiling from `1.18.18` to official npm
  `opencode-ai` `1.18.20` on the `opencode.server` axis. Compatible-extension:
  tagged OpenAPI byte-identical through `1.18.19`; selected execution, delete,
  import, and continuity closures unchanged; `surface-19` kept. Research 176,
  g04.029.
- Raised qualified Antigravity catalogue and headless ceiling from
  `1.1.15` to official GitHub `google-antigravity/antigravity-cli`
  `1.1.17`: `1.1.9..=1.1.17`, published intermediate `1.1.16` qualified,
  `1.1.8` stays independently unqualified, and `mcp` plus `--input-format`
  stay unmapped. Research 177, g04.030.
- Raised qualified Codex CLI ceiling from `0.148.0` to `0.149.0` on the
  `codex.cli` axis (both `codex.exec` and `codex.app-server` routes).
  Compatible-extension: exec help byte-identical, selected mapped flags
  unchanged, ModelListParams unchanged. Research 172, g04.025.
- Raised qualified Qwen Code headless ceiling from `0.21.14` to official
  `@qwen-code/qwen-code` `0.21.15` on the `qwen-code.package` axis.
  Compatible-extension: selected mapped flags unchanged; `types.ts` and
  `systemController.ts` byte-identical; `config.ts` changed only unmapped
  `--session-id` occupancy. Research 173, g04.026.
- Raised qualified Ollama attached ceiling from `0.32.14` to official
  GitHub `ollama/ollama` `v0.32.15` on the `ollama.runtime` axis.
  Compatible-extension: `api/types.go` byte-identical; selected native
  structs and routes unchanged; `routes.go` changed only unselected
  scheduler cache and parser-error cancel. Exclusions `0.32.2` and
  `0.32.10` stay. Research 174, g04.027.
- Raised qualified Claude Code headless and response-only ceilings from
  `2.1.235` to official npm `@anthropic-ai/claude-code` `2.1.238`.
  Compatible-extension: installer wrapper files match `2.1.235` except
  the version pin; selected stream-JSON flags stay; published
  intermediates `2.1.236` and `2.1.237` are qualified. Later stables
  stay AllowUnverified. Research 175, g04.028.
- Raised qualified Oh My Pi RPC ceiling from `17.3.8` to official npm
  `@oh-my-pi/pi-coding-agent` `17.4.0`: maintained `17.2.9..=17.4.0` on
  `oh-my-pi.rpc-v2-v17.2.9`. Minor-line step; private-milestone checked
  and mapping unchanged. Unpublished `17.3.6` stays. Tokenizer JS API,
  `omp ps`, `/cleanse`, and extended-context stay unmapped. Research 178,
  g04.031.

## [0.3.3] - 2026-08-19

### Added
- add the separately selectable `swallowtail-adapter-deepagents` package and
  exact Deep Agents ACP `0.1.25` route with qualified-only claim, empty extra
  argv, and host-owned provider API keys without a credential lease
- add the separately selectable `swallowtail-adapter-kiro` package and exact
  Kiro CLI ACP `2.18.1` route with qualified-only claim and local-account
  access
- add the separately selectable `swallowtail-adapter-openhands` package for
  exact OpenHands Agent Server `1.42.1` identity, driver, and prepared
  facade; `openhands.agent-server` is not a production route because live
  HTTP/WebSocket conversation stays unwired
- add the separately selectable `swallowtail-adapter-cline` package and exact
  Cline `3.0.55` ACP and headless routes with qualified-only claims,
  local-account access, and no credential lease
- add the separately selectable `swallowtail-adapter-goose` package and exact
  Goose `1.46.0` ACP route with qualified-only claim and local-config access
- add the separately selectable `swallowtail-adapter-copilot-cli` package and
  exact Copilot CLI `1.0.80` ACP route with qualified-only claim, visible
  public preview, and host-account access
- add the separately selectable `swallowtail-adapter-mistral-vibe` package
  and exact Mistral Vibe `2.24.2` headless route with qualified-only claim
  and local-config access
- add the separately selectable `swallowtail-adapter-qoder` package and exact
  Qoder CLI `1.1.25` headless route with qualified-only claim and
  local-config access
- add the separately selectable `swallowtail-adapter-zcode` package and exact
  ZCode app-server `0.16.3` route with qualified-only claim, host-approved
  Node payload, and no credential lease
- add the separately selectable `swallowtail-adapter-deepseek-harness` package
  and exact DeepSeek Harness JSON-RPC `0.1.0rc6` and local-server `0.1.0-rc.6`
  routes with qualified-only claims, host-owned Cordis configuration, and no
  credential lease

### Changed
- raise the Antigravity catalogue and headless qualified ceiling through
  official GitHub `google-antigravity/antigravity-cli` `1.1.15`:
  `1.1.9..=1.1.15`, `1.1.8` stays independently unqualified, and
  `--input-format` plus Gemini API-key sign-in stay unmapped
- raise the Oh My Pi RPC qualified ceiling through official npm
  `@oh-my-pi/pi-coding-agent` `17.3.8`: maintained `17.2.9..=17.3.8` on
  `oh-my-pi.rpc-v2-v17.2.9`; unpublished `17.3.6` stays; `providers.cacheRetention`,
  advisor, ACP, session switching, and subagent authority stay unmapped
- raise Kimi Code ACP, headless, and local-server qualified ceilings through
  official `@moonshot-ai/kimi-code` `0.37.2`: ACP and headless reuse
  declared-effort and stream-json through `0.29.0..=0.37.2`; local-server
  heartbeat-ping extends to `0.35.0..=0.37.2`; advertised ACP close/delete,
  `acp --login`, terminal-auth metadata, and watch-fs `runtime_id` stay
  unmapped
- raise the Qwen headless qualified ceiling through official npm
  `@qwen-code/qwen-code` `0.21.14`: `0.19.11..=0.20.1` and
  `0.21.0..=0.21.14`, unpublished stable `0.20.2` stays incompatible, and
  `qwen sessions ps` / `/advisor` / live-session registry stay unmapped
- raise the Grok Build ACP qualified ceiling through official npm
  `@xai-official/grok` `1.0.5`: maintained `1.0.4..=1.0.5` on
  cached-token-model-4-6-v3; alpha `1.0.6` stays UnverifiedNewer and is not
  official latest; `--leader-socket` and vendor `_x.ai/*` notifications stay
  unmapped
- raise the Claude Code headless and response-only qualified ceilings through
  official npm `@anthropic-ai/claude-code` `2.1.235`: headless
  `2.1.220..=2.1.235`, response-only `2.1.227..=2.1.235`; later stables stay
  AllowUnverified, and spellcheck plus unused help flags stay unmapped
- raise the Claude Agent ACP qualified ceiling through official npm
  `@agentclientprotocol/claude-agent-acp` `0.70.0`: `0.53.0..=0.70.0`
  excluding unpublished `0.58.0`; Providers API, goal, Air, and file-change
  initialize `_meta` stay unmapped
- raise the Codex exec, app-server, lifecycle, and thread-catalogue qualified
  ceiling through official npm `@openai/codex` `0.148.0`; later stables stay
  AllowUnverified, existing gaps stay incompatible, and `fork` / `thread/fork`
  / Bedrock stay unmapped
- raise the Pi RPC qualified ceiling through official npm
  `@earendil-works/pi-coding-agent` `0.84.2`: exact published points
  `0.80.10` through `0.84.2`, unpublished `0.83.1` stays incompatible, and
  `0.84.0` adds private message-update-delta; streaming `usage` on
  `message_update` stays unmapped
- raise the Ollama attached native runtime qualified ceiling through official
  GitHub `v0.32.14`: `0.14.0..=0.32.14` reuses `ollama.native-text-v1`, keeps
  `0.32.2` excluded, and adds GitHub-prerelease `0.32.10`
- add exact Cursor Agent catalogue, ACP, and headless milestones
  `2026.07.23-e383d2b`, `2026.08.04-aaa8809`, and `2026.08.11-e8db854`;
  calendar gaps stay unsupported
- replace the `claude-code.response-only` patch-version equality gate with a
  protocol-compatibility policy: `2.1.227` remains the proven floor,
  `2.1.228` adds live evidence, the qualified ceiling is `2.1.235`, later
  stable releases may run provisionally, known-bad releases can be denied
  explicitly, and every run still fails closed on command, init, tool/MCP,
  thinking, usage, assistant, or terminal drift
- advance current source to 40 packages and 47 production routes while
  preserving the immutable `v0.3.2` 30-package, 36-route baseline; publish
  [v0.3.3 candidate release notes](docs/releases/0.3.3.md) with upgrade,
  rollback, package, route, and source-only distribution truth

### Fixed
- upgrade transitive `h2` `0.4.15` to `0.4.17` for RUSTSEC-2026-0258
  unbounded empty DATA frames

## [0.3.2] - 2026-08-11

### Added
- add the separately selectable `swallowtail-adapter-command-code` package and
  exact Command Code `1.15.1` headless route with plan-mode one-shot runs,
  bounded NDJSON activity and usage, local provider account access, and
  explicit retained interactive continuity through exact session resume
- add the separately selectable `swallowtail-idioms` package with portable
  idiom records, deterministic confidence decay and merge, lint, bounded
  selection, fail-soft signal recording, a static-rules backend, and registry
  pull/push merge without transport authority
- add opt-in route-path idiom delivery through runtime host ports, fixed
  caller/provider/route/session folding, capability gating, and Codex
  app-server conformance
- add the exact Claude Code `2.1.227` `claude-code.response-only` route for one
  bounded text response through local Max/OAuth with empty tools and MCP, no
  working resource or retained session, and no structured-output claim

### Changed
- classify exact Claude Code medium-effort thinking-token estimates as
  content-free coalescible progress while validating and discarding the empty
  private-thinking envelope before the single assistant text result
- admit Codex app-server child activity from the exact root
  `subAgentActivity(kind=started)` spawn confirmation without weakening
  fail-closed handling for unobserved child identities
- advance current source to 30 packages and 36 production routes while
  preserving the immutable `v0.3.1` 28-package, 34-route baseline; publish
  [v0.3.2 candidate release notes](docs/releases/0.3.2.md) with upgrade,
  rollback, package, route, and source-only distribution truth

### Fixed
- diagnose approved npm shebang launches that require an exact host-side
  interpreter recipe when ambient `PATH` is intentionally absent
- keep Effigy doctor health routing cheap instead of invoking broad validation

## [0.3.1] - 2026-08-08

### Added
- publish [v0.3.1 release notes](docs/releases/0.3.1.md) for the compatible
  debug-observation and provider-session history patch
- add Contract 054 portable provider-session history pages: plan/request/
  response, plan-bound older cursors, `Exact` / `AtLeast` / `Unknown` totals,
  newest-first window helper, and a read-only driver role distinct from load
  and reconciliation
- prove `codex.app-server` synthetic newest-first history pages over bounded
  `thread/read(includeTurns: true)` without control side effects
- prove `opencode.http` and retained `alibaba.conversations` history pages over
  their existing ascending replay walks without live handles or control side
  effects; runtime history plans accept resource-free DirectModelInference
  posture as well as ambient working-resource harness routes
- document provider-session history pages and the Codex, OpenCode, and Alibaba
  retained mappings; ACP load-as-history routes stay unsupported
- add Contract 053 opt-in debug observation: structured `DebugObservation`
  records, defaulted `DiagnosticObserver::observe_debug`, and fail-soft
  `HostServices` emit helpers for restricted wire and lifecycle context
- emit failure-path debug observations across shared discovery/prep, ACP/RPC,
  headless, hosted HTTP/SSE/WS, realtime, remote ACP, Anthropic managed-agent,
  Ollama, llama.cpp attached/owned, and Bedrock catalogue surfaces without
  changing safe diagnostics
- document host opt-in wiring and the current emitter inventory in the
  debug-observation guide and runtime example

## [0.3.0] - 2026-08-08

### Added
- expose shared runtime helpers for installed-executable discovery, prepared
  plan construction, ordered event emission, and terminal projection
- add exact malformed-inbound diagnostics and regression coverage across the
  Codex app-server boundary

### Changed
- **Breaking:** make `codex_cli_binding` and `ollama_runtime_binding` return
  `Option<InterfaceVersionBinding>` so malformed provider-observed versions
  fail closed instead of reaching an infallible parse path
- centralize repeated adapter discovery, preparation, run-loop, and runtime
  plan-family machinery without changing the 28-package or 34-route set
- publish [v0.3.0 candidate release notes](docs/releases/0.3.0.md) with the
  exact API migration, rollback, and source-only distribution boundary

### Fixed
- bound local-process reader joins, preserve force-stop race truth, and close
  runtime waiter and sender-drop paths without hangs
- enforce remote ACP deadlines and joined worker cleanup across HTTP and
  WebSocket transports
- remove provider-reachable panic paths, reject malformed versions and
  unexpected tool-call shapes safely, and preserve exact adapter diagnostics
- make docs indexes, route inventory, MSRV selection, and literal-only version
  parse checks deterministic release gates

## [0.2.0] - 2026-08-06

### Added
- add the separately selectable `swallowtail-adapter-muse` package and exact
  `muse-code.headless` route for local Meta account access to
  `meta` / `muse-spark-1.2`
- add explicit Muse reasoning effort, prepared read-only execution, bounded
  JSONL activity, exact signed-payload discovery, and operator-gated live
  acceptance evidence

### Changed
- promote the coordinated source release to 28 packages and 34 production
  routes without changing crates.io, GitHub Release, binary, or installer
  posture
- raise the verified Rust floor to `1.95.0` for all packages and retire the
  separate Bedrock override; this breaking requirement makes the release
  `0.2.0`
- split Muse event and corpus internals before release without changing public
  API, diagnostics, or guaranteed behavior
- publish [v0.2.0 release notes](docs/releases/0.2.0.md) with exact upgrade,
  rollback, support, and known-limit guidance

## [0.1.1] - 2026-08-06

### Changed
- publish [v0.1.1 release notes](docs/releases/0.1.1.md) for the compatible
  source-tag repair

### Fixed
- preserve accepted Anthropic Managed Agents cancellation as `Cancelled` when
  cancellation and the operation deadline become ready concurrently
- synchronize Kimi detachment conformance with the fixture peer before
  asserting observer-close evidence under workspace contention

## [0.1.0] - 2026-08-06

### Added
- selected one annotated Git tag as the initial distribution; crates.io,
  GitHub Release assets, binaries, sidecars, and installers are excluded
- coordinated 27 independently selectable Rust library packages at version
  `0.1.0`
- qualified 33 production routes across installed harnesses, attached and
  owned local runtimes, hosted APIs and SDKs, and realtime services
- made adapter-local prepared facades the normal integration path while
  retaining provider-neutral low-level runtime roles
- added explicit configured-instance, model-route, access, host-service,
  policy, preflight, operation, event, terminal, and cleanup evidence
- added model catalogue, structured-run, interactive-session, realtime,
  callback, activity, task-list, subagent, lifecycle, reconciliation,
  restoration, detachment, and failure contracts where routes support them
- kept provider credentials, billing, prompts, tools, routing, retry, fallback,
  persistence, and product UI downstream
- retained exact provider and harness version qualification independently from
  the Swallowtail package version
- established Rust `1.90.0` as the general floor, Rust `1.94.1` for Bedrock,
  and Apple Silicon macOS as the initial verified target
- removed Bedrock's legacy Rustls 0.21 dependency path and automated advisory,
  license, and source policy
- replaced declaration hashing with a pinned 27-package semantic public API
  baseline containing 7,819 normalized entries
- documented every supported public item and made all 27 crates deny missing
  public documentation
- completed canonical guidance and compiling normal-path examples for all 33
  routes and every portable feature family
- added deterministic package, API, documentation, MSRV, security, route,
  guide, example, facade, lifecycle, and external-consumer validation surfaces

### Changed
- Established this tag as the first pre-1.0 public API and guaranteed-behavior
  baseline. Compatible changes advance the patch version; breaking API or
  guaranteed-behavior changes advance the minor version; provider
  qualification remains a separate axis.
- Defined migration from existing path or revision integrations: move every
  direct dependency to the same tag and update the lock file atomically.
  Rollback restores the previous manifests and lock file; `v0.1.0` never moves.
- Added [v0.1.0 release notes](docs/releases/0.1.0.md) covering package
  selection, route inventory, installation, limits, and remaining gates.
