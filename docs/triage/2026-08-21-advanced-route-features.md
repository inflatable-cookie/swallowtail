# 2026-08-21 Advanced Route Features

Status: promoted
Owner: Tom

## Context

Research-only inventory of official CLI/API features a message composer
could offer per production route, and what Swallowtail already exposes.

Seeded with four names:

1. Claude UltraCode
2. Fast mode
3. Agent swarms
4. Context size

Then expanded to official reasoning/effort, output limits, search,
permissions, session/swarm controls, and other composer-surfaceable flags
on each production row in
[provider-route-matrix.md](../guides/provider-route-matrix.md).

47 production routes. Auxiliary hosted catalogue branches
(`alibaba-model-studio.deployable-models`, `gemini.models`,
`openai.models`, `xai.models`) are inventory, not extra inference routes.

This document does not decide product policy, open cards, or change
adapters. Recommendations are not decisions. Tom's local orchestrator
schedules any later implementation.

## Existing Swallowtail Coverage

- Feature matrix
  (`docs/guides/provider-solution-feature-matrix.csv`):
  `output_token_limit`, `reasoning_selection`, `structured_output`,
  `attachments`, `consumer_tool_exchange`, `permission_exchange`,
  `question_exchange`, `external_search`, session lifecycle columns.
  Composite rows stay a solution scoreboard. A `Yes` on one branch never
  promotes a sibling.
- Generation-controls guide
  (`docs/guides/generation-controls-and-input-authority.md`):
  requested/planned/dispatched/accepted/effective/observed stay distinct.
  Do not translate arbitrary UI labels into provider strings. Do not
  emulate reasoning with prompt text. Unsupported controls fail at
  preparation or admission.
- Prepared guides under `docs/guides/*-prepared-integration.md`.
- Adapter typed inputs (`*/prepared*/input.rs`, `selection.rs`).
  Claude Agent / Claude Code effort sets are
  `default|low|medium|high|xhigh|max`. Muse effort includes `ultra`.
  Codex, OpenAI background, Oh My Pi, Ollama, Kimi, Antigravity, DeepSeek
  expose their own exact sets. Many ACP/headless routes expose none.

None of the prepared guides name UltraCode, Fast mode, swarms, or a
selectable context-size control.

## Cross-cutting incompatible reasons

Use these instead of repeating them on every row.

1. **No family promotion.** A `Yes` on one route never promotes another
   route from the same vendor
   (`generation-controls-and-input-authority.md`).
2. **No UI-label translation.** Do not map a composer "Fast" / "Ultra" /
   "Swarm" chip onto a different provider string.
3. **UltraCode is not an API effort.** Official Claude Code docs:
   Ultracode is a Claude Code setting that sends `xhigh` plus dynamic
   workflows. It is not an additional Messages API effort level
   ([model-config](https://code.claude.com/docs/en/model-config.md),
   [workflows](https://code.claude.com/docs/en/workflows)).
4. **Official swarm name is Agent teams, not swarm.** Claude Code:
   `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`
   ([agent-teams](https://code.claude.com/docs/en/agent-teams.md)).
   No production route documents a flag named `--swarm`.
5. **Context size is usually model evidence, not a portable control.**
   Catalogues may report windows. Few routes expose a composer-settable
   token window on the selected transport.
6. **Dangerous permission flags stay withheld.** Swallowtail does not
   pass `--yolo`, `--auto-approve true`, `--dangerously-skip-permissions`,
   `--allow-all`, or `--trust-all-tools` on qualified routes. Official
   existence is not an invitation to surface them.
7. **ACP session/new is thin.** Copilot official ACP docs: tool filter
   and reasoning are server-start flags, not `session/new` fields.

## Seed features

### 1. Claude UltraCode

Official names (Claude Code / Agent SDK, 2026):

- Product: **Ultracode** (Claude Code setting, not a model)
- CLI: `--effort ultracode` (v2.1.203+)
- Interactive: `/effort ultracode`
- Settings / Agent SDK: `"ultracode": true`;
  `applyFlagSettings()` accepts `effortLevel: "ultracode"`
- Behavior: sends model effort `xhigh` and turns on dynamic workflow
  orchestration for substantive tasks. Session-only. Not accepted by
  persisted `effortLevel` or `CLAUDE_CODE_EFFORT_LEVEL`.
- Sources:
  [model-config](https://code.claude.com/docs/en/model-config.md),
  [cli-reference](https://code.claude.com/docs/en/cli-reference),
  [workflows](https://code.claude.com/docs/en/workflows)

| Route | Swallowtail already has | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- |
| `claude-code.headless` | `--effort` for `default\|low\|medium\|high\|xhigh\|max` via `with_reasoning_mode` (`claude_code_command.rs`, guide) | `--effort ultracode` not in the admitted set | only-if: qualify v2.1.203+ and treat as setting + `xhigh`, not a seventh model effort | UltraCode ≠ API effort; generation-controls forbid UI-label translation |
| `claude-code.response-only` | same `--effort` set (`claude_code_response_command.rs`) | `--effort ultracode` not admitted | only-if: same as headless; response-only also suppresses tools/MCP | same; plus response-only has no workflow/tool surface |
| `claude-agent.acp` | ACP `session/set_config_option` `effort` for `default\|low\|medium\|high\|xhigh\|max` | Agent SDK `effortLevel: "ultracode"` / `"ultracode": true` not mapped | only-if: confirm ACP config option vs Agent SDK control request on the qualified adapter range | UltraCode is Claude Code/SDK, not a portable ACP effort enum |
| `anthropic.messages` | `maximum_output_tokens`; no reasoning column (`No`) | none for UltraCode | no | UltraCode is not a Messages API field |
| `anthropic.managed-agent` | operator-owned agent version owns model config; no independent reasoning | none for UltraCode | no | Managed Agents use `model.effort` / `model.speed`, not UltraCode |
| `bedrock.runtime` | `maximum_output_tokens` only | none for UltraCode | no | Fast/UltraCode not on Bedrock (Claude Code fast-mode docs exclude Bedrock) |
| every other production route | n/a | not official | no | do not flatten |

### 2. Fast mode

Official names differ by vendor. Do not share one composer chip.

**Claude Code** — product **Fast mode**; `/fast`; settings
`"fastMode": true`; print mode
`claude -p --settings '{"fastMode": true}'`. Opus 5 / Opus 4.8 only.
Research preview. Usage credits. Not on Bedrock / Vertex / Foundry.
Source: [fast-mode](https://code.claude.com/docs/en/fast-mode).

**Codex CLI** — `/fast` toggles a catalog-advertised Fast **service
tier**. Persist with `service_tier = "fast"` and
`[features].fast_mode = true`. Not a `--fast` flag.
Source: [Codex CLI reference](https://developers.openai.com/codex/cli/reference),
[Codex speed](https://developers.openai.com/codex/speed).

**Cursor Agent** — `/fast` toggles Fast when the current model supports
it; headless uses `--model` parameter syntax
`id[fast=true]` / `id[fast=false]`.
Sources: [CLI changelog](https://cursor.com/docs/cli/changelog),
[subagents model parameters](https://cursor.com/docs/subagents.md).

**Anthropic Managed Agents** — `model.speed`: `"standard"` or `"fast"`.
Source: [agent-setup](https://platform.claude.com/docs/en/managed-agents/agent-setup).

**Antigravity** — interactive `/fast` listed as "Enable fast mode (bypass
reasoning plans)". Distinct from Claude/Codex Fast.
Source: [CLI reference](https://www.antigravity.google/docs/cli/reference/).
Modes page says legacy `/fast` was removed in `1.1.0` as an execution
mode; treat currentness as unresolved if the two official pages disagree.

| Route | Swallowtail already has | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- |
| `claude-code.headless` | no Fast setting; effort only | `--settings '{"fastMode": true}'` / `fastMode` | only-if: subscription + usage credits + Opus 5/4.8 + qualified CLI | not an effort label; not on API-key-only or Bedrock |
| `claude-code.response-only` | no Fast setting | same `fastMode` | only-if: same gating; print-mode Fast needs settings at launch | same |
| `claude-agent.acp` | no Fast config option | not confirmed as an ACP `set_config_option` | no until official ACP field is cited | do not invent an ACP Fast key |
| `anthropic.managed-agent` | no `model.speed` | official `model.speed: "fast"` | only-if: agent-version create/update, not per-run Swallowtail input today | operator-owned agent version owns model config |
| `anthropic.messages` | no speed field | Messages `output_config.effort` exists; Fast mode on the Claude API is a separate platform feature | only-if: confirm Messages request field vs Console-only Fast | do not copy Claude Code `/fast` onto Messages |
| `codex.exec` | reasoning + search; no service tier | `service_tier` / `features.fast_mode` via `--config` | only-if: model catalog advertises Fast; ChatGPT credit vs API-key billing differ | Codex Fast ≠ Claude Fast |
| `codex.app-server` | session reasoning + plan mode; no Fast | `/fast` / `service_tier` | only-if: app-server config mapping is qualified | same |
| `cursor-agent.headless` | `--model` exact id only; no bracket params | `--model id[fast=true]` | yes: official headless `--model` parameter | Cursor Fast ≠ Claude Fast |
| `cursor-agent.acp` | no model/fast on prepared open | `/fast` and model params are interactive | only-if: ACP session config advertises Fast | ACP prepared path has no SessionOptions |
| `antigravity.headless` | `--effort low\|medium\|high` | interactive `/fast` (if still current) | no until headless flag is confirmed | official pages disagree; do not invent `--fast` |
| `openai.background` | reasoning + `max_output_tokens` | Responses `service_tier` not on this route's profile | only-if: official Responses field is bound on the exact background facade | do not flatten Codex `/fast` onto hosted Responses |
| every other route | n/a | not official as "Fast mode" | no | do not flatten |

### 3. Agent swarms

No official `--swarm` flag found on any production route's CLI/API.

Closest official names:

- Claude Code **Agent teams** (experimental, off by default):
  `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`; `--teammate-mode`
  `in-process|auto|tmux|iterm2`; `claude agents`.
  Source: [agent-teams](https://code.claude.com/docs/en/agent-teams.md).
- Claude Code **subagents** / `--agents` JSON / Agent tool (ordinary
  delegation, not teams).
- Claude Code **dynamic workflows** (what UltraCode turns on).
- xAI Responses **`grok-4.20-multi-agent`**: `reasoning.effort` controls
  **agent count** (4 or 16), not thinking depth.
  Source: [xAI reasoning](https://docs.x.ai/developers/model-capabilities/text/reasoning).
- Codex config sample: `[features].multi_agent = true`.
- Grok CLI: `--no-subagents`.
- OpenCode: `task` permission key (subagent type).
- Cline README (sibling of official cli-reference): `-y, --yolo`
  disables spawn/team tools; `--team-name`. Official
  [cli-reference](https://docs.cline.bot/cli/cli-reference) help text
  does not list those two; treat as unconfirmed on the help-menu source
  of truth.
- Deep Agents: built-in subagents; `--skills` / `--memory`.

| Route | Swallowtail already has | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- |
| `claude-code.headless` | fixed `--permission-mode plan`, `--tools Read,Glob,Grep`, `--no-session-persistence` | `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS`; `--teammate-mode`; `--agents` | no for teams (experimental env changes process topology); only-if for `--agents` JSON after qualification | official name is Agent teams, not swarm; headless is one-prompt |
| `claude-code.response-only` | `--tools ""` | none useful | no | tool-free route cannot host teams |
| `claude-agent.acp` | no public subagent control | Agent teams / `--agents` not on ACP spawn | no | ACP adapter does not pass extra argv; teams need env + extra processes |
| `xai.responses-websocket` | model + prompt only | `grok-4.20-multi-agent` effort-as-agent-count | only-if: that model is admitted on this facade | do not flatten onto `grok-build.acp` |
| `grok-build.acp` | no public task-control | `--no-subagents` | only-if: ACP spawn argv is extended and qualified | 0.2.117 task-control is private compatibility, not public authority |
| `codex.app-server` / `codex.exec` | no multi-agent feature flag | `features.multi_agent` | only-if: official config key is qualified on that branch | Codex feature flag ≠ Claude teams |
| `opencode.http` | permission/question callbacks; no subagent control | `task` permission / subagent type | only-if: catalogue and permission vocabulary already cover it | do not invent a swarm facade |
| every other route | observational child-agent activity at most | not official as swarm | no | do not invent `--swarm` |

### 4. Context size

Official composer-settable window controls (confirmed):

- Claude Code: `--autocompact <auto\|tokens>` / `/autocompact`
  (v2.1.221+). Model context (including 1M) is model selection, not a
  separate size flag.
  Source: [cli-reference](https://code.claude.com/docs/en/cli-reference).
- Cursor: `--model id[context=300k]` / `[context=1m]`.
  Source: [subagents](https://cursor.com/docs/subagents.md).
- Ollama native API: `options.num_ctx`; serve default
  `OLLAMA_CONTEXT_LENGTH`.
  Source: [Ollama FAQ](https://docs.ollama.com/faq),
  [chat API](https://docs.ollama.com/api/chat).
- llama.cpp server: `-c, --ctx-size`.
  Source: [llama-server README](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md).
- Gemini Live: `contextWindowCompression` /
  `triggerTokens` / sliding window.
  Source: [Live API](https://ai.google.dev/api/live).
- Kimi Code config: `max_context_size` on model entries (config, not a
  per-prompt flag).
  Source: [Kimi overrides](https://moonshotai.github.io/kimi-cli/en/configuration/overrides.html).
- Kimi local-server catalogue already reads `max_context_size` as
  inventory evidence (`local_server/catalogue.rs`). Not a composer set.

| Route | Swallowtail already has | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- |
| `claude-code.headless` / `claude-code.response-only` | no autocompact | `--autocompact` | yes as session compact threshold, not "context size" | do not relabel as context size |
| `cursor-agent.headless` | exact `--model` id | `[context=…]` parameter | yes | Cursor-only syntax; do not copy onto other Cursor transports without proof |
| `cursor-agent.acp` | no model params | same parameter language if ACP accepts it | only-if: ACP initialize/config advertises it | prepared ACP has no SessionOptions |
| `ollama.attached` | reasoning + `maximum_output_tokens` + schema | `options.num_ctx` | yes | output-token limit ≠ context window |
| `llama-cpp.owned` | serving start/stop only | `--ctx-size` on `llama-server` | only-if: serving profile, not inference composer | owned route has no inference role |
| `llama-cpp.attached` | `maximum_output_tokens` | per-request ctx not on OpenAI-chat facade | no | attached inference talks HTTP/SSE; ctx is server boot |
| `gemini.live` | media config + one rollover | `contextWindowCompression` | only-if: Live setup field is qualified | not an output-token max |
| `kimi-code.local-server` | catalogue `max_context_size` evidence | no per-turn setter | no | inventory ≠ control |
| catalogue-only routes | model list may include windows | not a run control | no | catalogue is not a composer generation surface |
| hosted APIs | model windows are provider facts | picking a 1M model is model selection | no as a separate "context size" knob | do not invent a portable context-size control |

## Per-route notes

Each note is one official feature/flag Swallowtail does not expose, or
an explicit N/A. Seed items above are not repeated unless the route
needs a local detail.

### `antigravity.catalogue`

Catalogue only (`agy models`). No composer generation controls.
Official headless flags do not apply.

### `antigravity.headless`

Official headless flags
([headless](https://antigravity.google/docs/cli/headless/)):
`--model`, `--effort low|medium|high`, `--agent`, `--json-schema`,
`--mode` (`default|accept-edits|plan`), `--sandbox`,
`--dangerously-skip-permissions`, `--continue`, `--conversation`,
`--print-timeout`.

Swallowtail already: `--model`, `--effort`, `--mode plan` from
`ResourceAccess::Read`, `--sandbox` from `ProviderEnforced`,
`--json-schema`, exact `--conversation` on continuation. Does not pass
`--dangerously-skip-permissions` or `--continue`.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Agent profile | `--agent` | not passed | yes | no for now: Research 205 empty set — host-local `agy agents`; missing selected `init.agent`; unproved fail-closed invalid `--agent` on qualified range; custom profiles may change tools/instructions | do not flatten onto Claude `--agent` |
| Permission skip | `--dangerously-skip-permissions` | withheld | official flag unused | no | dangerous-permission withhold |
| Output tokens | none official | none | none | no | — |
| Search | not a headless flag | `external_search` No | none confirmed | no | do not invent |
| Fast / UltraCode / swarm / ctx | see seed | — | `/fast` unresolved | no | official pages disagree |

### `codex.exec`

Official: `--search` (live vs default cached `web_search`),
`--sandbox`, `--ask-for-approval`, `--config` for
`model_reasoning_effort`, `service_tier`, `model_verbosity`,
`plan_mode_reasoning_effort`, `features.fast_mode`,
`features.multi_agent`
([CLI reference](https://developers.openai.com/codex/cli/reference),
[config sample](https://developers.openai.com/codex/config-sample)).

Swallowtail already: reasoning, one image, JSON Schema, host-approved
external search, fixed `approval_policy=never` + `--sandbox read-only`.
Matrix: `output_token_limit` No.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Fast tier | `service_tier` / `features.fast_mode` / `/fast` | not mapped | yes | only-if: catalog advertises Fast | see seed |
| Verbosity | `model_verbosity` | not mapped | yes | yes | not reasoning |
| Writable sandbox / approvals | `--sandbox workspace-write`, `--full-auto` | fixed read-only / never | withheld | no | route posture is structured read-only exec |
| Multi-agent feature | `features.multi_agent` | not mapped | yes | only-if | not a swarm |
| Output-token cap | no portable Codex limit | none | none | no | matrix No |

### `codex.app-server`

Swallowtail already: reasoning, `HarnessMode::Plan`, consumer tools,
typed questions, load/resume/import/history/archive/restore/delete.
No output-token limit. No exec-style search on this branch.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Fast tier | `/fast`, `service_tier` | not mapped | yes | only-if: app-server config path qualified | do not copy exec `--search` here |
| Verbosity | `model_verbosity` | not mapped | yes | yes | — |
| Personality | `personality` / `/personality` | not mapped | yes | only-if | product tone, not generation control |
| Plan-mode effort | `plan_mode_reasoning_effort` | session reasoning only | yes | only-if: keep distinct from turn reasoning | generation-controls: exact mapping |
| Multi-agent | `features.multi_agent` | not mapped | yes | only-if | — |

### `claude-agent.acp`

Swallowtail already: model, effort
`default|low|medium|high|xhigh|max`, plan mode, one-shot
`allow_once`/`reject_once`, load/resume/delete. Matrix: output limit
No, search No.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| UltraCode | Agent SDK `effortLevel: "ultracode"` | not in effort set | yes | only-if | see seed |
| Fast mode | Claude Code `fastMode` | not an ACP option (unconfirmed) | unconfirmed | no | do not invent |
| Agent teams | experimental env | not passed | yes | no | process topology |
| Output tokens | CLI `--max-budget-usd` / `--max-turns` are Claude Code print flags | not ACP fields | none on ACP | no | do not copy print flags onto ACP |
| Search | not on this ACP profile | No | none confirmed | no | — |

### `claude-code.headless`

Official print/CLI flags
([cli-reference](https://code.claude.com/docs/en/cli-reference)):
`--effort` including `ultracode`, `--permission-mode`
`default|acceptEdits|plan|auto|dontAsk|bypassPermissions`,
`--settings`/`fastMode`, `--autocompact`, `--max-turns`,
`--max-budget-usd`, `--json-schema`, `--fallback-model`, `--advisor`,
`--agents`, `--teammate-mode`, `--bare` (guide: do not select).

Swallowtail already: `--effort` minus `ultracode`, fixed
`--permission-mode plan`, `--tools Read,Glob,Grep`,
`--no-session-persistence`.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| UltraCode | `--effort ultracode` | omitted | yes | only-if | see seed |
| Fast mode | `fastMode` / `--settings` | omitted | yes | only-if | see seed |
| Agent teams | env + `--teammate-mode` | omitted | yes | no | see seed |
| Autocompact | `--autocompact` | omitted | yes | yes | not context-size |
| Max turns | `--max-turns` | closed adapter-local `ClaudeCodeMaximumTurns` on qualified `2.1.220..=2.1.241` | closed | delivered | print-mode only; positive integers only |
| Spend cap | `--max-budget-usd` | omitted | yes | only-if: API-key billing; subscription spend is different | — |
| JSON Schema | `--json-schema` | omitted | yes | only-if: qualify on this stream-json route | response-only explicitly rejects schema |
| Fallback model | `--fallback-model` | omitted | yes | no | Swallowtail forbids implicit fallback |
| Advisor | `--advisor` | omitted | yes | only-if | extra model, extra spend |
| Permission modes | `--permission-mode` set | fixed `plan` | other modes unused | only-if: consumer-mediated; never `bypassPermissions` by default | dangerous-permission withhold |

### `claude-code.response-only`

Same official CLI. Swallowtail forces `--tools ""`, `--safe-mode`, no
working resource.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| UltraCode / Fast / autocompact | same CLI | omitted | yes | only-if: still tool-free | UltraCode workflows need tools |
| `--json-schema` / `--fallback-model` / `--resume` | official | explicitly rejected | withheld | no | route contract |

### `cursor-agent.catalogue`

Catalogue only. No generation controls.

### `cursor-agent.acp`

Official ACP command is `agent acp` (hidden)
([parameters](https://cursor.com/docs/cli/reference/parameters)).
Interactive `/fast`, `/model` params, `/max-mode`.

Swallowtail: working resource only; observational permissions; no
SessionOptions.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Fast / effort / context | model params / `/fast` | not on prepared open | yes | only-if: ACP config options exist | no family promotion from headless |
| `--mode ask` | official | not selected | yes | only-if | plan/ask/agent are Cursor modes |
| Sandbox / `--force` | official | not selected | withheld | no | matrix: optional sandbox not selected |

### `cursor-agent.headless`

Official: `--model`, `--mode plan|ask`, `--sandbox enabled|disabled`,
`--force`/`--yolo`, `--trust`, `--resume`, `--continue`.
Model params: `fast`, `effort`, `context`.

Swallowtail: `--print --output-format stream-json --model --trust`
and `--mode plan` for Read. Does not pass `--sandbox`, `--force`,
`--yolo`, `--stream-partial-output`.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Fast | `--model id[fast=true]` | omitted | yes | yes | see seed |
| Context window | `--model id[context=300k\|1m]` | omitted | yes | yes | see seed |
| Effort | `--model id[effort=high]` | omitted | yes | yes | matrix `reasoning_selection` No until qualified |
| Ask mode | `--mode ask` | only plan vs agent via access | yes | yes | — |
| Sandbox | `--sandbox enabled` | not selected | yes | only-if | guide: optional sandbox not requested |

### `gemini-cli.acp`

Official: `--approval-mode default|auto_edit|yolo|plan`, `--sandbox`,
`--yolo` deprecated alias
([config.ts](https://github.com/google-gemini/gemini-cli/blob/main/packages/cli/src/config/config.ts),
[configuration](https://github.com/google-gemini/gemini-cli/blob/main/docs/cli/configuration.md)).
Thinking via `modelConfig.thinkingConfig` (`thinkingBudget` /
`thinkingLevel`) in settings, not a first-class Swallowtail control.

Swallowtail: `--approval-mode plan` (Read) or `auto_edit` (RW).
Family rejects reasoning, output limits, structured output, search.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| YOLO | `--approval-mode=yolo` | withheld | official unused | no | dangerous-permission withhold |
| Sandbox | `--sandbox` / `GEMINI_SANDBOX` | not forced | yes | only-if | guide: does not force Gemini sandbox |
| Thinking | `thinkingConfig` | rejected | yes | only-if: family reasoning disposition changes | matrix No; do not translate UI labels |

### `gemini-cli.headless`

Official same plus `--output-format stream-json`. Swallowtail: plan
approval, extensions/MCP disabled, no `--sandbox`/`--yolo`/`--resume`.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Sandbox | `--sandbox` | not passed | yes | only-if | same as ACP |
| Thinking / output cap | settings / not CLI-qualified here | rejected | yes | only-if | family-wide rejection |

### `grok-build.acp`

Official CLI
([docs.x.ai CLI reference](https://docs.x.ai/build/cli/reference)):
`--effort`, `--always-approve`/`--yolo`, `--sandbox`, `--max-turns`,
`--no-plan`, `--no-subagents`, `--disable-web-search`, `--allow`/`--deny`.
Official ACP docs do not document an effort field. Frozen 1.0.4/1.0.5
changelog pages do not say ACP clients can select effort on open or resume.

Swallowtail: `grok --no-auto-update agent stdio`. Session-negotiated
models. Matrix `reasoning_selection` No. 1.0.4–1.0.5 binds `grok-4.6`
and advertises `low|medium|high|xhigh` without a public reasoning claim. Exact
1.0.5 accepts a fail-open `_meta.reasoningEffort` open-time hint; exact
`x.ai/sessionConfig.options` effort membership and selected-value confirmation
remain unfrozen. Exact 1.0.4 has the parser without the new-session apply path.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Effort | CLI `--effort`; exact 1.0.5 unconfirmed open-time hint | not a public selection | yes | only-if: freeze exact response membership and selected-value confirmation | do not infer official ACP open effort or flatten onto `xai.responses-websocket` |
| Max turns | `--max-turns` | not passed (headless flag; ACP spawn has no extra argv) | yes | only-if | official note: `--max-turns` is headless `-p` |
| Web search disable | `--disable-web-search` | not passed | yes | only-if | search column is No |
| Subagents off | `--no-subagents` | not passed | yes | only-if | not a swarm control |
| YOLO | `--always-approve` | withheld | official unused | no | permission-stop truth |

### `kimi-code.acp`

Official CLI
([overrides](https://moonshotai.github.io/kimi-cli/en/configuration/overrides.html)):
`--thinking` / `--no-thinking`, `--plan`, `--yolo`/`--afk`, `-m`.

Swallowtail: ACP `SessionOptions` reasoning
`off|on|low|medium|high|xhigh|max` with snapshot membership, and optional
new-session `HarnessMode::Plan`. Load/resume/import do not redeclare either
control. No permission/question exchange. Headless reasoning is a different
branch.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Plan mode | `--plan` / `default_plan_mode`; ACP `mode=plan` | portable `HarnessMode::Plan` on exact `0.28.1` and `0.29.0..=0.38.0` new sessions | no for ACP plan | delivered | do not copy `--plan` argv onto ACP spawn; `auto|yolo` stay private |
| YOLO / AFK | `--yolo`, `--afk` | withheld | official unused | no | — |
| Effort `xhigh`/`max` | config `[thinking].effort` | portable `xhigh|max` on exact `0.29.0..=0.38.0` when advertised and confirmed | no for ACP snapshot-advertised rows | delivered | exact mapping; no UI labels |

### `kimi-code.headless`

Exact selected TypeScript package `0.38.0`: `kimi --model --prompt
--output-format stream-json`. Current official `--thinking` documentation is
for the newer Python CLI line and does not amend this exact package claim.
Earlier exact prompt-mode evidence says `--plan` cannot compose with prompt
mode.

Swallowtail: model + prompt + stream-json. Matrix: reasoning No on
TypeScript headless. Does not enable
`KIMI_CODE_EXPERIMENTAL_FLAG`.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Thinking | current Python CLI `--thinking` / `--no-thinking`; not frozen on exact TypeScript `0.38.0` | not passed | unconfirmed on selected route | no until exact package evidence | no cross-product/version-line promotion |
| Plan | exact earlier package exposes `--plan` but rejects prompt composition | not passed | incompatible on selected headless command | no | prompt-mode evidence is binding |
| Effort levels | `KIMI_MODEL_THINKING_EFFORT` | not passed | yes | only-if | env override ≠ typed input |

### `muse-code.headless`

Swallowtail already requires `--reasoning-effort`
`none|minimal|low|medium|high|xhigh|ultra`. `ultra` is Muse effort, not
Claude UltraCode. Guide prohibits web tools, writes, shell, session log.
No official UltraCode/Fast/swarm/context-size flags confirmed on this
signed payload.

### `command-code.headless`

Swallowtail-recorded official argv: `--permission-mode plan`,
`--max-turns 8`, `--no-session` / private `--resume`, never `--yolo`,
never `--continue` / `--fork-session`. Independent official page not
re-fetched this pass; treat extra flags as unconfirmed.

| Feature | Official (guide-recorded) | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Max turns | `--max-turns` | fixed 8 | consumer override unused | only-if: confirm official range | — |
| Effort | not recorded | none | unconfirmed | no | do not invent |

### `cline.acp`

Official
([cli-reference](https://docs.cline.bot/cli/cli-reference),
[ACP](https://docs.cline.bot/usage/acp)):
`cline --acp`; `--thinking none|low|medium|high|xhigh`; `-p --plan`;
`-m --model`; `--auto-approve` (ACP default false).

Swallowtail: `cline --acp` with optional exact-package new-session
`HarnessMode::Plan` selected through ACP configuration. Observe/cancel
permissions. Never `allow_always`. No `--auto-approve true`.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Thinking | `--thinking` | not passed | yes | yes | ACP spawn flag, like Copilot effort |
| Plan | `-p, --plan`; ACP `mode=plan` | portable `HarnessMode::Plan` on exact `3.0.55` new sessions | no for the qualified ACP row | delivered | do not copy root `--plan` onto ACP spawn or infer containment |
| Model | `-m, --model` | caller-supplied / harness default | yes | yes | — |
| Auto-approve | `--auto-approve true` | withheld | official unused | no | guide withhold |

### `cline.headless`

Official: `cline --json --auto-approve <bool> -p --thinking -m -t`.

Swallowtail: `--json --auto-approve false`, optional canonical `--plan`.
No model, no thinking. CLI `--timeout` unselected (host deadline used).
Omission is not implicit Plan. Plan is provider behavior, not isolation.
`-m` is evidence-closed at this package point by Research 221.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Thinking | `--thinking` | not passed | yes | yes | do not flatten onto `cline.acp` without repeating qualification |
| Plan | `-p` / `--plan` | optional `HarnessMode::Plan` as `--plan` | no | delivered | Research 220; exact `3.0.55` only; `act\|yolo\|zen` withheld |
| Model | `-m` | not passed | evidence-closed | no | Research 221; exact `3.0.55` leaves provider ambient, never validates `-m`, and persists the selection to shared settings |

### `goose.acp`

Official: `goose acp --with-builtin …`; session `/mode`
`auto|approve|chat|smart_approve`
([goose-cli-commands](https://github.com/block/goose/blob/main/documentation/docs/guides/goose-cli-commands.md),
[permissions](https://github.com/block/goose/blob/main/documentation/docs/guides/goose-permissions.md)).

Swallowtail: `goose acp` only. Does not pass `--with-builtin`. Never
`allow_always`. Does not select default GooseMode `auto`.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Builtins | `--with-builtin` | not passed | yes | only-if: host already configured extensions | — |
| Mode | `/mode` / configure | not selected | yes | only-if: ACP session config | do not auto-select `auto` |

### `kiro.acp`

Official
([CLI commands](https://kiro.dev/docs/reference/cli-commands/),
updated 2026-08-04):
`--effort low|medium|high|xhigh|max`, `--agent`, `--trust-all-tools`,
`--trust-tools`, `--cloud`, `--no-interactive`.

Swallowtail: `kiro-cli acp`. Does not pass `--cloud`, `--agent`,
`--trust-all-tools`. Field `prompt` on `session/prompt`.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Effort | `--effort` | not passed | yes | only-if: ACP spawn vs `session/set_model` | guide: `session/set_model` unsupported |
| Agent profile | `--agent` | not passed | yes | only-if | — |
| Cloud sessions | `--cloud` | not passed | yes | no | different topology; do not flatten |
| Trust-all | `--trust-all-tools` | withheld | official unused | no | — |

### `deepagents.acp`

Official
([LangChain ACP](https://docs.langchain.com/oss/javascript/deepagents/acp)):
`--name`, `--model`, `--workspace`, `--skills`, `--memory`, `--debug`,
`--log-file`. Swallowtail passes no extra argv and does not wrap `npx`.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Model | `--model` | not passed | yes | no for now: Research 206 empty set — generic access profile cannot prove provider agreement before spawn; CLI silent default on missing value; post-spawn construction; no ACP model confirmation | host keys still required; do not expose unbounded string |
| Skills / memory | `--skills`, `--memory` | not passed | yes | only-if | host paths, not prompt text |
| Workspace | `--workspace` | cwd from working resource | unused flag | no | resource already bound |

### `copilot-cli.acp`

Official
([ACP server](https://docs.github.com/en/copilot/reference/copilot-cli-reference/acp-server)):
`--effort` / `--reasoning-effort` `low|medium|high|xhigh|max`,
`--available-tools`, `--excluded-tools` apply to **every** session at
server start. `--yolo` / `--allow-all` withheld.

Swallowtail: `copilot --acp --stdio` only.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Effort | `--effort` at server start | not passed | yes | withheld on exact `1.0.80` | model-entitled values may fall back to the selected model's default; this route selects no model |
| Tool filter | `--available-tools` / `--excluded-tools` | not passed | yes | only-if: same lifetime | same |
| YOLO | `--yolo` / `--allow-all` | withheld | official unused | no | — |

### `mistral-vibe.headless`

Official
([Mistral agents](https://docs.mistral.ai/vibe/code/cli/agents),
[README](https://github.com/mistralai/mistral-vibe)):
`--agent ask|plan|accept-edits|auto-approve`, `--auto-approve`/`--yolo`,
`--prompt`, `--max-turns`.

Swallowtail: `--agent plan --max-turns 8 --trust`. Does not pass
`--auto-approve`/`--yolo`.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Agent profile | `--agent` set | fixed `plan` | other agents unused | only-if: never default `auto-approve` | programmatic default is auto-approve; Swallowtail refuses that |
| Max turns | `--max-turns` | fixed 8 | consumer override unused | yes | — |

### `qoder.headless`

Swallowtail-recorded official:
`--permission-mode dont_ask`, historical inert `--max-turns 8`,
`--no-session-persistence`. Exact selected CLI headless factory AgentLoop
ceiling is `1000`; argv `8` does not set it. `error_max_turns` mapping is
decoder-only. Does not pass `--yolo`, `bypass_permissions`, `accept_edits`.
Independent official page not re-fetched; extra flags unconfirmed.

### `deepseek-harness.jsonrpc`

One owned JSON-RPC run. No official composer effort/search/permission
flags on this frozen initialize/prompt/shutdown surface. Guide:
reasoning progress only.

### `zcode.app-server`

Host-supplied `plan` or `build`. `yolo` not admitted. Official extra
composer flags not independently confirmed beyond the guide.

### `deepseek-harness.local-server`

Frozen 11-method `/api` allowlist. Catalogue `session.search` is session
search, not web search. No reasoning/permission/attachment fields on the
allowlist.

### `oh-my-pi.rpc`

Swallowtail already: reasoning
`off|minimal|low|medium|high|xhigh|max`, one PNG, typed questions.
Guide: no output-token limit, no search, no permission exchange, no
subagent control. Official extra flags not independently re-fetched.

### `pi.rpc`

Swallowtail: attachments + typed questions. Guide: no reasoning
control. RPC `bash|switch_session|fork|clone|extensions` unselected.

### `qwen.headless`

Official
([headless](https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/),
[settings](https://qwenlm.github.io/qwen-code-docs/en/users/configuration/settings/)):
`--approval-mode plan|default|auto-edit|auto|yolo`, `--yolo`,
`--safe-mode`, `--max-session-turns`, `--max-wall-time`,
`--max-tool-calls`, `model.reasoningEffort` / `/effort`
`low|medium|high|xhigh|max`.

Swallowtail: `--safe-mode --approval-mode default` with optional portable
`HarnessMode::Plan` as `--approval-mode plan` on exact `0.21.15`, `0.22.0`,
and `0.22.1`; fixed wall/tool/turn caps; and exact `0.21.15` reasoning
selection for `qwen3.8-max` and `qwen3.8-max-preview`. Matrix: reasoning Yes,
output limit No, search No.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Effort | `/effort` / `model.reasoningEffort` | exact private stream-JSON control on `0.21.15` | delivered for two exact models | yes | no retroactive claim before `0.21.15` |
| Approval | `--approval-mode` / `--yolo` | optional `HarnessMode::Plan` as `--approval-mode plan` on exact `0.21.15`, `0.22.0`, `0.22.1`; omission keeps `default` | `auto-edit\|auto\|yolo` withheld | no | Plan is provider behavior; yolo stays out |
| Turn/tool caps | official flags | adapter-local caller-decreasing budgets on `0.21.15` | other package points unused | yes | no retroactive claim before `0.21.15` |

### `kimi-code.local-server`

Swallowtail already: reasoning, Manual/Auto/Yolo permission, profile,
disabled tools, archive/restore. No JSON Schema, no search, no
attachments. `max_context_size` is catalogue evidence.

### `opencode.http`

Swallowtail already: catalogue-gated reasoning, JSON Schema, one PNG,
one-shot permission/questions, load/resume/import/delete. Matrix:
output limit No, search No. Official permissions include `websearch`
and `task` (subagents)
([permissions](https://opencode.ai/docs/permissions/)).

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Web search | `websearch` permission key | `external_search` No | yes | only-if: ExternalSearchPolicy + route qualification | search ≠ network policy |
| Subagent task | `task` permission | no public subagent control | yes | only-if | not a swarm |
| Output tokens | not a first-class OpenCode field found | none | none confirmed | no | — |

### `anthropic.messages`

Official Messages
([effort](https://platform.claude.com/docs/en/build-with-claude/effort),
[thinking](https://platform.claude.com/docs/en/build-with-claude/thinking),
[web-search](https://platform.claude.com/docs/en/agents-and-tools/tool-use/web-search-tool)):
`max_tokens`, `output_config.effort`
`low|medium|high|xhigh|max`, `thinking`, newer
`web_search_20260318` (Swallowtail uses `web_search_20250305`).

Swallowtail: `maximum_output_tokens`, one PNG, `web_search_20250305`
with two uses + domain allowlist, exact `claude-opus-4-7`
`output_config.effort` on structured and direct-continuation profiles, and
adapter-local `AnthropicThinkingMode::adaptive()` omitted-display thinking on
the same model. Matrix: `reasoning_selection` Yes for that exact effort row.
Thinking is not a portable capability and adds no `ReasoningSummary` activity.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Effort | `output_config.effort` | exact `low|medium|high|xhigh|max` on `claude-opus-4-7` | no for delivered row | delivered by g04.037 | not UltraCode; not Claude Code `--effort` |
| Thinking | `thinking` | adapter-local adaptive omitted-display on `claude-opus-4-7`; private replay on continuation | no for delivered omitted row | delivered by g04.062 | official: do not pass `adaptive` as effort; summarized display stays out |
| Newer search tool | `web_search_20260318` | pinned `20250305` | version pin | only-if: qualify new type | do not silent-upgrade |

### `kimi-platform.chat`

Official: `reasoning_effort` `low|high|max` (K3 default `max`);
`max_completion_tokens` up to 1048576; K3 1M context is model fact;
K2.6 `thinking: {type: enabled|disabled}`; web search "not recommended
for production" on K3 quickstart
([kimi-k3-quickstart](https://platform.moonshot.ai/docs/guide/kimi-k3-quickstart)).

Swallowtail already: required `ReasoningMode` + `maximum_output_tokens`
(guide: `low|high|max`).

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Disable thinking (K2.6) | `thinking.type=disabled` | K3-oriented required reasoning | yes on other models | only-if: that model is admitted | K3 cannot disable thinking |
| Context 1M | model window | model selection | none as a setter | no | — |

### `deepseek.continuation`

Official
([thinking mode](https://api-docs.deepseek.com/guides/thinking_mode)):
`reasoning_effort` `low|high|max` (default `high`; `medium`/`xhigh`
map to `high`); `thinking: {type: enabled|disabled}`; `max_tokens`.

Swallowtail: required reasoning (guide: fixed `high`) + output bound.
Interactive tools on session only.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Effort ladder | `low\|high\|max` | high only | yes | yes | do not emit unmapped UI labels; official maps medium/xhigh→high |
| Disable thinking | `thinking.type=disabled` | not exposed | yes | yes | — |

### `alibaba.conversations`

Guide: no reasoning override, no output-token override, no tools on the
qualified subset, exact model `qwen3.7-plus-2026-05-26`. Official
OpenAI-compatible Responses fields exist on the wider Model Studio
surface; this pass did not confirm a composer-settable effort field on
the exact `openai-conversations-responses` facade Swallowtail pins.
Do not flatten from other Alibaba APIs.

### `openai.background`

Official Responses: `reasoning.effort`, `max_output_tokens`,
`service_tier`, hosted tools including search
([reasoning guide](https://developers.openai.com/api/docs/guides/reasoning)).

Swallowtail already: full reasoning enum, output bound, JSON Schema,
background + detach/reconcile. Guide: search remains outside profile.
No tools.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Search | Responses hosted search | not on profile | yes | only-if | do not copy Codex `--search` |
| Service tier | `service_tier` | not mapped | yes | only-if | not Codex `/fast` |

### `anthropic.managed-agent`

Official
([agent-setup](https://platform.claude.com/docs/en/managed-agents/agent-setup)):
`model.effort`, `model.speed` (`fast`), `model.inference_geo`, built-in
toolsets, MCP, custom tools, permission policies.

Swallowtail: custom consumer tools 0–8; no independent reasoning/output
controls; built-in tools/MCP/skills/multiagent unsupported.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Effort | `model.effort` | not on run input | yes | only-if: agent-version mutation vs per-run override | operator-owned agent version |
| Fast | `model.speed: "fast"` | not mapped | yes | only-if | see seed |
| Built-in tools / MCP | toolsets | unsupported | yes | only-if | guide exclusion |

### `xai.responses-websocket`

Official Responses
([chat API](https://docs.x.ai/developers/rest-api-reference/inference/chat),
[reasoning](https://docs.x.ai/developers/model-capabilities/text/reasoning)):
`reasoning.effort` / `reasoning_effort`, `max_output_tokens` (default
128000), web search / X search, `store`.

Swallowtail: model + prompt; usage + billed cost. No reasoning, no
output cap, no search. `store=false` on runs.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Effort | `reasoning.effort` | not mapped | yes | yes | do not flatten onto `grok-build.acp` |
| Output cap | `max_output_tokens` | not mapped | yes | yes | — |
| Search | web / X search tools | not mapped | yes | only-if | — |
| Multi-agent model | `grok-4.20-multi-agent` | not admitted | yes | only-if | effort means agent count there |

### `openai.realtime`

Official Realtime: `max_output_tokens` 1–4096 or `inf`;
`reasoning.effort` on reasoning-capable realtime models
([Realtime sessions](https://platform.openai.com/docs/api-reference/realtime-beta-sessions)).

Swallowtail already: `with_maximum_output_tokens` (≤4096). No
reasoning. Rollover disabled.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Reasoning | `reasoning.effort` | not mapped | yes | only-if: admitted realtime model supports it | do not copy Responses effort |

### `gemini.live`

Official
([Live capabilities](https://ai.google.dev/gemini-api/docs/live-api/capabilities),
[Live API](https://ai.google.dev/api/live)):
`thinking_config.thinking_level` `minimal|low|medium|high` (3.1 default
`minimal`) or `thinking_budget`; `contextWindowCompression`.

Swallowtail: media + one planned rollover. No reasoning, no
output-token max.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Thinking | `thinkingLevel` / `thinkingBudget` | not mapped | yes | yes | Gemini CLI rejection does not apply here |
| Context compression | `contextWindowCompression` | not mapped | yes | only-if | see seed |

### `bedrock.runtime`

Official ConverseStream
([API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_ConverseStream.html)):
`inferenceConfig.maxTokens` (Swallowtail has this), `temperature`,
`topP`, `additionalModelRequestFields` (model-specific thinking/effort),
`toolConfig`, `guardrailConfig`, `performanceConfig.latency`,
`serviceTier`.

Guide: no reasoning, tools, or guardrails.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Model-specific thinking | `additionalModelRequestFields` | not mapped | yes | only-if: per-model schema, never a generic "effort" string | generation-controls: exact mapping |
| Tools / guardrails | `toolConfig`, `guardrailConfig` | unsupported | yes | only-if | guide exclusion |
| Latency / service tier | `performanceConfig`, `serviceTier` | not mapped | yes | only-if | not Fast mode |

### `bedrock.catalogue`

`ListFoundationModels` only. No composer controls.

### `ollama.attached`

Official: `think` / `options.num_ctx` / `options.num_predict`;
OpenAI compat `reasoning_effort`.
Swallowtail: `off|low|medium|high` + output bound + schema.
`num_ctx` not mapped. `max` think level exists officially; Swallowtail
set stops at `high`.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Context | `num_ctx` | not mapped | yes | yes | see seed |
| Think `max` | official level | not in set | yes | only-if: model advertises it | exact advertised levels only |

### `llama-cpp.attached`

Official server controls vary by build. Exact attached `b9910` and owned
`b10069` evidence must be checked independently; server-start flags are not
per-request OpenAI-chat fields.
Attached route is OpenAI-chat HTTP against an already-running server.
Swallowtail: output-token bound only.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Per-request reasoning | depends on server build | not mapped | unconfirmed on this HTTP facade | only-if: request field exists on b9910 | do not invent |
| Ctx | `--ctx-size` | server-owned | none on attached | no | boot flag |

### `llama-cpp.owned`

Serving lifecycle only. Exact `b10069` documents `--ctx-size`, `--reasoning`,
`--reasoning-budget`, and `--n-predict` on `llama-server` start. It does not
document the earlier inventory's `--reasoning-effort` spelling.
Inference stays on a separately prepared attached route.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Ctx at serve | `--ctx-size` | not on serving input | yes | only-if: serving profile, not message composer | no inference role |
| Reasoning at serve | `--reasoning`, `--reasoning-budget` | not on serving input | yes | only-if: exact model/template semantics and serving profile | no inference role; not selected |

## Feature-gap inventory

Numbered `(route, official name)` pairs Swallowtail does not expose.
Withheld dangerous flags are included but marked `composer: no`.
Unconfirmed flags are omitted.

1. `claude-code.headless` / `--effort ultracode`
2. `claude-code.headless` / `fastMode` (`--settings`)
3. `claude-code.headless` / Agent teams env + `--teammate-mode`
4. `claude-code.headless` / `--autocompact`
5. `claude-code.headless` / `--max-turns`
6. `claude-code.headless` / `--max-budget-usd`
7. `claude-code.headless` / `--json-schema`
8. `claude-code.headless` / `--advisor`
9. `claude-code.headless` / `--permission-mode` beyond `plan`
10. `claude-code.response-only` / `--effort ultracode`
11. `claude-code.response-only` / `fastMode`
12. `claude-code.response-only` / `--autocompact`
13. `claude-agent.acp` / Agent SDK `effortLevel: "ultracode"`
14. `claude-agent.acp` / Agent teams
15. `codex.exec` / `service_tier` + `features.fast_mode`
16. `codex.exec` / `model_verbosity`
17. `codex.exec` / `features.multi_agent`
18. `codex.app-server` / Fast service tier
19. `codex.app-server` / `model_verbosity`
20. `codex.app-server` / `personality`
21. `codex.app-server` / `plan_mode_reasoning_effort`
22. `codex.app-server` / `features.multi_agent`
23. `cursor-agent.headless` / `--model [fast=…]`
24. `cursor-agent.headless` / `--model [context=…]`
25. `cursor-agent.headless` / `--model [effort=…]`
26. `cursor-agent.headless` / `--mode ask`
27. `cursor-agent.headless` / `--sandbox`
28. `cursor-agent.acp` / Fast / effort / context params
29. `gemini-cli.acp` / `--sandbox`
30. `gemini-cli.acp` / `thinkingConfig`
31. `gemini-cli.headless` / `--sandbox`
32. `gemini-cli.headless` / thinking
33. `grok-build.acp` / ACP `--effort`
34. `grok-build.acp` / `--disable-web-search`
35. `grok-build.acp` / `--no-subagents`
36. `kimi-code.acp` / `--plan`
37. `kimi-code.acp` / thinking effort above `high`
38. `kimi-code.headless` / `--thinking`
39. `kimi-code.headless` / `--plan`
40. `cline.acp` / `--thinking`
41. `cline.acp` / `--plan`
42. `cline.acp` / `--model`
43. `cline.headless` / `--thinking`
44. `cline.headless` / `--plan`
45. `cline.headless` / `--model`
46. `goose.acp` / `--with-builtin`
47. `goose.acp` / Goose mode
48. `kiro.acp` / `--effort`
49. `kiro.acp` / `--agent`
50. `deepagents.acp` / `--model`
51. `deepagents.acp` / `--skills` / `--memory`
52. `copilot-cli.acp` / `--effort` server-start
53. `copilot-cli.acp` / `--available-tools` / `--excluded-tools`
54. `mistral-vibe.headless` / `--agent` beyond `plan`
55. `mistral-vibe.headless` / consumer `--max-turns`
56. `qwen.headless` / `model.reasoningEffort`
57. `qwen.headless` / `--approval-mode` beyond `default` — Plan delivered on exact `0.21.15`, `0.22.0`, `0.22.1`; `auto-edit|auto|yolo` remain withheld
58. `qwen.headless` / consumer turn/tool caps
59. `antigravity.headless` / `--agent`
60. `opencode.http` / `websearch` permission
61. `opencode.http` / `task` subagent permission
62. `anthropic.messages` / `output_config.effort`
63. `anthropic.messages` / `thinking`
64. `kimi-platform.chat` / `thinking.type=disabled` (non-K3)
65. `deepseek.continuation` / `reasoning_effort` beyond fixed `high`
66. `deepseek.continuation` / `thinking.type=disabled`
67. `openai.background` / hosted search
68. `openai.background` / `service_tier`
69. `anthropic.managed-agent` / `model.effort`
70. `anthropic.managed-agent` / `model.speed: "fast"`
71. `anthropic.managed-agent` / built-in toolsets / MCP
72. `xai.responses-websocket` / `reasoning.effort`
73. `xai.responses-websocket` / `max_output_tokens`
74. `xai.responses-websocket` / web/X search
75. `xai.responses-websocket` / multi-agent model
76. `openai.realtime` / `reasoning.effort`
77. `gemini.live` / `thinkingLevel` / `thinkingBudget`
78. `gemini.live` / `contextWindowCompression`
79. `bedrock.runtime` / `additionalModelRequestFields` thinking
80. `bedrock.runtime` / `toolConfig` / `guardrailConfig`
81. `bedrock.runtime` / `performanceConfig` / `serviceTier`
82. `ollama.attached` / `options.num_ctx`
83. `ollama.attached` / think `max`
84. `llama-cpp.owned` / `--ctx-size`
85. `llama-cpp.owned` / `--reasoning` + `--reasoning-budget`

Not counted as gaps: catalogue-only rows; `llama-cpp.attached`
unconfirmed per-request reasoning; Alibaba unconfirmed facade fields;
Command Code / Qoder / Muse / Oh My Pi / Pi / ZCode / DeepSeek harness
flags not independently confirmed; withheld YOLO/auto-approve flags
already named in guides as explicit non-selection (they remain
documented above but are policy withholds, not missing composer
bindings).

**Feature-gap count: 85.**

**Production routes covered: 47.**

## Recommendations, not decisions

- Keep UltraCode, Fast, teams, and context-size as **per-route official
  names**. One composer chip per vendor nickname will violate
  generation-controls.
- If anything is scheduled later, start with flags that are already
  exact on the selected transport: Cursor headless `[fast|context|effort]`,
  Ollama `num_ctx`, Anthropic Messages `output_config.effort`,
  DeepSeek `reasoning_effort`, xAI Responses `reasoning.effort` +
  `max_output_tokens`, Copilot/Cline/Kiro spawn effort, Qwen
  `reasoningEffort` (after reversing the current No).
- Do not promote Claude Code UltraCode onto `anthropic.messages`,
  `anthropic.managed-agent`, or Bedrock.
- Do not enable Agent teams from a composer toggle. It is an
  experimental env that spawns processes.
- Do not add `--swarm`. The official Claude name is Agent teams. xAI
  multi-agent is a different model.
- Do not treat catalogue context-window numbers as a setter.
- Do not surface YOLO / skip-permissions as composer defaults.
- Do not open implementation cards from this doc. Tom's local
  orchestrator schedules any later work.

## Orchestrator Assessment (2026-08-21)

Disposition: promoted into
[`docs/roadmaps/g04/per-route-feature-completion.md`](../roadmaps/g04/per-route-feature-completion.md).
Do not promote the 85 entries as one feature tranche.

The inventory mixes four different shapes: exact route-local controls,
provider topology changes, explicit policy withholds, and surfaces that
still need identity evidence. They do not share one portable composer
contract. UltraCode, Fast mode, Agent teams, and context size stay
provider- and route-specific; none becomes a cross-provider label.

The first eligible exact-transport tranche is now complete through g04.035-039:
Cursor headless model parameters, Ollama `num_ctx`, Anthropic Messages effort,
DeepSeek effort, and xAI Responses reasoning/output bounds. The remaining
inventory stays promoted as selection input, not as one implementation tranche.
Select one route and one coherent control family at a time and recheck current
official evidence before compiling cards.

Experimental process-spawning controls, dangerous permission bypasses,
catalogue observations presented as setters, and unconfirmed fields stay
withheld. The next planning pass must reassess the remaining entries against
current production-matrix and contract truth before compiling g04.040.

## Copilot Evidence Stop And Next Selection (2026-08-22)

Disposition: Copilot CLI ACP session effort was promoted into
[`g04.040`](../roadmaps/g04/040-copilot-cli-acp-session-effort.md), then stopped
after Research 188 and card 110. PR 39 merged the evidence at `da0871d5`.

The current official ACP-server surface names `--effort` and
`--reasoning-effort` with `low`, `medium`, `high`, `xhigh`, and `max`, fixed at
server start and inherited by every session. Swallowtail owns one child for one
bounded prepared Copilot session, so the lifetime is a cleaner next fit than a
global configuration mutation. Exact package `1.0.80`, canonical syntax, the
no-model-route Contract 040 boundary, and every value remain gated by card 110
and Research 188.

No Copilot value is deliver-now on exact `1.0.80`. The package resolves startup
effort against the current model's entitled set and may substitute the model
default; `copilot-cli.acp` selects no model. Reopen only with an exact selected-
model route or an upstream interface that accepts one value without model
entitlement.

Qwen headless reasoning effort is promoted into
[`g04.041`](../roadmaps/g04/041-qwen-headless-reasoning-effort.md). Research 189
and cards 113-115 form one serial evidence-first lane. Exact `0.21.15` process
transport, model qualification, clamp, default, and run/turn/replacement
lifetime proof must precede binding. A global setting, `/effort`, user-config
mutation, or unleased synthetic config root is a stop. Cline thinking remains
a separate later spawn-control family. Parked families remain outside
this selection. Do not bulk-promote the remaining inventory.

## Qwen Delivery And Next Selection (2026-08-22)

Qwen headless reasoning selection is delivered through g04.041 and PR 40 at
`709d197c`. Exact package `0.21.15` admits `low`, `medium`, `high`, `xhigh`, and
`max` only for `qwen3.8-max` and `qwen3.8-max-preview` through the private
`initialize` then `set_effort` control exchange. Earlier `0.21.x` behavior is
not promoted.

Cline thinking controls are the next planning family. g04.042 must recheck the
current official and exact package surfaces for ACP and headless separately,
including syntax, value normalization, model dependence, spawn lifetime, and
absent-path behavior. Kiro remains parked with the other deferred route
surfaces. Do not implement until the evidence gate and cards are compiled.

## Cline Compilation (2026-08-22)

Cline thinking controls are promoted into
[`g04.042`](../roadmaps/g04/042-cline-thinking-controls.md). Research 190 and
cards 116-118 form one serial evidence-first lane. ACP and headless must be
classified independently despite their shared package axis. Binding is
conditional on exact `3.0.55` route/value rows that survive omission/default,
normalization, provider/model, and lifetime checks. No later feature family is
selected by this compilation.

## Cline Evidence Stop And Next Assessment (2026-08-22)

g04.042 stopped after Research 190 and card 116, then merged through PR 41 at
`27b34c7d`. Exact `3.0.55` ACP parses and discards thinking selection. Headless
applies selection only after provider/model resolution, may clamp, substitute,
remove, or budget-map it, and the production route selects no provider or model
and receives no applied-tier acknowledgement. Neither route has a deliver-now
row; cards 117-118 remain blocked.

The next planning checkpoint must reassess the remaining promoted inventory
against current production-route and contract truth, choose one coherent
route-local control family, and compile g04.043. New-route research does not
pre-empt this programme.

## OpenAI Background Search Selection (2026-08-22)

The reassessment selects provider-owned web search on `openai.background` for
[`g04.043`](../roadmaps/g04/043-openai-background-hosted-search.md). Current
official OpenAI surfaces name the non-preview Responses `web_search` tool for
new integrations, show it on `gpt-5.6`, expose a maximum total built-in-tool-
call field, and can include complete source evidence. The production route
already owns the matching model, public API-key billing boundary, background
execution, stream reattachment, retrieval, cancellation, deletion, detachment,
and reconciliation. Contract 041 and the portable external-search policy are
already realized.

Research 191 and cards 119-121 form one serial evidence-first lane. Card 119
must still prove the exact background request, positive bound, source/activity
events, existing-control combinations, retained lifecycle, and facade revision.
No tool, model, route, or capability claim follows from selection. Rich search
configuration and later feature families remain unselected.

## OpenAI Background Search Stop And Reasoning Correction (2026-08-23)

g04.043 stopped after Research 191 and card 119, then merged through PR 42 at
`685dbf1a`. Official docs prove the individual `web_search`, exact-model,
positive-bound, source, event, and background fields but not their complete
composition through this route's stream, reattachment, retrieve, account
policy, activity, and facade truth. Cards 120-121 are blocked. No search
capability shipped.

The same exact GPT-5.6 evidence lists reasoning
`none|low|medium|high|xhigh|max` and omits `minimal`, which the current guide
and validator admit. Named follow-up `g04.043-R1` is promoted into
[`g04.044`](../roadmaps/g04/044-openai-background-reasoning-vocabulary-correction.md)
with cards 122-123. The correction is route-local, uses a new exact opaque
facade point, permits no alias or fallback, and requires a Contract 036
next-minor disposition. No later feature family is selected by this
compilation.

## OpenAI Reasoning Correction Closeout And Next Planning Boundary (2026-08-23)

g04.044 completed through PR 43 at `bdb7ea88`. Exact GPT-5.6
`openai.background` reasoning now admits
`none|low|medium|high|xhigh|max`; unqualified `minimal` fails before effects
under a new opaque facade point. Contract 036 classifies the guarantee shrink
as next-minor release material without selecting a release.

The remaining promoted inventory stays selection input. Compile g04.045 by
rechecking current production-route and contract truth, then select one
coherent route-local control family. No implementation card is ready and no
specific provider/control is preselected by this closeout.

## Claude Code Headless Structured Output Selection (2026-08-23)

The reassessment selects `claude-code.headless` `--json-schema` for
[`g04.045`](../roadmaps/g04/045-claude-code-headless-structured-output.md).
The route already owns a bounded structured-run request, exact model, optional
reasoning, read-only Plan-mode tools, working-resource authority, activity,
usage, cancellation, and cleanup. Contracts 039 and 040 already state the
structured-output and enforcement boundary.

Selection is not qualification. Exact `2.1.238` currentness reused earlier
selected help and does not freeze `--json-schema`. Response-only Research 121
also proves that the flag may use a model-visible schema tool, retry, and exit
zero with `structured_output: null`. Research 192 and card 124 must therefore
settle exact package syntax, dialect/subset, enforcement source, attempt bound,
terminal failure, version, and selected-command composition. Cards 125-126 are
conditional on a non-empty deliver-now set. No earlier version, response-only
route, capability, or matrix claim is promoted by compilation.

Kimi headless thinking and plan were not selected. Current thinking docs refer
to a newer Python CLI line, not Swallowtail's exact TypeScript `0.38.0` axis;
earlier exact evidence rejects prompt plus `--plan`. No later feature family is
selected here.

## Claude Code Headless Structured Output Closeout (2026-08-23)

g04.045 stopped after Research 192 and card 124, then merged through PR 44 at
`8a2640ea`. Exact `2.1.238` local validation admits declared draft-07 and
rejects declared draft-2019-09 and draft-2020-12. Delivery remains withheld:
the full CLI-to-SDK runtime linkage and keyword subset, immutable retry bound,
and valid terminal/lifecycle truth are unqualified. Cards 125-126 are blocked;
no structured-output capability shipped.

The remaining promoted inventory stays selection input. Compile g04.046 by
rechecking current production-route and contract truth, then select one
coherent route-local control family. No implementation card or provider/control
is preselected by this closeout.

## Gemini Live Thinking-Level Selection (2026-08-23)

The reassessment selects caller-selectable thinking levels on `gemini.live`
for [`g04.046`](../roadmaps/g04/046-gemini-live-thinking-levels.md). The exact
production route already fixes `gemini-3.1-flash-live-preview`, hosted
`v1beta` raw WebSocket, project authorization API-key access, manual PCM,
output transcription, and one provider-planned rollover. Its current initial
and resume setup fixtures already send `thinkingLevel=MINIMAL`, but the route
does not expose or claim caller reasoning selection.

Current official exact-model and Live API documentation lists Thinking and
`minimal|low|medium|high` on this model. Research 193 and cards 127-129 form
one serial evidence-first lane. Card 127 must freeze exact facade/model/field
truth, preserve omission bytes, prove rollover/restoration stability, and
select a Contract 029 point before cards 128-129 may bind anything.

`thinkingBudget`, context compression, `includeThoughts`, thought summaries,
another Gemini route/model, consumer login, and live provider work stay out of
scope. No later feature family is selected by this compilation.

## Gemini Live Thinking-Level Closeout (2026-08-23)

g04.046 completed and merged through PR 45 at `04cc22f2`. Exact model
`gemini-3.1-flash-live-preview` now exposes dispatch-qualified
`minimal|low|medium|high` selection on a new opaque facade point. Omission keeps
the existing `MINIMAL` setup without claiming caller selection; one planned
rollover and fresh restoration preserve the selected level. Provider acceptance
and effective reasoning depth remain unclaimed.

This note remains the promoted selection inventory. The remaining rows stay
input to g04.047 compilation; no provider or control is preselected. New route
families and hosted OAuth remain outside this programme.

## Gemini Live Output-Token-Maximum Selection (2026-08-23)

The reassessment selects caller output-token maximum on `gemini.live` for
[`g04.047`](../roadmaps/g04/047-gemini-live-output-token-maximum.md). The exact
production route already fixes `gemini-3.1-flash-live-preview`, hosted
`v1beta` raw WebSocket, project authorization API-key access, manual PCM,
output transcription, caller thinking levels, and one provider-planned
rollover. The shared realtime request already carries an optional positive
maximum, so no new provider-neutral carrier is needed.

Current official references say `BidiGenerateContentSetup` accepts a
`GenerationConfig`, define `GenerationConfig.maxOutputTokens`, and list 65,536
as this exact model's output-token limit. The generation-config reference also
warns that not every parameter is configurable for every model. Research 194
and cards 130-132 therefore form one serial evidence-first lane. Card 130 must
prove the exact Live/model composition, close the numeric domain, preserve
omission bytes, and select a Contract 029 point before cards 131-132 may bind
anything.

Client-side truncation, effective generated-length claims, reasoning changes,
another Gemini route/model, consumer login, live provider work, and sibling-
route changes stay out of scope. No later feature family is selected here.

## Gemini Live Output-Token-Maximum Closeout (2026-08-23)

g04.047 completed and merged through PR 46 at `c2878262`. Exact model
`gemini-3.1-flash-live-preview` now exposes dispatch-qualified positive
`1..=65_536` selection through `generationConfig.maxOutputTokens` on opaque
facade point
`...BidiGenerateContent.thinking-output-max-2026-08-23`. Omission preserves
the existing setup bytes and claims no output-token capability. One planned
rollover and fresh restoration preserve the selected maximum, including when
it composes with any admitted thinking level. Provider acceptance and effective
generated length remain unclaimed.

The remaining promoted rows stay input to g04.048 compilation. Recheck exact
current route, contract, and official-source truth before selecting one
coherent control family; no route or control is preselected.

## Gemini Live Context-Window-Compression Selection (2026-08-23)

The reassessment selects `gemini.live`
`BidiGenerateContentSetup.contextWindowCompression` for
[`g04.048`](../roadmaps/g04/048-gemini-live-context-window-compression.md).
The exact route already fixes model `gemini-3.1-flash-live-preview`, hosted
`v1beta` raw WebSocket, project authorization API-key access, manual PCM,
output transcription, caller thinking/output maximum, and one planned
rollover.

Current official Google material explicitly names context-window compression
on Live and shows `{ "slidingWindow": {} }` with this exact model. It describes
provider defaults for omitted trigger/target values and says compression can
extend sessions, but setup completion returns no applied-configuration fields.
Exact explicit numeric domain and JSON integer representation remain open.

Contract 027 now permits a later evidence-qualified route-local extension
without creating a portable context capability or weakening rollover truth.
Research 195 and card 133 must close exact shapes, wire form, domain, omission,
thinking/output composition, latest-resumable-handle behavior, restoration,
and Contract 029 revision. Cards 134-135 are conditional on a non-empty
deliver-now set.

Default-only sliding window is a candidate, not preapproved. A shared runtime
carrier, generic context knob, live provider proof, long-session guarantee,
retained-history claim, another model/route, and later feature family remain
out of scope.

## Gemini Live Context-Window-Compression Closeout (2026-08-23)

g04.048 completed and merged through PR 47 at `47848056`. Exact model
`gemini-3.1-flash-live-preview` now exposes adapter-local default-only
`contextWindowCompression.slidingWindow = {}` dispatch on opaque facade point
`...BidiGenerateContent.thinking-output-max-context-compression-2026-08-23`.
Omission preserves the prior setup bytes. One planned rollover and fresh
restoration preserve the selected object. Explicit trigger and target token
forms remain withheld, and provider acceptance or effective compression is not
claimed.

The remaining promoted rows stay input to g04.049 compilation. Recheck exact
current route, contract, and official-source truth before selecting one
coherent control family; no route or control is preselected.

## OpenAI Background Service-Tier Selection (2026-08-23)

The reassessment selects `openai.background` Responses `service_tier` for
[`g04.049`](../roadmaps/g04/049-openai-background-service-tier.md). The exact
route already fixes model `gpt-5.6`, public Responses, reasoning, output bounds,
structured output, temporary retention, one stream reattachment, cancellation,
deletion, controlled detachment, and exact-run reconciliation.

Current official OpenAI create and retrieve references place `service_tier` on
the same request and returned Response object. They name project-default
`auto`, standard `default`, Flex, Fast/Priority, and access-controlled
Ultrafast behavior. The returned value may differ from the requested value.
This is a route-local operational selection, not a portable Fast or quality
control.

Research 196 and card 136 must close the complete current request/response
enum, exact `gpt-5.6` applicability, aliases, account/project access gates,
requested-versus-returned truth, omission bytes, reasoning/structured-output
composition, and ordinary/detached/reconciled lifecycle profiles. Cards
137-138 are conditional on a non-empty deliver-now set.

Bedrock Runtime service-performance controls remain promoted but are not this
lane. Its resolved Cargo SDK and qualified public SDK identity currently
disagree; the exact-pin currentness rule requires explicit operator authority
before reopening that family. g04.049 does not hide that correction inside
feature work.

## OpenAI Background Service-Tier Closeout (2026-08-23)

g04.049 completed and merged through PR 48 at `06c00e6c`. Exact model
`gpt-5.6` now exposes adapter-local explicit Responses
`service_tier: "default"` dispatch on ordinary attached runs and one in-process
reattachment under opaque facade point
`openai-responses-background-2026-08-23-service-tier`. Omission preserves prior
create bytes. Active-run detachment and restart reconciliation of a
selected-tier checkpoint fail closed before provider work. Returned tier,
project settings, price, latency, capacity, entitlement, fallback, and provider
acceptance remain unclaimed; `auto`, `flex`, `priority`, `fast`, `ultrafast`,
and `scale` remain withheld.

The remaining promoted rows stay input to g04.050 compilation. Recheck exact
current route, contract, and official-source truth before selecting one coherent
control family. g04.050 is the final roadmap in g04 unless the operator changes
the generation boundary.

## DeepSeek Structured-Run Thinking-Mode Selection (2026-08-23)

The final g04 reassessment selects explicit non-thinking mode on
`deepseek.continuation` one-request structured runs for
[`g04.050`](../roadmaps/g04/050-deepseek-structured-run-thinking-mode.md).
Exact model `deepseek-v4-pro` and the current OpenAI Chat Completions facade
already support enabled `low|high|max` reasoning. Current official DeepSeek
schema and Thinking Mode guidance independently name
`thinking.type=enabled|disabled` and list V4 Pro as supporting both modes.

Research 186 withheld disabled mode because the route has no qualified typed
control and because direct continuation depends on bounded private
`reasoning_content` replay. Research 197 and card 139 must now settle exact
disabled request composition, effort omission, response behavior, cache truth,
plan/evidence representation, and facade revision. Cards 140-141 are
conditional on a non-empty structured-run deliver-now set. Direct continuation
remains enabled-only; no portable `ReasoningMode("none")` or generic thinking
capability is planned.

Ollama attached `think=max` was reassessed but not selected. The current
selected-model catalogue advertises only generic `thinking` capability, not
exact levels, while exact runtime 0.32.15 maps `max` to `high` for
Harmony/GPT-OSS. Swallowtail cannot promise an exact distinct mode from that
evidence. Bedrock exact-pin correction still needs separate operator authority
and does not enter this lane.

g04.050 is the final numbered roadmap in g04. After its evidence, review,
merge, and closeout, reassess and close the generation boundary rather than
compiling g04.051.

## DeepSeek Structured-Run Thinking-Mode Closeout (2026-08-23)

g04.050 completed and merged through PR 49 at `52413da0`. Exact
`deepseek-v4-pro` one-request structured runs now expose adapter-local
`DeepSeekThinkingMode::disabled()`: requests send
`thinking.type=disabled`, omit `reasoning_effort`, and carry no portable
`ReasoningSelection`. Ordinary disabled responses remain available; non-null
private `reasoning_content` fails closed. Enabled `low|high|max` runs and every
direct-continuation path remain enabled-only and unchanged.

This note remains the promoted inventory for active g04 planning. The next pass
must reassess the remaining rows and present one coherent route-local family
for operator selection before compiling cards. No family is preselected and
the generation remains open.

## Post-g04.050 Reassessment (2026-08-23)

Disposition: recommend caller-decreasing turn and tool-call budgets on exact
`qwen.headless` `0.21.15`; await operator selection before compiling another
roadmap.

The current route already dispatches `--max-session-turns 24` and
`--max-tool-calls 16` on structured runs and every turn child. Current official
[headless documentation](https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/)
defines the first as a run turn cap and the second as a cumulative top-level
tool-call budget. The exact
[`v0.21.15` source](https://github.com/QwenLM/qwen-code/tree/v0.21.15)
validates integer turn limits, validates tool-call limits, and aborts before an
over-budget tool executes. Swallowtail's read-only tool allowlist, native
60-second wall bound, mandatory host deadline, exact model route, reasoning
handshake, and private continuation lifecycle already compose with the fixed
values.

If selected, the evidence gate should try to qualify only a typed
caller-decreasing domain: `1..=24` turns and `0..=16` tool calls, initially at
exact package `0.21.15`. Omission must retain the current `24` / `16` argv.
Preparation, immutable plan, request, every first/resumed/replacement child,
reasoning-selected children, terminal classification, and early mismatch
rejection must agree. The gate must settle whether a zero tool budget has
useful route truth and whether the turn counter is exact across this route's
one-prompt child shape before admitting either row.

This is an adapter-local execution-budget family, not a portable output-token,
reasoning, context, permission, or billing control. It does not select approval
modes, expand the tool set, expose subagents, change the 60-second native wall
bound, or qualify earlier/later package points.

Other conspicuous rows are weaker next steps. Anthropic adaptive thinking
requires new thinking-block stream handling and private-block preservation
before it can compose with direct tool continuation. Codex verbosity is
silently ignored by the exact harness when the selected model lacks support,
while the app-server model catalogue does not expose that support bit.
Mistral's caller max-turns control remains eligible but is narrower than the
already-frozen Qwen pair. These observations do not select those families.

## Qwen Turn/Tool Budget Compilation (2026-08-23)

The operator selected the post-g04.050 recommendation. g04.051, Research 198,
and cards 142-144 now form one serial evidence-first lane for exact
`qwen.headless` `0.21.15` caller-decreasing turn/tool budgets.

Card 142 owns zero-tool usefulness, turn-count exactness, child-local lifetime,
terminal classification, and ordinary/reasoning composition. Cards 143-144 are
conditional on a non-empty deliver-now table. Omission remains the existing
`24` turns and `16` tool calls. Native wall time, host deadline, tool set,
approval posture, model route, credentials, and currentness do not change.

No later route-control family is selected. After this lane is reviewed and
merged, reassess the promoted inventory. g04 remains open until explicit
operator direction.

## Post-g04.051 Reassessment (2026-08-23)

Disposition: select caller-decreasing maximum turns on exact
`mistral-vibe.headless` `2.24.2` and compile g04.052.

The current route already dispatches `--max-turns 8`. Current official
[README documentation](https://github.com/mistralai/mistral-vibe/blob/v2.24.2/README.md)
defines the flag as a limit on assistant turns. Exact
[`v2.24.2` source](https://github.com/mistralai/mistral-vibe/tree/v2.24.2)
parses an integer, installs `TurnLimitMiddleware`, checks the step counter
before a turn, and stops with a native turn-limit reason. The current
Swallowtail route already maps the programmatic limit exit separately from
successful `end_turn` and joins the child.

This is enough to compile an evidence-first lane, not to qualify a public
domain. Exact source accepts broader values: zero stops before an assistant
turn, negative values are not rejected at argparse, and omitting the flag is
unbounded. Research 199 and card 145 must settle the turn definition,
off-by-one behavior, useful positive subset, process/stderr/stream/terminal
truth, partial events, cancellation, deadline, and cleanup. The candidate
public subset is positive `1..=8`; caller omission must retain exact
`--max-turns 8` argv.

Cards 146-147 are conditional on a non-empty deliver-now table. The control
stays adapter-local and is not Contract 040 `OutputTokenLimit`. Output
streaming, plan agent, trust, workdir, access, host deadline, cancellation,
failure, cleanup, exact release membership, and qualified-only currentness do
not change. Vibe ACP, TUI, continuation, teleport, auth, model selection,
price/token limits, tools, and later releases remain outside this lane.

No later route-control family is selected. After g04.052 review, merge, and
shared closeout, reassess the remaining promoted inventory. g04 remains open
until explicit operator direction.

## Post-g04.053 Claim Correction (2026-08-24)

Disposition: evidence stop with claim/corpus correction. Research 200 admits
no deliver-now caller-decreasing row. Operator retains historical inert argv
`--max-turns 8`, records factory AgentLoop ceiling `1000`, and narrows
`error_max_turns` to decoder-only. Cards 149-150 stay blocked. No caller
max-turns feature ships. g04 remains open.

Earlier Post-g04.052 reassessment that treated omit as AgentLoop-unbounded and
argv `8` as a required loop bound is superseded by Research 200.

Other remaining controls have weaker immediate fit: Mistral agent profiles
cross write/approval posture; Kiro effort and agent selection need exact
model/profile entitlement evidence; OpenCode search and task permissions widen
authority; Claude headless maximum turns are absent from exact `2.1.238` help;
managed-agent and Bedrock controls cross operator-owned configuration or
model-specific schema boundaries. They remain promoted inventory, not rejected
work.

## Post-g04.053 Reassessment And g04.056 Compilation (2026-08-24)

Disposition: select exact `llama-cpp.owned` `b10069` `--ctx-size` and compile
g04.056 as one serial evidence-first lane.

The route already owns one exact operator-approved executable, GGUF artifact,
offline loopback child, bounded startup observation, health/properties/catalogue
readiness, endpoint authority, stop, and artifact release. Exact tagged
`b10069` documentation exposes `-c, --ctx-size N`, with `0` meaning loaded from
model metadata. An explicit selection therefore fits the serving profile
without becoming a message-composer or attached-inference control.

Selection is not qualification. Research 203 and card 155 must settle parser
breadth, useful positive domain, model-training clamp, host-resource failure,
effective-value observation, immutable start representation, and exact
revision posture. Caller omission keeps the current command with no
`--ctx-size`. Explicit zero is not an omission alias by inference. Cards
156-157 continue only for a non-empty deliver-now set.

Closeout: Research 203 admitted positive `1..=2147483647` as dispatch-only.
Cards 155-157 and PR 55 delivered exact adapter-local `--ctx-size N`, preserved
omission, and withheld accepted, effective, observed, model-fit, and allocation
truth. The family is no longer an open gap; remaining inventory awaits a fresh
reassessment.

This reassessment also corrects the adjacent exact-build note: tagged `b10069`
documents `--reasoning` and `--reasoning-budget`, not
`--reasoning-effort`. That reasoning family remains promoted but unselected;
its model/template semantics are a separate gate. Ollama `think=max` remains
withheld because exact `0.32.15` maps it to `high` for Harmony/GPT-OSS rather
than proving a distinct mode. Claude max turns remains absent from exact
`2.1.241` help. Authority-widening and parked families remain outside this
lane.

## Post-g04.056 Reassessment And g04.057 Compilation (2026-08-24)

Disposition: select `grok-build.acp` reasoning selection and compile g04.057 as
one serial evidence-first lane.

The route fixes `grok-4.5` on deprecated `0.2.114..=0.2.117` and `grok-4.6`
on maintained `1.0.4..=1.0.5`. Existing secret-free handshakes expose
`low|medium|high` for the former and add `xhigh` for the latter. Contract 034
already defines the typed negotiated-session-option boundary, and the
structured-run shape creates one operation-private provider session before its
first prompt. This gives reasoning selection a credible insertion point on both
public operation shapes without model inference.

Selection is not qualification. Research 204 and card 158 must freeze the
exact ACP option snapshot, private id/value mapping, one selection request,
effective confirmation, new-session lifetime, omission, drift, failure after
provider-session allocation, and `UnverifiedNewer` inheritance. Advertisement,
CLI `--effort`, and changelog text alone are insufficient. Attachment recovery,
load, and resume do not inherit mutation authority.

Cards 159-160 continue only for a non-empty exact version/model/value table.
The candidate values are `low|medium|high` for `grok-4.5` and those plus
`xhigh` for `grok-4.6`; none are prequalified. `off`, `minimal`, `max`, aliases,
clamping, default substitution, headless argv, web search, subagents,
permissions, and hosted xAI routes stay outside the lane.

Adjacent llama.cpp reasoning remains weaker: exact `b10069` exposes server
flags, but the qualified ChatML owned-serving profile does not prove a useful
model/template semantic effect. Other authority-widening controls remain
promoted inventory, not selected work. g04 stays open at operator direction.

## Post-g04.057 Evidence Stop (2026-08-24)

Disposition: Research 204 and card 158 stop `grok-build.acp` reasoning
selection. Cards 159-160 remain blocked.

Exact 1.0.4 and 1.0.5 binaries expose
`_meta["x.ai/sessionConfig"].options`, but preserved no-prompt handshakes
discarded the `session/new` body. Frozen evidence does not establish effort
membership or selected-value confirmation. Exact 1.0.5 accepts a fail-open
`_meta.reasoningEffort` hint; 1.0.4 does not apply it on `session/new`.
Contract 040 could admit a request-field mapping only after exact confirmation.

No route claim, driver, matrix, public API, or production code changes. The
false changelog lead is removed above. Reassess the remaining per-route feature
inventory before selecting the next lane. g04 stays open at operator direction.

## Post-g04.057 Reassessment And g04.058 Compilation (2026-08-24)

Disposition: select `antigravity.headless` agent-profile selection and compile
g04.058 as one serial evidence-first lane.

Exact qualified CLI `1.1.17` help exposes `--agent` plus `agent` and `agents`
listing commands. Current official headless documentation names `agy agents`,
describes `--agent` as a run selection, and states that stream-JSON `init.agent`
appears when explicitly selected. The production route already validates exact
model and `permission_mode=request-review` in that same init envelope while
holding resource access, isolation, effort, schema, deadline, conversation,
and child lifecycle in one immutable prepared shape.

Selection is not qualification. Research 205 and card 161 must freeze the
complete bounded listing/id shape, custom/account visibility, invalid or stale
selection behavior, silent fallback, exact init confirmation, version floor,
and composition with structured runs and exact-id continuation. Agent display
labels, profile bodies, prompts, tools, settings files, and account identity do
not become public data or values. Cards 162-163 continue only for a non-empty
exact deliver-now set.

Profile selection does not grant resource, tool, permission, subagent,
isolation, or session authority. Omission keeps current argv. Continuation must
reassert and confirm one immutable profile on every admitted child rather than
infer inheritance from the conversation id.

Adjacent Claude headless autocompact and maximum-turn controls are absent from
exact `2.1.241` help. Codex verbosity lacks model support evidence. llama.cpp
reasoning remains model/template-semantic. OpenCode permission controls widen
authority. Those families remain promoted but unselected. g04 stays open at
operator direction.

## Post-g04.058 Evidence Stop (2026-08-24)

Disposition: stop. Research 205 is an empty deliver-now set. Cards 162-163
stay blocked. No `--agent` binding ships.

Exact help and official docs still advertise `--agent`, `agy agents`, and
selected `init.agent`. Authorized stop basis: host-local unstable listing,
missing selected `init.agent` fixture, unproved fail-closed invalid `--agent`
semantics on exact qualified `1.1.9..=1.1.17`, and official custom-agent
tool/instruction authority risk. Official docs promise no silent fallback for
unknown `--model` only, not `--agent`. Production argv remains unchanged.

Two unauthorized live `--print` probes returned JSON `status: SUCCESS` while
host PATH drifted `1.1.9` → `1.1.19`. Those are authority-boundary /
`UnverifiedNewer` incidents only and are not projected onto the qualified
range.

Reassess the remaining per-route inventory before the next route-local lane.
g04 stays open at operator direction.

## Post-g04.058 Reassessment And g04.059 Compilation (2026-08-24)

Disposition: select `deepagents.acp` model selection and compile g04.059 as
one serial evidence-first lane.

The production route owns one exact `deepagents-acp@0.1.25` ACP child per
session and currently passes no extra argv. Current official LangChain docs
advertise `--model` and `provider:model` selection. That is a credible
server-start control because one child owns the entire prepared session.

It is not exact-version proof. Research 206 and card 164 must freeze the
published `0.1.25` CLI parser, provider integrations, omission default,
aliases, invalid values, missing/wrong provider key, fallback, any ACP model
confirmation, and fresh-restoration truth. Current documentation and exact
package source must remain separate evidence classes.

The selected provider must agree with explicit prepared host-owned access
evidence before spawn. Swallowtail does not inspect, inject, lease, or persist
key bytes. Missing auth or provider rejection cannot trigger an ambient model
or provider. If exact evidence cannot support that agreement and a bounded
no-fallback claim, Research 206 must promote an empty set.

Skills/memory stay out because they bind host paths and agent context. Gemini
CLI sandbox remains a containment-proof family with weak live testability.
OpenCode permission controls widen authority. Claude headless controls remain
absent from exact help; Codex verbosity lacks model-support evidence;
llama.cpp reasoning remains model/template semantic. g04 stays open at
operator direction.

## Post-g04.059 Evidence Stop (2026-08-24)

Disposition: stop. Research 206 is an empty deliver-now set. Cards 165-166
are blocked. Exact `deepagents-acp@0.1.25` advertises `--model`, but the
generic host-owned access profile cannot prove provider agreement before
spawn, the CLI silently retains its default when `--model` lacks a usable
value, model construction runs after spawn at `session/new`, and
initialize/`session/new` expose no model confirmation field. Production
continues without `--model`. Reassess the remaining per-route inventory
before the next route-local lane. g04 stays open at operator direction.

## Post-g04.059 Reassessment And g04.060 Compilation (2026-08-25)

Disposition: select catalogue-declared extended reasoning levels on
`kimi-code.acp` and compile g04.060 as one serial evidence-first lane.

The route already negotiates reasoning from the new session's exact
`thinking` select option, sends one `session/set_config_option`, and requires
the returned snapshot to confirm the effective value before readiness.
Swallowtail accepts `off|on|low|medium|high` but rejects any advertised row
above `high` as malformed.

Exact Kimi Code `0.38.0` source projects the current model's
`support_efforts` into that ACP option. Its exact source tests include
`xhigh` and `max`. The selected control therefore has a model-specific wire
advertisement and confirmation path already inside the qualified transport; it
does not require model-name inference, a new route, raw configuration, or
permission expansion.

Research 207 identified exact `0.29.0` as the first declared-effort milestone;
`0.28.1` remains the boolean boundary; ACP adapter blobs are byte-identical
through `0.38.0`, so no later split was required. g04.060 delivered
snapshot-advertised `xhigh` and `max` with effective confirmation. Arbitrary
values, aliases, clamping, load/resume mutation, headless promotion, and
`UnverifiedNewer` inheritance stay out.

Codex plan-mode effort was not selected: the current app-server path already
sends the chosen effort as both turn effort and Plan collaboration settings,
so the inventory does not yet establish a separate missing control. Codex
verbosity still lacks exact model-support evidence. Ollama `think=max`
remains withheld because exact `0.32.15` maps it to `high` for the relevant
family. Permission, multi-agent, skills, memory, and sandbox families remain
authority or containment gates. g04 stays open at operator direction.

## Post-g04.060 Reassessment And g04.061 Compilation (2026-08-25)

Disposition: select negotiated plan mode on `kimi-code.acp` and compile
g04.061 as one serial evidence-first lane.

Exact official Kimi Code `0.38.0` source already projects a `mode` select option
with `default|plan|auto|yolo`. Its exact dispatcher maps `plan` to
`setPlanMode(true)` plus manual permission, then rebuilds the config-option
snapshot. The production route already owns the bounded session-open snapshot,
one-option selection exchange, effective response confirmation, typed
`HarnessMode::Plan`, and immutable plan constraint through its existing
reasoning path and Contracts 012/034.

Selection is not qualification. Research 208 and card 170 must identify the
first exact qualified version with the full option, request, SDK application,
permission, response/update, and cleanup path. They must also prove composition
with every admitted reasoning value and settle load/resume/import/recovery
without mutation. Cards 171-172 continue only for a non-empty exact set.

Only portable `Plan` is a candidate. Provider `default`, `auto`, and `yolo`
may coexist as private snapshot rows but do not become public selections.
Plan mode does not imply process/filesystem containment; `AmbientHost` remains
the independent route truth. Permission widening, generic config, headless
`--plan`, Python `kimi-cli`, and sibling Kimi routes remain outside the lane.

Other remaining candidates are weaker. llama.cpp owned reasoning lacks exact
model/template semantics; Codex verbosity lacks selected-model support proof;
Kiro profiles, OpenCode permissions, sandbox, skills/memory, and multi-agent
families cross authority or containment boundaries. Ollama `think=max` still
maps to `high` for the relevant exact family. g04 stays open at operator
direction.

## Post-g04.061 Delivery (2026-08-25)

Disposition: delivered. Research 208 admitted exact `0.28.1` plus
`0.29.0..=0.38.0` `HarnessMode::Plan` with snapshot membership, one
`session/set_config_option` `{configId: mode, value: plan}`, and response
`currentValue=plan`. No behavior-revision split. Provider `auto|yolo` stay
private. Isolation remains `AmbientHost`. Headless `--plan` stays out.

## Post-g04.061 Reassessment And g04.062 Compilation (2026-08-25)

Disposition: select adaptive thinking on `anthropic.messages` and compile
g04.062 as one serial evidence-first lane.

Contracts 030 and 044 already authorize bounded provider-private continuation
while prohibiting hidden-reasoning disclosure. The current route does not meet
that boundary for adaptive thinking: its SSE grammar rejects thinking blocks
and its private session history reconstructs only tool use. Current official
Anthropic documentation requires the complete signed thinking sequence to be
returned unmodified with tool results.

## Post-g04.062 Delivery (2026-08-25)

Disposition: delivered and merged through PR 61 at `4ef5c5e9`. Research 209
admitted exact `claude-opus-4-7` adapter-local
`AnthropicThinkingMode::adaptive()` with
explicit omitted display on structured attempts and direct continuation.
Cards 174-175 bind request encoding, omitted SSE grammar, bounded zeroizing
private replay before `tool_use`, effort composition, and fail-closed
`thinking_delta`. No `ReasoningSummary` activity. Summarized display, manual
budgets, and other models stay out. g04 stays open at operator direction.

## Post-g04.062 Reassessment And g04.063 Compilation (2026-08-25)

Disposition: select process-local reasoning effort on `kimi-code.headless` and
compile g04.063 as one serial evidence-first lane.

The exact 0.38.0 configuration documentation exposes `[thinking].effort`,
per-model `support_efforts` and `default_effort`, and a temporary
`KIMI_MODEL_*` environment surface. The exact selected headless renderer,
options, and prompt-run source remains byte-identical to 0.37.2 through 0.38.0.
The existing route already selects one exact model and owns one prompt child,
so a typed process-local binding is a credible route-local candidate.

The evidence warning is decisive: unsupported configured effort may fall back
to the model default. Research 210 and card 176 must therefore freeze the
exact key, parser, precedence, executable range, selected model/provider,
supported/default values, invalid and fallback behavior, and stream-json
thinking disclosure. No value is prequalified. Cards 177-178 continue only
for exact rows that cannot be silently substituted, clamped, shadowed, or
defaulted.

No ACP or local-server promotion, Python `kimi-cli`, raw config/environment
surface, config mutation, synthetic config root, permission/plan control,
thought disclosure, currentness movement, generation rollover, or g04 closure
is authorized.

## Post-g04.063 Evidence Stop (2026-08-25)

Disposition: evidence stop after card 176. Research 210 empty deliver-now set.
Headless qualified ceiling retracted to `0.37.2` because naked `0.38.0`
default dispatch uses agent-core-v2 `runV2Print`. Cards 177-178 stay blocked.
Qualify Kimi headless v2 stream-json at `0.38.0` or document incompatible stop.
g04 remains open at operator direction.

## Post-g04.064 Kimi Headless V2 Qualification (2026-08-25)

Disposition: complete through cards 179-180. Research 211 admits adapter-private
`kimi.headless.stream-json.v2` at exact `0.38.0`. v1 ceiling `0.37.2` stands.
Synthetic `0.38.1` remains permitted `UnverifiedNewer` on v2. PR pending
orchestrator review. g04 remains open at operator direction.

## Post-g04.063 Merge And g04.064 Compilation (2026-08-25)

PR 62 fast-forwarded the evidence stop to `main` at `5f37ff6b` after all
required checks passed. g04 stays open.

Disposition: compile exact Kimi Code headless `0.38.0` default
agent-core-v2 stream-json qualification as one Contract 029 family lane.
Card 179 freezes actual v2 source, renderer, JSONL, stderr, terminal, retry,
tool-activity, cancellation, retained-state, and decoder truth. Card 180 is
conditional on Research 211 admitting an adapter-private revision. The worker
must not force legacy v1, use a live account, or turn a public lifecycle change
into a private compatibility claim. An incompatible stop is acceptable.

## Post-g04.064 Reassessment And g04.065 Compilation (2026-08-25)

Disposition: keep g04.063 cards 177-178 blocked and select Claude Code
headless Ultracode as the next serial evidence-first lane.

Kimi v2 qualification fixes the `0.38.0` route identity, parser, and lifecycle
claim. It does not add a headless effective-value confirmation exchange, a
session-open model-effort snapshot, authority over ambient
`thinking.enabled=false`, or an adapter-owned fail-closed binding for
`KIMI_MODEL_THINKING_EFFORT`. Research 210's empty deliver-now set stands.

The next strongest direct-transport lead is Claude Code's documented
`--effort ultracode` on `claude-code.headless`. The route already owns one
selected-model child and exact ordinary effort dispatch, but Ultracode is a
product setting: it sends `xhigh` and enables dynamic workflow orchestration.
It is not a seventh portable effort. The exact qualified `2.1.241` help corpus
also advertises only `low|medium|high|xhigh|max`, so documentation alone is
not enough.

Research 212 and card 181 froze the exact parser, first version, model,
entitlement, settings, alias/fallback, tool, process, persistence, output, and
lifecycle truth without a provider prompt. Research 212 admits no deliver-now
row because exact help omits `ultracode`, workflow topology is unbounded under
the selected Plan-mode command, and model or entitlement truth requires account
work. Cards 182-183 remain blocked.

Fast mode, Agent teams, response-only/ACP promotion, raw settings, permission
widening, and live account work stay outside this lane. g04 remains open at
operator direction.

## Post-g04.065 Evidence Stop And g04.066 Compilation (2026-08-25)

Disposition: select Codex Exec `model_verbosity` and compile g04.066 as one
serial evidence-first lane.

Research 212 admits no Claude Code Ultracode row. Hidden parser acceptance does
not settle selected-model entitlement or contain dynamic workflow topology, so
cards 182-183 remain blocked.

The remaining inventory was re-ranked by exact transport, selected-model
binding, lifecycle containment, and fail-closed authority. Claude headless
Fast and spend controls remain account or billing dependent; Agent teams alter
process topology; autocompact and maximum turns are absent from exact `2.1.241`
help; Codex Fast, multi-agent, and personality add separate service/topology or
product-policy concerns.

Codex Exec verbosity is the strongest next lead. The maintained route selects
one exact model, owns one ephemeral child, suppresses user config and rules,
and already passes typed config overrides. Current official schema exposes
`model_verbosity` as `low|medium|high`, while official model metadata carries
separate `support_verbosity` and `default_verbosity` fields. That split permits
an evidence gate that rejects unsupported model/provider rows before spawn
without inferring from a model-name prefix.

Research 213 and card 184 froze exact `rust-v0.149.1` source, config
parser/precedence, Responses-provider request mapping, release-tag model rows,
defaults, silent-ignore/fallback behavior, omission, and claim strength. Cards
185-186 bound exact `0.147.0`, `0.148.0`, `0.149.0`, and `0.149.1` openai
Responses exec rows for seven frozen slugs through adapter-local
`CodexModelVerbosity`. No generic settings map, app-server promotion, provider
prompt, currentness movement, generation rollover, or g04 closure is
authorized. Live-catalog acceptance remains withheld.

## Post-g04.066 Reassessment And g04.067 Compilation (2026-08-26)

Disposition: select OpenCode HTTP `websearch` and compile g04.067 as one serial
evidence-first lane.

The remaining inventory was re-ranked after Codex Exec verbosity landed.
Codex app-server personality is product-policy/prompt surface; service-tier
Fast is account/billing surface; multi-agent is topology; app-server v2 at the
exact qualified ceiling does not expose a first-class thread/turn verbosity
control. Other model/effort candidates still lack selected-value or effective
confirmation and remain weaker than a route-local tool already present in an
exact qualified transport.

OpenCode `v1.18.20` registers a native `websearch` tool, asks a dedicated
`websearch` permission against the query, and accepts ordered permission rules
on session creation. Swallowtail already owns the exact session-create body,
deny-first rules, one-shot permission callbacks, structured and interactive
prompt paths, and shared external-search/network policy vocabulary.

That is a credible evidence path, not a delivery decision. Exact source also
makes tool availability depend on provider/backend and environment facts, and
the hosted search call may use OpenCode, Exa, or Parallel arrangements. Card
187 and Research 214 must prove an exact host-bindable availability,
permission, policy, and profile row without live provider work or ambient
configuration inference. Otherwise the milestone stops honestly with an empty
set and cards 188-189 remain blocked.

Web fetch, generic tool/permission selection, task subagents, attached-server
configuration, provider/model expansion, currentness movement, generation
rollover, and g04 closure stay outside this lane.

## Post-g04.067 Evidence Stop (2026-08-26)

Disposition: stop OpenCode HTTP `websearch` after card 187. Research 214 is an
empty deliver-now set. Cards 188-189 remain blocked.

Exact `v1.18.20` source registers native `websearch` and a dedicated last-match
permission. Tool visibility still depends on provider `opencode`/`opencode-go`
or attached-server Exa/Parallel flags. Execute always POSTs to Exa or Parallel
MCP, and unflagged backend choice is a checksum of the session id assigned at
create. Existing prepared evidence cannot bind those facts without ambient
config, credential inspection, or live search. Permission syntax and the
shared HostApproved+Enabled pair are not enough.

The current deny-first session JSON, Disabled search claims, guide, and
feature-matrix `search No` row stay unchanged. Do not compile the next
per-route family from this note. Keep g04 open at operator direction.
Contract 029 currentness remains standing.

## Post-g04.067 Reassessment And g04.071 Compilation (2026-08-26)

Disposition: select Copilot CLI ACP built-in tool availability and compile
g04.071 as one serial evidence-first lane, narrowed to a closed
`--available-tools` allowlist.

The remaining inventory was re-ranked after the OpenCode search stop. Codex
personality remains product-policy/prompt surface; Fast remains account/billing
surface; multi-agent remains topology. Remaining model and effort leads still
lack selected-value or effective confirmation. Copilot's server-start tool
filter instead aligns with an exact process lifetime already owned by the
route.

Current official GitHub documentation names `--available-tools` and
`--excluded-tools`, applies them at ACP server start, and says the allowlist
takes precedence. Swallowtail exact `1.0.80` evidence already records both
flags as intentionally unmapped, owns one child and one session, validates
immutable preparation, observes and cancels permission requests, and repeats
the same plan on fresh context-losing replacement.

That is a credible evidence path, not a delivery decision. Card 195 and
Research 218 must freeze the exact `1.0.80` parser, delimiters, built-in tool
identifiers, registry assembly, unknown-name handling, filtering point,
permission behavior, and ambient extension/MCP/model/account effects. Only a
useful closed subset independent of those ambient facts can proceed to cards
196-197. Raw tool strings, `--excluded-tools`, consumer tools, MCP, extensions,
permission bypass, and shared tool vocabulary remain out.

The filter is provider-native behavior, not containment. It grants no
permission and proves no filesystem, process, network, sandbox, read-only, or
other isolation boundary. If exact evidence cannot sustain that separation or
a stable useful set, Research 218 must promote an empty deliver-now set and the
lane stops after card 195. Keep g04 open. Contract 029 currentness remains
standing.

## Post-g04.071 Evidence Stop (2026-08-26)

Disposition: stop Copilot CLI ACP built-in tool allowlist after card 195.
Research 218 is an empty deliver-now set. Cards 196-197 remain blocked.

Exact `1.0.80` parses `--available-tools` through commander `[tools...]` plus
`T5`/`xW` and stores the list on ACP `session/new`. Bare or empty input
collapses to omitted. Unknown names warn rather than fail spawn. Documented
identifiers are not a closed JS table; bare names match any registry source;
ACP still loads host MCP and `github-mcp-server` when the client sends
`mcpServers: []`; available/excluded precedence is unfrozen.

Current `copilot --acp --stdio` argv, unmapped fixtures, observe-and-stop
permission, guide, and `AmbientHost` stay unchanged. Do not compile the next
per-route family from this note. Keep g04 open at operator direction.
Contract 029 currentness remains standing.

## Post-g04.071 Reassessment And g04.072 Compilation (2026-08-26)

Disposition: select Grok Build ACP launch-time subagent suppression and compile
g04.072 as one serial evidence-first lane for `--no-subagents`.

The remaining inventory was re-ranked after three tool-availability leads
stopped on ambient registry or backend truth. Claude Code headless
`--max-turns` was rejected before compilation because exact qualified
`2.1.241` help is unchanged from `2.1.238` and does not expose the current-docs
flag. Grok Build exact installed `1.0.5` instead accepts `--no-subagents` as a
global option before `agent stdio`, on a route that already owns the child
process, immutable launch evidence, both ACP operation shapes, replacement,
cancellation, and joined cleanup.

That is a credible exact-package lead, not an effectiveness claim. Card 198 and
Research 219 must trace `1.0.4..=1.0.5` parser state through configuration,
agent construction, every subagent registry/spawn path, new and later ACP
sessions, operation-private runs, attachment recovery, and replacement.
Current public source may corroborate exact artifacts but cannot substitute for
them. Parser acceptance, help, and binary strings alone are insufficient.

Only a disabled-only adapter-local profile may proceed to cards 199-200. There
is no explicit enabled value, raw flag, generic topology map, agent-definition
surface, or portable child-control capability. Omission retains exact current
argv. Effective `--no-subagents` would be one provider-native topology
restriction; it would not add child observation or direct control, grant
permission, remove ordinary process tools, or prove sandbox, filesystem,
network, read-only, or OS descendant-process containment.

If exact evidence cannot prove complete immutable suppression without a
provider prompt, account inspection, tool/subagent execution, or paid work,
Research 219 must promote an empty deliver-now set and the lane stops after
card 198. Keep g04 open. Contract 029 currentness remains standing.

## Post-g04.072 Evidence Stop (2026-08-26)

Disposition: stop after card 198. Research 219 admits no deliver-now row.

Exact `@xai-official/grok@1.0.4`/`1.0.5` darwin-arm64 binaries parse root
`--no-subagents` before `agent stdio` and reject the flag on the ACP
subcommand. Repeats fail at parse. `--subagents` is not a clap option.
Unauthenticated initialize with and without the flag is structurally identical
after stripping `agentInstanceId`; `subagent_stop` and `deep-research` remain
advertised. Exact spawn-path application and env/config/`--agents` override
behavior are unfrozen. Later public `resolve(--subagents)` does not match these
packages.

Current `grok --no-auto-update agent stdio` argv, empty `SessionOptions`,
observe-and-stop permission, and `AmbientHost` stay unchanged. Do not compile
the next per-route family from this note. Keep g04 open at operator direction.
Contract 029 currentness remains standing.

## Post-g04.072 Reassessment And g04.073 Compilation (2026-08-26)

Disposition: select fixed-argument Plan mode on `cline.headless` and compile
g04.073 as one serial evidence-first lane.

The remaining inventory was re-ranked after three ambient-registry or hidden-
effect lanes stopped. Kimi headless Plan was rejected before compilation
because exact prompt-mode evidence already says `--plan` cannot compose with
the selected command. Cline ACP Plan was also rejected: exact `3.0.55` returns
into ACP without carrying the parsed root mode.

Cline headless is different. Exact qualified source parses explicit `--plan`
before persisted global settings, then carries the resolved mode into the
one-run config, system prompt, mode-tagged input, tool preset, and a command
guard that runs before approval. The production route already owns the exact
JSON child, explicit `--auto-approve false`, read-only working resource, host
deadline, event stream, cancellation, and joined cleanup. Contract 034 permits
portable `HarnessMode::Plan` through a fixed process argument when behavior is
equivalent.

That is a credible exact-package lead, not a delivery decision. Card 201 and
Research 220 must freeze parser conflicts and placement, ambient precedence,
every tool and write-capable seam, Plan-to-Act behavior, output confirmation,
retained state, and full one-run lifecycle. The JSON wire does not report mode
without unselected verbosity, so source-level application must remain distinct
from an effective-value observation.

Only portable `Plan` is a candidate. `act|yolo|zen`, raw flags, runtime mode
switching, ACP, model/thinking work, and permission widening stay out. Plan is
provider behavior, not containment: configuration remains `Ambient`, isolation
remains `AmbientHost`, and command blocking cannot establish complete
filesystem, network, shell, process, sandbox, or descendant containment. If
exact behavior is not equivalent or can widen to Act inside the selected run,
Research 220 must promote an empty deliver-now set and the lane stops after
card 201. Keep g04 open. Contract 029 currentness remains standing.

## Post-g04.073 Delivery (2026-08-26)

Disposition: delivered. Research 220 admitted exact `cline.headless` `3.0.55`
portable `HarnessMode::Plan` as canonical `--plan` before `-c <cwd> <prompt>`.
Omission keeps `--json --auto-approve false -c <cwd> <prompt>`. No behavior
revision. Observation withheld. `act|yolo|zen`, ACP `--plan`, and Plan-to-Act
stay out. Plan is provider behavior, not containment. Isolation remains
`AmbientHost`. Do not compile the next per-route family from this note. Keep
g04 open at operator direction. Contract 029 currentness remains standing.

## Post-g04.073 Reassessment And g04.074 Compilation (2026-08-26)

Disposition: select exact Cline headless model routing and compile g04.074 as
one serial evidence-first lane.

The original 85-item inventory is a dated research baseline, not a live queue:
it still names controls delivered by g04.035 onward. The programme progress and
route guides now own current disposition. After reconciling those surfaces,
`cline.headless --model` is the strongest remaining dependency-shaped lead.
The route already owns one exact JSON child and optional Plan, while g04.042
names absent provider/model selection as the blocker behind headless thinking.

Exact `3.0.55` source gives a real but conditional seam. Explicit `args.model`
wins over persisted and catalogue/default model state. Provider identity,
however, resolves independently from explicit argv, ambient last-used settings,
or `cline`; model membership may come from mutable provider resolution; and the
CLI attempts to persist the resolved provider/model before the run.

Card 204 and Research 221 must therefore settle closed model membership,
configured-instance and access-audience agreement, provider fixing, invalid and
unknown fallback, settings mutation, application, output observation, omission,
and optional Plan composition. An adapter-fixed provider argument is eligible
only when exact evidence derives it from current route facts. Caller provider
selection, API keys, open model strings, live catalogue authority, and new
configuration authority stay out.

If provider/model agreement remains ambient or post-spawn, membership is open,
unknown models silently fall back, or explicit selection necessarily mutates
ambient settings, Research 221 must promote an empty deliver-now set and the
lane stops after card 204. Thinking remains outside this lane; a delivered
model route would only permit a later reassessment of g04.042. Keep g04 open.
Contract 029 currentness remains standing.

## Post-g04.074 Evidence Stop (2026-08-26)

Disposition: stopped after evidence. Research 221 admits no deliver-now
`cline.headless` provider/model row on exact `3.0.55`. Cards 205-206 are
blocked. `cline.headless` / `--model` is now recorded as evidence-closed at
this package point, not merely unassessed.

Three independent gates fire. Provider identity stays ambient: without `-P` it
resolves to `lastUsedProvider` from durable settings, else the literal `cline`,
and no current route or access fact derives a provider — audience
`cline.local-account` names the shared provider-settings store, not a provider
choice. Model membership stays open: explicit `-m` bypasses persisted state,
the resolved catalogue, and the hardcoded fallback with no validation, no
provider/model agreement check, and no invalid-model exit path, and the routing
layer explicitly accommodates unlisted ids, so rejection is post-spawn at best.
Explicit selection mutates durable ambient configuration: `saveProviderSettings`
writes the resolved provider and model into `~/.cline/settings/providers.json`
and moves `lastUsedProvider` before the run, into a file the CLI, VS Code
extension, and hub share, with no flag to disable or scope the write.

Observation does not rescue a marginal case. `run_start` stays behind an
unselected `--verbose`, and `run_result.model` echoes the requested config
rather than a provider-confirmed applied model.

Contract 020 forbids turning any bundled, live, or account-scoped catalogue
into a preflight allowlist. Contract 033 grants no configuration mutation
authority and prohibits the synthesized configuration root that would be the
only containment. Reopening needs a later package point that fixes provider
identity from route facts, closes membership before provider effects, and
either omits or scopes the settings write — or separately authorized
Swallowtail configuration authority. A re-read of `3.0.55` will not change it.

The g04.042 thinking dependency is unchanged: cards 117-118 stay blocked for
the same absent provider/model selection. Keep g04 open. Do not compile the
next per-route family from this note. Contract 029 currentness remains
standing.

## Post-g04.074 Reassessment And g04.075 Compilation (2026-08-26)

Disposition: select fixed-argument Plan on `qwen.headless` and compile g04.075
as one serial evidence-first lane.

The remaining inventory was re-ranked after Cline model routing stopped on
ambient provider identity, open membership, and unavoidable shared-settings
mutation. Another Cline thinking pass cannot remove those gates. Account and
billing controls, ambient model/tool registries, product personality, writable
profiles, and process-topology features remain weaker next leads.

Qwen headless has a closed fixed-argument seam. Exact maintained evidence for
`0.21.15`, `0.22.0`, and `0.22.1` freezes a shared approval-mode parser with
`plan|default|auto-edit|auto|yolo`. Swallowtail already owns every run or turn
child and emits explicit `--safe-mode --approval-mode default`, a read-tool
allowlist, a write/process/tool denylist, one read-only working resource,
bounded turns/tools/time, exact model binding, optional exact reasoning, and
joined cleanup.

That is a credible portable Plan candidate, not a delivery decision. Card 207
and Research 222 must prove exact Plan semantics and precedence, safe-mode and
tool-filter composition, output truth, ambient-state behavior, and immutable
reapplication on structured runs, reasoning-control children, later turns,
resume, and fresh replacement. Parser acceptance, argv dispatch, prompt text,
or tool absence alone cannot establish applied or effective Plan behavior.

Only portable `HarnessMode::Plan` is eligible. Omission retains exact
`--approval-mode default`. `auto-edit|auto|yolo`, raw provider modes, writable
authority, generic tool policy, provider sandbox claims, sibling routes,
currentness, release, generation rollover, and g04 closure remain out. If no
exact point proves the full behavior without provider work, Research 222 must
promote an empty set and the lane stops after card 207.

## Post-g04.075 Closeout (2026-08-26)

Disposition: `qwen.headless` Plan is delivered on exact `0.21.15`, `0.22.0`,
and `0.22.1`. Reassess the remaining inventory before compiling another
serial family.

Research 222 proved `--approval-mode plan` is a complete fixed-argument Plan
posture: CLI wins settings and safe-mode defaults, `isPlanModeBlocked`
enforces it, text-input children omit `exit_plan_mode`, and stream-json
children cannot complete plan-exit without a host operation this route does
not send. Cards 207-209 bound portable `HarnessMode::Plan`. Omission keeps
`--approval-mode default`. Applied `session_start.permission_mode` is
observed. `auto-edit|auto|yolo` stay withheld.

Keep g04 open. Contract 029 currentness remains standing.

## Post-g04.075 Reassessment And g04.076 Compilation (2026-08-26)

Disposition: select provider sandboxing on `cursor-agent.headless` and compile
g04.076 as one serial evidence-first lane.

The remaining inventory was re-ranked after Qwen Plan delivery. Account and
billing controls, ambient model/tool registries, product personality, writable
profiles, and process-topology features remain weaker leads. Cursor sandboxing
is a material containment gap on a production route with an exact owned-child
seam and an existing portable isolation boundary.

All four qualified Cursor builds expose `--sandbox enabled|disabled`.
Swallowtail already owns one explicit-model structured child, binds
`Read|ReadWrite`, adds `--mode plan` only for `Read`, records
`HarnessIsolation::AmbientHost`, and proves deadline, cancellation, activity,
terminal, durable state, and joined cleanup. Current official Cursor material
describes native filesystem, network, and subprocess restrictions.

That is a credible candidate, not a containment claim. Current material also
says sandboxing applies to supported terminal commands, incompatible commands
may move toward approval, and user/project/team path or network configuration
can alter the boundary. Card 210 and Research 223 must bind exact build,
platform/backend, configuration precedence, filesystem/network/subprocess
rules, approval and escape paths, fallback, observation, and both access
profiles. Mutable current documentation cannot backport semantics to an exact
artifact.

Only `HarnessIsolation::ProviderEnforced` through canonical
`--sandbox enabled` is eligible. Omission remains exact no-flag
`AmbientHost`. `disabled`, raw configuration, network/path policy, host
isolation, force/yolo/auto-review, approval exchange, sibling routes, live
provider work, currentness, release, generation rollover, and g04 closure stay
out. If no exact preflight-bound row proves the full Contract 023 boundary,
Research 223 must promote an empty set and the lane stops after card 210.

## Post-g04.076 Closeout (2026-08-26)

Disposition: `cursor-agent.headless` provider sandboxing stops after card 210.
Research 223 is an empty deliver-now set. Reassess the remaining inventory
before compiling another serial family.

All four qualified Cursor builds parse `--sandbox enabled|disabled` and the
CLI override beats persisted `sandbox.mode`. Exact source still binds the
flag to a `cursorsandbox` shell-exec helper, not to the harness process.
Darwin "supported" is `/usr/bin/sandbox-exec` plus helper-binary presence.
Ambient `sandbox.json`, config, team controls, and the feature gate can widen
or disable the boundary. Print mode without `--force` denies approval rather
than containing the process. File/MCP/fetch tools are outside the helper.
Cards 211-212 stay blocked. Omission remains `AmbientHost` with no sandbox
argument.

Keep g04 open. Contract 029 currentness remains standing.

## Post-g04.076 Reassessment And g04.077 Compilation (2026-08-26)

Disposition: select Ask on `cursor-agent.headless` and compile g04.077 as one
serial evidence-first lane.

The remaining inventory was re-ranked after the Cursor sandbox evidence stop.
Account and billing controls, ambient model/tool registries, product
personality, writable profiles, and process-topology features remain weaker
leads. Claude headless autocompact is concrete, but current official docs state
that `CLAUDE_CODE_AUTO_COMPACT_WINDOW` overrides the CLI flag and saved
settings. The selected execution host clears ambient environment and applies
an opaque approved environment reference, so the adapter cannot preflight that
override without new environment-inspection authority.

Cursor Ask has a smaller authority seam. The four exact qualified builds
expose `--mode ask` beside Plan; prompt-free probes accept `ask|plan` and reject
Agent, case variants, invalid, and empty values. Current official documentation
describes Ask as read-only exploration and Q&A. The route owns the exact child,
working-resource access, explicit model and parameters, ambient configuration,
deadline, cancellation, activity, terminal result, durable retention, and
joined cleanup. Existing `Read` selects `--mode plan`; `ReadWrite` omits mode.

That evidence qualifies investigation, not delivery. Card 213 and Research
224 must freeze exact source, parser and repeated-value precedence,
configuration interaction, mode application, read-only behavior, tool/write
seams, stream observation, and all four qualified builds. Ask is eligible only
as a closed Cursor-local selection for `ResourceAccess::Read`. It does not add
portable `HarnessMode`, isolation, containment, permissions, tools, approval,
network, or writable authority. Existing Plan and no-mode behavior must remain
exact.

Cards 214-215 run only for a non-empty exact Research 224 set. Raw provider
modes, Agent, force/yolo/auto-review, sandboxing, live provider work,
currentness, release, generation rollover, and g04 closure stay out. If exact
behavior cannot be proved without provider work or ambient widening, the lane
stops after card 213.

## g04.077 Cursor Ask Outcome (2026-08-26)

Disposition: deliver Cursor headless Ask as one closed Cursor-local selection
at qualified dispatch and application; withhold effective and observed mode.

Card 213 and Research 224 froze the full chain on all four qualified builds.
Selection is exact and closed: commander `.choices(["plan","ask"])` with no
default, case-sensitive, rejecting `agent`, empty, and list values; no
persisted-config or environment key competes; a fresh headless session
inherits no mode metadata; and headless refuses model-initiated switch-mode
requests. `--plan` beats `--mode ask` in `chat.ts`. Application is exact too:
`--mode ask` becomes agent-store metadata `"search"` and `AgentMode.ASK` on
the outbound `UserMessage`.

The claim stops there. Ask's only local consumer picks `workspace_readonly`
instead of `workspace_readwrite` for the shell-exec sandbox policy, and only
when the sandbox is available — which this route's argv never makes true, and
which ambient `sandbox.mode`, team, and feature-gate state control when it is.
No tool registry, approval path, or write refusal keys on Ask, and the
qualified stream reports no mode.

That bounds the documentation, not the dispatch. Cards 214-215 bound Ask
through `CursorHeadlessReadMode` at the same tier the route already uses for
`--mode plan` and Research 183 model parameters. Read-only intent still comes
from the declared `ResourceAccess::Read` authority, and Swallowtail rejects Ask
with `ReadWrite` before process work even though the native CLI would not.

Raise the claim only for an exact build that proves a local Ask boundary
independent of ambient sandbox, approval, team, and feature-gate state, or a
qualified observation channel that reports applied or effective mode.

## Post-g04.077 Reassessment And g04.078 Compilation (2026-08-26)

Disposition: select reasoning selection and budget on `llama-cpp.owned` and
compile g04.078 as one serial evidence-first lane.

The remaining inventory was re-ranked after Cursor Ask delivery. Codex
app-server Plan-mode effort is not selected: the route already carries the
active model and reasoning effort inside the collaboration-mode preset, so a
second route-local setting would duplicate existing selection. Account and
billing controls, ambient registries, product personality, writable profiles,
and process-topology features remain weaker or authority-blocked leads.

Exact llama.cpp `b10069-178a6c449` exposes `--reasoning on|off|auto` and
`--reasoning-budget -1|0|N`. The owned route controls the exact server child,
immutable launch plan, operator-supplied model path, context size, readiness,
cancellation, terminal state, and joined cleanup. This is a credible
serving-owned seam.

It is not yet a reasoning capability. `auto`, explicit selection, and budget
behavior depend on the selected GGUF/chat template, reasoning start/end tags,
formatting, and server application. The operator supplies the model, and a
non-reasoning template may make a parsed flag ineffective. Card 216 and
Research 225 must freeze exact parser and precedence, model/template
applicability, failure or silent-no-op behavior, and prompt-free observation.
Requested, prepared, dispatched, parser-accepted, applied, effective, and
observed state stay separate.

Cards 217-218 run only for a non-empty exact Research 225 set whose decisive
model/template facts can be bound or rejected before process work. Omission
must preserve the current no-reasoning-argument launch and compose exactly
with context size. Raw provider values, portable reasoning APIs, model
download/load, template or format changes, attached-route inference, live
prompting, currentness, release, generation rollover, and g04 closure stay
out. If exact behavior needs a live model run or unbound ambient facts, the
lane stops after card 216.

### Outcome (2026-08-26)

Research 225 admitted one row and cards 216-218 delivered it. `--reasoning off`
ships as adapter-local dispatch-only owned-serving configuration. It is the
only value that needs no model or template fact before process work: exact
`common/arg.cpp` stores `enable_reasoning = 0`, and exact
`tools/server/server-context.cpp` evaluates
`enable_reasoning != 0 && template_supports_thinking`, so `0` short-circuits
before the template probe.

Everything else was withheld. `auto` stores the parser default and writes no
template kwarg, so it is byte-equivalent to omission. `on` shares the default's
startup result and differs only through a request-time
`chat_template_kwargs` override this serving route cannot observe.
`--reasoning-budget` is discarded whole by
`if (!chat_params.thinking_end_tag.empty())` in
`tools/server/server-common.cpp`, and that tag is produced per request, so
launch-time `0` or `N` can be silently inert with no warning or log.

Observation is closed on this route: `/props` `chat_template_caps` carries
exactly eight keys and none reports thinking support, `task_params::to_json`
emits no `reasoning_budget_tokens` in either branch, and the one
`thinking = %d` startup line is `LOG_TRC`, above the default verbosity of `3`.
The triage row for `llama-cpp.owned` reasoning is closed.

## Post-g04.078 Reassessment And g04.079 Compilation (2026-08-27)

Disposition: select maximum agentic turns on `claude-code.headless` and compile
g04.079 as one serial evidence-first lane.

The remaining inventory was re-ranked after llama.cpp owned reasoning
delivery. Claude Code autocompact remains weaker because current official
material says `CLAUDE_CODE_AUTO_COMPACT_WINDOW` can override the CLI while the
route cannot inspect its opaque approved environment. Fast and spend caps carry
account/billing authority. Agent, advisor, and team controls change model or
process topology. Writable modes and fallback remain deliberate withholds.

Maximum turns has a narrower route-owned seam. `claude-code.headless` already
owns one exact child, a read-only Plan profile, fixed `Read,Glob,Grep`, selected
model and reasoning, strict empty MCP, no session persistence, bounded stream
decoding, a host deadline, terminal mapping, and joined cleanup. Current
official docs describe positive print-mode `--max-turns` over agentic tool-use
turns, `error_max_turns` at the bound, and
`CLAUDE_CODE_MAX_TURNS` as an equivalent lower-precedence environment value.

That is a credible candidate, not delivery evidence. The exact qualified
window remains `2.1.220..=2.1.241`; its frozen ceiling help does not advertise
the flag, and current docs explicitly say help is incomplete. Card 219 and
Research 226 must freeze exact support membership, numeric parser domain,
repetition and environment precedence, counted-turn meaning, native loop
enforcement, stream/result/usage/exit truth, and current decoder behavior.
Mutable docs cannot backport any of those facts.

Cards 220-221 run only for a non-empty exact Research 226 set. Any binding is a
closed Claude Code-local positive maximum, never `OutputTokenLimit`, tool-call
budget, cost, wall time, or portable generation control. Omission keeps the
exact no-flag argv and approved environment; it makes no unlimited-execution
claim because ambient `CLAUDE_CODE_MAX_TURNS` may remain present. Explicit
selection is eligible only if exact argv overrides that ambient value and the
native loop enforces the selected bound.

`--max-budget-usd`, autocompact, Fast, Ultracode, schema, advisor, agents,
teams, fallback, permission changes, response-only, ACP, live provider work,
currentness, release, generation rollover, and g04 closure stay out. If exact
enforcement or terminal truth needs provider prompting, Research 226 must
promote an empty set and the lane stops after card 219.

## 2026-08-27 Maximum-Turn Outcome

Research 226 and g04.079 cards 219-221 closed this row. The evidence-first
gate passed, so binding and acceptance both ran.

Three facts changed the shape of the delivery from what the disposition above
expected.

- Help omission was the wrong signal. `--max-turns` is registered at every
  published version in `2.1.220..=2.1.241` and then explicitly hidden with
  `hideHelp()`. Absence from the frozen ceiling help meant nothing.
- The native parser is far wider than the documented positive domain. It
  coerces with `Number` and rejects only `NaN`, so zero, negatives, fractions,
  `Infinity`, exponent and hexadecimal forms, grouped digits, and the empty
  string all parse. The loop guard is a truthiness test, so a resolved `0`
  would be inert. The adapter therefore closes the domain itself rather than
  trusting Claude Code's own check.
- Explicit argv precedence is unconditional and provable without touching the
  approved environment. The resolver returns the argv value before it ever
  reads `CLAUDE_CODE_MAX_TURNS`, including for argv values the environment
  itself would reject.
- The route's qualified window is not the feature's evidence set. It permits
  later stable points as `UnverifiedNewer`, and its semantic segment contains
  `2.1.230`, which was never published. The selection therefore gates on the
  exact probed set and rejects both cases before process work.

Omission is unchanged and still makes no unlimited-execution claim: with the
flag absent an ambient `CLAUDE_CODE_MAX_TURNS` stays authoritative, and an
invalid ambient value aborts Claude Code at startup with exit `1`. That is
existing route truth, now written down rather than assumed.

Reaching the bound emits `error_max_turns` with no `result` field and exits
`1`. The existing decoder already reports that as a provider failure with no
output and unchanged joined cleanup, so terminal mapping did not widen and no
new diagnostic was admitted.

Everything the disposition excluded stayed excluded. The remaining Claude Code
rows in the table above are unchanged.

## Post-g04.079 Reassessment And g04.080 Compilation (2026-08-27)

Disposition: select provider-owned web search on `xai.responses-websocket` and
compile g04.080 as one serial evidence-first lane.

The remaining inventory was re-ranked after Claude Code maximum-turn delivery.
Codex app-server v2 exposes no first-class thread or turn verbosity control.
Claude Code autocompact remains shadowed by opaque approved-environment
authority. Ollama `think=max` maps to `high` on the exact relevant model family.
Account/billing controls, writable modes, ambient registries, and process
topology still carry wider or less observable authority.

The xAI seam is direct. The route already owns one exact Responses WebSocket,
selected Grok 4.5/4.6 models, structured runs, serial private continuation,
reasoning, output bounds, `store=false`, usage, billed cost, restoration, and
joined cleanup. Every current request emits `tools: []`.

Current primary xAI documentation says WebSocket `response.create` uses the
Responses create body and separately documents `web_search` on `grok-4.6`,
server-side search-call items, citations, and tool-turn bounds. That is a lead,
not delivery evidence. Card 222 and Research 227 must freeze exact WebSocket
composition, model/profile membership, canonical tool syntax, a positive
provider-side bound, response grammar, citations, usage, billing, failures,
terminal ordering, continuation, restoration, and cleanup.

Cards 223-224 run only for a non-empty exact Research 227 set. The public intent
is existing `ExternalSearchPolicy::Enabled`; exact xAI tool shape and bound stay
adapter-owned. Host external networking stays denied because the provider owns
the search. Omission must remain byte-equivalent with `tools: []`.

X and image search, code execution, files, MCP, functions, consumer tool
exchange, raw tool arrays, caller filters, host fetch, live provider work,
currentness, release, generation rollover, and g04 closure stay out. If exact
support or bounded event truth needs provider prompting, Research 227 must
promote an empty set and the lane stops after card 222.

## Post-g04.080 Evidence Stop (2026-08-27)

Disposition: stop. Research 227 admits no deliver-now row.

Official HTTP Responses `tools: [{"type":"web_search"}]` and WebSocket
`response.create` body-equivalence are frozen. `grok-4.6` is a candidate;
`grok-4.5` is withheld. `max_turns: 1` is the smallest documented positive
turn bound. Composed WebSocket search events, mixed `web_search_call` plus
message completed output, and citation/terminal mapping are not frozen
without provider work. The current fail-closed decoder requires a single
assistant message. Cards 223-224 are blocked. Omission remains `tools: []`.

## Post-g04.080 Reassessment And g04.081 Compilation (2026-08-27)

Disposition: select reasoning selection on `pi.sdk-sidecar` and compile
g04.081 as one serial evidence-first lane.

The historical inventory predates the SDK-sidecar route and much of the
delivered programme. The current production matrix still reports
`pi.sdk-sidecar` `reasoning_selection` as `No`. This is now the strongest
remaining route-local seam: the source-tagged sidecar accepts optional
bootstrap `thinkingLevel`, passes it into exact Pi SDK construction, and
reports `session.thinkingLevel` in bootstrap and state snapshots. Rust
preparation and startup deliberately omit and ignore it.

That existing seam does not itself qualify a control. Exact Pi 0.84.2 source
clamps a requested thinking level to the selected model's capabilities. Card
225 and Research 228 must therefore freeze the exact vocabulary, a closed
provider/model/value table, clamping and fallback behavior, stored/default
precedence, runtime replacement, and state confirmation. Static membership
must allow unsupported rows to reject before process or credential work.

New, load, resume, replacement, and fresh restoration remain separate. Any
claimed attachment must reapply the caller-declared mode and confirm the same
effective value before readiness. Omission preserves existing Pi
default/stored behavior and claims no portable selection.

Cards 226-227 run only for a non-empty exact Research 228 set. Dynamic level
changes, cycling, model switching, raw settings, `pi.rpc`, newer SDK
currentness, live provider work, release, generation rollover, and g04 closure
stay out.

## Post-g04.081 Completion (2026-08-27)

Disposition: complete through cards 225-227.

Research 228 admits one bounded deliver-now family on exact Pi `0.84.2`:
`anthropic` / `claude-opus-4-5` with `off`, `minimal`, `low`, `medium`, and
`high`. Static pi-ai metadata gates preparation; auth filters runtime
availability only. Cards 226-227 bind portable `ReasoningSelection`, dispatch
canonical bootstrap `thinkingLevel`, compare bootstrap/state before readiness,
and preserve omission. Feature matrix `reasoning_selection` is `Yes` for the
closed row.

## Post-g04.081 Evidence Stop (2026-08-27)

Disposition: superseded by completion above. The initial empty-set promotion
was reversed after review established static `getModel` membership for the
qualified row.

Card 225 froze exact Pi `0.84.2` thinking-level vocabulary, silent
`clampThinkingLevel` behavior, explicit bootstrap precedence over stored
session state, runtime factory reapplication on load/resume, and current
sidecar/Rust seams. The sidecar already accepts optional bootstrap
`thinkingLevel` and reports `session.thinkingLevel`, but Rust omits and ignores
it. Sidecar catalogue returns provider/id only. The bundled pi-ai `0.84.2`
corpus contains 1267 models; selectable models additionally depend on
configured auth. Contract 040 forbids portable clamping, so deliver-now
requires a closed static provider/model/value gate before process work. No
such table survives. Omission retains exact prior bootstrap bytes and Pi
default/stored behavior.

## Post-g04.081 Inventory Normalization (2026-08-27)

Disposition: promoted to the
[live per-route feature inventory](../roadmaps/g04/per-route-feature-inventory.md).

The original 85 numbered pairs now have one maintained disposition: 41 are
closed by numbered delivery, evidence-stop, correction, or withhold lanes; 34
remain active qualification candidates; ten have no active lane under current
policy, evidence, or contracts. This append-only assessment no longer owns the
live count or sequence.

The next acceleration unit is four independent evidence-only lanes: Codex
app-server verbosity, Gemini CLI headless thinking, Bedrock latency/service
tier, and Ollama think `max`. Shared programme, triage, matrix, index, and Next
Task promotion remains serial.

## Post-g04.082 Parallel Qualification Completion (2026-08-27)

Disposition: complete through cards 228-231 and Research 229-232.

All four evidence-only lanes close with honest empty deliver-now sets. Codex
app-server exposes no typed confirmable model-verbosity seam. Gemini headless
thinking is settings-backed, but the qualified adapter binds no isolated
settings seam and stream-json confirms no effective value. Bedrock Runtime
request fields cannot become a route-open control without closed model,
region, inference-profile, account, billing, and returned-state truth. Ollama
accepts `max` at the wire parser from `0.22.0`, but generic thinking capability
does not prove selected-model membership and some native paths rewrite it.

Original items 19, 32, 81, and 83 move to the closed disposition set. The live
inventory now owns 45 closed items, 30 active qualification candidates, and ten
with no active lane. No binding or acceptance roadmap follows from g04.082.
Compile the next bounded route-distinct evidence wave from the remaining active
backlog. Keep shared promotion serial and g04 open.

## Post-g04.082 Reassessment And g04.083 Compilation (2026-08-27)

Disposition: compile four package-distinct evidence-only lanes.

The next wave selects original items 2, 15, 30, and 76: Claude Code headless
Fast mode, Codex exec Fast/service tier, Gemini CLI ACP thinking, and OpenAI
Realtime reasoning effort. They touch four packages and have no evidence
dependency. Cards 232-235 and Research 233-236 own the questions.

The shared words do not create shared controls. Claude Fast remains a
subscription/model/credit setting. Codex Fast remains an exec configuration,
feature-gate, model-catalogue, and billing question. Gemini thinking must be
proved on ACP rather than promoted from the stopped headless lane. Realtime
reasoning must be tied to its exact model and session lifecycle rather than
borrowed from Responses.

Workers may freeze only route-local evidence and their assigned card, Research,
and log. Shared inventory, programme, triage, matrices, indexes, Next Task, and
any production binding remain serial. Provider prompts, credentials, paid work,
account inspection, install/update, ambient mutation, release, rollover, and
g04 closure stay out.

## Post-g04.083 Completion And g04.084 Compilation (2026-08-27)

Disposition: three evidence stops; one bounded delivery lane.

Research 233 closes Claude Code headless Fast because effective activation,
model/access membership, credit entitlement, billing, and latency cannot be
proved without account or provider work. Research 234 closes Codex exec Fast
because live-catalog substitution and silent tier downgrade prevent static
pre-prompt confirmation. Research 235 closes Gemini CLI ACP thinking because
ACP exposes no selected thinking option or confirmation seam.

Research 236 admits five future session-scoped OpenAI Realtime rows on exact
`gpt-realtime-2.1`: `minimal`, `low`, `medium`, `high`, and `xhigh`. The current
dated facade remains an honest empty set. g04.084 cards 236-237 bind those rows
at a new opaque facade point, require matching `session.updated`
acknowledgement, preserve omission, and keep per-response override and effective
reasoning out.

Original items 2, 15, and 30 move to the closed disposition set. Item 76 moves
to active delivery. The live inventory now owns 48 closed items, 26 active
qualification candidates, one active delivery item, and ten with no active
lane. PRs 86, 88, 87, and 85 landed serially through `c918d301`.

## Post-g04.084 Reassessment And g04.085 Compilation (2026-08-27)

Disposition: compile four package-distinct evidence-only lanes.

The next wave selects original items 4, 20, 31, and 41: Claude Code headless
autocompaction, Codex app-server personality, Gemini CLI headless sandboxing,
and Cline ACP Plan mode. They touch four packages and have no evidence
dependency. Cards 238-241 and Research 237-240 own the questions.

This selection favors exact official surfaces with bounded prompt-free
qualification paths. Autocompaction must not become context size; Codex
personality remains thread/turn/config-specific; Gemini sandbox selection does
not imply portable containment; and Cline headless Plan evidence does not
promote onto ACP.

Workers may freeze only route-local evidence and their assigned card, Research,
and log. Shared inventory, programme, triage, matrices, indexes, Next Task, and
any production binding remain serial. Provider prompts, credentials, paid work,
account inspection, install/update, ambient mutation, release, rollover, and
g04 closure stay out.

## Post-g04.085 Completion And g04.086 Compilation (2026-08-27)

Disposition: three evidence stops; one bounded delivery lane.

Research 237 closes Claude Code headless autocompaction because ambient
environment gates defeat operation-private precedence and prompt-free effective
confirmation is absent. Research 238 closes Codex app-server personality
because model membership, pre-effect rejection, returned selection, and cold
restoration do not close. Research 239 closes Gemini headless sandboxing because
ambient precedence, backend activation, and containment truth remain open.

Research 240 admits one exact `cline.acp` `3.0.55` `HarnessMode::Plan` row.
`session/new` advertises `plan`; one pre-prompt `session/set_config_option`
request selects it; the response confirms `mode.currentValue = plan`; the first
runtime manager builds from that stored mode. Root `--plan`, provider Act,
post-start mutation, load restoration, permissions, and containment stay out.

Original items 4, 20, and 31 move to the closed disposition set. Item 41 moves
to active delivery. The live inventory now owns 52 closed items, 22 active
qualification candidates, one active delivery item, and ten with no active
lane. PRs 94, 93, 91, and 92 landed serially through `abdaefd2`.

g04.086 cards 242-243 own Cline ACP Plan binding and route-local acceptance.
Shared closeout remains serial after merge. Keep g04 open.

## Post-g04.086 Cline ACP Plan Delivery (2026-08-27)

Disposition: original item 41 delivered and closed.

Cards 242-243 bind Research 240's exact `cline.acp` `3.0.55`
`HarnessMode::Plan` row. New sessions require unambiguous `plan` membership,
one pre-prompt `session/set_config_option` request, and exact returned
`mode.currentValue = plan` agreement before readiness. Omission emits no mode
request. Root `--plan`, Act selection, post-start mutation, load/resume
mutation, permission widening, and containment claims remain out.

PR 95 landed by fast-forward at `3f56aeb4`. The live ledger now owns 53 closed
items, 22 active qualification candidates, no active delivery item, and ten
items with no active lane. The next checkpoint compiles another bounded
parallel qualification wave. Keep g04 open.

## g04.087 Fourth Parallel Qualification Compilation (2026-08-27)

Disposition: select the lowest four package-distinct active candidates as
evidence-only lanes.

1. item 6 — Claude Code headless `--max-budget-usd`
2. item 18 — Codex app-server Fast / service tier
3. item 28 — Cursor ACP Fast, effort, and context model parameters
4. item 29 — Gemini CLI ACP native sandbox

Items 8-9 and 11-12 stay behind the Claude lane because they share
`swallowtail-adapter-claude-agent`. Item 21 stays behind the Codex lane. The
four selected workers otherwise own distinct packages, cards 244-247, Research
241-244, logs, and optional adapter-local frozen evidence.

Selection is not qualification. Claude subscription access must not become
API-key USD billing. Codex ChatGPT-credit Fast and API Priority remain
distinct. Cursor headless model strings do not authorize ACP parameters.
Gemini headless sandbox evidence does not settle ACP spawn/activation, and
sandbox selection never proves containment.

Run workers in parallel through manual harness handoffs. Integrate A-B-C-D
serially after exact-head review. Shared inventory, programme, triage, matrices,
indexes, and Next Task remain orchestrator-owned. Do not begin production
binding from an evidence worker. Keep g04 open.

## Post-g04.087 Completion And g04.088 Compilation (2026-08-28)

Disposition: four evidence stops; compile the next four package-distinct
qualification lanes.

Research 241 closes Claude headless spend capping because its local
catalog-priced USD ledger is not the selected subscription billing unit.
Research 242 closes Codex app-server Fast because membership can be softly
dropped or substituted, preference can diverge from request bytes, cold
persistence is absent, and the feature gate remains ambient. Research 243
closes Cursor ACP model parameters because membership is account-gated and the
selected ACP path lacks independent parameter selection and confirmation.
Research 244 closes Gemini ACP sandbox because ambient precedence, pre-ACP
re-exec/stdin handling, backend readiness, and confirmation do not close.

Original items 6, 18, 28, and 29 move to closed. The live ledger now owns 57
closed items, 18 active qualification candidates, no active delivery item,
and ten with no active lane. PRs 99, 97, 98, and 96 landed serially through
`e40a5407`. No production binding follows.

g04.088 selects the lowest remaining package-distinct items:

1. item 8 — Claude Code headless `--advisor`
2. item 21 — Codex app-server `plan_mode_reasoning_effort`
3. item 34 — Grok Build ACP `--disable-web-search`
4. item 42 — Cline ACP model selection

Cards 248-251 and Research 245-248 own evidence only. Advisor model/spend,
Plan-specific effort, provider web-search suppression, and ACP model selection
remain separate route-local claims. Run manual workers in parallel; integrate
A-B-C-D serially. Shared closeout and any non-empty delivery compilation stay
with the orchestrator. Keep g04 open.

## Post-g04.088 Completion And g04.089 Compilation (2026-08-28)

Disposition: four evidence stops; compile the next four package-distinct
qualification lanes.

Research 245 closes Claude headless advisor because entitlement, operation-
private precedence, effective attachment, unsupported-token rejection, and
extra-spend truth stay open. Research 246 closes Codex app-server Plan effort
because the key is ambient/session-static and the typed protocol has no
Plan-specific seam, confirmation, or restoration. Research 247 closes Grok
ACP web-search disable because exact application and backend suppression are
not observable from the qualified prompt-free surface. Research 248 closes
Cline ACP model selection because provider/model membership, route agreement,
and pre-effect rejection do not close.

Original items 8, 21, 34, and 42 move to closed. The live ledger now owns 61
closed items, 14 active qualification candidates, no active delivery item,
and ten with no active lane. PRs 104, 101, 103, and 102 landed serially through
`23278abe`. No production binding follows.

g04.089 selects the lowest remaining package-distinct items:

1. item 9 — Claude Code headless permission modes beyond Plan
2. item 46 — Goose ACP `--with-builtin`
3. item 48 — Kiro ACP `--effort`
4. item 54 — Mistral Vibe headless `--agent` beyond Plan

Cards 252-255 and Research 249-252 own evidence only. Dangerous permission
bypass, auto-approval, host extension setup, sibling-route inference, and
unconfirmed model/value membership remain out. Run manual workers in parallel;
integrate A-B-C-D serially. Shared closeout and any non-empty delivery
compilation stay with the orchestrator. Keep g04 open.

## Post-g04.089 Completion And Remainder Audit (2026-08-28)

Disposition: four evidence stops, sixteen audited closures, two parked Bedrock
rows, and two residual qualification lanes.

Research 249-252 admit honest empty deliver-now sets for Claude headless
permission modes, Goose ACP builtins, Kiro ACP effort, and Mistral Vibe
agent profiles beyond Plan. PRs 109, 107, 106, and 108 landed serially through
`e28979a0`. Original items 9, 46, 48, and 54 close.

The full remainder audit closes original items `3, 10-14, 17, 22, 39, 51, 61,
64, 69-71, 75`. They are process topology, dominated evidence questions,
already-relayed permission vocabulary, another model or route, operator-owned
agent-definition state, or host-path authority owned by another programme.
Deep Agents skills/memory moves to the harness skill-discovery and
resource-lifetime triage note.

Bedrock items 79-80 stay parked. Model-specific thinking reopens only for an
exact admitted model, region/profile, and official schema. Tools/guardrails
reopen only after contracts and the route guide admit a bounded surface.

g04.090 retains only item 47, Goose ACP mode, and item 49, Kiro ACP `--agent`.
Cards 256-257 and Research 253-254 own evidence only. Run both manual workers
in parallel; integrate Goose then Kiro. Keep g04 open.

## Post-g04.090 Residual Qualification Closeout (2026-08-28)

Disposition: two evidence stops; no active per-route feature lane remains.

Research 253 closes Goose ACP mode with an honest empty deliver-now set.
Membership is closed, but persisted malformed mode falls back to auto,
available non-chat modes widen permission authority, and confirmation needs a
live provider-backed session. Research 254 closes Kiro ACP agent profiles with
an honest empty set because membership is ambient, missing names silently fall
back, and ACP exposes no applied-profile confirmation.

PR 111 landed Goose at `9e317e20`; restacked PR 110 landed Kiro at
`96b937d1`. Original items 47 and 49 move to closed. The live ledger now has 83
closed dispositions, no active qualification or delivery item, and two parked
Bedrock items. No production binding follows. g04 remains open at operator
direction.
