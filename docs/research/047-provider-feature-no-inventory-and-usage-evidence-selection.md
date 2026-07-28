# 047 Provider Feature `No` Inventory And Usage-Evidence Selection

Status: promoted
Owner: Tom
Date: 2026-07-28

Historical snapshot: later route additions and completed usage work changed the
canonical counts. Research 049 owns the current generation-control subset; the
CSV and route-matrix gate own current whole-matrix counts.

## Question

Which remaining feature-matrix `No` cells are matrix errors, ready
implementation work, upstream absence, operation-shape mismatch, or unresolved
evidence work? Which feature family should run first?

## Method

Evidence was accessed 2026-07-28.

- parsed the canonical CSV with Python's CSV parser
- checked all 21 solution rows and every column from
  `unverified_newer_allowed` through `planned_connection_rollover`
- compared claims with production descriptors, prepared facades, examples,
  fixtures, contracts, and promoted research
- inspected exact maintained sources for the five remaining usage-evidence
  `No` cells
- added a machine gate for exact counts, unique cells, classifications, and
  serving-only non-applicability

No executable, credential, account, provider request, paid operation,
container, or model server was used.

## Integrity Correction

The initial CSV-aware inventory contained 458 `No` cells.

`llama-cpp.owned` is a model-serving lifecycle. It starts an approved
`llama-server`, observes readiness, returns an endpoint, then stops and joins
the process. The separate attached route owns inference.

Twenty-six inference, session, callback, retention, and remote-resource
columns were therefore false `No` values. They are now `Not applicable`.
`unverified_newer_allowed = No` remains: the exact opaque serving revision has
no forward-admission claim.

The corrected matrix has:

- 432 `No` cells
- 29 `Not applicable` cells in the audited span
- no `No` in `streaming_events`
- no `No` in `cancellation_or_interruption`

The last two columns are now complete for every applicable solution. No
runtime capability changed.

## Exact Corrected Inventory

| Feature | `No` count |
| --- | ---: |
| `unverified_newer_allowed` | 12 |
| `structured_run` | 2 |
| `interactive_session` | 10 |
| `realtime_media_session` | 18 |
| `usage_evidence` | 5 |
| `billed_cost_evidence` | 19 |
| `output_token_limit` | 13 |
| `reasoning_selection` | 14 |
| `structured_output` | 19 |
| `attachments` | 19 |
| `consumer_tool_exchange` | 17 |
| `approval_question_exchange` | 15 |
| `load_session` | 19 |
| `resume_session` | 17 |
| `working_resource` | 12 |
| `bounded_workspace_text_write` | 18 |
| `external_search` | 19 |
| `retained_background_execution` | 19 |
| `stream_reattachment` | 18 |
| `provider_managed_recovery` | 19 |
| `provider_session_archive` | 18 |
| `provider_session_restore` | 18 |
| `provider_session_delete` | 17 |
| `native_session_close` | 19 |
| `owned_remote_resource_cleanup` | 18 |
| `owned_runtime_lifecycle` | 19 |
| `planned_connection_rollover` | 19 |
| **Total** | **432** |

The canonical CSV is the exact cell inventory. The route-matrix gate visits
every current `No` once using `(provider, solution, feature)` identity and
rejects count, duplicate, classification, or unlisted-column drift.

## Classification

| Classification | Cells | Meaning |
| --- | ---: | --- |
| ready under existing contracts | 3 | Exact selected surface reports usage; Swallowtail does not project it yet |
| upstream unsupported | 2 | Exact selected Kimi surface supplies no usage record |
| operation-shape not applicable | 2 | Realtime routes do not expose the separately defined structured-run shape |
| missing shared contract or currentness evidence | 425 | Retain `No`; audit in its feature-family tranche |
| realized matrix error | 0 | The 26 serving-only errors were corrected before the final inventory |
| separate route required | 0 | No new route is recommended by this first pass |

The 425-cell classification is a triage result, not an assertion that the
upstream systems lack those features. Each later feature-family audit must
replace it with exact route evidence. This prevents a generic inventory pass
from borrowing capabilities across solutions.

## Usage-Evidence Findings

### Claude Agent ACP — ready

The selected adapter window is `0.53.0..=0.61.0`, excluding unpublished
`0.58.0`.

Every published qualified point returns prompt-response usage with input,
output, cache-read, cache-write, and total tokens. ACP `usage_update` is a
separate context-window occupancy record and may include cost. Swallowtail
currently accepts that update as progress and discards the prompt response's
typed usage.

The implementation should project prompt-response usage once as cumulative
operation usage. It must not relabel `usage_update.used` as input tokens.

Evidence:

- [Claude Agent ACP 0.53.0 source](https://github.com/agentclientprotocol/claude-agent-acp/blob/v0.53.0/src/acp-agent.ts)
- [Claude Agent ACP 0.61.0 source](https://github.com/agentclientprotocol/claude-agent-acp/blob/v0.61.0/src/acp-agent.ts)

### Pi RPC — ready

Exact qualified Pi `0.80.10` emits complete assistant messages in
`message_end`. Each assistant message includes input, output, cache-read, and
cache-write usage. A tool loop may emit several disjoint assistant-message
records.

Pi also exposes `get_session_stats`, but that command is cumulative across the
session and includes tool and compaction work. The first implementation should
use the already-streamed message records, sum disjoint assistant-message usage
inside one operation, and emit one terminal cumulative observation. It does
not need another RPC command or session persistence.

The same message and stats shapes remain documented at maintained `0.81.1`
and `0.82.1`. Those versions remain visible unverified-newer points until a
separate range qualification promotes them.

Evidence:

- [Pi 0.80.10 RPC events and message types](https://github.com/earendil-works/pi/blob/v0.80.10/packages/coding-agent/docs/rpc.md)
- [Pi 0.82.1 RPC documentation](https://github.com/earendil-works/pi/blob/v0.82.1/packages/coding-agent/docs/rpc.md)

### OpenCode HTTP — ready

OpenCode `message.part.updated` carries `step-finish` parts. Each part has
input, output, reasoning, cache-read, cache-write, and cost fields.

The selected guaranteed window `1.14.48..=1.18.4` has the same required token
shape at both boundaries and checked milestones. A multi-step agent turn may
contain several disjoint step-finish records. Swallowtail should sum their
token fields and emit one terminal cumulative usage observation. Cost remains
outside this tranche.

Evidence:

- [OpenCode 1.14.48 OpenAPI](https://github.com/anomalyco/opencode/blob/v1.14.48/packages/sdk/openapi.json)
- [OpenCode 1.18.4 OpenAPI](https://github.com/anomalyco/opencode/blob/v1.18.4/packages/sdk/openapi.json)

### Kimi Code — retain `No`

The selected Kimi Code surfaces are exact `0.29.2`:

- installed ACP plus headless command
- attached local server over HTTP and WebSocket

The qualified headless renderer, ACP adapter, and local WebSocket broadcaster
do not expose a token-usage record on those selected surfaces. Other Moonshot
SDKs and web products cannot supply capability to these routes.

Both Kimi solution cells remain `No`. This is selected-surface absence, not a
claim that Moonshot never measures usage internally.

Evidence:

- [Kimi Code 0.29.2 headless renderer](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/apps/kimi-code/src/cli/prompt-render.ts)
- [Kimi Code 0.29.2 ACP adapter](https://github.com/MoonshotAI/kimi-code/tree/%40moonshot-ai%2Fkimi-code%400.29.2/packages/acp-adapter)
- [Kimi Code 0.29.2 local WebSocket broadcaster](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/packages/kap-server/src/transport/ws/v1/sessionEventBroadcaster.ts)

## First Family Recommendation

Run usage evidence first.

It converts three of five current `No` cells under existing provider-neutral
types, records two honest upstream absences, and exercises three materially
different harness transports:

- ACP prompt response
- long-lived NDJSON RPC message events
- HTTP/SSE agent-step events

The common rule needs one narrow clarification: a harness adapter may combine
disjoint provider usage components into one operation aggregate, but must
replace cumulative snapshots rather than sum them again. Overflow, malformed
numbers, missing required fields, or ambiguous cumulative semantics fail
closed.

Usage is not billed cost, quota, rate, context occupancy, or a generation
limit. No cost cell changes in this tranche.

## Remaining Family Runway

After usage:

1. generation controls — output limit, reasoning, structured output
2. input and callback surfaces — attachments, tools, approvals, search
3. session continuity — load, resume, native close
4. provider retention — archive, restore, delete, remote cleanup
5. retained execution — background work, reattachment, recovery, rollover
6. runtime and workspace posture — working resource, bounded writes, owned
   lifecycle
7. realtime media — audit only routes whose operation shape can naturally
   support it

Each family gets its own currentness and contract gate. The 425 unresolved
cells remain visible until those passes complete.

## Promotion

- Corrected the serving-only solution row.
- Added exact `No` inventory and classification gates.
- Selected usage evidence for card 081.
- Tightened cards 081-083 around Claude Agent ACP, Pi RPC, and OpenCode.
