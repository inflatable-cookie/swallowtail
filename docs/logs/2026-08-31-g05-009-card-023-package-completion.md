# 2026-08-31 g05.009 Card 023 Package Completion

Status: complete; PR 133 merged at `58be7122`
Owner: Tom
Date: 2026-08-31

## Outcome

Card 023 completed candidate A of the Contract 061 Batch 9.4 package
partition. PR 133 merged exact reviewed head `fbb4b118` through merge commit
`58be7122`.

`CodexPreparedExec` now contributes the exact prepared truth admitted for all
35 `codex.exec` census rows, with 15 emitted and 20 withheld dispositions.
`OpenAiPreparedBackgroundRun` contributes the exact prepared truth admitted
for all 24 `openai.background` rows, with 22 emitted and 2 withheld
dispositions. Both ledgers prove the exact `(route_id, operation_shape,
semantic_id)` tuples independently and build no out-of-tranche exception list.

## Review

The first exact-head review found three execution misses: tuple identity was
reduced to semantic ID, cross-access counterexamples were absent, and prepared
activity descriptors incorrectly claimed observed state. The worker repaired
all three on the same PR. Exact head `fbb4b118` proves full tuple identity,
mixed-access rejection plus preparation-time fail-closed behavior, and
descriptor-only activity state while retaining the post-open lifecycle and
observation-only posture.

Bounded namespaced identities remain the Contract 061 extension path for
route-local semantics. `codex.exec` `control.maximum-output-tokens` remains
withheld because the named prepared facade carries no such value. Its census
source mismatch is preserved as an open planning note rather than widened into
this implementation.

## Validation

- exact CSV comparison: 35 `codex.exec` tuples and 24 `openai.background`
  tuples
- 313 focused package tests
- extracted-package verification for both adapters
- semantic API, routes, docs, and Northstar QA
- format and `git diff --check`
- god-files unchanged at 391 total: 342 warning, 42 high, 7 critical
- all five GitHub checks green on exact head `fbb4b118`

No live probe or provider contact occurred.

## Planning State

- Card 023 and candidate A are complete.
- The Batch 9.4 package-expansion note remains active planning evidence for
  candidates B-L.
- The Codex Exec output-token census-source note remains open and is an input
  to the next checkpoint.
- Candidates B-L remain unnumbered and unpromoted. Batch 9.5 remains
  uncompiled.
- PR 130 remains extracurricular and merge-withheld under its separate
  Contract 029 review.

## Next Move

Run a planning-only Batch 9.4 reassessment against current `main`. Resolve the
open Codex census-source question, compare candidates B-L under the existing
whole-candidate rubric, and promote at most one candidate only if every
facade, ledger, withholding, validation, and stop boundary is closed.

## Authority

- [PR 133](https://github.com/inflatable-cookie/swallowtail/pull/133)
- [exact-head review](https://github.com/inflatable-cookie/swallowtail/pull/133#issuecomment-5479727867)
- [card 023](../roadmaps/g05/batch-cards/023-contract-061-codex-openai-package-completion.md)
- [g05.009](../roadmaps/g05/009-contract-061-consumer-projection-realization.md)
- [Batch 9.4 package expansion](../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
