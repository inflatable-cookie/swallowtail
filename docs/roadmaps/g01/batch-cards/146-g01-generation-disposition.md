# 146 g01 Generation Disposition

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../049-generation-disposition-checkpoint.md`

## Objective

Assess g01 at 49 roadmaps and recommend closure, one final bounded roadmap, or
an explicit operator gate without creating g02 or selecting provider policy by
default.

## Governing Refs

- vision 001
- system architecture and repository authority map
- generation index and long-term plan
- roadmaps g01.047-049
- Research 030-032
- Specs README and provisional Spec 003
- Contracts 001, 004-007, and 029

## Scope

1. Inventory every g01 roadmap and batch card by completed, active, held,
   superseded, and planned status.
2. Identify any unresolved current-generation commitments in contracts,
   provisional specs, research promotions, logs, and front-door prose.
3. Preserve Grok cards 138-141 and their account/authentication gate unless
   evidence supports an explicit different disposition.
4. Compare:
   - closing g01 at roadmap 049
   - using roadmap 050 for one coherent existing commitment
   - pausing for operator direction
5. Recommend a generation disposition and exact next task.
6. Compile no provider implementation lane during this checkpoint.

## Acceptance Criteria

- [x] every non-completed g01 surface has an explicit disposition
- [x] held Grok evidence remains recoverable and truthful
- [x] the 30–50 generation rule drives the recommendation
- [x] no new provider or transport policy is invented
- [x] g02 is not created without an explicit settled direction
- [x] one sole next task or operator gate remains

## Validation

- roadmap/spec/log status audit
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- closing or extending g01 would hide unfinished authority work
- the next-generation focus would establish unsettled product policy
- held Grok work cannot be represented honestly across the boundary
- repository evidence does not distinguish closure from extension

## Auto-Continuation

No. Return any generation or next-programme policy choice to the operator.

## Inventory

- 49 g01 roadmaps: 48 completed, one on hold
- 146 batch cards: 142 completed, four on hold
- 35 active contracts with no unpromoted current-generation planning gap
- 32 promoted research records
- Spec 002 promoted
- stale Spec 001 archived after all durable outcomes were realized
- Spec 003 remains provisional and scoped only to held Grok delegated
  authentication

Roadmap 047 and cards 138-141 are the only unfinished g01 implementation
surfaces. Their exact artifact corpus is complete, but no Grok release is
qualified. Card 138 remains blocked on independently provisioned subscription
state or matching maintained documentation. Cards 139-141 remain held behind
it.

## Disposition Comparison

### Close At 049

Preferred. g01 is at the upper edge of its normal range. Its foundation,
consumer adoption, execution shapes, provider breadth, shared transports,
compatibility policy, and representative production proofs are realized.
Closing now makes the next programme choice explicit.

Before g01 is marked complete, roadmap 047 and cards 138-141 must be rehomed
together as held work. Spec 003 remains global provisional evidence and must
not govern another provider.

### Use Roadmap 050

Rejected. The only coherent unfinished commitment is Grok, and it cannot
advance without external account evidence. A new roadmap would duplicate the
existing hold or invent another provider lane merely to fill capacity.

### Remain Paused

Safe fallback if the operator does not yet want to choose a next-generation
programme. g01 stays active with no ready card and the Grok lane held.

## Recommendation

Approve g01 closure at roadmap 049 and choose the g02 programme. Compile g02
only after that direction is explicit, rehome the held Grok lane unchanged
during the boundary transition, then mark g01 complete.

No authority surface currently settles whether g02 should prioritize release
discipline and API stabilization, continued integration breadth, or another
programme. That choice remains with the operator.

## Operator Decision

The operator approved g01 closure and selected API stabilization, release
discipline, packaging, and consumer upgrade support as the primary g02
programme. Evidence-led provider breadth remains secondary.

Roadmap 047 moved to the shared backlog. Cards 138-141 remain with g01 as
backlog evidence. g02 begins at roadmap 001 with a research-and-contract gate;
no release mutation is authorized.

## Evidence

- status audit: 49 roadmaps, 146 cards, 35 contracts, 32 research records,
  three specs
- all non-completed roadmap and card surfaces now state their hold or rehome
  disposition
- no roadmap 050 or g02 surface created
- no provider, transport, protocol, host, or consumer implementation changed
- `effigy qa:docs` — passed
- `effigy qa:northstar` — passed
- `effigy doctor` — unchanged inherited 19 findings: 12 warnings, seven errors
- `git diff --check` — passed
