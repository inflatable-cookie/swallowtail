# 015 Provider-Owned Background Run Evidence

Status: promoted
Owner: Tom
Updated: 2026-07-21

## Question

Which route adds the most architectural information after the separate Bedrock
Runtime and control-plane SDK proofs without repeating an existing transport or
establishing an implicit provider, subscription, repository, or deployment
policy?

## Method

Official provider documentation, maintained-project documentation, current
protocol status, and the ten realized Swallowtail production routes were
checked on 2026-07-21. No credential store, provider account, subscription,
repository integration, or live endpoint was used.

Candidates were compared on operation ownership, stream recovery,
cancellation, credential and endpoint authority, provider retention, workspace
authority, support maturity, topology, and deterministic fixture quality.

## Realized Baseline

Swallowtail already proves:

- Codex one-shot JSONL and long-lived app-server RPC
- attached OpenCode HTTP/SSE harness execution
- Anthropic direct HTTP/SSE inference
- Gemini and Kimi ACP over local subprocesses
- attached and host-owned llama.cpp HTTP/SSE serving
- xAI connection-scoped Responses WebSocket sessions
- Bedrock Runtime through a provider-supported Rust SDK and EventStream
- Bedrock control-plane catalogue discovery through a separate Rust SDK client

Another synchronous Chat Completions, one-shot JSONL, attached OpenAI-
compatible server, or language-sidecar harness does not beat this baseline by
provider count alone.

## Leading Direct Route

OpenAI Responses background mode creates a provider-owned asynchronous
operation:

- `POST /v1/responses` with `background=true`
- `GET /v1/responses/{id}` while status is `queued` or `in_progress`
- `POST /v1/responses/{id}/cancel`, with idempotent repeated cancellation
- `background=true` plus `stream=true` for immediate SSE delivery
- stream reattachment through the response id and
  `starting_after=<sequence_number>` after a connection drop

The provider operation continues when the first stream disconnects. The
response id, provider operation state, SSE cursor, Swallowtail run id, and
common event sequence are therefore separate identities and lifecycles.

`store=false` does not mean no provider retention for this route. OpenAI states
that background response data is retained temporarily to support asynchronous
execution and polling. Zero Data Retention projects use temporary disk storage
for roughly ten minutes. That requirement must be explicit before preflight;
it cannot be inferred from a generic direct-inference or streaming capability.

Evidence:

- [OpenAI background mode](https://developers.openai.com/api/docs/guides/background)
- [Create a model response](https://developers.openai.com/api/reference/resources/responses/methods/create)

## Cloud Harness Recheck

Cursor now exposes a public-beta TypeScript SDK over the same harness used by
its desktop, CLI, and web products. It supports local execution or a dedicated
cloud VM. Its Cloud Agents API separates durable agents from per-prompt runs,
supports run-scoped status, streaming, cancellation, SSE reconnect through
`Last-Event-ID`, and agent archive, restore, and delete operations.

This is a strong later proof, but the cloud route also introduces provider-
owned repository checkout and mutation, GitHub integration authority, remote
environment lifecycle, artifact handoff, durable-agent deletion, and public-
beta support. Selecting the allowed repository and mutation posture is product
policy that the current Swallowtail authority surfaces do not fix. The local
TypeScript SDK route would instead add a language sidecar around a lifecycle
already covered by Codex, OpenCode, Gemini, Kimi, and ACP.

Evidence:

- [Cursor SDK release](https://cursor.com/changelog/sdk-release)
- [Cursor maintained cookbook](https://github.com/cursor/cookbook)

## Remaining Harness And Protocol Routes

Claude Agent SDK now has substantial permission, persisted-session, resume,
fork, and external session-store behavior, but its maintained embedding
surfaces remain Python and TypeScript. A Rust Swallowtail adapter would own a
language sidecar or provider binary and repeat existing harness lifecycle
before it proves a new transport.

Qwen Code headless mode adds useful wall-time, turn, and tool-call budgets, but
its JSON and stream-JSON routes repeat existing process shapes. Its ACP route
does not currently apply the same wall-time and tool-call budgets. Pi remains
an in-process TypeScript SDK or JSON RPC subprocess route.

Kimi Agent Rust is smaller and starts faster than the Python Kimi Code kernel,
but it remains an experimental standalone Wire-only binary, requires manual
API-key configuration, has no Kimi account login, and releases independently
from Kimi Code. It does not replace the completed provider-supported Kimi Code
ACP route yet.

ACP has since stabilized session resume, close, delete, request cancellation,
and several configuration extensions. The replacement Rust SDK remains
Preview, and the remote Streamable HTTP/WebSocket transport remains Active
rather than completed. Swallowtail already owns bounded ACP framing,
correlation, callbacks, persistent sessions, and two production agents. Recheck
the remote transport when it stabilizes or when a selected agent exposes it.

Evidence:

- [Claude Agent SDK sessions](https://code.claude.com/docs/en/agent-sdk/sessions)
- [Claude Agent SDK permissions](https://code.claude.com/docs/en/agent-sdk/permissions)
- [Qwen Code headless mode](https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/)
- [Pi coding-agent SDK](https://github.com/earendil-works/pi/blob/main/packages/coding-agent/docs/sdk.md)
- [Kimi Wire mode and Rust agent](https://moonshotai.github.io/kimi-cli/en/customization/wire-mode.html)
- [ACP RFD updates](https://agentclientprotocol.com/rfds/updates)

## Remaining Direct And Attached Routes

Current Z.AI and DeepSeek direct APIs expose supported HTTP streaming shapes
close to the existing Anthropic direct route. Alibaba Model Studio now exposes
an OpenAI-compatible Responses and Conversations surface, but its retrieve
operation documents synchronous responses only. These routes add valuable
provider breadth, reasoning, tool, or state details; they do not add the
provider-owned asynchronous lifecycle and recoverable background stream.

Ollama supplies native NDJSON plus partial OpenAI compatibility. vLLM now
exposes create, retrieve, and cancel Responses endpoints, but documents
Responses as non-background. Both remain useful attached-runtime proofs after
an explicit deployment is selected. Their current communication shapes mostly
repeat attached llama.cpp or direct Responses framing.

Evidence:

- [Z.AI Chat Completion](https://docs.z.ai/api-reference/llm/chat-completion)
- [DeepSeek Chat Completion](https://api-docs.deepseek.com/api/create-chat-completion)
- [Alibaba Model Studio Responses](https://help.aliyun.com/en/model-studio/qwen-api-via-openai-responses)
- [Alibaba response retrieval](https://help.aliyun.com/en/model-studio/retrieve-a-response)
- [Ollama streaming](https://docs.ollama.com/api/streaming)
- [Ollama OpenAI compatibility](https://docs.ollama.com/api/openai-compatibility)
- [vLLM OpenAI-compatible server](https://docs.vllm.ai/en/latest/serving/online_serving/openai_compatible_server/)

## Comparison

| Candidate | New information | Main pressure | Rank |
| --- | --- | --- | --- |
| OpenAI Responses background | provider-owned async run, poll state, native cancel, SSE cursor reattachment, temporary retention | remote operation versus local connection, explicit retention, management requests versus retry | 1 |
| Cursor Cloud Agents | durable cloud agent, run-scoped SSE recovery, provider VM and repository mutation | GitHub authority, remote workspace ownership, deletion, beta support | 2 |
| Claude Agent SDK | external session store, deferred input, rich permission policy | Python/TypeScript sidecar and overlap with existing harness roles | 3 |
| Kimi Agent Rust Wire | lightweight native harness binary and new protocol | experimental independent release, manual API key, lifecycle overlap | 4 |
| ACP Rust SDK or remote transport | shared implementation and future remote topology | Preview SDK, unfinished remote transport, current ACP overlap | 5 |
| Qwen, Pi, Z.AI, DeepSeek, Alibaba, Ollama, vLLM | provider and runtime breadth | mostly repeats JSONL, HTTP/SSE, RPC, or attached-server shapes | 6 |

## Decision

Select OpenAI Responses background mode as the next proof.

The route is direct model inference through the provider-supported public API,
not Codex harness execution. It uses one exact host-approved OpenAI API
endpoint, one public-API key lease for that audience, provider-metered API
billing, and provider support authority. ChatGPT login, ChatGPT subscription
entitlement, Codex login, delegated harness credentials, and community OAuth
routes are excluded.

The exact first subset is:

- one explicitly configured model route; no model or endpoint default
- one structured text run with a positive consumer-supplied output bound
- `background=true`, `stream=true`, and `store=false`
- explicit acceptance of required temporary provider retention before effects
- one provider inference attempt
- one initial SSE attachment and bounded cursor reattachment
- native provider cancellation when a response id is known
- text output, terminal status, token usage, and safe request correlation
- no tools, search, attachments, files, structured output, conversation,
  previous-response continuation, webhooks, Batch API, retry, or fallback

Create, retrieve or stream-reattach, and cancel are management requests for
one inference attempt. They are not hidden inference retries. The first proof
does not expose durable attach after process restart or let a consumer persist
a resumable background-run binding.

## Missing Shared Boundary

Contract 021 must settle before production:

- explicit provider-background-execution and temporary-retention selection
- provider run reference, remote status, provider event cursor, runtime run,
  and common event sequence separation
- provider operation lifetime versus each HTTP or SSE attachment
- bounded reattachment that cannot replay, duplicate, skip, or start a second
  inference attempt
- provider-native cancellation, idempotence, terminal races, and the case
  where a disconnect occurs before the provider reference is known
- cancellation or deadline when provider stop cannot be confirmed
- endpoint and API-key lease lifetime across create, reattach, and cancel
- ordered local task join before credential release even when remote work may
  still be running
- temporary provider retention as explicit access evidence rather than a
  misleading `store=false` privacy claim
- safe provider status, usage, rate, and request-correlation projection

No provisional product spec is needed for the bounded subset. It has no
durable detach, cross-process resume, repository mutation, webhook, provider
tool, subscription, or default-selection decision.

## Deterministic Seam

Freeze a dated official-schema and loopback HTTP/SSE corpus for:

- exact create request and background response identity
- queued, in-progress, completed, incomplete, failed, and cancelled states
- ordered provider `sequence_number` cursor handling
- connection loss followed by bounded `starting_after` reattachment
- duplicate, gap, mismatch, unknown-event, and malformed-event failure
- idempotent cancellation and completion-versus-cancel races
- disconnect before response identity and unconfirmed remote cancellation
- positive output bound, `store=false`, and explicit retention requirement
- usage, rate, request correlation, redaction, deadline, and joined cleanup

Default QA needs no OpenAI credential, account, external request, or paid
inference. Any installed or live probe remains separately gated.

## Promotion

- durable background-run and retention rules: Contract 021 through card 074
- exact fixtures and production proof: g01 roadmap 023 and cards 074-075
- later cloud-harness gate: Cursor repository and remote-workspace authority
