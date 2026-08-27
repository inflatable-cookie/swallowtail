# Per-Route Feature Completion

Status: active programme
Owner: Tom
Created: 2026-08-21
Evidence: `../triage/2026-08-21-advanced-route-features.md`
Live inventory: [Per-Route Feature Inventory](./per-route-feature-inventory.md)
Governing refs: Contracts 011, 020, 024, 037, 040, 041, 047, 052

## Purpose

Work through missing official capabilities on every production route without
flattening provider vocabulary or leaving the assessed feature inventory as a
research-only list.

## Delivery Rule

Use one route family and one coherent control family per numbered roadmap.
Before implementation, recheck the current official surface and classify each
candidate as:

- deliver now through the exact selected transport
- evidence-gated
- intentionally withheld
- not applicable to that route
- obsolete upstream

Only the first class becomes implementation. Every delivered control needs an
exact typed input, immutable plan/evidence binding, fail-closed validation,
deterministic fixtures, guide coverage, and feature-matrix truth. A route-local
name such as Fast, context, effort, thinking, service tier, agent, or team never
becomes a provider-neutral synonym.

## Initial Sequence

After the active Pi and Gemini decisions, start with the exact-transport set
already identified by the assessed inventory:

1. Cursor headless model parameters: Fast, context, and effort
2. Ollama attached `num_ctx`
3. Anthropic Messages `output_config.effort`
4. DeepSeek continuation reasoning controls
5. xAI Responses reasoning and output bounds

Then continue route-by-route through the production matrix. Re-rank only for a
consumer need, an upstream removal, or a shared contract dependency. New route
families do not interrupt this programme by default.

## Progress

- [g04.035 Cursor Headless Model Parameters](./035-cursor-headless-model-parameters.md)
  is complete. Research 183, cards 095-097, and typed headless model-parameter
  dispatch are realized on `cursor-agent.headless`.
- [g04.036 Ollama Attached Context Window](./036-ollama-attached-context-window.md)
  is complete. Research 184, cards 098-100, and adapter-local `options.num_ctx`
  dispatch are realized on `ollama.attached`.
- [g04.037 Anthropic Messages Effort](./037-anthropic-messages-effort.md) is
  complete and merged through PR 37 at `56a7b87b`. Research 185 and cards
  101-103 realize exact `claude-opus-4-7` `output_config.effort` dispatch on
  structured and fixed direct-continuation profiles.
- [g04.038 DeepSeek Continuation Reasoning Controls](./038-deepseek-continuation-reasoning-controls.md)
  is complete and merged through PR 36 at `badb400a`. Research 186 and cards
  104-106 realize exact V4 Pro `low`, `high`, and `max` effort while keeping
  thinking enabled and private continuation adapter-held.
- [g04.039 xAI Responses Reasoning And Output Bounds](./039-xai-responses-reasoning-output-bounds.md)
  is complete and merged through PR 38 at `e9ae1a49`. Research 187 and cards
  107-109 realize exact model-qualified reasoning and positive output-token
  bounds on structured and serial connection-local profiles.
- [g04.040 Copilot CLI ACP Session Effort](./040-copilot-cli-acp-session-effort.md)
  stopped after card 110 and merged through PR 39 at `da0871d5`. Research 188
  proves exact `1.0.80` model-entitles server-start effort and may substitute
  the selected model's default. The route selects no model, so cards 111-112
  are blocked and `reasoning_selection` remains `No`.
- [g04.041 Qwen Headless Reasoning Effort](./041-qwen-headless-reasoning-effort.md)
  is complete and merged through PR 40 at `709d197c`. Research 189 and cards
  113-115 realize exact `0.21.15` process-private reasoning selection for
  `qwen3.8-max` and `qwen3.8-max-preview` across runs, turns, resume, and fresh
  replacement.
- [g04.042 Cline Thinking Controls](./042-cline-thinking-controls.md) stopped
  after card 116 and merged through PR 41 at `27b34c7d`. Research 190 proves
  exact `3.0.55` ACP discards the parsed selection and headless model-entitles
  it while the route selects no model. Cards 117-118 are blocked and both
  `reasoning_selection` cells remain `No`.
- [g04.043 OpenAI Background Hosted Search](./043-openai-background-hosted-search.md)
  stopped after card 119 and merged through PR 42 at `685dbf1a`. Research 191
  proves the individual search fields but not the exact composed background
  route. Cards 120-121 are blocked and no search capability shipped.
- [g04.044 OpenAI Background Reasoning Vocabulary Correction](./044-openai-background-reasoning-vocabulary-correction.md)
  is complete and merged through PR 43 at `bdb7ea88`. Research 191 and cards
  122-123 correct exact GPT-5.6 reasoning to
  `none|low|medium|high|xhigh|max`, remove unqualified `minimal`, version the
  corrected opaque facade truth, and prove early rejection.
- [g04.045 Claude Code Headless Structured Output](./045-claude-code-headless-structured-output.md)
  stopped after card 124 and merged through PR 44 at `8a2640ea`. Research 192
  establishes draft-07 at the exact local validation boundary but admits no
  deliver-now row because runtime linkage, the full keyword subset, an
  immutable retry bound, and valid terminal/lifecycle truth remain unqualified.
  Cards 125-126 are blocked and no schema capability shipped.
- [g04.046 Gemini Live Thinking Levels](./046-gemini-live-thinking-levels.md)
  is complete and merged through PR 45 at `04cc22f2`. Research 193 and cards
  127-129 realize exact `minimal|low|medium|high` dispatch on fixed model
  `gemini-3.1-flash-live-preview`, preserve current `MINIMAL` omission bytes,
  and keep one-rollover/restoration truth under a new opaque facade point.
- [g04.047 Gemini Live Output-Token Maximum](./047-gemini-live-output-token-maximum.md)
  is complete and merged through PR 46 at `c2878262`. Research 194 and cards
  130-132 realize exact positive `1..=65_536`
  `generationConfig.maxOutputTokens` dispatch on fixed model
  `gemini-3.1-flash-live-preview`, preserve omission bytes, compose with every
  admitted thinking level, and keep one-rollover/restoration truth under the
  new opaque facade point.
- [g04.048 Gemini Live Context-Window Compression](./048-gemini-live-context-window-compression.md)
  is complete and merged through PR 47 at `47848056`. Research 195 and cards
  133-135 realize exact default-only
  `contextWindowCompression.slidingWindow = {}` dispatch on fixed model
  `gemini-3.1-flash-live-preview`, preserve omission bytes, withhold explicit
  numeric forms, and retain one-rollover/restoration truth.
- [g04.049 OpenAI Background Service Tier](./049-openai-background-service-tier.md)
  is complete and merged through PR 48 at `06c00e6c`. Research 196 and cards
  136-138 realize adapter-local explicit `service_tier: "default"` dispatch on
  ordinary attached runs and one in-process reattachment. Omission preserves
  prior bytes; detachment and selected-tier restart reconciliation fail closed;
  returned-tier observation and all other values remain withheld.
- [g04.050 DeepSeek Structured-Run Thinking Mode](./050-deepseek-structured-run-thinking-mode.md)
  is complete and merged through PR 49 at `52413da0`. Research 197 and cards
  139-141 realize adapter-local explicit `thinking.type=disabled` only for
  exact V4 Pro one-request structured runs, without a portable reasoning
  selection. Direct continuation stays enabled-only because its bounded tool
  loop requires private reasoning replay.
- [g04.051 Qwen Headless Turn And Tool Budgets](./051-qwen-headless-turn-and-tool-budgets.md)
  is complete and merged through PR 50 at `9807e322`. Research 198 and cards
  142-144 realize exact Qwen Code `0.21.15` caller-decreasing turn budgets
  `1..=24` and tool-call budgets `0..=16` across runs and every turn child.
  Omission preserves `24` / `16`; wall time and tool permissions remain fixed.
- [g04.052 Mistral Vibe Headless Maximum Turns](./052-mistral-vibe-headless-max-turns.md)
  is complete and merged through PR 51 at `2fb24536`. Research 199 and cards
  145-147 realize exact Vibe `2.24.2` caller-decreasing positive maximum turns
  `1..=8`; caller omission preserves `--max-turns 8`, and native limit remains
  provider-failed.
- [g04.053 Qoder Headless Maximum Turns](./053-qoder-headless-max-turns.md) is
  complete as an evidence stop and claim correction through PR 54. Research
  200 admits no deliver-now caller-decreasing row. Historical inert argv
  `--max-turns 8` is retained; factory AgentLoop ceiling is `1000`;
  `error_max_turns` is decoder-only. Cards 149-150 stay blocked. No caller
  max-turns feature ships.
- [g04.056 llama.cpp Owned Context Size](./056-llama-cpp-owned-context-size.md)
  is complete and merged through PR 55 at `54d021e4`. Research 203 and cards
  155-157 realize adapter-local dispatch-only `LlamaCppContextSize` for exact
  owned runtime `b10069-178a6c449`. Values `1..=2147483647` dispatch as
  `--ctx-size N`; omission preserves the no-flag launch. Acceptance,
  effectiveness, observation, model fit, and allocation remain withheld.
- [g04.057 Grok Build ACP Reasoning Selection](./057-grok-build-acp-reasoning-selection.md)
  stopped after card 158 and merged through PR 56 at `0b8639a7`. Research 204
  freezes exact response-channel presence but not effort membership or selected-
  value confirmation. Cards 159-160 are blocked. No reasoning-selection feature
  ships.
- [g04.058 Antigravity Headless Agent Profile Selection](./058-antigravity-headless-agent-profile-selection.md)
  stopped after card 161. Research 205 is an empty deliver-now set: host-local
  listing, missing selected `init.agent`, unproved fail-closed invalid
  `--agent` on the qualified range, and custom-profile authority risk. Cards
  162-163 are blocked. No agent-profile feature ships.
- [g04.059 Deep Agents ACP Model Selection](./059-deepagents-acp-model-selection.md)
  stopped after card 164. Research 206 is an empty deliver-now set: generic
  access profile cannot prove provider agreement before spawn; CLI silently
  retains the default when `--model` lacks a usable value; construction is
  post-spawn; initialize/`session/new` expose no model confirmation. Cards
  165-166 are blocked. No model-selection feature ships.
- [g04.060 Kimi Code ACP Catalogue-Declared Effort Levels](./060-kimi-code-acp-catalogue-declared-effort-levels.md)
  completed and merged through PR 59 at `dc191750`. Research 207 admits exact
  `0.29.0..=0.38.0` snapshot-advertised `xhigh|max` with effective
  confirmation under `kimi.acp.reasoning.declared-effort-v2`. Foreign rows
  coexist without public admission; load/resume remain non-mutating.
- [g04.061 Kimi Code ACP Plan Mode](./061-kimi-code-acp-plan-mode.md)
  completed and merged through PR 60 at `f21220cd`. Research 208 admits exact
  `0.28.1` plus `0.29.0..=0.38.0` `HarnessMode::Plan` with snapshot membership
  and response `currentValue=plan`. No new behavior revision;
  `default|auto|yolo` coexist without public admission; load/resume/import
  remain non-mutating; isolation stays `AmbientHost`.
- [g04.062 Anthropic Messages Adaptive Thinking](./062-anthropic-messages-adaptive-thinking.md)
  completed and merged through PR 61 at `4ef5c5e9`. Research 209 admits exact
  `claude-opus-4-7` adapter-local adaptive omitted-display thinking on
  structured attempts and direct continuation, with bounded private replay and
  no thought disclosure.
- [g04.063 Kimi Code Headless Reasoning Effort](./063-kimi-code-headless-reasoning-effort.md)
  stopped after card 176 and merged through PR 62 at `5f37ff6b`. Research 210
  is an honest empty deliver-now set:
  no headless confirmation transport, ambient thinking-disable shadowing,
  Kimi-protocol default fallback before env override, and no headless catalogue
  snapshot for selected-model agreement. Cards 177-178 stay blocked. No
  headless reasoning-effort feature ships.
- [g04.064 Kimi Code 0.38.0 Headless V2 Useful Newer](./064-kimi-code-0-38-0-headless-v2-useful-newer.md)
  completed through cards 179-180: exact `0.38.0` qualifies under
  `kimi.headless.stream-json.v2`; v1 through `0.37.2` preserved.
- Post-v2 reassessment keeps g04.063 cards 177-178 blocked. Qualifying the v2
  stream does not add effective effort confirmation, a model-effort snapshot,
  ambient configuration authority, or a fail-closed child-environment binding.
- [g04.065 Claude Code Headless Ultracode](./065-claude-code-headless-ultracode.md)
  stopped after card 181. Research 212 admits no deliver-now row: exact
  `2.1.241` help omits `ultracode`, parser accepts the hidden value from
  `2.1.203+`, and dynamic workflow topology remains unbounded for binding.
  Cards 182-183 are blocked.
- [g04.066 Codex Exec Model Verbosity](./066-codex-exec-model-verbosity.md)
  is complete and merged through PR 65 at `46070dfd`. Research 213 admits exact
  `0.147.0`, `0.148.0`, `0.149.0`, and `0.149.1` default openai Responses exec
  for seven frozen slugs × `low|medium|high`. Cards 185-186 bind adapter-local
  `CodexModelVerbosity`. Live-catalog acceptance and effective length stay
  withheld.
- [g04.067 OpenCode HTTP Web Search](./067-opencode-http-web-search.md)
  stopped after card 187. Research 214 admits no deliver-now row: exact
  `v1.18.20` registers `websearch`, but visibility and Exa/Parallel MCP
  backends depend on attached-server provider/env facts and a post-create
  session checksum. Cards 188-189 are blocked.
- [g04.071 Copilot CLI ACP Built-In Tool Allowlist](./071-copilot-cli-acp-built-in-tool-allowlist.md)
  stopped after card 195. Research 218 admits no deliver-now row: exact
  `1.0.80` parses `--available-tools` and stores it on ACP `session/new`, but
  built-in membership, unknown-name failure, available/excluded precedence,
  and host MCP/plugin registry composition remain unbound. Cards 196-197 are
  blocked.
- [g04.072 Grok Build ACP Subagents Disabled](./072-grok-build-acp-subagents-disabled.md)
  stopped after card 198. Research 219 admits no deliver-now row: exact
  `1.0.4`/`1.0.5` parse root `--no-subagents` before `agent stdio`, but
  unauthenticated initialize does not expose applied suppression and spawn-path
  coverage stays unfrozen. Cards 199-200 are blocked.
- [g04.073 Cline Headless Plan Mode](./073-cline-headless-plan-mode.md) is
  complete. Cards 201-203 delivered exact `3.0.55` portable
  `HarnessMode::Plan` as canonical `--plan`. Research 220 promoted. Omission
  retains `--json --auto-approve false -c <cwd> <prompt>`. Observation
  withheld. Plan is provider behavior, not isolation. ACP, `act|yolo|zen`,
  mode switching, authority widening, and live provider work stay out.
- [g04.074 Cline Headless Model Selection](./074-cline-headless-model-selection.md)
  stopped after card 204. Research 221 admits no deliver-now row: exact
  `3.0.55` parses `-m/--model` and `-P/--provider`, but provider identity stays
  ambient without `-P`, explicit `-m` is never validated against membership or
  against the selected provider, and `saveProviderSettings` writes the resolved
  provider and model into shared durable settings before the run with no way to
  disable or scope it. Cards 205-206 are blocked. The g04.042 thinking
  dependency is not removed.
- [g04.075 Qwen Headless Plan Mode](./075-qwen-headless-plan-mode.md) is
  complete. Cards 207-209 delivered exact `0.21.15`, `0.22.0`, and `0.22.1`
  portable `HarnessMode::Plan` as canonical `--approval-mode plan`. Research
  222 promoted. Omission retains `--approval-mode default`. Applied
  `session_start.permission_mode` is observed. Plan is provider behavior, not
  isolation. `auto-edit|auto|yolo`, `/plan`, `set_permission_mode`, writable
  authority, and live provider work stay out.
- [g04.076 Cursor Headless Provider Sandbox](./076-cursor-headless-provider-sandbox.md)
  stopped after card 210. Research 223 is an empty deliver-now set: exact
  `--sandbox enabled` parses on all four qualified builds, but the helper is
  shell-exec only, Darwin "supported" is binary presence, ambient
  `sandbox.json`/config/team/feature-gate state can widen or disable the
  boundary, and print-mode denial is approval rather than process containment.
  Cards 211-212 are blocked. Omission stays `AmbientHost` with no flag.
- [g04.077 Cursor Headless Ask Mode](./077-cursor-headless-ask-mode.md) is
  complete. Cards 213-215 delivered one closed Cursor-local
  `CursorHeadlessReadMode`. Research 224 admits four deliver-now rows:
  `--mode ask` with `ResourceAccess::Read` on each exact qualified build, at
  qualified dispatch and application only. Omission keeps `Read` on
  `--mode plan` and `ReadWrite` on no mode. Read-write authority and newer
  unverified releases reject before process work. Effective and observed mode
  stay withheld: Ask's only local consumer is an inert shell-exec sandbox
  policy type and the qualified stream reports no mode, so no locally enforced
  read-only boundary is claimed. Portable `HarnessMode::Ask`, raw modes,
  `--plan`, Agent selection, and force flags stay out.
- [g04.078 llama.cpp Owned Reasoning Controls](./078-llama-cpp-owned-reasoning-controls.md)
  is complete. Research 225 and cards 216-218 realize adapter-local
  dispatch-only `LlamaCppReasoningSelection::Disabled` for exact owned runtime
  `b10069-178a6c449`, dispatched as `--reasoning off`; omission preserves the
  no-flag launch and every context-size row is unchanged. `off` is the only
  value whose applied server state is template-independent: exact source
  short-circuits `enable_thinking` to `false` before the template is probed.
  `--reasoning on` and `auto` are withheld as an unobservable per-request
  distinction and an exact synonym for the default. `--reasoning-budget` is
  withheld entirely because exact source discards it without a template
  thinking end tag, and no prompt-free channel reports that tag. Effective and
  observed reasoning behavior stay withheld.
- [g04.079 Claude Code Headless Maximum Turns](./079-claude-code-headless-maximum-turns.md)
  is complete. Research 226 and cards 219-221 realize closed adapter-local
  `ClaudeCodeMaximumTurns`, one canonical `--max-turns <n>` appended to the
  existing command on every published `2.1.220..=2.1.241` version. The flag is
  hidden rather than absent: it is registered with `hideHelp()` at every probed
  point. The native parser is far wider than the documented positive domain, so
  the adapter closes it to positive 32-bit integers; the loop guard is a
  truthiness test under which a resolved `0` would be inert. The version gate
  is the exact probed set rather than the qualified window, so both
  `UnverifiedNewer` points and the never-published in-range `2.1.230` reject
  before process work. Explicit argv overrides `CLAUDE_CODE_MAX_TURNS`
  unconditionally, with no environment inspection or mutation, and omission
  keeps the exact prior argv while leaving any ambient value authoritative.
  Prepared `start_run` is the only surface that dispatches a bound, so prepared
  and dispatched state agree by construction rather than comparison. A counted turn is one tool-use round trip.
  Reaching the native bound stays `error_max_turns`, exit `1`, and a provider
  failure with no output. A distinct terminal diagnostic and any portable
  budget vocabulary stay out.
- [g04.080 xAI Responses WebSocket Web Search](./080-xai-responses-websocket-web-search.md)
  stopped after card 222. Research 227 admits no deliver-now row. Official HTTP
  `web_search` and WebSocket body-equivalence are frozen; composed socket
  events, mixed completed output, and citation/terminal mapping are not.
  Cards 223-224 are blocked. Omission remains exact `tools: []`. Host
  networking stays denied.
- [g04.081 Pi SDK Sidecar Reasoning Selection](./081-pi-sdk-sidecar-reasoning-selection.md)
  is complete. Research 228 and cards 225-227 deliver one bounded
  `anthropic/claude-opus-4-5` portable reasoning row on exact Pi `0.84.2`.
  Omission retains exact bootstrap bytes and no portable selection claim when
  options are empty.
- [g04.082 Parallel Per-Route Feature Qualification](./082-parallel-per-route-feature-qualification.md)
  is complete. Cards 228-231 and Research 229-232 close four route-distinct
  candidates with honest empty deliver-now sets: Codex app-server exposes no
  typed confirmable verbosity seam; Gemini headless binds no isolated settings
  seam or effective-value confirmation; Bedrock cannot close model, region,
  inference-profile, account, or returned-state truth; and Ollama exposes no
  exact selected-model `max` membership. PRs 81-84 landed serially through
  `5d9fa3f5`. No binding or acceptance lane follows.
- [g04.083 Parallel Per-Route Feature Qualification II](./083-second-parallel-per-route-feature-qualification.md)
  is complete. Cards 232-234 and Research 233-235 close Claude Code headless
  Fast, Codex exec Fast/service tier, and Gemini CLI ACP thinking with honest
  empty deliver-now sets. Card 235 and Research 236 qualify five future
  session-scoped OpenAI Realtime effort rows. PRs 86, 88, 87, and 85 landed
  serially through `c918d301`.
- [g04.084 OpenAI Realtime Reasoning Effort](./084-openai-realtime-reasoning-effort.md)
  is complete and merged through PR 90 at `266ec857`. Cards 236-237 bind and
  prove Research 236's exact
  `minimal|low|medium|high|xhigh` rows at a new opaque facade point while
  preserving historical and omission behavior.
- [g04.085 Parallel Per-Route Feature Qualification III](./085-third-parallel-per-route-feature-qualification.md)
  is complete. Cards 238-240 and Research 237-239 close Claude Code headless
  autocompaction, Codex app-server personality, and Gemini CLI headless
  sandboxing with honest empty sets. Card 241 and Research 240 qualify one exact
  Cline ACP Plan row. PRs 94, 93, 91, and 92 landed serially through
  `abdaefd2`.
- [g04.086 Cline ACP Plan Mode](./086-cline-acp-plan-mode.md) is ready. Cards
  242-243 bind and prove Research 240's exact new-session
  `HarnessMode::Plan` row through selected-value confirmation before readiness.

## Current Execution Boundary

The normalized inventory has 52 closed original items, 22 active qualification
candidates, one active delivery item, and ten items with no active lane. Execute
g04.086 cards 242-243 as one serial Cline ACP worker lane. Keep g04 open.
Contract 029 currentness remains standing.

## Exclusions

- dangerous permission-bypass defaults
- experimental process-spawning teams as a generic composer toggle
- catalogue observations presented as run controls
- unconfirmed flags or UI-label translation
- sibling-route promotion
- new route families

## Closeout Rule

Each numbered roadmap closes one route-local control family, updates the
feature inventory disposition, and names the next route. The programme may span
generation boundaries; it does not justify bulk implementation or a generation
rollover by itself.
