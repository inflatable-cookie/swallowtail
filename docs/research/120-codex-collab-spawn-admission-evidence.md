# 120 Codex Collab Spawn Child-Thread Admission Evidence

Status: draft
Owner: Tom
Date: 2026-08-10
Card: g03 batch 197

## Question

How does codex app-server 0.147.0 announce sub-agent spawns on a
multi-agent/collab turn, why did the g03/007-008 admission path not cover the
live sequence, and what is the implementation-ready delta?

## Live Failure Record

2026-08-10 16:24 UTC. A Nucleus operation drove a live collab turn through the
swallowtail codex app-server adapter (session originator `swallowtail`,
`codex-cli 0.147.0`). The user asked the root agent to spawn sub-agents; the
turn failed in full with:

```
swallowtail.codex.app_server.lifecycle_owner_mismatch
Codex app-server lifecycle belongs to an unknown operation thread
```

Rollout evidence (session files under `~/.codex/sessions/2026/08/10/`):

| time (UTC) | event | source |
| --- | --- | --- |
| 16:23:25 | root thread `019fec7c-6cd7` session starts (turn `019fec7c-6d42`) | `rollout-2026-08-10T17-23-25-019fec7c-6cd7-78d2-90bb-e415ea96ae00.jsonl` |
| 16:24:26 | turn `019fec7d-5db2` starts: user asks to spawn sub-agents | same file |
| 16:24:30.049 | root model emits `spawn_agent` function call (`ui_ping_one`) | same file, `response_item` `function_call` |
| 16:24:30.120 | `sub_agent_activity` event: `agent_thread_id=019fec7d-6a6a-7c33-b6ab-de5a6d102519`, `agent_path=/root/ui_ping_one`, `kind=started` | same file |
| 16:24:30.123 | child thread `019fec7d-6a6a` `task_started` (turn `019fec7d-6aa8`) | `rollout-2026-08-10T17-24-30-019fec7d-6a6a-7c33-b6ab-de5a6d102519.jsonl` |

Both files end at the spawn moment; the root turn never completed and only one
of the three planned spawns happened. The adapter failure (`lifecycle_owner_mismatch`
from `verify_child_lifecycle_owner`,
`crates/swallowtail-adapter-codex/src/turn_state/notifications.rs:180-205`)
propagates through `rpc/pump.rs:132` and fails the whole operation.

The child did emit one activity before the failure: the parent-envelope
`subAgentActivity` item (`kind=started`, carrying `agentThreadId`). The adapter
projected it as a child-attributed observation without admission, then failed
on the child's `turn/started`.

## Installed Protocol Surface (0.147.0)

Primary evidence generated from the installed CLI:

```sh
codex app-server generate-json-schema --out /tmp/codex-schema-0.147.0
```

- Notification union includes `Turn/startedNotification` (params: `threadId`,
  `turn`) and `Thread/startedNotification` (params: `thread`).
- `ThreadItem` discriminators include `collabAgentToolCall` and
  `subAgentActivity` (no other spawn-carrying item type).
- `collabAgentToolCall` requires `agentsStates, id, receiverThreadIds,
  senderThreadId, status, tool, type`. `tool` enum: `spawnAgent, sendInput,
  resumeAgent, wait, closeAgent`. `CollabAgentStatus`: `pendingInit, running,
  interrupted, completed, errored, shutdown, notFound`.
- `subAgentActivity` requires `agentPath, agentThreadId, id, kind, type`;
  `kind` enum: `started, interacted, interrupted`.

Source citations (openai/codex @ `rust-v0.147.0`):

- `codex-rs/app-server-protocol/src/protocol/v2/item.rs` — item conversion.
- `codex-rs/core/src/tools/handlers/multi_agents/spawn.rs` (v1) — emits
  collabAgentToolCall item/started with **empty** `receiver_thread_ids`, then
  after the spawn returns emits item/completed with `receiver_thread_ids` and
  `agents_states` populated.
- `codex-rs/core/src/tools/handlers/multi_agents_v2/spawn.rs` (v2, the live
  flow) — child thread creation and initial-input submission happen inside
  `spawn_agent_with_communication`; `emit_sub_agent_activity` (item/started +
  item/completed of `subAgentActivity`) runs after the spawn returns.
- `codex-rs/core/src/agent/control/spawn.rs` — `notify_thread_created` fires
  inside the spawn; the child's turn is submitted via `send_op`.
- `codex-rs/app-server/src/lib.rs:1120-1146` — on any thread creation the
  app-server attaches a conversation listener for the new thread to every
  initialized connection, so the root connection receives the child's
  `turn/started` and items without an explicit subscribe.

## Adapter Admission Path Today

`admit_spawned_children` (`turn_state/notifications.rs:312-339`) admits a
thread id only from an activity whose projection has
`subagent_control() == Some(Spawn)` with phase `Completed` and status
`Completed`. The only such projection is the parent envelope's
`collabAgentToolCall` (`tool: spawnAgent`) **item/completed**
(`app_server_activity/subagent.rs` `collaboration()`,
`app_server_activity/item.rs`).

- `subAgentActivity` items project `control: None`
  (`app_server_activity/subagent.rs` `activity()`) — they carry the exact
  child id (`agentThreadId`) but never admit.
- Child lifecycle (`turn/started`, `turn/completed`) requires the id to be in
  `admitted_child_threads` (`verify_child_lifecycle_owner`) and hard-fails the
  turn otherwise.
- The corpus's baked-in ordering assumption is explicit in the frozen fixture:
  `child-turn-lifecycle` expects
  `["root_spawn_completed", "child_turn_started", "child_activity", "child_turn_completed"]`
  (`crates/swallowtail-adapter-codex/tests/fixtures/activity/app-server.jsonl`).

## The Gap

**Ordering race, not a missing item type.** The app-server provides no
ordering guarantee between the child thread's `turn/started` and the parent
thread's spawn `collabAgentToolCall` item/completed. Both are asynchronous
emissions: the child's turn starts inside the spawn call (thread created,
initial op submitted), while the collab item/completed is emitted only after
the spawn handler returns and the tool result is recorded. In the live 0.147.0
run the child's `turn/started` won the race (~3 ms after the sub-agent
activity; the collab item/completed never reached the adapter before the
failure). The 0.146.0 frozen capture happened to show the opposite order — the
adapter's assumption was never guaranteed.

The child identity is unavailable before the spawn completes: the v1 collab
item/started carries empty `receiver_thread_ids` (source), so admission cannot
simply move to the spawn item/started.

The earliest notification that carries the exact child id is the
parent-envelope `subAgentActivity` (`kind=started`), which the adapter already
observes (child-attributed, control `None`). Live evidence places it 3 ms
before the child's `turn/started`; source places it after the spawn returns
but before the collab item/completed is recorded. It narrows the race window
but does not, from source alone, guarantee victory: the child's turn start is
itself asynchronous.

## Recoverable State And Reconciliation

The failed operation is honestly terminal (`provider_failed`); the consumer
sees the failure and can retry. The child thread (`019fec7d-6a6a`) persists as
a real codex thread with session files, so its partial turn is recoverable as
a **new** operation through the thread catalogue/import path (research 093,
g03.037 card 093), not by re-admitting the failed turn.

g03/026-038 cover operation-level recovery — cross-process reconciliation,
detached-operation cleanup, checkpoint/restoration of active turns. None of
them re-admit an in-turn notification gap or resurrect a failed operation.
No coverage.

## Proposed Card Split

Contract delta (Contract 045 amendment — recommend, do not write):

- The operation-local admission evidence set widens from "completed spawn
  collaboration item" to include the provider's spawn-confirmation
  observation: the parent-envelope `subAgentActivity` (`kind=started`) with
  exact `agentThreadId`.
- The adapter may not assume ordering between child-lifecycle envelopes and
  admission evidence; child lifecycle for an id already established by
  spawn-confirmation topology evidence is admitted and observed without
  failing the operation. Never-observed ids still fail closed.

Implementation card:

- Admit the child when the parent envelope projects `subAgentActivity`
  (`kind=started`) — same bounded `MAX_ADMITTED_CHILD_THREADS` set, cleared at
  operation terminal, same fail-closed posture for unknown ids.
- Keep the existing `collabAgentToolCall` spawnAgent item/completed admission
  (covers v1 flows and non-spawn collab actions).
- Evidence test (this card) reproduces the live ordering and currently fails
  closed; the implementation card flips it to assert admission.
- Verify with a second live collab capture that the subAgentActivity-started
  admission precedes the child lifecycle in practice; if a capture shows the
  lifecycle racing ahead even of that, add bounded child-lifecycle deferral
  until admission evidence or a bounded window elapses.

Contract impact: Contract 045 "Admission" paragraph — the qualified topology
observation set and the ordering tolerance are both stated there today; the
amendment is additive, keeps observation-only posture, and does not touch
control, callbacks, terminal, or session authority.

## Stop Conditions

Not triggered.

- Installed schema generated from the installed CLI (0.147.0) — available.
- Live rollout logs found under `~/.codex/sessions/2026/08/10/` — available.
- The gap extends g03/007-008's ownership model (admission source and
  ordering), it does not contradict it.
- No protocol capability is missing: the spawn-confirmation observation exists
  on the wire today.

## Sources

- Installed schema: `codex app-server generate-json-schema` (codex-cli
  0.147.0, `/Users/tom/.local/bin/codex`) — `ServerNotification.json`,
  `codex_app_server_protocol.schemas.json`.
- Live rollouts: `~/.codex/sessions/2026/08/10/rollout-2026-08-10T17-23-25-*`
  and `...17-24-30-*`.
- Adapter: `crates/swallowtail-adapter-codex/src/turn_state/notifications.rs`,
  `turn_state.rs`, `app_server_activity/{item,subagent,projection}.rs`,
  `crates/swallowtail-adapter-codex/tests/fixtures/activity/app-server.jsonl`,
  `rpc/pump.rs`.
- Codex source @ `rust-v0.147.0` (github.com/openai/codex):
  `codex-rs/core/src/tools/handlers/multi_agents/spawn.rs`,
  `codex-rs/core/src/tools/handlers/multi_agents_v2/spawn.rs`,
  `codex-rs/core/src/tools/handlers/multi_agents_v2.rs`,
  `codex-rs/core/src/agent/control/spawn.rs`,
  `codex-rs/core/src/agent/control.rs`,
  `codex-rs/app-server/src/lib.rs`,
  `codex-rs/app-server/src/request_processors/thread_lifecycle.rs`,
  `codex-rs/app-server-protocol/src/protocol/v2/item.rs`.
- Governing cards: g03/007, g03/008; Contract 045; research 037, 064, 072,
  093.
