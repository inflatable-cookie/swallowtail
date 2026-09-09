# g05.043 Roadmap Backlog Retirement

Status: complete; PR #302 merged as `8ae707d5`
Owner: Northstar documentation cleanup worker
Created: 2026-09-09
Governing refs: Contract 001; installed Northstar Chatterbox and roadmap-backlog retirement procedures; operator cleanup authority of 2026-09-09
Depends on: g05.038; completed queue task `960585b1-6e7f-4a96-9427-411c00bb2776`

## Outcome

Retire Swallowtail's duplicate `docs/roadmaps/backlog/` intake layer after
giving every item a truthful disposition. Roadmaps retain promoted executable
tasks. `docs/triage/` retains unresolved or deferred candidates without
becoming execution authority.

## Ready-State Rubric

- [x] g05.038 merged and closed; `main` is clean and synchronized at
      `d85bb989be046b636337794fe7c3a71ba174822c`.
- [x] The queue has no unfinished Swallowtail task, worker, reviewer, or PR.
- [x] All nine backlog items have one unambiguous disposition below.
- [x] Scope is bounded to documentation, planning, instruction, template,
      fixture, and local-checker cleanup.
- [x] Mutable paths, preservation rules, validation, evidence, and stops are
      explicit.
- [x] No product, release, tag, new-lane, task-execution, abandonment, or
      generation-rollover authority follows.

## Decisions

Freeze this disposition manifest before deletion:

| Former backlog item | Disposition | Current destination |
| --- | --- | --- |
| Provider-Session Management Binding Persistence | unresolved/deferred; move its full current meaning | one new unique note at `docs/triage/20260909-151758-provider-session-management-binding-persistence.md`; promotion still requires a concrete consumer need after the matrix checkpoint |
| Aider Headless Route | already parked; remove duplicate stub | `docs/triage/2026-08-21-deferred-route-surfaces.md`; Research 143/153 and historical g03 evidence remain provenance |
| Hosted Interactive OAuth | already parked; remove duplicate stub | `docs/triage/2026-08-21-deferred-route-surfaces.md`; Contract 057 and historical g04 evidence remain provenance |
| Kiro Headless Route | already parked; remove duplicate stub | `docs/triage/2026-08-21-deferred-route-surfaces.md`; existing `kiro.acp` support is unchanged |
| OpenHands Agent Server Production Wiring | already parked; remove duplicate stub | `docs/triage/2026-08-21-deferred-route-surfaces.md`; Research 154-155 and historical g03 evidence remain provenance |
| Gemini CLI Range Requalification | implemented; remove | standing Contract 029 lane plus the compacted g04 roll-up and qualified-range evidence |
| Grok Build Maintained ACP Range | implemented; remove | compacted g01-g03 evidence, Contracts 015/029, and Research 070/085 |
| Pi RPC Session Continuity | implemented with the RPC fresh-only boundary retained; remove the backlog stub and fully promoted resolved triage duplicate | `pi.sdk-sidecar`, Contracts 017/019/029, Research 180-181, and the compacted g04 roll-up; `pi.rpc` remains fresh-only |
| Python Kimi CLI Headless Route | declined/no current need; remove without a placeholder | Research 068 and maintained native Kimi Code routes retain the decision evidence; reopening requires a new explicit operator selection |

Migration is not approval. None of these dispositions creates an executable
product task or changes the existing g05 frontier beyond this bounded cleanup.

## Dispatch manifest

- **State:** ready; one documentation cleanup lane; no concurrent Swallowtail
  sibling owns the affected paths.
- **Completion:** the manifest is applied, the backlog directory and live
  scaffolding are gone, current doctrine and checks agree on task-versus-triage
  ownership, review accepts the exact head, and `main` is synchronized.
- **Owned mutable paths:** `AGENTS.md`, `docs/README.md`,
  `docs/contracts/001-working-rules.md`, `docs/roadmaps/**`, the exact current
  triage notes and inbound links named by the manifest, relevant unsubmitted
  `docs/handoffs/**`, one cleanup log plus `docs/logs/README.md`, `effigy.toml`,
  and roadmap-specific local checker fixtures/tests.
- **Reserved closeout surfaces:** this task, `docs/roadmaps/README.md`,
  `docs/roadmaps/g05/README.md`, `docs/roadmaps/generation-index.md`, and the
  cleanup handoff.
- **Worker:** automatic general implementation pool; independent reviewer must
  use another provider/model identity.
- **Excluded:** Rust/product/runtime changes, provider calls, queued-work
  disposition, new product tasks, candidate changes, tags, releases,
  publication, consumer pins, generation rollover, and broad historical prose
  modernization.
- **Escalation:** Chatterbox owns an ambiguous destination or live-path
  collision; the operator owns product, release, abandonment, and generation
  choices.

## Work

1. Recheck the clean synchronized integration checkout, queue, active PRs,
   handoffs, agents, and worktrees. Stop if another unfinished lane owns an
   affected path.
2. Materialize the provider-session binding-persistence triage note with the
   full deferred scope, constraints, source, promotion gate, operator owner,
   and next check. Remove the fully promoted Pi continuity triage duplicate
   only after its current meaning and inbound links reach the canonical
   destinations above.
3. Repair current inbound links and wording, then delete every file under
   `docs/roadmaps/backlog/` and the directory itself. Do not retain aliases,
   moved-to stubs, or an empty directory.
4. Remove live backlog doctrine and checker/index scaffolding from current
   front doors, Contract 001, agent instructions, task templates, Effigy docs
   policy, deterministic checker fixtures, and unsubmitted handoffs. Preserve
   clearly historical logs, archive roll-ups, closed handoffs, immutable queue
   records, and quoted evidence unless deletion creates a current broken link.
5. Publish one closeout log carrying this manifest, exact changes/deletions,
   validation, retained historical exceptions, reviewed head, PR/merge, and
   the unchanged approved frontier.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Every item is dispositioned | A deleted file's only current meaning disappears | Nine-row manifest audit against the semantic diff and surviving destinations |
| Triage is intake, not execution | The new persistence note claims ready status, ordering, or dispatch authority | Exact note review plus current doctrine and task-pointer checks |
| Backlog is gone | A directory, index, template, alias, moved-to stub, or live checker still requires it | `find docs -type d -name backlog -print`, current-surface string inventory, and deterministic docs checks |
| Delivered decisions remain true | Pi becomes resumable over RPC, a parked route becomes active, or a declined route gains implied priority | Contract/route/task diff review and unchanged feature/route claims |
| Frontier and release state stay bounded | Cleanup tags `v0.4.4`, changes candidate identity, or promotes another product task | Diff review; candidate `49c9e3b2` and tree `1a9db127` remain unchanged; g05.036 stays a separate operator decision |
| History stays useful | Historical provenance is rewritten wholesale or a deleted target leaves a broken current link | Retained-exception inventory plus link validation |

## Stop conditions

- Stop if another unfinished lane owns any cleanup path.
- Stop if a backlog item has no unique truthful destination or its disposition
  conflicts with current authority.
- Stop rather than turning a candidate into a task, changing task order,
  rewriting history broadly, or crossing product/release scope.
- Ask the operator only if a new product, release, abandonment, or generation
  decision is genuinely required.

## Evidence

On completion record the nine-item manifest, new triage destination, deleted
and repaired paths, retained historical exceptions, validation actually run,
reviewed exact head, PR, merge, synchronized integration head, and unchanged
frontier.
## Result

- The frozen nine-item disposition manifest was applied. Binding persistence
  moved to the deferred triage note
  `docs/triage/20260909-151758-provider-session-management-binding-persistence.md`;
  the Aider, Hosted Interactive OAuth, Kiro, and OpenHands stubs were removed
  behind `docs/triage/2026-08-21-deferred-route-surfaces.md`; Gemini and Grok
  behind the standing Contract 029 lane and compacted evidence; Pi RPC
  continuity behind `pi.sdk-sidecar` with the fresh-only boundary retained;
  Python Kimi declined behind Research 068. `docs/roadmaps/backlog/` is
  deleted with no aliases, stubs, or empty directory, and live scaffolding is
  removed from `effigy.toml` and the roadmaps index.
- PR #302 was independently accepted at exact head
  `52423f055e7579a91b5b005d17662c04bf80c271`; review comment `5603707134`
  carried the accepted `ready_to_merge` marker and reported no blocking
  findings. It merged through the queue as
  `8ae707d5c367d24c566a8ad26c800cc159b2a44e`.
- `effigy qa:docs`, `effigy qa:northstar`, `git diff --check`, the backlog
  directory inventory, and the live `roadmaps/backlog` string inventory
  passed. No validation failure was deferred.
- Candidate `49c9e3b2`, tree `1a9db127`, and g05.036 are untouched. No tag,
  release, consumer pin, candidate, generation rollover, or product scope
  change. The exact-SHA `v0.4.4` tag decision remains with the operator.
- The integration checkout is synchronized at
  `8ae707d5c367d24c566a8ad26c800cc159b2a44e`. No new dispatch is authorized
  by this closeout; planning direction is needed before selecting the next
  task.

## Next task

Return to Chatterbox after merge. The retirement is complete through PR #302
at `8ae707d5`; planning direction is needed before selecting the next task.
The separate exact-SHA `v0.4.4` tag decision remains operator-owned; this
cleanup grants no tag authority.
