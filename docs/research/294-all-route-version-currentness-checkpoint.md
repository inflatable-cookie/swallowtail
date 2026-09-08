# 294 All-Route Version Currentness Checkpoint

Status: promoted
Owner: Tom
Date: 2026-09-08
Base: canonical `main` `abb1516f62de64114135781a783fd91990995db6`

## Question

After OpenCode HTTP `1.18.29` reached its qualified ceiling, what are the
current official stable points and safe local observations for every
production family, and which one family should be ranked next for the
Contract 029 Upgrade Workflow?

This is CHECKPOINT mode. It records observations only. It changes no
compatibility claim, feature-matrix row, fixture, route, guide, contract,
roadmap, or runtime behavior.

## Method

Compared all 41 production solution rows in the current feature matrix with
the current adapter `selection.rs` claims. Re-probed the documented official
channels on 2026-09-08:

- npm `latest` metadata and publication times for published CLI/package
  families;
- GitHub latest stable releases for binary, release-tag, and source-tag
  families;
- crates.io max stable for the Bedrock SDK axes;
- the Kiro stable manifest;
- ACP registry JSON as discovery metadata and ACP schema releases; and
- the documented hosted-API pages for opaque facade routes.

Safe local `command -v` and promptless `--version` checks were run only for
executables already on `PATH`. No prompt, login, authentication, catalogue,
provider session, install, update, host replacement, downloaded-artifact
execution, or live provider probe was used.

The official-channel observations below are point-in-time observations. They
do not qualify a newer point. Package metadata is not substituted for a
runtime identity where the matrix names a separate payload or build axis.

## Host observations

Present executable observations:

| Executable | Observation |
| --- | --- |
| `qwen` | `0.21.2` |
| `claude-agent-acp` | `0.63.0` |
| `claude` | `2.1.258 (Claude Code)` |
| `pi` | `0.83.0` |
| `command-code` | `1.15.1` |
| `cursor-agent` | `2026.08.04-aaa8809` |
| `agy` | `1.1.19` |
| `gemini` | `0.53.0` |
| `llama-server` | `0.1.0-dev`, build `10450`, commit `ece963f41` |
| `muse` | `Muse Code 1.0.3 (1.0.3-R2198.1)` |
| `kimi` | `0.34.0` |
| `omp` | `omp/18.1.6` |
| `ollama` | `0.33.3` |
| `codex` | `codex-cli 0.153.3` |
| `opencode` | `1.18.18` |
| `grok` | `1.0.13 (5e9a58528b76) [stable]` |
| Node runtime used by the exact sidecar axes | `v22.23.2` |

`cline`, `dsh`, `copilot`, `goose`, `kiro-cli`, `deepagents-acp`, `vibe`,
`qodercli`, and `zcode` were absent from `PATH`. A missing local install is
an observation, not a compatibility gap.

## Compatibility result

The checkpoint partition is 10 unchanged, 8 visible unverified-newer, and 23
record-only/deferred. There is no new material candidate: OpenCode is now on
its qualified `1.18.29` ceiling. Vocabulary follows Contract 029.

| Provider / solution | Local observation | Current official point | Swallowtail boundary | Result |
| --- | --- | --- | --- | --- |
| Alibaba \| Qwen Code headless (`qwen.headless`) | `qwen` `0.21.2` | npm [`@qwen-code/qwen-code` `0.23.0`](https://registry.npmjs.org/@qwen-code%2Fqwen-code/latest), published `2026-09-03T11:53:52.432Z` | deprecated `0.19.11..=0.20.1`; deprecated `0.21.0..=0.21.14`; maintained exact `0.21.15` and `0.22.0..=0.22.3`; `0.21.16` gap; `AllowUnverified` | `visible unverified-newer` |
| Alibaba Cloud \| Model Studio Conversations and Responses (`alibaba.conversations`) | hosted API | Official [Responses API documentation](https://www.alibabacloud.com/help/en/model-studio/qwen-api-via-openai-responses), observed `2026-09-08`; no replacement dated facade identity | exact `openai-conversations-responses` facade; `QualifiedOnly` | `unchanged` |
| Amazon Web Services \| Amazon Bedrock (`bedrock.catalogue`; `bedrock.runtime`) | embedded SDK | crates.io [`aws-sdk-bedrockruntime` `1.143.0`](https://crates.io/crates/aws-sdk-bedrockruntime), updated `2026-09-04`; [`aws-sdk-bedrock` `1.155.0`](https://crates.io/crates/aws-sdk-bedrock), updated `2026-09-04` | exact SDK/service axes; Cargo pins `1.139.0` and `1.150.0`; exact service facades; `QualifiedOnly` | `record only; future range work deferred` |
| Anthropic \| Claude Agent ACP (`claude-agent.acp`) | `claude-agent-acp` `0.63.0` | npm [`@agentclientprotocol/claude-agent-acp` `0.75.1`](https://registry.npmjs.org/@agentclientprotocol%2Fclaude-agent-acp/latest), published `2026-09-05T14:37:57.183Z`; GitHub [`v0.75.1`](https://github.com/agentclientprotocol/claude-agent-acp/releases/tag/v0.75.1), published `2026-09-05T14:35:46Z` | `0.53.0..=0.73.0` excluding `0.58.0`; `AllowUnverified` | `visible unverified-newer` |
| Anthropic \| Claude Agent SDK sidecar (`claude-agent.sdk`) | Node `22.23.2`; exact sidecar route | npm [`@anthropic-ai/claude-agent-sdk` `0.3.263`](https://registry.npmjs.org/@anthropic-ai/claude-agent-sdk/latest), published `2026-09-06T02:09:39.522Z` | exact package `0.3.259`, native `2.1.259`, Node `22.23.2`, sidecar wire, and source-tag axes; `QualifiedOnly` | `record only; future range work deferred` |
| Anthropic \| Claude Code headless and response-only (`claude-code.headless`; `claude-code.response-only`) | `claude` `2.1.258` | npm [`@anthropic-ai/claude-code` `2.1.263`](https://registry.npmjs.org/@anthropic-ai/claude-code/latest), published `2026-09-06T02:07:58.391Z`; GitHub [`v2.1.263`](https://github.com/anthropics/claude-code/releases/tag/v2.1.263), published `2026-09-06T02:54:20Z` | headless `2.1.220..=2.1.257`; response-only `2.1.227..=2.1.257`; gaps `2.1.244`, `2.1.249`, and `2.1.253..=2.1.256`; watcher exact `2.1.251`; `AllowUnverified` | `visible unverified-newer` |
| Anthropic \| Managed Agents (`anthropic.managed-agent`) | hosted API | Official [Managed Agents/versioning reference](https://platform.claude.com/docs/en/api/versioning), observed `2026-09-08`; no replacement dated facade identity | exact `anthropic-managed-agents-facade`; `QualifiedOnly` | `unchanged` |
| Anthropic \| Messages (`anthropic.messages`) | hosted API | Official [API versioning](https://docs.anthropic.com/en/api/versioning), observed `2026-09-08`; no replacement dated facade identity | exact `anthropic-2023-06-01` facade; `QualifiedOnly` | `unchanged` |
| Bad Logic \| Pi coding agent RPC (`pi.rpc`) | `pi` `0.83.0` | npm [`@earendil-works/pi-coding-agent` `0.85.1`](https://registry.npmjs.org/@earendil-works%2Fpi-coding-agent/latest), published `2026-09-05T12:17:19.281Z`; GitHub [`v0.85.1`](https://github.com/earendil-works/pi/releases/tag/v0.85.1), published `2026-09-05T12:29:01Z` | maintained exact published points through `0.84.4`; unpublished `0.83.1` gap; `AllowUnverified` | `visible unverified-newer` |
| Bad Logic \| Pi coding agent SDK sidecar (`pi.sdk-sidecar`) | `pi` `0.83.0`; Node `22.23.2` | same npm package `0.85.1`; no separate sidecar channel | exact package `0.84.2`, exact Node `22.23.2`, sidecar wire, and source-tag axes; `QualifiedOnly` | `record only; future range work deferred` |
| Cline \| Cline ACP (`cline.acp`) | `cline` missing | npm [`cline` `3.0.61`](https://registry.npmjs.org/cline/latest), published `2026-09-02T04:49:45.974Z`; nightly ignored | exact `3.0.55`; `QualifiedOnly` | `record only; future range work deferred` |
| Cline \| Cline headless (`cline.headless`) | `cline` missing | same npm package `3.0.61`; nightly ignored | exact `3.0.55`; `QualifiedOnly` | `record only; future range work deferred` |
| Command Code \| Command Code headless (`command-code.headless`) | `command-code` `1.15.1` | npm [`command-code` `1.50.1`](https://registry.npmjs.org/command-code/latest), published `2026-09-07T20:32:48.240Z`; alpha/beta/rc tags ignored | exact `1.15.1`; `QualifiedOnly` | `record only; future range work deferred` |
| Cursor \| Cursor Agent catalogue, ACP, and headless (`cursor-agent.catalogue`; `cursor-agent.acp`; `cursor-agent.headless`) | `cursor-agent` `2026.08.04-aaa8809` | ACP [registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json) reports Cursor `2026.09.02`, binary `2026.09.02-c22c1a3` | exact `2026.07.01-41b2de7`, `2026.07.23-e383d2b`, `2026.08.04-aaa8809`, and `2026.08.11-e8db854`; no inferred gap; `AllowUnverified` | `visible unverified-newer` |
| DeepSeek \| DeepSeek Harness JSON-RPC (`deepseek-harness.jsonrpc`) | `dsh` missing | npm [`@deepseek-ai/dsh` `0.1.2-rc.1`](https://registry.npmjs.org/@deepseek-ai%2Fdsh/latest), published `2026-09-03T06:21:52.107Z`; current channel remains prerelease | exact runtime-bin `0.1.0rc6`; `QualifiedOnly` | `record only; future range work deferred` |
| DeepSeek \| DeepSeek Harness Web `/api` (`deepseek-harness.local-server`) | `dsh` missing | same npm package `0.1.2-rc.1`; current channel remains prerelease | exact Web `0.1.0-rc.6`; `QualifiedOnly`; do not flatten onto JSON-RPC | `record only; future range work deferred` |
| DeepSeek \| Open Platform continuation (`deepseek.continuation`) | hosted API | Official [Chat Completions API](https://api-docs.deepseek.com/api/create-chat-completion/), observed `2026-09-08`; unversioned OpenAI-compatible endpoint; no replacement dated facade identity | exact `deepseek-openai-chat-2026-07-22` facade; `QualifiedOnly` | `unchanged` |
| GitHub \| Copilot CLI ACP (`copilot-cli.acp`) | `copilot` missing | npm [`@github/copilot` `1.0.83`](https://registry.npmjs.org/@github%2Fcopilot/latest), published `2026-09-04T15:42:30.708Z`; prerelease `1.0.83-5` ignored | exact `1.0.80`; `QualifiedOnly` | `record only; future range work deferred` |
| Google-Antigravity \| Antigravity catalogue and headless (`antigravity.catalogue`; `antigravity.headless`) | `agy` `1.1.19` | GitHub [`1.1.27`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.1.27), published `2026-09-05T04:23:25Z` | maintained `1.1.9..=1.1.17`; stop remains on unbounded HTTP 502 retry authority from Research 283; `AllowUnverified` does not reopen it | `record only; future range work deferred` |
| Google \| Gemini CLI ACP and headless (`gemini-cli.acp`; `gemini-cli.headless`) | `gemini` `0.53.0` | npm [`@google/gemini-cli` `0.58.0`](https://registry.npmjs.org/@google/gemini-cli/latest), published `2026-09-01T20:50:39.295Z`; GitHub [`v0.58.0`](https://github.com/google-gemini/gemini-cli/releases/tag/v0.58.0), published `2026-09-01T20:51:17Z` | both axes maintained `0.51.0..=0.56.0`; Gemini requalification remains deferred | `record only; future range work deferred` |
| Google \| Gemini Live API (`gemini.live`) | hosted realtime API | Official [Gemini Live API](https://ai.google.dev/gemini-api/docs/live-api), observed `2026-09-08`; no replacement dated facade identity | exact `google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent.thinking-output-max-context-compression-2026-08-23` facade; `QualifiedOnly` | `unchanged` |
| Goose \| Goose ACP (`goose.acp`) | `goose` missing | GitHub [`v1.49.0`](https://github.com/aaif-goose/goose/releases/tag/v1.49.0), published `2026-09-03T19:34:26Z` | exact `1.46.0`; `QualifiedOnly` | `record only; future range work deferred` |
| Kiro \| Kiro ACP (`kiro.acp`) | `kiro-cli` missing | official [stable manifest](https://prod.download.cli.kiro.dev/stable/latest/manifest.json) reports `2.21.1` | exact `2.18.1`; `QualifiedOnly` | `record only; future range work deferred` |
| LangChain \| Deep Agents ACP (`deepagents.acp`) | `deepagents-acp` missing | npm [`deepagents-acp` `0.1.29`](https://registry.npmjs.org/deepagents-acp/latest), published `2026-09-03T16:05:59.467Z`; ACP registry remains `0.1.7` discovery metadata | exact `0.1.25`; `QualifiedOnly`; do not bind stale registry metadata | `record only; future range work deferred` |
| llama.cpp \| attached server (`llama-cpp.attached`) | `llama-server` dev build `10450`, commit `ece963f41` | GitHub [`v0.4.0`](https://github.com/ggml-org/llama.cpp/releases/tag/v0.4.0), published `2026-09-04T19:56:47Z`; release tag is not selected build identity | exact attached build `b9910/f5525f7e7`; `QualifiedOnly` | `record only; future range work deferred` |
| llama.cpp \| owned server lifecycle (`llama-cpp.owned`) | same dev build `10450`, commit `ece963f41` | same GitHub `v0.4.0`; no build-to-claim inference | exact owned build `b10069/178a6c449`; `QualifiedOnly` | `record only; future range work deferred` |
| Meta \| Muse Code headless (`muse-code.headless`) | `muse` `1.0.3 (1.0.3-R2198.1)` | no public package or release channel for the signed payload; local authority remains the exact payload record | exact opaque `0.2.1-R1215.1`; mutable launcher is not the execution target; `QualifiedOnly` | `record only; future range work deferred` |
| Mistral \| Mistral Vibe headless (`mistral-vibe.headless`) | `vibe` missing | GitHub [`v2.25.0`](https://github.com/mistralai/mistral-vibe/releases/tag/v2.25.0), published `2026-09-04T08:53:52Z`; [PyPI `2.25.0`](https://pypi.org/project/mistral-vibe/) | exact `2.24.2`; `QualifiedOnly` | `record only; future range work deferred` |
| Moonshot AI \| Kimi Code installed harness (`kimi-code.acp`; `kimi-code.headless`) | `kimi` `0.34.0` | npm [`@moonshot-ai/kimi-code` `0.41.0`](https://registry.npmjs.org/@moonshot-ai/kimi-code/latest), published `2026-09-04T11:01:04.740Z`; GitHub [`@0.41.0`](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai/kimi-code%400.41.0) | ACP `QualifiedOnly` exact `0.28.1` plus `0.29.0..=0.38.0`, excluding `0.39.0` and `0.39.1`; headless v1 `0.29.0..=0.32.0`, v2 `0.33.0..=0.39.1` | `record only; future range work deferred` |
| Moonshot AI \| Kimi Code local server (`kimi-code.local-server`) | `kimi` `0.34.0` | same npm/GitHub `0.41.0` | exact `0.28.1` plus `0.29.0..=0.38.0`; `AllowUnverified`; Research 282 stop on uncontained Bash `cwd` authority remains | `record only; future range work deferred` |
| Moonshot AI \| Kimi Platform Chat API (`kimi-platform.chat`) | hosted API | Official [Kimi Chat API](https://platform.kimi.ai/docs/api/chat), observed `2026-09-08`; no replacement dated facade identity | exact `kimi-platform-chat-2026-07-21` facade; `QualifiedOnly` | `unchanged` |
| Oh My Pi \| coding agent RPC (`oh-my-pi.rpc`) | `omp` `18.1.6` | npm [`@oh-my-pi/pi-coding-agent` `18.1.14`](https://registry.npmjs.org/@oh-my-pi%2Fpi-coding-agent/latest), published `2026-09-07T18:46:59.646Z`; major-line reset from qualified 17.x | maintained `17.2.9..=17.4.0`; 18.x requires identity evidence | `record only; future range work deferred` |
| Ollama \| native attached runtime (`ollama.attached`) | `ollama` `0.33.3` | GitHub [`v0.33.3`](https://github.com/ollama/ollama/releases/tag/v0.33.3), published `2026-09-02T00:11:33Z` | `0.14.0..=0.32.15`; exclusions `0.32.2` and `0.32.10`; `AllowUnverified` | `visible unverified-newer` |
| OpenAI \| Codex app-server and exec (`codex.app-server`; `codex.exec`) | `codex` `0.153.3` | npm [`@openai/codex` `0.153.4`](https://registry.npmjs.org/@openai/codex/latest), published `2026-09-04T23:31:18.513Z`; GitHub [`rust-v0.153.4`](https://github.com/openai/codex/releases/tag/rust-v0.153.4), published `2026-09-04T23:25:48Z`; alpha channels ignored | maintained through `0.152.1`; gaps `0.82.0..=0.83.0`, `0.108.0`, `0.109.0`, `0.149.2`, `0.150.2`, and `0.151.1`; `AllowUnverified` | `visible unverified-newer` |
| OpenAI \| Realtime API (`openai.realtime`) | hosted realtime API | Official [Realtime guide](https://developers.openai.com/api/docs/guides/realtime) and [model reference](https://developers.openai.com/api/docs/models/gpt-realtime-2.1), observed `2026-09-08`; no replacement dated facade identity | exact `openai-realtime-reasoning-2026-08-27` facade; superseded point retained as proof; `QualifiedOnly` | `unchanged` |
| OpenAI \| Responses background API (`openai.background`) | hosted API | Official [Background mode guide](https://developers.openai.com/api/docs/guides/background), observed `2026-09-08`; no replacement dated facade identity | exact `openai-responses-background-2026-08-23-service-tier` facade; `QualifiedOnly` | `unchanged` |
| OpenCode \| HTTP server (`opencode.http`) | `opencode` `1.18.18` | npm [`opencode-ai` `1.18.29`](https://registry.npmjs.org/opencode-ai/latest), published `2026-09-04T23:46:25.653Z`; GitHub [`v1.18.29`](https://github.com/anomalyco/opencode/releases/tag/v1.18.29), published `2026-09-04T23:47:16Z` | published qualified segments through exact `1.18.29`; `AllowUnverified`; Research 292 completed this extension | `unchanged` |
| Qoder \| Qoder headless (`qoder.headless`) | `qodercli` missing | npm [`@qoder-ai/qodercli` `1.1.46`](https://registry.npmjs.org/@qoder-ai/qodercli/latest), published `2026-09-07T12:41:14.957Z` | exact `1.1.25`; `QualifiedOnly` | `record only; future range work deferred` |
| xAI \| Grok Build ACP (`grok-build.acp`) | `grok` `1.0.13` stable | npm [`@xai-official/grok` `latest`](https://registry.npmjs.org/@xai-official/grok/latest) is `1.0.13`; stable `1.0.14..=1.0.23` exist outside the `latest` dist-tag; alpha `1.0.24` ignored | deprecated `0.2.114..=0.2.117`; maintained `1.0.4..=1.0.5`; later stable unverified; do not flatten npm `latest`, alpha, or ACP registry identity | `visible unverified-newer` |
| xAI \| Responses WebSocket API (`xai.responses-websocket`) | hosted realtime API | Official [WebSocket mode](https://docs.x.ai/developers/advanced-api-usage/websocket-mode), observed `2026-09-08`; no replacement dated facade identity | exact `xai-responses-websocket-2026-04-23` facade; `QualifiedOnly` | `unchanged` |
| Z.AI \| ZCode App-Server (`zcode.app-server`) | `zcode` missing | npm [`zcode-app-cli` `3.11.2-22`](https://registry.npmjs.org/zcode-app-cli/latest), published `2026-09-07T15:59:53.778Z`; packaging metadata is not the runtime axis | exact `zcode.runtime` `0.16.3`; `QualifiedOnly`; do not flatten npm packaging onto `zcode.cjs` | `record only; future range work deferred` |

## Shared ACP surfaces

- Stable ACP schema remains GitHub [`schema-v1.21.0`](https://github.com/agentclientprotocol/agent-client-protocol/releases/tag/schema-v1.21.0), published `2026-08-20`; the frozen corpus records `v1.20.0`. Result: `record only; future range work deferred`.
- ACP agent registry remains `1.0.0` with 39 agents. Current selected metadata includes Cursor `2026.09.02` / binary `2026.09.02-c22c1a3`, Gemini `0.58.0`, and DeepAgents `0.1.7`. Registry metadata is discovery evidence, not a Swallowtail compatibility claim. Result: `record only; future range work deferred`.

## Changed observations since Research 284

- OpenCode npm and GitHub remain at `1.18.29`; Research 292 already completed
  that compatible `surface-19` extension. Its row is now unchanged, not a
  new candidate.
- Official latest moved for Claude Agent ACP (`0.75.1`), Claude Code
  (`2.1.263`), Pi (`0.85.1`), Command Code (`1.50.1`), Oh My Pi (`18.1.14`),
  Codex (`0.153.4`), Qoder (`1.1.46`), and ZCode packaging (`3.11.2-22`).
- Official latest also moved for Antigravity (`1.1.27`) and llama.cpp
  (`v0.4.0`); their existing stop/qualified-only boundaries remain.
- Kiro's stable manifest moved to `2.21.1`.
- Host Ollama moved to `0.33.3` and host Codex to `0.153.3`; both are above
  their qualified ceilings and remain observation-only. No host was changed.
- The latest Qwen, Cline, Copilot, DeepSeek, Deep Agents, Gemini, Kimi,
  Goose, Mistral Vibe, and OpenCode channel values otherwise remain as
  recorded above. Grok npm `latest` remains `1.0.13`; stable versions through
  `1.0.23` remain outside that dist-tag and alpha `1.0.24` remains ignored.

## Candidate ranking

1. **Cursor Agent** (`cursor-agent.catalogue`; `cursor-agent.acp`;
   `cursor-agent.headless`) at registry `2026.09.02` / binary
   `2026.09.02-c22c1a3`. The host `2026.08.04-aaa8809` is already inside the
   qualified date/build window, the claim permits unverified newer points, and
   no active stop or operator deferral blocks identity work.
2. **Claude Agent ACP** `0.75.1`. The host `0.63.0` is a deprecated qualified
   point and official channels agree, but this family was only recently
   qualified through `0.73.0` and now spans two newer published points.
3. **Pi RPC** `0.85.1`. The host `0.83.0` is qualified and official channels
   agree; `0.83.1` remains a gap and the family was recently qualified through
   `0.84.4`.
4. **Qwen Code** `0.23.0`. `AllowUnverified` and a qualified host remain, but
   the minor-line move from `0.22.x` needs its own identity evidence.

Ollama and Codex are below these candidates because their current hosts are
already unverified newer. Claude Code and Codex were also recently closed.
Antigravity remains stopped on the Contract 023 retry authority question;
Kimi local server remains stopped by Research 282; Kimi ACP remains under the
A2 cap; Gemini remains deferred; and Oh My Pi remains a major-line reset.

## Decision

Rank **Cursor Agent `2026.09.02`** as the next family for a separately
compiled Contract 029 Upgrade Workflow identity run. This checkpoint does not
dispatch or compile that family, and it changes no production claim.

Keep the following unchanged:

- Kimi ACP capped at `0.38.0` under A2;
- Kimi local server stopped at `0.38.0` under Research 282;
- Antigravity stopped at `1.1.17` under Research 283;
- Gemini CLI requalification deferred;
- Claude Code watcher exact at `2.1.251`; and
- all current qualified points, gaps, exclusions, and qualified-only axes.

## Sources

The package, release, registry, and hosted-documentation links in the table
are the documented official channels re-probed on 2026-09-08. Repository
authority and claim comparison used:

- [Contract 029](../contracts/029-interface-version-qualification-and-compatibility.md);
- [Standing Lanes](../roadmaps/standing-lanes.md);
- [Version Currentness Checkpoint guide](../guides/version-currentness-checkpoint.md);
- [Research 284](./284-all-route-version-currentness-checkpoint.md);
- [Research 292](./292-opencode-http-1-18-29-identity.md); and
- [Research 293](./293-codex-permission-exchange-reconciliation.md).

No claim, matrix, fixture, guide, contract, crate, or provider-wide harness
activity record was changed.
