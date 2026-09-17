# g05.084 Roll Over To g06 And Compact g05

Status: ready
Owner: Tom
Created: 2026-09-17
Governing refs: Contract 001; the installed Northstar lifecycle-maintenance
(closed-generation compaction) procedure
Depends on: g05.066, g05.069, g05.080, g05.081 (serial edge — compact only
settled work)

## Outcome

Close g05, open g06, and compact g05 into a non-procedural archive roll-up
without losing an open commitment. g05 is at 84 numbered tasks, past its
30–50 range, and Tom authorized the structural rollover on 2026-09-17.

## Ready-State Rubric

- [x] The operator authorized the rollover explicitly on 2026-09-17.
- [x] g05.038 established the Swallowtail precedent and the preservation
      oracle for compacting `g01`–`g04` into `archive/gNN.md` roll-ups.
- [x] Every open commitment is named below and has a destination.
- [x] The task may edit planning, instruction, and checker surfaces only; it
      changes no product code and no release state.
- [ ] `main` is clean and synchronized with `origin/main` at execution time.
- [ ] No unfinished Swallowtail queue record outside the four serial
      prerequisites is in flight.

## Decisions

- `g05` is safely closed once its four live lanes settle. Its durable rules
  already live in `docs/contracts/` and `docs/architecture/`; its lasting
  outcomes live in `docs/logs/` closeouts and the generation index census.
- `g06` carries the unclosed product outcomes rather than restating g05. Its
  purpose is recorded as the carried-forward set, not as new product policy:
  Chatterbox settles any wider g06 focus with the operator separately.
- Compaction deletes the numbered `g05` task files only after the roll-up, the
  preservation manifest, and the carried-forward tasks exist and validate.

## Dispatch manifest

- **State:** ready; serial after g05.066, g05.069, g05.080, and g05.081;
  documentation-only lane; no automatic successor.
- **Completion:** `archive/g05.md` is a non-procedural roll-up naming the
  generation's boundary, shipped outcomes, and selected evidence;
  `archive/preservation-manifest.md` records the classification, the deleted
  set, and every carried-forward destination; `docs/roadmaps/g06/` exists with
  its README and carried-forward tasks; `generation-index.md` shows g05
  completed and g06 active; the front door names g06 and its Next Task;
  `.northstar/lifecycle/v1/projection-targets.json` declares `g06` and its
  targets; the `g05` docs index policy in `effigy.toml` becomes `g06`; every
  generated lifecycle projection block is regenerated with its new digest; the
  full docs board passes; independent exact-head review accepts the head.
- **Owned mutable paths:** `docs/roadmaps/archive/g05.md`;
  `docs/roadmaps/archive/preservation-manifest.md`; `docs/roadmaps/g06/**`;
  `docs/roadmaps/README.md`; `docs/roadmaps/generation-index.md`;
  `docs/roadmaps/standing-lanes.md`; `docs/roadmaps/status-grammar.md`;
  `docs/README.md`; `.northstar/lifecycle/v1/**`; `effigy.toml`; and the
  removal of `docs/roadmaps/g05/**` after carry-forward.
- **Reserved closeout surfaces:** `docs/logs/` (one closeout record);
  the lifecycle task record and generated projections.
- **Worker:** documentation-migration worker with Northstar
  lifecycle-maintenance loaded; no product authority.
- **Excluded:** product/runtime code; contracts and architecture semantics
  (reference them, do not rewrite them); release, tag, or publication; consumer
  repositories; inventing a new g06 product goal; deleting a note or record
  whose destination is unclear instead of stopping.
- **Escalation:** operator via Chatterbox for any unresolved destination or a
  new g06 goal; Queue coordinator for mechanical blockers.

## Carry-forward set

Preserve each of these as one top-level `g06.NNN` task with its full outcome,
rubric, acceptance, evidence, and stop conditions, unless it completed before
this task ran:

- the four serial prerequisites (`g05.066`, `g05.069`, `g05.080`, `g05.081`)
  if still unfinished;
- `g05.035` shared harness capability and producer boundary — planned, no
  implementation authority;
- `g05.039`/`g05.040` Qoder skill visibility — gated on a non-empty
  Research 256 deliver-now disposition that does not currently exist;
- `g05.041` persistent permission grant admission — no operator promotion;
- `g05.042` registered-tool adoption for remaining ACP routes — no consumer
  requirement or operator direction.

Anything not carried forward and not completable is recorded with its reason in
the roll-up; the five triage notes stay where they are and are not triage
inputs to this task.

## Work

1. Run the preservation oracle: classify every `g05` task, confirm no live rule
   exists only inside the tree, and confirm every open commitment has an active
   destination. Stop and name the blocker if any does not.
2. Write `archive/g05.md` as a non-procedural roll-up on the `g04` model, and
   update `archive/preservation-manifest.md` with the classification, the
   deleted set, and the destination map.
3. Open `docs/roadmaps/g06/` with its README and the carried-forward task set,
   using the flattened `gNN.NNN` shape and the task template.
4. Update the front door, the generation index, status grammar, standing lanes,
   the lifecycle projection targets, the lifecycle projection blocks and their
   digests, and the `effigy.toml` index policy.
5. Record g05's closure: its closed disposition and the task digest, in the
   shape the installed lifecycle procedure requires.
6. Delete `docs/roadmaps/g05/**` last, then run the full docs board and confirm
   no reference resolves into the removed tree.
7. Commit and push as one coherent planning change for independent exact-head
   review. Its own task record is folded into `archive/g05.md` and the closeout
   log before its file is deleted.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| No open commitment is lost | A planned `g05` task has no `g06` destination and is not named as closed | destination map in the preservation manifest covers every task |
| No live rule lives only in the removed tree | A contract or guide is cited only from a deleted task file | each durable rule resolves to a contract, architecture note, or guide |
| The index and projections agree with reality | the projection block still declares `g05` active, or names a removed path | `effigy qa:docs` and the lifecycle projection check pass |
| Compaction is not re-expanded | a later run re-creates `g05` files from a stale index | no remaining reference resolves into `docs/roadmaps/g05/` |

## Stop conditions

- Stop and name the blocker when an open commitment has no active home, two
  destinations conflict, or a clean detection of generation signals is not
  possible.
- Stop rather than guess if the operator's g06 goal is needed to place a task.
- Do not compact an unfinished serial prerequisite; wait for it.

## Evidence

On completion, record the roll-up and manifest paths, the carry-forward
mapping, the removed path set, validation actually run, PR link, reviewed exact
head, and merge commit.

## Next task

Chatterbox resumes planning on the g06 frontier: reconcile the carried-forward
set, compile the next ready lane, and settle g06's wider focus with the
operator.
