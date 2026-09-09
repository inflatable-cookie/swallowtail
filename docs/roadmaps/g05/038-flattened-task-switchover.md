# g05.038 Flattened Task Switchover

Status: complete; PR #301 merged as `8dc1f161`
Owner: Northstar documentation migration worker
Created: 2026-09-09
Governing refs: Contract 001; installed Northstar project-refresh, lifecycle-maintenance, and compile-roadmaps procedures; operator switchover authority of 2026-09-09
Depends on: completed Swallowtail Card 154 queue task `d3390475-778e-4850-bcee-7b2ff91e487a`

## Outcome

Replace Swallowtail's milestone-plus-batch-card planning hierarchy with the
installed generation-plus-task model. Compact closed generations `g01`–`g04`,
make `g05` the single expanded generation roadmap, and preserve every open
commitment as one top-level `g05.NNN` Northstar task. Do not alter product code,
the frozen `v0.4.4` candidate, Card 323's Desktop execution, or release state.

## Ready-State Rubric

- [x] Scope is bounded to documentation, instruction, template, and local
  deterministic checker migration.
- [x] Installed Northstar authority defines `docs/roadmaps/gNN/NNN-<slug>.md`
  as the sole executable planning unit and rejects active milestone wrappers
  and `batch-cards/`.
- [x] Swallowtail `main` is clean and synchronized at
  `0015e8195d2cc36cad55347edfdf81e6bb816bb1`.
- [x] Every submitted Swallowtail old-format queue record is done. The only
  unfinished adjacent record is Desktop Card 323; it writes Desktop only and
  remains bound to frozen candidate merge `49c9e3b2` and tree `1a9db127`.
- [x] Scope, preservation rules, validation, evidence, and stops are explicit.
- [x] No generation rollover or product/release authority is implied.

## Decisions

- `g01`–`g04` are safely closed from their front-door status and generation
  boundaries. Preserve durable authority in current contracts/architecture,
  open commitments in the active backlog or standing lanes, and selected
  release/PR/validation evidence in one non-procedural roll-up per generation.
- `g05.001`–`g05.037` keep their original roadmap IDs and become tasks. Fold
  completed card outcomes into their owning task or the migration evidence
  record before deleting the nested files.
- Preserve unresolved old cards as distinct tasks after this migration task:
  former cards `005`, `006`, `130`, and `134` map to `g05.039`–`g05.042` in
  dependency order. Preserve full scope, ownership, acceptance, evidence, and
  stop conditions. Other gated follow-ons already have an active backlog home
  or an honest stopped parent and do not become duplicate executable tasks.
- `g05.036` remains the release-acceptance task. It records the frozen candidate
  and waits for Desktop Card 323; it cannot tag or publish.

## Dispatch manifest

- **State:** ready; one migration lane; no concurrent Swallowtail planning or
  implementation siblings; Desktop Card 323 may continue because it owns no
  Swallowtail mutable path.
- **Completion:** one active generation-plus-task hierarchy, closed-generation
  roll-ups, a preserved old/new map, repaired current links and checkers,
  exact-head independent review, merge, and synchronized `main`.
- **Owned mutable paths:** `AGENTS.md`, `docs/README.md`, `docs/contracts/001-working-rules.md`,
  `docs/roadmaps/**`, relevant unsubmitted `docs/handoffs/**`, `docs/logs/README.md`,
  one migration log, `effigy.toml`, and roadmap-specific local checker/tests.
- **Reserved closeout surfaces:** this task, `docs/roadmaps/g05/README.md`,
  `docs/roadmaps/README.md`, `docs/roadmaps/generation-index.md`, and the
  migration handoff.
- **Worker:** automatic general implementation pool; independent reviewer must
  use another provider/model identity.
- **Excluded:** Rust/product/runtime changes, provider calls, queue-plugin
  changes, Card 323 mutation, candidate changes, tags, releases, publication,
  consumer repins, generation rollover, and historical prose modernization.
- **Escalation:** Chatterbox owns ambiguous task ownership, ordering, IDs, or
  evidence destinations; operator owns release and generation choices.

## Work

1. Freeze a preservation manifest before deletion: every expanded generation,
   inbound current links, unique authority destinations, open commitments,
   selected material evidence, exact paths to remove, and the active old/new
   ID map.
2. Compact safely closed `g01`–`g04` into `docs/roadmaps/archive/g01.md` through
   `g04.md`. Rehome open commitments, rewrite current links, and delete only
   manifest-listed expanded trees. Keep logs and closed handoffs historical.
3. Flatten `g05`: retain tasks `001`–`037`, create `039`–`042` from the four
   unresolved cards, absorb material completed-card evidence, remove the
   nested `batch-cards/` hierarchy and milestone/card indirection, and make the
   generation README the roadmap plus approved frontier.
4. Replace legacy task/card wording and structure in current front doors,
   instructions, Contract 001, templates, unsubmitted handoffs, Effigy docs
   policy, and local roadmap checkers. Keep “queue task” and “Effigy task”
   distinct from the canonical Northstar task.
5. Prove unique `gNN.NNN` identities, coherent frontier pointers, complete link
   destinations, exact deletion coverage, preservation, and idempotent second-
   pass lifecycle/currentness behavior.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| One executable level | A current surface dispatches a nested card or milestone wrapper | Repository-wide current-surface search and deterministic checker fixtures reject it |
| Current work survives | Former card `005`, `006`, `130`, or `134` is deleted without a top-level task carrying its full contract | Frozen old/new map plus semantic diff of `g05.039`–`g05.042` |
| Closed history stays useful | A deleted generation held the only live rule, open commitment, or material release evidence | Preservation manifest, four roll-ups, current destinations, and link check |
| Frontier is singular | Front doors name different active tasks or a ready task sits outside the generation frontier | Roadmap checker fixtures and front-door comparison |
| Frozen release state is unchanged | Candidate SHA/tree, Card 323 budget, tag state, or release note meaning changes | Diff review plus unchanged candidate/release identities |
| Migration is direct | A compatibility alias, empty `batch-cards/`, legacy template, or dual checker path remains | Path/string inventory and second lifecycle pass |

## Stop conditions

- Stop if any unfinished Swallowtail old-format queue record appears before
  mutation, or another lane owns a path this task must remove.
- Stop if unique authority or an open commitment has no safe destination.
- Stop for Chatterbox if task ownership, dependency order, ID assignment, or
  evidence destination differs materially from the frozen mapping above.
- Stop rather than rewriting historical evidence or broadening into product,
  provider, release, or queue-plugin work.

## Evidence

On completion record the historic-generation classification and preservation
manifest, old/new active map, changed/deleted files, validation, reviewed exact
head, PR, merge, retained exceptions, new frontier, and dispatch-resumption
state.

## Result

- The frozen preservation manifest and old/new task map were applied. Closed
  generations `g01`–`g04` are compacted, `g05` has 42 top-level tasks, and
  former cards `005`, `006`, `130`, and `134` are preserved as `g05.039`–
  `g05.042`.
- PR #301 was independently accepted at exact head
  `67432708f092c517e50cc577b791007f264f04eb`; review comment `5603203437`
  carried the accepted `ready_to_merge` marker and reported no blocking
  findings. It merged through the queue as
  `8dc1f161b35ca5620441f6d9d30bb8808721c68d`.
- `effigy qa:docs`, the feature-matrix cross-classification check, the
  roadmap status and number-collision fixtures, and `git diff --check` passed.
  The configured Pinned MSRV floor tests job was skipped; no migration failure
  was deferred.
- The Desktop Card 323 exact-tree acceptance remains reconciled in `g05.036`.
  Candidate merge `49c9e3b291609c9ebf5b35a284c08302f3b8d5e3`, tree
  `1a9db12742b68e839dfebe42f0633f5d1aa0265b`, tags, releases, and consumer
  pins are unchanged. The exact-SHA `v0.4.4` tag decision remains with the
  operator.
- The integration checkout is synchronized at
  `8dc1f161b35ca5620441f6d9d30bb8808721c68d`. No new dispatch is authorized
  by this closeout; planning direction is needed before selecting the next
  task.

## Next task

Return to Chatterbox after merge. Desktop Card 323 completed during this
migration (2026-09-09); its exact-tree acceptance is reconciled into g05.036
in this batch. Present the separate exact-SHA tag decision: no tag follows
without explicit operator authority naming the exact SHA. Planning direction
is needed before selecting the next task.
