# 002 Boundary Classification And Operator Decision Packet

Status: completed
Owner: Tom
Created: 2026-08-28
Milestone: `../001-harness-skill-and-watcher-surface-inventory.md`
Depends on: card 001; promoted Research 255

## Goal

Classify the evidence and present only the product decisions that can change a
portable architecture or proof route.

## Scope

1. Classify each surface as portable candidate, provider-local capability,
   host-owned mechanism, consumer-owned projection, unsafe, or unavailable.
2. Resolve what existing Contracts 013, 017, 023, 041, and 044 already govern.
3. Prepare the operator choices for discovery scope, stop authority, output
   exposure, and active-watcher turn completion.
4. Identify whether skill discovery and watcher enforcement need separate
   specs, contracts, and proof routes.

## Output

Update the promoted triage note with one bounded decision packet. Keep evidence,
recommendations, and operator choices separate. Do not create an active spec,
edit architecture or contracts, or ready card 003 before the operator answers.

## Outcome

Research 255 is classified in the promoted triage note. The packet separates
portable candidates, provider-local evidence, host-owned enforcement,
consumer-owned projection, unsafe behavior, and unavailable claims. It records
the exact coverage and remaining gaps in Contracts 013, 017, 023, 041, and
044, recommends separate skill-discovery and watcher-enforcement tracks, and
returns four bounded choices to the operator. No product choice, architecture,
contract, public type, or proof route was selected.

## Acceptance Criteria

- [x] evidence and recommendation remain separate
- [x] privacy and authority boundaries are explicit
- [x] no architecture or public type is promoted before operator decisions
- [x] card 003 remains planned until those decisions are recorded

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Stop Conditions

- Research 255 cannot support a recommendation without provider work or an
  ambient host scan
- one choice would silently combine skill visibility, process authority, and
  consumer presentation
- existing contracts settle less of the boundary than the packet claims
- a recommendation would preselect a public type or proof route

## Auto-Continuation

No. Return the decision packet to the operator.
