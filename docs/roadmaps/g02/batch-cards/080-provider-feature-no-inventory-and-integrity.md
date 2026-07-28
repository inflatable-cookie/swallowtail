# 080 Provider Feature `No` Inventory And Integrity

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../025-provider-feature-matrix-no-closure-programme.md`

## Objective

Inventory and classify every remaining feature-matrix `No` before selecting
another implementation lane.

## Governing Refs

- provider solution feature matrix
- provider route matrix
- Research 003 and 042-046
- Contracts 014-039

## Scope

1. Parse the CSV with a real CSV parser; do not split quoted rows on commas.
2. Record exact `No` counts by feature column and exact solution cells.
3. Cross-check each cell against realized driver roles, prepared facades,
   examples, and deterministic conformance.
4. Correct only false negatives already proven by production code.
5. For plausible remaining capabilities, refresh current official provider or
   maintained-project evidence.
6. Classify each `No` as:
   - realized matrix error
   - ready under an existing contract
   - missing shared contract or currentness evidence
   - upstream unsupported
   - operation-shape not applicable
   - separate route or solution required
7. Rank feature families by consumer value, number of honest conversions, and
   architectural information.
8. Publish Research 047 and tighten cards 081-083 around the first selected
   family.

## Acceptance Criteria

- [x] every current `No` cell appears exactly once in the audit
- [x] CSV-aware counts are machine-checked
- [x] realized code and matrix claims agree
- [x] current upstream evidence supports every capability recommendation
- [x] `No`, `Not applicable`, and separate-route requirements stay distinct
- [x] one first feature family is recommended with explicit tradeoffs
- [x] unresolved product-policy choices stop for operator input

## Validation

- matrix integrity selector
- research and docs links
- any affected prepared facade and example checks
- `git diff --check`

## Stop Conditions

- the matrix lacks enough semantic precision to classify a feature
- a recommendation depends on private or unauthorized provider access
- a capability would be borrowed from another solution
- the first family would encode unsettled consumer product policy

## Auto-Continuation

Continue to card 081 only when its selected feature family and governing
contract work are unambiguous. Otherwise stop for operator input.

## Outcome

Research 047 records the exact CSV-aware audit.

- initial `No` cells: 458
- corrected serving-only cells: 26 changed to `Not applicable`
- current unique `No` cells: 432
- current `Not applicable` cells in the audited span: 29
- applicable `streaming_events` and `cancellation_or_interruption` gaps: zero

The route-matrix gate fixes every count and classifies every current cell
exactly once. Usage evidence is the first family: Claude Agent ACP, Pi RPC,
and OpenCode are ready under existing runtime types; both selected Kimi Code
surfaces remain unsupported.

No operator product-policy decision is required.
