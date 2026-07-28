# 051 Session Continuity `No` Currentness And Tranche Selection

Status: promoted
Owner: Tom
Date: 2026-07-28

## Question

Which of the 58 starting load-session, resume-session, and
native-session-close `No` cells can the selected route support? Which first
tranche improves consumer continuity without flattening replay, attachment,
local teardown, archive, or deletion?

## Method

Evidence was accessed 2026-07-28.

- parsed the canonical 22-solution CSV by exact route and feature
- checked all 58 starting cells against realized prepared plans, bindings,
  fixtures, and lifecycle capabilities
- checked exact tags or revisions for maintained harnesses and attached
  servers
- used current official provider documentation for hosted and opaque
  interfaces
- kept composite-solution branches separate
- applied Contracts 017 and 038 before treating similarly named upstream
  operations as portable load, resume, or close
- used no executable, account, credential, provider request, container, or
  model server

No audited `No` has a realized prepared path. The matrix is internally
correct. The dispositions below classify future conversion, contract
expansion, ordering blockage, or honest absence. They do not claim current
Swallowtail support.

## Dispositions

| Code | Cells | Meaning |
| --- | ---: | --- |
| `R` | 7 | Exact selected surface fits existing Contracts 017 and 038; exact range corpus and implementation remain |
| `C` | 4 | Upstream continuity exists, but the selected Swallowtail route currently has operation-owned cleanup; a retained-session contract branch is required |
| `B` | 1 | Upstream advertises load, but exact response ordering cannot prove replay completion before readiness |
| `U` | 10 | Exact selected route exposes no qualifying history, replay-free attachment, or provider-native active-session close |
| `M` | 36 | The selected operation shape has no reusable provider session; process, request, run, connection, or serving lifecycle does not qualify |
| **Total** | **58** | Every starting `No` exactly once |

The seven `R` cells are actionable under existing shared contracts. The four
`C` cells are real upstream opportunities but require a deliberate retained
hosted-session branch. No native-close `No` is an honest conversion.

## Exact Cell Audit

A dash means the cell was already `Yes` or `Not applicable`; it is not part of
the 58-cell audit.

| Route | Load | Resume | Native close | Exact finding |
| --- | --- | --- | --- | --- |
| `qwen.headless` | `M` | `M` | `M` | Exact headless continuation restores context only to execute another prompt. It does not return a ready interactive attachment or bounded replay phase. |
| `alibaba.conversations` | `C` | `C` | `U` | Current Conversations can retrieve a conversation, list its items, and continue it. Contract 025 currently makes the selected conversation operation-owned and deletes it during cleanup. No separate active-session close exists. |
| `bedrock.catalogue; bedrock.runtime` | `M` | `M` | `M` | The selected catalogue and Runtime invocation branches have no reusable provider session. |
| `claude-agent.acp` | `R` | `R` | — | Exact `0.53.0..=0.61.0`, excluding withdrawn `0.58.0`, advertises ACP load and resume. Existing close preserves provider history. |
| `claude-code.headless` | `M` | `M` | `M` | Exact headless continuation executes a prompt and exits. Swallowtail also selects `--no-session-persistence`; no ready session handle or native close is exposed. |
| `anthropic.managed-agent` | `C` | `C` | `U` | Managed Agent sessions persist, expose event history, and accept later messages. The selected `AgentRun` route deletes its operation-owned session and environment during cleanup. Archive, interrupt, and delete are not native close. |
| `anthropic.messages` | `M` | `M` | `M` | Direct continuation is consumer-owned message replay. The provider exposes no reusable Messages session. |
| `pi.rpc` | `R` | `R` | `U` | Exact `0.80.10` RPC can switch to a persisted session and read ordered messages or entries. Process exit is not provider-native close. |
| `deepseek.continuation` | `M` | `M` | `M` | Continuation is consumer-owned request history, not provider session attachment. |
| `gemini-cli.acp + gemini-cli.headless` | `B` | `U` | `U` | Exact ACP load starts history streaming without awaiting replay completion before returning. Headless resume executes a prompt; ACP exposes no replay-free resume or native close. |
| `gemini.live` | `M` | `M` | `M` | Continuity is connection-scoped realtime state. Disconnect is not a reusable provider session or native close. |
| `llama-cpp.attached` | `M` | `M` | `M` | The attached inference server exposes model runtime state, not reusable conversation sessions. |
| `kimi-code.acp + kimi-code.headless` | — | — | `U` | ACP load and resume are already realized. Neither ACP nor headless exposes a qualifying native close. |
| `kimi-code.local-server` | `U` | — | `U` | Resume is realized. Session metadata and bounded WebSocket resynchronization do not provide complete durable history replay; abort, archive, and restore are not native close. |
| `kimi-platform.chat` | `M` | `M` | `M` | Chat requests have consumer-owned continuation and no reusable provider session. |
| `ollama.attached` | `M` | `M` | `M` | The attached runtime exposes model inference, not reusable conversation sessions. |
| `codex.app-server; codex.exec` | `R` | — | `U` | App-server resume can return reconstructed ordered turns; replay-free resume is already realized. Unsubscribe and idle unload do not provide an explicit provider-native close. |
| `openai.realtime` | `M` | `M` | `M` | Realtime state is connection-scoped. Connection teardown is not reusable-session close. |
| `openai.background` | `M` | `M` | `M` | A retained background response is a run resource, not an interactive provider session. |
| `opencode.http` | `R` | `R` | `U` | The server can retrieve a session, list its messages, and continue the exact session. Abort and delete are distinct from native close. |
| `xai.responses-websocket` | `M` | `M` | `M` | The selected one-response WebSocket route has connection-local response state, not a reusable session. |

## Feature Boundaries

### Load

Load attaches to a provider-owned durable session and transports bounded,
ordered provider history before returning a ready session. Metadata lookup,
consumer-local transcript reconstruction, connection resynchronization, or
starting another prompt does not qualify.

Codex, Claude Agent ACP, Pi RPC, and OpenCode have qualifying exact surfaces.
Gemini ACP does not: exact `0.51.0` and `0.52.0` return from load while history
streaming continues asynchronously, with no completion phase boundary.

### Resume

Resume attaches to the exact provider session without replay. It is not new
session creation, headless prompt continuation, or a load call whose returned
history Swallowtail silently discards.

Claude Agent ACP, Pi RPC, and OpenCode have qualifying exact surfaces. Codex
resume and both Kimi routes already have realized support.

### Native Close

Native close ends an active provider session while preserving exact
provider-state truth. It is not:

- process exit or connection drop
- local runtime cleanup
- unsubscribe, abort, cancel, or interrupt
- archive, delete, or secure erasure
- model-server shutdown

None of the 20 starting native-close cells exposes a qualifying operation on
its selected route. All remain `No`.

## Current Evidence

### Contract-Ready Routes

- [Codex app-server protocol](https://github.com/openai/codex/blob/main/codex-rs/app-server/README.md)
- [Claude Agent ACP `0.61.0` implementation](https://github.com/agentclientprotocol/claude-agent-acp/blob/v0.61.0/src/acp-agent.ts)
- [Pi RPC `0.80.10`](https://github.com/earendil-works/pi/blob/v0.80.10/packages/coding-agent/docs/rpc.md)
- [OpenCode server API](https://dev.opencode.ai/docs/server/)
- [OpenCode `1.18.4` OpenAPI](https://github.com/anomalyco/opencode/blob/v1.18.4/packages/sdk/openapi.json)

### Contract Expansion

- [Alibaba Model Studio Conversations](https://www.alibabacloud.com/help/en/model-studio/openai-compatible-conversations)
- [Anthropic Managed Agent sessions](https://platform.claude.com/docs/en/managed-agents/sessions)
- [Anthropic Managed Agent session operations](https://platform.claude.com/docs/en/managed-agents/session-operations)
- [Anthropic Managed Agent events and streaming](https://platform.claude.com/docs/en/managed-agents/events-and-streaming)

### Installed Harness Exclusions

- [Qwen Code `0.19.11` headless mode](https://github.com/QwenLM/qwen-code/blob/v0.19.11/docs/users/features/headless.md)
- [Claude Code session management](https://code.claude.com/docs/en/sessions)
- [Gemini CLI session management](https://geminicli.com/docs/cli/session-management/)
- [Gemini CLI `0.51.0` ACP load implementation](https://github.com/google-gemini/gemini-cli/blob/v0.51.0/packages/cli/src/acp/acpSessionManager.ts)
- [Gemini CLI `0.52.0` ACP load implementation](https://github.com/google-gemini/gemini-cli/blob/v0.52.0/packages/cli/src/acp/acpSessionManager.ts)

Kimi local-server findings retain the exact REST, WebSocket, and version
evidence promoted in [Research 040](040-kimi-code-local-server-route-evidence.md).
Codex, Claude Agent, and OpenCode version segmentation retains
[Research 037](037-codex-app-server-lifecycle-range-evidence.md),
[Research 038](038-acp-v1-and-claude-agent-lifecycle-currentness.md), and
[Research 039](039-opencode-session-deletion-range-evidence.md).

## First Tranche

Select five audited cells across three existing prepared adapters:

- Codex app-server: load
- Claude Agent ACP: load and resume
- OpenCode HTTP: load and resume

This tranche gives:

- the highest-value existing consumer route
- one shared-protocol load/resume proof
- one attached HTTP/SSE load/resume proof
- history-bearing load and replay-free resume under the same portable contract
- exact local and remote-authoritative topology evidence

It adds no provider, endpoint, credential mechanism, access profile, topology,
container, or product-policy choice. Pi RPC load and resume remain the next
contract-ready continuation tranche.

## Contract And Corpus Gate

Contracts 017 and 038 already settle the selected five cells. Card 093 should
not add a shared contract unless exact corpus work finds a contradiction.

Freeze these offline corpora before production changes:

- Codex `0.80.0..=0.145.0`: history-bearing resume responses across exact
  schema milestones; bounded ordered replay; replay-free resume selection;
  wrong thread, malformed history, overflow, cancellation, disconnect, and
  cleanup
- Claude Agent ACP `0.53.0..=0.61.0`, excluding `0.58.0`: load and resume
  negotiation; bounded replay completion; replay-free resume; opaque binding;
  mismatch, cancellation, close-with-retained-history, disconnect, and cleanup
- OpenCode `1.14.48..=1.18.4`: session detail, message-list pagination and
  ordering, exact-session continuation, opaque binding, mismatch, abort,
  disconnect, and attached-server cleanup across all qualified schema segments

Alibaba Conversations and Anthropic Managed Agents stay outside this tranche.
Their future route must make retained provider sessions explicit instead of
weakening operation-owned cleanup.

## Promotion

- Classified all 58 starting cells exactly once.
- Found seven contract-ready cells, four shared-contract expansion cells, one
  upstream ordering block, ten exact route absences, and 36 operation-shape
  mismatches.
- Found no realized matrix error.
- Kept all 20 native-close cells honestly `No`.
- Selected a five-cell, three-route tranche under existing shared contracts.
- Made card 093's exact version-corpus envelope ready.
