# Retained Execution `No` Currentness And Tranche Selection

Status: promoted
Owner: Tom
Date: 2026-07-28

## Question

Which of the 59 retained-background-execution, stream-reattachment, and
provider-managed-recovery `No` cells are non-applicable operation shapes,
exact upstream absences, separate-route work, or shared contract work?

## Method

Evidence was accessed on 2026-07-28.

- parsed all 22 canonical solution rows and froze the 59 starting cells
- compared every cell with Contracts 009, 021-022, 029, and 037-039
- inspected realized prepared policies, capabilities, validation, and fixtures
- reused the exact Kimi `0.28.1` and `0.29.0..=0.29.2` source corpus
- reused the exact OpenCode `1.14.48..=1.18.4` OpenAPI corpus
- checked current official Anthropic Message Batches, Amazon Bedrock
  asynchronous invocation, Alibaba Responses, Gemini Live session management,
  and maintained OpenCode server documentation

No executable, account, credential, provider request, paid operation,
container, or model server was used.

## Terms

- retained background execution is one provider-owned asynchronous inference
  operation under Contract 021; it is not a durable harness session
- retrieval observes one retained operation without attaching to its event
  stream
- stream reattachment attaches again to the same active operation or turn
  without replaying input
- transport reconnect only replaces a connection and proves no replay or
  recovery by itself
- provider-managed recovery is provider or harness retry or rescheduling
  accepted before effects; it is not Swallowtail or consumer retry

## Classification

The table classifies every starting `No` exactly once. A dash was already
`Yes` or `Not applicable` and is outside the 59-cell inventory.

- `NA` — the feature does not apply to the selected operation shape
- `U` — the selected surface does not expose the feature
- `S` — a separate route, operation contract, and exact corpus are required
- `C` — the selected route needs a shared contract expansion and exact corpus

| Solution route | Background | Reattach | Recovery |
| --- | --- | --- | --- |
| `qwen.headless` | NA | NA | U |
| `alibaba.conversations` | U | NA | U |
| `bedrock.catalogue; bedrock.runtime` | S | U | U |
| `claude-agent.acp` | NA | NA | U |
| `claude-code.headless` | NA | NA | U |
| `anthropic.managed-agent` | NA | — | — |
| `anthropic.messages` | S | NA | U |
| `pi.rpc` | NA | NA | U |
| `deepseek.continuation` | U | NA | U |
| `gemini-cli.acp + gemini-cli.headless` | NA | NA | U |
| `gemini.live` | NA | U | U |
| `llama-cpp.attached` | NA | NA | NA |
| `llama-cpp.owned` | — | — | — |
| `kimi-code.acp + kimi-code.headless` | NA | NA | C |
| `kimi-code.local-server` | NA | C | C |
| `kimi-platform.chat` | U | NA | U |
| `ollama.attached` | NA | NA | NA |
| `codex.app-server; codex.exec` | NA | NA | U |
| `openai.realtime` | NA | NA | U |
| `openai.background` | — | — | U |
| `opencode.http` | NA | U | U |
| `xai.responses-websocket` | NA | NA | U |

Exact totals:

| Classification | Cells |
| --- | ---: |
| operation-shape non-applicability | 32 |
| selected-surface absence | 22 |
| separate route and operation contract | 2 |
| shared contract and exact corpus | 3 |
| **Total** | **59** |

There are no realized matrix false negatives. All 59 cells remain `No` until
their final dispositions are applied by the implementation closeout.

## Operation-Shape Findings

Contract 021 deliberately limits background execution to one structured
direct-inference operation. A durable ACP, CLI, RPC, app-server, or attached
HTTP harness session does not become a background run because its process or
session can outlive one request.

Connection-scoped realtime operations, attached inference streams, one-shot
CLI processes, and consumer-owned continuation cannot reattach to a retained
operation that does not exist. Session load or resume also cannot substitute
for stream reattachment.

Provider-managed recovery remains meaningful for harnesses that may retry
internally. It is non-applicable only to the selected attached local model
runtimes, where no provider or harness recovery layer exists.

## Selected-Surface Absence

Alibaba's current selected international Responses route explicitly rejects
`background` and supports synchronous calls only. A separate China
application API cannot lend its endpoint audience, access, billing, or support
authority to this route.

Gemini Live session resumption supports planned connection rollover with
bounded resumption handles. The provider can mark a session non-resumable
during generation or function calls. That is not a general active-stream
reattachment or recovery guarantee, and the separate planned-rollover cell
already records the supported behavior.

OpenCode exposes asynchronous prompt acceptance and one server event SSE
stream. The exact selected OpenAPI surface has no event cursor, replay
identifier, or `Last-Event-ID` contract. Session message history can support a
future reconciliation design but cannot make the current stream reattach.

The remaining direct, installed, realtime, and attached routes expose no exact
provider-managed retry or rescheduling agreement. Internal implementation
behavior cannot become a Swallowtail capability without an exact opt-in and
observable evidence.

## Separate Routes

Amazon Bedrock Runtime exposes `StartAsyncInvoke`, `GetAsyncInvoke`, and
`ListAsyncInvokes`. That is not the selected `ConverseStream` route. Current
supported asynchronous models are media-oriented, and results are written to
caller-selected S3 storage. A proof needs a separate route plus model,
artifact, object-storage, access, retention, cancellation, and cleanup
contracts.

Anthropic Message Batches begin processing after one batch create request and
may take up to 24 hours. A batch contains multiple Messages requests and has
its own result and cancellation lifecycle. It is not one Contract 021
structured run. A proof needs a batch operation contract and separate facade
role rather than relabelling `anthropic.messages`.

Neither route belongs in the first tranche.

## Kimi Policy Defect

Kimi Code's qualified headless and local-server protocols both expose
`turn.step.retrying` with exact attempt and delay evidence. Production decoders
accept the event and can still report completion.

The same prepared operations currently require
`ProviderRecoveryPolicy::Prohibited`. That is not an honest agreement: the
harness may perform provider-managed recovery after Swallowtail accepted a
policy that prohibited it.

The repair must:

- require explicit `ManagedAllowed` before Kimi headless or local-server work
- advertise recovery only on the exact routes and version segments with
  qualified retry evidence
- preserve attempt numbers as provider evidence without exposing provider
  error text
- perform no Swallowtail retry, replay, fallback, or model change
- fail closed on malformed, decreasing, contradictory, or out-of-range retry
  evidence

The combined ACP and headless solution can claim only route-dependent recovery.
ACP remains recovery-unsupported.

## Kimi Reattachment

The local-server WebSocket v2 protocol already carries durable `{seq, epoch}`
cursors. A subscription can begin from the last accepted cursor, and the
server returns an exact acknowledgement or `resync_required`.

The realized driver uses the cursor for a new turn subscription but currently
fails an active turn when its WebSocket attachment is lost. A bounded
reattachment can reconnect to the same server, session, prompt, runtime turn,
credential lease, deadline, and cursor without submitting the prompt again.

This is durable harness-turn reattachment, not Contract 021 background
execution and not session resume. The shared contract must define:

- same-turn identity and no prompt replay
- one maximum automatic reattachment
- strict cursor and epoch continuity
- exact acknowledgement with no resynchronization requirement
- failure before a known prompt id as unconfirmed provider state
- cancellation and deadline behavior while detached
- joined network work before credential and owned-runtime release

## Selected Tranche

Cards 104-106 should close one Kimi integrity tranche:

1. explicit provider-managed recovery acceptance for Kimi headless
2. explicit provider-managed recovery acceptance for Kimi local server
3. maximum-one active-turn stream reattachment for Kimi local server

This selection is unambiguous because two current Kimi paths already accept
retry evidence under a contradictory prohibited policy. The cursor
reattachment uses the same exact version and event corpus and adds no provider,
account, credential, endpoint, billing, topology, or product-policy choice.

Expected final matrix movement:

- `kimi-code.acp + kimi-code.headless` recovery: `No` to `Partial`
- `kimi-code.local-server` reattachment: `No` to `Yes`
- `kimi-code.local-server` recovery: `No` to `Yes`
- 32 non-applicable starting cells: `No` to `Not applicable`
- the remaining 24 starting cells stay `No`

## Sources

- [Kimi Code `0.29.2` WebSocket control](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/packages/kap-server/src/protocol/ws-control.ts)
- [Kimi Code `0.29.2` event schemas](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/packages/kap-server/src/protocol/events-zod.ts)
- [Kimi Code `0.29.2` default headless runner](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/apps/kimi-code/src/cli/run-prompt.ts)
- [OpenCode server API](https://dev.opencode.ai/docs/server/)
- [Anthropic Message Batches API](https://platform.claude.com/docs/en/api/messages/batches/create)
- [Amazon Bedrock Runtime operations](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_Operations_Amazon_Bedrock_Runtime.html)
- [Amazon Bedrock asynchronous model compatibility](https://docs.aws.amazon.com/bedrock/latest/userguide/models-api-compatibility.html)
- [Alibaba Responses compatibility](https://www.alibabacloud.com/help/en/model-studio/qwen-api-via-openai-responses)
- [Gemini Live session management](https://ai.google.dev/gemini-api/docs/live-api/session-management)
