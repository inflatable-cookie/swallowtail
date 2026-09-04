# 284 All-Route Version Currentness Checkpoint

Status: promoted
Owner: Tom
Date: 2026-09-04

## Question

After the v0.4.0 release freeze and post-release investigation stops on Kimi Code
local server `0.41.0` and Antigravity `1.1.26`, what are the current official
stable points and safe local observations for every production family, and
which one family should be the next Contract 029 currentness candidate?

This checkpoint is research only. It changes no compatibility claim, feature-
matrix row, fixture, route, or runtime behavior. It keeps Kimi ACP under the
A2 cap, Kimi local server stopped at `0.38.0` on Bash `cwd` authority,
Antigravity stopped at `1.1.17` on Contract 023 HTTP 502 unbounded retry, and
Gemini CLI deferred.

## Method

Compared:

- all production rows in the current production feature matrix with current
  adapter `selection.rs` claims;
- safe local `command -v` and promptless `--version` observations for named
  executables already installed;
- fresh official npm `latest`, GitHub latest stable releases or tags, the Kiro
  stable manifest, crates.io max stable, PyPI, and ACP registry metadata; and
- official hosted-API documentation for opaque facade routes.

The observation date is 2026-09-04. Official package, release, registry, and
hosted-documentation channels were re-probed during this checkpoint. Safe local
checks resolved `qwen`, `claude-agent-acp`, `claude`, `pi`, `command-code`,
`cursor-agent`, `agy`, `gemini`, `llama-server`, `muse`, `kimi`, `omp`,
`ollama`, `codex`, `opencode`, and `grok`. Their observations are recorded
below. `cline`, `dsh`, `copilot`, `goose`, `kiro-cli`, `deepagents-acp`,
`vibe`, `qodercli`, and `zcode` were missing from `PATH`. A missing local
install is an observation, not a compatibility gap.

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
- `muse` promptless replay returns `1.0.3 (1.0.3-R2198.1)`;
- `kimi` `0.34.0`;
- `omp` `18.1.6`;
- `ollama` `0.33.2`;
- `codex-cli` `0.150.1`;
- `opencode` `1.18.18`; and
- `grok` `1.0.13 (5e9a58528b76) [stable]`.

No provider prompt, authentication, catalogue, session, install, update, host
change, or live probe was used. Official artifacts were not installed, executed,
or substituted for host tools. Hosted latest-model values and ACP registry
package versions are discovery evidence, not Swallowtail compatibility claims.

## Compatibility Result

Vocabulary follows [Contract 029](../contracts/029-interface-version-qualification-and-compatibility.md):
`unchanged`, `visible unverified-newer`, `record only; future range work deferred`,
and `material candidate`.

Partition across the 41 production feature-matrix solution rows (40 historical
family lines plus the v0.4.0 `claude-agent.sdk` row): 9 unchanged, 8 visible
unverified-newer, 23 record only; future range work deferred, and 1 material
candidate.

| Provider / solution | Local observation | Official stable point | Swallowtail boundary | Result |
| --- | --- | --- | --- | --- |
| Alibaba \| Qwen Code headless (`qwen.headless`) | `qwen`; `--version` `0.21.2` | npm [`@qwen-code/qwen-code` `0.23.0`](https://registry.npmjs.org/@qwen-code%2Fqwen-code/latest), published 2026-09-03T11:53:52.432Z | deprecated `0.19.11..=0.20.1`; deprecated `0.21.0..=0.21.14`; maintained exact `0.21.15` and `0.22.0..=0.22.3`; `0.21.16` gap; AllowUnverified | `visible unverified-newer` |
| Alibaba Cloud \| Model Studio Conversations and Responses (`alibaba.conversations`) | hosted API | Official [Responses API documentation](https://www.alibabacloud.com/help/en/model-studio/qwen-api-via-openai-responses), observed 2026-09-04; no replacement dated facade identity | exact `openai-conversations-responses` facade; QualifiedOnly | `unchanged` |
| Amazon Web Services \| Amazon Bedrock catalogue and Runtime (`bedrock.catalogue`; `bedrock.runtime`) | embedded SDK | crates.io [`aws-sdk-bedrockruntime` `1.142.0`](https://crates.io/crates/aws-sdk-bedrockruntime) and [`aws-sdk-bedrock` `1.154.0`](https://crates.io/crates/aws-sdk-bedrock), max stable observed 2026-09-04 | exact SDK/service axes; Cargo pins `aws-sdk-bedrockruntime =1.139.0` and `aws-sdk-bedrock =1.150.0`; exact service facades; QualifiedOnly | `record only; future range work deferred` |
| Anthropic \| Claude Agent ACP (`claude-agent.acp`) | `claude-agent-acp`; `--version` `0.63.0` | npm [`@agentclientprotocol/claude-agent-acp` `0.74.0`](https://registry.npmjs.org/@agentclientprotocol%2Fclaude-agent-acp/latest), published 2026-09-04T11:11:03.220Z; GitHub [`v0.74.0`](https://github.com/agentclientprotocol/claude-agent-acp/releases/tag/v0.74.0) | `0.53.0..=0.73.0` excluding `0.58.0`; AllowUnverified | `visible unverified-newer` |
| Anthropic \| Claude Agent SDK sidecar (`claude-agent.sdk`) | installed harness / source-tagged Node sidecar; host Node `22.23.2` | npm [`@anthropic-ai/claude-agent-sdk` `0.3.260`](https://registry.npmjs.org/@anthropic-ai%2Fclaude-agent-sdk/latest), published 2026-09-03T22:33:32.324Z | exact `0.3.259` package, exact native `2.1.259`, Node `22.23.2`, sidecar wire, and source-tag axes; QualifiedOnly; no unverified-newer posture | `record only; future range work deferred` |
| Anthropic \| Claude Code headless and response-only (`claude-code.headless`; `claude-code.response-only`) | `claude`; `--version` `2.1.258 (Claude Code)` | npm [`@anthropic-ai/claude-code` `2.1.260`](https://registry.npmjs.org/@anthropic-ai%2Fclaude-code/latest), published 2026-09-03T22:32:02.087Z; GitHub [`v2.1.260`](https://github.com/anthropics/claude-code/releases/tag/v2.1.260) | headless `2.1.220..=2.1.257`; response-only `2.1.227..=2.1.257`; gaps `2.1.244`, `2.1.249`, and hop-skipped `2.1.253..=2.1.256`; AllowUnverified; watcher stays exact `2.1.251` | `visible unverified-newer` |
| Anthropic \| Managed Agents (`anthropic.managed-agent`) | hosted API | Official [Claude API versioning and Managed Agents reference](https://platform.claude.com/docs/en/api/versioning), observed 2026-09-04; no replacement beta facade identity | exact `anthropic-managed-agents-facade`; QualifiedOnly | `unchanged` |
| Anthropic \| Messages (`anthropic.messages`) | hosted API | Official [API versioning](https://docs.anthropic.com/en/api/versioning), observed 2026-09-04; no replacement dated facade identity | exact `anthropic-2023-06-01` facade; QualifiedOnly | `unchanged` |
| Bad Logic \| Pi coding agent RPC (`pi.rpc`) | `pi`; `--version` `0.83.0` | npm [`@earendil-works/pi-coding-agent` `0.85.0`](https://registry.npmjs.org/@earendil-works%2Fpi-coding-agent/latest), published 2026-09-04T10:18:05.208Z; GitHub [`v0.85.0`](https://github.com/badlogic/pi-mono/releases/tag/v0.85.0) | maintained exact published points through `0.84.4`; `0.83.1` gap; AllowUnverified | `visible unverified-newer` |
| Bad Logic \| Pi coding agent SDK sidecar (`pi.sdk-sidecar`) | `pi`; `--version` `0.83.0` | same upstream npm package `0.85.0`; sidecar has no separate current package channel | exact `0.84.2` sidecar package, exact Node `22.23.2`, sidecar wire, and source-tag axes; QualifiedOnly | `record only; future range work deferred` |
| Cline \| Cline ACP (`cline.acp`) | `cline` missing from `PATH` | npm [`cline` `3.0.61`](https://registry.npmjs.org/cline/latest), published 2026-09-02; nightly ignored | exact `3.0.55`; QualifiedOnly | `record only; future range work deferred` |
| Cline \| Cline headless (`cline.headless`) | `cline` missing from `PATH` | same npm package `3.0.61`; nightly ignored | exact `3.0.55`; QualifiedOnly | `record only; future range work deferred` |
| Command Code \| Command Code headless (`command-code.headless`) | `command-code`; `--version` `1.15.1` | npm [`command-code` `1.47.1`](https://registry.npmjs.org/command-code/latest), published 2026-09-04T15:44:40.437Z; alpha/beta/rc tags ignored | exact `1.15.1`; QualifiedOnly | `record only; future range work deferred` |
| Cursor \| Cursor Agent catalogue, ACP, and headless (`cursor-agent.catalogue`; `cursor-agent.acp`; `cursor-agent.headless`) | `cursor-agent`; `--version` `2026.08.04-aaa8809` | ACP registry [Cursor `2026.09.02`](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json), binary identity `2026.09.02-c22c1a3` | exact `2026.07.01-41b2de7`, `2026.07.23-e383d2b`, `2026.08.04-aaa8809`, and `2026.08.11-e8db854`; no inferred gap; AllowUnverified | `visible unverified-newer` |
| DeepSeek \| DeepSeek Harness JSON-RPC (`deepseek-harness.jsonrpc`) | `dsh` missing from `PATH` | npm [`@deepseek-ai/dsh` `0.1.2-rc.1`](https://registry.npmjs.org/@deepseek-ai%2Fdsh/latest), published 2026-09-03T06:21:52.107Z; current channel remains prerelease | exact runtime-bin `0.1.0rc6`; QualifiedOnly | `record only; future range work deferred` |
| DeepSeek \| DeepSeek Harness Web `/api` (`deepseek-harness.local-server`) | `dsh` missing from `PATH` | same npm package `0.1.2-rc.1`; current channel remains prerelease | exact Web `0.1.0-rc.6`; QualifiedOnly; do not flatten onto JSON-RPC | `record only; future range work deferred` |
| DeepSeek \| Open Platform continuation (`deepseek.continuation`) | hosted API | Official [DeepSeek Chat Completions API](https://api-docs.deepseek.com/api/create-chat-completion/) remains an unversioned OpenAI-compatible endpoint; no replacement dated facade identity | exact `deepseek-openai-chat-2026-07-22` facade; QualifiedOnly | `unchanged` |
| GitHub \| Copilot CLI ACP (`copilot-cli.acp`) | `copilot` missing from `PATH` | npm [`@github/copilot` `1.0.83`](https://registry.npmjs.org/@github%2Fcopilot/latest), published 2026-09-04T15:42:30.708Z; prerelease `1.0.83-5` ignored | exact `1.0.80`; QualifiedOnly | `record only; future range work deferred` |
| Google-Antigravity \| Antigravity catalogue and headless (`antigravity.catalogue`; `antigravity.headless`) | `agy`; `--version` `1.1.19` | GitHub release [`1.1.26`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.1.26), published 2026-09-04T03:28:48Z | maintained `1.1.9..=1.1.17`; later stable points visible above ceiling; AllowUnverified; stopped on Contract 023 unbounded HTTP 502 retry authority, keeping ceiling at `1.1.17` | `record only; future range work deferred` |
| Google \| Gemini CLI ACP and headless (`gemini-cli.acp`; `gemini-cli.headless`) | `gemini`; `--version` `0.53.0` | npm [`@google/gemini-cli` `0.58.0`](https://registry.npmjs.org/@google%2Fgemini-cli/latest), published 2026-09-01; GitHub [`v0.58.0`](https://github.com/google-gemini/gemini-cli/releases/tag/v0.58.0); preview/nightly ignored | both axes maintained `0.51.0..=0.56.0`; later stable unverified; Gemini requalification deferred | `record only; future range work deferred` |
| Google \| Gemini Live API (`gemini.live`) | hosted realtime API | Official [Gemini Live API](https://ai.google.dev/gemini-api/docs/live-api), observed 2026-09-04; no replacement dated facade identity | exact `google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent.thinking-output-max-context-compression-2026-08-23` facade; QualifiedOnly | `unchanged` |
| Goose \| Goose ACP (`goose.acp`) | `goose` missing from `PATH` | GitHub release [`v1.49.0`](https://github.com/aaif-goose/goose/releases/tag/v1.49.0), published 2026-09-03T19:34:26Z | exact `1.46.0`; QualifiedOnly | `record only; future range work deferred` |
| Kiro \| Kiro ACP (`kiro.acp`) | `kiro-cli` missing from `PATH` | official [stable manifest](https://prod.download.cli.kiro.dev/stable/latest/manifest.json) reports `2.21.0` | exact `2.18.1`; QualifiedOnly | `record only; future range work deferred` |
| LangChain \| Deep Agents ACP (`deepagents.acp`) | `deepagents-acp` missing from `PATH` | npm [`deepagents-acp` `0.1.29`](https://registry.npmjs.org/deepagents-acp/latest), published 2026-09-03T16:05:59.467Z; ACP registry `DeepAgents` remains `0.1.7` discovery metadata | exact `0.1.25`; QualifiedOnly; do not bind the stale registry value | `record only; future range work deferred` |
| llama.cpp \| llama.cpp attached server (`llama-cpp.attached`) | `llama-server`; `0.1.0-dev`, build `10450`, commit `ece963f41` | GitHub [`v0.3.0`](https://github.com/ggml-org/llama.cpp/releases/tag/v0.3.0), published 2026-08-25; release tag is not selected build identity | exact attached build `b9910/f5525f7e7`; QualifiedOnly | `record only; future range work deferred` |
| llama.cpp \| llama.cpp owned server lifecycle (`llama-cpp.owned`) | same `llama-server` build `10450`, commit `ece963f41` | same GitHub latest `v0.3.0`; no build-to-claim inference | exact owned build `b10069/178a6c449`; QualifiedOnly | `record only; future range work deferred` |
| Meta \| Muse Code headless (`muse-code.headless`) | `muse`; `--version` promptless replay returns `Muse Code 1.0.3 (1.0.3-R2198.1)` | no public package or release channel for the signed payload located; local authority remains the exact payload record | exact opaque `0.2.1-R1215.1`; QualifiedOnly; mutable launcher is not the execution target | `record only; future range work deferred` |
| Mistral \| Mistral Vibe headless (`mistral-vibe.headless`) | `vibe` missing from `PATH` | GitHub [`v2.25.0`](https://github.com/mistralai/mistral-vibe/releases/tag/v2.25.0), published 2026-09-04T08:53:52Z; PyPI [`2.25.0`](https://pypi.org/project/mistral-vibe/) | exact `2.24.2`; QualifiedOnly | `record only; future range work deferred` |
| Moonshot AI \| Kimi Code installed harness (`kimi-code.acp`; `kimi-code.headless`) | `kimi`; `--version` `0.34.0` | npm [`@moonshot-ai/kimi-code` `0.41.0`](https://registry.npmjs.org/@moonshot-ai%2Fkimi-code/latest), published 2026-09-04T11:01:04.740Z; GitHub [`@0.41.0`](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.41.0) | ACP QualifiedOnly exact `0.28.1` plus `0.29.0..=0.38.0`, with `0.39.0` and `0.39.1` excluded; every ACP point above `0.38.0` fails closed; headless v1 `0.29.0..=0.32.0`, v2 `0.33.0..=0.39.1` | `record only; future range work deferred` |
| Moonshot AI \| Kimi Code local server (`kimi-code.local-server`) | `kimi`; `--version` `0.34.0` | same npm package `0.41.0`, published 2026-09-04T11:01:04.740Z; GitHub tag `@0.41.0` | exact `0.28.1` plus `0.29.0..=0.38.0`; AllowUnverified; stopped at `0.41.0` ([Research 282](./282-kimi-code-local-server-0-41-0-identity.md)) on uncontained Bash `cwd` authority change at `0.40.0`, keeping ceiling at `0.38.0` | `record only; future range work deferred` |
| Moonshot AI \| Kimi Platform Chat API (`kimi-platform.chat`) | hosted API | Official [Kimi Chat Completions API](https://platform.kimi.ai/docs/api/chat), observed 2026-09-04; no replacement dated facade identity | exact `kimi-platform-chat-2026-07-21` facade; QualifiedOnly | `unchanged` |
| Oh My Pi \| Oh My Pi coding agent RPC (`oh-my-pi.rpc`) | `omp`; `--version` `omp/18.1.6` | npm [`@oh-my-pi/pi-coding-agent` `18.1.10`](https://registry.npmjs.org/@oh-my-pi%2Fpi-coding-agent/latest), published 2026-09-04T10:12:21.333Z; major line reset from qualified 17.x | maintained `17.2.9..=17.4.0`; 18.x requires identity evidence before any claim | `record only; future range work deferred` |
| Ollama \| Ollama native attached runtime (`ollama.attached`) | `ollama`; `--version` `ollama version is 0.33.2` | GitHub release [`v0.33.3`](https://github.com/ollama/ollama/releases/tag/v0.33.3), published 2026-09-02T00:11:33Z | `0.14.0..=0.32.15`; exclusions `0.32.2` and `0.32.10`; AllowUnverified | `visible unverified-newer` |
| OpenAI \| Codex app-server and exec (`codex.app-server`; `codex.exec`) | `codex`; `--version` `codex-cli 0.150.1` | npm [`@openai/codex` `0.153.2`](https://registry.npmjs.org/@openai%2Fcodex/latest), published 2026-09-03T23:57:21.074Z; GitHub [`rust-v0.153.2`](https://github.com/openai/codex/releases/tag/rust-v0.153.2); alpha `0.154.0-alpha.3` ignored | exec and app-server maintained through `0.152.1`; gaps `0.82.0..=0.83.0`, `0.108.0`, `0.109.0`, `0.149.2`, `0.150.2`, and `0.151.1`; AllowUnverified | `visible unverified-newer` |
| OpenAI \| Realtime API (`openai.realtime`) | hosted realtime API | Official [Realtime guide](https://developers.openai.com/api/docs/guides/realtime) and [`gpt-realtime-2.1` model reference](https://developers.openai.com/api/docs/models/gpt-realtime-2.1), observed 2026-09-04; no replacement dated facade identity | exact `openai-realtime-reasoning-2026-08-27` facade; superseded point retained as proof; QualifiedOnly | `unchanged` |
| OpenAI \| Responses background API (`openai.background`) | hosted API | Official [Background mode guide](https://developers.openai.com/api/docs/guides/background), observed 2026-09-04; no replacement dated facade identity | exact `openai-responses-background-2026-08-23-service-tier` facade; QualifiedOnly | `unchanged` |
| OpenCode \| OpenCode HTTP server (`opencode.http`) | `opencode`; `--version` `1.18.18` | npm [`opencode-ai` `1.18.28`](https://registry.npmjs.org/opencode-ai/latest), published 2026-09-04T15:40:40.661Z; GitHub [`v1.18.28`](https://github.com/anomalyco/opencode/releases/tag/v1.18.28) | published qualified segments through `1.18.20`; AllowUnverified | `material candidate` |
| Qoder \| Qoder headless (`qoder.headless`) | `qodercli` missing from `PATH` | npm [`@qoder-ai/qodercli` `1.1.43`](https://registry.npmjs.org/@qoder-ai%2Fqodercli/latest), published 2026-09-04T14:29:53.008Z | exact `1.1.25`; QualifiedOnly | `record only; future range work deferred` |
| xAI \| Grok Build ACP (`grok-build.acp`) | `grok`; `--version` `1.0.13 (5e9a58528b76) [stable]` | npm [`@xai-official/grok` `1.0.13`](https://registry.npmjs.org/@xai-official%2Fgrok/latest); stable `1.0.14..=1.0.18` exist off `latest`; alpha `1.0.19` ignored | deprecated `0.2.114..=0.2.117`; maintained `1.0.4..=1.0.5`; later stable unverified; do not flatten npm latest, alpha, or ACP registry identity | `visible unverified-newer` |
| xAI \| Responses WebSocket API (`xai.responses-websocket`) | hosted realtime API | Official [xAI WebSocket mode](https://docs.x.ai/developers/advanced-api-usage/websocket-mode), observed 2026-09-04; no replacement dated facade identity | exact `xai-responses-websocket-2026-04-23` facade; QualifiedOnly | `unchanged` |
| Z.AI \| ZCode App-Server (`zcode.app-server`) | `zcode` missing from `PATH` | npm [`zcode-app-cli` `3.10.2-19`](https://registry.npmjs.org/zcode-app-cli/latest), published 2026-09-02T10:14:10.329Z; packaging metadata is not the runtime axis | exact `zcode.runtime` `0.16.3`; QualifiedOnly; do not flatten npm packaging onto `zcode.cjs` | `record only; future range work deferred` |

Shared ACP checkpoint surfaces:

- Stable ACP schema: GitHub [schema `schema-v1.21.0`](https://github.com/agentclientprotocol/agent-client-protocol/releases/tag/schema-v1.21.0),
  published 2026-08-20. Frozen corpus records `v1.20.0`. Result: `record only; future range work deferred`.
- ACP agent registry: `1.0.0`, 39 agents; selected entries include Cursor
  `2026.09.02` (binary identity `2026.09.02-c22c1a3`), Gemini `0.58.0`,
  DeepAgents `0.1.7`. Result: `record only; future range work deferred`.

## Changed Observations Since Research 276

1. **v0.4.0 Release and Freeze Audit Complete**:
   The v0.4.0 release freeze ([Research 281](./281-v0-4-0-compatibility-and-freeze-audit.md))
   was completed and tagged. In-progress currentness work resumed post-release.

2. **Kimi Code Local Server Identity Stop (`0.41.0`)**:
   During the post-release investigation of Kimi Code local server (Card 062 /
   g05.026), npm published `0.41.0` while the lane was open targeting `0.40.1`.
   Following the Contract 029 In-Run Latest Movement rule, the identity scope
   was extended to include `0.41.0`. The investigation stopped on
   `0.40.0..=0.41.0` in [Research 282](./282-kimi-code-local-server-0-41-0-identity.md)
   because upstream added an uncontained Bash tool `cwd` change authority that
   mutates working directories for local-server clients without client opt-in or
   session containment, violating [Contract 017](../contracts/017-provider-owned-session-load-replay-and-host-containment.md)
   and [Contract 023](../contracts/023-harness-operation-isolation-and-native-boundary.md).
   The qualified ceiling remains held at `0.38.0`. Reopen condition: when a named
   Swallowtail control, HostEnforced/ProviderEnforced isolation, or restored
   provider workspace assertion contains Bash cwd for a local-server client.

3. **Antigravity Identity Stop (`1.1.26`)**:
   The post-release investigation of Antigravity CLI (Card 071 / g05.027)
   analyzed hops `1.1.18..=1.1.26`. The investigation stopped in Research 283
   because upstream introduced unbounded HTTP 502 retry behavior against model
   endpoints at `1.1.22` without a published finite retry bound or a
   deterministic disable control, violating [Contract 023](../contracts/023-harness-operation-isolation-and-native-boundary.md).
   The qualified ceiling remains held at `1.1.17`. Reopen condition: when official
   evidence names a finite retry policy plus a deterministic disable or bound, or
   the operator separately accepts the exact provider retry behavior under
   Contract 023.

4. **Kimi ACP Cap (`0.38.0`)**:
   Remains QualifiedOnly at `0.38.0` under the A2 cap (spawn of host process
   when `terminal: false` advertised). Reopen condition: when every invocation
   path fails closed again for a terminal-less client, or upstream supplies a
   ProviderEnforced boundary satisfying Contracts 017 and 023.

5. **Gemini CLI Requalification Deferral**:
   Both axes (`gemini-cli.acp` and `gemini-cli.headless`) remain maintained
   through `0.56.0`, with newer official points (`0.58.0`) unverified due to
   standing operator deferral.

6. **Recent Closed Families Visible Newer Observations**:
   - Claude Code moved npm and GitHub to `2.1.260`; host remains `2.1.258`.
     Ceiling remains `2.1.257`; watcher remains exact `2.1.251`.
   - Codex moved npm and GitHub to `0.153.2`; host remains `0.150.1`.
     Ceiling remains `0.152.1`.
   Both families were recently qualified and closed; visible newer versions
   are recorded without reopening.

7. **Other Package and Release Channel Movements**:
   - OpenCode moved npm and GitHub to `1.18.28` (published 2026-09-04); host is `1.18.18`.
   - Pi RPC moved npm and GitHub to `0.85.0`; host is `0.83.0`.
   - Qwen Code moved npm to `0.23.0`; host is `0.21.2`.
   - Claude Agent ACP moved npm and GitHub to `0.74.0`; host is `0.63.0`.
   - Claude Agent SDK sidecar moved npm to `0.3.260`; QualifiedOnly claim remains `0.3.259`.
   - Cursor Agent moved in ACP registry to `2026.09.02` with binary identity `2026.09.02-c22c1a3`; host is `2026.08.04-aaa8809`.
   - Ollama moved GitHub to `0.33.3`; host is `0.33.2`.
   - Mistral Vibe moved to `v2.25.0` (GitHub/PyPI).
   - Goose moved to `v1.49.0` (GitHub).
   - Copilot CLI moved to `1.0.83` (npm).
   - Command Code moved to `1.47.1` (npm).
   - Deep Agents moved to `0.1.29` (npm).
   - DeepSeek Harness moved to `0.1.2-rc.1` (npm).
   - Muse Code host observation moved: promptless replay returns `1.0.3 (1.0.3-R2198.1)` (previously `1.0.1`); signed payload remains exact opaque `0.2.1-R1215.1` (QualifiedOnly).
   - Oh My Pi moved npm to `18.1.10`; host is `18.1.6`.
   - Qoder moved to `1.1.43` (npm).

## Candidate Ranking

Under Contract 029 and the version-currentness skill ranking criteria:
1. `AllowUnverified` families whose official and/or host stable is newer than the
   qualified ceiling, and whose host already sits on a qualified bound, rank
   first.
2. Families with active stops (Kimi local server on Bash cwd, Antigravity on
   unbounded 502 retries, Kimi ACP on A2) or active deferrals (Gemini CLI) fail
   ranking until their named reopen conditions are met.
3. Recently closed families (Codex, Claude Code) are not reopened immediately.

### Top-Ranked Candidate: OpenCode HTTP server (`opencode.http`) `1.18.28`

Reasons:
- **Official Consensus**: npm and GitHub independently agree on the stable
  point `1.18.28` published on 2026-09-04.
- **Qualified Host Baseline**: Safe local discovery observes `opencode` `1.18.18`
  in `PATH`, which sits squarely within Swallowtail's qualified segment
  (`1.14.48..=1.18.20`), directly satisfying the preferred posture.
- **Runway Depth**: OpenCode has 8 published patch hops on npm
  (`1.18.21..=1.18.28`) above the `1.18.20` ceiling.
- **Clean Architectural Runway**: Unlike Kimi local server (stopped on Bash cwd
  authority in Research 282), Antigravity (stopped on unbounded HTTP 502 retries
  in Research 283), Kimi ACP (capped under A2), and Gemini CLI (operator
  deferral), OpenCode has no open stops, no contract violations, and no operator
  deferrals.
- **Post-Release Succession**: Both previously prioritized candidate lanes (Kimi
  local server and Antigravity) are stopped with ceilings held; OpenCode is the
  clear, unblocked candidate.

### Secondary Candidates

- **Cursor Agent** (`2026.09.02-c22c1a3`): Host `2026.08.04-aaa8809` is qualified;
  ceiling is `2026.08.11-e8db854`. Satisfies AllowUnverified-plus-qualified-host
  rule. Ranks directly behind OpenCode.
- **Claude Agent ACP** `0.74.0`: Single hop above recently closed `0.73.0`; host
  is `0.63.0` (deprecated qualified).
- **Pi RPC** `0.85.0`: Single hop above `0.84.4`; host is `0.83.0`.
- **Qwen Code** `0.23.0`: Minor line reset from `0.22.x`; host is `0.21.2`.
- **Codex** `0.153.2` and **Claude Code** `2.1.260`: Visible newer; both families
  were recently closed and are held.

## In-Run Latest Movement Note

When the candidate OpenCode lane is compiled and executed, if npm/GitHub latest
moves mid-run before the identity commit lands, the worker must follow the
Contract 029 In-Run Latest Movement rule (as demonstrated during Card 062 /
Research 282): re-probe official latest, add each newly published stable point as
a further hop, recompute identity from official artifacts, and extend the ledger
without losing earlier hops. Stop and ask only on selected mapped surface/capability
change, major-line reset, or channel disagreement.

## Decision

Designate OpenCode HTTP server `1.18.28` as the top-ranked next material candidate
for compilation by Chatterbox.

This checkpoint changes no claim, selection, matrix, fixture, or route. It does
not compile identity or claim cards. It keeps:
- Kimi ACP capped at `0.38.0` under A2;
- Kimi local server stopped at `0.38.0` under Research 282;
- Antigravity stopped at `1.1.17` under Research 283;
- Gemini CLI requalification deferred;
- Claude Code watcher exact `2.1.251`; and
- all other qualified points and boundaries intact.

## Repository Evidence

- [Production solution feature matrix](../guides/provider-solution-feature-matrix.csv)
  — 41 current solution rows.
- Current adapter claims in `crates/swallowtail-adapter-*/src/selection.rs`
  and route-specific selection modules.
- [Contract 029](../contracts/029-interface-version-qualification-and-compatibility.md),
  [Contract 017](../contracts/017-provider-owned-session-load-replay-and-host-containment.md), and
  [Contract 023](../contracts/023-harness-operation-isolation-and-native-boundary.md).
- [Research 276](./276-all-route-version-currentness-checkpoint.md),
  [Research 281](./281-v0-4-0-compatibility-and-freeze-audit.md), and
  [Research 282](./282-kimi-code-local-server-0-41-0-identity.md).

## Sources

### Package and release channels

- [Qwen Code npm](https://registry.npmjs.org/@qwen-code%2Fqwen-code/latest)
- [Bedrock Runtime crates.io](https://crates.io/crates/aws-sdk-bedrockruntime) and [Bedrock crates.io](https://crates.io/crates/aws-sdk-bedrock)
- [Claude Agent ACP npm](https://registry.npmjs.org/@agentclientprotocol%2Fclaude-agent-acp/latest) and [releases](https://github.com/agentclientprotocol/claude-agent-acp/releases)
- [Claude Agent SDK npm](https://registry.npmjs.org/@anthropic-ai%2Fclaude-agent-sdk/latest)
- [Claude Code npm](https://registry.npmjs.org/@anthropic-ai%2Fclaude-code/latest) and [`v2.1.260`](https://github.com/anthropics/claude-code/releases/tag/v2.1.260)
- [Pi npm](https://registry.npmjs.org/@earendil-works%2Fpi-coding-agent/latest) and [Pi releases](https://github.com/badlogic/pi-mono/releases)
- [Cline npm](https://registry.npmjs.org/cline/latest), [Command Code npm](https://registry.npmjs.org/command-code/latest), and [Copilot npm](https://registry.npmjs.org/@github%2Fcopilot/latest)
- [Cursor ACP registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json), [ACP schema releases](https://github.com/agentclientprotocol/agent-client-protocol/releases), and [Cursor ACP documentation](https://cursor.com/docs/cli/acp)
- [DeepSeek Harness npm](https://registry.npmjs.org/@deepseek-ai%2Fdsh/latest), [Deep Agents ACP npm](https://registry.npmjs.org/deepagents-acp/latest), and [Qoder npm](https://registry.npmjs.org/@qoder-ai/qodercli/latest)
- [Antigravity `1.1.26`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.1.26), [Gemini CLI `0.58.0`](https://github.com/google-gemini/gemini-cli/releases/tag/v0.58.0), and [Gemini CLI npm](https://registry.npmjs.org/@google%2Fgemini-cli/latest)
- [Goose `v1.49.0`](https://github.com/aaif-goose/goose/releases/tag/v1.49.0), [Kiro stable manifest](https://prod.download.cli.kiro.dev/stable/latest/manifest.json), [llama.cpp `v0.3.0`](https://github.com/ggml-org/llama.cpp/releases/tag/v0.3.0), and [Ollama `v0.33.3`](https://github.com/ollama/ollama/releases/tag/v0.33.3)
- [Mistral Vibe `v2.25.0`](https://github.com/mistralai/mistral-vibe/releases/tag/v2.25.0) and [PyPI](https://pypi.org/project/mistral-vibe/)
- [Kimi Code npm](https://registry.npmjs.org/@moonshot-ai/kimi-code/latest) and [`@0.41.0`](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.41.0)
- [Oh My Pi npm](https://registry.npmjs.org/@oh-my-pi%2Fpi-coding-agent/latest), [Ollama releases](https://github.com/ollama/ollama/releases), [OpenCode npm](https://registry.npmjs.org/opencode-ai/latest), and [OpenCode `v1.18.28`](https://github.com/anomalyco/opencode/releases/tag/v1.18.28)
- [Codex npm](https://registry.npmjs.org/@openai/codex/latest) and [`rust-v0.153.2`](https://github.com/openai/codex/releases/tag/rust-v0.153.2)
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
  2026-09-04;
- current `selection.rs` claim constants and segments;
- current [provider-solution feature matrix](../guides/provider-solution-feature-matrix.csv);
- frozen currentness records [127](./127-all-route-version-currentness-checkpoint.md),
  [159](./159-post-harness-expansion-version-currentness-checkpoint.md),
  [263](./263-all-route-version-currentness-checkpoint.md),
  [265](./265-all-route-version-currentness-checkpoint.md),
  [267](./267-all-route-version-currentness-checkpoint.md),
  [269](./269-all-route-version-currentness-checkpoint.md),
  [271](./271-all-route-version-currentness-checkpoint.md),
  [274](./274-all-route-version-currentness-checkpoint.md), and
  [276](./276-all-route-version-currentness-checkpoint.md);
- the merged [Kimi Code local server `0.41.0` identity record](./282-kimi-code-local-server-0-41-0-identity.md); and
- the merged [v0.4.0 compatibility and freeze audit record](./281-v0-4-0-compatibility-and-freeze-audit.md).
