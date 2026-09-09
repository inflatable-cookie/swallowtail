# 2026-09-09 g05.038 Flattened Task Switchover Migration

Status: migrated, unmerged
Owner: Northstar documentation migration worker
Task: `docs/roadmaps/g05/038-flattened-task-switchover.md`
Manifest: `docs/roadmaps/archive/preservation-manifest.md`

## What Changed

Swallowtail planning moved once from milestone-plus-batch-card to the
installed generation-plus-task model. `g01`–`g04` are compacted into
`docs/roadmaps/archive/g01.md` through `g04.md`; `g05` is the sole expanded
generation with 42 top-level tasks (`001`–`042`); the 155-file
`batch-cards/` hierarchy is deleted. Former cards `005`, `006`, `130`,
`134` are preserved in full as `g05.039`–`g05.042`. Consumed card manifests
in `g05.029`, `g05.034`, and `g05.035` are folded into evidence ledgers with
task-owned held gates (087 qualified-ranges, real-route gate, first-turn
diagnostic, Kimi follow-on scope). Instructions (`AGENTS.md`), Contract 001,
templates (batch-card template replaced by the roadmaps task template),
Effigy docs policy, and all three local roadmap checkers plus their fixtures
now speak tasks.

## Current State

- Frontier: ready tasks `029`, `031`, `034`, `036`, `038`; planned `035`,
  `039`–`042`; stops at `002`, `003`, `007`, `011`; 28 complete.
- Desktop Card 323 completed exact-tree acceptance of candidate merge
  `49c9e3b2` on 2026-09-09 (PR #186, review `5602628180`, merge
  `32344306fe30043e2d08dd9ac4e3a216b320c81f`, closeout `da9edb8d...`); the
  evidence is reconciled into `g05.036` in this batch. The exact-SHA tag
  decision remains a separate explicit operator decision; no tag authority
  follows from the migration.
- Candidate, tree (`1a9db127…`), tags, releases, Card 323 ownership, and
  consumer/provider state are unchanged.

## Validation

- `python3 scripts/check-roadmap-status-drift.py` — passed (census parity,
  task index coverage, annotation agreement, no nested dispatch).
- `bash scripts/tests/roadmap-status-drift.sh` — passed (includes new
  legacy-level rejection and census-parity fixtures).
- `bash scripts/tests/roadmap-number-collision.sh` — passed (task-occupancy
  fixtures).
- `bash scripts/check-provider-route-matrix.sh` — passed (41 rows, 719
  unavailable cells, 4 pinned producer gaps on tasks `041`/`042`).
- `effigy qa:docs` — full docs board (record result at PR time).
- `git diff --check` — clean.

## Retained Exceptions

- Past-tense card evidence citations stay where they attribute frozen
  outcomes; rewriting them to task IDs would falsify which unit did the work.
- `docs/logs/`, closed `docs/handoffs/`, immutable queue records, old specs,
  release notes, and archive prose are not modernized.
- `Card129` / `Research 290` cross-classification identifiers stay as frozen
  validator vocabulary.
- Frozen planning-history pointers (`this card's ## Result`, serial-edge rows,
  resolved runway conditionals) name retired card files; destinations live in
  git history and the folded ledgers.

## Next Move

Review this migration at its exact head, merge through the queue, synchronize
`main`, and return to Chatterbox. After merge, present the separate exact-SHA
`v0.4.4` tag decision; resume only the approved flattened frontier.
