# 159 Post-Harness-Expansion Version Currentness Checkpoint

Status: promoted
Owner: Tom
Date: 2026-08-19
Card: g03 evidence-gate reassessment after 097

## Question

After harness-route expansion closeout, do current official stable points
and this host's installed versions justify raising any Swallowtail
qualified bound, compiling one family through the Upgrade Workflow, or
leaving g03 evidence-gated?

## Method

Followed Contract 029's Recurring Currentness Checkpoint and
`docs/guides/version-currentness-checkpoint.md`. Compared:

- adapter `selection.rs` claims and the production feature-matrix version
  columns
- safe local `--version` for tools on `PATH`
- official npm `latest`, GitHub latest stable releases/tags, crates.io
  max stable, PyPI, the Kiro installer manifest, and ACP registry
  discovery metadata
- Research 091 / 127 result vocabulary

Observation time is 2026-08-19. The previous full checkpoint is Research
127 (2026-08-17). g03.072–g03.085 then qualified the 127 AllowUnverified
families except deferred Gemini. g03.086–g03.097 added production routes
that 127 did not cover.

No provider prompt, authentication, catalogue, session, install, host
update, consumer edit, claim edit, or workspace `qa`. Preview, nightly,
alpha, and development channels do not change stable truth. Registry
membership is not a Swallowtail claim. Missing local install is not a
gap.

## Compatibility Result

Result vocabulary:

- `unchanged` — local and official points still sit on the qualified bound
- `visible unverified-newer` — a later stable exists; AllowUnverified
  already classifies it; no bound change in this checkpoint
- `record only; future range work deferred` — newer point exists, but
  extension needs a dedicated family card, an exact-pin reopen, or an
  existing deferral still holds
- `material candidate` — enough evidence to ask before compiling a
  single-family range card; this checkpoint does not compile one

A `visible unverified-newer` row is not permission to skip the family.
Do not leave the current host or official stable UnverifiedNewer without
a named incompatible reason.

### Installed and attached harnesses

| Surface | Local observation | Current external point | Swallowtail boundary | Result |
| --- | --- | --- | --- | --- |
| Codex exec / app-server | `codex-cli 0.147.0` | npm `@openai/codex` `0.148.0` (2026-08-18); alpha `0.149.0-alpha.1` ignored; `0.147.1` unpublished | `codex.cli` through `0.147.0`; AllowUnverified | material candidate; host sits on the qualified ceiling |
| Claude Agent ACP | `claude-agent-acp 0.63.0` | npm/GitHub `0.70.0` (2026-08-18); registry lists `0.70.0` | `0.53.0..=0.69.0` excluding `0.58.0`; AllowUnverified | visible unverified-newer |
| Claude Code headless | `2.1.235 (Claude Code)` | npm `@anthropic-ai/claude-code` `2.1.235` (2026-08-18) | `2.1.220..=2.1.234`; AllowUnverified | visible unverified-newer |
| Claude Code response-only | same CLI `2.1.235` | same npm `2.1.235` | `2.1.227..=2.1.234`; later stable provisional UnverifiedNewer | visible unverified-newer |
| Gemini ACP | `gemini 0.53.0` | npm `@google/gemini-cli` `0.55.1` (2026-08-11) | exact `0.51.0` | record only; Gemini requalification remains deferred |
| Gemini headless | `0.53.0` | `0.55.1` | `0.51.0..=0.52.0`; AllowUnverified | record only; same deferral as ACP |
| Grok Build ACP | `grok 1.0.5 (5115b46bc909) [stable]` | npm `@xai-official/grok` latest `1.0.5` (2026-08-16); dist-tag `alpha` `1.0.6` ignored; registry lists `1.0.6` | deprecated `0.2.114..=0.2.117`; maintained exact `1.0.4`; AllowUnverified | visible unverified-newer; do not treat alpha/registry `1.0.6` as latest |
| Kimi ACP / headless / local-server | `kimi 0.34.0` | npm `@moonshot-ai/kimi-code` `0.37.2` (2026-08-18) | ACP exact `0.28.1` plus `0.29.0..=0.36.1`; headless and local-server `0.29.0..=0.36.1`; AllowUnverified | visible unverified-newer |
| OpenCode HTTP | `opencode 1.18.18` | npm `opencode-ai` `1.18.18` | published segments through `1.18.18`; AllowUnverified | unchanged |
| Pi RPC | `pi 0.83.0` | npm `@earendil-works/pi-coding-agent` `0.84.2` | exact published points through `0.84.2`; unpublished `0.83.1` incompatible; AllowUnverified | unchanged |
| Qwen headless | `qwen 0.21.2` | npm `@qwen-code/qwen-code` `0.21.14` (2026-08-19) | `0.19.11..=0.20.1` and `0.21.0..=0.21.13`; AllowUnverified | visible unverified-newer |
| Antigravity | `agy 1.1.9` | GitHub `google-antigravity/antigravity-cli` `1.1.15` (2026-08-19) | `1.1.9..=1.1.14`; `1.1.8` independently unqualified; AllowUnverified | visible unverified-newer; g03.085's synthetic later fixture was `1.1.15` and that tag now exists |
| Cursor Agent | `cursor-agent 2026.08.04-aaa8809` | ACP registry Cursor `2026.08.11` | exact `2026.07.01-41b2de7`, `2026.07.23-e383d2b`, `2026.08.04-aaa8809`, `2026.08.11-e8db854`; AllowUnverified | unchanged; host sits on an inner qualified build |
| Oh My Pi RPC | `omp/17.2.15` | npm `@oh-my-pi/pi-coding-agent` `17.3.8` (2026-08-19); `17.3.6` unpublished | `17.2.9..=17.3.7`; AllowUnverified | visible unverified-newer; keep the unpublished `17.3.6` gap |
| Command Code | `command-code 1.15.1` | npm `command-code` `1.28.1` | exact `1.15.1`; QualifiedOnly | record only; do not reopen the exact pin |
| Muse Code | `Muse Code 0.2.1 (0.2.1-R1215.1)` | no public npm latest for the signed payload | exact opaque `0.2.1-R1215.1`; QualifiedOnly | unchanged on the payload axis |
| DeepSeek Harness JSON-RPC | `dsh` not on PATH | npm `@deepseek-ai/dsh` `0.1.0-rc.7` | exact runtime-bin `0.1.0rc6`; QualifiedOnly | record only |
| DeepSeek Harness Web | `dsh` not on PATH | same npm `0.1.0-rc.7` | exact `0.1.0-rc.6`; QualifiedOnly | record only |
| ZCode app-server | no `zcode` on PATH | npm `zcode-app-cli` `3.7.7-14` | exact `zcode.runtime` `0.16.3`; QualifiedOnly | unchanged on the runtime axis; do not flatten desktop/npm `3.7.7` onto `0.16.3` |
| Cline ACP / headless | not on PATH | npm `cline` `3.0.55` | exact `3.0.55`; QualifiedOnly | unchanged |
| Goose ACP | not on PATH | GitHub `block/goose` `v1.46.0`; `v2.0` RCs ignored | exact `1.46.0`; QualifiedOnly | unchanged |
| Copilot CLI ACP | not on PATH | npm `@github/copilot` `1.0.80`; registry `1.0.80` | exact `1.0.80`; QualifiedOnly | unchanged |
| Mistral Vibe headless | not on PATH | GitHub/PyPI `2.24.2` | exact `2.24.2`; QualifiedOnly | unchanged |
| Qoder headless | not on PATH | npm `@qoder-ai/qodercli` `1.1.26` (2026-08-19) | exact `1.1.25`; QualifiedOnly | record only; do not reopen the exact pin |
| Kiro ACP | not on PATH | installer manifest `https://prod.download.cli.kiro.dev/stable/latest/manifest.json` still `2.18.1` | exact `2.18.1`; QualifiedOnly | unchanged |
| Deep Agents ACP | not on PATH | npm `deepagents-acp` `0.1.25`; registry still lists `0.1.7` | exact `0.1.25`; QualifiedOnly | unchanged; do not bind the stale registry version |
| OpenHands Agent Server | n/a | PyPI `openhands-agent-server` `1.42.1` | package exists; **not** a production route | record only; live HTTP/WebSocket stays unwired |

### Local model runtimes

| Surface | Local observation | Current external point | Swallowtail boundary | Result |
| --- | --- | --- | --- | --- |
| Ollama attached | `ollama version is 0.32.14` | GitHub `ollama/ollama` `v0.32.14` | `0.14.0..=0.32.14`; exclusions `0.32.2` and `0.32.10`; AllowUnverified | unchanged |
| llama.cpp attached | `llama-server` build `10450` / commit `ece963f41` (observation only) | GitHub `ggml-org/llama.cpp` `b10502` (2026-08-19) | exact `b9910/f5525f7e7`; QualifiedOnly | record only; do not flatten the host `llama-server` onto the attached pin |
| llama.cpp owned | not re-probed | same `b10502` | exact `b10069/178a6c449`; QualifiedOnly | record only |

### Shared ACP

| Surface | Local observation | Current external point | Swallowtail boundary | Result |
| --- | --- | --- | --- | --- |
| stable ACP schema | local wrapper evidence only | GitHub `agentclientprotocol/agent-client-protocol` latest release still `schema-v1.20.0` (2026-07-21) | schema `v1.20.0`; wire v1 | unchanged |
| ACP agent registry | n/a | registry `version` `1.0.0`; 38 agents | discovery metadata, not a Swallowtail claim | record only; Research 158 already closed watchlist import |

### Hosted APIs and SDKs

Opaque facade pins. No live API call. "Latest model" is ignored.

| Surface | Local observation | Current external point | Swallowtail boundary | Result |
| --- | --- | --- | --- | --- |
| Anthropic Messages | n/a | still the public `anthropic-version: 2023-06-01` header family | exact `anthropic-2023-06-01`; QualifiedOnly | unchanged |
| Anthropic Managed Agents | n/a | no replacement beta id observed | exact `managed-agents-2026-04-01` | unchanged |
| OpenAI Realtime | n/a | no new facade id observed | exact `openai-realtime-2026-07-22` | unchanged |
| OpenAI Background Responses | n/a | no new facade id observed | exact `openai-responses-background-2026-07-21` | unchanged |
| DeepSeek Open Platform | n/a | no new facade id observed | exact `deepseek-openai-chat-2026-07-22` | unchanged |
| Kimi Platform Chat | n/a | no new facade id observed | exact `kimi-platform-chat-2026-07-21` | unchanged |
| Alibaba Model Studio | n/a | no new facade id observed | exact `model-studio-2026-07-22` | unchanged |
| Gemini Live | n/a | still `v1beta` `BidiGenerateContent` | exact `google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent` | unchanged |
| xAI Responses WebSocket | n/a | no new facade id observed | exact `xai-responses-websocket-2026-04-23` | unchanged |
| Bedrock Runtime / catalogue | n/a | crates.io `aws-sdk-bedrockruntime` `1.140.0`, `aws-sdk-bedrock` `1.151.0` | claimed SDK constants `1.136.0` / `1.148.0`; Cargo pins `=1.139.0` / `=1.150.0`; QualifiedOnly | record only; same Cargo-versus-constant mismatch as Research 127 |

## Rank after this record

Prefer AllowUnverified families whose official stable is newer than the
qualified ceiling. Skip Gemini. Skip exact-pin / QualifiedOnly reopen.
Prefer a host that already sits on a qualified bound.

1. **Codex `0.148.0`** — host is exactly qualified `0.147.0`; official
   latest is the next published stable; `0.147.1` unpublished
2. Claude Agent ACP `0.70.0`
3. Claude Code `2.1.235` (headless and response-only stay one family)
4. Grok Build `1.0.5` (not alpha `1.0.6`)
5. Qwen `0.21.14`
6. Kimi Code `0.37.2`
7. Oh My Pi `17.3.8`
8. Antigravity `1.1.15`

Unchanged AllowUnverified: OpenCode `1.18.18`, Pi `0.84.2`, Ollama
`0.32.14`, Cursor `2026.08.11`.

## Decision

Do not bulk-bump. Do not edit claims in this checkpoint. Do not promote
watchlist candidates, Aider, `kiro.headless`, or Gemini.

Compile Codex `0.148.0` next through the Upgrade Workflow, one family.
Remaining rows stay one-family-later. Exact-pin drift (Qoder `1.1.26`,
Command Code `1.28.1`, DeepSeek `0.1.0-rc.7`, llama.cpp `b10502`) stays
closed unless the operator asks to reopen a pin.

Qualified bounds did not change. Current source stays 40 packages and 47
production routes. Immutable `v0.3.2` stays 30 packages and 36 routes.

No consumer-reproduced Swallowtail defect was in scope this pass.

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
- [Cline npm metadata](https://registry.npmjs.org/cline/latest)
- [Copilot CLI npm metadata](https://registry.npmjs.org/@github%2Fcopilot/latest)
- [Qoder CLI npm metadata](https://registry.npmjs.org/@qoder-ai%2Fqodercli/latest)
- [Deep Agents ACP npm metadata](https://registry.npmjs.org/deepagents-acp/latest)
- [Claude Agent ACP releases](https://github.com/agentclientprotocol/claude-agent-acp/releases)
- [ACP releases](https://github.com/agentclientprotocol/agent-client-protocol/releases)
- [Antigravity CLI tags](https://github.com/google-antigravity/antigravity-cli/tags)
- [Goose releases](https://github.com/block/goose/releases)
- [Mistral Vibe releases](https://github.com/mistralai/mistral-vibe/releases)
- [Ollama releases](https://github.com/ollama/ollama/releases)
- [llama.cpp releases](https://github.com/ggml-org/llama.cpp/releases)
- [Kiro installer manifest](https://prod.download.cli.kiro.dev/stable/latest/manifest.json)
- [ACP agent registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)
- crates.io `aws-sdk-bedrockruntime` and `aws-sdk-bedrock`
- PyPI `mistral-vibe` and `openhands-agent-server`
- local safe `--version` observations
- Research 127, 158; g03.072–g03.097
