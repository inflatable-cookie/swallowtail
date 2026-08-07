# 152 Docs Index Checks And Drift Repair

Status: planned
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

- [ ] `effigy docs check index` passes for logs, research, and roadmaps
- [ ] every `.md` file under the three directories is indexed exactly once
- [ ] no dangling links in the three indexes

## Stop Conditions

- stop if an index entry must point at a non-existent file

## Auto-Continuation

Yes, to card 153 after acceptance.

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
