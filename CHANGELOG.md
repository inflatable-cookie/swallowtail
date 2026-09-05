# Changelog

All notable Swallowtail changes are recorded here. Releases are source-only
annotated Git tags from the canonical repository.

## [Unreleased]

### Added
- publish Contract 061 candidate K across the Mistral Vibe, Muse Code, Oh
  My Pi, and Qwen adapters. Prepared facades emit the exact 52-row tranche;
  the single interactive Oh My Pi attachment row is consumer-mediated per
  turn from its bounded plan capability, with provider-free ledgers and
  negative assembly coverage. g05.009 card 099.
- complete Contract 061 candidate B across Alibaba Conversations,
  Anthropic Managed Agents and Messages, and xAI Responses WebSocket. The
  prepared facades publish 74 exact rows and withhold only the two named
  Alibaba matrix-descriptor-only rows; adapter-local ledgers cover all 76
  rows, including the single consumer-mediated Managed Agents per-turn
  exchange. g05.009 card 098.
- admit explicit Claude Agent SDK `Bash` profiles under a read-write working
  resource lease. Every Bash call remains consumer-mediated in `default`,
  `plan`, and `acceptEdits`; the callback carries bounded command and
  description views with truncation, while the sidecar retains and returns the
  full input unchanged on allow. Provider-free fake-SDK proofs cover denial,
  execution, truncation, and no live credentials or provider calls. g05.029
  card 081.

## [0.4.1] - 2026-09-05

### Added
- complete Contract 061 candidate F across Kimi ACP, headless, local-server,
  and Kimi Platform routes. Prepared facades now publish exact bounded route
  contributions; Kimi ACP adds outcome-backed catalogue observation and a
  projected-open seam that preserves exact reasoning/Plan acknowledgement and
  negotiated-model evidence without changing the existing open path. The four
  ledgers prove 75 emitted and 14 construction-time withheld rows across 89
  tuples. g05.009 card 034.
- add provider-neutral compound acknowledgement truth for independently
  state-associated reasoning and Plan halves. Exact provider values attach
  only to effective or rejected halves; reasoning-first rejection may mark a
  requested Plan half terminally not dispatched without claiming pending
  work. The value remains post-open, observation-only evidence and adds no
  request, mutation, routing, or acknowledgement authority. g05.009 card 079.
- select the `claude-agent.sdk` permission mode at open and change it
  mid-session. `ClaudeAgentSdkSessionProfile` binds an explicit admitted tool
  set and one of `default`, `plan`, or `acceptEdits` as an additive prepared
  input; the sidecar passes the admitted set as `tools`, never sets
  `allowedTools`, adds every withheld admissible tool to `disallowedTools`,
  and exposes a `set_permission_mode` wire command whose confirmed mode the
  new `ClaudeAgentSdkPreparedSession::open_route_session` handle returns or
  fails typed. `bypassPermissions`, `auto`, and `dontAsk` are unrepresentable
  and are refused by name before the SDK is loaded. The default profile is the
  unchanged read-only `Read`/`Glob`/`Grep` set under `default` mode.
- run a multi-turn `claude-agent.sdk` editing session. Admitting `Edit`,
  `Write`, or `MultiEdit` binds `ResourceAccess::ReadWrite` on the
  working-resource lease into the plan, the session access policy, and the
  `claude-agent-sdk-ambient-read-write` instance policy; a host that resolves
  a read-only lease fails the agreement before the sidecar starts. The lease
  is a location scope under `AmbientHost` and makes no bounded-filesystem
  claim. Every write is offered to the consumer through `canUseTool` with its
  tool input intact unless the consumer chose `acceptEdits`, and the
  provider-free fake-SDK proofs show a two-turn session writing only what the
  host admitted. g05.029 card 080.
- complete Contract 061 candidate C consumer route projections across
  `swallowtail-adapter-antigravity`, `swallowtail-adapter-bedrock`, and
  `swallowtail-adapter-cursor`. Prepared catalogue, headless, continuation,
  runtime, and ACP operation facades emit exact route and capability truth
  with deterministic per-route ledgers proving 51 emitted and 43 withheld rows
  across 94 census tuples, while keeping four no-control audits as negative
  coverage and withholding catalogue activity observation. g05.009 card 069.
- publish Contract 061 consumer-route projection for candidate J: exact 10
  `llama-cpp.attached`, 6 `llama-cpp.owned`, and 19 `ollama.attached` rows
  with 32 emitted and 3 construction-time withheld. g05.009 card 068.

### Changed
- scope shared preflight consumer-tool exclusion to bounded writable session
  profiles, admitting tool calls for ambient read-write harnesses while
  preserving the existing bounded rejection. g05.029 card 089.

## [0.4.0] - 2026-09-04

### Breaking
- require `InteractiveSessionHandle::close` callers to provide exact
  `HostServices` and a `SessionCleanupRequest` with one caller-selected
  absolute deadline. The boundary now covers active-turn interruption,
  escalation, task and pump joins, credential release, and resource release;
  expiry reports failed cleanup rather than allowing an unbounded or falsely
  clean close. The prior zero-argument close has no compatibility shim. This
  coordinated public API break targets `v0.4.0`. g05.023 card 058.
- remove the previously guaranteed but unqualified OpenAI Background `minimal`
  reasoning value from exact GPT-5.6 `openai.background` preparation. Admitted
  values are `none`, `low`, `medium`, `high`, `xhigh`, and `max`; `minimal` now
  fails before endpoint, credential, request, or provider work. The corrected
  opaque facade point is `openai-responses-background-2026-08-23` with private
  behavior revision `openai.responses-background-v2`. This guaranteed-behavior
  shrink requires coordinated pre-1.0 minor `v0.4.0`. g04.044.
- reject selected-tier OpenAI Background checkpoint restart reconciliation
  before network work. Explicit default-tier dispatch remains ordinary attached
  and one-reattachment only; selected-tier checkpoints cannot be restarted.
  Research 196, g04.049.
- cap `kimi-code.acp` at `0.38.0` under `QualifiedOnly`. Exact `0.39.0` and
  `0.39.1`, unpublished `0.38.1`/`0.39.2`, and farther `0.40.x` fail closed
  because the newer terminal path creates an uncontained local process. This
  shrinks the previously permitted unverified-newer posture above the maintained
  range. g05.017, card 043.

### Added
- add provider-neutral, operation-scoped task-reap reservations. The exact
  selected `ScopedTaskService` must grant owned reap authority before
  credential, resource, process, task, or provider effects, then binds that
  grant to one exact-host/exact-scope task before its future is polled.
  Unsupported, closing, and capacity-exhausted hosts fail at reservation;
  valid later relinquishment cannot lose a shutdown or capacity race.
  Host-local closes reservation admission during explicit shutdown, waits
  unused grants and reservation-backed tasks to settle, and joins bounded
  per-reservation reapers outside the task tree. Starting a reserved join moves
  the worker into that host ownership before returning its observation future,
  so cancelling the future before or after polling cannot detach work or
  release shutdown early. Forged or mismatched authority fails closed before
  task work. Ordinary unreserved spawn, explicit join, and join-on-drop are
  unchanged. The grant is ownership, not a boolean support probe, and there is
  no global task parking. Contracts 009, 010, 017, 019, and 047; g05.025 card
  061.
- add provider-neutral scoped-task relinquishment for caller-bounded
  operations. The selected `ScopedTaskService` accepts an unfinished
  `JoinedTask` only under its exact execution host and `ScopeId`, clears caller
  ownership only after a host reaper accepts it, and returns
  `TaskRelinquishOutcome::AcceptedForReap`. The result is not joined or cleanup
  completion. Host-local retains each per-transfer reaper under the outer
  selected-host lifecycle, whose explicit shutdown joins all retained reapers
  outside the task tree. Task-service clones carry weak handoff authority only.
  There is no adapter-global parking. Wrong authority, finished tasks, and
  repeat transfer fail closed while ordinary join/drop ownership stays
  unchanged. Contracts 009, 010, 019, and 037; g05.024 card 060.
- separate root process exit from owned descendant-tree completion in runtime
  process evidence: `ProcessTreeCompletion::{RootOnly, OwnedTreeEmpty}`,
  `ProcessExit::attesting_empty_owned_tree`, and
  `ProcessExit::tree_completion`. `ProcessExit::new` stays root-only, so no
  existing caller or fixture host gains a tree claim, and only a host whose
  concrete mechanism observed an empty owned tree may construct the attested
  state. `swallowtail-host-local` proves descendant enrollment and termination
  but cannot observe emptiness under `forbid(unsafe_code)`, so every local
  exit reports root-only on every platform. Card 059 falsified the candidate
  primitives natively — setsid escape, descriptor end-of-file with a live
  child, released-group identity, and reparenting to `launchd` — and found no
  sound owned-tree observation within the current ordinary host-local authority
  on macOS; the host stays root-only and adds no unsafe. Contracts 010 and 019,
  g05.023 cards 057 and 059.
- publish exact negotiated model-options observation on projected Claude Agent
  ACP session open: after existing model confirmation, parse one bounded
  `configOptions[id=model]` select, retain it on the session handle through
  the existing runtime seam, and emit only
  `feature.negotiated-model-options-observation` from a distinct active
  source. Required missing model entry fails both public opens through
  existing confirmation. Snapshot-detail malformation that still confirms
  `currentValue` is no snapshot on preserved `open_session` and close+fail on
  `open_session_with_projection` with
  `swallowtail.negotiated_model_options.invalid`. Load, resume, catalogue, and
  prepared contribution stay negative. Research 279, g05.022 card 056.
- add the provider-free `claude-agent.sdk` route: Anthropic's official
  `@anthropic-ai/claude-agent-sdk` `0.3.259`, carrying native `2.1.259`, driven
  through a source-tagged Node sidecar (exact Node `22.23.2`) over the private
  bounded `swallowtail-claude-agent-sdk-jsonl-v1` wire at behavior revision
  `claude-agent.sdk-v1`. Five qualified-only one-point axes bind the SDK
  wrapper, native binary, runtime, wire, and sidecar revision independently; no
  ACP or Claude Code claim transfers in either direction. The application
  provisions the runtime, the `.` SDK entry point, its peer dependencies, and
  the platform package; Swallowtail never installs, vendors, updates, repairs,
  or redistributes them. Swallowtail holds no subscription credential: it
  leases a delegated reference, admits only `firstParty` provenance with an
  `oauth` key source, refuses account identity fields, and mechanically forbids
  the credential-bearing `/bridge` and `/browser` subpaths, API-key helpers,
  and cloud auth refresh. The selected model is sent explicitly and confirmed
  from `system/init` evidence. Tool availability is restricted with `tools` and
  never auto-allowed with `allowedTools`; the read-only allow-list is enforced
  before any consumer round trip, and an allowed decision returns the
  provider's tool input unchanged while that input never crosses the wire. Open,
  start-turn, interrupt, and close are bounded by caller-supplied host
  deadlines that bound the return and the cleanup after it. The route is
  Unix-only: Windows retains no tree owner, so the addable row reports
  `Unsupported` and open refuses there. Close reports `Clean` only on
  `ProcessTreeCompletion::OwnedTreeEmpty`; on ordinary macOS, confirmed root
  completion after the declared descendant termination attempt is the
  operator-accepted `Degraded` posture, and an observed surviving descendant or
  unconfirmed root exit is `Failed`. Session persistence is disabled, so
  resume, fork, session management, model, effort, thinking, usage detail, MCP,
  hooks, plugins, skills, subagents, Bash, and terminal remain later layers.
  The exact package and native points were rebound from `0.3.258` after a full
  15-file package-tree inventory across the hop: 7 files identical, 8 changed,
  none touching the mapped subset. The new `permissionPrompts` selector,
  `user_message_uuids` correlation fields, task summary documentation, managed
  MCP settings, and the bridge and browser implementation changes are all
  classified unmapped, the credential-bearing subpath declarations are
  byte-identical, and `sdkCompat.harnessSchema` is unchanged, so the behavior
  revision, wire, Node, and sidecar source-tag axes all stay put.
  Every effect is preceded by cleanup authority: open reserves the
  open-guardian, pump, and close-guardian lanes from the exact selected task
  service and starts both guardian tasks before it acquires a credential,
  resolves a resource, starts the sidecar, or contacts the provider. A host that
  cannot commit those lanes, or cannot create those workers, refuses the whole
  operation there; after open returns, activating the cleanup guardian cannot
  fail. One enclosing guardian owns the connection, process, pump, remaining
  turn-deadline task, and both leases, and runs the ordered continuation whole:
  interrupt, native close, host termination request, root observation, pump
  join, working-resource release, credential release. Close hands that set over
  before it returns a future, so a runtime that refuses an elapsed deadline,
  missing time service, or wrong host without polling cannot strand live state.
  Caller expiry, caller cancellation, and a dropped cleanup future all transfer
  the guardian — never the pump alone — under the held exact-host, exact-scope
  reservation rather than joining it on the dropping thread, so no lease is
  released around still-live work and the caller reports unconfirmed cleanup
  without waiting. Cancelling a pending open also releases its guard's cleanup
  signal before that handoff, so the credential and working resource it holds
  are released immediately instead of at the abandoned open deadline. `AcceptedForReap` stays ownership transfer only and never
  becomes join or cleanup evidence. Research 278 and 280, Contracts 009, 010,
  017, 019, 029, and 047, g05.022 card 055.
- add `pi.sdk-sidecar` as a qualified exact `0.84.2` route through the
  application-provisioned Node `22.23.2` runtime, source-tagged sidecar, and
  private `swallowtail-pi-sdk-jsonl-v1` wire. New, load, and resume use the
  exact host-leased cwd with bounded typed replay on load, durable app-owned
  session state, no archive/restore/delete, and no substitution for `pi.rpc`.
  New consumers provision those exact runtime, sidecar, wire, SDK, and session
  directory axes; existing v0.3.3 consumers have no action unless opting into
  the new route. Rollback reverts to `v0.3.3`, omits the route and sidecar
  calls, and does not mix workspace versions or alias `pi.rpc`. Research 181
  and 228, g04.033/card 092 and g04.081.
- bind Cline ACP portable `HarnessMode::Plan` on exact `3.0.55`: optional
  `ClineSessionProfileInput::with_harness_mode(Plan)` advertises
  `HarnessModeSelection(Plan)`, requires unique `session/new` plan membership,
  sends one `session/set_config_option` `{configId: mode, value: plan}`, and
  requires response `mode.currentValue = plan` before readiness. Omission keeps
  current initialize/`session/new`/prompt frames with no mode request or Plan
  claim. Fresh context-losing replacement renegotiates the immutable selection.
  Plan widens no permission, auto-approve, resource, isolation, or account
  authority. Research 240, g04.086.
- bind OpenAI Realtime session-scoped reasoning effort on facade
  `openai-realtime-reasoning-2026-08-27`: admit exact
  `minimal|low|medium|high|xhigh` through
  `OpenAiRealtimeSessionProfileInput::with_reasoning_mode`, encode
  `session.update.session.reasoning.effort`, require matching
  `session.updated` acknowledgement, preserve omission bytes, and retain
  superseded `openai-realtime-2026-07-22` proof. Research 236, g04.084.
- add closed adapter-local `ClaudeCodeMaximumTurns` on `claude-code.headless`:
  `ClaudeCodeRunProfileInput::with_maximum_turns` dispatches one canonical
  `--max-turns <n>` appended to the existing command on the exact versions
  Research 226 probed. The type admits positive 32-bit integers only: exact
  artifacts show the native parser coerces with `Number` and rejects only
  `NaN`, so zero, negatives, fractions, `Infinity`, exponent, hexadecimal,
  grouped-digit, and empty inputs all parse, while the loop guard is a
  truthiness test under which a resolved `0` disables enforcement outright.
  The version gate is the exact probed set, not the qualified window: both
  `UnverifiedNewer` points and the never-published in-range `2.1.230` reject
  before process work. `ClaudeCodePreparedRun::start_run` is the only surface
  that dispatches a bound: `low_level_driver` returns an unbound driver and no
  public seam attaches one to a hand-built `ClaudeCodeHeadlessDriver`, so a
  bound can never be paired with another run's plan or with a run that omitted
  the selection. Omission
  preserves the exact prior argv and the approved environment, and is not a
  claim of unlimited execution because an ambient `CLAUDE_CODE_MAX_TURNS` stays
  authoritative when the flag is absent; explicit argv overrides it
  unconditionally with no environment inspection or mutation. A counted turn is
  one tool-use round trip only, never output tokens, tool calls, provider
  requests, retries, cost, or wall time. Reaching the native bound stays a
  provider failure with no output and unchanged joined cleanup.
  Research 226, g04.079
- add closed adapter-local `LlamaCppReasoningSelection` on `llama-cpp.owned`:
  `Disabled` dispatches canonical `--reasoning off` on exact
  `b10069-178a6c449` through `LlamaCppOwnedServingSelection::with_reasoning`
  and `LlamaCppOwnedDriver::with_reasoning`; omission preserves the current
  no-reasoning-argument launch and every `--ctx-size` row is unchanged.
  `--reasoning on` and `auto` are withheld as an unobservable per-request
  distinction and an exact synonym for the default; `--reasoning-budget` is
  withheld entirely because exact source discards it without a template
  thinking end tag. This is owned-serving dispatch and applied server state
  only: effective and observed reasoning behavior, model reasoning
  capability, and attached-route reasoning support stay unclaimed.
  Research 225, g04.078
- add closed Cursor-local `CursorHeadlessReadMode` on `cursor-agent.headless`:
  `Ask` dispatches canonical `--mode ask` on exact `2026.07.01-41b2de7`,
  `2026.07.23-e383d2b`, `2026.08.04-aaa8809`, and `2026.08.11-e8db854` with
  `ResourceAccess::Read` only; omission keeps `Read` on `--mode plan` and
  `ReadWrite` on no mode; read-write authority and newer unverified releases
  reject before process work; Ask is qualified dispatch and application only,
  claims no locally enforced read-only boundary, and adds no portable
  `HarnessMode`, isolation, permission, tool, approval, or network authority;
  Research 224, g04.077
- add optional portable `HarnessMode::Plan` on `qwen.headless` exact
  `0.21.15`, `0.22.0`, and `0.22.1` as canonical `--approval-mode plan`;
  omission keeps `--approval-mode default`; applied `session_start.permission_mode`
  is observed; `auto-edit|auto|yolo` stay unselected; Research 222, g04.075
- add typed exact-Codex-Exec model verbosity: `CodexModelVerbosity` admits
  `low|medium|high` on exact published `0.147.0`, `0.148.0`, `0.149.0`, and
  `0.149.1` for seven frozen slugs
  through prepared evidence and `--config model_verbosity="<value>"`; omission
  preserves the prior argv; live-catalog acceptance and effective length stay
  unclaimed; Research 213, g04.066
- add typed exact-llama.cpp-owned-`b10069` context-size selection:
  `LlamaCppContextSize` admits `1..=2147483647` and dispatches exact
  `--ctx-size N` through immutable prepared serving evidence; omission
  preserves the prior eleven-argument launch, while runtime acceptance,
  effective context, model fit, allocation, and observation remain unclaimed;
  Research 203, g04.056
- add typed exact-Mistral-Vibe-`2.24.2` caller-decreasing maximum turns:
  `MistralVibeMaxTurns` admits `1..=8` for one headless print child; omission
  preserves `--max-turns 8`, native limit remains provider-failed, and the
  fixed plan agent, trust, working-resource, deadline, cancellation, and
  cleanup boundaries remain unchanged; Research 199, g04.052
- add typed exact-Qwen-Code-`0.21.15` caller-decreasing headless budgets:
  `QwenSessionTurnBudget` admits `1..=24` and `QwenToolCallBudget` admits
  `0..=16` across structured runs, first and resumed turns, and fresh
  replacement children; omission preserves `--max-session-turns 24` and
  `--max-tool-calls 16`, the fixed 60-second wall bound and tool policy remain
  unchanged, and the process-local counters reset for every child; Research
  198, g04.051
- add typed DeepSeek V4 Pro adapter-local non-thinking selection for one-request
  structured runs: `DeepSeekThinkingMode::disabled()` dispatches exact
  `thinking.type=disabled`, omits `reasoning_effort`, and carries no portable
  `ReasoningSelection`; ordinary responses remain available while unexpected
  private `reasoning_content` fails closed, and every direct-continuation path
  remains enabled-only; Research 197, g04.050
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
  restoration; the reasoning-selected path and all direct continuation remain
  fixed to `thinking.type=enabled`, private reasoning replay remains adapter-
  held, and dispatch does not claim provider acceptance or effective depth;
  Research 186, g04.038
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
- raise the Codex CLI qualified ceiling from `0.152.0` to official npm
  `@openai/codex` `0.152.1` on the shared `codex.cli` exec and app-server
  axes. Compatible extension of the existing JSONL and workspace-roots
  behaviors: selected mapped flags and methods stay; the complete
  shipped-tree and source-tree deltas are Guardian, test, and version-bump
  bounded; downloaded official binaries were hashed and never executed.
  Unpublished `0.149.2`, `0.150.2`, and `0.151.1` stay incompatible;
  unpublished `0.152.2` remains visible `UnverifiedNewer`. Feature-specific
  exact version sets stay on the `0.147.0..=0.149.1` probed points.
  Research 275, g05.020.
- raise the Claude Code headless and response-only qualified ceilings from
  `2.1.252` to official npm `@anthropic-ai/claude-code` `2.1.257`. Compatible
  extension of the existing stream-JSON behaviors: selected mapped flags stay;
  `--system-prompt-snapshot`, `--bg` resume wording, `sdk-tools.d.ts` extras,
  and changelog `2.1.257` surfaces stay unmapped. Official `2.1.257` help is
  not byte-identical to frozen `2.1.252`. Unpublished `2.1.244`, `2.1.249`,
  and hop-skipped `2.1.253` through `2.1.256` stay incompatible; unpublished
  `2.1.258` remains visible `UnverifiedNewer`. Watcher stays exact `2.1.251`
  and is not live-ready. Maximum-turn and other feature-specific exact
  version sets stay on the `2.1.220..=2.1.241` probed points. Research 273,
  g05.019.
- raise the Claude Agent ACP qualified ceiling from `0.70.0` to official npm
  `@agentclientprotocol/claude-agent-acp` `0.73.0`. Operator restart after
  official latest moved during the unmerged `0.72.0` family. Compatible
  extension of `claude-agent.acp.initialize-meta-extensions-v7`: selected
  mapped ACP routes stay; `#1004` keeps mapped mode `id`/`category` and
  `plan`/`acceptEdits`; `#1045` leaves steering unmapped and the permission
  callback observable contract unchanged; `0.72.0` effort,
  `user_message_uuid` result attribution, PostModelSwitch mirroring, and
  PreModelSwitch session/new fallback stay classified and selected-compatible
  because Swallowtail still confirms model/effort via explicit
  `set_config_option` plus exact match. `0.73.0` changes only `package.json`
  (version plus Agent SDK pin `0.3.252`→`0.3.257`); every `dist/**` file is
  byte-identical to `0.72.0`. Additive `sessionCapabilities.subagents`,
  native subagent/async-task updates, session titles, Providers API, ACP SDK
  `1.4.0` elicitation rename, and the Agent SDK pin stay unmapped. Published
  intermediates `0.71.0`, `0.72.0`, and `0.73.0` are qualified; unpublished
  `0.58.0` stays incompatible; unpublished `0.74.0` remains visible
  `UnverifiedNewer`. Host `0.63.0` stays observation-only. Claude Code and
  the watcher stay untouched. Research 272, g05.018.
- cap `kimi-code.acp` at `0.38.0` under `QualifiedOnly`. Operator A2: indefinite
  posture change with one artifact-level reopen trigger. Claim identity moved
  from `kimi.acp.executable-window-2` to `kimi.acp.executable-window-5` because
  Contract 029 revises the claim when newer-version posture or exclusions
  change. Frozen historical `window-3` and `window-4` stay reserved.
  Segments stay exact `0.28.1` plus `0.29.0..=0.38.0`; exact `0.39.0`
  and `0.39.1` stay excluded. Unpublished `0.38.1`, unpublished `0.39.2`, and
  farther `0.40.x` fail closed with them. No new ACP behavior revision.
  `kimi-code.local-server` and headless stay `AllowUnverified`. g05.017, card
  043.
- correct the Kimi Code headless compatibility split and qualify the
  agent-core-v2 revision through official npm and GitHub
  `@moonshot-ai/kimi-code` `0.39.1`. The default `kimi -p` engine becomes
  agent-core-v2 `runV2Print` at `0.33.0`, not `0.38.0`:
  `experimental-v2.ts` redefines `isKimiV2Enabled()` there from
  `KIMI_CODE_EXPERIMENTAL_FLAG` truthy to `KIMI_CODE_LEGACY_FLAG` not truthy,
  and this adapter never sets that flag. `kimi.headless.stream-json.v1`
  therefore corrects down from `0.29.0..=0.37.2` to `0.29.0..=0.32.0`, and
  `kimi.headless.stream-json.v2` corrects down and extends from exact
  `0.38.0` to `0.33.0..=0.39.1`. `0.33.0..=0.37.2` had been claimed as
  qualified v1 since g04.064 while emitting the `system.version` preamble the
  v1 decoder rejects; host `0.34.0` sat inside that span and now classifies
  as qualified Maintained v2. Unpublished `0.39.2` remains visible
  `UnverifiedNewer` on the headless axis. Research 270, g05.016.
- stop the Kimi Code ACP axis at `0.38.0` and exclude exact `0.39.0` and
  `0.39.1`. From `0.39.0` the agent-core-v2 ACP terminal runner replaces two
  fail-closed errors with a local host-process spawn in the leased working
  resource, and this route always advertises `terminal: false`, so that
  branch always applies. The route declares `HarnessIsolation::AmbientHost`
  with no isolation claim, Contract 015 denies filesystem containment from
  process ownership and treats a terminal request from a terminal-less client
  as scope-stopping, and no adapter or runtime control mediates the spawn.
  Both points classify `Incompatible` rather than unverified-newer, because
  exclusions are assessed before the newer-version path. The ACP ceiling
  stayed `0.38.0`. That landing left `AllowUnverified` above the named
  exclusions; card 043 later moved the posture to `QualifiedOnly`. No new ACP
  behavior revision was created: wire-shape stability across
  `0.38.0`→`0.39.1` is real and is not sufficient to qualify an authority
  change. Research 270, g05.016.
- raise the Pi RPC qualified ceiling from `0.84.3` to official npm
  `@earendil-works/pi-coding-agent` `0.84.4`. Compatible extension of
  `pi.rpc.strict-lf-v0.84.0-message-update-delta`: selected mapped commands
  and argv stay; `clear_queue`, terminal capability overrides, extension UI
  prompt events, bundled bin path, and streaming `usage` stay unmapped.
  Standing-unused help-unselected `--` and `powershell` remain unmapped
  carry-forwards, not new `0.84.4` deltas. Unpublished `0.83.1` stays incompatible;
  unpublished `0.84.5` remains visible `UnverifiedNewer`. `pi.sdk-sidecar`
  stays exact `0.84.2`. Research 268, g05.015.
- raise the Claude Code headless and response-only qualified ceilings from
  `2.1.251` to official npm `@anthropic-ai/claude-code` `2.1.252`. Compatible
  extension of the existing stream-JSON behaviors: selected mapped flags stay;
  `--restricted`, background-session commands, and watcher flags stay unmapped.
  Official `2.1.252` help is byte-identical to frozen `2.1.251`. Unpublished
  `2.1.244` and `2.1.249` stay incompatible; unpublished `2.1.253` remains
  visible `UnverifiedNewer`. Watcher stays exact `2.1.251` and is not
  live-ready. Maximum-turn and other feature-specific exact version sets stay
  on the `2.1.220..=2.1.241` probed points. Research 266, g05.014.
- raise the Codex CLI qualified ceiling from `0.151.0` to official npm
  `@openai/codex` `0.152.0` on the shared `codex.cli` exec and app-server
  axes. Compatible extension of the existing JSONL and workspace-roots
  behaviors: selected mapped flags stay; `fork`, `--thread-source`,
  `thread/turns/list`, `thread/items/list`, `--code-mode-host`,
  `thread/shellCommand`, and ModelProvider auth-recovery notifications stay
  unmapped. Unpublished `0.149.2`, `0.150.2`, and `0.151.1` stay
  incompatible; `0.152.1` remains visible `UnverifiedNewer`. Feature-specific
  exact version sets stay on the `0.147.0..=0.149.1` probed points.
  Research 264.
- raise the Codex CLI qualified ceiling from `0.149.1` to official npm
  `@openai/codex` `0.151.0` on the shared `codex.cli` exec and app-server
  axes. Compatible extension of the existing JSONL and workspace-roots
  behaviors: selected mapped flags stay; `fork`, `--thread-source`,
  `thread/turns/list`, `thread/items/list`, and `--code-mode-host` stay
  unmapped. Published intermediates `0.150.0` and `0.150.1` are qualified;
  unpublished `0.149.2` and `0.150.2` stay incompatible; `0.151.1` remains
  visible `UnverifiedNewer`. Feature-specific exact version sets stay on
  the `0.147.0..=0.149.1` probed points. Research 262.
- raise the Claude Code headless and response-only qualified ceilings from
  `2.1.241` to official npm `@anthropic-ai/claude-code` `2.1.251`. Compatible
  extension of the existing stream-JSON behaviors: selected mapped flags stay;
  `--restricted`, background-session commands, and watcher flags stay unmapped.
  Published intermediates `2.1.242`, `2.1.243`, `2.1.245`, `2.1.246`,
  `2.1.247`, `2.1.248`, and `2.1.250` are qualified; unpublished `2.1.244` and
  `2.1.249` stay incompatible; `2.1.252` remains visible `UnverifiedNewer`.
  Maximum-turn and other feature-specific exact version sets stay on the
  `2.1.220..=2.1.241` probed points. Research 261, g05.005.
- raise the Qwen headless qualified ceiling through official npm
  `@qwen-code/qwen-code` `0.22.3`: keep exact `0.21.15` reasoning-control,
  extend same-revision `0.22.0..=0.22.3` via published `0.22.2`, keep
  unpublished `0.21.16` incompatible, and leave reasoning/budgets exact
  `0.21.15`. Research 258, g05.004.
- raise the Qwen headless qualified ceiling through official npm
  `@qwen-code/qwen-code` `0.22.1`: keep exact `0.21.15` reasoning-control,
  add same-revision `0.22.0..=0.22.1`, keep unpublished `0.21.16`
  incompatible, and leave reasoning/budgets exact `0.21.15`. Research 216,
  g04.069.
- raise the Pi RPC qualified ceiling through official npm
  `@earendil-works/pi-coding-agent` `0.84.3`: exact published points
  `0.80.10` through `0.84.3`, unpublished `0.83.1` stays incompatible, and
  `0.84.0` keeps private message-update-delta; `toolcall_start` extras,
  `--`, `powershell`, bundled bin path, and streaming `usage` stay
  unmapped. Research 215, g04.068.
- Qualify Kimi Code headless exact `0.38.0` under adapter-private
  `kimi.headless.stream-json.v2` for default agent-core-v2 `runV2Print`
  stream-json. Deprecate `0.29.0..=0.37.2` under
  `kimi.headless.stream-json.v1`. Keep public facade
  `kimi-headless-stream-json-v1`, enforce matching `system.version` preamble at
  runtime, and keep synthetic `0.38.1` permitted `UnverifiedNewer` on the v2
  revision. Research 211, g04.064.
- Retract Kimi Code headless qualified ceiling from `0.38.0` to `0.37.2` under
  existing `kimi.headless.stream-json.v1`. Research 210 proved naked npm
  `0.38.0` `kimi -p` defaults to agent-core-v2 `runV2Print`, not the legacy v1
  corpus Swallowtail audited; the adapter does not set `KIMI_CODE_LEGACY_FLAG`.
  Exact `0.38.0` headless remains visible `UnverifiedNewer` until v2
  stream-json is independently qualified. ACP and local-server `0.38.0`
  qualifications stand. Research 210, g04.063.
- Raised qualified Claude Code headless and response-only ceilings from
  `2.1.238` to official npm `@anthropic-ai/claude-code` `2.1.241`.
  Compatible-extension: official extracted `--help` is byte-identical to
  `2.1.238`. Published intermediates `2.1.239` and `2.1.240` are
  qualified; `2.1.242` remains visible `UnverifiedNewer`. Research 202,
  g04.055.
- Raised qualified Codex CLI ceiling from `0.149.0` to `0.149.1` on the
  `codex.cli` axis (both `codex.exec` and `codex.app-server` routes).
  Compatible-extension: selected mapped flags unchanged, schema bundles
  byte-identical, ModelListParams unchanged. Exec help differs only by
  unmapped `--thread-source`. Research 201, g04.054.
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
- Raised qualified OpenCode HTTP ceiling from `1.18.20` to official npm
  `opencode-ai` `1.18.28` on the `opencode.server` axis. Compatible-extension:
  all eight published patch hops keep `surface-19`; baseline, historical gaps,
  claim id, and `AllowUnverified` stay unchanged. Research 285, g05.028.
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
- advance current source to 40 packages and 49 production routes, including
  `pi.sdk-sidecar` and `claude-agent.sdk`, while preserving the immutable
  `v0.3.3` 40-package, 47-route baseline; publish
  [v0.4.0 candidate release notes](docs/releases/0.4.0.md) with every classified
  break, upgrade, rollback, package, route, known limit, and source-only
  distribution truth

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
