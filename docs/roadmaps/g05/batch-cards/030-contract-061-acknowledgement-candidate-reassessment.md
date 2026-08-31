# 030 Contract 061 Acknowledgement Candidate Reassessment

Status: ready
Owner: Tom
Created: 2026-08-31
Updated: 2026-08-31
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: completed card 024; Batch 9.4 lifecycle-priority sequence

## Goal

Audit candidates D, F, and G against current `main`. Promote at most one exact
active-session-acknowledgement package tranche, or record an honest stop if no
candidate passes the existing Batch 9.4 rubric.

## Scope

1. Reconcile the exact census sets for candidate D (53 rows), F (89 rows), and
   G (48 rows), including candidate G's two no-control audits.
2. Name every prepared facade, active-observation facade, source identity, and
   acknowledgement state needed by each complete adapter-package remainder.
3. Trace each exact active-session acknowledgement from provider validation
   through retained adapter evidence. Prepared or documentation-only truth is
   not an acknowledgement source.
4. Identify construction-time withholding rules for catalogue-only,
   incompatible-operation, documentation-only, and unobserved rows.
5. Test each candidate against the Batch 9.4 promotion rubric: exact ledgers,
   fail-closed source/applicability assembly, lifecycle and authority
   distinctions, package extraction, public API stability, and the four-package
   focused-validation maximum.
6. Update the Batch 9.4 checkpoint with a current-main disposition for D, F,
   and G. Do not weaken a blocker to force a selection.
7. If exactly one candidate passes, compile one numbered implementation card
   with exact rows, facades, sources, counterexamples, validation, stops, and
   one reviewable PR boundary. Otherwise leave an explicit evidence or
   mechanism stop and name the next planning move.
8. Reconcile the milestone, g05 front door, indexes, closeout log, and sole
   Next Task. Stop for orchestrator exact-head review.

## Out Of Scope

- Rust, manifest, release-baseline, contract, architecture, or census edits
- provider contact, live probes, compatibility/currentness, watcher, skill
  inventory, papercut, or generation-closeout work
- implementing the selected acknowledgement candidate
- promoting per-turn candidates B/K/L, breadth candidates C/E/I/J, or more
  than one acknowledgement candidate
- compiling Batch 9.5

## Acceptance Criteria

- [ ] D, F, and G totals reconcile to 53, 89, and 48 exact rows with no filter
      or exception list
- [ ] every route's prepared and active-observation facade and source identity
      are named, including the exact acknowledgement transition retained
- [ ] documentation, prepared success, and session existence cannot masquerade
      as active-session acknowledgement
- [ ] construction-time withholding and negative no-control coverage are
      explicit for every affected route
- [ ] each candidate has a concrete deterministic ledger and mixed-assembly
      proof plan that preserves lifecycle, authority, source, route,
      operation, instance, access, and acknowledgement truth
- [ ] the selected candidate, if any, needs no new runtime/testkit/core public
      type, composer rule, fixed maximum, callback, registry, provider payload,
      or contract amendment
- [ ] exactly one implementation card is ready, or the checkpoint records why
      none can be promoted on current `main`
- [ ] shared planning surfaces and the sole Next Task agree

## Review Oracle

- a candidate promoted because the matrix documents acknowledgement, without
  retained active-observation evidence — fail
- an acknowledgement row emitted from prepared-operation success or session
  existence — fail
- a current/pending/rejected/effective state flattened into one supported
  boolean — fail
- a partial adapter package or route omitted to fit the tranche — fail
- one route's active source, acknowledgement state, or applicability borrowed
  by another route under a matching source ID — fail
- a candidate selected despite needing a new shared public or contract
  decision — stop; do not compile its implementation card

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

No Rust validation, live probe, or provider contact belongs to this planning
card.

## Auto-Continuation

No. Return one reviewable planning PR. The orchestrator reviews the exact
candidate disposition and any compiled implementation card before dispatch.

## Evidence

- [Batch 9.4 package expansion](../../../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [Contract 061](../../../contracts/061-consumer-route-feature-and-control-projection.md)
- [reviewed census](../../../triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [completed card 024](024-contract-061-deepagents-kiro-qoder-zcode-package-completion.md)
