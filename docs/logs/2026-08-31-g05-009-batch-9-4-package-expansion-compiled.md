# 2026-08-31 g05.009 Batch 9.4 Package Expansion Compiled

Status: complete; 716-row partition compiled; card 023 ready
Owner: Tom
Date: 2026-08-31

## Outcome

The 716 census rows left after PR 131 are assigned exactly once to 12 bounded
candidate batches. Every candidate owns the complete remaining contribution
of one to four adapter packages. The partition keeps all nine no-control route
audits, eight remaining per-turn rows, and three remaining exact
active-session acknowledgement rows in their original route and lifecycle
boundaries.

Candidate A passes the readiness rubric and is promoted as card 023. It closes
the two package remainders already begun by PR 131: 35 `codex.exec` rows and 24
`openai.background` rows. Candidates B-L remain planning rows without card
numbers or execution authority until their exact façade and ledger audits pass
the same rubric.

## Current State

- g05.009 remains the one ready milestone.
- Card 022 is complete through PR 131 at `fdd2b018`.
- Card 023 is the sole ready card and is provider-free.
- Contract 061 and the Batch 9.1 runtime public baseline are unchanged.
- Batch 9.5, provider contact, PR 127, PR 130, blocked watcher/skill lanes, and
  generation closeout remain outside the active task.

## Next Move

Implement card 023 as one reviewable two-package PR, then stop for exact-head
orchestrator review. Do not promote another candidate from the planning table
until that checkpoint closes and its own readiness audit is compiled.

## Validation

- census partition assertion: 716 rows, 46 route IDs, 31 adapter packages,
  nine no-control audits, and eight remaining per-turn rows, each assigned
  once
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

The first docs run rejected the generation-index Next Task lead verb
`Execute`. Rewriting it as the concrete `Implement` action cleared the check;
the full rerun passed.

## Authority

- [Batch 9.4 package expansion](../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [g05.009](../roadmaps/g05/009-contract-061-consumer-projection-realization.md)
- [card 023](../roadmaps/g05/batch-cards/023-contract-061-codex-openai-package-completion.md)
- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
