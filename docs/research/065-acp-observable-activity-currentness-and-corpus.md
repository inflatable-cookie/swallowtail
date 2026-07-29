# 065 ACP Observable Activity Currentness And Corpus

Status: promoted
Owner: Tom
Date: 2026-07-29

## Question

What exact stable ACP activity semantics and harness version milestones may
the shared protocol projection and Claude Agent, Gemini CLI, and Kimi Code
adapters rely on?

## Method

Evidence was accessed on 2026-07-29.

- checked the maintained ACP stable schema artifact, Rust schema and core SDK,
  and remote transport SDK
- compared stable ACP schema artifacts `v1.15.0..=v1.20.0`
- inspected exact tagged Claude Agent, Gemini CLI, and Kimi Code activity
  sources across every Swallowtail compatibility segment
- checked current stable releases beyond each qualified upper bound
- froze bounded shared and adapter-local fixtures
- reused existing lifecycle, continuity, access, and transport corpora

No executable, installation, authentication, provider request, model call,
account, consumer repository, or live transport was used.

## ACP Currentness

The current stable axes remain separate:

| Axis | Current exact evidence |
| --- | --- |
| wire protocol | ACP v1 |
| stable schema artifact | `schema-v1.20.0` |
| schema package | `agent-client-protocol-schema = 1.6.0` |
| Rust core SDK | `agent-client-protocol = 2.0.0` |
| Rust remote transport SDK | `agent-client-protocol-http = 2.0.0` |
| draft protocol | v2 alpha, excluded |
| remote transport RFD | Active, not a stable provider route |

The stable v1 session-update set is unchanged across the harness schema
artifacts selected here. It contains:

- user, assistant, and thought content chunks
- tool-call creation and partial updates
- authoritative full-plan replacement
- authoritative command, mode, and configuration updates
- partial session-info updates
- authoritative session usage evidence

Message ids are optional opaque identity. Tool-call ids are required. Plans
have no provider plan id in stable v1; an adapter may mint operation-local
identity but must retain replacement semantics.

`tool_call` initiates a tool record. `tool_call_update` changes only supplied
fields. Its content and location collections replace prior collections.
Pending and in-progress statuses are non-terminal; completed and failed are
terminal. A tool update does not settle a permission request or the prompt.

`session/request_permission` remains a separate bidirectional request.
`session/prompt` response stop truth remains separate terminal truth. Neither
may be inferred from the tool lifecycle.

## Thought Disclosure

The stable schema describes `agent_thought_chunk` as agent reasoning and sends
it through the real-time client update channel. It is client-display content,
not an undocumented server trace.

That does not make every thought chunk a reasoning summary:

- Claude Agent and Kimi use it for readable thought output.
- Gemini uses it for model thought subject and description, but also for
  operational file-access warnings.
- ACP does not claim completeness, hidden chain-of-thought, or one semantic
  subtype.

The shared decoder must therefore preserve an ACP thought chunk without
choosing provider policy. Each exact adapter decides whether the content is a
portable reasoning summary, warning activity, another display activity, or
excluded. Contract 044 now states this explicitly.

## Raw And Metadata Boundary

Stable ACP tool records include typed display fields plus `rawInput`,
`rawOutput`, and open `_meta`.

Only bounded typed display fields are shared projection candidates:

- title, kind, status
- typed content blocks
- typed diff content
- typed locations

`rawInput`, `rawOutput`, and untyped `_meta` remain excluded. A provider
adapter may qualify a bounded typed metadata subset. This is not permission to
expose a raw ACP or provider envelope.

Mode, available-command, configuration, session-info, and usage updates remain
typed session metadata or evidence. They do not become agent activity to fill
a timeline.

## Claude Agent Range

The guaranteed range remains `0.53.0..=0.61.0`, excluding unpublished
`0.58.0`, across four existing behavior segments.

| Segment | ACP SDK and schema | Activity delta |
| --- | --- | --- |
| `0.53.0` | SDK `1.0.0`, schema `v1.15.0` | baseline message, thought, plan, tool, command, mode, usage, and permission shapes |
| `0.54.0..=0.59.0` except `0.58.0` | SDK `1.1.0..=1.2.1`, schema `v1.16.0..=v1.17.0` | session configuration display joins the same stream |
| `0.60.0` | SDK `1.2.1`, schema `v1.17.0` | provider capability metadata; no portable activity widening |
| `0.61.0` | SDK `1.3.0`, schema `v1.18.0` | steering and richer tool metadata; exact typed subset stays adapter-owned |

Claude-specific tool metadata includes tool name, parent tool-use identity,
non-execution reason, and terminal identity. Parent identity is display
correlation, not cross-route authority. Subagent text depends on provider
options and client capabilities.

`0.62.0` has the same `acp-agent.ts` digest as `0.61.0`. `0.63.0` changes tool
progress and denial correlation. Both remain permitted unverified-newer on
the `0.61.0` activity guarantee.

## Gemini CLI Range

The ACP guarantee remains exact `0.51.0`, SDK `0.16.1`, stable schema
`v1.19.0`.

The qualified source emits:

- assistant, user-history, and thought chunks
- tool-call creation and terminal updates
- typed tool text, diff, and locations
- available-command replacement
- permission requests and prompt completion

It does not emit stable ACP plan, usage, session-info, configuration, or
current-mode updates. A provider mode change is emitted as assistant display
text using `[MODE_UPDATE]`; it is not authoritative mode evidence.

Gemini `0.53.0` is current stable. Its ACP session source is byte-identical to
`0.51.0`, but the complete harness release remains unqualified. It may run as
unverified-newer without widening the `0.51.0` activity profile.

## Kimi Code Range

The ACP guarantee remains exact `0.28.1` plus `0.29.0..=0.29.2`, SDK
`0.23.0`, stable schema `v1.19.1`.

The qualified source emits:

- assistant and thought chunks
- lazy tool creation when argument deltas precede dispatch
- tool updates with replacement content and terminal status
- authoritative plan replacement from structured todo display
- available-command and configuration replacement

Kimi tool ids compose turn id and native tool-call id. This makes identity
session-safe but not cross-route authority. `rawInput` and `rawOutput` remain
excluded.

Kimi `0.30.0` is current stable. Its activity mapper is byte-identical to
every qualified release, but the complete harness release remains
unqualified. It may run on the `0.29.2` guarantee without widening it.

## Transport Boundary

ACP semantic messages are independent of physical transport.

- stdio uses bounded UTF-8 NDJSON and owns an installed process lifecycle
- remote HTTP/SSE and WebSocket use explicit connection and transport
  identity, affinity, bounds, and cleanup

The same session update can cross either transport. This does not merge their
transport, endpoint, authentication, retry, recovery, or support claims.
Transport identity does not create provider identity.

## Deterministic Outcomes

The corpus freezes:

- all stable selected session updates
- optional message and required tool identity
- content delta versus replacement truth
- permission and completion separation
- safe namespaced unknown updates without raw payload
- fail-closed missing session, discriminator, content, tool identity, tool
  status, plan entries, and usage shapes
- semantic equality across stdio and explicit remote framing
- every guaranteed harness segment and current unverified-newer release

Card 126 may add shared bounded decoding. It must not add provider identity,
runtime events, adapter activity profiles, or raw JSON exposure.

## Sources

- [ACP stable schema `v1.20.0`](https://github.com/agentclientprotocol/agent-client-protocol/releases/tag/schema-v1.20.0)
- [ACP Rust SDK `v1.6.0`](https://github.com/agentclientprotocol/agent-client-protocol/releases/tag/v1.6.0)
- [ACP prompt turn](https://agentclientprotocol.com/protocol/prompt-turn)
- [ACP tool calls](https://agentclientprotocol.com/protocol/tool-calls)
- [ACP agent plan](https://agentclientprotocol.com/protocol/agent-plan)
- [ACP remote transport RFD](https://agentclientprotocol.com/rfds/streamable-http-websocket-transport)
- [Rust core SDK `2.0.0`](https://docs.rs/agent-client-protocol/2.0.0)
- [Rust remote transport SDK `2.0.0`](https://docs.rs/agent-client-protocol-http/2.0.0)
- [Claude Agent ACP `0.61.0`](https://github.com/agentclientprotocol/claude-agent-acp/blob/v0.61.0/src/acp-agent.ts)
- [Claude Agent ACP `0.63.0`](https://github.com/agentclientprotocol/claude-agent-acp/blob/v0.63.0/src/acp-agent.ts)
- [Gemini CLI `0.51.0` ACP session](https://github.com/google-gemini/gemini-cli/blob/v0.51.0/packages/cli/src/acp/acpSession.ts)
- [Gemini CLI `0.53.0` release](https://github.com/google-gemini/gemini-cli/releases/tag/v0.53.0)
- [Kimi Code `0.29.2` ACP events](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/packages/acp-adapter/src/events-map.ts)
- [Kimi Code `0.30.0` release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.30.0)

## Promotion

- Clarified ACP raw fields, thought classification, plan replacement, and
  metadata boundaries in Contract 044.
- Recorded the realized stable-schema corpus boundary in system architecture.
- Completed card 125 without changing production behavior.
- Selected card 126 as the shared protocol projection continuation.
