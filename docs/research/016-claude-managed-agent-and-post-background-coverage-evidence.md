# 016 Claude Managed Agent And Post-Background Coverage Evidence

Status: promoted
Owner: Tom
Updated: 2026-07-21

## Question

Which current provider or transport route adds the most architectural
information after the OpenAI background proof without making repository
mutation, local containment, provider choice, or deployment policy implicit?

## Method

Official provider documentation, maintained-project documentation, current
RFD status, and realized Swallowtail contracts were checked on 2026-07-21. No
credential source, provider account, repository integration, local model, or
live endpoint was used.

Candidates were compared on harness ownership, resource identity, retention,
retry, event authority, callback behavior, cleanup, topology, access,
metering, support status, and deterministic fixture quality.

## Currentness Delta

Research 015 compared the local Claude Agent SDK. Anthropic now also exposes
Claude Managed Agents, a distinct provider-hosted harness in beta. It is
enabled by default for Claude API accounts and uses the
`managed-agents-2026-04-01` beta header.

The hosted surface separates four resources:

- a reusable, versioned agent definition containing model, system, tools,
  MCP servers, and skills
- an environment configuration describing the remote sandbox template
- a session that binds one agent version to one environment
- persisted events that drive and observe the session

The implementation recheck fixed three wire details. A cloud environment may
use limited networking with an explicit empty `allowed_hosts` list while both
MCP-server and package-manager allowances are false. Session creation may use
`agent_with_overrides` to pin the version, replace the model and full custom-
tool list, and clear MCP servers and skills without mutating the agent. The
current API event union names interruption `user.interrupt`; an older curl
example using `interrupt` is not authoritative for the frozen corpus.

Each cloud session receives a fresh Linux container. Sessions preserve
conversation history until deletion and checkpoint sandbox state for 30 days
after the last activity. Managed Agents is not eligible for Zero Data
Retention or HIPAA BAA coverage. Session deletion permanently removes the
session record, events, and associated sandbox, but not independent agents,
environments, files, memory stores, vaults, or skills.

Evidence:

- [Claude Managed Agents overview](https://platform.claude.com/docs/en/managed-agents/overview)
- [Agent setup](https://platform.claude.com/docs/en/managed-agents/agent-setup)
- [Cloud environment setup](https://platform.claude.com/docs/en/managed-agents/environments)
- [Start a session](https://platform.claude.com/docs/en/managed-agents/sessions)
- [Session operations](https://platform.claude.com/docs/en/managed-agents/session-operations)

## Lifecycle And Event Authority

Managed sessions move through `idle`, `running`, `rescheduling`, and
`terminated`. `rescheduling` means Anthropic is automatically retrying after a
transient error. Swallowtail cannot claim one provider inference attempt for
this route and must require explicit acceptance of provider-managed recovery.

Communication is event-based. User messages start or continue work; a user
interrupt redirects active execution. Custom tool use and tool confirmation
pause the session at `idle` with `requires_action` until correlated results or
decisions arrive.

Persisted events are authoritative. Optional `event_start` and `event_delta`
previews are best effort, connection-local, non-replayable, and never final.
After disconnect, the client reopens the stream and reconciles against
persisted event history; it cannot request missed preview deltas. The first
proof excludes previews and consumes authoritative buffered events only.

Evidence:

- [Session event stream](https://platform.claude.com/docs/en/managed-agents/events-and-streaming)
- [Permission policies](https://platform.claude.com/docs/en/managed-agents/permission-policies)
- [Webhook delivery](https://platform.claude.com/docs/en/managed-agents/webhooks)

## Access And Metering

The route uses the first-party Claude API endpoint, a Claude API key, API
billing, and provider beta support. It is not Claude subscription, Claude Code
login, Agent SDK subprocess access, Bedrock, or another cloud marketplace.

Billing has two independent dimensions: model tokens and time spent in
`running` session status. Idle, rescheduling, and terminated time is not billed
as session runtime. The session object exposes cumulative token usage; the
first proof must not infer exact billed runtime cost from local elapsed time.

Managed Agent create and read endpoints have separate organization limits.
The general organization Rate Limits API does not expose Managed Agents
limits, so response evidence and documented service limits remain distinct.

Evidence:

- [Claude API overview](https://platform.claude.com/docs/en/api/overview)
- [Claude Managed Agents pricing](https://platform.claude.com/docs/en/about-claude/pricing#claude-managed-agents-pricing)
- [Organization Rate Limits API](https://platform.claude.com/docs/en/manage-claude/rate-limits-api)

## Bounded First Proof

Select one resource-free structured harness run, not a remote repository
agent:

- one exact first-party Claude API endpoint and API-key lease
- one operator-supplied agent id pinned to one exact agent version and model
  route
- one Swallowtail-owned ephemeral cloud environment with limited networking,
  no package-manager or MCP allowance, and no consumer files
- one Swallowtail-owned session created inside that environment
- one text task with an explicit deadline and bounded declared custom tools
- no provider built-in tools, MCP servers, skills, multiagent, file resources,
  GitHub integration, vaults, memory, webhooks, outcomes, schedules, or
  research-preview features
- authoritative persisted events only; no best-effort preview projection
- explicit acceptance of durable provider retention and provider-managed
  rescheduling
- native interrupt on cancellation or deadline
- session deletion followed by environment deletion before credential release

The provider agent definition remains operator-owned. Swallowtail may retrieve
and validate the pinned version but does not create, update, archive, or delete
it. The temporary environment and session are owned by the driver and must be
deleted on every terminal path. Failure to confirm deletion remains degraded
cleanup and cannot be reported as remote-state removal.

This subset exercises a real remote harness, state machine, custom callback
wait, provider-owned recovery, durable retention, persisted event recovery,
remote resource ownership, and two-dimensional metering without granting
repository or filesystem authority.

## Remaining Candidate Recheck

Cursor Cloud Agents remains strong but still combines a dedicated VM, repo
clone, branch or PR mutation, GitHub authority, artifacts, and durable agent
lifecycle. Selecting its repository posture remains product policy.

ACP's Streamable HTTP and WebSocket RFD moved to Active on 2026-07-02, but the
reference implementation is still in progress. SDK support, protocol-version
headers, resumability, security hardening, and standardized reconnect remain
future phases. It is not ready for a production Swallowtail transport claim.

Qwen Code's bounded headless route remains a JSON or stream-JSON subprocess.
Its ACP daemon still does not apply the same wall-time and tool-call budgets.
Pi remains a TypeScript SDK or JSON-RPC subprocess. Kimi Agent Rust remains an
experimental Wire-only binary with manual API-key access and independent
release cadence. These routes are useful breadth but repeat realized local
harness lifecycle.

Z.AI and DeepSeek remain hosted Chat Completions SSE routes. Alibaba Responses
supports stored retrieval but still documents synchronous execution only.
Ollama and SGLang remain attached native or OpenAI-compatible streaming
servers. vLLM now exposes background Responses management, a currentness
change from Research 015, but it repeats the realized attached-runtime and
background-run shapes unless a separate serving deployment is selected.

Evidence:

- [Cursor SDK and Cloud Agents announcement](https://forum.cursor.com/t/cursor-sdk-cloud-agents-api-updates/159284)
- [ACP Streamable HTTP and WebSocket RFD](https://agentclientprotocol.com/rfds/streamable-http-websocket-transport)
- [Qwen Code headless mode](https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/)
- [Pi coding-agent SDK](https://github.com/earendil-works/pi/blob/main/packages/coding-agent/docs/sdk.md)
- [Kimi Wire and Rust agent](https://moonshotai.github.io/kimi-cli/en/customization/wire-mode.html)
- [Z.AI Chat Completion](https://docs.z.ai/api-reference/llm/chat-completion)
- [DeepSeek Chat Completion](https://api-docs.deepseek.com/api/create-chat-completion)
- [Alibaba response retrieval](https://help.aliyun.com/en/model-studio/retrieve-a-response)
- [Ollama streaming](https://docs.ollama.com/api/streaming)
- [vLLM online serving](https://docs.vllm.ai/en/latest/serving/online_serving/)
- [SGLang OpenAI-compatible serving](https://docs.sglang.io/docs/basic_usage/overview)

## Comparison

| Candidate | New information | Main pressure | Rank |
| --- | --- | --- | --- |
| Claude Managed Agents | provider-hosted harness, versioned agent, ephemeral environment, durable session, provider rescheduling, persisted events, runtime metering | remote resource ownership, durable retention, provider recovery, authoritative replay, deletion truth | 1 |
| Cursor Cloud Agents | durable cloud agent, repo VM, run recovery, branch/PR and artifact lifecycle | repository, GitHub, remote mutation, artifact, deletion policy | 2 |
| ACP remote transport | shared HTTP/2 SSE/WebSocket topology with connection/session split | Active RFD, incomplete SDK, reconnect, resumability, security hardening | 3 |
| Qwen, Pi, Kimi Rust | additional local harnesses and budgets | repeats subprocess or language-sidecar shapes; experimental drift | 4 |
| Z.AI, DeepSeek, Alibaba | direct provider and tool breadth | repeats hosted JSON/SSE or stored synchronous response shapes | 5 |
| Ollama, vLLM, SGLang | additional attached serving facades | repeats attached HTTP and background lifecycle without new serving authority | 6 |

## Decision

Select Claude Managed Agents as the next proof. Use the bounded resource-free
subset above. It creates no consumer repository, grants no provider filesystem
or external network authority, requires no local container, and selects no
default model or provider route.

The choice is evidence-led rather than a consumer preference. Claude Managed
Agents now adds more new contract pressure than Cursor while avoiding Cursor's
unsettled repository policy. Its first-party beta support, default API-account
availability, stable resource endpoints, and offline fixture seam are strong
enough for a bounded pre-1.0 proof.

## Required Promotion

Contract 022 must settle:

- provider agent definition, version, environment, session, sandbox, persisted
  event, runtime run, and stream attachment identity
- operator-owned provider configuration versus Swallowtail-owned ephemeral
  remote resources
- explicit durable provider retention and delete-on-close authority
- provider-managed rescheduling versus Swallowtail retry
- authoritative persisted events versus best-effort connection previews
- disconnect recovery through history reconciliation rather than preview replay
- callback and confirmation correlation while the provider state is idle
- native interrupt, terminal races, deletion confirmation, and degraded cleanup
- token usage versus provider-running time and inferred versus exact cost
- endpoint and credential lifetime through session and environment deletion

No provisional product spec is required for the resource-free subset. Files,
repositories, GitHub, built-in provider tools, external sandbox network, MCP,
skills, multiagent, schedules, webhooks, memory, and cross-process resume remain
excluded.

## Concrete Sequence

1. Promote Contract 022 and the minimum shared durable-retention,
   provider-recovery, and remote-deletion records.
2. Freeze a dated first-party REST/SSE corpus and deterministic loopback seam.
3. Implement the resource-free Managed Agents structured-run driver.
4. Prove local and remote-authoritative host topology, callbacks, reconnect,
   interruption, deletion, redaction, and joined cleanup.
5. Return to Cursor only after the operator selects repository and remote-
   mutation authority.

## Promotion

- durable remote-harness rules: Contract 022
- implementation sequence: g01 roadmap 025 and cards 077-079
- later repository-backed cloud-harness gate: Cursor remains policy-bound
