# 022 Contract 061 Composer And Two-Route Vertical

Status: ready; one four-package 51-row implementation tranche
Owner: Tom
Created: 2026-08-31
Updated: 2026-08-31
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: Contract 061; completed Batch 9.1 public baseline gate

## Goal

Realize the Contract 061 runtime composer, portable conformance, and the first
meaningful vertical through all 36 `codex.app-server` census rows and all 15
`openai.realtime` census rows without widening descriptive truth into execution
or mutation authority.

## Scope

1. Add the runtime-owned public family and fixed admission maxima named by the
   [Batch 9.1 gate](../../../triage/2026-08-31-contract-061-batch-9-1-public-baseline-gate.md)
   under one focused `swallowtail-runtime` module re-exported from the crate
   root.
2. Implement immutable contribution admission and
   `compose_consumer_route_projection` as one pure fail-closed composer over
   exact `ConfiguredProviderInstanceRecord`, `PreparedOperationEvidence`,
   typed source identities, and only the contributions supplied by the
   consumer.
3. Implement the three immutable views, exact-size row iteration, all nine
   failure kinds, source replacement, fixed row/value/extension/source/reason
   maxima, UTF-8 byte counting, and reject-without-truncation behavior exactly
   as selected by Batch 9.1.
4. Add `swallowtail-testkit` fixtures and portable assertions for every fixed
   maximum, failure kind, Contract 061 counterexample, lifecycle split,
   identical-row source replacement, unknown/absence behavior, and forbidden
   raw or presentation data.
5. Add
   `consumer_route_projection_contribution(&self, source_id)` with the exact
   Batch 9.1 signature to `CodexPreparedCatalogue`, `CodexPreparedSession`,
   `CodexPreparedSessionCatalogue`, `CodexPreparedSessionHistory`,
   `CodexPreparedSessionImport`, `CodexPreparedSessionReconciliation`,
   `CodexPreparedArchive`, `CodexPreparedRestore`, and `CodexPreparedDelete`.
   Each method emits only its exact prepared-operation truth.
6. Prove all 36 `codex.app-server` census rows across exact prepared operation
   snapshots. Emit only rows with exact runtime/prepared authority; record an
   explicit withheld disposition for any matrix-only or incompatible-operation
   row. Keep its per-turn user-input exchange per-turn and do not infer provider
   mutation or acknowledgement.
7. Add the same prepared contribution method to
   `OpenAiPreparedRealtimeSession`. Preserve its existing `open_session` method
   and add the exact `OpenAiRealtimeProjectionOpenFuture`,
   `OpenAiRealtimeProjectionOpenOutcome`,
   `OpenAiRealtimeProjectionOpenFailure`, and
   `open_session_with_projection` surface from Batch 9.1.
8. Refactor only the private Realtime open lifecycle needed for both public
   methods to share the same transport, setup, `session.updated` validation,
   handle construction, failure, and cleanup path. Do not duplicate the
   lifecycle or expose `SessionReasoningAck`.
9. Return a success contribution only after exact `session.updated`. Return a
   `Rejected` contribution only for an exact well-formed different reasoning
   effort. Missing, malformed, out-of-order, transport, setup, timeout,
   disconnect, and unknown failures return `Runtime` with no invented rejected
   state. Omitted reasoning produces no reasoning state.
10. Prove all 15 `openai.realtime` census rows, including distinct requested,
    pending, matching-effective, exact rejected, transport-failed, and absent
    reasoning fixtures. A row without exact route-local authority stays an
    explicit withheld disposition.
11. Add a deterministic coverage ledger or equivalent test data that maps the
    exact 51 first-tranche census rows to provider-free adapter proofs. Claim no
    implementation coverage for the remaining 716 rows.
12. Reconcile card, milestone, batch-card index, g05/generation indexes, and
    the sole Next Task after validation. Stop for orchestrator review and the
    required two-route checkpoint.

## Exact Realtime Surface

The binding public signatures are:

```rust
pub type OpenAiRealtimeProjectionOpenFuture = BoxFuture<
    'static,
    Result<OpenAiRealtimeProjectionOpenOutcome, OpenAiRealtimeProjectionOpenFailure>,
>;

pub struct OpenAiRealtimeProjectionOpenOutcome { /* private fields */ }

impl OpenAiRealtimeProjectionOpenOutcome {
    pub fn session(&self) -> &dyn RealtimeMediaSessionHandle;
    pub const fn contribution(&self) -> &ConsumerRouteProjectionContribution;
    pub fn into_parts(
        self,
    ) -> (
        Box<dyn RealtimeMediaSessionHandle>,
        ConsumerRouteProjectionContribution,
    );
}

pub enum OpenAiRealtimeProjectionOpenFailure {
    Runtime(RuntimeFailure),
    Rejected {
        failure: RuntimeFailure,
        contribution: ConsumerRouteProjectionContribution,
    },
}

impl OpenAiRealtimeProjectionOpenFailure {
    pub const fn failure(&self) -> &RuntimeFailure;
    pub const fn rejected_contribution(
        &self,
    ) -> Option<&ConsumerRouteProjectionContribution>;
    pub fn into_parts(
        self,
    ) -> (RuntimeFailure, Option<ConsumerRouteProjectionContribution>);
}

impl OpenAiPreparedRealtimeSession {
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure>;

    pub fn open_session_with_projection(
        &self,
        active_session_source_id: ConsumerRouteProjectionSourceId,
        services: HostServices,
    ) -> OpenAiRealtimeProjectionOpenFuture;
}
```

Do not add another public route, result shape, generic handle member, callback,
downcast, provider payload, or composer execution seam.

## Out Of Scope

- changes to `swallowtail-core` or Contracts 037, 047, 057, or 061
- an umbrella crate, adapter registry, runtime route enumeration, or adapter
  callback/downcast discovery
- caller-supplied limits, a 767-row snapshot cap, or a closed availability
  reason taxonomy
- consumer UI, defaults, persistence, routing, execution, fallback, or
  provider mutation authority
- the remaining 716 census rows, package-coherent expansion, or the final
  all-route audit
- provider contact, live probes, compatibility/currentness work, watcher
  restart, PR 127, or generation closeout

## Acceptance Criteria

- [ ] runtime and core dependency direction remains acyclic and unchanged
- [ ] the public runtime surface, constants, failure kinds, accessors, and
      composer match the Batch 9.1 baseline exactly
- [ ] invalid contributions fail before composition and mixed snapshots reject
      as a whole
- [ ] equal rows with a changed source identity produce a replacement snapshot
- [ ] testkit covers every fixed maximum, failure kind, and Contract 061
      counterexample without adapter production dependencies
- [ ] the nine named Codex prepared facades and coverage ledger disposition
      exactly the 36 `codex.app-server` rows across their operation shapes;
      matrix-only or incompatible-operation rows are withheld, not emitted
- [ ] per-turn exchange remains per-turn and creates no mutation claim
- [ ] existing `OpenAiPreparedRealtimeSession::open_session` remains source and
      behavior compatible
- [ ] both OpenAI public open methods share one private low-level lifecycle
- [ ] only a matching exact `session.updated` acknowledgement produces
      provider-effective reasoning
- [ ] only an exact well-formed different effort produces a rejected-state
      contribution; every unknown failure carries none
- [ ] deterministic coverage evidence proves exactly the selected 51 rows and
      does not imply the other 716
- [ ] no raw target, command, credential, path, environment value, provider
      payload, unbounded diagnostic, or presentation prose enters projection

## Review Oracle

Invariant: one exact immutable snapshot preserves source, lifecycle, and
acknowledgement truth without creating execution or mutation authority.

Counterexamples and required proof:

- a route-wide Codex capability combined with a prepared operation that does
  not admit it — reject or withhold the usable row before publication
- otherwise equal rows from a changed source id or stale configured revision —
  replace for the former; reject the mixed snapshot for the latter
- a per-turn exchange or post-open option described as session-start mutable —
  reject with `MutationAuthorityAbsent`
- an absent source reason replaced with adapter or provider text — reject the
  reason and preserve unknown/absence
- OpenAI exact matching acknowledgement inferred from successful handle
  construction — fail; only the parsed matching `session.updated` may prove it
- missing, malformed, out-of-order, transport-failed, or timed-out OpenAI
  acknowledgement reported as rejected — fail; the typed error carries no
  contribution
- one route or the common kernel presented as completion of the tranche — fail;
  all exact 51 rows need provider-free proof

## Validation

- `cargo fmt -p swallowtail-runtime -p swallowtail-testkit -p swallowtail-adapter-codex -p swallowtail-adapter-openai -- --check`
- `effigy validate:focused swallowtail-runtime swallowtail-testkit swallowtail-adapter-codex swallowtail-adapter-openai`
- `effigy package:verify-affected swallowtail-runtime swallowtail-testkit swallowtail-adapter-codex swallowtail-adapter-openai`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

No live probe or provider contact belongs to this card.

## Auto-Continuation

No. Stop after one reviewable PR. The orchestrator must review the exact 51-row
proof and compile package-expansion planning before another card can become
ready.

## Stop Conditions

- Stop if any exact public name, signature, fixed maximum, or failure mapping
  must change.
- Stop if a Codex row lacks exact prepared-operation authority or an OpenAI
  state lacks exact `session.updated` evidence.
- Stop if the shared composer needs core changes, adapter dependencies, a
  registry, runtime enumeration, callbacks, downcasts, or provider payloads.
- Stop if the four-package tranche cannot prove all 51 rows without widening
  into another package or route.
- Stop if Contracts 037, 047, 057, or 061 need amendment.
- Stop before package expansion, provider contact, PR 127, or generation
  closeout.

## Evidence

- Contract 061 and its four named fail-closed points
- accepted Contract 061 realization decisions
- completed Batch 9.1 public baseline gate
- reviewed 767-row census; exact first-tranche split is 36 Codex plus 15
  OpenAI Realtime rows
- operator-approved additive Realtime prepared-open seam on 2026-08-31
