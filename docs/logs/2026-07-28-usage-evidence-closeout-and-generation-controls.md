# Usage-Evidence Closeout And Generation Controls

Date: 2026-07-28
Card: `../roadmaps/g02/batch-cards/083-provider-feature-tranche-closeout-and-continuation.md`

## Outcome

The first provider-feature tranche is closed.

- Claude Agent ACP, Pi RPC, and OpenCode now report exact cumulative usage
  through public prepared paths.
- The usage column has 19 `Yes`, two honest Kimi `No`, and one serving-only
  `Not applicable`.
- The current 22-solution matrix has 451 total `No` and 29 total
  `Not applicable` cells.
- Machine checks classify all 451 remaining `No` values without a generic
  unexplained bucket.

The counts differ from historical Research 047 because Claude Code headless
was added after that inventory. The historical record remains unchanged.

## Validation

- 23 dirty-snapshot local package archives assembled
- extracted package workspace check and no-run passed
- packaged structured and Kimi suites passed
- workspace: 935 tests passed, four skipped
- focused Clippy, examples, routes, and matrix checks passed
- the unreleased `0.1.0` public-declaration baseline records the intentional
  additive `TokenUsage` reasoning and aggregation surface

No package was published and no release candidate was created.

## Continuation

Roadmap 026 and cards 084-087 cover the next matrix family:

- 14 output-token-limit `No`
- 14 reasoning-selection `No`
- 20 structured-output `No`

Card 084 audits all 48 cells before contract or implementation selection.
