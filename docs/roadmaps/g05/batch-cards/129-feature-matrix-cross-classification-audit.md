# 129 Feature Matrix Cross Classification Audit

Status: ready
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-07
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: the operator's 2026-09-07 ruling that an unexplained cross is a work item, not a result; `docs/guides/provider-solution-feature-matrix.csv`; Contract 061

## Rule (operator, 2026-09-07)

Every unavailable cell in the feature matrix is exactly one of:

- **provider limitation**: the provider or harness cannot do it on the
  qualified version, with a citation to the frozen evidence (research doc,
  corpus line, vendor doc); or
- **producer gap**: the provider can do it and Swallowtail has not built the
  seam, with the card that builds it.

A cross with neither is a defect in the matrix. "Withheld" is not a third
kind: it is a producer gap with a reason.

## Scope

1. Add two columns to the matrix CSV and its rendered guide: `cross_kind`
   (`provider_limitation` | `producer_gap`) and `cross_ref` (evidence path or
   card id), populated for every unavailable cell on every route, starting
   with the three Bovine routes across MCP/tools, permissions, skills, model
   inventory/selection, cancellation/reconciliation, packaging, and registered
   tools; then the remaining 46 routes.
2. Add the missing capability columns the 2026-09-07 matrix exposed:
   registered tools, client MCP servers, selected-skill bundle, persistent
   permission grants, pre-session model catalogue.
3. `scripts/check-provider-route-matrix.sh`: fail when an unavailable cell
   lacks a kind and reference, or when a `producer_gap` references a card
   that does not exist or is complete.
4. Output a ranked backlog of every `producer_gap` on a route a consumer
   currently pins (Bovine: Claude SDK, Codex app-server, Grok ACP; plus the
   Nucleus routes), for Chatterbox to promote in consumer priority order.

## Out Of Scope

Building any seam; changing any claim; live probes.

## Acceptance Criteria

- [ ] no unavailable cell without kind and reference, enforced by the check
- [ ] every `provider_limitation` cites frozen evidence that says so
- [ ] every `producer_gap` names an existing, non-complete card, or the audit
      creates a stub card the backlog lists
- [ ] the ranked producer-gap backlog for pinned routes is in the card Result

## Validation

- `effigy qa:routes`; `effigy qa:docs`; `git diff --check`

## Review Oracle

Invariant: reading a cross tells you who owes the work. Smallest
counterexample: a cross whose reference is a card marked complete.

## Auto-Continuation

No. Stop for exact-head review; Chatterbox promotes the backlog.
