# 2026-08-05 Missing Route Guide Closure

Roadmap: `../roadmaps/g03/042-complete-integration-guide-system.md`
Card: `../roadmaps/g03/batch-cards/119-missing-route-guides-and-examples.md`

## Changed

- added route-explicit Antigravity guidance for catalogue, structured run, and
  exact-id continuation
- added route-explicit Cursor guidance for catalogue, ACP session, and headless
  structured run
- added Grok Build ACP run/session guidance and preserved permission-stop,
  private task-control, durable-state, and attachment-recovery truth
- added Oh My Pi catalogue/run/session guidance with local auth, RPC v2,
  reasoning, PNG, question, framing, and context-losing replacement boundaries
- added six compiling normal-path examples for the previously uncovered
  Antigravity, Cursor, and Grok routes
- moved all seven guide-map rows from missing to partial

## Validation

- focused Antigravity, Cursor, Grok, and Oh My Pi validation — 138 tests passed
- `effigy check:examples` — passed
- `effigy qa:docs` — passed
- `cargo fmt --all -- --check` — passed
- `git diff --check` — passed

No live or authenticated provider work ran.

## Next Move

Execute card 120: deepen every installed and attached harness route guide
against the Contract 052 route checklist.
