# 2026-09-01 g05.009 Card 031 Closeout And Lifecycle Reassessment

Status: complete; PR 141 merged; no implementation card ready
Owner: Tom
Date: 2026-09-01
Contracts: 037, 047, 057, 061

## Outcome

Card 031 is complete. PR 141 merged exact reviewed head
`1edc7e73019a450605cb681eb56aeb35ad188557` through
`5d1f173ad0637c16c24f5134ef45dc559f67c61d`.

The three candidate D ledgers reconcile independently:

| Route | Census | Emitted | Withheld |
| --- | ---: | ---: | ---: |
| `claude-agent.acp` | 30 | 29 | 1 |
| `claude-code.headless` | 12 | 11 | 1 |
| `claude-code.response-only` | 11 | 9 | 2 |
| **Total** | **53** | **49** | **4** |

Exact ACP reasoning acknowledgement now survives through the accepted
adapter-owned projected-open result. Prepared and active sources remain
distinct. Malformed, missing, duplicate, unadvertised, unqualified, or
unbounded confirmation publishes no state. The existing and additive open
methods share one private lifecycle.

Only the Claude Agent adapter semantic API baseline changed, with 27 additive
lines. Focused validation passed 188 tests, package and semantic API checks
passed, the god-file scan improved from 391 to 387, and all five CI jobs were
green. No provider contact or live probe occurred.

## Census State

Cards 022-024 and 031 prove 201 of 767 rows across eleven routes. The exact
remainder is 566 rows across 37 route IDs and 24 adapter packages in candidates
B, C, E-G, and I-L. Batch 9.5 remains uncompiled.

## Lifecycle Reassessment

The current-main audit returned to candidates F and G. Neither passes the
Batch 9.4 promotion rubric.

- F remains the larger coupled stop: 89 rows, two packages, four route shapes,
  and three unproved Kimi ACP post-open families — compound reasoning-and-plan
  acknowledgement, negotiated model options, and provider-session catalogue.
- G is the narrower gate: 48 rows across four complete package remainders.
  Its negative no-control rows have a proved pattern, but `cline.acp` discards
  exact Plan confirmation, exposes no negotiated model-option snapshot, and
  has no adapter-owned active-observation projection result.

Card 031's route-local public API grants no authority to Kimi or Cline. No
candidate is promoted and no card 032 is compiled.

## Current State

- g05.009 is `strict-paused`
- no implementation card is ready
- the sole Next Task is the `cline.acp` active-observation operator decision
- acceptance would authorize a later exact API gate and candidate G
  reassessment, not implementation by itself
- candidates B, C, E, F, and I-L remain unchanged
- watcher, skill, currentness, papercut, provider, and Batch 9.5 lanes were not
  touched

## Next Move

Decide whether the Cline adapter may retain exact effective/rejected Plan
acknowledgement plus exact bounded negotiated model options and expose both
through one additive adapter-owned projected-open seam while preserving
`ClinePreparedSession::open_session`.

## Authority

- [completed card 031](../roadmaps/g05/batch-cards/031-contract-061-claude-agent-package-and-acknowledgement.md)
- [g05.009](../roadmaps/g05/009-contract-061-consumer-projection-realization.md)
- [Batch 9.4 checkpoint](../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [Cline active-observation gate](../triage/2026-09-01-contract-061-cline-active-observation-public-baseline-gate.md)
- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
