# 081 Harness Usage-Evidence Currentness And Contract

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../025-provider-feature-matrix-no-closure-programme.md`
Depends on: card 080

## Objective

Freeze exact usage-evidence behavior for Claude Agent ACP, Pi RPC, and
OpenCode, then promote the one common aggregation rule needed by card 082.

## Governing Refs

- Research 047
- Contracts 011, 012, 014, and 029
- Claude Agent ACP `0.53.0..=0.61.0`, excluding `0.58.0`
- Pi RPC exact qualified `0.80.10`
- OpenCode HTTP `1.14.48..=1.18.4`

## Scope

1. Freeze deterministic records for:
   - Claude prompt-response input, output, cache-read, cache-write, and total
     usage
   - Pi assistant `message_end` usage across a multi-step tool turn
   - OpenCode `step-finish` usage across a multi-step agent turn
2. Check usage field stability at every existing Claude and OpenCode behavior
   boundary and the exact Pi point.
3. Define one terminal cumulative operation observation:
   - sum only disjoint provider components
   - replace cumulative snapshots
   - do not mix context occupancy, cost, rate, quota, or token limits
   - fail closed on malformed values, overflow, ambiguous semantics, or
     missing required terminal evidence
4. Promote the rule into Contract 011 and architecture.
5. Keep both Kimi solution cells `No`; add no Kimi fixture or alternate route.
6. Tighten card 082 with exact fixture and prepared-facade acceptance.

## Out Of Scope

- billed-cost evidence
- session context occupancy
- Pi range expansion above `0.80.10`
- Kimi web or adjacent SDK routes
- live provider access
- consumer edits

## Acceptance Criteria

- [x] card 080 names one unambiguous feature family
- [x] exact provider records are frozen without private payloads
- [x] Claude, Pi, and OpenCode version claims are bounded
- [x] cumulative replacement and disjoint aggregation are testable
- [x] Kimi absence remains route-specific
- [x] card 082 can execute without new product policy

## Validation

- focused protocol fixture tests
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Stop Conditions

- a selected record cannot distinguish disjoint from cumulative usage
- usage would need to be inferred from text or cost
- a version milestone changes required field semantics
- implementation would require live authentication

## Auto-Continuation

Continue to card 082 when contract and fixture validation pass.

## Outcome

Research 048 freezes:

- cumulative Claude prompt usage across all nine published qualified adapter
  points
- disjoint Pi assistant-message usage at exact `0.80.10`
- one identical required OpenCode step-finish usage schema across all 45
  qualified releases

Contract 011 and architecture now define cumulative replacement, disjoint
aggregation, failure on ambiguity or overflow, and separation from context,
cost, rate, quota, and token limits.

Three focused corpus tests pass. Card 082 needs one optional
`TokenUsage::reasoning_tokens` dimension for OpenCode before implementing the
three adapters.
