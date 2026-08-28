# 2026-08-28 g05.001 Card 003 Contract Promotion

## Result

Card 003 and g05.001 are complete.

- Contract 058 governs effective selected-context skill visibility, including
  operator-installed global and project skills without ambient scanning.
- Contract 059 governs host-owned turn-scoped watchers, model and operator
  control, bounded summaries, explicit wait, joined cleanup, and same-turn
  completion interception.
- Product guardrails record both boundaries. No unrealized package structure
  was added to system architecture.
- Qoder `1.1.25` is the sole first skill-evidence candidate.
- Claude Code headless is the sole first watcher-mechanism candidate.
- Cards 004, 007, and 008 are independent ready worker lanes. All later route
  binding remains planned behind explicit gates.

## Startup Check

`effigy doctor` reports the inherited `scan.god-files` error with 381 findings,
plus the stale graph and generated-in-src warnings. This card does not alter
that baseline. `effigy test --plan` selects the full workspace Nextest suite;
the docs-only orchestrator closeout does not run it.

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Next Move

Launch cards 004, 007, and 008 in parallel through their manual worker
handoffs. Review, fix, merge, and restack serially. Do not start dependent
cards early.
