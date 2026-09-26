# Swallowtail — current state

Swallowtail is a pre-1.0 Rust library for driving AI model providers and agent
harnesses through exact, testable routes. The current source tag is `v0.5.1`
(40 packages, 51 production routes); see [releases](releases/README.md).

What is true now:

- Every production route family is qualified against its official release
  through the Contract 029 currentness procedure.
- Contract 063 admits a consumer-supplied streamable-HTTP MCP placement.
  `opencode.acp` is live-proven honouring it; `claude-agent.acp` on exact
  `0.79.0` spent its one live attempt with typed `cleanup_failed` (Research
  352); `copilot-cli.acp`, `gemini-cli.acp`, `goose.acp` and `kiro.acp` emit
  the entry with live honouring unproven (Research 351).
- Unavailable feature-matrix cells are provider limitations with frozen
  evidence or producer gaps owned by a [plan](plan.md) item.

When sources disagree: contracts govern behaviour, then architecture, then
vision. Specs and research stay provisional until promoted.

## By topic

- Vision: [knowledge/vision.md](knowledge/vision.md)
- Architecture: [knowledge/architecture/](knowledge/architecture/README.md)
- Contracts: [knowledge/contracts/](knowledge/contracts/README.md)
- All knowledge, one owner per topic: [knowledge/README.md](knowledge/README.md)
- Consumer guides and matrices: [guides/](guides/README.md)
- Retained research evidence: [research/](research/README.md)
- Release notes: [releases/](releases/README.md)

## What's next

See [plan.md](plan.md). Open questions: [knowledge/questions.md](knowledge/questions.md).
