# Harness Usage-Evidence Implementation

Date: 2026-07-28
Card: `../roadmaps/g02/batch-cards/082-first-feature-family-implementation-tranche.md`

## Outcome

Claude Agent ACP, Pi RPC, and OpenCode now emit one cumulative typed
`ProviderObservation::Usage` before terminal settlement.

- Claude uses prompt-response totals and keeps ACP `usage_update` as context
  occupancy.
- Pi sums disjoint assistant `message_end` records once.
- OpenCode sums disjoint `step-finish` records once and rejects repeated part
  identifiers.
- `TokenUsage` preserves optional reasoning tokens independently.

Malformed, missing, negative, fractional, duplicate, and overflowing evidence
fails closed where the qualified wire shape can express it.

## Matrix

Exactly three usage cells changed from `No` to `Yes`. The current 22-solution
matrix now has:

- 19 usage `Yes`
- two Kimi usage `No`
- one serving-only `Not applicable`
- 451 total `No` cells across the audited feature columns
- 29 total `Not applicable` cells

The total differs from Research 047 because Claude Code headless was added
after that historical inventory.

## Validation

- runtime: 68 tests passed
- Claude Agent: 45 tests passed
- Pi: 33 tests passed
- OpenCode: 62 tests passed
- focused Clippy passed
- all package examples compiled
- route and matrix gate passed
- all 23 dirty-snapshot local packages assembled and the extracted workspace
  passed check, no-run, structured-suite, and Kimi package verification
- workspace: 935 tests passed, four skipped

Card 083 closes the usage tranche. Card 084 owns the 48-cell
generation-control audit.
