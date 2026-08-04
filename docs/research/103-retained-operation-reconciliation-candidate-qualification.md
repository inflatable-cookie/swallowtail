# 103 Retained Operation Reconciliation Candidate Qualification

Status: promoted
Owner: Tom
Updated: 2026-08-04

## Trigger

Roadmap g03.032 left two exact candidates after ACP retained history failed the
read-only gate: Gemini CLI headless durable transcripts and Anthropic Managed
Agents persisted sessions.

## Gemini CLI Qualification

The qualified route remains exact `0.51.0..=0.52.0`. Swallowtail selects the
session id and passes `--session-id`, while Gemini records project-scoped JSONL
containing messages, tools, timestamps, usage, and session metadata.

That is useful history, but not a Contract 048 observation surface:

- the transcript has no exact run terminal record
- `--list-sessions` returns metadata, not exact terminal state or bounded
  transcript history
- exact `v0.51.0` and `v0.52.0` `sessions.ts` call `generateSummary(config)`
  before listing
- summary generation may instantiate the model client, issue a provider
  request, and append summary or scratchpad metadata to the transcript
- direct JSONL parsing would require new provider-private filesystem and schema
  authority and would still not distinguish a crash from clean completion

The frozen `retention.json` source hash already corresponds to this exact
`sessions.ts` implementation. The prior description of the post-delete list
as read-only was incorrect. Contract 038 now records that defect; g03.033 must
remove the unsupported confirmation claim before adding new recovery work.

Gemini is blocked until a qualified side-effect-free status or export operation
contains exact terminal evidence for the selected run.

## Anthropic Managed Agents Qualification

The current first subset uses beta `managed-agents-2026-04-01`. The official
surface exposes:

- exact session retrieval with running, idle, rescheduling, or terminated state
- bounded paginated persisted-event retrieval for one exact session
- ordered authoritative event identities, including submitted user messages,
  interrupts, running state, idle reasons, retries exhausted, and termination
- provider retention until explicit deletion

This passes the read-only observation gate. Exact session and event `GET`
requests require no stream attachment, message, interrupt, callback response,
resume, or deletion.

The current Swallowtail shape cannot yet survive process loss. Provisioning
creates the environment, creates the session, sends the message, and subscribes
before returning a run handle. Environment and session identity remain private
through that interval, and ordinary terminal cleanup always deletes the session
then environment.

The selected mapping therefore needs:

1. an opt-in recoverable profile
2. a route-bound run checkpoint emitted after exact resource creation and
   before submitted work can be lost
3. bounded read-only session and persisted-event reconciliation
4. a separate opaque owned-resource cleanup binding and explicit cleanup role

The reconciliation mapping is exact but conservative:

| Provider evidence | Portable state |
| --- | --- |
| running or rescheduling | `Active` |
| idle `requires_action` | `WaitingForProviderInput` |
| retries exhausted | `Failed` |
| persisted user interrupt followed by corresponding idle end-turn | `Cancelled` |
| natural idle end-turn exactly attributed to the submitted operation message | `Completed` |
| bare termination, incomplete event history, or ambiguous ordering | `Unknown` |

A callback wait is observational only. A bare terminated session cannot decide
completion versus failure. Cleanup cannot be derived from the checkpoint and
cannot implicitly interrupt active work.

## Selection

Anthropic Managed Agents is selected for g03.033. It has the strongest exact,
read-only retained-operation surface and a bounded route-specific realization.
Gemini remains visible as blocked evidence rather than fake support.

The implementation runway starts with the Gemini management truth repair,
then adds portable waiting/cleanup vocabulary, Anthropic reconciliation, and
prepared/package acceptance. No authenticated provider work is required.

## Sources

- Gemini CLI
  [`v0.51.0` sessions source](https://github.com/google-gemini/gemini-cli/blob/v0.51.0/packages/cli/src/utils/sessions.ts)
  and
  [`v0.52.0` sessions source](https://github.com/google-gemini/gemini-cli/blob/v0.52.0/packages/cli/src/utils/sessions.ts)
- Gemini CLI
  [`v0.52.0` recording service](https://github.com/google-gemini/gemini-cli/blob/v0.52.0/packages/core/src/services/chatRecordingService.ts)
  and
  [`session summary service`](https://github.com/google-gemini/gemini-cli/blob/v0.52.0/packages/core/src/services/sessionSummaryService.ts)
- Anthropic Managed Agents
  [sessions](https://platform.claude.com/docs/en/managed-agents/sessions),
  [session operations](https://platform.claude.com/docs/en/managed-agents/session-operations),
  and
  [events and streaming](https://platform.claude.com/docs/en/managed-agents/events-and-streaming)
- Anthropic API
  [retrieve a session](https://platform.claude.com/docs/en/api/beta/sessions/retrieve)
  and
  [list session events](https://platform.claude.com/docs/en/api/beta/sessions/events/list)
- Swallowtail Contracts 022, 038, and 048; Research 099
