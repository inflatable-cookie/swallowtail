# 2026-08-31 g05.008 Consumer Projection Contract Promotion

Status: complete
Owner: Tom
Date: 2026-08-31

## Decision

Promote Spec 012 into
[Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md).
The contract composes Contracts 037, 047, and 057 into one descriptive
consumer projection of route, model, and operation features and
lifecycle-scoped controls. It amends none of them.

Contract 061 owns:

- selection-summary, session-start, and active-session views over one shared
  semantic vocabulary, with per-turn and post-open observation kept distinct
- exact applicability: configured-instance id and revision, route, model where
  applicable, operation shape, access mode, and resource constraints
- one immutable snapshot bound to those identities plus each source evidence
  identity, replaced rather than mutated, with no universal clock or watcher
- descriptor semantics: value kind, admitted domain or explicit unenumerated
  bound, omission truth, lifecycle, actor posture, and state support
- bounded namespaced provider-native extensions that cannot widen support
- existing source availability dimensions plus bounded safe reasons, with no
  exhaustive portable reason taxonomy
- a descriptive authority boundary: no execution, mutation, acknowledgement,
  routing, default, fallback, or preflight bypass

Provider and solution feature matrices stay documentation cross-checks. The
projection inherits the bounds of the records it composes and introduces no new
numeric cap; a projection-specific bound stays a later planning decision.

## Review Oracle Mapping

Each Spec 012 counterexample has a named fail-closed point in the contract's
`Fail-Closed Composition` table and a matching `Conformance` and `Acceptance`
row:

| Counterexample | Named point | Behavior |
| --- | --- | --- |
| route-wide capability plus incompatible model or prepared evidence | applicability disagreement | reject the row, or publish it without the usable and currently available claim, before publication |
| valid descriptor plus stale instance revision or superseded source | snapshot identity disagreement | reject the whole assembly |
| post-open option list presented as selectable or acknowledged | absent mutation authority | hold at observation-only |
| missing source truth replaced by an exhaustive availability reason | unbounded reason claim | retain unknown or absence plus at most a source-supplied bounded safe reason |

## Current State

- Contract 061 active; contract front door, index, and summaries updated
- Contracts 037, 047, and 057 unchanged; no amendment entry added
- Spec 012 archived as promoted; specs front door and archive index agree
- g05.008 and card 021 complete; implementation unplanned
- documentation-only diff; no Rust, manifest, public API baseline, or
  architecture realization claim

## Next Move

Orchestrator reassessment. No implementation roadmap, card, provider turn, or
public API work is authorized by this promotion.

## Authority

- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
- [Spec 012](../specs/archive/012-consumer-route-feature-and-control-projection.md)
- [g05.008](../roadmaps/g05/008-consumer-route-feature-and-control-projection.md)
- [card 021](../roadmaps/g05/batch-cards/021-consumer-route-feature-and-control-projection-contract.md)
- [Projection triage and census synthesis](../triage/2026-08-30-consumer-route-feature-and-option-projection.md)
