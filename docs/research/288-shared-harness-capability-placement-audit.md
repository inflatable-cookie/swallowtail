# Research 288 — Shared Harness Capability Placement Audit

Status: complete evidence; no production claim
Owner: Tom
Date: 2026-09-06
Audited baseline: `v0.4.3` at `cbd4ddc8f9d6aa55bd947b92a55ea3a779582b79`; current planning base `f2d93fa237373640170a3872e9c08dda23f73e7f`
Consumer intake: Desktop `docs/roadmaps/dispatch-manifest.md` at `d4e56c5acb86bb084f5377cfaa516a03cfda76c3`, section `Swallowtail shared harness capability lane` (operator-confirmed; object is not present in this repository)

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
| `claude-agent.sdk` | one bounded text turn; current SDK profile rejects portable developer instructions | reusable session; exact SDK resume work is separate and version-qualified | admitted built-in read/write/Bash set; route-local tool admission; permission mode has exact supported values | current released profile sends no client MCP servers | one active turn; scheduling policy bounds exist, but `schedule_harness_message` returns `Rejected` |
| `claude-agent.acp` | bounded text turns; setup-time reasoning and Plan; no embedded context or references | new, load with replay, resume without replay; exact binding | read callbacks, typed questions, provider-owned tool activity, optional one-shot permission exchange | `mcpServers: []` on new/load/resume | one active turn; upstream steering metadata and queue ownership are deliberately unmapped |
| `codex.app-server` | bounded text turns; setup-time developer instructions, reasoning, Plan | new, load, resume, catalogue, import, history, reconciliation, archive, restore, delete | dynamic native client tools and typed question exchange; no approval grant | MCP activity is observable, but no public server-registration or MCP-result dispatch API exists | serialized turns; no scheduling implementation or capability row |
| `grok-build.acp` | bounded text turns; portable instructions, reasoning, tools, and Plan are not mapped | durable provider-owned local state; exact attachment recovery only; no public load/resume/management | provider-owned tool activity; optional one-shot `allow_once` or `reject_once` permission response | `mcpServers: []` | one active turn; no queue, steering, or acknowledgement surface |

The source tree after `v0.4.3` changes planning only for this scope. No current
code extends these claims.

## Exact Gaps

1. No central, typed registration snapshot for consumer tools and MCP servers
   names exact schemas, versions, transport, lifecycle, execution host, or
   authority source.
2. No common host service opens a registered server, dispatches its tool call,
   or returns an exact result while preserving native-client, MCP, app-tool,
   and provider-owned-tool identity.
3. No portable schema-discovery namespace prevents collisions or binds a
   schema revision and digest to the admitted operation.
4. No common server lease owns ready, reconnect, version negotiation, stale
   request rejection, and joined teardown. Contract 060 supplies the lifecycle
   precedent but only for watchers.
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

Centralize reusable registration, lease, transport, correlation, admission,
and lifecycle machinery in Swallowtail. Keep route translation in each
adapter. Keep server implementation and business semantics in Longhorn. Keep
skill discovery selection, context assembly, task policy, queue UX, and
product persistence in Desktop.

Do not build a second provider registry. A new service composes through
`HostServices`, immutable prepared plans, `CallbackExchange`, and Contract 061
projection. It may reuse Contract 060's private transport patterns but cannot
reuse its watcher authority or closed protocol namespace.

## Evidence

- common turn/session surface: `crates/swallowtail-runtime/src/handles.rs`,
  `crates/swallowtail-runtime/src/session_options.rs`,
  `crates/swallowtail-runtime/src/roles/api/turn_and_serving.rs`
- callback and admission: `crates/swallowtail-runtime/src/callback/`, Contract
  012, Contract 041
- scheduling shape and current default rejection:
  `crates/swallowtail-runtime/src/harness_rpc.rs`,
  `crates/swallowtail-adapter-claude-agent/src/sdk/driver/handle.rs`
- Claude ACP withheld capability census: Research 279
- Claude SDK route evidence and upstream gaps: Research 278 and 280
- Codex dynamic-tool and continuity fixtures:
  `crates/swallowtail-adapter-codex/tests/app_server/`
- Grok permission and session fixtures:
  `crates/swallowtail-adapter-grok/tests/acp/`
- route projection census:
  `docs/triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv`

No live provider, credential, consumer, server, or network operation was used.
`effigy test --plan` resolves to `cargo nextest run --workspace`; no test suite
was run for this evidence-only audit.
