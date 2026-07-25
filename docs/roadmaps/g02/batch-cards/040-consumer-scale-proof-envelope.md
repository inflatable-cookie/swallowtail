# 040 Consumer-Scale Proof Envelope

Status: completed
Owner: Tom
Created: 2026-07-25
Completed: 2026-07-25
Milestone: `../014-consumer-scale-application-proof-and-hardening.md`

## Objective

Turn “prove Swallowtail in a working app at scale” into one exact,
consumer-owned Nucleus-first workload envelope.

## Governing Refs

- Contract 036
- consumer runtime evidence architecture
- repository authority map
- current Nucleus and Soundcheck authority surfaces

## Scope

1. Audit current normal application paths, existing live/native proof,
   diagnostics, persistence, and gated commands.
2. Select exact Nucleus scenarios for catalogue, read-only chat, callback,
   cancellation, deadline, process/session turnover, restart, recovery, and
   bounded workspace execution.
3. Record repetitions, supported concurrency, time, provider spend, test data,
   workspace effects, versions, topology, and stop conditions.
4. Separate read-only pilot, provider-write, and workspace-write authority.
5. Define safe evidence capture and defect triage.
6. Record the later Soundcheck structured-run comparison.
7. Do not edit either consumer or make a provider call.

## Acceptance Criteria

- [x] every scenario enters through a normal application path
- [x] exact workload and lifecycle counts replace vague “scale” wording
- [x] concurrency is bounded by realized application and provider behavior
- [x] provider state, cost, credentials, test data, and workspace effects are
      explicit
- [x] safe diagnostics and persisted consumer evidence are named
- [x] Swallowtail versus consumer defect ownership is testable
- [x] cards 041-043 have exact authority and stop gates

## Validation

- Swallowtail docs and Northstar checks
- Nucleus and Soundcheck read-only task and authority audit
- `git diff --check`

## Stop Conditions

- the current consumer path cannot be identified without editing active work
- a live call or workspace write becomes necessary
- exact scale or spend would establish unresolved product policy
- the selected evidence would expose prompts, provider payloads, or secrets

## Audit Findings

### Nucleus

- The native path is
  `AgentChatPanel` -> `send_agent_chat_message` ->
  `LocalCodexChatService` -> `SwallowtailCodexSessionRuntime` -> Codex
  app-server. Catalogue discovery uses the same prepared Swallowtail
  integration.
- Agent Chat opens read-only sessions with 30-second catalogue and 180-second
  turn deadlines. Its dynamic tools are exactly `task_ledger` and
  `task_workflow`.
- Tool-enabled stored chats intentionally reopen a fresh provider session with
  transcript context. They do not claim native resume because the qualified
  Codex schema cannot safely redeclare tools during resume.
- One mutex serializes native Agent Chat turns. Multiple cached sessions may
  keep separate app-server children alive, but parallel turn execution is not
  a realized product capability.
- The desktop database and review snapshots are fixed under
  `$HOME/.nucleus/state`. The native app has no explicit isolated-state
  override. Repointing `HOME` is too broad and is not an acceptable proof
  mechanism.
- Swallowtail exposes cancellation, and Nucleus uses it in lower-level smoke
  and task paths. Native Agent Chat exposes only message send; it has no normal
  UI or Tauri cancellation command.
- A gated `nucleusd` Codex read-only smoke exists. It uses `gpt-5.4-mini`, low
  reasoning, a 120-second limit, explicit provider-write confirmation, no task
  mutation, joined cleanup, and safe summary evidence. It is useful
  diagnostic evidence, not normal-path application proof.
- The installed executable observed during this audit is
  `codex-cli 0.145.0`. The consumers use Swallowtail through sibling path
  dependencies.
- The Nucleus worktree contains unrelated active desktop and Agent Chat work.
  Consumer edits must wait for a non-overlapping companion lane or a
  checkpoint.

### Soundcheck

- The native structured-run path already uses prepared Swallowtail catalogue
  and exec profiles with schema, screenshot, reasoning, search, progress,
  cancellation, deadline, validation, and cleanup behavior.
- `SOUNDCHECK_LIBRARY_DB_PATH` provides an app-specific isolated data path.
- Existing native acceptance covers authenticated catalogue, normal workflow,
  progress, and cancellation. It does not replace the repeated application
  workload below.
- The Soundcheck worktree contains a large unrelated DAW-state lane. Consumer
  edits and live proof wait until that lane permits them.

## Required Nucleus Companion Work

Before a paid or rate-limited pilot, Nucleus must add:

1. one explicit desktop database and snapshot root override scoped to Nucleus;
2. one normal Agent Chat cancellation command and UI action wired to the
   active Swallowtail turn;
3. one explicit proof-only turn-deadline override using the production
   deadline path; and
4. one Effigy selector that launches the native proof profile and captures
   only the safe evidence named below.

These are consumer integration features. They do not belong in Swallowtail and
do not weaken the production defaults.

## Recommended Nucleus Pilot Envelope

The pilot uses a fresh Nucleus state root and a dedicated disposable fixture
repository mounted read-only. It uses ambient Codex subscription access
without copying credentials. The exact Swallowtail source commit, Nucleus
commit, Codex executable version, selected model, compatibility
classification, macOS version, and local-host topology are frozen before the
first call.

The selected model is `gpt-5.4-mini` if catalogue discovery still reports it.
Absence is a stop condition, not a fallback. The pilot performs:

- 3 native app launches separated by 2 full app restarts;
- 3 catalogue attempts, one per launch;
- 12 Agent Chat turn attempts:
  - 6 ordinary successful turns;
  - 3 successful read-only `task_ledger` or `task_workflow` inspection
    callbacks against seeded fixture records;
  - 1 operator cancellation through the normal Agent Chat control;
  - 1 controlled deadline through the proof-only deadline setting; and
  - 1 successful post-restart recovery turn;
- 6 app-server process and provider-thread lifecycles:
  - 2 conversations opened during launch one;
  - 3 sessions opened or reopened during launch two; and
  - 1 conversation reopened during launch three;
- no parallel turns and no more than 3 live app-server children;
- no task, workspace, SCM, forge, proposal, or provider-account mutation.

At most 3 failed-scenario reruns are permitted: 15 turn attempts total, 6
provider threads total, and 60 minutes wall time. The route must remain
subscription-backed. If Codex reports separately metered API billing, a
credential-audience change, or a paid route outside that subscription, stop.

## Recommended Sustained Nucleus Envelope

After the pilot and every discovered defect close:

- 5 native app launches separated by 4 restarts;
- 5 catalogue attempts;
- 50 Agent Chat turn attempts across 2 durable conversations reopened on
  every launch:
  - 35 ordinary successful turns;
  - 10 successful read-only inspection callbacks;
  - 3 operator cancellations; and
  - 2 controlled deadlines;
- 10 app-server process and provider-thread lifecycles;
- one active turn at a time and no more than 2 live app-server children;
- at most 5 exact-scenario reruns, for 55 turn attempts total;
- 4 hours maximum wall time;
- the same subscription, model, fixture repository, and isolated state
  boundary as the pilot.

Writable task execution is a separate tranche. If separately approved, it
adds exactly 10 task-run attempts in disposable Git worktrees: 6 completions,
2 cancellations, 1 approval or user-input stop, and 1 process-interruption
recovery. Only one task may run at once. No commit, push, forge, review
acceptance, or durable user-workspace mutation is permitted. Its provider-call
and 4-hour wall limits are approved independently.

## Recommended Soundcheck Envelope

After Nucleus passes and Soundcheck's current lane permits proof work:

- 4 native launches using a fresh `SOUNDCHECK_LIBRARY_DB_PATH`;
- 16 normal product workflow starts: 8 baseline structured runs, 4 screenshot
  runs, 2 search-enabled runs, 1 cancellation, and 1 controlled deadline;
- no more than 20 provider run attempts, allowing at most 4 product-owned
  repair attempts;
- one active structured run at a time;
- 2 hours maximum wall time;
- fixture screenshots and library data only;
- no proposal application and no DAW, library, or user-file mutation.

Search needs separate network and provider-spend approval. Its absence does not
silently become a non-search run.

## Evidence And Defect Rules

Each attempt records a generated scenario ID, exact version tuple, operation
kind, expected terminal state, observed terminal state, event counts, callback
counts, usage and rate evidence when supplied, process cleanup, persisted
consumer state, elapsed time, and a redacted error code or summary.

Credentials, prompts, assistant output, raw provider payloads, raw streams,
absolute user paths, and raw provider thread or turn IDs are not retained.
Provider references may appear only as salted run-local correlation hashes.

A failure belongs to Swallowtail when the prepared plan, transport,
normalization, lifecycle, cancellation, deadline, callback exchange,
compatibility classification, cleanup, or stable diagnostics violate a
Swallowtail contract. Consumer prompt, fixture, persistence, UI, authorization,
or workflow failures stay with the consumer. Every Swallowtail defect first
gets a deterministic fixture regression, then repository validation, then an
exact replay of the failed application scenario.

Any secret exposure, raw-payload retention, state-boundary escape, unjoined
child, unexplained provider state, silent fallback, rate or spend boundary
breach, or mismatch between expected and persisted terminal state stops the
run.

## Execution Evidence

- Nucleus and Soundcheck repository, task, worktree, native-path, persistence,
  cancellation, and live-smoke surfaces were inspected read-only.
- Nucleus g05 cards 007-009 now implement the required isolated profile,
  bounded deadline, normal cancellation, exact terminal persistence, proof
  selectors, and sanitized evidence in its working tree.
- Nucleus deterministic readiness passes without credentials or provider
  calls. Its proof changes still need an exact source checkpoint before card
  041 can freeze the pilot tuple.
- No provider call, consumer edit, credential read, workspace write, registry
  mutation, push, tag, or release occurred.
- The exact workload is accepted as the planned envelope. Card 041 remains
  gated on an exact Nucleus checkpoint and explicit live-call approval.

## Auto-Continuation

No. Card 041 needs a Nucleus companion roadmap, non-overlapping consumer
authority, and explicit approval of the 15-attempt pilot ceiling.
