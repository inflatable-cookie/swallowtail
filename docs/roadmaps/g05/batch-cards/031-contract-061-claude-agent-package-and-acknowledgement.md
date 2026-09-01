# 031 Contract 061 Claude Agent Package And Acknowledgement

Status: complete; PR 141 merged at `5d1f173a`
Owner: Tom
Created: 2026-08-31
Updated: 2026-09-01
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: completed card 030; accepted Claude Agent acknowledgement
public-baseline gate

## Goal

Complete candidate D's 53-row Contract 061 package remainder and preserve the
exact `claude-agent.acp` reasoning acknowledgement through one additive
adapter-owned prepared-open result without changing the existing open path.

## Scope

1. Retain the exact Claude Agent ACP reasoning confirmation inside
   `swallowtail-adapter-claude-agent`: absent when reasoning was not requested,
   provider-effective on an exact match, and rejected only for an exact
   well-formed different value admitted by that response's effort options and
   the qualified route's reasoning modes.
2. Add `ClaudeAgentProjectionOpenFuture`,
   `ClaudeAgentProjectionOpenOutcome`, and
   `ClaudeAgentProjectionOpenFailure` with the exact signatures and accessors
   fixed by the
   [public-baseline gate](../../../triage/2026-08-31-contract-061-claude-agent-acknowledgement-public-baseline-gate.md).
3. Add `ClaudeAgentPreparedSession::open_session_with_projection` with distinct
   prepared and active-session source IDs. Keep
   `ClaudeAgentPreparedSession::open_session` public, source-compatible, and
   behaviorally unchanged. Serve both methods from one private low-level open
   lifecycle.
4. Add the established
   `consumer_route_projection_contribution(source_id)` shape to
   `ClaudeAgentPreparedRun`, `ClaudeAgentPreparedSession`,
   `ClaudeAgentPreparedDelete`, `ClaudeCodePreparedRun`, and
   `ClaudeCodeResponsePreparedRun`. Use only exact prepared evidence, requests,
   activity profiles, and adapter-local acknowledgement truth.
5. Disposition exactly 53 census tuples with no filter or exception list: 30
   `claude-agent.acp`, 12 `claude-code.headless`, and 11
   `claude-code.response-only`. Name each tuple once with an emitted or
   construction-time-withheld reason.
6. Keep model-catalogue and documentation-only rows withheld without catalogue
   evidence. Keep lifecycle actions descriptive, activity descriptor-only,
   and prepared reasoning at requested/prepared/pending until the exact open
   acknowledgement exists.
7. Emit `feature.active-session-reasoning-ack` only from the additive
   `claude-agent.acp` session-open result. Matching confirmation carries exact
   provider-effective state; an exact well-formed mismatch returns
   `Rejected` with exact rejected state and no session. Unknown or malformed
   failures return `Runtime` with no contribution.
8. Add deterministic provider-free fixtures for prepared, pending, omitted,
   matching-effective, exact-rejected, malformed, missing, unadvertised,
   duplicate, and source-identity disagreement cases. Prove the existing and
   additive open paths share failure codes and cleanup behavior.
9. Update only the Claude Agent adapter semantic API baseline. Keep
   `swallowtail-runtime`, `swallowtail-testkit`, `swallowtail-core`, Contracts
   037/047/057/061, the census, and route compatibility claims unchanged.
10. Stop after one reviewable one-package PR and return it for orchestrator
    exact-head review.

## Out Of Scope

- `kimi-code.acp`, `cline.acp`, candidates F/G, or any cross-route
  acknowledgement abstraction
- changes to `EffectiveReasoningSetup` or any runtime/testkit/core public API
- negotiated model-option or provider-session-catalogue observation seams
- any census row outside candidate D or any Batch 9.5 work
- a generic acknowledgement payload, callback, adapter registry, runtime route
  enumeration, handle downcast, or composer-side execution
- provider contact, live probes, compatibility/currentness work, watcher or
  skill-discovery work, papercuts, or generation closeout

## Acceptance Criteria

- [x] independent ledgers reconcile exactly to 30, 12, and 11 rows, with each
      `(route_id, operation_shape, semantic_id)` named once and no exception
      list
- [x] every emitted row retains exact source, route, operation, lifecycle,
      value, omission, applicability, access, and evidence truth
- [x] prepared reasoning is requested/prepared/pending only; it never claims
      provider-effective or rejected state
- [x] an exact matching ACP effort acknowledgement returns an open session and
      a contribution carrying the exact provider-effective value
- [x] only an exact, well-formed different effort admitted by both the response
      and qualified route returns `Rejected` with its exact value; missing,
      malformed, duplicate,
      unadvertised, unbounded, transport, setup, or cleanup failures carry no
      rejected contribution
- [x] omitted reasoning produces no acknowledgement row and no unused
      active-observation source
- [x] prepared and active-session source IDs are distinct; prepared rows use
      `AdapterContribution` and acknowledged rows use
      `ActiveSessionObservation`
- [x] existing `open_session` callers retain the same signature, handle,
      failure code, and cleanup outcome; both public methods share one private
      lifecycle
- [x] activity remains descriptor-only; model-catalogue, documentation-only,
      incompatible-operation, and unproved lifecycle rows are withheld at
      construction
- [x] matching-source cross-route, cross-operation, cross-access,
      cross-instance, and stale-revision assembly fail closed
- [x] only the Claude Agent adapter semantic API baseline changes; shared
      public APIs, contracts, census, provider claims, and compatibility claims
      remain unchanged
- [x] touched source stays below configured god-file thresholds and the scan
      does not exceed the accepted repository baseline

## Review Oracle

Invariant: one exact Claude Agent interactive-session acknowledgement is the
only new active truth. Prepared success, another Claude route, or an ambiguous
provider value cannot substitute for it.

Counterexamples and required proof:

- mark a prepared reasoning control provider-effective before open — fail; no
  active observation exists yet
- return rejected state for a value absent from the confirmation's advertised
  effort options or the qualified route's reasoning modes — fail as `Runtime`
  with no contribution
- return a rejected contribution for missing, malformed, duplicate, or
  unbounded confirmation — fail; no exact rejected value exists
- reuse the prepared source ID as the active source — fail before provider
  work
- attach the active source to prepared selection or session-start rows — fail
  contribution admission
- preserve an acknowledgement row when reasoning was omitted — fail; both the
  row and unused source must be absent
- make `open_session` and `open_session_with_projection` disagree on failure
  code, handle wrapping, or cleanup for the same fixture — fail the shared
  lifecycle proof
- publish ACP acknowledgement on `claude-code.headless`,
  `claude-code.response-only`, or structured-run applicability — fail closed
- reach 53 rows by filtering an emitted superset, ignoring an operation shape,
  or borrowing another route identity — fail exact ledger reconciliation

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy --json scan god-files`
- `git diff --check`

No live probe or provider contact belongs to this card.

## Auto-Continuation

No. Stop after one reviewable PR. The orchestrator must review the exact
53-row proof and public API head before another Batch 9.4 candidate is
reassessed or promoted.

## Closeout

PR 141 merged exact reviewed head `1edc7e73019a450605cb681eb56aeb35ad188557`
through `5d1f173ad0637c16c24f5134ef45dc559f67c61d`. The independent
ledgers reconcile to 30 = 29 emitted / 1 withheld for `claude-agent.acp`,
12 = 11/1 for `claude-code.headless`, and 11 = 9/2 for
`claude-code.response-only`.

The adapter retains exact reasoning acknowledgement, keeps prepared and
active-observation sources distinct, and serves the preserved and additive
open methods from one private lifecycle. Missing, malformed, duplicate,
unadvertised, unqualified, or unbounded confirmation produces runtime failure
with no contribution. Only the Claude Agent adapter API baseline changed,
with 27 additive lines. Focused validation passed 188 tests, extracted-package
and semantic API checks passed, the god-file scan improved from 391 to 387,
and all five CI jobs were green. No provider contact or live probe occurred.

Candidate D therefore adds 53 proved rows to the earlier 148. Batch 9.4 now
has 201 proved rows and 566 remaining rows in candidates B, C, E-G, and I-L.
The next current-main lifecycle audit is recorded in the Batch 9.4 checkpoint;
it promotes no implementation card.

## Stop Conditions

- Stop if the card needs a runtime/testkit/core public type, source kind,
  composer rule, fixed maximum, failure kind, or contract amendment.
- Stop if exact rejected reasoning cannot be retained without exposing raw ACP
  payload or accepting an unadvertised/unbounded provider value.
- Stop if the existing and additive open methods cannot share one private
  lifecycle with identical failure and cleanup behavior.
- Stop if the 30/12/11 ledgers need an exception list, inferred support, or
  truth borrowed from another route or operation shape.
- Stop if scope widens to Kimi, Cline, another Batch 9.4 candidate, Batch 9.5,
  or provider contact.

## Evidence

- [Claude Agent acknowledgement public-baseline gate](../../../triage/2026-08-31-contract-061-claude-agent-acknowledgement-public-baseline-gate.md)
- [card 030 acknowledgement stop](030-contract-061-acknowledgement-candidate-reassessment.md)
- [Batch 9.4 package expansion](../../../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [Contract 061](../../../contracts/061-consumer-route-feature-and-control-projection.md)
- [reviewed census](../../../triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [Batch 9.1 public baseline](../../../triage/2026-08-31-contract-061-batch-9-1-public-baseline-gate.md)
