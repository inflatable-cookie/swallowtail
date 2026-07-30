# 072 Subagent Topology, Observation, And Control Evidence

Status: promoted
Owner: Tom
Date: 2026-07-30

## Question

Which selected Swallowtail routes expose enough structured child-agent truth
for a consumer to build a browseable side-job view, and which expose direct
operator control?

## Method

Evidence was checked on 2026-07-30.

- inspected every production harness activity decoder and frozen corpus
- checked exact Codex `0.146.0` app-server and exec item types
- checked exact Kimi local-server WebSocket v2 events through `0.31.0`
- checked selected ACP, headless, RPC, HTTP/SSE, and managed-agent routes for
  stable child identity, parentage, lifecycle, attributed activity, and
  targetable control
- reused the qualified activity and route profiles; no provider was invoked

No credential, live model request, paid operation, or consumer repository was
used.

## Positive Route Evidence

| Route | Structured child truth |
| --- | --- |
| Codex app-server `0.85.0+` | collaborative tool call, sender and receiver thread ids, prompt, child states, optional model and reasoning effort |
| Codex app-server `0.140.0+` | child activity item with child thread id and path |
| Codex exec `0.92.0+` | collaborative tool call with the same operation-local child identity and configuration, but no durable inspection channel |
| Kimi local-server `0.28.1+` | spawned, started, suspended, completed, and failed events with stable child id, name, originating tool id, and background posture |

Codex collaborative actions are `spawn`, `send input`, `resume`, `wait`, and
`close`. They are provider-owned actions initiated by the main agent. Their
presence on the wire does not grant the Swallowtail consumer authority to call
them.

Codex app-server `0.146.0` also exposes experimental descendant
`thread/list` filters and ordinary `thread/read`. Those are provider-session
inspection methods. They are not part of the current operation-local turn
handle and do not make a raw child thread id sufficient authority.

## Negative Route Evidence

The selected Claude Agent ACP, Gemini ACP, Kimi ACP, Qwen headless, Claude
headless, Gemini headless, Kimi headless, Pi RPC, OpenCode HTTP/SSE, and
Anthropic Managed Agents routes do not expose a qualified combination of
stable child id, parent relation, and child lifecycle.

Several expose a generic provider tool whose name suggests delegation. Qwen
also advertises agent, sub-session, team, task, and message commands. Those
surfaces do not expose the inner child calls in the selected structured event
stream. Tool names are not topology.

ACP v1 has no portable subagent graph or targetable child-control method.
Claude bridge Task-family tool rendering therefore remains ordinary
provider-tool activity unless a future exact adapter surface supplies child
identity.

## Control Finding

No selected route currently qualifies direct operator control of an active
child.

- whole-turn interruption is not child interruption
- sending a message to the main turn is not child steering
- observing the main agent call `send input` or `close` is not consumer
  control
- starting an unrelated provider turn on a child thread is not equivalent to
  the harness collaboration primitive
- a raw provider child id is correlation evidence, not ambient authority

A future direct control surface needs a bound handle, exact supported actions,
target and operation validation, acknowledgement or uncertainty truth,
deadline and cancellation behavior, and joined cleanup. Unsupported controls
must remain absent rather than degrade to main-turn operations.

## Selection

Add first-class child-work detail to observable activity:

- explicit primary or child actor
- bounded child snapshots
- operation-local child id
- operation, child, or unknown parent
- pending, running, waiting, completed, failed, interrupted, shutdown, or
  unknown status
- optional label, task description, model, reasoning mode, background posture,
  and originating provider activity
- separately typed provider-owned collaborative action

Consumers can maintain a browseable graph from the ordered stream. Swallowtail
does not persist the graph.

Do not expose a generic operator-control handle in this tranche. The contract
defines the qualification boundary so a later route can add one without
misrepresenting existing interruption or messaging.

## Sources

- [Codex `0.146.0` app-server item types](https://github.com/openai/codex/blob/rust-v0.146.0/codex-rs/app-server-protocol/src/protocol/v2/item.rs)
- [Codex `0.146.0` thread protocol](https://github.com/openai/codex/blob/rust-v0.146.0/codex-rs/app-server-protocol/src/protocol/v2/thread.rs)
- [Codex app-server documentation](https://github.com/openai/codex/blob/main/codex-rs/app-server/README.md)
- [Kimi Code repository](https://github.com/MoonshotAI/kimi-code)
- [Agent Client Protocol repository](https://github.com/agentclientprotocol/agent-client-protocol)

## Promotion

- added Contract 045
- extended the activity profile with exact child-observation and visible
  collaboration-action constraints
- realized Codex app-server, Codex exec, and Kimi local-server child snapshots
- retained direct operator control as unsupported
