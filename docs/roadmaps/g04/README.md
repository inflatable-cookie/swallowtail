# g04 Route Readiness And Connection Admission

Status: active
Owner: Tom
Created: 2026-08-19

## Purpose

Give consuming applications a portable library surface for discovering addable
routes, admitting configured connections, collecting or launching required
credentials, observing readiness and updates, and exposing the model list those
connections can actually run.

g04 does not ship a connection server, UI, router, or secret store.
Swallowtail remains mechanism. Persistence is a port with an optional simple
adapter. Poodle, T3 Code, Nucleus, and later consumers own presentation chrome
and selection policy.

## Generation Runway

| Goal | State | Governing refs | First milestone |
| --- | --- | --- | --- |
| Inventory existing instance, access, discovery, catalogue, version, and prepared-facade records against the consumer connection lifecycle. | completed | Contracts 005-006, 008, 014, 020, 029, 032, 037, 047; Spec 011 | `g04.001` |
| Fold inventory into Spec 011 and name contract targets without facade code. | completed | Spec 011; Research 168 | `g04.002` |
| Pin the post-g03 source tree as an immutable tag before facade implementation. | completed | Contract 036 | `g04.003` |
| Promote the readiness/admission contract after that tag. | completed | Contract 057; 006, 008, 010, 014, 015, 017, 029, 032, 037, 047 | `g04.004` |
| Realize the persistence port and optional simple adapter. | completed | Contract 057 | `g04.005` |
| Realize addable-route catalog, admission, and config field descriptors. | completed | Contract 057 | `g04.006` |
| Realize library-max sign-in loops through host ports. | completed | Contracts 057, 006, 010, 014, 017 | `g04.007` |
| Realize readiness refresh, authenticated-subject observation, and Contract 029 updates. | completed | Contracts 057, 006, 029, 032, 047 | `g04.008` |
| Realize the model-presentation overlay without flattening catalogues. | completed | Contracts 057, 020 | `g04.009` |
| Prove representative hosted, installed, and local-runtime shapes and publish a consumer path. | completed | Contracts 011, 037, 052, 057 | `g04.010` |
| Expand addable-route coverage on the proved hosted, installed, and local-runtime shapes. | completed | Contracts 011, 037, 052, 057 | `g04.015` |
| Close remaining 057/047 seams and expand addable coverage on proved shapes. | completed | Contracts 020, 037, 047, 057 | `g04.020` |
| Realize a full Pi SDK sidecar route with exact session attachment. | completed | Contracts 017, 019, 023, 029, 037, 057 | `g04.033` |
| Work through official per-route feature gaps one route and one control family at a time. | active | Contracts 011, 020, 024, 037, 040, 041, 047, 052; per-route feature programme | `g04.037` |

## Planned Next Roadmaps

- [g04.023 047 Presentation Metadata](023-047-presentation-metadata.md) — completed and merged, cards 065-067
- [g04.024 Hosted API-Key Kimi Platform Chat](024-hosted-api-key-kimi-platform-chat.md) — completed and merged through PR 31 at `a08c89a1`, cards 076-078
- [g04.025 Codex 0.149.0 Useful Newer](025-codex-0-149-0-useful-newer.md) — standing currentness, completed
- [g04.026 Qwen Headless 0.21.15 Useful Newer](026-qwen-headless-0-21-15-useful-newer.md) — standing currentness, completed
- [g04.027 Ollama 0.32.15 Useful Newer](027-ollama-0-32-15-useful-newer.md) — standing currentness, completed
- [g04.028 Claude Code 2.1.238 Useful Newer](028-claude-code-2-1-238-useful-newer.md) — standing currentness, completed
- [g04.029 OpenCode HTTP 1.18.20 Useful Newer](029-opencode-http-1-18-20-useful-newer.md) — standing currentness, completed
- [g04.030 Antigravity 1.1.17 Useful Newer](030-antigravity-1-1-17-useful-newer.md) — standing currentness, completed
- [g04.031 Oh My Pi 17.4.0 Useful Newer](031-oh-my-pi-17-4-0-useful-newer.md) — standing currentness, completed
- [g04.032 Kimi Code 0.38.0 Useful Newer](032-kimi-code-0-38-0-useful-newer.md) — standing currentness, completed
- [g04.033 Pi SDK Sidecar Route](033-pi-sdk-sidecar-route.md) — completed and merged through PR 32 at `9aac2dd1`, cards 089-092
- [g04.034 Gemini CLI 0.56.0 Useful Newer](034-gemini-cli-0-56-0-useful-newer.md) — standing currentness, completed
- [g04.035 Cursor Headless Model Parameters](035-cursor-headless-model-parameters.md) — complete, cards 095-097
- [g04.036 Ollama Attached Context Window](036-ollama-attached-context-window.md) — complete, cards 098-100
- [g04.037 Anthropic Messages Effort](037-anthropic-messages-effort.md) — complete, cards 101-103
- [g04.038 DeepSeek Continuation Reasoning Controls](038-deepseek-continuation-reasoning-controls.md) — complete, cards 104-106
- [g04.039 xAI Responses Reasoning And Output Bounds](039-xai-responses-reasoning-output-bounds.md) — complete and merged through PR 38 at `e9ae1a49`, cards 107-109
- [g04.040 Copilot CLI ACP Session Effort](040-copilot-cli-acp-session-effort.md) — stopped after card 110 and merged through PR 39 at `da0871d5`; cards 111-112 blocked
- [g04.041 Qwen Headless Reasoning Effort](041-qwen-headless-reasoning-effort.md) — complete and merged through PR 40 at `709d197c`, cards 113-115
- [g04.042 Cline Thinking Controls](042-cline-thinking-controls.md) — stopped after card 116 and merged through PR 41 at `27b34c7d`; cards 117-118 blocked
- [g04.043 OpenAI Background Hosted Search](043-openai-background-hosted-search.md) — stopped after card 119 and merged through PR 42 at `685dbf1a`; cards 120-121 blocked
- [g04.044 OpenAI Background Reasoning Vocabulary Correction](044-openai-background-reasoning-vocabulary-correction.md) — complete and merged through PR 43 at `bdb7ea88`, cards 122-123
- [g04.045 Claude Code Headless Structured Output](045-claude-code-headless-structured-output.md) — stopped after card 124 and merged through PR 44 at `8a2640ea`; cards 125-126 blocked
- [g04.046 Gemini Live Thinking Levels](046-gemini-live-thinking-levels.md) — complete and merged through PR 45 at `04cc22f2`, cards 127-129
- [g04.047 Gemini Live Output-Token Maximum](047-gemini-live-output-token-maximum.md) — complete and merged through PR 46 at `c2878262`, cards 130-132
- [g04.048 Gemini Live Context-Window Compression](048-gemini-live-context-window-compression.md) — complete and merged through PR 47 at `47848056`, cards 133-135
- [g04.049 OpenAI Background Service Tier](049-openai-background-service-tier.md) — complete and merged through PR 48 at `06c00e6c`, cards 136-138
- [g04.050 DeepSeek Structured-Run Thinking Mode](050-deepseek-structured-run-thinking-mode.md) — complete and merged through PR 49 at `52413da0`, cards 139-141
- [g04.051 Qwen Headless Turn And Tool Budgets](051-qwen-headless-turn-and-tool-budgets.md) — complete and merged through PR 50 at `9807e322`, cards 142-144
- [g04.052 Mistral Vibe Headless Maximum Turns](052-mistral-vibe-headless-max-turns.md) — complete and merged through PR 51 at `2fb24536`, cards 145-147
- [g04.053 Qoder Headless Maximum Turns](053-qoder-headless-max-turns.md) — complete; evidence stop and claim correction, cards 148-150
- [g04.054 Codex 0.149.1 Useful Newer](054-codex-0-149-1-useful-newer.md) — standing currentness, completed
- [g04.055 Claude Code 2.1.241 Useful Newer](055-claude-code-2-1-241-useful-newer.md) — standing currentness, completed
- [g04.056 llama.cpp Owned Context Size](056-llama-cpp-owned-context-size.md) — complete and merged through PR 55 at `54d021e4`, cards 155-157
- [g04.057 Grok Build ACP Reasoning Selection](057-grok-build-acp-reasoning-selection.md) — stopped after card 158 and merged through PR 56 at `0b8639a7`; cards 159-160 blocked
- [g04.058 Antigravity Headless Agent Profile Selection](058-antigravity-headless-agent-profile-selection.md) — planned, card 161 ready; cards 162-163 conditional

g04 has 58 numbered roadmaps: 52 completed milestones, five honest evidence
stops at 040, 042, 043, 045, and 057, and one planned milestone. The operator
keeps the generation active. Execute g04.058 next; do not close g04 without
explicit operator direction.

## Current Checkpoint

- g04.001 through g04.022 are complete. PR 20 is on `main` at `281244db`
- g04.023 is on `main` at `deedc3e4` through PR 23; cards 065-067 are
  complete
- g04.024 is on `main` at `a08c89a1` through PR 31; cards 076-078 are
  complete
- g04.025 standing currentness complete. PR 19 is on `main` at `25fc3e35`
- g04.026 standing currentness is on `main` at `550ba112`: Qwen 0.21.15
  qualified through PR 21
- g04.027 standing currentness is on `main` at `0c528209`: Ollama 0.32.15
  qualified through PR 22
- g04.028 is on `main` at `0cd5735d` through PR 24: Claude Code headless
  and response-only `2.1.238`; cards 079-080 are complete
- g04.029 is on `main` at `3dd72fcf` through PR 25: OpenCode HTTP
  `1.18.20`; cards 081-082 are complete
- g04.030 is on `main` at `a8317ac4` through PR 26: Antigravity catalogue
  and headless `1.1.17`; cards 083-084 are complete
- g04.031 is on `main` at `6d86feb6` through PR 27: Oh My Pi RPC
  `17.4.0`; cards 085-086 are complete
- g04.032 is on `main` at `7889cc63` through PR 30: Kimi Code ACP,
  headless, and local-server `0.38.0`; cards 087-088 are complete
- g04.033 is on `main` at `9aac2dd1` through PR 32: the Pi SDK sidecar
  route is realized and both Pi routes are retained; cards 089-092 are
  complete
- g04.034 is complete: cards 093-094 qualified Gemini CLI `0.56.0` across
  separate ACP and headless axes for enterprise API-key access
- g04.035 is complete: cards 095-097 froze exact Cursor model-parameter
  evidence, added typed headless binding, and proved bounded dispatch
- g04.036 is complete: cards 098-100 froze exact Ollama `num_ctx` evidence,
  added adapter-local binding, and proved bounded native dispatch
- g04.037 is on `main` at `56a7b87b` through PR 37: exact
  `claude-opus-4-7` Messages effort is realized; cards 101-103 are complete
- g04.038 is on `main` at `badb400a` through PR 36: exact V4 Pro
  `low`/`high`/`max` reasoning is realized with thinking fixed enabled; cards
  104-106 are complete
- g04.039 is on `main` at `e9ae1a49` through PR 38: exact Grok 4.5/4.6
  reasoning and positive output bounds are realized; cards 107-109 are complete
- g04.040 stopped after card 110 and merged through PR 39 at `da0871d5`:
  Research 188 proves exact Copilot CLI ACP `1.0.80` model-entitles startup
  effort and may substitute the selected model's default; the route selects no
  model, so cards 111-112 are blocked and no reasoning control shipped
- g04.041 is on `main` at `709d197c` through PR 40: exact Qwen `0.21.15`
  reasoning selection is realized for `qwen3.8-max` and
  `qwen3.8-max-preview`; cards 113-115 are complete
- g04.042 stopped after card 116 and merged through PR 41 at `27b34c7d`:
  Research 190 proves exact Cline `3.0.55` ACP discards thinking selection and
  headless model-entitles it while the route selects no model; cards 117-118
  are blocked and no reasoning control shipped
- g04.043 stopped after card 119 and merged through PR 42 at `685dbf1a`:
  Research 191 proves individual `web_search`, model, bound, source, and
  background fields but not the exact composed route; cards 120-121 are
  blocked and no search behavior shipped
- g04.044 is on `main` at `bdb7ea88` through PR 43: exact GPT-5.6 reasoning is
  corrected to `none|low|medium|high|xhigh|max`; cards 122-123 remove
  unqualified `minimal`, bind a new opaque facade point, and prove rejection
  before effects
- g04.045 stopped after card 124 and merged through PR 44 at `8a2640ea`:
  Research 192 establishes draft-07 at the exact local validation boundary but
  withholds delivery because runtime linkage, the full keyword subset, an
  immutable retry bound, and valid terminal/lifecycle truth remain unqualified;
  cards 125-126 are blocked and no schema capability shipped
- g04.046 is on `main` at `04cc22f2` through PR 45: Research 193 and cards
  127-129 realize exact `minimal|low|medium|high` dispatch for
  `gemini-3.1-flash-live-preview`, preserve `MINIMAL` omission bytes, and keep
  one-rollover/restoration truth under a new opaque facade point
- g04.047 is on `main` at `c2878262` through PR 46: Research 194 and cards
  130-132 realize exact positive `1..=65_536`
  `generationConfig.maxOutputTokens` dispatch for `gemini.live`, preserve
  omission bytes, compose with every admitted thinking level, and keep one-
  rollover/restoration truth under a new opaque facade point
- g04.048 is on `main` at `47848056` through PR 47: Research 195 and cards
  133-135 realize exact default-only
  `contextWindowCompression.slidingWindow = {}` dispatch, preserve omission
  bytes, withhold explicit numeric forms, and keep one-rollover/restoration
  truth under a new opaque facade point
- g04.049 is on `main` at `06c00e6c` through PR 48: Research 196 and cards
  136-138 realize adapter-local explicit `service_tier: "default"` dispatch on
  ordinary attached runs and one in-process reattachment; omission preserves
  prior bytes, detachment and selected-tier restart reconciliation fail closed,
  and returned-tier truth remains unclaimed
- g04.050 is on `main` at `52413da0` through PR 49: Research 197 and cards
  139-141 realize adapter-local explicit `thinking.type=disabled` for exact
  DeepSeek V4 Pro one-request structured runs; enabled reasoning and every
  direct-continuation path remain enabled-only
- g04.051 is on `main` at `9807e322` through PR 50: Research 198 and cards
  142-144 realize exact Qwen Code `0.21.15` caller-decreasing turn budgets
  `1..=24` and tool-call budgets `0..=16` across every child shape
- g04.052 is on `main` at `2fb24536` through PR 51: Research 199 and cards
  145-147 realize exact Mistral Vibe `2.24.2` caller-decreasing maximum turns
  `1..=8`, with omission fixed at `8` and native limit provider-failed
- g04.053 is complete through PR 54 as an evidence stop and claim correction:
  Research 200 empty deliver-now; historical inert `--max-turns 8`; factory
  AgentLoop ceiling `1000`; `error_max_turns` decoder-only; cards 149-150
  blocked
- g04.054 standing currentness is complete: Research 201 and cards 151-152
  qualify Codex exec and app-server through official `0.149.1` without moving
  the generation pointer
- g04.055 standing currentness is complete: Research 202 and cards 153-154
  qualify Claude Code headless and response-only through official `2.1.241`
  without moving the generation pointer
- g04.056 is on `main` at `54d021e4` through PR 55: Research 203 and cards
  155-157 realize dispatch-only `LlamaCppContextSize` `1..=2147483647` on exact
  `llama-cpp.owned` `b10069`; omission preserves the no-flag launch
- g04.057 is stopped on `main` at `0b8639a7` through PR 56: Research 204 and
  card 158 freeze exact Grok ACP response-channel presence but no confirmed
  effort selection; cards 159-160 are blocked
- g04.058 is compiled: Research 205 and card 161 own exact Antigravity agent-
  profile listing, id, dispatch, confirmation, fallback, authority, and
  lifecycle evidence; cards 162-163 are conditional
- Contract 029 currentness remains standing and does not move the generation
  pointer
- g04 remains active at 58 roadmaps; generation closure awaits explicit
  operator direction
- `v0.3.3` remains `51d18620`

## Current Planning Checkpoint

1. g04.033 cards 089-092 executed: the Pi SDK sidecar and Contract 017
   attachment are proved; the recorded disposition retains both Pi routes.
2. g04.035-039 completed the initial five per-route feature families.
3. g04.040 stopped honestly after its exact evidence gate.
4. g04.041 cards 113-115 delivered exact Qwen headless reasoning selection and
   merged through PR 40 at `709d197c`.
5. g04.042 stopped honestly after its exact evidence gate and merged through
   PR 41 at `27b34c7d`.
6. g04.043 stopped honestly after Research 191 and card 119; PR 42 landed the
   evidence at `685dbf1a` without a search claim.
7. g04.044 cards 122-123 corrected the exact GPT-5.6 reasoning vocabulary and
   merged through PR 43 at `bdb7ea88`; Contract 036 now requires a next-minor
   source release without selecting one.
8. g04.045 stopped honestly after Research 192 and card 124; PR 44 landed the
   evidence at `8a2640ea` without a structured-output capability claim. Cards
   125-126 are blocked.
9. g04.046 cards 127-129 delivered exact `gemini.live` thinking levels and
   merged through PR 45 at `04cc22f2`.
10. g04.047 cards 130-132 delivered exact Gemini Live output-token-maximum
    dispatch and merged through PR 46 at `c2878262`.
11. g04.048 cards 133-135 delivered exact default-only Gemini Live context-
    window-compression dispatch and merged through PR 47 at `47848056`.
12. g04.049 cards 136-138 delivered dispatch-only OpenAI Background
    `service_tier: "default"` and merged through PR 48 at `06c00e6c`.
13. g04.050 cards 139-141 delivered exact adapter-local disabled thinking for
    DeepSeek V4 Pro one-request structured runs and merged through PR 49 at
    `52413da0`; direct continuation remains enabled-only.
14. g04.051 cards 142-144 delivered exact Qwen Code `0.21.15` caller-
    decreasing turn and tool-call budgets and merged through PR 50 at
    `9807e322`.
15. g04.052 cards 145-147 delivered exact Mistral Vibe `2.24.2` caller-
    decreasing maximum turns and merged through PR 51 at `2fb24536`.
16. g04.053 completed exact Qoder `1.1.25` maximum-turn evidence stop and
    claim correction as cards 148-150 through PR 54 (no caller binding).
17. g04.054-055 completed standing currentness without moving the programme
    pointer.
18. g04.056 cards 155-157 delivered exact owned llama.cpp adapter-local
    `--ctx-size N` dispatch and merged through PR 55 at `54d021e4`.
19. g04.057 stopped honestly after Research 204 and card 158; PR 56 landed the
    exact response-channel evidence at `0b8639a7` without a reasoning-selection
    claim. Cards 159-160 are blocked.
20. g04.058 and cards 161-163 compile Antigravity headless agent-profile
    selection as one serial evidence-first lane. Card 161 is ready; later cards
    are conditional.
21. g04 remains active at operator direction. Contract 029 remains standing.

New route-family research does not pre-empt this sequence.

## Milestones

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md) —
  post-Pi/Gemini route-local delivery sequence
- [036 Ollama Attached Context Window](./036-ollama-attached-context-window.md) — complete, cards 098-100
- [037 Anthropic Messages Effort](./037-anthropic-messages-effort.md) — complete, cards 101-103
- [038 DeepSeek Continuation Reasoning Controls](./038-deepseek-continuation-reasoning-controls.md) — complete, cards 104-106
- [039 xAI Responses Reasoning And Output Bounds](./039-xai-responses-reasoning-output-bounds.md) — complete and merged through PR 38 at `e9ae1a49`, cards 107-109
- [040 Copilot CLI ACP Session Effort](./040-copilot-cli-acp-session-effort.md) — stopped after card 110 and merged through PR 39 at `da0871d5`; cards 111-112 blocked
- [041 Qwen Headless Reasoning Effort](./041-qwen-headless-reasoning-effort.md) — complete and merged through PR 40 at `709d197c`, cards 113-115
- [042 Cline Thinking Controls](./042-cline-thinking-controls.md) — stopped after card 116 and merged through PR 41 at `27b34c7d`; cards 117-118 blocked
- [043 OpenAI Background Hosted Search](./043-openai-background-hosted-search.md) — stopped after card 119 and merged through PR 42 at `685dbf1a`; cards 120-121 blocked
- [044 OpenAI Background Reasoning Vocabulary Correction](./044-openai-background-reasoning-vocabulary-correction.md) — complete and merged through PR 43 at `bdb7ea88`, cards 122-123
- [045 Claude Code Headless Structured Output](./045-claude-code-headless-structured-output.md) — stopped after card 124 and merged through PR 44 at `8a2640ea`; cards 125-126 blocked
- [046 Gemini Live Thinking Levels](./046-gemini-live-thinking-levels.md) — complete and merged through PR 45 at `04cc22f2`, cards 127-129
- [047 Gemini Live Output-Token Maximum](./047-gemini-live-output-token-maximum.md) — complete and merged through PR 46 at `c2878262`, cards 130-132
- [048 Gemini Live Context-Window Compression](./048-gemini-live-context-window-compression.md) — complete and merged through PR 47 at `47848056`, cards 133-135
- [049 OpenAI Background Service Tier](./049-openai-background-service-tier.md) — complete and merged through PR 48 at `06c00e6c`, cards 136-138
- [050 DeepSeek Structured-Run Thinking Mode](./050-deepseek-structured-run-thinking-mode.md) — complete and merged through PR 49 at `52413da0`, cards 139-141
- [051 Qwen Headless Turn And Tool Budgets](./051-qwen-headless-turn-and-tool-budgets.md) — complete and merged through PR 50 at `9807e322`, cards 142-144
- [052 Mistral Vibe Headless Maximum Turns](./052-mistral-vibe-headless-max-turns.md) — complete and merged through PR 51 at `2fb24536`, cards 145-147
- [053 Qoder Headless Maximum Turns](./053-qoder-headless-max-turns.md) — complete; evidence stop and claim correction, cards 148-150
- [054 Codex 0.149.1 Useful Newer](./054-codex-0-149-1-useful-newer.md) — completed (standing currentness), cards 151-152
- [055 Claude Code 2.1.241 Useful Newer](./055-claude-code-2-1-241-useful-newer.md) — completed (standing currentness), cards 153-154
- [056 llama.cpp Owned Context Size](./056-llama-cpp-owned-context-size.md) — complete and merged through PR 55 at `54d021e4`, cards 155-157
- [057 Grok Build ACP Reasoning Selection](./057-grok-build-acp-reasoning-selection.md) — stopped after card 158 and merged through PR 56 at `0b8639a7`; cards 159-160 blocked
- [058 Antigravity Headless Agent Profile Selection](./058-antigravity-headless-agent-profile-selection.md) — planned, card 161 ready; cards 162-163 conditional
- [035 Cursor Headless Model Parameters](./035-cursor-headless-model-parameters.md) — complete, cards 095-097
- [034 Gemini CLI 0.56.0 Useful Newer](./034-gemini-cli-0-56-0-useful-newer.md) — completed (standing currentness)
- [033 Pi SDK Sidecar Route](./033-pi-sdk-sidecar-route.md) — completed and merged through PR 32 at `9aac2dd1`, cards 089-092
- [001 Route Availability And Readiness Evidence](./001-route-availability-and-readiness-evidence.md) — completed
- [002 Route Readiness Spec And Contract Targets](./002-route-readiness-spec-and-contract-targets.md) — completed
- [003 Current Source Tag Before Readiness](./003-current-source-tag-before-readiness.md) — completed
- [004 Readiness And Admission Contract Promotion](./004-readiness-admission-contract-promotion.md) — completed
- [005 Connection Lifecycle Kernel](./005-connection-lifecycle-kernel.md) — completed
- [006 Addable Catalog, Admission, And Config Fields](./006-addable-catalog-admission-and-config-fields.md) — completed
- [007 Sign-In Loop And Host Ports](./007-sign-in-loop-and-host-ports.md) — completed
- [008 Readiness Refresh, Subject, And Updates](./008-readiness-refresh-subject-and-updates.md) — completed
- [009 Model Presentation Overlay](./009-model-presentation-overlay.md) — completed
- [010 First-Proof Route Inventory](./010-first-proof-route-inventory.md) — completed
- [011 Hosted API-Key Anthropic Messages](./011-hosted-api-key-anthropic-messages.md) — completed
- [012 Installed Codex App-Server](./012-installed-codex-app-server.md) — completed
- [013 Local Ollama Attach](./013-local-ollama-attach.md) — completed
- [014 Connection Lifecycle Consumer Path](./014-connection-lifecycle-consumer-path.md) — completed
- [015 Second-Proof Addable Inventory](./015-second-proof-addable-inventory.md) — completed
- [016 Hosted API-Key DeepSeek Continuation](./016-hosted-api-key-deepseek-continuation.md) — completed
- [017 Cline Stable Clippy Result Large Err](./017-cline-stable-clippy-result-large-err.md) — completed
- [018 Installed Claude Agent ACP](./018-installed-claude-agent-acp.md) — completed
- [019 Local llama.cpp Attached](./019-local-llama-cpp-attached.md) — completed
- [020 Config-Ref Prepare Handoff](./020-config-ref-prepare-handoff.md) — completed
- [021 Unmarked Overlay Rows](./021-unmarked-overlay-rows.md) — completed
- [022 Further Addable Inventory](./022-further-addable-inventory.md) — completed
- [023 047 Presentation Metadata](./023-047-presentation-metadata.md) — completed and merged
- [024 Hosted API-Key Kimi Platform Chat](./024-hosted-api-key-kimi-platform-chat.md) — completed and merged through PR 31 at `a08c89a1`, cards 076-078
- [025 Codex 0.149.0 Useful Newer](./025-codex-0-149-0-useful-newer.md) — completed (standing currentness)
- [026 Qwen Headless 0.21.15 Useful Newer](./026-qwen-headless-0-21-15-useful-newer.md) — completed (standing currentness)
- [027 Ollama 0.32.15 Useful Newer](./027-ollama-0-32-15-useful-newer.md) — completed (standing currentness)
- [028 Claude Code 2.1.238 Useful Newer](./028-claude-code-2-1-238-useful-newer.md) — completed (standing currentness)
- [029 OpenCode HTTP 1.18.20 Useful Newer](./029-opencode-http-1-18-20-useful-newer.md) — completed (standing currentness)
- [030 Antigravity 1.1.17 Useful Newer](./030-antigravity-1-1-17-useful-newer.md) — completed (standing currentness)
- [031 Oh My Pi 17.4.0 Useful Newer](./031-oh-my-pi-17-4-0-useful-newer.md) — completed (standing currentness)
- [032 Kimi Code 0.38.0 Useful Newer](./032-kimi-code-0-38-0-useful-newer.md) — completed (standing currentness)
- [033 Pi SDK Sidecar Route](./033-pi-sdk-sidecar-route.md) — completed and merged through PR 32 at `9aac2dd1`, cards 089-092
