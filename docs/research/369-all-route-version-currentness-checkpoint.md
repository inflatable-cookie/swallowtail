# Research 369: All-Route Version Currentness Checkpoint

Status: research checkpoint complete; official channels reconciled; no compatibility claim, fixture, or matrix change

Owner: standing version-currentness lane
Date: 2026-10-07
Observation window: 2026-10-07 16:33–16:53 UTC
Base: canonical `main` `257bbd02c5b5dbbcb5801af8edb56bdbf0274ec5`
Authority: Contract 029; version-currentness checkpoint procedure; Queue brief
for task 093

## Question

What is the current official stable point, local observation, qualified bound,
and next evidence gate for every production Swallowtail route and separate
interface axis before preparation of v0.5.2?

Answer: the current route inventory reconciles to 51 distinct production route
IDs. Official versions newer than current qualification, local versions newer
than their route bound, exact-pin gates, major-line resets, and source gaps are
listed below. For Claude Code, the selected official channel is npm `latest`,
which agrees with GitHub's latest non-prerelease release at `2.1.292`. The npm
`stable` dist-tag remains `2.1.285`; Anthropic documents it as a deliberately
delayed channel, typically about one week behind `latest`. These are separate
channels, not conflicting reports of the same channel. All newer points remain
unqualified until separate family tasks complete identity and route-specific
qualification. This checkpoint changes no claim and authorizes no bulk update.

## Method and limits

Re-read the production feature matrix, route guide, adapter selection claims,
Contracts 015 and 029, the version-currentness procedure, Research 308, and
later qualification records through Research 367 for the original capture.
Counted route IDs by splitting the current CSV's `;` and `+` route separators;
all 51 IDs are unique. Re-read
local command versions only where the earlier safe, promptless observation was
available. Public registry, release, manifest, and documentation metadata were
read only. No provider prompt, live catalogue, login, credential, installation,
host update, or downloaded-artifact execution was used.

Before resubmission, refreshed canonical `main` to
`d338af132c224ac3fbcdae5f2b18207217ef0185` at 2026-10-07 18:04 UTC and
reconciled the merged Research 368 SDK usage/context evidence. The current
feature CSV still has 51 unique production route IDs; PR #412 changes
`claude-agent.sdk` `usage_evidence` from `No` to `Yes`. Research 368 records
that evidence on the frozen SDK `0.3.284`, adds no version claim, and does not
transfer registered-tool live acceptance. The SDK currentness row therefore
keeps its exact `0.3.284` ceiling, current package `0.3.292`, and separate
artifact-identity and registered-tool gates. This checkpoint is Research 369
because refreshed canonical `main` already contains Research 368.

The observation cutoff is 2026-10-07 16:53 UTC; Claude Code npm and GitHub
metadata were re-probed at 16:53 UTC. Registry or release metadata discovers a
point and its publication ordering; it does not qualify that point.
The “hops” below count published stable package/tag points after the exact
qualified point on that official channel. A gate marked “identity chain” still
requires the family task to inspect the exact selected artifacts and mapped
surface. No conclusion is drawn from a `latest` tag alone.

The lane's prior Gemini deferral is lifted; Gemini CLI is included at its
current qualified ceiling. Q-003 and Q-004 remain answered as recorded in
`questions.md`; Antigravity's retry-disabled `1.2.11` and Kimi local-server's
`2.1.1` evidence are retained. The family list does not reopen those rulings.

## Current route and interface inventory

One row appears for each production route ID. Separate siblings remain separate
even where they share a package or vendor channel. The machine-readable family
list is this table; every row points to this record, Research 369.

| Family / route | Qualified ceiling and posture | Official current point (publication date) | Package or interface axis; official channel | Local observation | Eligibility, classification, or gate | Research pointer |
| --- | --- | --- | --- | --- | --- | --- |
| `qwen.headless` | Maintained exact `0.21.15`, `0.22.0..=0.24.2` excluding `0.22.4`, `0.23.5`; later stable unverified | `0.25.0` (2026-10-05) | `@qwen-code/qwen-code`; npm latest | `qwen 0.21.2` | Six stable points after `0.24.2`; identity-chain task before extension. | [369](#current-route-and-interface-inventory) |
| `alibaba.conversations` | Exact opaque `model-studio-2026-07-22`; `QualifiedOnly` | No ordered public stable version; exact API facade | `alibaba.conversations-responses-facade`; official Model Studio API docs | Not applicable | No scalar “latest” exists; recheck exact facade docs in a family task. Matrix axis label differs from selection axis `alibaba-model-studio.responses-facade`; route and exact revision reconcile, but note the label drift for planner follow-up. | [369](#current-route-and-interface-inventory) |
| `bedrock.catalogue` | Exact SDK `1.150.0`; service model exact | `aws-sdk-bedrock 1.161.0` (2026-10-01) | `aws-sdk-bedrock`; crates.io stable | Not applicable | Eleven published stable crate points after `1.150.0`; SDK is the axis, not a Bedrock service API version. Identity task required. | [369](#current-route-and-interface-inventory) |
| `bedrock.runtime` | Exact SDK `1.139.0`; service model exact | `aws-sdk-bedrockruntime 1.148.0` (2026-10-01) | `aws-sdk-bedrockruntime`; crates.io stable | Not applicable | Nine published stable crate points after `1.139.0`; keep separate from catalogue SDK. Identity task required. | [369](#current-route-and-interface-inventory) |
| `claude-agent.acp` | Maintained `0.53.0..=0.81.2` excluding `0.58.0`; later stable unverified | `0.87.0` (2026-10-07) | `@agentclientprotocol/claude-agent-acp`; npm/GitHub and ACP registry agree | `claude-agent-acp 0.63.0` | Seven stable package hops; no stop to be inferred from metadata. Compare every selected hop in a one-family task. | [369](#current-route-and-interface-inventory) |
| `claude-agent.sdk` | Exact `0.3.284` package with native `2.1.284`; `QualifiedOnly` | SDK package `0.3.292` (2026-10-06); embedded native identity not established from packument metadata | `@anthropic-ai/claude-agent-sdk`; npm. `@anthropic-ai/claude-code@2.1.292` is a separate host CLI axis. | Node `22.23.2`; installed `claude 2.1.286` identifies the host CLI, not the embedded SDK | Eight SDK package hops after `0.3.284`; read the exact versioned platform artifact manifest and selected SDK surface before updating either package or native claim. Research 368 provides usage/context evidence for frozen `0.3.284` only; it adds no version claim and does not transfer Research 301's registered-tool live acceptance. | [369](#current-route-and-interface-inventory) |
| `claude-code.headless` | Maintained `2.1.220..=2.1.281`; excludes unpublished `2.1.244`, `.249`, `.253..=.256`, `.262`, `.264`, `.279`; newer selected-channel points are `UnverifiedNewer` | npm `latest` `2.1.292` and GitHub latest non-prerelease `v2.1.292` (2026-10-06); delayed npm `stable` tag `2.1.285` (2026-09-29) | `@anthropic-ai/claude-code`; npm latest and GitHub latest release agree | `claude 2.1.286` | Eleven published points `2.1.282`–`2.1.292` after the qualified ceiling. Metadata discovery only; qualify the headless route separately. npm `stable` is a distinct delayed channel, not a disagreement. | [369](#current-route-and-interface-inventory) |
| `claude-code.response-only` | Maintained `2.1.227..=2.1.278` on `stream-json.v1`, then `2.1.280..=2.1.281` on `stream-json.v2`; same unpublished holes; newer selected-channel points are `UnverifiedNewer` | npm `latest` `2.1.292` and GitHub latest non-prerelease `v2.1.292` (2026-10-06); delayed npm `stable` tag `2.1.285` (2026-09-29) | `@anthropic-ai/claude-code`; npm latest and GitHub latest release agree | `claude 2.1.286` | Eleven published points `2.1.282`–`2.1.292` after the qualified ceiling. Preserve the stream-json v1/v2 split and deny-list; qualify this response-only route separately. No evidence transfers from headless. | [369](#current-route-and-interface-inventory) |
| `anthropic.managed-agent` | Exact opaque `managed-agents-2026-04-01`; `QualifiedOnly` | No ordered public stable version; exact API facade | Anthropic Managed Agents API docs | Not applicable | No versioned release channel; recheck the exact beta facade and route controls. | [369](#current-route-and-interface-inventory) |
| `anthropic.messages` | Exact `anthropic-2023-06-01`; `QualifiedOnly` | No newer dated API version observed; exact Messages API revision | Anthropic Messages API docs | Not applicable | No ordered release stream; latest model aliases are out of scope. | [369](#current-route-and-interface-inventory) |
| `pi.rpc` | Maintained published points `0.80.10..=0.86.1`, excluding `0.83.1`, `0.84.5`, `0.85.2`; later stable unverified | `1.0.4` (2026-10-05) | `@earendil-works/pi-coding-agent`; npm/GitHub agree | `pi 0.87.1` | Ten stable points after `0.86.1`; major-line reset to `1.x`. The installed host and official point exceed the current bound. Identity/mapping task, no range inference. | [369](#current-route-and-interface-inventory) |
| `pi.sdk-sidecar` | Exact `0.84.2`; `QualifiedOnly` | `1.0.4` (2026-10-05) | `@earendil-works/pi-coding-agent`; npm/GitHub agree | Node `22.23.2` | Sixteen package points after `0.84.2`, including `1.0.0`; separate exact sidecar qualification from `pi.rpc`. | [369](#current-route-and-interface-inventory) |
| `cline.acp` | Exact `3.0.55`; `QualifiedOnly` | `3.0.69` (2026-10-07) | npm package `cline` | Missing from PATH | Thirteen stable package points; exact-pin reopening and per-hop selected ACP identity. GitHub Cline `v4.1.23` is the editor product, not this npm route. | [369](#current-route-and-interface-inventory) |
| `cline.headless` | Exact `3.0.55`; `QualifiedOnly` | `3.0.69` (2026-10-07) | npm package `cline` | Missing from PATH | Thirteen stable package points; preserve separate headless/ACP surfaces and exact-pin gate. | [369](#current-route-and-interface-inventory) |
| `command-code.headless` | Exact `1.65.0`; `QualifiedOnly` | `1.77.0` (2026-10-07) | npm package `command-code` | `command-code 1.65.0` | Twenty-nine stable points after the pin; exact-pin reopening, not automatic extension. | [369](#current-route-and-interface-inventory) |
| `cursor-agent.catalogue` | Exact published points `2026.07.01-41b2de7`, `2026.07.23-e383d2b`, `2026.08.04-aaa8809`, `2026.08.11-e8db854`, `2026.08.31-4057e58`, `2026.09.02-c22c1a3`, `2026.09.10-fd3934a`, `2026.09.15-d2fe57e`, `2026.09.18-9a7762b`; no inferred gap | `2026.10.01` (2026-10-01) | Official ACP registry entry/archive; version-date channel | `cursor-agent 2026.09.18-9a7762b` | Newer official entry is discovery only. Enumerate ordered official artifacts and hashes before stating a hop count or qualifying. | [369](#current-route-and-interface-inventory) |
| `cursor-agent.acp` | Exact published points `2026.07.01-41b2de7`, `2026.07.23-e383d2b`, `2026.08.04-aaa8809`, `2026.08.11-e8db854`, `2026.08.31-4057e58`, `2026.09.02-c22c1a3`, `2026.09.10-fd3934a`, `2026.09.15-d2fe57e`, `2026.09.18-9a7762b`; no inferred gap | `2026.10.01` (2026-10-01) | Official ACP registry entry/archive; version-date channel | `cursor-agent 2026.09.18-9a7762b` | Same release point, separate ACP mapping and qualification. Artifact history and selected ACP identity gate. | [369](#current-route-and-interface-inventory) |
| `cursor-agent.headless` | Exact published points `2026.07.01-41b2de7`, `2026.07.23-e383d2b`, `2026.08.04-aaa8809`, `2026.08.11-e8db854`, `2026.08.31-4057e58`, `2026.09.02-c22c1a3`, `2026.09.10-fd3934a`, `2026.09.15-d2fe57e`, `2026.09.18-9a7762b`; no inferred gap | `2026.10.01` (2026-10-01) | Official ACP registry entry/archive; version-date channel | `cursor-agent 2026.09.18-9a7762b` | Same release point, separate headless mapping and qualification. Artifact history and selected CLI identity gate. | [369](#current-route-and-interface-inventory) |
| `deepseek-harness.jsonrpc` | Exact prerelease `0.1.0rc6`; `QualifiedOnly` | No stable release; npm `latest`/`next` `0.2.0-rc.2` (2026-09-29), separate `alpha` `0.2.1-alpha.1` (2026-10-03) | `@deepseek-ai/dsh`; npm prerelease tags | Missing from PATH | Twenty-four published alpha/RC points follow normalized `0.1.0-rc.6`; enumerate and inspect exact runtime-bin chain before qualification. | [369](#current-route-and-interface-inventory) |
| `deepseek-harness.local-server` | Exact prerelease `0.1.0-rc.6`; `QualifiedOnly` | No stable release; npm `latest`/`next` `0.2.0-rc.2` (2026-09-29), separate `alpha` `0.2.1-alpha.1` (2026-10-03) | `@deepseek-ai/dsh`; npm prerelease tags | Missing from PATH | Same published package stream, distinct web surface and exact old pin; do not transfer JSON-RPC evidence. | [369](#current-route-and-interface-inventory) |
| `deepseek.continuation` | Exact opaque `deepseek-openai-chat-2026-07-22`; `QualifiedOnly` | No ordered public stable version; exact API facade | DeepSeek Open Platform API docs | Not applicable | No scalar release; model aliases and model `latest` are not interface versions. | [369](#current-route-and-interface-inventory) |
| `copilot-cli.acp` | Exact `1.0.80`; `QualifiedOnly` | `1.0.93` (2026-10-07) | `@github/copilot`; npm package | Missing from PATH | Thirteen stable package points after exact pin; package identity and ACP mapping task. | [369](#current-route-and-interface-inventory) |
| `antigravity.catalogue` | Maintained `1.1.9..=1.2.11` | `1.3.1` (2026-10-07) | Official Antigravity CLI GitHub releases | `agy` present; `--version` deliberately skipped because host CLI auto-updates | Eight stable release points after `1.2.11`; metadata is not catalogue qualification. Keep catalogue separate from headless. | [369](#current-route-and-interface-inventory) |
| `antigravity.headless` | Deprecated `1.1.9..=1.1.17`; exact `1.2.11` with `AGY_CLI_MODEL_API_MAX_RETRIES=0` pinned; `1.1.18..=1.2.10` unqualified | `1.3.1` (2026-10-07) | Official Antigravity CLI GitHub releases | `agy` present; version probe skipped as above | Eight stable points newer than the pinned ceiling. The retry ruling only qualifies `1.2.11`; current release needs an adaptation task and explicit retry-bound evidence. | [369](#current-route-and-interface-inventory) |
| `gemini-cli.acp` | Maintained `0.51.0..=0.61.0`, excluding `0.56.1`, `0.59.1`; later stable unverified | `0.63.0` (2026-10-06) | `@google/gemini-cli`; npm/GitHub agree | `gemini` present; `--version` skipped because host CLI auto-updates | Two stable hops (`0.62.0`, `0.63.0`). Prior Gemini deferral is lifted; qualify ACP independently. | [369](#current-route-and-interface-inventory) |
| `gemini-cli.headless` | Maintained `0.51.0..=0.61.0`, excluding `0.56.1`, `0.59.1`; later stable unverified | `0.63.0` (2026-10-06) | `@google/gemini-cli`; npm/GitHub agree | `gemini` present; version probe skipped as above | Two stable hops; separate headless mapping from ACP. Gemini deferral is not repeated. | [369](#current-route-and-interface-inventory) |
| `gemini.live` | Exact opaque `google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent.thinking-output-max-context-compression-2026-08-23`; `QualifiedOnly` | No ordered public stable version; exact Live API facade | Google Gemini Live API docs | Not applicable | Recheck the exact facade and request shape; ignore hosted model aliases. | [369](#current-route-and-interface-inventory) |
| `goose.acp` | Exact `1.50.1`; `QualifiedOnly` | `1.53.0` (2026-10-02) | Official Goose GitHub stable releases | Missing from PATH | Three stable points (`1.51.0`–`1.53.0`); exact ACP qualification task. Carry the HTTP MCP honouring gate as exact-point evidence only. | [369](#current-route-and-interface-inventory) |
| `kiro.acp` | Exact `2.21.4`; `QualifiedOnly` | CLI `2.28.0` (2026-10-05); changelog also lists patch `2.27.1` (2026-10-02) | Official Kiro CLI changelog | `kiro-cli` missing from PATH | Twelve published stable points after the pin: `2.22.0`, `2.22.1`, `2.23.0`, `2.23.1`, `2.24.0`, `2.24.1`, `2.25.0`, `2.26.0`, `2.26.1`, `2.27.0`, `2.27.1`, `2.28.0`. Count main and patch releases as separate points, consistent with [Research 320](./320-kiro-acp-2-21-4-identity.md#method-and-channel-boundary). Missing install is not a support gap; exact identity task. | [369](#current-route-and-interface-inventory) |
| `deepagents.acp` | Exact `0.1.30`; `QualifiedOnly` | `0.1.34` (2026-10-05) | npm package `deepagents-acp` | Missing from PATH | Four stable package points (`0.1.31`–`0.1.34`); exact ACP identity task. Do not substitute npm `deepagents` or ACP registry metadata. | [369](#current-route-and-interface-inventory) |
| `llama-cpp.attached` | Exact `b9910` / commit prefix `f5525f7e`; `QualifiedOnly` | Stable semantic release `v0.6.0` (2026-10-05) | Official llama.cpp GitHub stable release tags | `llama-server --version` timed out after 8 seconds; no retry | Version scheme changes from `bNNNN` nightly/dev builds to stable `vX.Y.Z`. Correlate runtime artifact identity before a new qualification; nightly `b` tags are not stable updates. | [369](#current-route-and-interface-inventory) |
| `llama-cpp.owned` | Exact `b10069` / commit prefix `178a6c449`; `QualifiedOnly` | Stable semantic release `v0.6.0` (2026-10-05) | Official llama.cpp GitHub stable release tags | Same timed-out observation as attached; no retry | Same release-scheme reset, but owned lifecycle remains a separate route axis. Do not inherit attached qualification. | [369](#current-route-and-interface-inventory) |
| `muse-code.headless` | Exact signed payload `0.2.1-R1215.1`; `QualifiedOnly` | No public official package or release channel for the signed payload located | Vendor-signed payload axis; no ordered public stable source found in prior checkpoints | `muse --version` prints launcher `1.0.3`; it does not identify selected payload | Gate: vendor-signed payload metadata and exact versioned artifact identity. Mutable launcher version is not the route version. | [369](#current-route-and-interface-inventory) |
| `mistral-vibe.headless` | Exact `2.25.4`; `QualifiedOnly` | `2.26.0` (2026-10-06) | Official GitHub release and PyPI stable package agree | `vibe` missing from PATH | Four PyPI stable points after the pin: `2.25.5`, `2.25.7`, `2.25.8`, `2.26.0`; GitHub latest agrees. Preserve the `--legacy-harness` adaptation from Research 321 and inspect each package/tag point. | [369](#current-route-and-interface-inventory) |
| `kimi-code.acp` | Exact `0.28.1` plus `0.29.0..=0.38.0`; `QualifiedOnly`; higher ACP revisions fail closed | `2.1.1` (2026-09-24) | `@moonshot-ai/kimi-code`; npm/GitHub agree | `kimi 0.34.0` | Thirteen stable package points after `0.38.0`, including the `2.0.0` reset. Identity task must keep the ACP terminal-runner stop; do not borrow local-server evidence. | [369](#current-route-and-interface-inventory) |
| `kimi-code.headless` | Qualified `0.29.0..=0.32.0` (ACP v1), `0.33.0..=0.43.0` (v2); later stable unverified | `2.1.1` (2026-09-24) | `@moonshot-ai/kimi-code`; npm/GitHub agree | `kimi 0.34.0` | Six stable package points after `0.43.0`, including `2.0.0`; major-line identity reset, separate from ACP and local-server. | [369](#current-route-and-interface-inventory) |
| `kimi-code.local-server` | Exact `0.28.1`; published points in `0.29.0..=0.39.1`, `0.40.0..=0.43.1`, and `2.0.0..=2.1.1` under `AllowUnverified`; excludes unpublished `0.39.2`, `0.40.2`, `0.41.1`, `0.42.1`, `0.43.2`, `1.x`, and `2.0.3`; documented `cwd` caveat from `0.40.0` | `2.1.1` (2026-09-24) | `@moonshot-ai/kimi-code`; npm/GitHub agree | `kimi 0.34.0` | Official latest equals qualified ceiling. Q-004 and Research 357 remain answered; unchanged on this route. | [369](#current-route-and-interface-inventory) |
| `kimi-platform.chat` | Exact opaque `kimi-platform-chat-2026-07-21`; `QualifiedOnly` | No ordered public stable version; exact API facade | Kimi Platform Chat API docs | Not applicable | No scalar release; ignore model aliases. | [369](#current-route-and-interface-inventory) |
| `oh-my-pi.rpc` | Deprecated `17.2.9..=17.4.2`; maintained `18.0.0..=18.2.7`; later stable unverified | `18.8.0` (2026-10-07) | `@oh-my-pi/pi-coding-agent`; npm/GitHub agree | `omp 18.3.4` | Twenty-nine stable points after `18.2.7`; host and official point exceed the qualified ceiling. Keep earlier RPC mapping gates. | [369](#current-route-and-interface-inventory) |
| `ollama.attached` | `0.14.0..=0.34.4`, excluding `0.32.2`, `0.32.10`; `AllowUnverified` | `v0.40.0` (2026-09-25) | Official Ollama GitHub stable releases | Client `0.33.3`; no running runtime reachable | Three stable release tags after `v0.34.4`: `v0.35.0`, `v0.35.1`, `v0.40.0`. `b`/RC tags excluded; inspect selected server surface before any extension. | [369](#current-route-and-interface-inventory) |
| `codex.app-server` | Maintained segments `0.80.0..=0.81.0`, `0.84.0..=0.107.0`, `0.110.0..=0.155.1`; later stable unverified | `0.161.0` (2026-10-07) | `@openai/codex`; npm/GitHub agree | `codex 0.159.0` | Twelve stable package points after `0.155.1`; host and official latest exceed the current bound. Preserve app-server-specific controls. | [369](#current-route-and-interface-inventory) |
| `codex.exec` | Same maintained package segments through `0.155.1`; later stable unverified | `0.161.0` (2026-10-07) | `@openai/codex`; npm/GitHub agree | `codex 0.159.0` | Twelve stable points; separate exec interface qualification from app-server. | [369](#current-route-and-interface-inventory) |
| `openai.realtime` | Exact opaque `openai-realtime-reasoning-2026-08-27`; `QualifiedOnly` | No ordered public stable version; exact Realtime API facade | OpenAI Realtime API docs | Not applicable | Exact current facade; prior `openai-realtime-2026-07-22` remains historical evidence. No model-alias comparison. | [369](#current-route-and-interface-inventory) |
| `openai.background` | Exact opaque `openai-responses-background-2026-08-23-service-tier`; `QualifiedOnly` | No ordered public stable version; exact Responses background facade | OpenAI Responses API docs | Not applicable | Exact current facade; recheck request/service-tier behavior rather than infer a date-based range. | [369](#current-route-and-interface-inventory) |
| `opencode.acp` | Deprecated `1.18.18..=1.18.30`; maintained `1.18.31..=1.18.32`; later stable unverified | `1.18.35` (2026-10-06) | `opencode-ai`; npm/GitHub agree | `opencode 1.18.32` | Three stable package points after current ceiling; one-family ACP mapping task. | [369](#current-route-and-interface-inventory) |
| `opencode.http` | Published qualified segments through `1.18.31`; later stable unverified | `1.18.35` (2026-10-06) | `opencode-ai`; npm/GitHub agree | `opencode 1.18.32` | Four stable package points after HTTP ceiling; preserve import/history/reconciliation/detachment limits from the current guide. | [369](#current-route-and-interface-inventory) |
| `qoder.headless` | Exact `1.1.54`; `QualifiedOnly` | `1.1.65` (2026-09-30) | `@qoder-ai/qodercli`; npm latest | `qodercli` missing from PATH | Eleven stable points after exact pin; identity task. Missing install is not a support gap. | [369](#current-route-and-interface-inventory) |
| `grok-build.catalogue` | Exact `1.0.30`; `QualifiedOnly` | `1.0.46` (2026-09-30) | `@xai-official/grok`; npm latest `1.0.46`; alpha `1.0.50` excluded | Isolated-home `grok --version`: `1.0.46` | Sixteen stable points after the exact catalogue point; current host and stable exceed its bound. Separate catalogue gate; no live catalogue here. | [369](#current-route-and-interface-inventory) |
| `grok-build.acp` | Maintained through exact `1.0.41`; later stable unverified | `1.0.46` (2026-09-30) | `@xai-official/grok`; npm latest `1.0.46`; alpha `1.0.50` excluded | Isolated-home `grok --version`: `1.0.46` | Five stable points after `1.0.41`; keep ACP HTTP MCP at its exact prior evidence point. | [369](#current-route-and-interface-inventory) |
| `xai.responses-websocket` | Exact opaque `xai-responses-websocket-2026-04-23`; `QualifiedOnly` | No ordered public stable version; exact Responses WebSocket API facade | xAI Responses API docs | Not applicable | No scalar interface release; model changes are not API-version changes. | [369](#current-route-and-interface-inventory) |
| `zcode.app-server` | Exact runtime `0.16.3`; `QualifiedOnly` | No trustworthy official `zcode.cjs` runtime release channel located | `zcode.runtime`; vendor artifact identity required | `zcode` missing from PATH | npm `zcode-app-cli@3.14.4-32` (2026-10-05) is launcher packaging, not the selected runtime. Gate: vendor-published signed/runtime artifact and `zcode.cjs` identity. | [369](#current-route-and-interface-inventory) |
| Shared `acp.schema` | Frozen exact schema artifact `schema-v1.20.0`; wire `protocolVersion` remains integer `1` | `schema-v1.24.1` (2026-09-30) | Official ACP schema release tags | No separate host executable; repository lifecycle/activity corpora use `schema-v1.20.0` | Five stable schema artifacts after `v1.20.0`: `v1.21.0`, `v1.22.0`, `v1.23.0`, `v1.24.0`, `v1.24.1`. Schema discovery is not permission to rewrite shared protocol evidence. | [369](#current-route-and-interface-inventory) |
| Embedded native runtime: `claude-agent.sdk` | Exact `2.1.284`; `QualifiedOnly`, coupled to SDK `0.3.284` | Latest SDK package is `0.3.292`; embedded native version for that package not read from an artifact | Platform-specific `@anthropic-ai/claude-agent-sdk-*` package manifest; npm metadata alone does not expose the embedded Claude build | Installed `claude 2.1.286` is the host CLI, not the embedded native | Read the current platform artifact manifest and compare its exact embedded build before treating `2.1.292` as current. | [369](#current-route-and-interface-inventory) |
| Embedded Node runtime: `claude-agent.sdk` | Exact `22.23.2` sidecar runtime | Current Node 22 stable `22.23.3` (2026-09-23); current Node major latest `26.11.0` (2026-10-07) | Official Node distribution index | `node --version`: `v22.23.2` | One newer patch on the pinned Node 22 line; `26.x` is a major reset, not compatible evidence. Exact sidecar pin reopens in its own family task. | [369](#current-route-and-interface-inventory) |
| Embedded Node runtime: `pi.sdk-sidecar` | Exact `22.23.2` sidecar runtime | Current Node 22 stable `22.23.3` (2026-09-23); current Node major latest `26.11.0` (2026-10-07) | Official Node distribution index | `node --version`: `v22.23.2` | Same Node patch discovery, but qualify the Pi sidecar tuple separately from Claude. | [369](#current-route-and-interface-inventory) |
## Ranked follow-up register

Ranking rule: routes with a newer stable on a maintained or `AllowUnverified`
surface are first; exact-pin and `QualifiedOnly` families follow; major-line,
prerelease, and source-identity resets follow those; source gaps are last because
the current official point cannot yet be established. This is an input to the
planner, not a dispatch order or compatibility ruling. Each family needs its
own task; siblings with different route claims stay separate. No bulk campaign
is authorized here.

| Rank | Family / impacted route(s) | Exact qualified point → current official point | Published newer stable hops or evidence needed | Package/channel and action gate |
| ---: | --- | --- | --- | --- |
| 1 | `codex.app-server` | `0.155.1 → 0.161.0` | 12 | `@openai/codex`; inspect all npm/GitHub points and app-server-selected behavior. |
| 2 | `codex.exec` | `0.155.1 → 0.161.0` | 12 | Same package, separate exec mapping and controls. |
| 3 | `gemini-cli.acp` | `0.61.0 → 0.63.0` | 2 | Gemini deferral is lifted; qualify ACP only and preserve exact HTTP MCP boundary. |
| 4 | `gemini-cli.headless` | `0.61.0 → 0.63.0` | 2 | Same package, separate headless profile. |
| 5 | `claude-agent.acp` | `0.81.2 → 0.87.0` | 7 | Recheck ACP package, GitHub release, and ACP registry agreement; identity task. |
| 6 | `claude-code.headless` | `2.1.281 → npm latest/GitHub latest release 2.1.292`; delayed npm `stable` tag `2.1.285` | 11 published points: `2.1.282`–`2.1.292` | npm latest and GitHub latest non-prerelease agree; metadata only. Qualify headless route independently. Anthropic documents `stable` as a separate delayed channel. |
| 7 | `claude-code.response-only` | `2.1.281 (stream-json.v2) → npm latest/GitHub latest release 2.1.292`; delayed npm `stable` tag `2.1.285` | 11 published points: `2.1.282`–`2.1.292` | Same channel agreement; retain the route-specific stream protocol and qualify response-only independently. Do not transfer headless evidence. |
| 8 | `qwen.headless` | `0.24.2 → 0.25.0` | 6 | npm CLI package; compare its exact selected points, not the unrelated TypeScript SDK GitHub tag. |
| 9 | `antigravity.catalogue` | `1.2.11 → 1.3.1` | 8 | Official CLI releases; catalog mapping task. ACP registry `antigravity-acp@1.3.0` is a different product. |
| 10 | `antigravity.headless` | pinned `1.2.11 → 1.3.1` | 8 | Keep `AGY_CLI_MODEL_API_MAX_RETRIES=0`; the pin qualifies only `1.2.11`. Current release requires an adaptation task with bounded retry evidence. |
| 11 | `bedrock.catalogue` | `aws-sdk-bedrock 1.150.0 → 1.161.0` | 11 crate points | Exact crates.io crate pin; inspect control-plane SDK chain. |
| 12 | `bedrock.runtime` | `aws-sdk-bedrockruntime 1.139.0 → 1.148.0` | 9 crate points | Separate exact runtime SDK pin and EventStream chain. |
| 13 | `opencode.acp` | `1.18.32 → 1.18.35` | 3 | Same package as HTTP, but ACP identity and protocol qualification remain distinct. |
| 14 | `opencode.http` | `1.18.31 → 1.18.35` | 4 | Preserve import/history/reconciliation/detachment ceiling while assessing newer server. |
| 15 | `ollama.attached` | `v0.34.4 → v0.40.0` | 3 stable tags: `v0.35.0`, `.35.1`, `.40.0` | Official GitHub stable tags; inspect attached runtime only. No runtime was reachable locally. |
| 16 | `oh-my-pi.rpc` | `18.2.7 → 18.8.0` | 29 | npm/GitHub; local `18.3.4` is already newer than the bound. Preserve the RPC mapping boundary. |
| 17 | `claude-agent.sdk` package and embedded native | `0.3.284 / 2.1.284 → package 0.3.292; native point pending artifact identity` | 8 SDK package points | Exact package artifact-tree and embedded-native identity task. Research 368's usage evidence is for frozen `0.3.284` and adds no version claim; do not infer native `2.1.292` or registered-tool acceptance from it. |
| 18 | `claude-agent.sdk` Node runtime | `22.23.2 → 22.23.3` on Node 22 | 1 patch | Exact Node pin; Node `26.11.0` is a major-line reset and remains out of scope. |
| 19 | `pi.rpc` | `0.86.1 → 1.0.4` | 10 | npm/GitHub; major-line reset requires identity and mapping review. Local `0.87.1` is newer than the bound. |
| 20 | `pi.sdk-sidecar` package | `0.84.2 → 1.0.4` | 16 | Exact SDK sidecar tuple; major-line reset; keep separate from Pi RPC. |
| 21 | `pi.sdk-sidecar` Node runtime | `22.23.2 → 22.23.3` on Node 22 | 1 patch | Qualify the sidecar runtime together with its exact Pi package tuple. |
| 22 | `cline.acp` | `3.0.55 → 3.0.69` | 13 | Exact npm package; inspect ACP only. General Cline GitHub release `v4.1.23` is not this package. |
| 23 | `cline.headless` | `3.0.55 → 3.0.69` | 13 | Same package, separate headless selected behavior. |
| 24 | `command-code.headless` | `1.65.0 → 1.77.0` | 29 | Exact pin; per-hop identity and selected headless behavior required. |
| 25 | `copilot-cli.acp` | `1.0.80 → 1.0.93` | 13 | `@github/copilot`; exact ACP pin task. |
| 26 | `qoder.headless` | `1.1.54 → 1.1.65` | 11 | Exact pin; missing local install does not alter priority or count as a gap. |
| 27 | `goose.acp` | `1.50.1 → 1.53.0` | 3 | Exact ACP pin; preserve live HTTP MCP honouring as a one-point gate only. |
| 28 | `kiro.acp` | `2.21.4 → 2.28.0` | 12 published stable points, including main and patch releases | Exact pin; official changelog only. Missing local install is not a gap. |
| 29 | `deepagents.acp` | `0.1.30 → 0.1.34` | 4 | `deepagents-acp` npm channel; exact ACP identity task. `deepagents` and registry entry `0.1.7` do not match this axis. |
| 30 | `mistral-vibe.headless` | `2.25.4 → 2.26.0` | 4 PyPI stable points; GitHub tags agree on latest | Exact point; preserve `--legacy-harness` adaptation. PyPI lacks GitHub-only publications; compare distribution artifacts. |
| 31 | `cursor-agent.catalogue` | `2026.09.18-9a7762b → 2026.10.01` | Registry shows current archive; exact ordered history and hop count still needed | Official registry is discovery, not qualification; frozen archive inventory/hashes and catalogue selected-path comparison are the gate. |
| 32 | `cursor-agent.acp` | `2026.09.18-9a7762b → 2026.10.01` | Same release; hop count still needed | Separate ACP artifact, protocol, and mapping evidence. |
| 33 | `cursor-agent.headless` | `2026.09.18-9a7762b → 2026.10.01` | Same release; hop count still needed | Separate headless artifact and selected-path evidence. |
| 34 | `kimi-code.acp` | `0.38.0 → 2.1.1` | 13 package points | `@moonshot-ai/kimi-code`; includes 0.39/0.40–0.43 and the `2.0.0` reset. Reopen fail-closed terminal-runner stop; do not inherit local-server evidence. |
| 35 | `kimi-code.headless` | `0.43.0 → 2.1.1` | 6 package points | Same-package major reset; qualify the v2 system-version path separately. |
| 36 | `grok-build.catalogue` | `1.0.30 → 1.0.46` | 16 stable points | Current isolated local version is also `1.0.46`; exact catalogue remains `QualifiedOnly`. No live catalogue in this checkpoint. Alpha `1.0.50` is excluded. |
| 37 | `grok-build.acp` | `1.0.41 → 1.0.46` | 5 stable points | Same package, separate ACP mapping. Keep HTTP MCP at its exact evidence point. Alpha `1.0.50` is excluded. |
| 38 | `deepseek-harness.jsonrpc` | `0.1.0rc6 → 0.2.0-rc.2` | 24 later published alpha/RC points; no stable point | The pin is itself prerelease; inspect complete runtime-bin chain and route-specific JSON-RPC surface. |
| 39 | `deepseek-harness.local-server` | `0.1.0-rc.6 → 0.2.0-rc.2` | Same 24 later prerelease points; no stable point | Separate web/API surface; no transfer from JSON-RPC. |
| 40 | Shared `acp.schema` | `schema-v1.20.0 → schema-v1.24.1` | 5 schema releases | Wire stays ACP protocol `1`; inspect schema additions against frozen lifecycle/activity corpora before any shared protocol update. |
| 41 | `llama-cpp.attached` | `b9910 → stable v0.6.0` | No directly comparable hop count; version-scheme change | Obtain exact official runtime artifact/source identity. Do not treat nightly `b` tags as stable. |
| 42 | `llama-cpp.owned` | `b10069 → stable v0.6.0` | No directly comparable hop count; version-scheme change | Separate owned-runtime lifecycle qualification after identity mapping. |
| 43 | `muse-code.headless` | `0.2.1-R1215.1` | Official version movement cannot be established | Find vendor-signed payload metadata and exact artifact; `muse 1.0.3` is a mutable launcher. |
| 44 | `zcode.app-server` | `0.16.3` | Official version movement cannot be established | Find signed/current `zcode.cjs` identity. npm `zcode-app-cli@3.14.4-32` is only a wrapper package. |

The exact API facades (`alibaba.conversations`, both Anthropic APIs,
`deepseek.continuation`, `gemini.live`, `kimi-platform.chat`, OpenAI Realtime
and Responses background, and `xai.responses-websocket`) have no ordered
public release point on their selected axes. Their exact opaque/date facade
bindings remain the bound. Official docs are linked below for recheck; model
aliases and hosted “latest model” values are excluded. `kimi-code.local-server`
also has no newer point: official `2.1.1` equals its current qualified ceiling.

## Channel reconciliation and unresolved items

- npm `latest` and GitHub latest stable agree for Claude Agent ACP, Pi, Gemini
  CLI, Kimi Code, OpenCode, and Codex. Mistral GitHub and PyPI both point to
  `2.26.0`; Goose, Antigravity, Ollama, and llama.cpp use their named official
  GitHub stable channels.
- Claude Code channel selection follows the [checkpoint procedure's Sources
  section](../knowledge/operations/version-currentness-checkpoint.md#sources)
  and the [version-currentness reference's Official channels
  section](../../.cursor/skills/version-currentness/reference.md#official-channels):
  npm `latest` is the selected channel for published CLIs. [Research
  348](./348-claude-code-2-1-281-narrowed-response-only-claim.md#official-latest)
  records that npm `latest`/`next` and GitHub latest agreed at `2.1.281`, and
  explicitly says npm `stable` was not this family's channel. Anthropic's
  [release-channel documentation](https://code.claude.com/docs/en/setup#configure-release-channel)
  defines `latest` as the default immediate channel and `stable` as typically
  about one week old, skipping releases with major regressions. At the 2026-10-07
  16:53 UTC re-probe, npm `latest=2.1.292` (published 2026-10-06) and GitHub's
  latest non-prerelease release `v2.1.292` agree; npm `stable=2.1.285`
  (published 2026-09-29) is the separately documented delayed channel. There is
  no same-channel disagreement. The eleven npm latest points after the
  qualified `2.1.281` are discovery only; headless and response-only still
  need independent family qualification.
- Qwen's GitHub latest tag is for its TypeScript SDK; route `qwen.headless`
  uses npm `@qwen-code/qwen-code`. Cline GitHub `v4.1.23` is the editor
  product; Swallowtail routes use npm package `cline@3.0.69`. These are
  different product surfaces, not conflicting version observations.
- npm `deepagents` `1.14.2` is not the route package `deepagents-acp@0.1.34`;
  the ACP registry's `deepagents` metadata `0.1.7` is discovery metadata, not
  the npm package release. Do not merge their versions.
- npm `@xai-official/grok` `latest=1.0.46`; its `alpha=1.0.50` is excluded.
  Cursor ACP registry metadata `version=1.0.0` describes registry format; its
  Cursor entry `2026.10.01` is the agent archive point. Antigravity ACP
  registry `1.3.0` is a separate server product from CLI `1.3.1`.
- Node 26.11.0 is latest overall, while the SDK sidecars pin Node 22.23.2.
  The newest Node 22 patch is 22.23.3. Do not flatten the major versions.
- The Alibaba matrix/selection axis labels differ as noted in the route list.
  The single route, exact facade string, adapter, and maintained evidence agree;
  this is a naming reconciliation note, not a route-count ambiguity. No matrix
  edit is made here.
- The version-currentness lane's named Antigravity headless interior-point
  backfill lead stays out of scope. A family task may inspect release hops to
  assess the current `1.3.1` target, but this checkpoint authorizes no bulk
  interior-point claim backfill.
- `llama-server --version` timed out in the 8-second safe observation and was
  not retried. A selected official runtime artifact identity is needed; this
  local observation alone does not make an install gap.
- `agy` and `gemini` were present but their auto-updating `--version` commands
  were skipped. This is an observation boundary, not permission to update and
  not a support-gap classification.
- The installed Muse launcher reported `1.0.3`, not the signed payload bound.
  Prior checkpoints did not locate a public release channel for that payload.
  An exact vendor-signed payload manifest is the gate. The installed ZCode
  executable is missing; the discovered npm wrapper version does not identify
  route runtime `zcode.cjs`.

## Source ledger

Npm package metadata was read from each package's public full packument; the
URL pattern is `https://registry.npmjs.org/<package-name-with-scope-slash-as-%2F>`.
Crates.io package/version metadata and official GitHub release metadata supplied
publication dates and hop lists. Host observations are not version authority.

| Source key | Official source |
| --- | --- |
| npm Qwen | <https://registry.npmjs.org/@qwen-code%2Fqwen-code> |
| npm Claude Agent ACP | <https://registry.npmjs.org/@agentclientprotocol%2Fclaude-agent-acp> |
| npm Claude Agent SDK | <https://registry.npmjs.org/@anthropic-ai%2Fclaude-agent-sdk> |
| npm Claude Code | <https://registry.npmjs.org/@anthropic-ai%2Fclaude-code> |
| Claude Code latest release | <https://api.github.com/repos/anthropics/claude-code/releases/latest> |
| Anthropic Claude Code release channels | <https://code.claude.com/docs/en/setup#configure-release-channel> |
| npm Pi | <https://registry.npmjs.org/@earendil-works%2Fpi-coding-agent> |
| npm Cline | <https://registry.npmjs.org/cline> |
| npm Command Code | <https://registry.npmjs.org/command-code> |
| npm Copilot CLI | <https://registry.npmjs.org/@github%2Fcopilot> |
| npm Gemini CLI | <https://registry.npmjs.org/@google%2Fgemini-cli> |
| npm Kimi Code | <https://registry.npmjs.org/@moonshot-ai%2Fkimi-code> |
| npm Oh My Pi | <https://registry.npmjs.org/@oh-my-pi%2Fpi-coding-agent> |
| npm Deep Agents ACP | <https://registry.npmjs.org/deepagents-acp> |
| npm DeepSeek Harness | <https://registry.npmjs.org/@deepseek-ai%2Fdsh> |
| npm xAI Grok Build | <https://registry.npmjs.org/@xai-official%2Fgrok> |
| npm Qoder | <https://registry.npmjs.org/@qoder-ai%2Fqodercli> |
| npm OpenCode | <https://registry.npmjs.org/opencode-ai> |
| npm Codex | <https://registry.npmjs.org/@openai%2Fcodex> |
| crates.io Bedrock control plane | <https://crates.io/api/v1/crates/aws-sdk-bedrock> |
| crates.io Bedrock runtime | <https://crates.io/api/v1/crates/aws-sdk-bedrockruntime> |
| Antigravity CLI stable releases | <https://github.com/google-antigravity/antigravity-cli/releases> |
| Goose stable releases | <https://github.com/aaif-goose/goose/releases> |
| Ollama stable releases | <https://github.com/ollama/ollama/releases> |
| llama.cpp stable/nightly tags | <https://github.com/ggml-org/llama.cpp/releases> and <https://github.com/ggml-org/llama.cpp/tags> |
| Mistral Vibe GitHub | <https://github.com/mistralai/mistral-vibe/releases> |
| Mistral Vibe PyPI | <https://pypi.org/pypi/mistral-vibe/json> |
| ACP schema releases | <https://github.com/agentclientprotocol/agent-client-protocol/releases> |
| ACP registry discovery | <https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json> |
| Kiro CLI changelog | <https://kiro.dev/changelog/cli/> |
| Node distribution index | <https://nodejs.org/dist/index.json> |
| Alibaba Model Studio Responses API | <https://help.aliyun.com/en/model-studio/qwen-api-via-openai-responses> |
| Anthropic Managed Agents | <https://platform.claude.com/docs/en/agents-and-tools/managed-agents/overview> |
| Anthropic Messages API | <https://platform.claude.com/docs/en/api/messages> |
| DeepSeek Open Platform | <https://api-docs.deepseek.com/api/create-chat-completion> |
| Google Gemini Live API | <https://ai.google.dev/api/live.md> |
| Kimi Platform API | <https://platform.kimi.ai/docs/api/chat> |
| OpenAI Realtime API | <https://platform.openai.com/docs/api-reference/realtime> |
| OpenAI Responses background | <https://developers.openai.com/api/docs/guides/background> |
| xAI Responses WebSocket | <https://docs.x.ai/developers/advanced-api-usage/websocket-mode> |

Official runtime sources for Muse Code's signed payload and ZCode's `zcode.cjs`
were not located in the current or cited prior public checkpoint sources. The
actionable source gate for Muse is a vendor-signed versioned payload manifest;
for ZCode it is the vendor's signed/current runtime artifact identity. The
npm wrapper source <https://registry.npmjs.org/zcode-app-cli> was checked only
to distinguish packaging metadata from the selected runtime axis.

## Disposition

This record identifies the remaining one-family work and resolves the Claude
Code channel question from the selected official channel and Anthropic's
channel documentation. The planner can use this checkpoint to create separate
family tasks; metadata discovery does not qualify any newer point. Existing
exact-pin, major-line, adaptation, and source-identity gates remain explicit.
No claims, fixtures, matrices, release state, host state, or consumer state
changed.
