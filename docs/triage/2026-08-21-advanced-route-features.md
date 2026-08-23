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
| Agent profile | `--agent` | not passed | yes | only-if: `agy agents` list is bound as exact ids | do not flatten onto Claude `--agent` |
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
| Max turns | `--max-turns` | omitted | yes | yes | print-mode only |
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
Changelog: ACP clients can specify reasoning effort when opening or
resuming (Grok Build 1.0.x).

Swallowtail: `grok --no-auto-update agent stdio`. Session-negotiated
models. Matrix `reasoning_selection` No. 1.0.4–1.0.5 binds `grok-4.6`
and effort `xhigh` internally without a public reasoning claim.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Effort | `--effort` / ACP open effort | not a public selection | yes | only-if: qualify ACP option on 1.0.4+ | do not flatten onto `xai.responses-websocket` |
| Max turns | `--max-turns` | not passed (headless flag; ACP spawn has no extra argv) | yes | only-if | official note: `--max-turns` is headless `-p` |
| Web search disable | `--disable-web-search` | not passed | yes | only-if | search column is No |
| Subagents off | `--no-subagents` | not passed | yes | only-if | not a swarm control |
| YOLO | `--always-approve` | withheld | official unused | no | permission-stop truth |

### `kimi-code.acp`

Official CLI
([overrides](https://moonshotai.github.io/kimi-cli/en/configuration/overrides.html)):
`--thinking` / `--no-thinking`, `--plan`, `--yolo`/`--afk`, `-m`.

Swallowtail: ACP `SessionOptions` reasoning
`off|on|low|medium|high`; load/resume/import. No plan mode. No
permission/question exchange. Headless reasoning is a different branch.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Plan mode | `--plan` / `default_plan_mode` | not mapped | yes | only-if: ACP config option exists | do not copy `--plan` argv onto ACP spawn |
| YOLO / AFK | `--yolo`, `--afk` | withheld | official unused | no | — |
| Effort `xhigh`/`max` | config `[thinking].effort` | portable set stops at `high` | yes | only-if: model `support_efforts` | exact mapping; no UI labels |

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

Swallowtail: `cline --acp` only. Observe/cancel permissions. Never
`allow_always`. No `--auto-approve true`.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Thinking | `--thinking` | not passed | yes | yes | ACP spawn flag, like Copilot effort |
| Plan | `-p, --plan` | not passed | yes | yes | Cline plan/act ≠ Swallowtail HarnessMode unless mapped |
| Model | `-m, --model` | caller-supplied / harness default | yes | yes | — |
| Auto-approve | `--auto-approve true` | withheld | official unused | no | guide withhold |

### `cline.headless`

Official: `cline --json --auto-approve <bool> -p --thinking -m -t`.

Swallowtail: `--json --auto-approve false`. No model, no thinking, no
`--plan`. CLI `--timeout` unselected (host deadline used).

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Thinking | `--thinking` | not passed | yes | yes | do not flatten onto `cline.acp` without repeating qualification |
| Plan | `-p` | not passed | yes | yes | — |
| Model | `-m` | not passed | yes | yes | — |

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
| Model | `--model` | not passed | yes | yes | host keys still required |
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
`--permission-mode dont_ask`, `--max-turns 8`,
`--no-session-persistence`. Does not pass `--yolo`,
`bypass_permissions`, `accept_edits`. Independent official page not
re-fetched; extra flags unconfirmed.

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

Swallowtail: `--safe-mode --approval-mode default`, fixed wall/tool/turn caps,
and exact `0.21.15` reasoning selection for `qwen3.8-max` and
`qwen3.8-max-preview`. Matrix: reasoning Yes, output limit No, search No.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Effort | `/effort` / `model.reasoningEffort` | exact private stream-JSON control on `0.21.15` | delivered for two exact models | yes | no retroactive claim before `0.21.15` |
| Approval | `--approval-mode` / `--yolo` | fixed `default` | other modes unused | only-if: never yolo by default | — |
| Turn/tool caps | official flags | fixed constants | consumer override unused | yes | — |

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
with two uses + domain allowlist. Matrix: `reasoning_selection` No.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Effort | `output_config.effort` | not mapped | yes | yes | not UltraCode; not Claude Code `--effort` |
| Thinking | `thinking` | not mapped | yes | only-if: keep distinct from effort | official: do not pass `adaptive` as effort |
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

Official server: `--reasoning`, `--reasoning-effort`,
`--reasoning-budget`, `--ctx-size` at **server** start.
Attached route is OpenAI-chat HTTP against an already-running server.
Swallowtail: output-token bound only.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Per-request reasoning | depends on server build | not mapped | unconfirmed on this HTTP facade | only-if: request field exists on b9910 | do not invent |
| Ctx | `--ctx-size` | server-owned | none on attached | no | boot flag |

### `llama-cpp.owned`

Serving lifecycle only. Official `--ctx-size`, `--reasoning`,
`--reasoning-effort`, `--n-predict` belong on `llama-server` start.
Inference stays on a separately prepared attached route.

| Feature | Official | Swallowtail | Gap | Composer-surfaceable | Incompatible reason |
| --- | --- | --- | --- | --- | --- |
| Ctx / reasoning at serve | `--ctx-size`, `--reasoning-effort` | not on serving input | yes | only-if: serving profile, not message composer | no inference role |

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
57. `qwen.headless` / `--approval-mode` beyond `default`
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
85. `llama-cpp.owned` / `--reasoning-effort`

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
