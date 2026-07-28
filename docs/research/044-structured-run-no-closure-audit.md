# 044 Structured-Run No-Closure Audit

Status: promoted
Owner: Tom
Date: 2026-07-27

## Question

Which `structured_run = No` values in the 21-row solution matrix describe a
missing Swallowtail role rather than an upstream limitation?

## Method

The audit started from all twelve `No` rows. It checked current provider
documentation, maintained harness documentation and source, qualified
Swallowtail corpora, existing driver lifecycles, and the solution-facade
grouping rule.

No credential, account, provider request, model invocation, process install,
deployment, or paid operation was used.

## Result

Eight rows have a definite bounded-run surface. Kimi local server has one with
explicit durable session retention. llama.cpp owned serving is not an
inference operation at its prepared solution boundary. Gemini Live and OpenAI
Realtime remain realtime-media sessions.

| Current `No` solution | Upstream bounded surface | Disposition |
| --- | --- | --- |
| Alibaba Model Studio Conversations and Responses | one Responses request with optional streaming and `store=false` | add a resource-free structured branch |
| Claude Agent ACP | one ACP session, one terminal prompt, native close where qualified | add an exact ACP single-turn projection |
| Pi RPC | one prompt through an ephemeral RPC process; print and JSON modes corroborate one-shot behavior | add an exact RPC single-turn projection |
| DeepSeek Open Platform continuation | one buffered or streamed Chat Completions request | add a no-tool structured branch beside continuation |
| Gemini CLI ACP | one ACP turn; first-party headless JSON and JSONL also run once and exit | add a separately qualified headless structured branch |
| Kimi Code ACP | one ACP turn; maintained Kimi Code also exposes non-interactive streamed output | add a separately qualified headless structured branch |
| Kimi Code local server | create one session, run one prompt, close operation work | add a structured branch with durable retention accepted and no deletion claim |
| OpenCode HTTP server | create one session, prompt once, then delete; `opencode run --attach` corroborates the shape | add an attached HTTP structured branch |
| xAI Responses WebSocket | first `response.create` is a complete streamed response; continuation is optional | add a one-response connection branch |
| llama.cpp owned server lifecycle | serving start returns an endpoint; attached route owns inference | change to `Not applicable` |
| Gemini Live | bounded turns exist inside a realtime-media connection | keep `No`; add a separate Gemini generate-content solution later |
| OpenAI Realtime | bounded responses exist inside a realtime-media connection | keep `No`; OpenAI background Responses already owns structured execution |

After implementation the matrix target is:

- 18 `Yes`
- 2 `No`
- 1 `Not applicable`

## Operation Boundary

A structured run is one consumer operation, not necessarily one provider
request or a stateless transport. An exact provider adapter may internally:

1. open one operation-private process, connection, or session
2. submit one consumer request
3. stream ordered progress, output, callbacks, usage, and failure evidence
4. reach one terminal outcome
5. close and join operation-owned runtime work

That projection does not make the underlying transport stateless. Session,
connection, provider-retention, callback, cancellation, and cleanup truth
remain visible in the driver capability, immutable plan, operation policy,
events, terminal result, and prepared evidence.

There is no generic interactive-session-to-run adapter. Each production route
must register and qualify `StructuredRun` independently. Unsupported request
features fail before provider effects.

## Retention Decision

The operator accepts Kimi local-server structured runs even though the exact
server exposes no qualified thread deletion.

The route therefore requires:

- `ProviderRetentionPolicy::DurableAllowed`
- an immutable durable-provider-session capability
- no delete, hard-delete, or secure-erasure claim
- no cleanup wording that implies provider history removal
- explicit session archive only when the exact route performs it

Closing the run joins Swallowtail-owned REST, WebSocket, task, timer,
credential, and optional owned-process work. It does not delete the Kimi
thread.

## Kimi Currentness Delta

Research 046 supersedes this currentness gate: exact `0.29.1` and `0.29.2`
source, corpora, and behavior milestones are now qualified. Card 078 may use
that evidence.

At the time of this audit, Swallowtail guaranteed Kimi Code `0.28.1` and
`0.29.0`. The maintained project had published `0.29.1` and `0.29.2`;
`0.29.2` was published on 2026-07-27. Research 046 later compared the selected
ACP, headless, local-server, lifecycle, and model-catalogue surfaces and
qualified both releases. A successful unverified execution was not used as
range evidence.

## Sequencing

1. Promote the single-turn projection and retention boundary.
2. Prove resource-free Alibaba and DeepSeek structured requests plus one xAI
   WebSocket response.
3. Prove installed and attached harness single-turn paths.
4. Qualify Kimi `0.29.2`, then add ACP/headless and retained local-server
   structured paths.
5. Correct llama.cpp owned and realtime matrix classifications.

The direct batch maximizes transport information before repeated harness
projections. Harness routes remain separately registered and versioned.

## Evidence

- [Alibaba Responses API](https://www.alibabacloud.com/help/en/model-studio/qwen-api-via-openai-responses)
- [ACP session close](https://agentclientprotocol.com/announcements/session-close-stabilized)
- [ACP terminal prompt response](https://agentclientprotocol.com/rfds/message-id)
- [Claude Code non-interactive mode](https://docs.anthropic.com/en/docs/claude-code/cli-usage)
- [Pi programmatic modes](https://github.com/earendil-works/pi/blob/main/packages/coding-agent/README.md)
- [DeepSeek Chat Completions](https://api-docs.deepseek.com/api/create-chat-completion)
- [Gemini CLI headless mode](https://geminicli.com/docs/cli/headless/)
- [Kimi Code command reference](https://www.kimi.com/code/docs/en/kimi-code-cli/reference/kimi-command.html)
- [OpenCode non-interactive run](https://opencode.ai/docs/cli/)
- [xAI Responses WebSocket](https://docs.x.ai/developers/advanced-api-usage/websocket-mode)
- [Gemini generate-content API](https://ai.google.dev/api/generate-content)
- [Kimi Code `0.29.2` release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.29.2)

## Promotion

- Contract 039 governs bounded single-turn projection.
- Contract 011 gains a provider-neutral projection conformance pack.
- Roadmap g02.022 sequences implementation.
