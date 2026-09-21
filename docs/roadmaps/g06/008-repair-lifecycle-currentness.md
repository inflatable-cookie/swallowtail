# g06.008 — Repair lifecycle currentness

Owner: repository maintainers
Created: 2026-09-21
Governing refs: installed Northstar lifecycle currentness contract; Northstar g03.012; Queue Spec 006; Northstar g03.020
UI classification: none

## Outcome

Swallowtail has one mechanical lifecycle authority: no lifecycle-managed task file
or declared projection target carries a duplicate status header, and no
live-currentness section names a terminal task.

## Ready-State Rubric

- [x] The installed Northstar skill exposes `lifecycle:run audit-currentness`.
- [x] The integration checkout is clean and synchronized with `origin/main`.
- [x] The read-only audit recorded the exact findings below.
- [x] The repair boundary excludes product files and semantic rewriting.

## Decisions

- Repair only findings emitted by `audit-currentness`.
- Delete duplicate status header lines without changing surrounding semantic
  prose.
- Replace a stale terminal-task frontier value with task-free sequencing intent.
  Preserve Goal/State history.
- Return ambiguous meaning to Chatterbox untouched.

## UI Design Brief

Not applicable.

## Dispatch manifest

- **State:** ready; one documentation-only repair lane.
- **Owned mutable paths:** the exact files listed under Evidence, this task, and
  its submitted handoff.
- **Reserved closeout surfaces:** this task's lifecycle record, declared
  generated projection blocks, and deletion of the submitted handoff belong to
  the repository hook.
- **Worker:** automatic mechanical pool; independent exact-head review remains
  required.
- **Excluded:** product code, releases, generation rollover or compaction, new
  product planning, arbitrary prose cleanup, Queue/Effigy source, and Paseo
  thread/workspace disposal.
- **Escalation:** any finding requiring semantic judgment returns to the owning
  Chatterbox without edits.

## Work

1. Re-run `effigy skill run northstar/lifecycle:run -- audit-currentness --repo .`
   and confirm the inventory below.
2. Delete only the duplicate status header lines the audit reports.
3. Replace each stale terminal-task frontier value with task-free sequencing
   intent; preserve history cells.
4. Prove the audit is clean, lifecycle projections verify, documentation checks
   and normal QA pass, and the diff contains no product files.
5. Land through Queue review and repository-hook closeout.

## Acceptance and review oracle

- `audit-currentness` reports `status: ok` with no violations.
- Lifecycle projection verification reports no drift.
- Every change outside this task and handoff is one listed mechanical repair;
  surrounding semantic prose remains intact.
- Repository docs checks, normal QA, and `git diff --check` pass.
- Exact-head independent review confirms no product or unrelated planning
  change.

## Stop conditions

Stop before editing when a stale pointer cannot be replaced without choosing
product direction, a status-looking line carries semantic meaning beyond the
marker, the audit differs materially from Evidence, the repository has
concurrent conflicting documentation changes, or validation exposes a broader
defect. Never renumber or rewrite task files to satisfy the audit.

## Evidence

Read-only currentness findings from the installed audit (2 total):

- `docs/roadmaps/g06/README.md` `Frontier` names terminal task `g06.006`.
- `duplicate-status-header` in
  `docs/roadmaps/g06/006-command-code-1-54-0-paid-model-live-requalification.md`.

Transient-handoff backlink inventory: none found.

## Next task

Return to the existing project frontier after hook closeout. This repair
authorizes no product successor.
