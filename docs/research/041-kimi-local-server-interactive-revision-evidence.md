# 041 Kimi Local Server Interactive Revision Evidence

Status: promoted
Owner: Tom
Date: 2026-07-27

## Question

Does the full Kimi local-server interactive surface behave identically across
the qualified `0.28.1` and `0.29.0` releases, and can Swallowtail preserve its
events, approvals, questions, interruption, and replay truth?

## Method

The exact annotated release trees already pinned by Research 040 were compared
for prompt schemas and routes, approval and question schemas and routes,
WebSocket control, and the event union. No installation, login, live server,
provider request, or session effect was used.

Sources:

- [`0.28.1` prompt schema](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.28.1/packages/protocol/src/rest/prompt.ts)
- [`0.29.0` prompt schema](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.0/packages/protocol/src/rest/prompt.ts)
- [`0.29.0` prompt routes](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.0/packages/kap-server/src/routes/prompts.ts)
- [`0.29.0` WebSocket control](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.0/packages/kap-server/src/protocol/ws-control.ts)
- [`0.29.0` event schemas](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.0/packages/kap-server/src/protocol/events-zod.ts)
- [`0.29.0` approval schema](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.0/packages/kap-server/src/protocol/approval.ts)
- [`0.29.0` question schema](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.0/packages/kap-server/src/protocol/question.ts)

## Revision Delta

The two releases do not have one identical interactive behavior revision.

Unchanged across both exact releases:

- WebSocket protocol version `2`, durable `{seq, epoch}` cursors, volatile
  deltas, offset alignment, resynchronization reasons, and abort control
- approval request, response, list, and resolve shapes
- structured question request, answer, list, resolve, and dismiss shapes
- the selected turn lifecycle, output delta, reasoning delta, status, warning,
  and error events

Changed in `0.29.0`:

- prompt submission adds optional `profile`
- prompt submission adds optional `disabled_tools`
- prompt handling binds the selected profile and session tool denylist
- the event union adds `agent.created` and `agent.disposed`

The compatibility claim must therefore keep two exact maintained milestones:

| Release | Behavior revision |
| --- | --- |
| `0.28.1` | `kimi.local-server.rest-ws-v2-baseline` |
| `0.29.0` | `kimi.local-server.rest-ws-v2-profile-tools` |

Stable newer releases remain visible and unverified. They are not denied by an
upper range bound.

## Selected Interactive Boundary

The first driver needs only the common exact subset:

- session create and exact lookup for resume
- prompt submit with text content, explicit permission mode, and qualified
  reasoning fields
- `turn.started`, `assistant.delta`, `thinking.delta`, `turn.ended`, warning,
  error, and session status events
- pending approval and question lookup plus correlated response routes
- prompt abort through WebSocket control
- durable cursor and epoch tracking with explicit resynchronization failure

The exact `0.29.0` milestone also permits explicit `profile` and
`disabled_tools` prompt controls. The `0.28.1` baseline rejects those controls
before effects. Agent lifecycle events, steering, goals, swarm, terminal
attachment, filesystem watching, and raw provider event forwarding remain
outside the first selected boundary. Their presence is not silently treated
as completion.

## Callback And Authority Finding

Kimi approvals and questions are richer than one common confirmation dialog.
They include provider ids, tool display data, multi-question bundles,
multi-select answers, free text, and provider-specific response fields.

Contract 012 already permits declared provider extensions to be answered by a
consumer through the bounded callback exchange. The core access policy,
however, only encoded reject and observe-and-stop. The missing mechanism is an
explicit consumer-mediated provider-request mode:

- reject remains the default
- approval posture remains `Never` unless the caller opts in
- opt-in declares exact Kimi approval and question namespaces
- requests and responses remain bounded opaque extension payloads
- Swallowtail validates correlation and wire shape but never decides an answer
- cancellation, timeout, terminal state, or close abandons pending callbacks

This is transport authority, not product approval policy.

## Promotion

- Contract 012 records exchangeable declared provider extensions.
- Contract 013 records consumer-mediated approval as explicit opt-in with
  reject/never defaults unchanged.
- The local-server compatibility claim carries separate `0.28.1` and `0.29.0`
  behavior revisions.
- Roadmap card 064 consumes the bounded common subset and the exact
  `0.29.0`-only profile and tool-denylist milestone.
