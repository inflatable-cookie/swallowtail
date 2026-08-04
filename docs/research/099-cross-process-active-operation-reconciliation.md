# 099 Cross-Process Active Operation Reconciliation

Status: promoted
Owner: Tom
Updated: 2026-08-04

## Trigger

T3 Code leaves locally active turns marked as working after application restart.
Closing the application can also cancel work even when the provider can retain
it. Swallowtail already imports provider sessions and persists exact resume
bindings, but neither operation answers what happened to a consumer turn that
lost its runtime handle.

## Finding

This is adjacent to provider-session import, not an import extension.

- import selects and attaches a provider session
- reconciliation observes an already-bound session after attachment loss
- reattachment resumes one live event stream
- cancellation changes provider state

Combining them would turn a read-only restart check into implicit attachment or
control authority.

OpenCode exposes the first complete observation surface: exact session lookup,
session status, and retained paged messages. It does not expose a prompt or turn
identifier from `prompt_async`, so its attribution is session-scoped. An idle
session therefore proves only `InactiveUnresolved`, never completion.

Codex app-server `thread/read` carries exact turn records. The promoted mapping
now admits exact turn status when present and fails closed when the requested
turn is absent. Kimi local server carries exact turn ids plus resumable event
cursors. OpenAI background Responses now carries a strict route-bound
provider-run checkpoint and one exact read-only retrieve path. Anthropic
Managed Agents also exposes exact session retrieval plus persisted, ordered
event history. Those surfaces pass the observation gate, but the current
driver hides the session and environment references until after dispatch and
deletes both on every ordinary close. A recoverable profile therefore needs an
early durable checkpoint plus a separate exact-resource cleanup binding.
Reconciliation itself remains read-only.

Gemini CLI's durable transcript does not pass the gate. Exact qualified
`0.51.0` and `0.52.0` source runs summary generation before
`--list-sessions` returns. That path may issue a Gemini request and append
summary or scratchpad metadata to the transcript. The list result also omits
an exact terminal record. Directly parsing provider-private JSONL would add
filesystem and private-schema authority without solving terminal attribution.

ACP child-process routes can often reload retained history. Qualification of
Claude Agent ACP and Kimi ACP found no read-only history operation, however.
Stable ACP defines `session/load` as restoring resumable context, reconnecting
specified MCP servers, replaying history, and returning a ready session.
Swallowtail's two drivers follow that shape and return a live session handle.

The Claude Agent continuity corpus proves the load shape through `0.61.0`; the
installed exact `0.63.0` source still calls `getOrCreateSession` before
`getSessionMessages`. The separately qualified `0.62.0..=0.64.0` artifacts do
not expose another history operation. The underlying history read is not a
separate ACP method. Kimi ACP `0.28.1..=0.31.1` likewise exposes session
metadata through `session/list`, but history only through `session/load`;
`session/resume` is the same continuation without replay. Closing either
loaded handle immediately does not retroactively make the load read-only.

The original child and live turn also do not survive as an observable
attachment. Retained replay cannot prove a surviving turn or exact terminal
state. Both ACP candidates therefore fail Contract 048 without a new qualified
read-only history surface.

## Route Classification

| Class | Routes | Promotion gate |
| --- | --- | --- |
| supported exact-turn observation | `codex.app-server` | realized in g03.027; missing status remains exact-attribution `Unknown` |
| supported session-scoped observation | `opencode.http` | realized in g03.027 |
| supported exact-turn observation | `kimi-code.local-server` | realized in g03.029 with a persisted operation checkpoint and finite cursor replay |
| supported exact-run observation | `openai.background` | realized in g03.030 with a persisted provider-run checkpoint and one exact retrieve request |
| selected exact-run observation candidate | `anthropic.managed-agent` | emit a strict route-bound checkpoint before dispatch can be lost; retrieve the exact session and bounded persisted events; keep recovered cleanup behind a separate exact-resource binding |
| blocked: load is continuation | `claude-agent.acp`; `kimi-code.acp` | expose a separately qualified bounded history read which creates no resumable context, session handle, MCP connection, callback, provider request, or control authority |
| blocked: list mutates retained state and omits terminal truth | `gemini-cli.headless` | expose a qualified side-effect-free status or transcript-export operation with an exact terminal record |
| blocked pending durable provider identity | `cursor-agent.acp`; `gemini-cli.acp`; `grok-build.acp`; `pi.rpc`; `qwen.headless`; `antigravity.headless`; `alibaba.conversations` | expose and persist an exact route-qualified operation/session reference plus bounded status or history lookup |
| not applicable to current lifecycle | catalogue-only routes; stateless direct runs; connection-only continuation and realtime routes; attached/owned local model inference | provider work cannot outlive the selected operation attachment or no retained operation identity exists |

The classification is deliberately separate from the main solution feature
CSV. It records routes that can move, their current evidence strength, and the
next gate without adding permanent `No` columns to the conversion scoreboard.

## Promoted Decisions

- Contract 048 owns the read-only boundary.
- The runtime carries distinct attribution and state; Codex is the first exact
  turn mapping and OpenCode is the first session-scoped mapping.
- Session-scoped evidence cannot claim a terminal result.
- Reconciliation returns a bounded replacement snapshot with explicit
  completeness, not an append-only transcript delta.
- Observation never grants cancellation, callback, provider-request, prompt,
  resume, import, or child-control authority.
- ACP load/replay cannot implement observation: loading resumable context and
  then closing it remains a continuation operation.
- Gemini `--list-sessions` cannot implement observation while it may run
  summary inference and mutate the transcript; transcript presence is not
  terminal evidence.
- Anthropic Managed Agents is the selected next mapping. Exact session and
  event retrieval satisfy observation; early checkpoint persistence and
  recovered-resource cleanup remain unrealized.
- A reconciliation checkpoint grants no deletion authority. Cleanup of an
  exact driver-owned recovered session and environment requires a separate
  opaque binding and explicit operation.
- Controlled shutdown preservation remains separate work. This tranche does
  not change ordinary handle close semantics.

## Sources

- T3 Code restart symptoms: `pingdotgg/t3code` issues 2173 and 2886
- OpenCode server API: `/session/status`, `/session/:id`, and
  `/session/:id/message`
- Swallowtail Contracts 017, 021, 022, 042, and 046
- qualified Codex thread catalogue and Kimi local-server corpora in this repo
- stable ACP v1 schema `LoadSessionRequest` and `ResumeSessionRequest`
- Claude Agent ACP `session-continuity-corpus.json` plus installed `0.63.0`
  `loadSession`, `getOrCreateSession`, and `replaySessionHistory` source
- Kimi ACP `0.28.1` protocol corpus and `0.28.1..=0.31.1` session-list range
- Gemini CLI `v0.51.0` and `v0.52.0` `sessions.ts`, recording service, and
  session-summary source
- Anthropic Managed Agents `managed-agents-2026-04-01` session retrieval and
  persisted-events API documentation
