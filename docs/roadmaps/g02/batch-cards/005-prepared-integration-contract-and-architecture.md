# 005 Prepared Integration Contract And Architecture

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../002-prepared-consumer-integration-boundary.md`

## Objective

Promote the accepted prepared-integration shape into durable architecture and
Contract 037 before public API implementation.

## Governing Refs

- Vision 001
- product guardrails and repository authority map
- Research 034
- provisional Spec 005
- Contracts 001-002, 004, 008-010, 013, 029, 032-033, and 036
- roadmap g02.002

## Scope

1. Promote the two-layer low-level/prepared integration model into system
   architecture.
2. Add Contract 037 covering:
   - prepared integration ownership and inspectability
   - plan-derived versus explicit runtime request state
   - access observation or caller-assertion provenance
   - safe preparation stages and diagnostic chaining
   - per-host joined service composition
   - preservation of low-level roles and provider-specific operation shapes
3. Make narrow consistency updates to existing contracts only where required.
4. Archive Spec 005 after all durable decisions are promoted.
5. Rebaseline cards 006-010 from the active contract.
6. Record the frozen `0.1.0` candidate as superseded pending replacement,
   without deleting evidence or mutating external release state.

## Acceptance Criteria

- [x] preparation cannot select provider, model, credential, endpoint, billing,
      writable access, network, search, or tools implicitly
- [x] request plan echoes are derived or explicit, never unrelated defaults
- [x] access provenance and diagnostics are safe and machine-distinct
- [x] local composition has no global executor or detached task
- [x] low-level roles remain supported
- [x] Codex exec and app-server remain separate
- [x] cards 006-007 need no fresh contract decision

## Validation

- architecture and contract boundary audit
- public dependency-direction review
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` delta review
- `git diff --check`

## Evidence Required

- Contract 037 and indexed architecture refs
- Spec 005 promotion/archive record
- rebaselined card statuses and exact continuation pointer
- log naming retained safety rules and candidate hold

## Stop Conditions

- access provenance remains ambiguous
- preparation requires hidden provider or topology routing
- a new crate becomes necessary without package-contract review
- the facade would flatten operation shapes
- the contract cannot keep consumer policy downstream

## Auto-Continuation

Yes, only if card 006 is rebaselined to ready from active Contract 037.
Otherwise stop with the exact unresolved contract point.

## Evidence

- Contract 037 fixes the layered boundary, explicit preparation inputs,
  plan-derived request agreement, observed or caller-asserted access
  provenance, safe failure stages, joined per-host composition, and low-level
  escape hatch.
- System architecture records the contracted layer and current realization
  gap without changing the 23-crate graph.
- Contracts 008-010, 032-033, and 036 carry narrow consistency rules.
- Spec 005 is promoted and archived.
- The first `0.1.0` candidate and handoffs are marked superseded while all
  frozen evidence remains retained.
- Card 006 is ready. Cards 007-010 remain sequenced without a fresh contract
  decision.
- `effigy qa:docs`, `effigy qa:northstar`, and `git diff --check` pass.
  `effigy doctor` remains at the inherited 19 findings.

## Closeout

Completed 2026-07-24. No runtime code, consumer repository, frozen candidate
artifact, registry, tag, push, workflow, or external release state changed.
