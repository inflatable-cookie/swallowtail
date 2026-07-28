# Provider Feature `No` Inventory And Usage Selection

Date: 2026-07-28
Card: `../roadmaps/g02/batch-cards/080-provider-feature-no-inventory-and-integrity.md`

## Outcome

Card 080 completes the first exact audit of every feature-matrix `No`.

- CSV-aware initial count: 458
- serving-only llama.cpp corrections: 26 `No` to `Not applicable`
- corrected count: 432 unique `No` cells
- audited `Not applicable` count: 29
- applicable streaming-event gaps: zero
- applicable cancellation/interruption gaps: zero

The route-matrix selector now fixes every count, visits every current cell
once, rejects duplicates and unlisted drift, and checks the first evidence
classifications.

## Selection

Usage evidence is first.

- Claude Agent ACP returns aggregate prompt usage across the qualified
  `0.53.0..=0.61.0` range.
- Pi RPC `0.80.10` returns disjoint assistant-message usage.
- OpenCode `1.14.48..=1.18.4` returns disjoint step-finish usage.
- Kimi Code installed and local-server `0.29.2` surfaces expose no qualified
  usage record and remain `No`.

The implementation will emit one terminal cumulative operation observation.
It will not mix cost, context occupancy, rate, quota, or token limits.

## Current State

Research 047 is promoted. Card 081 is ready for exact records and the
cumulative aggregation contract clarification. Cards 082-083 retain
implementation, package proof, and selection of the next matrix family.

