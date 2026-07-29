# 061 Residual Feature Matrix `No` Currentness And Tranche Selection

Status: promoted
Owner: Tom
Date: 2026-07-29

## Question

Which of the 61 remaining unverified-newer, interactive-session,
realtime-media, and billed-cost `No` cells are applicable selected-surface
gaps, category errors, separate-route work, or exact contract and corpus
candidates?

## Method

Evidence was accessed on 2026-07-29.

- parsed the canonical 22-solution CSV and froze all 61 starting cells
- compared every cell with Contracts 004, 009, 011-012, 016-017, 026, 029,
  031, and 037
- inspected realized prepared APIs, exact version claims, fixtures, and
  provider observations
- checked current first-party or maintained-project documentation for every
  plausible interactive, realtime-media, compatibility, and billed-cost path
- kept provider capability separate from the selected route and operation

No executable, credential, account, provider request, paid operation,
container, harness server, or model server was used.

## Classification

The table accounts for every starting `No` exactly once.

- `I` — no runtime-observed ordered interface axis; policy is not applicable
- `C` — exact contract or corpus candidate; retain `No` until realized
- `O` — operation shape does not apply; change to `Not applicable`
- `R` — provider capability exists on a separate route; retain `No`
- `B` — selected access or compute has no provider billing boundary; change
  to `Not applicable`
- `S` — applicable selected surface supplies no exact evidence; retain `No`
- `E` — a cost-like field exists but is not provider-authoritative billed
  cost; retain `No`
- dash — the starting cell was already positive or not applicable

| Solution route | Unverified newer | Interactive | Realtime media | Billed cost |
| --- | --- | --- | --- | --- |
| `qwen.headless` | — | C | O | S |
| `alibaba.conversations` | I | — | R | S |
| `bedrock.catalogue; bedrock.runtime` | I | C | R | S |
| `claude-agent.acp` | — | — | O | E |
| `claude-code.headless` | — | C | O | B |
| `anthropic.managed-agent` | I | C | O | S |
| `anthropic.messages` | I | — | O | S |
| `pi.rpc` | — | — | O | E |
| `deepseek.continuation` | I | — | O | S |
| `gemini-cli.acp + gemini-cli.headless` | — | — | O | S |
| `gemini.live` | I | O | — | S |
| `llama-cpp.attached` | C | C | O | B |
| `llama-cpp.owned` | C | — | — | — |
| `kimi-code.acp + kimi-code.headless` | — | — | O | B |
| `kimi-code.local-server` | — | — | O | B |
| `kimi-platform.chat` | I | C | O | S |
| `ollama.attached` | — | C | O | B |
| `codex.app-server; codex.exec` | — | — | O | S |
| `openai.realtime` | I | O | — | S |
| `openai.background` | I | O | O | S |
| `opencode.http` | — | — | O | E |
| `xai.responses-websocket` | I | — | R | — |

Exact totals:

| Classification | Cells | Final value |
| --- | ---: | --- |
| interface axis not runtime ordered | 10 | `Not applicable` |
| contract or corpus required | 9 | `No` |
| operation shape not applicable | 19 | `Not applicable` |
| separate route and contract required | 3 | `No` |
| no provider billing boundary | 5 | `Not applicable` |
| selected-surface absence | 12 | `No` |
| non-authoritative cost evidence | 3 | `No` |
| **Total** | **61** | **34 `Not applicable`, 27 `No`** |

The full matrix therefore moves from 270 `No` and 182 `Not applicable` cells
to 236 `No` and 216 `Not applicable` cells. No capability becomes `Yes` in
this audit.

## Unverified-Newer Findings

Ten cells do not have a runtime-observed ordered external interface.

Hosted API facade labels, embedded SDK revisions, and service revisions are
qualification identities, not semver observations discovered on a client
device. Contract 029 requires exact qualification for those opaque axes.
`UnverifiedNewer` admission is therefore not a meaningful runtime policy for
Alibaba Conversations, Bedrock, Anthropic Managed Agents, Anthropic Messages,
DeepSeek, Gemini Live, Kimi Platform, OpenAI Realtime, OpenAI background, or
xAI Responses WebSocket.

llama.cpp is different. The selected attached and owned routes observe exact
build and commit identities, while upstream publishes ordered build tags.
Current upstream releases are already newer than the selected b9910 and
b10069 points. Upstream does not provide semver or a stable channel and warns
consumers to test before updating. Swallowtail may eventually admit exact
newer build observations as visible mileage-may-vary evidence, but only after:

- separating the ordered build number from the exact commit identity
- proving build observation and comparison
- freezing REST and process-lifecycle drift boundaries
- retaining the existing guaranteed points unchanged

Both llama.cpp cells remain `No` pending that contract and corpus work.

## Interactive-Session Findings

Seven selected routes have a plausible reusable multi-turn operation:

| Route | Continuity owner | Current evidence |
| --- | --- | --- |
| `qwen.headless` | harness-retained local transcript, resumed by a new process | exact selected CLI source accepts `--continue` and `--resume` with a headless prompt |
| `claude-code.headless` | harness-retained local transcript, resumed by a new process | current official CLI exposes print-mode continuation and resume |
| `bedrock.runtime` | consumer-supplied message history | Bedrock documents Converse as multi-turn chat |
| `anthropic.managed-agent` | provider-owned durable session | a session retains history and accepts later user events |
| `llama-cpp.attached` | consumer-supplied message history | chat completions accepts an ordered `messages` transcript |
| `kimi-platform.chat` | consumer-supplied message history | Kimi documents multi-turn chat by replaying prior messages |
| `ollama.attached` | consumer-supplied message history | `/api/chat` accepts chat history as ordered messages |

These are not one lifecycle hidden behind one generic call. Headless harnesses
start another owned process against retained provider state. Managed Agents
continues a remote provider-owned session. Bedrock, llama.cpp, Kimi Platform,
and Ollama require Swallowtail to retain and replay bounded consumer-owned
history.

Gemini Live and OpenAI Realtime already expose the dedicated realtime-media
session role. They do not need a second ordinary interactive-session claim.
OpenAI background is one retained response with retrieval and reattachment,
not a reusable thread. Those three cells become `Not applicable`.

## Realtime-Media Findings

Sixteen selected solutions are installed text harnesses, ordinary hosted
inference, retained responses, or attached text runtimes. Streaming text,
image attachment, audio files, and WebSocket transport do not create a
realtime-media operation. Their cells become `Not applicable`.

Three providers expose useful realtime media on routes Swallowtail has not
selected:

- Alibaba Model Studio Qwen-Omni-Realtime uses separate workspace-scoped
  WebSocket, WebRTC, or AOQ endpoints for streaming audio and image input and
  audio or text output.
- Amazon Nova Sonic uses
  `InvokeModelWithBidirectionalStream`, not the selected Bedrock Converse
  EventStream branch.
- xAI Voice uses `wss://api.x.ai/v1/realtime`, not the selected
  `wss://api.x.ai/v1/responses` route.

These remain `No` as separate-route and contract work. Borrowing any of them
would flatten endpoint, credential, model, protocol, lifecycle, and version
identity.

## Billed-Cost Findings

No billed-cost `No` is a false negative.

Contract 016 requires the exact amount declared charged by the provider for
one attempt, with exact currency and scale. Token usage, published rates,
consumer multiplication, harness estimates, and daily account reports do not
qualify.

Current provider surfaces confirm the distinction:

- Bedrock invocation records expose token counts. AWS documents estimated
  per-request calculation and invoice reconciliation through aggregated Cost
  and Usage Reports, which carry no request identifier.
- Anthropic Messages reports usage dimensions. Its separate organization Cost
  API reports daily cost buckets rather than one attempt's receipt.
- OpenAI Realtime reports token usage in `response.done` and tells consumers
  to estimate costs from usage. The Responses family likewise reports usage;
  organization costs are a separate aggregate surface.
- Alibaba Responses and DeepSeek Chat expose token usage, not an exact charged
  amount.
- Gemini Live exposes modality token counts, not a currency amount.
- Kimi Platform Chat exposes token usage and published pricing, not an exact
  per-attempt bill.

Claude Agent ACP, Pi, and OpenCode expose cost-like fields, but the selected
harness is the reporter. Pi calculates cost from configured model pricing.
OpenCode's selected schema has no currency. Claude Agent can operate through
different subscription and API-key access profiles, while its update is not a
provider billing receipt. All three remain `No`.

Five cells are category errors:

- Claude Code headless uses the selected subscription access surface.
- Kimi Code installed and local-server routes use membership-backed harness
  access.
- llama.cpp attached and Ollama attached use local compute.

Those routes may have subscription fees or machine costs, but there is no
per-attempt provider billing boundary for Swallowtail to observe. Their cells
become `Not applicable`.

## Ranked Candidates

1. `qwen.headless` plus `ollama.attached` interactive sessions
2. `anthropic.managed-agent` interactive sessions
3. Bedrock and Kimi Platform consumer-owned direct continuation
4. Claude Code headless continuation
5. llama.cpp interactive continuation
6. llama.cpp ordered unverified-newer admission
7. separate Alibaba, Bedrock, and xAI realtime-media routes

The first pair is the smallest high-information tranche:

- Qwen proves harness-retained transcript continuation across owned child
  processes.
- Ollama proves resource-free consumer-owned transcript replay across direct
  HTTP turns.
- both are deterministic fixture-first paths
- neither requires a live provider credential, container, or owned model
  server during default validation
- Claude Code would initially duplicate Qwen's lifecycle, while Managed
  Agents adds paid remote resource lifecycle before the shared session split
  is proven

Card 116 should verify contract fit and freeze exact Qwen `0.19.11` plus
Ollama `0.14.0..=0.32.1` corpora. No other candidate enters card 117.

## Sources

- [Qwen headless mode](https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/)
- [Qwen Code `v0.19.11` CLI configuration](https://github.com/QwenLM/qwen-code/blob/v0.19.11/packages/cli/src/config/config.ts)
- [Amazon Bedrock supported APIs](https://docs.aws.amazon.com/bedrock/latest/userguide/apis.html)
- [Amazon Bedrock Converse](https://docs.aws.amazon.com/bedrock/latest/userguide/conversation-inference.html)
- [Claude Managed Agents sessions](https://platform.claude.com/docs/en/managed-agents/sessions)
- [Claude Managed Agents event stream](https://platform.claude.com/docs/en/managed-agents/events-and-streaming)
- [Ollama chat API](https://docs.ollama.com/api/chat)
- [llama.cpp server API](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md)
- [llama.cpp releases](https://github.com/ggml-org/llama.cpp/releases)
- [llama.cpp breaking-change discussion](https://github.com/ggml-org/llama.cpp/discussions/9276)
- [Kimi multi-turn chat](https://platform.kimi.ai/docs/guide/engage-in-multi-turn-conversations-using-kimi-api)
- [Qwen-Omni-Realtime](https://help.aliyun.com/en/model-studio/realtime)
- [Amazon Nova bidirectional streaming](https://docs.aws.amazon.com/nova/latest/userguide/speech-bidirection.html)
- [xAI Voice WebSocket reference](https://docs.x.ai/developers/rest-api-reference/inference/voice)
- [AWS per-request cost metadata](https://docs.aws.amazon.com/bedrock/latest/userguide/cost-mgmt-request-metadata.html)
- [Anthropic Usage and Cost API](https://platform.claude.com/docs/en/manage-claude/usage-cost-api)
- [OpenAI Realtime cost guidance](https://developers.openai.com/api/docs/guides/realtime-costs)
- [Alibaba Responses usage](https://help.aliyun.com/en/model-studio/qwen-api-via-openai-responses)
- [DeepSeek Chat response usage](https://api-docs.deepseek.com/api/create-chat-completion)
- [Gemini Live API reference](https://ai.google.dev/api/live)
- [Kimi Chat API](https://platform.kimi.ai/docs/api/chat)
- [Research 048](048-harness-usage-evidence-currentness-and-corpora.md)
- [Contract 016](../contracts/016-connection-scoped-direct-sessions-and-billed-cost.md)
- [Contract 026](../contracts/026-realtime-media-direct-session-boundary.md)
- [Contract 029](../contracts/029-interface-version-qualification-and-compatibility.md)

## Promotion

- Reclassified 34 category-error cells as `Not applicable`.
- Retained 27 exact `No` cells with machine-checkable reasons.
- Selected Qwen headless and Ollama attached interactive sessions for cards
  116-117.
- Left billed-cost implementation closed until a selected provider surface
  reports an exact per-attempt charge.
