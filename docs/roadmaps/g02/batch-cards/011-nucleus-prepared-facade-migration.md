# 011 Nucleus Prepared Facade Migration

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../004-nucleus-prepared-facade-adoption.md`

## Objective

Replace Nucleus's manual Codex preparation with the completed facade across
catalogue, Agent Chat, bounded task execution, and smoke paths.

## Governing Refs

- Swallowtail Contract 037 and completed roadmap g02.003
- Swallowtail repository authority map
- Nucleus Contracts 030-031
- Nucleus `AGENTS.md` and active roadmap at execution time

## Scope

1. Enter Nucleus under explicit consumer-repository authority.
2. Map existing stable instance, target, environment, access evidence, model,
   resource, reasoning, and tool inputs to prepared setup.
3. Migrate model catalogue and read-only Agent Chat.
4. Migrate bounded task execution and confirmed smoke paths.
5. Preserve existing Nucleus traits, DTOs, product tools, callback execution,
   receipts, persistence, and UI projections.
6. Keep the prior path available only through source rollback, not a runtime
   shim.

## Acceptance Criteria

- [x] every live Nucleus Codex path uses the prepared facade
- [x] exact version, configuration, and access policy are bound once
- [x] read-only and bounded-workspace paths remain distinct
- [x] tools remain declared and executed by Nucleus
- [x] no product type enters Swallowtail
- [x] deterministic parity passes before deletion

## Validation

- focused Nucleus adapter and server tests
- deterministic catalogue, chat, task, callback, and smoke fixtures
- `effigy check:rust`
- Nucleus normal focused QA selectors
- `git diff --check`

## Evidence Required

- mapping of old helpers to prepared inputs
- deterministic parity results
- exact changed-file and ownership audit
- card 012 readiness assessment

## Exact Migration Inputs

| Current Nucleus surface | Prepared replacement |
| --- | --- |
| `swallowtail_codex::host::local_host` and `services` | retain Nucleus path and environment authority; approve the selected executable with `approve_installed_executable`, retain its opaque target, and compose one exact host service set |
| `swallowtail_codex::discovery::installed_codex_version` | remove after every path uses `prepare_codex(CodexPreparedDriver::AppServer, ...)` |
| `swallowtail_codex::preflight::catalog_plan` | `CodexPreparedIntegration::prepare_catalogue` with the existing catalogue request id and deadline |
| `swallowtail_codex::preflight::session_plan` plus manual `OpenSessionRequest` policy copy | `prepare_read_only_session(CodexSessionProfileInput)` |
| `swallowtail_codex::preflight::task_session_plan` plus bounded policy copy | `prepare_bounded_workspace_session(CodexSessionProfileInput)` |
| `swallowtail_codex::smoke::run_codex_read_only_smoke` setup | the same read-only prepared profile; keep smoke prompt, turn deadline, outcome, and confirmation gate in Nucleus |
| direct `CodexAppServerDriver` construction | construct from `prepared.environment()` and execute `prepared_profile.plan()` plus `prepared_profile.request()` |

Stable Nucleus instance ids and route ids remain consumer inputs. Map the
existing access profile and status into `PreparedAccessEvidence`; use
`caller_asserted` only while Nucleus remains the named authority for that
status. The model, reasoning mode, developer instructions, dynamic tools,
working resource, and request ids map directly into `CodexModelSelection`,
`CodexSessionProfileInput`, and `SessionOptions`.

Keep `TurnRequest` construction, turn deadlines, callback execution, progress,
provider linkage, receipts, persistence, and terminal/cleanup projection
unchanged. Nucleus's existing fresh-session rule for tool-enabled stored chats
also remains unchanged because prepared resume rejects tool redeclaration.

Migration targets:

- `crates/nucleus-agent-adapters/src/swallowtail_codex.rs`
- `crates/nucleus-agent-adapters/src/swallowtail_codex/task_execution.rs`
- `crates/nucleus-agent-adapters/src/swallowtail_codex/smoke.rs`
- delete the superseded `discovery.rs` and `preflight.rs` only after parity
- narrow `host.rs` only after its executable, resource, environment, time, and
  joined-task authority is represented by the composed host

Card 012 stays planned until deterministic catalogue, chat, bounded-task, and
smoke parity passes inside Nucleus.

## Completion Evidence

- Nucleus catalogue, Agent Chat, task execution, and smoke now use
  `prepare_codex` and the matching prepared profile.
- `discovery.rs`, `preflight.rs`, the custom thread task service, manual
  host-service composition, and copied access-policy construction are removed.
- Deterministic preparation binds exact version `0.145.0`, caller-asserted
  access provenance, catalogue, read-only, and bounded-workspace policy.
- 18 focused adapter tests and 1,991 server tests pass.
- Nucleus health, workspace check, docs QA, Northstar QA, and
  `git diff --check` pass.
- Nucleus doctor no longer reports a health-task compile error; its known
  god-file error and generated-source warning remain.
- Authenticated installed probes remain separately gated and were not run.
- Nucleus retained executable-path and environment resolution as execution-host
  authority. Swallowtail owns version discovery and provider plan assembly.

Card 012 completed the deletion, ownership audit, rollback record, and
acceptance closeout in the same consumer-authorized batch.

## Stop Conditions

- facade needs a Nucleus-only public type
- migration changes tool, task, receipt, persistence, or UI policy
- bounded task access widens
- the Nucleus repository has conflicting user changes that cannot be preserved
- consumer mutation authority is absent

## Auto-Continuation

Yes, only within Nucleus and only after card 012 is explicitly ready from
passing functional parity.
