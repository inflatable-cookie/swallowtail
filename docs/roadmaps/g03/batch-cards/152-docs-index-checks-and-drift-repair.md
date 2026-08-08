# 152 Docs Index Checks And Drift Repair

Status: completed
Owner: Tom
Created: 2026-08-08
Milestone: `../051-validation-machinery-and-index-closure.md`
Depends on: card 151

## Goal

Make the logs, research, and roadmaps indexes machine-checked and repair the
verified drift.

## Scope

1. Add `docs_policy.indexes` entries for `logs` and `research` in
   `effigy.toml` mirroring the existing `vision` entry, and wire
   `effigy docs check index` tasks for logs, research, and roadmaps into
   `qa:docs`.
2. Add the four missing log entries to `docs/logs/README.md`:
   `2026-07-26-nucleus-native-pilot-clean-launch-four.md`,
   `2026-07-26-nucleus-native-pilot-clean-launch-three.md`,
   `2026-07-26-nucleus-native-pilot-closeout.md`,
   `2026-08-02-acp-stable-session-list-codec.md`.
3. Repair the roadmaps drift: add the three missing g01 handoff entries, add
   `049-generation-closeout-and-g03-cutover.md` to g02, and remove the two
   dangling g02 links (`grok-build-maintained-acp-range.md`,
   `provider-session-management-binding-persistence.md`).
4. Confirm `docs/research/README.md` stays complete (currently 112/112).

## Out Of Scope

- index policy beyond logs, research, and roadmaps
- CI wiring (card 153)

## Acceptance

- [x] `effigy docs check index` passes for logs, research, and roadmaps
- [x] every `.md` file under the three directories is indexed exactly once
- [x] no dangling links in the three indexes

## Stop Conditions

- stop if an index entry must point at a non-existent file

## Auto-Continuation

Yes, to card 153 after acceptance.

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`

## Completion Evidence

- the effigy index check requires every file under a directory to be linked
  with a `(./file.md)`-prefixed markdown link, so all three indexes were
  converted: logs (410 links plus four new entries), research (112 links),
  the roadmaps root Index section, the g01/g02/g03 Milestones sections,
  the backlog items, and the g03 batch-cards index (157 entries converted
  plus six previously unindexed cards 064-069 added)
- drift repaired: the four missing log entries, the three missing g01
  handoff records, and the missing g02 closeout milestone; the audit's
  "two dangling g02 links" were re-verified as resolving correctly to
  `../backlog/` files and needed no change
- eight index policies wired in `effigy.toml` (logs, research, roadmaps,
  g01, g02, g03, backlog, batch-cards) with per-directory excludes, and
  eight `qa:docs:index:*` tasks added to the `qa:docs` sequence
- the roadmaps next-action check now walks the index's linked files, so
  `generation-index.md` and `long-term-plan.md` gained `## Next Task`
  pointers to the front door
- `qa:docs` runs all fifteen checks green, `qa:northstar` passes, and a
  repo-wide link scan finds zero broken links across 2,465 links
