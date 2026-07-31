# 001 Installed Harness And Protocol Maintenance Inventory

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../001-installed-harness-and-protocol-currentness-baseline.md`

## Objective

Create one canonical maintenance inventory for installed harness routes and
their shared protocol dependencies before browsing current releases or
selecting implementation.

## Governing Refs

- Vision 001
- Contracts 005, 011, 029, 036, 037, and 044
- provider route and solution feature matrices
- adapter compatibility descriptors and deterministic corpora
- g02 range-maintenance research and release records

## Scope

1. Enumerate production routes whose interface is supplied by an installed
   agent harness.
2. Enumerate shared protocol revisions those routes depend on, including ACP.
3. For each route record:
   - integration family, driver, and transport identity
   - exact qualified baseline and latest point
   - intermediate behavior milestones and exclusions
   - unverified-newer posture
   - evidence date and source owner
   - deterministic corpus and live-probe availability
   - known consumer exposure
4. Identify stale, missing, contradictory, or overly broad currentness claims.
5. Rank the smallest high-risk source set for card 002.
6. Record the result in Research 074 and refresh canonical currentness indexes
   only where repository evidence already settles them.

## Out Of Scope

- external browsing or release-history conclusions
- changing a qualified compatibility range
- production code, fixtures, or live probes
- installing or updating harnesses
- provider authentication, prompt, model, or billing effects
- consumer edits, publication, or candidate mutation

## Acceptance Criteria

- [x] every installed-harness production route has one inventory row
- [x] shared protocol versions are separate from executable versions
- [x] baseline, milestones, exclusions, and upper posture are distinct
- [x] evidence dates and source authority are explicit
- [x] missing evidence remains a gap rather than an inferred claim
- [x] card 002 receives a bounded authoritative source set
- [x] no product-policy or provider-selection decision is required

## Validation

- route and feature-matrix integrity checks
- roadmap, research, release, and descriptor cross-check
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Stop Conditions

- canonical route identity cannot be reconciled across matrices and code
- a range claim lacks any recoverable corpus or authority source
- selecting card 002 scope would establish product policy

## Auto-Continuation

Yes, only into card 002 when the inventory yields a bounded currentness source
set with no unresolved provider-policy choice.

## Evidence

- Research 074 inventories 13 route ids across ten harness solutions
- the exact route, lifecycle, feature, and activity matrices reconcile
- every ordered claim retains visible unverified-newer execution
- shared ACP wire/schema axes remain separate from harness executable claims
- deterministic corpus and repeatable live-probe posture is recorded per
  route family
- Codex and Soundcheck/Nucleus exposure is separated from package-only routes
- card 002 is bounded to Codex, ACP/Claude Agent, Gemini, Pi, and Qwen, plus a
  narrow OpenCode live-selector classification
- the stale post-elicitation matrix totals were corrected without changing a
  feature cell
- `effigy qa:routes`, docs QA, Northstar QA, and diff checks pass
