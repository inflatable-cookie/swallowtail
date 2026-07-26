# Soundcheck Proof Gate Audit

Date: 2026-07-26
Card: `../roadmaps/g02/batch-cards/043-soundcheck-secondary-application-proof.md`

## Outcome

Card 043 is paused. The read-only gate audit made no Soundcheck edit and no
provider call.

Soundcheck is at `656555e817782483a66be5566e759d1a789fea87` with a large
uncommitted M11 tranche. Its sole next task is active card 088, which changes
the exact assistant evidence, prompt, screenshot, validation, and proposal
path this proof must exercise. Cards 089 and 090 remain in the same runway.
The consumer is moving and its current roadmap does not permit the separate
proof.

## Existing Fit

The current product path already preserves the required ownership split:

- prepared Codex catalogue and structured exec remain separate
- default model is `gpt-5.4-mini` with low reasoning
- OAuth subscription access is explicit
- cancellation is request-scoped
- schemas, screenshots, external search, deadlines, and cleanup pass through
  Swallowtail
- prompts, validation, repair, ranking, review, and application remain in
  Soundcheck
- `SOUNDCHECK_LIBRARY_DB_PATH` isolates the library and adjacent screenshots

## Gaps

- every primary research turn enables bounded external search; the planned
  2-search split was false
- product deadlines are fixed at 15 minutes for research and 3 minutes for
  secondary turns; no proof-only bounded deadline exists
- no deterministic assistant-data and screenshot seed exists
- no sanitized ledger reconciles primary and secondary provider attempts
  across restarts
- secondary repair, ranking, or companion turns can make workflow count differ
  from provider-attempt count

## Corrected Envelope

The proposed workload remains 16 primary product workflows across 4 launches:

- 8 text-only ordinary
- 4 screenshot-backed ordinary
- 2 ordinary with required search-progress observation
- 1 cancellation
- 1 controlled deadline

All 16 primary research attempts require external-search authority. Up to 4
Soundcheck-owned repair, ranking, or companion attempts may occur. The hard
ceilings remain 20 provider attempts, 4 launches, serial execution, and
2 hours.

No live user library, screenshot, plugin state, or DAW state is in scope.
Fixture product data and images must be approved, non-sensitive, and confined
to the fresh proof root.

## Next

Wait for Soundcheck cards 088-090 to reach one clean checkpoint. Then freeze
the exact source and runtime tuple and obtain consumer authority for one
proof-readiness batch: deterministic seed, bounded deadline control, sanitized
attempt ledger, and teardown. Ask for the corrected live envelope only after
that deterministic support passes.
