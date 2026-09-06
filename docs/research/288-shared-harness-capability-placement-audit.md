# Research 288 — Shared Harness Capability Placement Audit

Status: complete evidence; no production claim
Owner: Tom
Date: 2026-09-07
Audited baseline: `v0.4.3` at `cbd4ddc8f9d6aa55bd947b92a55ea3a779582b79`; current planning base `8a377c1bf732372bec3cfa9db2a6a60308580b3d`
Consumer intake: Desktop `docs/roadmaps/dispatch-manifest.md` at `d4e56c5acb86bb084f5377cfaa516a03cfda76c3`, section `Swallowtail shared harness capability lane` (operator-confirmed; object is not present in this repository)
Ownership decision: Desktop `docs/specs/010-contextual-chat-and-task-queue.md` at `30a338f2` ([exact source](https://github.com/acowtancy/bovine-accelerator-desktop/blob/30a338f2/docs/specs/010-contextual-chat-and-task-queue.md)); counterpart planning: [Longhorn PR 22](https://github.com/inflatable-cookie/longhorn/pull/22) independently passed at aligned head `6ce4aa1beafad6748af238d19fb47ffaa1ad342f` ([review](https://github.com/inflatable-cookie/longhorn/pull/22#issuecomment-5563025824))

## Question

What reusable producer machinery can support contextual Desktop chat, tools,
MCP servers, skills, permissions, continuation, and safe queueing across
Claude, Codex, and Grok without flattening route behavior or moving consumer
policy into Swallowtail?

## Existing Common Substrate

- Contracts 003, 009-013, 017, 019, 023, 028, 029, 033, 037, 041, 047,
  051, 057, 058, 060-062 already separate capability, prepared-plan,
  host-service, process, credential, resource, session, callback, version,
  discovery, and projection authority.
- `SessionOptions` carries setup-time developer instructions, reasoning,
  harness mode, idioms, and native client-tool declarations.
- `TurnRequest` carries one bounded text input, deadline, attachments, and
  structured-output request. It is not a context graph or queue record.
- `CallbackExchange` already supplies exactly-once correlation, deadline,
  abandonment, and typed or namespaced responses for one active operation.
- Contract 060 proves a closed, operation-scoped HTTP/MCP bridge, private
  bearer authority, ready-before-provider ordering, exact turn binding, and
  joined teardown. It is watcher-only and cannot be widened by inference.
- Contract 061 publishes descriptive route and control truth. Projection does
  not register a server, authorize a tool, or execute work.

## Released Route Truth

| Route | Text and instructions | Session and continuation | Tools and permissions | MCP | Queue or steering |
| --- | --- | --- | --- | --- | --- |
| `claude-code` | bounded structured-run input plus qualified watcher instructions when opted in | one operation-scoped run; no reusable SDK/ACP session facade | built-in Claude Code tools plus the closed Contract 059 watcher family when opted in; no generic consumer-tool registration | released `--mcp-config` + `--strict-mcp-config` attachment of the private `swallowtail-watchers` Contract 060 bridge; omission sends an empty server object | no portable queue or mid-turn steering; watcher completion re-entry is the closed Contract 060 route seam |
| `claude-agent.sdk` | one bounded text turn; current SDK profile rejects portable developer instructions | reusable session; exact SDK resume work is separate and version-qualified | admitted built-in read/write/Bash set; route-local tool admission; permission mode has exact supported values | current released profile sends no client MCP servers | one active turn; scheduling policy bounds exist, but `schedule_harness_message` returns `Rejected` |
| `claude-agent.acp` | bounded text turns; setup-time reasoning and Plan; no embedded context or references | new, load with replay, resume without replay; exact binding | read callbacks, typed questions, provider-owned tool activity, optional one-shot permission exchange | `mcpServers: []` on new/load/resume | one active turn; upstream steering metadata and queue ownership are deliberately unmapped |
| `codex.app-server` | bounded text turns; setup-time developer instructions, reasoning, Plan | new, load, resume, catalogue, import, history, reconciliation, archive, restore, delete | dynamic native client tools and typed question exchange; no approval grant | MCP activity is observable, but no public server-registration or MCP-result dispatch API exists | serialized turns; no scheduling implementation or capability row |
| `grok-build.acp` | bounded text turns; portable instructions, reasoning, tools, and Plan are not mapped | durable provider-owned local state; exact attachment recovery only; no public load/resume/management | provider-owned tool activity; optional one-shot `allow_once` or `reject_once` permission response | `mcpServers: []` | one active turn; no queue, steering, or acknowledgement surface |

The table above remains frozen to released `v0.4.3`. Post-tag PR 255 adds the
merged, untagged evidence below; it does not change released claims.

## Card 084 Merged Evidence

Card 084 merged through PR 255 at `8a377c1b` after independent review of exact
head `f52c48c6`. It adds consumer-declared stdio MCP servers to
`claude-agent.sdk`: strict config, explicit child environment, per-call
`canUseTool` mediation, bounded status, typed required-connect failure, and an
unchanged default open. It does not admit SSE/HTTP/in-process or managed MCP,
prove a live provider turn, integrate the centralized registration/Contract
060 bridge, or create released `v0.4.3` behavior. The source is merged and
untagged evidence only.

The archived workspace row for branch `g05-card084-claude-client-mcp` at
`7cb08b1f` is preservation history only. Its branch remains intact, but it is
not a second owner or parallel lane. PR 255 is the sole Card 084 evidence line;
do not recreate it as another producer lane.

## Exact Gaps

1. No central, typed registration snapshot for consumer tools and MCP servers
   names exact schemas, versions, transport, lifecycle, execution host, or
   authority source.
2. Contract 060 opens and joins the released watcher-only bridge, but no common
   host service opens an arbitrary registered server, dispatches its tool call,
   or returns an exact result while preserving native-client, MCP, app-tool,
   and provider-owned-tool identity.
3. No portable schema-discovery namespace prevents collisions or binds a
   schema revision and digest to the admitted operation.
4. Contract 060 already owns operation lease, private loopback, bearer,
   ready-before-provider, correlation, and joined teardown for watchers. No
   generalized registered-server profile adds negotiation and reconnect. A
   second lease or loopback lifecycle would conflict with live prior art.
5. No task/session/attempt binding spans server registration, provider call,
   consumer execution, result submission, cancellation, and terminal cleanup.
6. No common Allow/Deny result distinguishes consumer policy, stale authority,
   unsupported provider response, provider rejection, cancellation, timeout,
   and transport failure.
7. No common repository-skill/reference input binds a selected immutable skill
   snapshot and required references to one session or turn. Contracts 058 and
   062 discover skills; they do not transport or enforce them.
8. No Claude/Codex/Grok route has qualified mid-turn steering. Contract 028's
   scheduling vocabulary is realized by Pi and cannot be borrowed.

## Placement Verdict

Centralize the namespaced registration snapshot and reusable bridge lease,
listener, transport, correlation, and operation lifecycle in Swallowtail. Keep
route translation in each adapter. Longhorn owns only a transport-neutral typed
host dispatch/validation library. Desktop owns domain tool names and schemas,
effects and business policy, app-context disclosure, durable admission IDs,
and packaging/startup. Desktop links the Longhorn library; slice 1 has no
standalone Longhorn daemon.

Do not build a second provider registry, operation lease, or private loopback
lifecycle. Amend Contract 060 to factor its existing operation-scoped bridge
lease and transport lifecycle into the reusable kernel. Keep `WatcherBridge`
as the compatible closed watcher profile over that kernel; registered tool
servers use a distinct profile and protocol namespace and never inherit
watcher authority. A new service composes through `HostServices`, immutable
prepared plans, `CallbackExchange`, and Contract 061 projection.

Desktop issues opaque durable task/session/attempt admission through an
authenticated host binding. One Desktop task+attempt maps to one Swallowtail
operation/turn attempt. Model arguments and provider IDs carry no identity
authority. Desktop process incarnation plus Swallowtail lease generation is
the instance authority; PID is diagnostic only. A dispatch retry creates a
fresh Desktop attempt, and mutating or indeterminate calls never replay.

## Evidence

- common turn/session surface: `crates/swallowtail-runtime/src/handles.rs`,
  `crates/swallowtail-runtime/src/session_options.rs`,
  `crates/swallowtail-runtime/src/roles/api/turn_and_serving.rs`
- callback and admission: `crates/swallowtail-runtime/src/callback/`, Contract
  012, Contract 041
- released Claude Code watcher MCP attachment and omission:
  `crates/swallowtail-adapter-claude-agent/tests/claude_code_structured_run/watcher_cases.rs`,
  `crates/swallowtail-adapter-claude-agent/src/claude_code_watcher/material.rs`,
  Contract 060
- scheduling shape and current default rejection:
  `crates/swallowtail-runtime/src/harness_rpc.rs`,
  `crates/swallowtail-adapter-claude-agent/src/sdk/driver/handle.rs`
- Claude ACP withheld capability census: Research 279
- Claude SDK route evidence and upstream gaps: Research 278 and 280
- Codex dynamic-tool and continuity fixtures:
  `crates/swallowtail-adapter-codex/tests/app_server/`; Codex MCP progress
  projection: `crates/swallowtail-adapter-codex/src/app_server_activity/projection.rs`
- Grok permission and session fixtures:
  `crates/swallowtail-adapter-grok/tests/acp/`
- route projection census:
  `docs/triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv`

No live provider, credential, consumer, server, or network operation was used.
`effigy test --plan` resolves to `cargo nextest run --workspace`; no test suite
was run for this evidence-only audit.
