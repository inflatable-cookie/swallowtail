# 127 All-Route Version Currentness Checkpoint

Status: promoted
Owner: Tom
Date: 2026-08-17

## Question

Do current official stable points, and the versions this host actually has
installed, justify raising any Swallowtail qualified bound, compiling a new
g03 range-maintenance milestone, or leaving g03 evidence-gated?

## Method

The checkpoint compared:

- adapter `selection.rs` claims and the production feature-matrix version
  columns
- safe local `--version` for tools on `PATH`
- official npm `latest`, GitHub latest stable releases/tags, crates.io max
  stable, and the ACP agent registry
- Research 089 / 091 result vocabulary

It ran no provider prompt, authentication flow, model catalogue, session
operation, install, update, consumer edit, or workspace `qa`.

Preview, nightly, alpha, and development channels do not change stable
compatibility truth. Semver or "latest" alone does not extend a range.
Hosted API "latest model" is not a Swallowtail compatibility axis.

Observation time is 2026-08-17. The previous full checkpoint is Research 091
(2026-07-31). Routes added after 091 are in scope: Oh My Pi, Muse Code,
Command Code, Claude Code response-only, DeepSeek Harness JSON-RPC, DeepSeek
Harness Web, and ZCode.

## Compatibility Result

Result vocabulary:

- `unchanged` — local and official points still sit on the qualified bound
- `visible unverified-newer` — a later stable point exists; AllowUnverified
  already classifies it; no bound change
- `record only; future range work deferred` — newer point exists, but
  extension needs a dedicated family card or an existing deferral still holds
- `material candidate` — enough evidence to *ask* before compiling a
  single-family range card; this checkpoint does not compile one

### Installed and attached harnesses

| Surface | Local observation | Current external point | Swallowtail boundary | Result |
| --- | --- | --- | --- | --- |
| Codex exec / app-server | `codex-cli 0.147.0` | npm `@openai/codex` `0.147.0` (published 2026-08-07; was alpha in Research 074/091) | `codex.cli` through `0.146.0`; gaps `0.82.0..=0.83.x`, `0.108.0`/`0.109.0`; AllowUnverified | material candidate for a dedicated `0.147.0` range card; bound stays `0.146.0` |
| Claude Agent ACP | `claude-agent-acp 0.63.0` | npm/GitHub `0.69.0` (2026-08-16); ACP registry lists Claude Agent `0.69.0` | `0.53.0..=0.64.0` excluding `0.58.0`; AllowUnverified | visible unverified-newer |
| Claude Code headless | `2.1.233` | npm `@anthropic-ai/claude-code` `2.1.234` (2026-08-17) | exact `2.1.220` | record only; future range work deferred |
| Claude Code response-only | same CLI `2.1.233` | same npm `2.1.234` | qualified `2.1.227..=2.1.228`; empty deny-list; later stable provisional UnverifiedNewer | visible unverified-newer |
| Gemini ACP | `gemini 0.53.0` | npm `@google/gemini-cli` `0.55.1` | exact `0.51.0` | record only; Gemini requalification remains deferred |
| Gemini headless | `0.53.0` | `0.55.1` | `0.51.0..=0.52.0`; AllowUnverified | record only; same deferral as ACP |
| Grok Build ACP | `grok 1.0.4 (d846eb93d94d) [stable]` | npm `@xai-official/grok` `1.0.4` (2026-08-13); `0.2.121` still published; alpha `1.0.5` ignored | `grok-build.executable` `0.2.114..=0.2.117`; AllowUnverified | material candidate for identity investigation; do not treat `1.0.x` as `0.2` compatible |
| Kimi ACP / headless / local-server | `kimi 0.34.0` | npm `@moonshot-ai/kimi-code` `0.36.1` | ACP exact `0.28.1` plus `0.29.0..=0.31.1`; headless `0.29.0..=0.31.1`; local-server exact `0.28.1` plus `0.29.0..=0.31.1`; AllowUnverified | visible unverified-newer |
| OpenCode HTTP | `opencode 1.18.18` | npm `opencode-ai` `1.18.18` | published segments through `1.18.10`, not one closed interval; AllowUnverified | visible unverified-newer |
| Pi RPC | `pi 0.83.0` | npm `@earendil-works/pi-coding-agent` `0.84.2` | exact published points `0.80.10` through `0.83.0`; AllowUnverified | visible unverified-newer |
| Qwen headless | `qwen 0.21.2` | npm `@qwen-code/qwen-code` `0.21.13` | `0.19.11..=0.20.1` and `0.21.0..=0.21.2`; AllowUnverified | visible unverified-newer |
| Antigravity | `agy 1.1.9` | GitHub `google-antigravity/antigravity-cli` `1.1.13` (2026-08-14); tags `1.1.13`..`1.1.9` | exact `1.1.9`; AllowUnverified | visible unverified-newer |
| Cursor Agent | `cursor-agent 2026.08.04-aaa8809` | ACP registry Cursor `2026.08.11`; npm `cursor-agent@1.0.3` is a different axis | exact `2026.07.01-41b2de7` and `2026.07.23-e383d2b`; no inferred gap; AllowUnverified | visible unverified-newer |
| Oh My Pi RPC | `omp/17.2.15` | npm `@oh-my-pi/pi-coding-agent` `17.3.5` | exact `17.2.9`; AllowUnverified | visible unverified-newer |
| Command Code | `command-code 1.15.1` | npm `command-code` `1.26.0` | exact `1.15.1`; QualifiedOnly | record only; future range work deferred |
| Muse Code | `Muse Code 0.2.1 (0.2.1-R1215.1)` | no public npm latest for the signed payload | exact opaque `0.1.0-R708.1`; QualifiedOnly | record only; host payload moved; future range work deferred |
| DeepSeek Harness JSON-RPC | `dsh` not on PATH | npm `@deepseek-ai/dsh` `0.1.0-rc.7` (2026-08-17) | exact runtime-bin `0.1.0rc6`; QualifiedOnly | record only; do not flatten onto Web `0.1.0-rc.6` |
| DeepSeek Harness Web | `dsh` not on PATH | same npm `0.1.0-rc.7` | exact `0.1.0-rc.6`; QualifiedOnly; claim already denies `0.1.0-rc.7` | record only; future range work deferred |
| ZCode app-server | no `zcode` on PATH | npm `zcode-app-cli` still `3.7.7-13` | exact `zcode.runtime` `0.16.3`; QualifiedOnly | unchanged on the runtime axis; do not flatten desktop/npm `3.7.7` onto `0.16.3` |

### Local model runtimes

| Surface | Local observation | Current external point | Swallowtail boundary | Result |
| --- | --- | --- | --- | --- |
| Ollama attached | `ollama version is 0.32.9` | GitHub `ollama/ollama` `v0.32.14` (2026-08-15) | `0.14.0..=0.32.1`; exclusion `0.32.2`; AllowUnverified | visible unverified-newer |
| llama.cpp attached | not re-probed | GitHub `ggml-org/llama.cpp` `b10472` (2026-08-17) | exact `b9910/f5525f7e7`; QualifiedOnly | record only; future range work deferred |
| llama.cpp owned | not re-probed | same `b10472` | exact `b10069/178a6c449`; QualifiedOnly | record only; future range work deferred |

### Shared ACP

| Surface | Local observation | Current external point | Swallowtail boundary | Result |
| --- | --- | --- | --- | --- |
| stable ACP schema | local wrapper evidence only | GitHub `agentclientprotocol/agent-client-protocol` latest release still `schema-v1.20.0` (2026-07-21) | schema `v1.20.0`; wire v1 | unchanged |
| ACP agent registry | n/a | `cdn.agentclientprotocol.com/registry/v1/latest/registry.json` `version` `1.0.0`; Claude Agent `0.69.0`; Cursor `2026.08.11` | registry is discovery metadata, not a Swallowtail claim | record only; does not move schema `v1.20.0` |

### Hosted APIs and SDKs

Opaque facade pins. No live API call. "Latest model" is ignored.

| Surface | Local observation | Current external point | Swallowtail boundary | Result |
| --- | --- | --- | --- | --- |
| Anthropic Messages | n/a | still the public `anthropic-version: 2023-06-01` header family | exact `anthropic-2023-06-01`; QualifiedOnly | unchanged; no new facade revision evidence |
| Anthropic Managed Agents | n/a | no replacement beta id observed | exact `managed-agents-2026-04-01` | unchanged |
| OpenAI Realtime | n/a | no new facade id observed | exact `openai-realtime-2026-07-22` | unchanged |
| OpenAI Background Responses | n/a | no new facade id observed | exact `openai-responses-background-2026-07-21` | unchanged |
| DeepSeek Open Platform | n/a | no new facade id observed | exact `deepseek-openai-chat-2026-07-22` | unchanged |
| Kimi Platform Chat | n/a | no new facade id observed | exact `kimi-platform-chat-2026-07-21` | unchanged |
| Alibaba Model Studio | n/a | no new facade id observed | exact `model-studio-2026-07-22` | unchanged |
| Gemini Live | n/a | still `v1beta` `BidiGenerateContent` | exact `google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent` | unchanged |
| xAI Responses WebSocket | n/a | no new facade id observed | exact `xai-responses-websocket-2026-04-23` | unchanged |
| Bedrock Runtime / catalogue | n/a | crates.io `aws-sdk-bedrockruntime` `1.140.0`, `aws-sdk-bedrock` `1.151.0` | claimed SDK constants `1.136.0` / `1.148.0`; Cargo pins `=1.139.0` / `=1.150.0`; service facades `bedrock-converse-stream` / `bedrock-list-foundation-models`; QualifiedOnly | record only; Cargo pins sit above the claimed SDK constants; crates.io is one patch above Cargo. Not a hosted API revision |

## Notes On The Two Material Candidates

### Codex `0.147.0`

Research 091 recorded stable npm `0.146.0` and treated `0.147.0` as alpha.
It is now the official `latest` and this host's installed CLI. Nucleus already
ran `0.147.0` as UnverifiedNewer; g03.047/048 repaired malformed inbound and
collab spawn admission against that live surface. Raising the qualified bound
from `0.146.0` to `0.147.0` would be useful-newer-support work, not a silent
semver bump, and still needs a dedicated range card with corpus/claim updates.
This checkpoint does not compile that card.

### Grok `1.0.x`

`@xai-official/grok` kept the same `grok` bin. After qualified `0.2.117`
(2026-07-30) it published `0.2.118`–`0.2.121`, then reset to `1.0.0` on
2026-08-07. This host already runs `1.0.4`. AllowUnverified would classify
`1.0.4` as UnverifiedNewer on the same semantic axis. That is too generous
for a major-line reset until ACP handshake evidence exists. Do not extend
`0.2.114..=0.2.117` across the reset. Do not flatten `1.0.x` onto Grok Build
`0.2` behavior. Qualify `1.0.x` as a same-axis milestone segment after
handshake corpus (g03.073 card 234).

## Decision

Do not compile a catch-all g03 range-maintenance milestone. Keep family
qualification evidence-gated.

The operator then promoted the sweep into a named recurring process:
Contract 029's Recurring Currentness Checkpoint, g03.072, and
`docs/guides/version-currentness-checkpoint.md`. Codex `0.147.0` is the
first family card. Remaining rows stay one-family-later.

Qualified bounds did not change in the checkpoint itself.

## Contract Result

Contract 029 now names the recurring checkpoint. No compatibility-claim
membership, package count, or route inventory changed in the checkpoint.
Immutable `v0.3.2` stays 30 packages / 36 routes. Current source stays 32
packages / 39 routes from g03.071.

The Bedrock Cargo-versus-constant mismatch remains recorded, not repaired.

## Sources

- [Codex npm metadata](https://registry.npmjs.org/@openai%2Fcodex/latest)
- [Claude Code npm metadata](https://registry.npmjs.org/@anthropic-ai%2Fclaude-code/latest)
- [Gemini CLI npm metadata](https://registry.npmjs.org/@google%2Fgemini-cli/latest)
- [Kimi Code npm metadata](https://registry.npmjs.org/@moonshot-ai%2Fkimi-code/latest)
- [Grok npm metadata](https://registry.npmjs.org/@xai-official%2Fgrok/latest)
- [Pi npm metadata](https://registry.npmjs.org/@earendil-works%2Fpi-coding-agent/latest)
- [Qwen Code npm metadata](https://registry.npmjs.org/@qwen-code%2Fqwen-code/latest)
- [OpenCode npm metadata](https://registry.npmjs.org/opencode-ai/latest)
- [Oh My Pi npm metadata](https://registry.npmjs.org/@oh-my-pi%2Fpi-coding-agent/latest)
- [Command Code npm metadata](https://registry.npmjs.org/command-code/latest)
- [DeepSeek Harness npm metadata](https://registry.npmjs.org/@deepseek-ai%2Fdsh/latest)
- [ZCode packaging npm metadata](https://registry.npmjs.org/zcode-app-cli/latest)
- [Claude Agent ACP npm metadata](https://registry.npmjs.org/@agentclientprotocol%2Fclaude-agent-acp/latest)
- [Claude Agent ACP releases](https://github.com/agentclientprotocol/claude-agent-acp/releases)
- [ACP releases](https://github.com/agentclientprotocol/agent-client-protocol/releases)
- [Antigravity CLI tags](https://github.com/google-antigravity/antigravity-cli/tags)
- [Ollama releases](https://github.com/ollama/ollama/releases)
- [llama.cpp releases](https://github.com/ggml-org/llama.cpp/releases)
- [ACP agent registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)
- crates.io `aws-sdk-bedrockruntime` and `aws-sdk-bedrock`
- local safe `--version` observations
- Research 089, 091, 074
