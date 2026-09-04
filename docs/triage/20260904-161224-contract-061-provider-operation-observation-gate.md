# Contract 061 Provider-Operation Observation Public-Baseline Gate

Status: promoted 2026-09-04; Contract 061 amended and ready card 073 owns the runtime baseline; retained as the exact names, admission, and assertion evidence that card owns; pruned when card 073 closes
Owner: Tom
Date: 2026-09-04
Source: Contract 061, the Kimi active-observation gate, the candidate I audit,
and current `main` at `13df1599c96a455689ddea564c45d5a4ffbd4e9a`

## Purpose

Fix one additive shared representation for observed truth produced by a
completed provider operation that opens no session. This gate covers only
`OperationShape::ProviderSessionCatalogue` and
`OperationShape::ProviderSessionHistory`.

This note is planning evidence. It edits no contract or Rust, audits no
candidate L rows, and grants no implementation or coverage authority.

## Existing Meanings Stay Exact

These current public definitions remain byte-identical in meaning:

| Existing name | Current definition | Exact anchor |
| --- | --- | --- |
| `ConsumerRouteProjectionSourceKind::ActiveSessionObservation` | “One exact post-open active-session observation.” | `crates/swallowtail-runtime/src/consumer_route_projection/identity.rs:16-17` |
| `ConsumerRouteLifecycle::PostOpenObservationOnly` | “Observed only after the session opens.” | `crates/swallowtail-runtime/src/consumer_route_projection/semantics/posture.rs:14-15` |
| `ConsumerRouteActiveSessionState` | “Immutable post-open observation and exact negotiated state.” | `crates/swallowtail-runtime/src/consumer_route_projection/views.rs:34-37` |

None includes a catalogue call, history-page call, completed provider
operation, or operation outcome. No existing constructor, accessor, view, or
admission rule changes meaning.

## Selected Additive Vocabulary

### Required public names

```rust
pub enum ConsumerRouteProjectionSourceKind {
    // existing variants unchanged

    /// One exact observation from a completed provider operation that opened no session.
    ProviderOperationObservation,
}

pub enum ConsumerRouteLifecycle {
    // existing variants unchanged

    /// Observed only after a provider operation completes without opening a session.
    PostOperationObservationOnly,
}

pub enum ConsumerRouteSourceClass {
    // existing variants unchanged

    /// A validated successful provider-operation outcome.
    ProviderOperationOutcome,
}

pub enum ConsumerRouteEvidenceStrength {
    // existing variants unchanged

    /// The completed provider operation returned validated outcome evidence.
    CompletedProviderOperation,
}

pub enum ConsumerRouteProjectionFailureKind {
    // existing variants unchanged

    /// Provider-operation source, shape, outcome, or row semantics are invalid.
    ProviderOperationObservationInvalid,
}

/// Borrowed successful outcome that may prove provider-operation observation.
pub enum ConsumerRouteProviderOperationOutcome<'a> {
    /// One validated provider-session catalogue outcome.
    ProviderSessionCatalogue(&'a ProviderSessionCatalogueOutcome),
    /// One validated provider-session history page.
    ProviderSessionHistory(&'a ProviderSessionHistoryPage),
}

/// Immutable observation rows from one completed provider operation.
pub struct ConsumerRouteProviderOperationObservation { /* private fields */ }

impl ConsumerRouteProviderOperationObservation {
    pub fn new(
        evidence: &PreparedOperationEvidence,
        outcome: ConsumerRouteProviderOperationOutcome<'_>,
        source: ConsumerRouteProjectionSourceIdentity,
        rows: impl IntoIterator<Item = ConsumerRouteProjectionRow>,
    ) -> Result<Self, ConsumerRouteProjectionFailure>;

    pub const fn applicability(&self) -> &ConsumerRouteApplicability;
    pub const fn source(&self) -> &ConsumerRouteProjectionSourceIdentity;
    pub fn rows(&self) -> impl ExactSizeIterator<Item = &ConsumerRouteProjectionRow>;
}

/// Immutable observations produced by completed provider operations.
pub struct ConsumerRouteProviderOperationState { /* private fields */ }

impl ConsumerRouteProviderOperationState {
    pub fn rows(&self) -> impl ExactSizeIterator<Item = &ConsumerRouteProjectionRow>;
}

impl<'a> ConsumerRouteProjectionInput<'a> {
    pub fn with_provider_operation_observations(
        self,
        observations: impl IntoIterator<
            Item = &'a ConsumerRouteProviderOperationObservation,
        >,
    ) -> Self;
}

impl ConsumerRouteProjection {
    pub const fn provider_operation_state(
        &self,
    ) -> &ConsumerRouteProviderOperationState;
}
```

`ConsumerRouteProjectionContribution::new`, its five arguments, its three row
accessors, and the three existing projection accessors stay unchanged. The new
observation value is separate rather than another optional argument on the
existing constructor.

The two observed census identities remain bounded namespaced control
extensions with semantic IDs `control.provider-session-catalogue` and
`control.provider-session-history`, qualified by exact route and interface
version. They do not use `ConsumerRouteControlId::SessionCatalogueBounds` or
`SessionHistoryBounds`; those existing variants name prepared query-bound
rows. This gate adds no portable control-ID variant.

### Fixed maximum

```rust
/// Maximum provider-operation observation rows in one exact route projection.
pub const MAX_CONSUMER_ROUTE_PROVIDER_OPERATION_ROWS: usize = 4;
```

| Carrier route | Gate rows | Route inventory high-water |
| --- | ---: | ---: |
| `deepseek-harness.local-server` | catalogue + history = 2 | 2 |
| `kimi-code.acp` | catalogue = 1 | 1 |
| `opencode.http` | catalogue = 1 | 1 |

The maximum is the carrier high-water mark 2 plus 2 rows of explicit headroom
(100%). It is per exact route projection, fixed by the library, and not caller
configurable. It does not use the global 767-row census as a cap.

No other maximum changes:

- the new view carries descriptor rows, not catalogue candidates or history
  items, so it needs no value or payload collection maximum;
- the longest new semantic ID is 34 UTF-8 bytes and the longest carrier route
  ID is 29 bytes, both below
  `MAX_CONSUMER_ROUTE_EXTENSION_TEXT_BYTES = 128`;
- at most two new namespaced rows take the current conservative extension
  high-water from 13 to 15, below
  `MAX_CONSUMER_ROUTE_NAMESPACED_EXTENSIONS = 16`;
- one new independently replaceable outcome source takes the accepted
  top-level source-class count from four to five, below
  `MAX_CONSUMER_ROUTE_SOURCE_IDENTITIES = 16`.

## Admission

`ConsumerRouteProviderOperationObservation::new` is the only admission path
for the new source kind and view.

1. `source.kind()` must be
   `ConsumerRouteProjectionSourceKind::ProviderOperationObservation`.
2. `evidence.plan().requirements().operation_shape()` must be exactly
   `ProviderSessionCatalogue` for a
   `ConsumerRouteProviderOperationOutcome::ProviderSessionCatalogue`, or
   exactly `ProviderSessionHistory` for
   `ConsumerRouteProviderOperationOutcome::ProviderSessionHistory`.
3. The successful outcome must retain its source plan privately when its
   existing constructor validates it. Admission compares that retained plan's
   `PreflightPlan` with `evidence.plan()`. Existing outcome constructor
   signatures and public accessors stay unchanged.
4. Every row must use the supplied exact source identity,
   `PostOperationObservationOnly`, `ProviderOperationOutcome`, and
   `CompletedProviderOperation`.
5. Every row must be `ObservationOnly`, carry
   `ConsumerRouteMutationAuthority::Absent`, use
   `ConsumerRouteValueKind::BoundedQuery` with
   `ConsumerRouteValueDomain::Descriptor` and
   `ConsumerRouteOmissionSemantics::NotSelectable`, and expose observed state
   only: requested, prepared, pending, provider-effective, and rejected are
   false; observed is true.
6. The row applicability must equal the applicability derived from the exact
   prepared evidence, including instance, revision, driver, facade, host,
   role, execution layer, operation shape, model binding, access profile,
   credential mechanism, all five access-state dimensions, and resource
   constraints.
7. The row count must not exceed
   `MAX_CONSUMER_ROUTE_PROVIDER_OPERATION_ROWS`. Duplicate semantic identities,
   over-bound extension text, unknown sources, or safe reasons from another
   source retain their existing failures.
8. A failed, cancelled, timed-out, malformed, cleanup-incomplete, or otherwise
   non-successful operation produces no outcome value and therefore no
   provider-operation observation.

The constructor rejects rule 1-5 or an operation/outcome mismatch with:

```rust
ConsumerRouteProjectionFailureKind::ProviderOperationObservationInvalid
```

Exact diagnostics:

| Rejection | Code | Message |
| --- | --- | --- |
| session or other unapproved shape | `swallowtail.consumer_route_projection.provider_operation_shape_rejected` | `Provider-operation observation requires an admitted completed operation shape` |
| plan/outcome mismatch or non-outcome source | `swallowtail.consumer_route_projection.provider_operation_evidence_rejected` | `Provider-operation observation does not match its completed outcome evidence` |
| wrong lifecycle, source class, strength, actor, state, value, or authority | `swallowtail.consumer_route_projection.provider_operation_row_rejected` | `Provider-operation observation row claims incompatible lifecycle or authority` |
| new source kind supplied through ordinary prepared contribution admission | `swallowtail.consumer_route_projection.provider_operation_source_rejected` | `Prepared contribution cannot publish provider-operation observation` |
| more than four observation rows | `swallowtail.consumer_route_projection.provider_operation_limit_exceeded` | `Projected provider-operation state exceeds the fixed row maximum` |

The typed outcome argument makes “prepared record only” unconstructible on the
new path. Ordinary `ConsumerRouteProjectionContribution::new` explicitly
rejects `ProviderOperationObservation` in its source set, even when no row uses
that source. A prepared plan therefore cannot masquerade as completed outcome
evidence.

## Composition And Replacement

The pure composer gains one borrowed observation collection and one fourth
merge pass. The original configured record and prepared evidence remain the
snapshot anchors.

- ordinary `ConsumerRouteProjectionContribution` values feed only selection,
  session-start/per-turn, and active-session views;
- `ConsumerRouteProviderOperationObservation` values feed only
  `ConsumerRouteProviderOperationState`;
- a prepared contribution and a provider-operation observation for the same
  route may co-compose, but never merge: they retain distinct values, source
  kinds, source IDs, lifecycle bands, source classes, evidence strengths, and
  row collections;
- the prepared feature row may describe that a catalogue operation exists;
  only the successful outcome may publish the observed catalogue/history
  control row;
- a source ID repeated across configured, prepared, adapter, active-session,
  or provider-operation classes remains `DuplicateSource`; changing the
  outcome source ID produces a replacement snapshot even when rows are equal;
- exact applicability equality applies before the new rows enter the snapshot.
  Cross-instance, revision, route, model, operation, access-profile,
  credential, entitlement, endpoint, runtime, support, or resource assembly
  rejects the whole composition as `SnapshotIdentityDisagreement`;
- a catalogue observation and a history observation cannot share one snapshot,
  because their operation shapes differ. They compose as separate immutable
  projections;
- namespaced-extension admission counts all four views together. Duplicate
  semantic identities are rejected within the provider-operation view.

The composer still receives no previous projection and performs no provider
work. It does not copy catalogue candidates, history items, cursors, provider
session references, request IDs, payloads, diagnostics, credentials, targets,
or paths into a projected row.

## Proving Consumers And Third Carrier

All anchors resolve on `13df1599c96a455689ddea564c45d5a4ffbd4e9a`.

### Proving consumer 1: DeepSeek Harness local server

| Census row | Prepared anchor | Completed-outcome anchor |
| --- | --- | --- |
| 44 `control.provider-session-catalogue` | `crates/swallowtail-adapter-deepseek-harness/src/web_prepared.rs:301-354`, `:840-897`; the plan binds `OperationShape::ProviderSessionCatalogue` and `DriverRole::ProviderSessionCatalogue` at `:310-313` | `crates/swallowtail-adapter-deepseek-harness/src/web/driver.rs:384-553`; `list_provider_sessions` performs `SessionList` and returns `ProviderSessionCatalogueOutcome::new(..., CleanupOutcome::NotApplicable)` at `:546-552` |
| 45 `control.provider-session-history` | `crates/swallowtail-adapter-deepseek-harness/src/web_prepared.rs:356-437`, `:1082-1140`; the plan binds `OperationShape::ProviderSessionHistory` and `DriverRole::ProviderSessionHistory` at `:388-391`; the facade exposes no resume handle | `crates/swallowtail-adapter-deepseek-harness/src/web/driver.rs:557-674`; `page_provider_session_history` returns `ProviderSessionHistoryPage::new(..., CleanupOutcome::NotApplicable)` at `:667-673` |

The exact census anchors are
`docs/triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv:758-759`.
Neither operation opens or returns a session handle.

### Proving consumer 2: Kimi ACP session catalogue

`KimiPreparedSessionCatalogue` retains
`PreparedProviderSessionCatalogueEvidence` and exposes `list_sessions` at
`crates/swallowtail-adapter-kimi/src/prepared_profile/provider_session_catalogue.rs:22-74`.
The ACP driver initializes a catalogue attachment, calls `list_sessions`,
joins cleanup, and returns `ProviderSessionCatalogueOutcome::new` at
`crates/swallowtail-adapter-kimi/src/driver/session_catalogue.rs:49-115`.
The exact census row is
`docs/triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv:763`.
It is the `kimi-code.acp` `control.provider-session-catalogue` row. The prepared
facade continues to emit only the prepared catalogue feature row; the new
observed control row exists only after the outcome returns.

### Third carrier only: OpenCode HTTP

No candidate L audit is performed. The carrier anchor only is:

- `OpenCodePreparedSessionCatalogue::list_sessions` at
  `crates/swallowtail-adapter-opencode/src/prepared_profile/provider_sessions/catalogue.rs:16-65`;
- `OpenCodeHttpDriver::execute_provider_session_catalogue` returns
  `ProviderSessionCatalogueOutcome::new` after releasing access at
  `crates/swallowtail-adapter-opencode/src/driver/provider_session_import.rs:29-69`;
- the exact `opencode.http` `control.provider-session-catalogue` census row is
  `docs/triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv:741`.

No other OpenCode row, facade, disposition, or candidate-L conclusion is part
of this gate.

## Draft Contract 061 Amendment

The following text is ready for Chatterbox promotion.

### Boundary replacement

> In:
>
> - one public projection family with selection-summary, session-start,
>   active-session, and provider-operation views

### Projection Views introduction replacement

> The four views share one semantic vocabulary and one snapshot identity. They
> remain separate surfaces because their evidence and lifecycle differ. A row
> admitted to one view gains no standing in another.

### New Projection Views subsection

> ### Provider-Operation State
>
> Project only observations returned by a completed provider operation that
> opens no session. This view admits provider-session catalogue and
> provider-session history outcomes only. A prepared plan, successful
> preparation, open session, failed operation, or documentation row is not a
> completed provider-operation observation.
>
> Provider-operation state is descriptor-only and observation-only. It carries
> no candidate, history item, cursor, provider-session reference, request,
> execution, resume, import, mutation, or acknowledgement authority.

### Descriptor Semantics lifecycle replacement

> - lifecycle: selection-summary, session-start-only, per-turn,
>   between-turn-negotiable, separately qualified mid-turn-negotiable,
>   post-open-observation-only, or post-operation-observation-only

### Snapshot Identity And Replacement addition

> A provider-operation observation names its completed outcome as an
> independently replaceable source. It remains distinct from the prepared
> operation that admitted dispatch. A changed outcome source identity creates
> a replacement snapshot even when its descriptor rows are equal.

### Fail-Closed Composition addition

> | Prepared plan, session-shaped source, or mismatched operation outcome presented as a completed provider-operation observation | provider-operation observation disagreement | reject the observation before publication; only a matching completed provider-session catalogue or provider-session history outcome may enter the provider-operation view |

> Prepared contributions and provider-operation observations may co-compose
> under exact snapshot agreement, but they never merge into one source or row.
> Cross-operation, cross-access, or repeated-source assembly rejects the whole
> snapshot. Absence of a successful outcome remains absence, never prepared or
> observed truth.

### Conformance additions

> - provider-operation rows appear only in the provider-operation view with a
>   completed provider-session catalogue or history outcome
> - interactive-session, structured-run, active-session, and prepared-only
>   evidence cannot enter the provider-operation source kind or lifecycle
> - provider-operation rows obey the fixed row maximum, remain
>   descriptor-only and observation-only, and carry no mutation authority or
>   provider payload
> - prepared and provider-operation sources remain distinct under replacement,
>   duplicate-source, exact applicability, and cross-access composition

### Acceptance addition

> - completed provider-session catalogue and history observations remain
>   distinct from preparation and active-session state, and a failed or absent
>   outcome publishes no provider-operation row

## Draft Runtime And Testkit Assertions

The runtime baseline card must add these assertions to the complete portable
Contract 061 suite. Names and expectations are fixed here.

```rust
pub fn assert_consumer_route_provider_operation_observation_contract() {
    assert_provider_operation_session_shape_is_rejected();
    assert_prepared_record_cannot_masquerade_as_provider_operation_observation();
    assert_provider_operation_row_maximum_is_fixed();
    assert_provider_operation_state_is_honest_descriptor_only_observation();
    assert_provider_operation_sources_compose_without_merging();
    assert_provider_operation_cross_access_and_source_disagreement_fail_closed();
}
```

Required assertion bodies:

1. `assert_provider_operation_session_shape_is_rejected`
   - pair canonical interactive-session `PreparedOperationEvidence` with a
     valid catalogue outcome;
   - expect `ProviderOperationObservationInvalid` and diagnostic
     `swallowtail.consumer_route_projection.provider_operation_shape_rejected`;
   - repeat for structured-run evidence;
   - prove catalogue evidence + catalogue outcome and history evidence +
     history page are the only admitted pairs.
2. `assert_prepared_record_cannot_masquerade_as_provider_operation_observation`
   - pass a `ProviderOperationObservation` source to ordinary
     `ConsumerRouteProjectionContribution::new`;
   - expect `ProviderOperationObservationInvalid` and diagnostic
     `swallowtail.consumer_route_projection.provider_operation_source_rejected`;
   - prove no new observation can be constructed without a successful typed
     outcome and that a failed outcome publishes no row.
3. `assert_provider_operation_row_maximum_is_fixed`
   - admit exactly `MAX_CONSUMER_ROUTE_PROVIDER_OPERATION_ROWS` unique rows;
   - reject maximum + 1 with `LimitExceeded` and
     `swallowtail.consumer_route_projection.provider_operation_limit_exceeded`;
   - prove no caller-selected maximum and no truncation path exists.
4. `assert_provider_operation_state_is_honest_descriptor_only_observation`
   - compose one namespaced `control.provider-session-catalogue` row and one
     separate history projection;
   - assert fourth-view-only placement, `BoundedQuery`, `Descriptor`,
     `NotSelectable`, `ObservationOnly`, `PostOperationObservationOnly`,
     `ProviderOperationOutcome`, `CompletedProviderOperation`, observed true,
     all other state flags false, mutation authority `Absent`, and source kind
     `ProviderOperationObservation`;
   - assert no candidate, item, cursor, provider-session reference, request,
     target, credential, path, command, or provider payload accessor exists.
5. `assert_provider_operation_sources_compose_without_merging`
   - co-compose one ordinary prepared contribution and one operation
     observation with distinct source IDs;
   - assert prepared rows remain in their existing view, operation rows remain
     in the fourth view, and both exact sources survive;
   - change only the outcome source ID and assert a separately composed
     replacement snapshot.
6. `assert_provider_operation_cross_access_and_source_disagreement_fail_closed`
   - reuse the testkit's degraded-runtime and exhausted-entitlement fixtures;
   - expect `SnapshotIdentityDisagreement` in both directions;
   - repeat one source ID across prepared and operation kinds and expect
     `DuplicateSource`;
   - pair catalogue prepared evidence with history outcome and expect
     `ProviderOperationObservationInvalid` /
     `swallowtail.consumer_route_projection.provider_operation_evidence_rejected`.

The complete suite `assert_consumer_route_projection_contract()` must call
`assert_consumer_route_provider_operation_observation_contract()`.

## Readiness Rubric

Posture: `strict-ready` for promotion into Contract 061 and compilation of one
runtime/testkit baseline card. No implementation is authorized by this note.

1. **Package ownership and dependency direction — pass.** New public
   vocabulary, admission, view, composer, fixture, and assertions stay in
   `swallowtail-runtime` and `swallowtail-testkit`; runtime imports no adapter
   and enumerates no route.
2. **Exact public names, signatures, and additivity — pass.** One source kind,
   lifecycle, outcome reference, observation value, fourth view, input
   extension, accessor, source-class variant, and evidence-strength variant are
   fixed. Existing constructors, accessors, and session-scoped meanings stay
   unchanged.
3. **Fixed maxima — pass.** The sole new collection has fixed maximum 4 from a
   measured carrier high-water 2 plus 100% headroom. Existing text, extension,
   and source maxima cover the added values.
4. **Admission, composition, failure, and replacement — pass.** Only two typed
   completed outcomes enter; plan-only and session-shaped attempts have a
   named failure; cross-access, source identity, view separation, and
   replacement behavior are exact.
5. **Two-route proof and portable assertions — pass.** DeepSeek Harness rows
   44/45 and the Kimi catalogue row resolve to current-main prepared and
   successful-outcome anchors. Six provider-free testkit assertions are fixed.
   OpenCode is evidence of recurrence only.
6. **Validation and stops — pass.** The planning gate names docs/Northstar/diff
   validation. The later runtime baseline must stop on any reinterpretation,
   third operation shape, untyped success claim, payload projection, caller
   limit, adapter dependency, registry, callback, or execution/mutation
   authority.

## Open Decisions

None. The operator already chose a shared gate with DeepSeek Harness and Kimi
as proving consumers. This note selects one additive fourth-view shape and
closes its names, maximum, admission, composition, failure, replacement,
contract text, and portable assertions. Chatterbox may promote or reject the
baseline, but no remaining semantic choice is delegated to the runtime worker.

## Review Oracle

Invariant: provider-operation observation is additive and outcome-backed.

- reinterpret any existing session-scoped name — fail
- admit a prepared plan, successful preparation, session handle, or failed
  operation as observed outcome — fail
- admit an operation shape other than `ProviderSessionCatalogue` or
  `ProviderSessionHistory` — fail
- place a provider-operation row in an existing view or an existing row in the
  fourth view — fail
- merge prepared and outcome sources or reuse their source ID — fail
- project candidate/history payload content or infer mutation, resume, import,
  or acknowledgement authority — fail
- exceed or caller-replace the fixed maximum — fail
- treat the OpenCode anchor as a candidate L audit — fail

## Next Move

Chatterbox may promote the drafted Contract 061 amendment, then compile one
runtime/testkit baseline card. Candidate I completion and card 034 stay closed
until that baseline merges.

## Authority

- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
- [g05.009](../roadmaps/g05/009-contract-061-consumer-projection-realization.md)
- [Card 070](../roadmaps/g05/batch-cards/070-contract-061-provider-operation-observation-gate.md)
- [Kimi active-observation gate](2026-09-01-contract-061-kimi-active-observation-public-baseline-gate.md)
- [candidate I audit](20260904-140002-contract-061-candidate-i-audit.md)
- [Batch 9.1 public baseline](2026-08-31-contract-061-batch-9-1-public-baseline-gate.md)
- [reviewed census](2026-08-30-consumer-route-feature-and-option-projection-census.csv)
