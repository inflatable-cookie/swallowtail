# 269 All-Route Version Currentness Checkpoint

Status: promoted
Owner: Tom
Date: 2026-09-01

## Question

After Pi RPC `0.84.4` qualification, what are the current official stable
points and safe local observations for every production family, and which one
family should enter the next Contract 029 Upgrade Workflow run?

This checkpoint is research only. It does not change a compatibility claim,
feature-matrix row, fixture, or route.

## Method

Compared:

- all 40 rows in the current production feature matrix with the current
  adapter `selection.rs` claims;
- safe local `command -v` and `--version` observations for named executables;
- official npm `latest`, GitHub latest stable releases or tags, the Kiro
  stable manifest, crates.io max stable, PyPI, and ACP registry metadata;
- official hosted-API documentation for the opaque facade routes.

Observation date is 2026-09-01. Safe `command -v` and `--version` checks under
the normal host login resolved `qwen`, `claude-agent-acp`, `claude`, `pi`,
`command-code`, `cursor-agent`, `agy`, `gemini`, `llama-server`, `muse`,
`kimi`, `omp`, `ollama`, `codex`, `opencode`, and `grok`; their observations
are recorded in the table. `cline`, `dsh`, `copilot`, `goose`, `kiro-cli`,
`deepagents-acp`, `vibe`, `qodercli`, and `zcode` remained missing from
`PATH`. A missing local install is an observation, not a compatibility gap.
Login PATH prefers `/Users/tom/.local/bin/codex` (`codex-cli 0.150.1`) over
Homebrew `codex-cli 0.146.0`; the login observation is recorded.

No provider prompt, authentication, catalogue, session, install, update, host
change, or live probe was used. Official artifacts were not replaced or
executed. Hosted "latest model" values and ACP registry package versions were
not treated as Swallowtail compatibility claims.

## Compatibility Result

Result vocabulary:

- `unchanged` — the official point remains on the current qualified boundary,
  or the opaque facade has no replacement identity;
- `visible unverified-newer` — a later ordered stable exists and the claim
  already permits an exact forward attempt;
- `record only; future range work deferred` — a newer point exists, but the
  claim is exact/opaque, a major reset needs identity evidence, or an existing
  deferral remains;
- `material candidate` — enough evidence exists to select one family for a
  dedicated Upgrade Workflow card; this checkpoint does not compile that card.

The numbered rows below reproduce the 40 production feature-matrix families
exactly once. The boundary is the current repository claim, not a new claim
made by this record.

Partition: 15 unchanged, 5 visible unverified-newer, 19 record-only, 1 material
candidate.

| # | Surface | Local observation | Current official point | Swallowtail boundary | Result |
| ---: | --- | --- | --- | --- | --- |
| 1 | Qwen Code headless (`qwen.headless`) | `qwen` resolves; `--version` `0.21.2` | npm [`@qwen-code/qwen-code` `0.22.3`](https://registry.npmjs.org/@qwen-code%2Fqwen-code/latest), published 2026-08-28 | `0.19.11..=0.20.1` deprecated; `0.21.0..=0.21.14` deprecated; exact `0.21.15` and `0.22.0..=0.22.3` maintained; `0.21.16` gap; AllowUnverified | unchanged |
| 2 | Alibaba Model Studio Conversations and Responses (`alibaba.conversations`) | n/a; hosted API | Official [Responses API documentation](https://www.alibabacloud.com/help/en/model-studio/qwen-api-via-openai-responses), current page observed 2026-09-01; no replacement dated facade identity | exact `openai-conversations-responses` facade; QualifiedOnly | unchanged |
| 3 | Amazon Bedrock catalogue and Runtime (`bedrock.catalogue`; `bedrock.runtime`) | n/a; embedded SDK | crates.io [`aws-sdk-bedrockruntime` `1.142.0`](https://crates.io/crates/aws-sdk-bedrockruntime) and [`aws-sdk-bedrock` `1.154.0`](https://crates.io/crates/aws-sdk-bedrock), max versions observed 2026-09-01 | exact SDK/service axes; Cargo pins `aws-sdk-bedrockruntime =1.139.0` and `aws-sdk-bedrock =1.150.0`; exact service facades; QualifiedOnly | record only; future range work deferred |
| 4 | Claude Agent ACP (`claude-agent.acp`) | `claude-agent-acp` resolves; `--version` `0.63.0` | npm [`@agentclientprotocol/claude-agent-acp` `0.70.0`](https://registry.npmjs.org/@agentclientprotocol%2Fclaude-agent-acp/latest), published 2026-08-18; GitHub [`v0.70.0`](https://github.com/agentclientprotocol/claude-agent-acp/releases/tag/v0.70.0) | `0.53.0..=0.70.0` excluding `0.58.0`; AllowUnverified | unchanged |
| 5 | Claude Code headless and response-only (`claude-code.headless`; `claude-code.response-only`) | `claude` resolves; `--version` `2.1.251 (Claude Code)` | npm [`@anthropic-ai/claude-code` `2.1.252`](https://registry.npmjs.org/@anthropic-ai%2Fclaude-code/latest), published 2026-08-31; GitHub [`v2.1.252`](https://github.com/anthropics/claude-code/releases/tag/v2.1.252) | headless `2.1.220..=2.1.252`; response-only `2.1.227..=2.1.252`; gaps `2.1.244` and `2.1.249`; AllowUnverified | unchanged |
| 6 | Anthropic Managed Agents (`anthropic.managed-agent`) | n/a; hosted API | Official [Claude API versioning and Managed Agents reference](https://platform.claude.com/docs/en/api/versioning); no replacement beta facade identity observed | exact `anthropic.managed-agents-facade`; QualifiedOnly | unchanged |
| 7 | Anthropic Messages (`anthropic.messages`) | n/a; hosted API | Official [API versioning](https://docs.anthropic.com/en/api/versioning) still documents `anthropic-version: 2023-06-01` | exact `anthropic-2023-06-01` facade; QualifiedOnly | unchanged |
| 8 | Pi coding agent RPC (`pi.rpc`) | `pi` resolves; `--version` `0.83.0` | npm [`@earendil-works/pi-coding-agent` `0.84.4`](https://registry.npmjs.org/@earendil-works%2Fpi-coding-agent/latest), published 2026-08-28; GitHub [`v0.84.4`](https://github.com/badlogic/pi-mono/releases/tag/v0.84.4) | maintained exact published points through `0.84.4`; `0.83.1` gap; AllowUnverified | unchanged |
| 9 | Pi coding agent SDK sidecar (`pi.sdk-sidecar`) | `pi` resolves; `--version` `0.83.0` | same upstream npm package currently `0.84.4`; sidecar has no separate current package channel | exact `0.84.2` sidecar package, exact Node `22.23.2`, sidecar wire, and source-tag axes; QualifiedOnly | record only; future range work deferred |
| 10 | Cline ACP (`cline.acp`) | `cline` not on `PATH` | npm [`cline` `3.0.60`](https://registry.npmjs.org/cline/latest), published 2026-08-26 | exact `3.0.55`; QualifiedOnly | record only; future range work deferred |
| 11 | Cline headless (`cline.headless`) | `cline` not on `PATH` | npm [`cline` `3.0.60`](https://registry.npmjs.org/cline/latest), published 2026-08-26 | exact `3.0.55`; QualifiedOnly | record only; future range work deferred |
| 12 | Command Code headless (`command-code.headless`) | `command-code` resolves; `--version` `1.15.1` | npm [`command-code` `1.39.3`](https://registry.npmjs.org/command-code/latest), published 2026-09-01 | exact `1.15.1`; QualifiedOnly | record only; future range work deferred |
| 13 | Cursor Agent catalogue, ACP, and headless (`cursor-agent.catalogue`; `cursor-agent.acp`; `cursor-agent.headless`) | `cursor-agent` resolves; `--version` `2026.08.04-aaa8809` | ACP registry [Cursor `2026.08.11`](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json); no later registry build | exact `2026.07.01-41b2de7`, `2026.07.23-e383d2b`, `2026.08.04-aaa8809`, and `2026.08.11-e8db854`; no inferred gap; AllowUnverified | unchanged |
| 14 | DeepSeek Harness JSON-RPC (`deepseek-harness.jsonrpc`) | `dsh` not on `PATH` | npm [`@deepseek-ai/dsh` `0.1.1-rc.2`](https://registry.npmjs.org/@deepseek-ai%2Fdsh/latest), published 2026-08-21; current channel remains prerelease | exact runtime-bin `0.1.0rc6`; QualifiedOnly | record only; future range work deferred |
| 15 | DeepSeek Harness Web `/api` (`deepseek-harness.local-server`) | `dsh` not on `PATH` | same npm [`@deepseek-ai/dsh` `0.1.1-rc.2`](https://registry.npmjs.org/@deepseek-ai%2Fdsh/latest); current channel remains prerelease | exact Web `0.1.0-rc.6`; QualifiedOnly; do not flatten onto JSON-RPC | record only; future range work deferred |
| 16 | DeepSeek Open Platform continuation (`deepseek.continuation`) | n/a; hosted API | Official [DeepSeek Chat Completions API](https://api-docs.deepseek.com/api/create-chat-completion/) remains an unversioned OpenAI-compatible endpoint; no replacement dated facade identity | exact `deepseek-openai-chat-2026-07-22` facade; QualifiedOnly | unchanged |
| 17 | GitHub Copilot CLI ACP (`copilot-cli.acp`) | `copilot` not on `PATH` | npm [`@github/copilot` `1.0.82`](https://registry.npmjs.org/@github%2Fcopilot/latest), published 2026-08-29; prerelease `1.0.83-0` ignored | exact `1.0.80`; QualifiedOnly | record only; future range work deferred |
| 18 | Antigravity catalogue and headless (`antigravity.catalogue`; `antigravity.headless`) | `agy` resolves; `--version` `1.1.19` | GitHub release [`1.1.23`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.1.23), published 2026-09-01 | maintained `1.1.9..=1.1.17`; later stable points visible above the ceiling; AllowUnverified | visible unverified-newer |
| 19 | Gemini CLI ACP and headless (`gemini-cli.acp`; `gemini-cli.headless`) | `gemini` resolves; `--version` `0.53.0` | npm [`@google/gemini-cli` `0.57.0`](https://registry.npmjs.org/@google%2Fgemini-cli/latest), published 2026-08-25; GitHub [`v0.57.0`](https://github.com/google-gemini/gemini-cli/releases/tag/v0.57.0); preview/nightly ignored | both axes maintained `0.51.0..=0.56.0`; later stable unverified; Gemini requalification deferred | record only; future range work deferred |
| 20 | Gemini Live API (`gemini.live`) | n/a; hosted realtime API | Official [Gemini Live API](https://ai.google.dev/gemini-api/docs/live-api) overview remains the current Live API page observed 2026-09-01; no replacement dated facade identity | exact `google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent.thinking-output-max-context-compression-2026-08-23` facade; QualifiedOnly | unchanged |
| 21 | Goose ACP (`goose.acp`) | `goose` not on `PATH` | GitHub release [`v1.48.0`](https://github.com/aaif-goose/goose/releases/tag/v1.48.0), published 2026-08-27; repository moved from `block/goose` | exact `1.46.0`; QualifiedOnly | record only; future range work deferred |
| 22 | Kiro ACP (`kiro.acp`) | `kiro-cli` not on `PATH` | official [stable manifest](https://prod.download.cli.kiro.dev/stable/latest/manifest.json) reports `2.20.2` | exact `2.18.1`; QualifiedOnly | record only; future range work deferred |
| 23 | Deep Agents ACP (`deepagents.acp`) | `deepagents-acp` not on `PATH` | npm [`deepagents-acp` `0.1.28`](https://registry.npmjs.org/deepagents-acp/latest), published 2026-08-27; ACP registry `DeepAgents` remains `0.1.7` discovery metadata | exact `0.1.25`; QualifiedOnly; do not bind the stale registry value | record only; future range work deferred |
| 24 | llama.cpp attached server (`llama-cpp.attached`) | `llama-server` resolves; `--version` `0.1.0-dev (build 10450, commit ece963f41)` | GitHub latest release [`v0.3.0`](https://github.com/ggml-org/llama.cpp/releases/tag/v0.3.0), published 2026-08-25; release tag is not the selected build identity | exact attached build `b9910/f5525f7e7`; QualifiedOnly | record only; future range work deferred |
| 25 | llama.cpp owned server lifecycle (`llama-cpp.owned`) | `llama-server` resolves; `--version` `0.1.0-dev (build 10450, commit ece963f41)` | same GitHub latest [`v0.3.0`](https://github.com/ggml-org/llama.cpp/releases/tag/v0.3.0); no build-to-claim inference | exact owned build `b10069/178a6c449`; QualifiedOnly | record only; future range work deferred |
| 26 | Muse Code headless (`muse-code.headless`) | `muse` resolves; `--version` `Muse Code 1.0.1 (1.0.1-R2006.1)` | no public package or release channel for the signed payload located; local authority remains the exact payload record | exact opaque `0.2.1-R1215.1`; QualifiedOnly; mutable launcher is not the execution target | record only; future range work deferred |
| 27 | Mistral Vibe headless (`mistral-vibe.headless`) | `vibe` not on `PATH` | GitHub [`v2.24.5`](https://github.com/mistralai/mistral-vibe/releases/tag/v2.24.5) and PyPI [`2.24.5`](https://pypi.org/project/mistral-vibe/), published 2026-08-27 | exact `2.24.2`; QualifiedOnly | record only; future range work deferred |
| 28 | Kimi Code installed harness (`kimi-code.acp`; `kimi-code.headless`) | `kimi` resolves; `--version` `0.34.0` | npm [`@moonshot-ai/kimi-code` `0.39.1`](https://registry.npmjs.org/@moonshot-ai%2Fkimi-code/latest), published 2026-08-28; GitHub [`@0.39.1`](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai/kimi-code%400.39.1) | ACP exact `0.28.1` plus `0.29.0..=0.38.0`; headless `0.29.0..=0.37.2` v1 plus exact `0.38.0` v2; AllowUnverified | material candidate |
| 29 | Kimi Code local server (`kimi-code.local-server`) | `kimi` resolves; `--version` `0.34.0` | same npm [`@moonshot-ai/kimi-code` `0.39.1`](https://registry.npmjs.org/@moonshot-ai/kimi-code/latest) | exact `0.28.1` plus `0.29.0..=0.38.0`; AllowUnverified | visible unverified-newer |
| 30 | Kimi Platform Chat API (`kimi-platform.chat`) | n/a; hosted API | Official [Kimi Chat Completions API](https://platform.kimi.ai/docs/api/chat) remains an unversioned API surface; no replacement dated facade identity | exact `kimi-platform-chat-2026-07-21` facade; QualifiedOnly | unchanged |
| 31 | Oh My Pi RPC (`oh-my-pi.rpc`) | `omp` resolves; `--version` `omp/18.0.11` | npm [`@oh-my-pi/pi-coding-agent` `18.1.0`](https://registry.npmjs.org/@oh-my-pi%2Fpi-coding-agent/latest), published 2026-09-01; major line reset from 17.x | maintained `17.2.9..=17.4.0`; later stable unverified only under ordered semantics; major reset requires identity evidence | record only; future range work deferred |
| 32 | Ollama native attached runtime (`ollama.attached`) | `ollama` resolves; `--version` `ollama version is 0.33.1` | GitHub release [`v0.33.2`](https://github.com/ollama/ollama/releases/tag/v0.33.2), published 2026-08-27 | `0.14.0..=0.32.15`; exclusions `0.32.2` and `0.32.10`; AllowUnverified | visible unverified-newer |
| 33 | Codex app-server and exec (`codex.app-server`; `codex.exec`) | `codex-cli 0.150.1` | npm [`@openai/codex` `0.152.0`](https://registry.npmjs.org/@openai%2Fcodex/latest), published 2026-09-01; GitHub [`rust-v0.152.0`](https://github.com/openai/codex/releases/tag/rust-v0.152.0), published 2026-09-01; alpha `0.153.0` ignored | exec and app-server maintained through `0.152.0`; existing gaps `0.82.0..=0.83.0`, `0.108.0`, `0.109.0`, `0.149.2`, `0.150.2`, and `0.151.1`; AllowUnverified | unchanged |
| 34 | OpenAI Realtime API (`openai.realtime`) | n/a; hosted realtime API | Official [Realtime guide](https://developers.openai.com/api/docs/guides/realtime) and [`gpt-realtime-2.1` model reference](https://developers.openai.com/api/docs/models/gpt-realtime-2.1) show no replacement dated facade identity | exact `openai-realtime-reasoning-2026-08-27` facade; superseded point retained as proof; QualifiedOnly | unchanged |
| 35 | OpenAI Background Responses (`openai.background`) | n/a; hosted API | Official [Background mode guide](https://developers.openai.com/api/docs/guides/background) shows the same retained Responses surface; no replacement dated facade identity | exact `openai-responses-background-2026-08-23-service-tier` facade; QualifiedOnly | unchanged |
| 36 | OpenCode HTTP server (`opencode.http`) | `opencode` resolves; `--version` `1.18.18` | npm [`opencode-ai` `1.18.25`](https://registry.npmjs.org/opencode-ai/latest), published 2026-08-28; GitHub [`v1.18.25`](https://github.com/anomalyco/opencode/releases/tag/v1.18.25) | published qualified segments through `1.18.20`; AllowUnverified | visible unverified-newer |
| 37 | Qoder headless (`qoder.headless`) | `qodercli` not on `PATH` | npm [`@qoder-ai/qodercli` `1.1.40`](https://registry.npmjs.org/@qoder-ai%2Fqodercli/latest), published 2026-09-01 | exact `1.1.25`; QualifiedOnly | record only; future range work deferred |
| 38 | Grok Build ACP (`grok-build.acp`) | `grok` resolves; `--version` `grok 1.0.13 (5e9a58528b76) [stable]` | npm [`@xai-official/grok` latest `1.0.13`](https://registry.npmjs.org/@xai-official/grok/latest), published 2026-08-28; `1.0.14` and `1.0.15` exist off `latest`; `alpha` is `1.0.16` and ignored; ACP registry `1.0.16` is discovery metadata | deprecated `0.2.114..=0.2.117`; maintained `1.0.4..=1.0.5`; later stable unverified; do not flatten registry/alpha identity into npm `latest` | visible unverified-newer |
| 39 | xAI Responses WebSocket API (`xai.responses-websocket`) | n/a; hosted realtime API | Official [xAI WebSocket mode](https://docs.x.ai/developers/advanced-api-usage/websocket-mode) remains the same unversioned Responses WebSocket surface; no replacement dated facade identity | exact `xai-responses-websocket-2026-04-23` facade; QualifiedOnly | unchanged |
| 40 | ZCode app-server (`zcode.app-server`) | exact interpreted payload not on `PATH` | npm [`zcode-app-cli` `3.10.2-18`](https://registry.npmjs.org/zcode-app-cli/latest) is packaging metadata, not the selected runtime axis | exact `zcode.runtime` `0.16.3`; QualifiedOnly; do not flatten npm packaging onto `zcode.cjs` | record only; future range work deferred |

### Shared ACP checkpoint surfaces

These are shared protocol/discovery observations, not additional feature-matrix
families and not new Swallowtail claims.

| Surface | Local observation | Current official point | Swallowtail boundary | Result |
| --- | --- | --- | --- | --- |
| Stable ACP schema | local wrapper evidence only; wire negotiation remains integer `1` | GitHub [schema `v1.21.0`](https://github.com/agentclientprotocol/agent-client-protocol/releases/tag/schema-v1.21.0), published 2026-08-20 | frozen currentness corpus records schema `v1.20.0`; adapter routes select ACP wire v1 and keep schema evidence separate | record only; future range work deferred |
| ACP agent registry | n/a | registry [`version` `1.0.0`](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json), 39 agents; selected entries include Claude `0.70.0`, Cursor `2026.08.11`, Gemini `0.57.0`, and Qwen `0.22.3` | discovery metadata only; it cannot move package, executable, schema, or facade claims | record only; future range work deferred |

## Changed Observations Since Research 267

- Pi RPC official `0.84.4` now matches the qualified ceiling after g05.015;
  classification moves from material candidate to unchanged. Unpublished
  `0.83.1` remains absent. Unpublished `0.84.5` remains absent.
- Command Code npm `latest` moved `1.39.2` → `1.39.3`. Exact `1.15.1` stays
  record-only.
- Qoder CLI npm `latest` moved `1.1.39` → `1.1.40`. Exact `1.1.25` stays
  record-only.
- Grok npm `1.0.14` and `1.0.15` exist off the `latest` tag. Official `latest`
  remains `1.0.13`. Alpha `1.0.16` and registry `1.0.16` stay discovery-only.

All other official points and host observations match Research 267.

## Rank After This Record

1. **Kimi Code installed harness `0.39.1`** — the host is on the qualified ACP
   window at `0.34.0`, the official npm and GitHub stable point is `0.39.1`,
   and the current claim already permits an exact forward attempt. Published
   stable points after the current `0.38.0` ceiling contain `0.39.0` and
   `0.39.1`. The next ordered stable `0.39.2` is not published. Headless v2
   remains exact `0.38.0` on `kimi.headless.stream-json.v2`; later identity
   work must settle whether `0.39.1` extends that pin or leaves it behind.
   Do not flatten this family onto `kimi-code.local-server`.

No second family is selected in this record. OpenCode, Ollama, Antigravity,
Grok, and Kimi local-server remain visible ordered newer candidates for later
one-family runs. Gemini remains deferred by operator decision. Oh My Pi's 18.x
reset, exact/opaque families, the exact llama.cpp build axes, the DeepSeek RC
axis, and the Bedrock SDK pins remain record-only until their named identity or
corpus work is authorized. Host Muse `1.0.1-R2006.1` is a different opaque
payload than exact `0.2.1-R1215.1` and stays record-only.

## Decision

Compile Kimi Code installed harness `0.39.1` next through the Contract 029
Upgrade Workflow, one family only. This checkpoint itself changes no claim,
selection, matrix, fixture, or route. Do not bulk-bump from registry `latest`;
do not contact a provider; do not lift Gemini's deferral. Do not compile the
identity or claim cards in this record. The g05.009 provider-operation
observation decision, card 034 planned/not-ready posture, and 249/518
projection counts are unchanged.

## Repository Evidence

- [Production solution feature matrix](../guides/provider-solution-feature-matrix.csv)
  — 40 current family rows.
- Current adapter claims in `crates/swallowtail-adapter-*/src/selection.rs`
  and the route-specific OpenAI Realtime selection module.
- [Contract 029](../contracts/029-interface-version-qualification-and-compatibility.md)
  and the [version-currentness runbook](../guides/version-currentness-checkpoint.md).
- [Research 127](./127-all-route-version-currentness-checkpoint.md),
  [Research 159](./159-post-harness-expansion-version-currentness-checkpoint.md),
  [Research 263](./263-all-route-version-currentness-checkpoint.md),
  [Research 265](./265-all-route-version-currentness-checkpoint.md),
  [Research 267](./267-all-route-version-currentness-checkpoint.md), and
  [Research 268](./268-pi-rpc-0-84-4-identity.md).

## Sources

### Package and release channels

- [Codex npm metadata](https://registry.npmjs.org/@openai%2Fcodex/latest)
- [Codex GitHub releases](https://github.com/openai/codex/releases)
- [Claude Agent ACP npm metadata](https://registry.npmjs.org/@agentclientprotocol%2Fclaude-agent-acp/latest)
- [Claude Agent ACP releases](https://github.com/agentclientprotocol/claude-agent-acp/releases)
- [Claude Code npm metadata](https://registry.npmjs.org/@anthropic-ai%2Fclaude-code/latest)
- [Claude Code releases](https://github.com/anthropics/claude-code/releases)
- [Gemini CLI npm metadata](https://registry.npmjs.org/@google%2Fgemini-cli/latest)
- [Gemini CLI releases](https://github.com/google-gemini/gemini-cli/releases)
- [Grok npm metadata](https://registry.npmjs.org/@xai-official/grok/latest)
- [Kimi Code npm metadata](https://registry.npmjs.org/@moonshot-ai%2Fkimi-code/latest)
- [Kimi Code releases](https://github.com/MoonshotAI/kimi-code/releases)
- [OpenCode npm metadata](https://registry.npmjs.org/opencode-ai/latest)
- [OpenCode releases](https://github.com/anomalyco/opencode/releases)
- [Pi npm metadata](https://registry.npmjs.org/@earendil-works%2Fpi-coding-agent/latest)
- [Pi GitHub releases](https://github.com/badlogic/pi-mono/releases)
- [Qwen Code npm metadata](https://registry.npmjs.org/@qwen-code%2Fqwen-code/latest)
- [Oh My Pi npm metadata](https://registry.npmjs.org/@oh-my-pi%2Fpi-coding-agent/latest)
- [Command Code npm metadata](https://registry.npmjs.org/command-code/latest)
- [DeepSeek Harness npm metadata](https://registry.npmjs.org/@deepseek-ai%2Fdsh/latest)
- [ZCode packaging npm metadata](https://registry.npmjs.org/zcode-app-cli/latest)
- [Cline npm metadata](https://registry.npmjs.org/cline/latest)
- [Copilot CLI npm metadata](https://registry.npmjs.org/@github%2Fcopilot/latest)
- [Deep Agents ACP npm metadata](https://registry.npmjs.org/deepagents-acp/latest)
- [Qoder CLI npm metadata](https://registry.npmjs.org/@qoder-ai%2Fqodercli/latest)
- [Antigravity CLI releases](https://github.com/google-antigravity/antigravity-cli/releases)
- [Goose releases](https://github.com/aaif-goose/goose/releases)
- [Mistral Vibe releases](https://github.com/mistralai/mistral-vibe/releases)
- [Mistral Vibe PyPI](https://pypi.org/project/mistral-vibe/)
- [Kiro stable manifest](https://prod.download.cli.kiro.dev/stable/latest/manifest.json)
- [Ollama releases](https://github.com/ollama/ollama/releases)
- [llama.cpp releases](https://github.com/ggml-org/llama.cpp/releases)
- [ACP schema releases](https://github.com/agentclientprotocol/agent-client-protocol/releases)
- [ACP agent registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)
- [Bedrock Runtime crates.io metadata](https://crates.io/crates/aws-sdk-bedrockruntime)
- [Bedrock control-plane crates.io metadata](https://crates.io/crates/aws-sdk-bedrock)

### Hosted API channels

- [Anthropic API versioning](https://docs.anthropic.com/en/api/versioning)
- [Alibaba Model Studio Responses](https://www.alibabacloud.com/help/en/model-studio/qwen-api-via-openai-responses)
- [DeepSeek Chat Completions](https://api-docs.deepseek.com/api/create-chat-completion/)
- [Gemini Live API](https://ai.google.dev/gemini-api/docs/live-api)
- [Kimi Chat API](https://platform.kimi.ai/docs/api/chat)
- [OpenAI Realtime guide](https://developers.openai.com/api/docs/guides/realtime)
- [OpenAI Realtime model reference](https://developers.openai.com/api/docs/models/gpt-realtime-2.1)
- [OpenAI Background mode](https://developers.openai.com/api/docs/guides/background)
- [xAI WebSocket mode](https://docs.x.ai/developers/advanced-api-usage/websocket-mode)

### Local and repository evidence

- safe local executable discovery and `--version` observations on 2026-09-01
- current `selection.rs` claim constants and segments
- current [provider-solution feature matrix](../guides/provider-solution-feature-matrix.csv)
- frozen currentness records [127](./127-all-route-version-currentness-checkpoint.md),
  [159](./159-post-harness-expansion-version-currentness-checkpoint.md),
  [263](./263-all-route-version-currentness-checkpoint.md),
  [265](./265-all-route-version-currentness-checkpoint.md),
  [267](./267-all-route-version-currentness-checkpoint.md), and
  [268](./268-pi-rpc-0-84-4-identity.md)
