# 021 Consumer Route Feature And Control Projection Contract

Status: ready
Owner: Tom
Created: 2026-08-31
Milestone: `../008-consumer-route-feature-and-control-projection.md`
Depends on: operator-approved Spec 012 contract direction

## Goal

Promote Spec 012 into one dedicated composing Contract 061 and leave every
implementation decision for post-contract reassessment.

## Scope

1. Create
   `docs/contracts/061-consumer-route-feature-and-control-projection.md` from
   Spec 012.
2. Govern selection-summary, session-start, and active-session projection
   views; exact source, snapshot, applicability, lifecycle, value-domain, and
   state truth; bounded namespaced extensions; bounded safe reasons; immutable
   replacement; and fail-closed composition.
3. Preserve Contracts 037, 047, and 057 without edits or authority changes.
4. Add Contract 061 to the contract front door, index, and summaries.
5. Archive Spec 012 after the contract is active and reconcile the spec,
   roadmap, batch-card, log, triage, and sole Next Task surfaces.
6. Close g05.008 with implementation unplanned and return to orchestrator
   reassessment.

## Out Of Scope

- Rust, manifests, public API baselines, crate/module placement, or tests
- amendments to Contracts 037, 047, or 057
- an exhaustive availability-reason enum
- route enumeration, adapter registry, provider downcasts, or consumer UI
- provider contact, live probes, version/currentness work, or route claims
- PR 127, watcher retries, Darwin, fallback, or another Claude turn
- compiling an implementation roadmap or card

## Acceptance Criteria

- [ ] Contract 061 owns one descriptive composing surface and does not absorb
      preparation, selection, or pre-session lifecycle authority
- [ ] exact configured-instance revision, route, model when applicable,
      operation shape, evidence identity, applicability, and lifecycle survive
      projection
- [ ] selection-summary, session-start, per-turn, post-open observation, and
      exact negotiated state remain distinct
- [ ] route-wide, matrix, catalogue, prepared, and negotiated evidence cannot
      silently widen one another
- [ ] source dimensions and bounded safe reasons remain visible without a
      claimed exhaustive reason taxonomy
- [ ] snapshot replacement and cross-boundary rejection are explicit
- [ ] every Spec 012 review-oracle counterexample names its rejection or
      withholding point
- [ ] Spec 012 is archived and all required indexes agree
- [ ] no production code, existing contract amendment, architecture
      realization claim, or implementation card is added

## Review Oracle

Invariant: projection preserves exact descriptive truth without creating
execution or mutation authority.

Counterexamples and required proof:

- route capability plus incompatible model/prepared evidence — reject or
  withhold the usable row before projection publication
- valid descriptor plus stale configured-instance revision — reject the mixed
  snapshot
- post-open option observation presented as selectable or acknowledged — keep
  observation-only without an exact mutation/acknowledgement source
- absent source truth converted to an exhaustive availability reason — retain
  unknown/absence plus only a bounded safe reason

Review must map each counterexample to a normative Contract 061 clause and its
acceptance section.

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- changed-path proof contains documentation only

## Auto-Continuation

No. Return to the orchestrator for post-contract implementation readiness
reassessment.

## Stop Conditions

- Stop if contract drafting exposes a new product, public-API, persistence,
  security, or lifecycle decision not settled by Spec 012.
- Stop if any existing contract must change.
- Stop if implementation scope enters the diff.
- Stop if the review oracle cannot be stated without inventing source truth.

## Evidence

- reviewed 767-row census across all 48 production routes
- operator decisions recorded on 2026-08-31
- Spec 012 settled composing-contract and deferred-taxonomy boundary
