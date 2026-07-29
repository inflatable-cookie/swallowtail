# Runtime Ownership And Rollover `No` Currentness And Negative Closeout

Status: promoted
Owner: Tom
Date: 2026-07-28

## Question

Which of the 40 owned-runtime-lifecycle and planned-connection-rollover `No`
cells are applicable selected-surface gaps, false negatives, or operation-shape
non-applicability?

## Method

Evidence was accessed on 2026-07-28.

- parsed all 22 canonical solution rows and froze the 40 starting cells
- compared every cell with Contracts 004, 009, 018, 026-027, 029, 031, and
  037
- inspected each route's realized preparation, ownership, operation, stop,
  connection, cancellation, and cleanup shape
- checked the current official OpenAI Realtime WebSocket guide against
  Contract 027's provider-warning, private-handle, successor-setup, and
  continuity-confirmation requirements
- retained the existing Gemini Live and owned Kimi/llama.cpp positive proofs
  outside the starting inventory

No executable, account, credential, provider request, paid operation,
container, sandbox, harness server, or model server was used.

## Terms

`owned_runtime_lifecycle` applies when a prepared solution can start or attach
to a separately addressable local runtime or foreground server and an owned
topology can later stop and join that process. It does not mean:

- a child process scoped to one run or session
- an adapter-owned network connection
- a provider-managed service, environment, conversation, or retained run
- cleanup of operation-owned remote resources
- authority over an attached server

The current positive routes remain deliberately different:

- `llama-cpp.owned` owns ephemeral model serving, endpoint readiness, artifact
  lifetime, and joined stop
- `kimi-code.local-server` owns an optional foreground harness server, while
  its attached topology remains external

`planned_connection_rollover` applies only to a live realtime-media operation
that crosses a provider-announced connection lifetime boundary using the
latest private resumable handle and confirmed successor setup. It is not:

- reconnect after loss
- stream reattachment
- provider-managed recovery
- session load or resume
- prompt replay or inference retry
- starting another harness process

## Classification

The table classifies every starting `No` exactly once. A dash was already
`Yes` or `Not applicable` and is outside the inventory.

- `NA` — the feature does not apply to the selected route and operation shape
- `U` — the selected applicable surface does not expose the required protocol

| Solution route | Owned runtime lifecycle | Planned rollover |
| --- | --- | --- |
| `qwen.headless` | NA | NA |
| `alibaba.conversations` | NA | NA |
| `bedrock.catalogue; bedrock.runtime` | NA | NA |
| `claude-agent.acp` | NA | NA |
| `claude-code.headless` | NA | NA |
| `anthropic.managed-agent` | NA | NA |
| `anthropic.messages` | NA | NA |
| `pi.rpc` | NA | NA |
| `deepseek.continuation` | NA | NA |
| `gemini-cli.acp + gemini-cli.headless` | NA | NA |
| `gemini.live` | NA | — |
| `llama-cpp.attached` | NA | NA |
| `llama-cpp.owned` | — | — |
| `kimi-code.acp + kimi-code.headless` | NA | NA |
| `kimi-code.local-server` | — | NA |
| `kimi-platform.chat` | NA | NA |
| `ollama.attached` | NA | NA |
| `codex.app-server; codex.exec` | NA | NA |
| `openai.realtime` | NA | U |
| `openai.background` | NA | NA |
| `opencode.http` | NA | NA |
| `xai.responses-websocket` | NA | NA |

Exact totals:

| Classification | Cells |
| --- | ---: |
| operation-shape non-applicability | 39 |
| selected-surface absence | 1 |
| contract or corpus candidate | 0 |
| realized matrix false negative | 0 |
| **Total** | **40** |

## Ownership Findings

All 20 ownership `No` cells were category errors.

Installed harness children remain owned and joined, but their lifecycle is
already the run or session lifecycle. They do not expose a reusable runtime or
server handle before the operation. Codex app-server, ACP, RPC, CLI, and
headless process cleanup therefore cannot become an owned-runtime claim.

Attached llama.cpp, Ollama, and OpenCode preserve their external servers.
Hosted APIs, SDK routes, provider conversations, managed-agent resources,
background responses, and realtime connections remain provider-owned or
operation-scoped. Their cleanup truth belongs to connection, provider-resource,
or operation lifecycle, not local runtime ownership.

## Rollover Findings

Nineteen rollover `No` cells were category errors. Structured runs, ordinary
interactive harness sessions, retained responses, attached servers, and
connection-local text continuation do not implement the realtime-media
rollover operation defined by Contract 027.

OpenAI Realtime is the only applicable remaining `No`. The current official
WebSocket guide creates and manages one Realtime session over one WebSocket.
It documents client and server lifecycle events but no provider end-of-life
warning, private resumption handle, replacement setup carrying that handle, or
continuity confirmation. Swallowtail's selected Realtime facade therefore
keeps planned rollover disabled.

This is a selected-surface absence, not a claim that applications cannot open
a fresh connection and stitch consumer state. Fresh-session reconstruction
would be reconnect or replay, not Contract 027 continuity.

## Negative Closeout

No implementation tranche is justified.

The matrix should change 39 starting cells from `No` to `Not applicable`.
OpenAI Realtime planned rollover remains `No`. Contracts 004, 009, 018,
026-027, and 031 already express the required distinctions, so no contract or
corpus batch should run.

Cards 112-113 should be superseded by this negative closeout. Card 114 should
close the family using matrix and docs validation only.

## Sources

- [OpenAI Realtime API with WebSocket](https://developers.openai.com/api/docs/guides/realtime-websocket)
- [Contract 018](../contracts/018-owned-ephemeral-model-serving-lifecycle.md)
- [Contract 027](../contracts/027-planned-connection-rollover-and-realtime-continuity.md)
- [Contract 031](../contracts/031-attached-native-runtime-version-and-residency.md)

## Promotion

- Reclassified 39 category-error cells as `Not applicable`.
- Retained one exact OpenAI Realtime selected-surface `No`.
- Selected a negative implementation closeout for roadmap g02.033.
