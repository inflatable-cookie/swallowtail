# 274 All-Route Version Currentness Checkpoint

Status: promoted
Owner: Tom
Date: 2026-09-01

## Question

After Claude Code `2.1.257` was qualified, what are the current official
stable points and safe local observations for every production family, and
which one family should enter the next Contract 029 Upgrade Workflow run?

This checkpoint is research only. It changes no compatibility claim, feature-
matrix row, fixture, route, or runtime behavior. It does not reopen the
recently completed Claude Code family, `kimi-code.acp`, or Gemini's deferred
requalification.

## Method

Compared:

- all 40 rows in the current production feature matrix with the current
  adapter `selection.rs` claims;
- safe local `command -v` and `--version` observations for named
  executables;
- official npm `latest`, GitHub latest stable releases or tags, the Kiro
  stable manifest, crates.io max stable, PyPI, and ACP registry metadata;
- official hosted-API documentation for opaque facade routes.

Observation date is 2026-09-01. Safe local checks resolved `qwen`,
`claude-agent-acp`, `claude`, `pi`, `command-code`, `cursor-agent`, `agy`,
`gemini`, `llama-server`, `muse`, `kimi`, `omp`, `ollama`, `codex`,
`opencode`, and `grok`. Their observations are recorded below. `cline`, `dsh`,
`copilot`, `goose`, `kiro-cli`, `deepagents-acp`, `vibe`, `qodercli`, and
`zcode` were missing from `PATH`. A missing local install is an observation,
not a compatibility gap.

Observed host values were:

- `qwen` `0.21.2`;
- `claude-agent-acp` `0.63.0`;
- `claude` `2.1.257 (Claude Code)`;
- `pi` `0.83.0`;
- `command-code` `1.15.1`;
- `cursor-agent` `2026.08.04-aaa8809`;
- `agy` `1.1.19`;
- `gemini` `0.53.0`;
- `llama-server` build `10450`, commit `ece963f41`;
- `muse` `1.0.1 (1.0.1-R2006.1)`;
- `kimi` `0.34.0`;
- `omp` `18.0.11`;
- `ollama` `0.33.1`;
- `codex-cli` `0.150.1`;
- `opencode` `1.18.18`; and
- `grok` `1.0.13`.

The official package and release probes found npm `latest` movement for
Claude Code, Codex, Gemini CLI, Command Code, Oh My Pi, and OpenCode. GitHub
stable releases corroborate the Claude Code, Codex, and Gemini CLI points.
The Kiro manifest moved independently. Other channels remain as recorded in
Research 271, with the already-landed Claude Agent ACP `0.73.0` and Claude
Code `2.1.257` claim boundaries reflected in the current matrix and claims.

No provider prompt, authentication, catalogue, session, install, update, host
change, or live probe was used. Official artifacts were not downloaded,
executed, or substituted for host tools. Hosted "latest model" values and ACP
registry package versions were not treated as Swallowtail compatibility
claims.

## Compatibility Result

Result vocabulary:

- `unchanged` — the official point remains on the current qualified boundary,
  or the opaque facade has no replacement identity;
- `visible unverified-newer` — a later ordered stable exists and the claim
  already permits an exact forward attempt;
- `record only; future range work deferred` — a newer point exists, but the
  claim is exact/opaque, a major reset needs identity evidence, or an existing
  deferral still holds;
- `material candidate` — enough evidence exists to select one family for a
  dedicated Upgrade Workflow card; this checkpoint does not compile that
  card.

The numbered rows below reproduce the 40 production feature-matrix families
exactly once. The boundary is the current repository claim, not a new claim
made by this record.

Partition: 12 unchanged, 7 visible unverified-newer, 20 record-only, 1
material candidate.

| # | Surface | Local observation | Current official point | Swallowtail boundary | Result |
| ---: | --- | --- | --- | --- | --- |
| 1 | Qwen Code headless (`qwen.headless`) | `qwen` resolves; `--version` `0.21.2` | npm [`@qwen-code/qwen-code` `0.22.3`](https://registry.npmjs.org/@qwen-code%2Fqwen-code/latest), published 2026-08-28 | deprecated `0.19.11..=0.20.1`; deprecated `0.21.0..=0.21.14`; maintained exact `0.21.15` and `0.22.0..=0.22.3`; `0.21.16` gap; AllowUnverified | unchanged |
| 2 | Alibaba Model Studio Conversations and Responses (`alibaba.conversations`) | n/a; hosted API | Official [Responses API documentation](https://www.alibabacloud.com/help/en/model-studio/qwen-api-via-openai-responses), page observed 2026-09-01; no replacement dated facade identity | exact `openai-conversations-responses` facade; QualifiedOnly | unchanged |
| 3 | Amazon Bedrock catalogue and Runtime (`bedrock.catalogue`; `bedrock.runtime`) | n/a; embedded SDK | crates.io [`aws-sdk-bedrockruntime` `1.142.0`](https://crates.io/crates/aws-sdk-bedrockruntime) and [`aws-sdk-bedrock` `1.154.0`](https://crates.io/crates/aws-sdk-bedrock), max stable observed 2026-09-01 | exact SDK/service axes; Cargo pins `aws-sdk-bedrockruntime =1.139.0` and `aws-sdk-bedrock =1.150.0`; exact service facades; QualifiedOnly | record only; future range work deferred |
| 4 | Claude Agent ACP (`claude-agent.acp`) | `claude-agent-acp` resolves; `--version` `0.63.0` | npm [`@agentclientprotocol/claude-agent-acp` `0.73.0`](https://registry.npmjs.org/@agentclientprotocol%2Fclaude-agent-acp/latest), published 2026-09-01; GitHub [`v0.73.0`](https://github.com/agentclientprotocol/claude-agent-acp/releases/tag/v0.73.0) | `0.53.0..=0.73.0` excluding `0.58.0`; AllowUnverified | unchanged |
| 5 | Claude Code headless and response-only (`claude-code.headless`; `claude-code.response-only`) | `claude` resolves; `--version` `2.1.257 (Claude Code)` | npm [`@anthropic-ai/claude-code` `2.1.258`](https://registry.npmjs.org/@anthropic-ai%2Fclaude-code/latest), published 2026-09-01T22:25:07.449Z; GitHub [`v2.1.258`](https://github.com/anthropics/claude-code/releases/tag/v2.1.258), published 2026-09-01T22:33:20Z | headless `2.1.220..=2.1.257`; response-only `2.1.227..=2.1.257`; gaps `2.1.244`, `2.1.249`, and hop-skipped `2.1.253..=2.1.256`; AllowUnverified; watcher stays exact `2.1.251` | visible unverified-newer |
| 6 | Anthropic Managed Agents (`anthropic.managed-agent`) | n/a; hosted API | Official [Claude API versioning and Managed Agents reference](https://platform.claude.com/docs/en/api/versioning); no replacement beta facade identity observed | exact `anthropic.managed-agents-facade`; QualifiedOnly | unchanged |
| 7 | Anthropic Messages (`anthropic.messages`) | n/a; hosted API | Official [API versioning](https://docs.anthropic.com/en/api/versioning) still documents the Messages versioning surface; no replacement dated facade identity | exact `anthropic-2023-06-01` facade; QualifiedOnly | unchanged |
| 8 | Pi coding agent RPC (`pi.rpc`) | `pi` resolves; `--version` `0.83.0` | npm [`@earendil-works/pi-coding-agent` `0.84.4`](https://registry.npmjs.org/@earendil-works%2Fpi-coding-agent/latest), published 2026-08-28; GitHub [`v0.84.4`](https://github.com/badlogic/pi-mono/releases/tag/v0.84.4) | maintained exact published points through `0.84.4`; `0.83.1` gap; AllowUnverified | unchanged |
| 9 | Pi coding agent SDK sidecar (`pi.sdk-sidecar`) | `pi` resolves; `--version` `0.83.0` | same upstream npm package currently `0.84.4`; sidecar has no separate current package channel | exact `0.84.2` sidecar package, exact Node `22.23.2`, sidecar wire, and source-tag axes; QualifiedOnly | record only; future range work deferred |
| 10 | Cline ACP (`cline.acp`) | `cline` not on `PATH` | npm [`cline` `3.0.60`](https://registry.npmjs.org/cline/latest), published 2026-08-26 | exact `3.0.55`; QualifiedOnly | record only; future range work deferred |
| 11 | Cline headless (`cline.headless`) | `cline` not on `PATH` | npm [`cline` `3.0.60`](https://registry.npmjs.org/cline/latest), published 2026-08-26 | exact `3.0.55`; QualifiedOnly | record only; future range work deferred |
| 12 | Command Code headless (`command-code.headless`) | `command-code` resolves; `--version` `1.15.1` | npm [`command-code` `1.40.1`](https://registry.npmjs.org/command-code/latest), published 2026-09-01T21:20:21.503Z | exact `1.15.1`; QualifiedOnly | record only; future range work deferred |
| 13 | Cursor Agent catalogue, ACP, and headless (`cursor-agent.catalogue`; `cursor-agent.acp`; `cursor-agent.headless`) | `cursor-agent` resolves; `--version` `2026.08.04-aaa8809` | ACP registry [Cursor `2026.08.31`](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json), download identity `2026.08.31-4057e58` | exact `2026.07.01-41b2de7`, `2026.07.23-e383d2b`, `2026.08.04-aaa8809`, and `2026.08.11-e8db854`; no inferred gap; AllowUnverified | visible unverified-newer |
| 14 | DeepSeek Harness JSON-RPC (`deepseek-harness.jsonrpc`) | `dsh` not on `PATH` | npm [`@deepseek-ai/dsh` `0.1.1-rc.2`](https://registry.npmjs.org/@deepseek-ai%2Fdsh/latest), published 2026-08-21; current channel remains prerelease | exact runtime-bin `0.1.0rc6`; QualifiedOnly | record only; future range work deferred |
| 15 | DeepSeek Harness Web `/api` (`deepseek-harness.local-server`) | `dsh` not on `PATH` | same npm [`@deepseek-ai/dsh` `0.1.1-rc.2`](https://registry.npmjs.org/@deepseek-ai%2Fdsh/latest); current channel remains prerelease | exact Web `0.1.0-rc.6`; QualifiedOnly; do not flatten onto JSON-RPC | record only; future range work deferred |
| 16 | DeepSeek Open Platform continuation (`deepseek.continuation`) | n/a; hosted API | Official [DeepSeek Chat Completions API](https://api-docs.deepseek.com/api/create-chat-completion/) remains an unversioned OpenAI-compatible endpoint; no replacement dated facade identity | exact `deepseek-openai-chat-2026-07-22` facade; QualifiedOnly | unchanged |
| 17 | GitHub Copilot CLI ACP (`copilot-cli.acp`) | `copilot` not on `PATH` | npm [`@github/copilot` `1.0.82`](https://registry.npmjs.org/@github%2Fcopilot/latest), published 2026-08-29; prerelease `1.0.83-1` ignored | exact `1.0.80`; QualifiedOnly | record only; future range work deferred |
| 18 | Antigravity catalogue and headless (`antigravity.catalogue`; `antigravity.headless`) | `agy` resolves; `--version` `1.1.19` | GitHub release [`1.1.23`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.1.23), published 2026-09-01 | maintained `1.1.9..=1.1.17`; later stable points visible above the ceiling; AllowUnverified | visible unverified-newer |
| 19 | Gemini CLI ACP and headless (`gemini-cli.acp`; `gemini-cli.headless`) | `gemini` resolves; `--version` `0.53.0` | npm [`@google/gemini-cli` `0.58.0`](https://registry.npmjs.org/@google%2Fgemini-cli/latest), published 2026-09-01T20:50:39.295Z; GitHub [`v0.58.0`](https://github.com/google-gemini/gemini-cli/releases/tag/v0.58.0), published 2026-09-01T20:51:17Z; preview/nightly ignored | both axes maintained `0.51.0..=0.56.0`; later stable unverified; Gemini requalification deferred | record only; future range work deferred |
| 20 | Gemini Live API (`gemini.live`) | n/a; hosted realtime API | Official [Gemini Live API](https://ai.google.dev/gemini-api/docs/live-api) overview remains the current Live API page observed 2026-09-01; no replacement dated facade identity | exact `google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent.thinking-output-max-context-compression-2026-08-23` facade; QualifiedOnly | unchanged |
| 21 | Goose ACP (`goose.acp`) | `goose` not on `PATH` | GitHub release [`v1.48.0`](https://github.com/aaif-goose/goose/releases/tag/v1.48.0), published 2026-08-27; repository moved from `block/goose` | exact `1.46.0`; QualifiedOnly | record only; future range work deferred |
| 22 | Kiro ACP (`kiro.acp`) | `kiro-cli` not on `PATH` | official [stable manifest](https://prod.download.cli.kiro.dev/stable/latest/manifest.json) reports `2.21.0` | exact `2.18.1`; QualifiedOnly | record only; future range work deferred |
| 23 | Deep Agents ACP (`deepagents.acp`) | `deepagents-acp` not on `PATH` | npm [`deepagents-acp` `0.1.28`](https://registry.npmjs.org/deepagents-acp/latest), published 2026-08-27; ACP registry `DeepAgents` remains `0.1.7` discovery metadata | exact `0.1.25`; QualifiedOnly; do not bind the stale registry value | record only; future range work deferred |
| 24 | llama.cpp attached server (`llama-cpp.attached`) | `llama-server` resolves; `--version` `0.1.0-dev` build `10450`, commit `ece963f41` | GitHub latest release [`v0.3.0`](https://github.com/ggml-org/llama.cpp/releases/tag/v0.3.0), published 2026-08-25; release tag is not the selected build identity | exact attached build `b9910/f5525f7e7`; QualifiedOnly | record only; future range work deferred |
| 25 | llama.cpp owned server lifecycle (`llama-cpp.owned`) | `llama-server` resolves; `--version` `0.1.0-dev` build `10450`, commit `ece963f41` | same GitHub latest [`v0.3.0`](https://github.com/ggml-org/llama.cpp/releases/tag/v0.3.0), published 2026-08-25; no build-to-claim inference | exact owned build `b10069/178a6c449`; QualifiedOnly | record only; future range work deferred |
| 26 | Muse Code headless (`muse-code.headless`) | `muse` resolves; `--version` `Muse Code 1.0.1 (1.0.1-R2006.1)` | no public package or release channel for the signed payload located; local authority remains the exact payload record | exact opaque `0.2.1-R1215.1`; QualifiedOnly; mutable launcher is not the execution target | record only; future range work deferred |
| 27 | Mistral Vibe headless (`mistral-vibe.headless`) | `vibe` not on `PATH` | GitHub [`v2.24.5`](https://github.com/mistralai/mistral-vibe/releases/tag/v2.24.5) and PyPI [`2.24.5`](https://pypi.org/project/mistral-vibe/), published 2026-08-27 | exact `2.24.2`; QualifiedOnly | record only; future range work deferred |
| 28 | Kimi Code installed harness (`kimi-code.acp`; `kimi-code.headless`) | `kimi` resolves; `--version` `0.34.0` | npm [`@moonshot-ai/kimi-code` `0.39.1`](https://registry.npmjs.org/@moonshot-ai%2Fkimi-code/latest), published 2026-08-28; GitHub [`@0.39.1`](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai/kimi-code%400.39.1) | ACP QualifiedOnly exact `0.28.1` plus `0.29.0..=0.38.0`, with `0.39.0` and `0.39.1` excluded; headless `0.29.0..=0.32.0` v1 plus `0.33.0..=0.39.1` v2; every ACP point above `0.38.0` fails closed | record only; future range work deferred |
| 29 | Kimi Code local server (`kimi-code.local-server`) | `kimi` resolves; `--version` `0.34.0` | same npm [`@moonshot-ai/kimi-code` `0.39.1`](https://registry.npmjs.org/@moonshot-ai%2Fkimi-code/latest), published 2026-08-28 | exact `0.28.1` plus `0.29.0..=0.38.0`; AllowUnverified | visible unverified-newer |
| 30 | Kimi Platform Chat API (`kimi-platform.chat`) | n/a; hosted API | Official [Kimi Chat Completions API](https://platform.kimi.ai/docs/api/chat) remains an unversioned API surface; no replacement dated facade identity | exact `kimi-platform-chat-2026-07-21` facade; QualifiedOnly | unchanged |
| 31 | Oh My Pi RPC (`oh-my-pi.rpc`) | `omp` resolves; `--version` `omp/18.0.11` | npm [`@oh-my-pi/pi-coding-agent` `18.1.2`](https://registry.npmjs.org/@oh-my-pi%2Fpi-coding-agent/latest), published 2026-09-01T20:30:05.424Z; major line reset from qualified 17.x | maintained `17.2.9..=17.4.0`; 18.x requires identity evidence before any claim | record only; future range work deferred |
| 32 | Ollama native attached runtime (`ollama.attached`) | `ollama` resolves; `--version` `0.33.1` | GitHub release [`v0.33.2`](https://github.com/ollama/ollama/releases/tag/v0.33.2), published 2026-08-27 | `0.14.0..=0.32.15`; exclusions `0.32.2` and `0.32.10`; AllowUnverified | visible unverified-newer |
| 33 | Codex app-server and exec (`codex.app-server`; `codex.exec`) | `codex` resolves; `--version` `codex-cli 0.150.1` | npm [`@openai/codex` `0.152.1`](https://registry.npmjs.org/@openai%2Fcodex/latest), published 2026-09-01T22:36:50.784Z; GitHub [`rust-v0.152.1`](https://github.com/openai/codex/releases/tag/rust-v0.152.1), published 2026-09-01T22:33:02Z; alpha `0.153.0-alpha.4` ignored | exec and app-server maintained through `0.152.0`; gaps `0.82.0..=0.83.0`, `0.108.0`, `0.109.0`, `0.149.2`, `0.150.2`, and `0.151.1`; AllowUnverified | material candidate |
| 34 | OpenAI Realtime API (`openai.realtime`) | n/a; hosted realtime API | Official [Realtime guide](https://developers.openai.com/api/docs/guides/realtime) and [`gpt-realtime-2.1` model reference](https://developers.openai.com/api/docs/models/gpt-realtime-2.1) show no replacement dated facade identity | exact `openai-realtime-reasoning-2026-08-27` facade; superseded point retained as proof; QualifiedOnly | unchanged |
| 35 | OpenAI Background Responses (`openai.background`) | n/a; hosted API | Official [Background mode guide](https://developers.openai.com/api/docs/guides/background) shows the same retained Responses surface; no replacement dated facade identity | exact `openai-responses-background-2026-08-23-service-tier` facade; QualifiedOnly | unchanged |
| 36 | OpenCode HTTP server (`opencode.http`) | `opencode` resolves; `--version` `1.18.18` | npm [`opencode-ai` `1.18.26`](https://registry.npmjs.org/opencode-ai/latest), published 2026-09-01T21:51:32.962Z; GitHub [`v1.18.26`](https://github.com/anomalyco/opencode/releases/tag/v1.18.26), published 2026-09-01 | published qualified segments through `1.18.20`; AllowUnverified | visible unverified-newer |
| 37 | Qoder headless (`qoder.headless`) | `qodercli` not on `PATH` | npm [`@qoder-ai/qodercli` `1.1.40`](https://registry.npmjs.org/@qoder-ai/qodercli/latest), published 2026-09-01 | exact `1.1.25`; QualifiedOnly | record only; future range work deferred |
| 38 | Grok Build ACP (`grok-build.acp`) | `grok` resolves; `--version` `grok 1.0.13 (5e9a58528b76) [stable]` | npm [`@xai-official/grok` `1.0.13`](https://registry.npmjs.org/@xai-official/grok/latest), published 2026-08-28; published `1.0.14`, `1.0.15`, and `1.0.16` exist off `latest`; alpha `1.0.17` and ACP registry `1.0.17` are ignored | deprecated `0.2.114..=0.2.117`; maintained `1.0.4..=1.0.5`; later stable unverified; do not flatten registry or alpha identity onto npm `latest` | visible unverified-newer |
| 39 | xAI Responses WebSocket API (`xai.responses-websocket`) | n/a; hosted realtime API | Official [xAI WebSocket mode](https://docs.x.ai/developers/advanced-api-usage/websocket-mode) remains the same unversioned Responses WebSocket surface; no replacement dated facade identity | exact `xai-responses-websocket-2026-04-23` facade; QualifiedOnly | unchanged |
| 40 | ZCode app-server (`zcode.app-server`) | exact interpreted payload not on `PATH` | npm [`zcode-app-cli` `3.10.2-18`](https://registry.npmjs.org/zcode-app-cli/latest), published 2026-08-31; packaging metadata is not the selected runtime axis | exact `zcode.runtime` `0.16.3`; QualifiedOnly; do not flatten npm packaging onto `zcode.cjs` | record only; future range work deferred |

### Shared ACP checkpoint surfaces

These are shared protocol/discovery observations, not additional feature-matrix
families and not new Swallowtail claims.

| Surface | Local observation | Current official point | Swallowtail boundary | Result |
| --- | --- | --- | --- | --- |
| Stable ACP schema | local wrapper evidence only; wire negotiation remains integer `1` | GitHub [schema `v1.21.0`](https://github.com/agentclientprotocol/agent-client-protocol/releases/tag/schema-v1.21.0), published 2026-08-20 | frozen currentness corpus records schema `v1.20.0`; adapter routes select ACP wire v1 and keep schema evidence separate | record only; future range work deferred |
| ACP agent registry | n/a | registry [`version` `1.0.0`](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json), 39 agents; selected entries include Claude `0.73.0`, Cursor `2026.08.31`, Gemini `0.58.0`, and Qwen `0.22.3` | discovery metadata only; it cannot move package, executable, schema, or facade claims | record only; future range work deferred |

## Changed Observations Since Research 271 And 273

- Claude Code npm and GitHub moved `2.1.257` → `2.1.258`. The host remains
  qualified `2.1.257`. This is a visible newer point after the just-completed
  g05.019 family; it is not reopened here. Watcher remains exact `2.1.251`.
- Codex npm and GitHub moved `0.152.0` → `0.152.1`. The host remains inside
  the qualified range at `0.150.1`. This is the sole material candidate.
  Published `0.152.1` is the only stable hop after the current ceiling;
  `0.153.0-alpha.4` is not stable evidence.
- Gemini CLI npm and GitHub moved `0.57.0` → `0.58.0`. The later stable is
  recorded, but Gemini requalification remains deferred by operator decision.
- Command Code npm moved `1.39.3` → `1.40.1`, with published `1.40.0` and
  `1.40.1` after the observed point. Its exact qualified-only claim stays
  record-only.
- Oh My Pi npm moved `18.1.1` → `18.1.2`. The 18.x major reset still needs
  identity evidence and does not inherit the qualified 17.x segment.
- OpenCode npm and GitHub moved `1.18.25` → `1.18.26`; its host remains
  qualified at `1.18.18`, so the point stays a later visible candidate.
- The Kiro stable manifest moved `2.20.2` → `2.21.0`; the exact `2.18.1`
  qualified-only claim remains unchanged.
- The ACP registry now advertises Claude `0.73.0` and Gemini `0.58.0`; it
  remains discovery metadata and does not replace the package channels or
  move any claim.

All other official points and safe host observations remain as recorded in
Research 271, subject to the current Claude Agent ACP `0.73.0` and Claude
Code `2.1.257` claim boundaries already landed before this checkpoint.

## Rank After This Record

1. **Codex `0.152.1`** — npm and GitHub agree on the current stable point;
   the host is already inside the qualified `0.152.0` range at `0.150.1`;
   both `codex.exec` and `codex.app-server` share the ordered `codex.cli`
   family; and no deferral, exact-pin posture, or family split blocks an
   identity run. This is the only family selected by this checkpoint.

Claude Code `2.1.258` remains visible unverified-newer, but g05.019 just
closed the Claude Code family and the currentness lane does not reopen a
closed family in this checkpoint. Cursor `2026.08.31-4057e58`, Antigravity
`1.1.23`, Kimi local server `0.39.1`, Ollama `0.33.2`, OpenCode `1.18.26`,
and Grok Build npm `1.0.13` remain later ordered candidates for subsequent
one-family runs. Gemini stays deferred. `kimi-code.acp` stays QualifiedOnly
at `0.38.0` and is not reopened. Oh My Pi's 18.x reset, exact/opaque
families, exact llama.cpp build axes, the DeepSeek RC axis, and the Bedrock
SDK pins remain record-only until their named identity or corpus work is
authorized. Muse `1.0.1-R2006.1` is a different opaque payload from exact
`0.2.1-R1215.1` and stays record-only.

## Decision

Compile Codex `0.152.1` next through the Contract 029 Upgrade Workflow, one
family only. This checkpoint itself changes no claim, selection, matrix,
fixture, or route. Do not bulk-bump from registry `latest`; do not contact a
provider; do not reopen Claude Code, `kimi-code.acp`, or Gemini's deferral;
and do not compile the Codex identity or claim cards in this record. The
g05.009 provider-operation observation decision, card 034 planned/not-ready
posture, and 249/518 projection counts are unchanged.

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
  [Research 267](./267-all-route-version-currentness-checkpoint.md),
  [Research 269](./269-all-route-version-currentness-checkpoint.md),
  [Research 271](./271-all-route-version-currentness-checkpoint.md), and
  [Research 273](./273-claude-code-2-1-257-identity.md).

## Sources

### Package and release channels

- [Codex npm metadata](https://registry.npmjs.org/@openai%2Fcodex/latest)
- [Codex `rust-v0.152.1` release](https://github.com/openai/codex/releases/tag/rust-v0.152.1)
- [Claude Code npm metadata](https://registry.npmjs.org/@anthropic-ai%2Fclaude-code/latest)
- [Claude Code `v2.1.258` release](https://github.com/anthropics/claude-code/releases/tag/v2.1.258)
- [Claude Agent ACP npm metadata](https://registry.npmjs.org/@agentclientprotocol%2Fclaude-agent-acp/latest)
- [Claude Agent ACP releases](https://github.com/agentclientprotocol/claude-agent-acp/releases)
- [Gemini CLI npm metadata](https://registry.npmjs.org/@google%2Fgemini-cli/latest)
- [Gemini CLI `v0.58.0` release](https://github.com/google-gemini/gemini-cli/releases/tag/v0.58.0)
- [Grok npm metadata](https://registry.npmjs.org/@xai-official/grok/latest)
- [Kimi Code npm metadata](https://registry.npmjs.org/@moonshot-ai%2Fkimi-code/latest)
- [Kimi Code releases](https://github.com/MoonshotAI/kimi-code/releases)
- [OpenCode npm metadata](https://registry.npmjs.org/opencode-ai/latest)
- [OpenCode `v1.18.26` release](https://github.com/anomalyco/opencode/releases/tag/v1.18.26)
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
- [Anthropic Managed Agents reference](https://platform.claude.com/docs/en/api/versioning)
- [Alibaba Model Studio Responses](https://www.alibabacloud.com/help/en/model-studio/qwen-api-via-openai-responses)
- [DeepSeek Chat Completions](https://api-docs.deepseek.com/api/create-chat-completion/)
- [Gemini Live API](https://ai.google.dev/gemini-api/docs/live-api)
- [Kimi Chat API](https://platform.kimi.ai/docs/api/chat)
- [OpenAI Realtime guide](https://developers.openai.com/api/docs/guides/realtime)
- [OpenAI Realtime model reference](https://developers.openai.com/api/docs/models/gpt-realtime-2.1)
- [OpenAI Background mode](https://developers.openai.com/api/docs/guides/background)
- [xAI WebSocket mode](https://docs.x.ai/developers/advanced-api-usage/websocket-mode)

### Local and repository evidence

- safe local executable discovery and `--version` observations on 2026-09-01;
- current `selection.rs` claim constants and segments;
- current [provider-solution feature matrix](../guides/provider-solution-feature-matrix.csv);
- frozen currentness records [127](./127-all-route-version-currentness-checkpoint.md),
  [159](./159-post-harness-expansion-version-currentness-checkpoint.md),
  [263](./263-all-route-version-currentness-checkpoint.md),
  [265](./265-all-route-version-currentness-checkpoint.md),
  [267](./267-all-route-version-currentness-checkpoint.md),
  [269](./269-all-route-version-currentness-checkpoint.md),
  [271](./271-all-route-version-currentness-checkpoint.md), and
  [273](./273-claude-code-2-1-257-identity.md).
