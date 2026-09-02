# 276 All-Route Version Currentness Checkpoint

Status: promoted
Owner: Tom
Date: 2026-09-02

## Question

After Codex `0.152.1` was qualified, what are the current official stable
points and safe local observations for every production family, and which one
family should enter the next Contract 029 Upgrade Workflow run?

This checkpoint is research only. It changes no compatibility claim, feature-
matrix row, fixture, route, or runtime behavior. It does not reopen the
just-completed Codex family, `kimi-code.acp`, or Gemini's deferred
requalification.

## Method

Compared:

- all 40 rows in the current production feature matrix with current adapter
  `selection.rs` claims;
- safe local `command -v` and promptless `--version` observations for named
  executables already installed;
- fresh official npm `latest`, GitHub latest stable releases or tags, the Kiro
  stable manifest, crates.io max stable, PyPI, and ACP registry metadata; and
- official hosted-API documentation for opaque facade routes.

The observation date is 2026-09-02. Official package, release, registry, and
hosted-documentation channels were re-probed during this checkpoint. Safe
local checks resolved `qwen`, `claude-agent-acp`, `claude`, `pi`,
`command-code`, `cursor-agent`, `agy`, `gemini`, `llama-server`, `muse`,
`kimi`, `omp`, `ollama`, `codex`, `opencode`, and `grok`. Their observations
are recorded below. `cline`, `dsh`, `copilot`, `goose`, `kiro-cli`,
`deepagents-acp`, `vibe`, `qodercli`, and `zcode` were missing from `PATH`. A
missing local install is an observation, not a compatibility gap.

Observed host values were:

- `qwen` `0.21.2`;
- `claude-agent-acp` `0.63.0`;
- `claude` `2.1.258 (Claude Code)`;
- `pi` `0.83.0`;
- `command-code` `1.15.1`;
- `cursor-agent` `2026.08.04-aaa8809`;
- `agy` `1.1.19`;
- `gemini` `0.53.0`;
- `llama-server` `0.1.0-dev`, build `10450`, commit `ece963f41`;
- `muse` `1.0.1 (1.0.1-R2006.1)`;
- `kimi` `0.34.0`;
- `omp` `18.0.11`;
- `ollama` `0.33.1`;
- `codex-cli` `0.150.1`;
- `opencode` `1.18.18`; and
- `grok` `1.0.13 (5e9a58528b76) [stable]`.

No provider prompt, authentication, catalogue, session, install, update,
host change, or live probe was used. Official artifacts were not installed,
executed, or substituted for host tools. Hosted latest-model values and ACP
registry package versions are discovery evidence, not Swallowtail
compatibility claims.

## Compatibility Result

Result vocabulary:

- `unchanged` — the official point remains on the current qualified boundary,
  or the opaque facade has no replacement identity;
- `visible unverified-newer` — a later ordered stable exists and the claim
  already permits an exact forward attempt;
- `record only; future range work deferred` — a newer point exists, but the
  claim is exact/opaque, a major reset needs identity evidence, or an existing
  deferral or stop still holds; and
- `material candidate` — enough evidence exists to select one family for a
  dedicated Upgrade Workflow card; this checkpoint does not compile that
  card.

The numbered rows below reproduce the 40 production feature-matrix families
exactly once. The boundary is the current repository claim, not a new claim
made by this record.

Partition: 13 unchanged, 6 visible unverified-newer, 20 record-only, and 1
material candidate.

| # | Surface | Local observation | Current official point | Swallowtail boundary | Result |
| ---: | --- | --- | --- | --- | --- |
| 1 | Qwen Code headless (`qwen.headless`) | `qwen`; `--version` `0.21.2` | npm [`@qwen-code/qwen-code` `0.22.3`](https://registry.npmjs.org/@qwen-code%2Fqwen-code/latest), published 2026-08-28 | deprecated `0.19.11..=0.20.1`; deprecated `0.21.0..=0.21.14`; maintained exact `0.21.15` and `0.22.0..=0.22.3`; `0.21.16` gap; AllowUnverified | unchanged |
| 2 | Alibaba Model Studio Conversations and Responses (`alibaba.conversations`) | hosted API | Official [Responses API documentation](https://www.alibabacloud.com/help/en/model-studio/qwen-api-via-openai-responses), observed 2026-09-02; no replacement dated facade identity | exact `openai-conversations-responses` facade; QualifiedOnly | unchanged |
| 3 | Amazon Bedrock catalogue and Runtime (`bedrock.catalogue`; `bedrock.runtime`) | embedded SDK | crates.io [`aws-sdk-bedrockruntime` `1.142.0`](https://crates.io/crates/aws-sdk-bedrockruntime) and [`aws-sdk-bedrock` `1.154.0`](https://crates.io/crates/aws-sdk-bedrock), max stable observed 2026-09-02 | exact SDK/service axes; Cargo pins `aws-sdk-bedrockruntime =1.139.0` and `aws-sdk-bedrock =1.150.0`; exact service facades; QualifiedOnly | record only; future range work deferred |
| 4 | Claude Agent ACP (`claude-agent.acp`) | `claude-agent-acp`; `--version` `0.63.0` | npm [`@agentclientprotocol/claude-agent-acp` `0.73.0`](https://registry.npmjs.org/@agentclientprotocol%2Fclaude-agent-acp/latest), published 2026-09-01; GitHub [`v0.73.0`](https://github.com/agentclientprotocol/claude-agent-acp/releases/tag/v0.73.0) | `0.53.0..=0.73.0` excluding `0.58.0`; AllowUnverified | unchanged |
| 5 | Claude Code headless and response-only (`claude-code.headless`; `claude-code.response-only`) | `claude`; `--version` `2.1.258 (Claude Code)` | npm [`@anthropic-ai/claude-code` `2.1.258`](https://registry.npmjs.org/@anthropic-ai%2Fclaude-code/latest), published 2026-09-01T22:25:07.449Z; GitHub [`v2.1.258`](https://github.com/anthropics/claude-code/releases/tag/v2.1.258) | headless `2.1.220..=2.1.257`; response-only `2.1.227..=2.1.257`; gaps `2.1.244`, `2.1.249`, and hop-skipped `2.1.253..=2.1.256`; AllowUnverified; watcher stays exact `2.1.251` | visible unverified-newer |
| 6 | Anthropic Managed Agents (`anthropic.managed-agent`) | hosted API | Official [Claude API versioning and Managed Agents reference](https://platform.claude.com/docs/en/api/versioning), observed 2026-09-02; no replacement beta facade identity | exact `anthropic-managed-agents-facade`; QualifiedOnly | unchanged |
| 7 | Anthropic Messages (`anthropic.messages`) | hosted API | Official [API versioning](https://docs.anthropic.com/en/api/versioning), observed 2026-09-02; no replacement dated facade identity | exact `anthropic-2023-06-01` facade; QualifiedOnly | unchanged |
| 8 | Pi coding agent RPC (`pi.rpc`) | `pi`; `--version` `0.83.0` | npm [`@earendil-works/pi-coding-agent` `0.84.4`](https://registry.npmjs.org/@earendil-works%2Fpi-coding-agent/latest), published 2026-08-28; GitHub [`v0.84.4`](https://github.com/badlogic/pi-mono/releases/tag/v0.84.4) | maintained exact published points through `0.84.4`; `0.83.1` gap; AllowUnverified | unchanged |
| 9 | Pi coding agent SDK sidecar (`pi.sdk-sidecar`) | `pi`; `--version` `0.83.0` | same upstream npm package `0.84.4`; sidecar has no separate current package channel | exact `0.84.2` sidecar package, exact Node `22.23.2`, sidecar wire, and source-tag axes; QualifiedOnly | record only; future range work deferred |
| 10 | Cline ACP (`cline.acp`) | `cline` missing from `PATH` | npm [`cline` `3.0.61`](https://registry.npmjs.org/cline/latest), published 2026-09-02; nightly ignored | exact `3.0.55`; QualifiedOnly | record only; future range work deferred |
| 11 | Cline headless (`cline.headless`) | `cline` missing from `PATH` | same npm package `3.0.61`; nightly ignored | exact `3.0.55`; QualifiedOnly | record only; future range work deferred |
| 12 | Command Code headless (`command-code.headless`) | `command-code`; `--version` `1.15.1` | npm [`command-code` `1.40.1`](https://registry.npmjs.org/command-code/latest), published 2026-09-01T21:20:21.503Z; alpha/beta/rc tags ignored | exact `1.15.1`; QualifiedOnly | record only; future range work deferred |
| 13 | Cursor Agent catalogue, ACP, and headless (`cursor-agent.catalogue`; `cursor-agent.acp`; `cursor-agent.headless`) | `cursor-agent`; `--version` `2026.08.04-aaa8809` | ACP registry [Cursor `2026.08.31`](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json), binary identity `2026.08.31-4057e58` | exact `2026.07.01-41b2de7`, `2026.07.23-e383d2b`, `2026.08.04-aaa8809`, and `2026.08.11-e8db854`; no inferred gap; AllowUnverified | visible unverified-newer |
| 14 | DeepSeek Harness JSON-RPC (`deepseek-harness.jsonrpc`) | `dsh` missing from `PATH` | npm [`@deepseek-ai/dsh` `0.1.1-rc.2`](https://registry.npmjs.org/@deepseek-ai%2Fdsh/latest), published 2026-08-21; current channel remains prerelease | exact runtime-bin `0.1.0rc6`; QualifiedOnly | record only; future range work deferred |
| 15 | DeepSeek Harness Web `/api` (`deepseek-harness.local-server`) | `dsh` missing from `PATH` | same npm package `0.1.1-rc.2`; current channel remains prerelease | exact Web `0.1.0-rc.6`; QualifiedOnly; do not flatten onto JSON-RPC | record only; future range work deferred |
| 16 | DeepSeek Open Platform continuation (`deepseek.continuation`) | hosted API | Official [DeepSeek Chat Completions API](https://api-docs.deepseek.com/api/create-chat-completion/) remains an unversioned OpenAI-compatible endpoint; no replacement dated facade identity | exact `deepseek-openai-chat-2026-07-22` facade; QualifiedOnly | unchanged |
| 17 | GitHub Copilot CLI ACP (`copilot-cli.acp`) | `copilot` missing from `PATH` | npm [`@github/copilot` `1.0.82`](https://registry.npmjs.org/@github%2Fcopilot/latest), published 2026-08-29; prerelease `1.0.83-2` ignored | exact `1.0.80`; QualifiedOnly | record only; future range work deferred |
| 18 | Antigravity catalogue and headless (`antigravity.catalogue`; `antigravity.headless`) | `agy`; `--version` `1.1.19` | GitHub release [`1.1.24`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.1.24), published 2026-09-02 | maintained `1.1.9..=1.1.17`; later stable points visible above the ceiling; AllowUnverified | visible unverified-newer |
| 19 | Gemini CLI ACP and headless (`gemini-cli.acp`; `gemini-cli.headless`) | `gemini`; `--version` `0.53.0` | npm [`@google/gemini-cli` `0.58.0`](https://registry.npmjs.org/@google%2Fgemini-cli/latest), published 2026-09-01; GitHub [`v0.58.0`](https://github.com/google-gemini/gemini-cli/releases/tag/v0.58.0); preview/nightly ignored | both axes maintained `0.51.0..=0.56.0`; later stable unverified; Gemini requalification deferred | record only; future range work deferred |
| 20 | Gemini Live API (`gemini.live`) | hosted realtime API | Official [Gemini Live API](https://ai.google.dev/gemini-api/docs/live-api), observed 2026-09-02; no replacement dated facade identity | exact `google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent.thinking-output-max-context-compression-2026-08-23` facade; QualifiedOnly | unchanged |
| 21 | Goose ACP (`goose.acp`) | `goose` missing from `PATH` | GitHub release [`v1.48.0`](https://github.com/aaif-goose/goose/releases/tag/v1.48.0), published 2026-08-27 | exact `1.46.0`; QualifiedOnly | record only; future range work deferred |
| 22 | Kiro ACP (`kiro.acp`) | `kiro-cli` missing from `PATH` | official [stable manifest](https://prod.download.cli.kiro.dev/stable/latest/manifest.json) reports `2.21.0` | exact `2.18.1`; QualifiedOnly | record only; future range work deferred |
| 23 | Deep Agents ACP (`deepagents.acp`) | `deepagents-acp` missing from `PATH` | npm [`deepagents-acp` `0.1.28`](https://registry.npmjs.org/deepagents-acp/latest), published 2026-08-27; ACP registry `DeepAgents` remains `0.1.7` discovery metadata | exact `0.1.25`; QualifiedOnly; do not bind the stale registry value | record only; future range work deferred |
| 24 | llama.cpp attached server (`llama-cpp.attached`) | `llama-server`; `0.1.0-dev`, build `10450`, commit `ece963f41` | GitHub [`v0.3.0`](https://github.com/ggml-org/llama.cpp/releases/tag/v0.3.0), published 2026-08-25; release tag is not selected build identity | exact attached build `b9910/f5525f7e7`; QualifiedOnly | record only; future range work deferred |
| 25 | llama.cpp owned server lifecycle (`llama-cpp.owned`) | same `llama-server` build `10450`, commit `ece963f41` | same GitHub latest `v0.3.0`; no build-to-claim inference | exact owned build `b10069/178a6c449`; QualifiedOnly | record only; future range work deferred |
| 26 | Muse Code headless (`muse-code.headless`) | `muse`; `--version` `Muse Code 1.0.1 (1.0.1-R2006.1)` | no public package or release channel for the signed payload located; local authority remains the exact payload record | exact opaque `0.2.1-R1215.1`; QualifiedOnly; mutable launcher is not the execution target | record only; future range work deferred |
| 27 | Mistral Vibe headless (`mistral-vibe.headless`) | `vibe` missing from `PATH` | GitHub [`v2.24.5`](https://github.com/mistralai/mistral-vibe/releases/tag/v2.24.5) and PyPI [`2.24.5`](https://pypi.org/project/mistral-vibe/), published 2026-08-27 | exact `2.24.2`; QualifiedOnly | record only; future range work deferred |
| 28 | Kimi Code installed harness (`kimi-code.acp`; `kimi-code.headless`) | `kimi`; `--version` `0.34.0` | npm [`@moonshot-ai/kimi-code` `0.40.1`](https://registry.npmjs.org/@moonshot-ai%2Fkimi-code/latest), published 2026-09-02T09:20:41.955Z; GitHub [`@0.40.1`](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.40.1) | ACP QualifiedOnly exact `0.28.1` plus `0.29.0..=0.38.0`, with `0.39.0` and `0.39.1` excluded; every ACP point above `0.38.0` fails closed; headless v1 `0.29.0..=0.32.0`, v2 `0.33.0..=0.39.1` | record only; future range work deferred |
| 29 | Kimi Code local server (`kimi-code.local-server`) | `kimi`; `--version` `0.34.0` | same npm package `0.40.1`, published 2026-09-02T09:20:41.955Z; GitHub tag `@0.40.1` | exact `0.28.1` plus `0.29.0..=0.38.0`; AllowUnverified | material candidate |
| 30 | Kimi Platform Chat API (`kimi-platform.chat`) | hosted API | Official [Kimi Chat Completions API](https://platform.kimi.ai/docs/api/chat), observed 2026-09-02; no replacement dated facade identity | exact `kimi-platform-chat-2026-07-21` facade; QualifiedOnly | unchanged |
| 31 | Oh My Pi RPC (`oh-my-pi.rpc`) | `omp`; `--version` `omp/18.0.11` | npm [`@oh-my-pi/pi-coding-agent` `18.1.2`](https://registry.npmjs.org/@oh-my-pi%2Fpi-coding-agent/latest), published 2026-09-01T20:30:05.424Z; major line reset from qualified 17.x | maintained `17.2.9..=17.4.0`; 18.x requires identity evidence before any claim | record only; future range work deferred |
| 32 | Ollama native attached runtime (`ollama.attached`) | `ollama`; `--version` `0.33.1` | GitHub release [`v0.33.2`](https://github.com/ollama/ollama/releases/tag/v0.33.2), published 2026-08-27; `0.33.3-rc0` ignored | `0.14.0..=0.32.15`; exclusions `0.32.2` and `0.32.10`; AllowUnverified | visible unverified-newer |
| 33 | Codex app-server and exec (`codex.app-server`; `codex.exec`) | `codex`; `--version` `codex-cli 0.150.1` | npm [`@openai/codex` `0.152.1`](https://registry.npmjs.org/@openai%2Fcodex/latest), published 2026-09-01T22:36:50.784Z; GitHub [`rust-v0.152.1`](https://github.com/openai/codex/releases/tag/rust-v0.152.1); alpha `0.153.0-alpha.5` ignored | exec and app-server maintained through `0.152.1`; gaps `0.82.0..=0.83.0`, `0.108.0`, `0.109.0`, `0.149.2`, `0.150.2`, and `0.151.1`; AllowUnverified | unchanged |
| 34 | OpenAI Realtime API (`openai.realtime`) | hosted realtime API | Official [Realtime guide](https://developers.openai.com/api/docs/guides/realtime) and [`gpt-realtime-2.1` model reference](https://developers.openai.com/api/docs/models/gpt-realtime-2.1), observed 2026-09-02; no replacement dated facade identity | exact `openai-realtime-reasoning-2026-08-27` facade; superseded point retained as proof; QualifiedOnly | unchanged |
| 35 | OpenAI Background Responses (`openai.background`) | hosted API | Official [Background mode guide](https://developers.openai.com/api/docs/guides/background), observed 2026-09-02; no replacement dated facade identity | exact `openai-responses-background-2026-08-23-service-tier` facade; QualifiedOnly | unchanged |
| 36 | OpenCode HTTP server (`opencode.http`) | `opencode`; `--version` `1.18.18` | npm [`opencode-ai` `1.18.26`](https://registry.npmjs.org/opencode-ai/latest), published 2026-09-01T21:51:32.962Z; GitHub [`v1.18.26`](https://github.com/anomalyco/opencode/releases/tag/v1.18.26) | published qualified segments through `1.18.20`; AllowUnverified | visible unverified-newer |
| 37 | Qoder headless (`qoder.headless`) | `qodercli` missing from `PATH` | npm [`@qoder-ai/qodercli` `1.1.41`](https://registry.npmjs.org/@qoder-ai%2Fqodercli/latest), published 2026-09-02T09:33:59.682Z | exact `1.1.25`; QualifiedOnly | record only; future range work deferred |
| 38 | Grok Build ACP (`grok-build.acp`) | `grok`; `--version` `1.0.13 (5e9a58528b76) [stable]` | npm [`@xai-official/grok` `1.0.13`](https://registry.npmjs.org/@xai-official%2Fgrok/latest); stable `1.0.14..=1.0.16` exist off `latest`; alpha `1.0.17` ignored | deprecated `0.2.114..=0.2.117`; maintained `1.0.4..=1.0.5`; later stable unverified; do not flatten npm latest, alpha, or ACP registry identity | visible unverified-newer |
| 39 | xAI Responses WebSocket API (`xai.responses-websocket`) | hosted realtime API | Official [xAI WebSocket mode](https://docs.x.ai/developers/advanced-api-usage/websocket-mode), observed 2026-09-02; no replacement dated facade identity | exact `xai-responses-websocket-2026-04-23` facade; QualifiedOnly | unchanged |
| 40 | ZCode app-server (`zcode.app-server`) | `zcode` missing from `PATH` | npm [`zcode-app-cli` `3.10.2-19`](https://registry.npmjs.org/zcode-app-cli/latest), published 2026-09-02T10:14:10.329Z; packaging metadata is not the runtime axis | exact `zcode.runtime` `0.16.3`; QualifiedOnly; do not flatten npm packaging onto `zcode.cjs` | record only; future range work deferred |

### Shared ACP checkpoint surfaces

These are shared protocol/discovery observations, not additional feature-matrix
families and not new Swallowtail claims.

| Surface | Current official point | Boundary | Result |
| --- | --- | --- | --- |
| Stable ACP schema | GitHub [schema `v1.21.0`](https://github.com/agentclientprotocol/agent-client-protocol/releases/tag/schema-v1.21.0), published 2026-08-20 | frozen currentness corpus records schema `v1.20.0`; adapter routes select ACP wire v1 and keep schema evidence separate | record only; future range work deferred |
| ACP agent registry | registry [`1.0.0`](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json), 39 agents; selected entries include Claude `0.73.0`, Cursor `2026.08.31`, Gemini `0.58.0`, and Qwen `0.22.3` | discovery metadata only; it cannot move package, executable, schema, or facade claims | record only; future range work deferred |

## Changed Observations Since Research 274 And Merged Codex 0.152.1

- Codex npm and GitHub remain at `0.152.1`, now equal to the qualified
  boundary. The host is still `0.150.1` inside the qualified range. The
  just-closed Codex family is unchanged and is not reopened. Alpha
  `0.153.0-alpha.5` is not stable evidence.
- Kimi Code npm and GitHub moved `0.39.1` → `0.40.1`, with stable `0.40.0`
  and `0.40.1` published on 2026-09-02. Kimi ACP remains capped at
  QualifiedOnly `0.38.0` under the A2 gate; the installed headless and local
  server routes remain separate. This checkpoint selects only the local
  server family for a future identity run.
- Claude Code npm, GitHub, and the already-installed host moved or now report
  `2.1.258`; the qualified boundary remains `2.1.257`. It is visible newer,
  but the just-closed Claude Code family is not reopened. Watcher remains
  exact `2.1.251`.
- Antigravity moved from the previously observed `1.1.23` to `1.1.24`.
  Cursor's ACP registry moved to `2026.08.31` with binary identity
  `2026.08.31-4057e58`; Ollama moved to `0.33.2`; OpenCode moved to
  `1.18.26`; and Grok's stable npm channel now has `1.0.13` as `latest` with
  off-latest stable `1.0.14..=1.0.16`. These remain visible unverified-newer
  observations, not claims.
- Qoder moved to `1.1.41`; Cline moved to `3.0.61`; Copilot moved to
  `1.0.82`; Vibe moved to `2.24.5`; Goose is `1.48.0`; Kiro is `2.21.0`;
  Bedrock maxima are `1.142.0` and `1.154.0`; and ZCode packaging is
  `3.10.2-19`. Their exact, opaque, RC, or qualified-only boundaries keep
  them record-only.
- Gemini CLI moved to `0.58.0`, but the operator deferral remains. Oh My Pi
  remains an 18.x major reset requiring identity. DSH remains on a release
  candidate. No current authority explicitly changes any named stop.

## Rank After This Record

1. **Kimi Code local server `0.40.1`** — this is a separate production family
   from the installed ACP/headless harness. npm and GitHub independently agree
   on the stable point published 2026-09-02. The local-server claim is
   AllowUnverified through `0.38.0`, and the installed host `0.34.0` remains on
   that qualified boundary. Research 270 recorded `kimi web` protocol deltas
   beginning at `0.39.0`; the `0.40.x` release notes add current local/web
   behavior changes. That is enough evidence for one dedicated identity
   Upgrade Workflow run, without flattening the sibling routes.

Claude Code `2.1.258` is visible newer but is not reopened after the completed
family. Cursor `2026.08.31-4057e58`, Antigravity `1.1.24`, Ollama `0.33.2`,
OpenCode `1.18.26`, and Grok stable `1.0.13` remain later visible candidates
for subsequent serial runs. Gemini stays deferred. `kimi-code.acp` stays
QualifiedOnly at `0.38.0`; its A2 reopen trigger is unchanged and never
automatically restores AllowUnverified. Exact/opaque families, the Oh My Pi
18.x reset, exact llama.cpp builds, the DeepSeek RC axis, and Bedrock SDK pins
remain record-only until their named identity or corpus work is authorized.

## Decision

Implement Kimi Code local-server useful-newer qualification for official
`0.40.1` next through the Contract 029 Upgrade Workflow, one family only.
This checkpoint itself changes no claim, selection, matrix, fixture, or route,
and it does not compile identity or claim cards. Keep the installed Kimi ACP
family QualifiedOnly under its A2 `0.38.0` cap. Do not reopen the just-closed
Codex or Claude Code families. Do not bulk-bump from registry `latest`; do not
contact a provider; and do not lift Gemini's deferral. The g05.009
provider-operation observation decision, card 034 planned/not-ready posture,
and `249` proved / `518` remaining projection counts are unchanged.

## Repository Evidence

- [Production solution feature matrix](../guides/provider-solution-feature-matrix.csv)
  — 40 current family rows.
- Current adapter claims in `crates/swallowtail-adapter-*/src/selection.rs`
  and the route-specific OpenAI Realtime selection module.
- [Contract 029](../contracts/029-interface-version-qualification-and-compatibility.md)
  and the [version-currentness runbook](../guides/version-currentness-checkpoint.md).
- [Research 270](./270-kimi-code-0-39-1-identity.md), [Research 274](./274-all-route-version-currentness-checkpoint.md),
  and [Research 275](./275-codex-0-152-1-identity.md).

## Sources

### Package and release channels

- [Qwen Code npm](https://registry.npmjs.org/@qwen-code%2Fqwen-code/latest)
- [Bedrock Runtime crates.io](https://crates.io/crates/aws-sdk-bedrockruntime) and [Bedrock crates.io](https://crates.io/crates/aws-sdk-bedrock)
- [Claude Agent ACP npm](https://registry.npmjs.org/@agentclientprotocol%2Fclaude-agent-acp/latest) and [releases](https://github.com/agentclientprotocol/claude-agent-acp/releases)
- [Claude Code npm](https://registry.npmjs.org/@anthropic-ai%2Fclaude-code/latest) and [`v2.1.258`](https://github.com/anthropics/claude-code/releases/tag/v2.1.258)
- [Pi npm](https://registry.npmjs.org/@earendil-works%2Fpi-coding-agent/latest) and [Pi releases](https://github.com/badlogic/pi-mono/releases)
- [Cline npm](https://registry.npmjs.org/cline/latest), [Command Code npm](https://registry.npmjs.org/command-code/latest), and [Copilot npm](https://registry.npmjs.org/@github%2Fcopilot/latest)
- [Cursor ACP registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json), [ACP schema releases](https://github.com/agentclientprotocol/agent-client-protocol/releases), and [Cursor ACP documentation](https://cursor.com/docs/cli/acp)
- [DeepSeek Harness npm](https://registry.npmjs.org/@deepseek-ai%2Fdsh/latest), [Deep Agents ACP npm](https://registry.npmjs.org/deepagents-acp/latest), and [Qoder npm](https://registry.npmjs.org/@qoder-ai/qodercli/latest)
- [Antigravity `1.1.24`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.1.24), [Gemini CLI `0.58.0`](https://github.com/google-gemini/gemini-cli/releases/tag/v0.58.0), and [Gemini CLI npm](https://registry.npmjs.org/@google%2Fgemini-cli/latest)
- [Goose `1.48.0`](https://github.com/aaif-goose/goose/releases/tag/v1.48.0), [Kiro stable manifest](https://prod.download.cli.kiro.dev/stable/latest/manifest.json), [llama.cpp `v0.3.0`](https://github.com/ggml-org/llama.cpp/releases/tag/v0.3.0), and [Ollama `v0.33.2`](https://github.com/ollama/ollama/releases/tag/v0.33.2)
- [Mistral Vibe `v2.24.5`](https://github.com/mistralai/mistral-vibe/releases/tag/v2.24.5) and [PyPI](https://pypi.org/project/mistral-vibe/)
- [Kimi Code npm](https://registry.npmjs.org/@moonshot-ai/kimi-code/latest) and [`@0.40.1`](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.40.1)
- [Oh My Pi npm](https://registry.npmjs.org/@oh-my-pi%2Fpi-coding-agent/latest), [Ollama releases](https://github.com/ollama/ollama/releases), [OpenCode npm](https://registry.npmjs.org/opencode-ai/latest), and [OpenCode `v1.18.26`](https://github.com/anomalyco/opencode/releases/tag/v1.18.26)
- [Codex npm](https://registry.npmjs.org/@openai/codex/latest) and [`rust-v0.152.1`](https://github.com/openai/codex/releases/tag/rust-v0.152.1)
- [Grok npm](https://registry.npmjs.org/@xai-official/grok/latest) and [ZCode npm](https://registry.npmjs.org/zcode-app-cli/latest)
- [ACP agent registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)

### Hosted API channels

- [Alibaba Model Studio Responses](https://www.alibabacloud.com/help/en/model-studio/qwen-api-via-openai-responses)
- [Anthropic API versioning](https://docs.anthropic.com/en/api/versioning) and [Managed Agents reference](https://platform.claude.com/docs/en/api/versioning)
- [DeepSeek Chat Completions](https://api-docs.deepseek.com/api/create-chat-completion/)
- [Gemini Live API](https://ai.google.dev/gemini-api/docs/live-api)
- [Kimi Chat API](https://platform.kimi.ai/docs/api/chat)
- [OpenAI Realtime guide](https://developers.openai.com/api/docs/guides/realtime), [model reference](https://developers.openai.com/api/docs/models/gpt-realtime-2.1), and [Background mode](https://developers.openai.com/api/docs/guides/background)
- [xAI WebSocket mode](https://docs.x.ai/developers/advanced-api-usage/websocket-mode)

### Local and repository evidence

- safe local executable discovery and promptless `--version` observations on
  2026-09-02;
- current `selection.rs` claim constants and segments;
- current [provider-solution feature matrix](../guides/provider-solution-feature-matrix.csv);
- frozen currentness records [127](./127-all-route-version-currentness-checkpoint.md),
  [159](./159-post-harness-expansion-version-currentness-checkpoint.md),
  [263](./263-all-route-version-currentness-checkpoint.md),
  [265](./265-all-route-version-currentness-checkpoint.md),
  [267](./267-all-route-version-currentness-checkpoint.md),
  [269](./269-all-route-version-currentness-checkpoint.md),
  [271](./271-all-route-version-currentness-checkpoint.md), and
  [274](./274-all-route-version-currentness-checkpoint.md); and
- the merged [Codex `0.152.1` identity record](./275-codex-0-152-1-identity.md).
