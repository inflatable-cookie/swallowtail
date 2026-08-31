# 2026-08-31 Contract 061 Batch 9.1 Public Baseline Gate

Status: complete; strict-ready; card 022 ready
Owner: Tom
Source: Contract 061, accepted realization decisions, and current runtime and
adapter source

## Purpose

Close as much of g05.009 Batch 9.1 as current authority and realized source
permit, then apply the card-readiness rubric. This is planning evidence. It
does not implement Rust, dispatch a worker, contact a provider, or authorize
PR 127.

The review uses main through
`be98c30d682bea9ab01c5fa5e9af46e7180d4fbc` and the reviewed 767-row census.

## Settled Package And Composition Shape

The accepted realization decisions and current import graph close these parts
of the public baseline:

- a private `consumer_route_projection` module in `swallowtail-runtime` is
  publicly re-exported from the crate root;
- the public family is named `ConsumerRouteProjection`; its three immutable
  views are `ConsumerRouteSelectionSummary`,
  `ConsumerRouteSessionStartControls`, and
  `ConsumerRouteActiveSessionState`;
- adapter facades emit immutable
  `ConsumerRouteProjectionContribution` values; the runtime imports no adapter
  package and performs no route enumeration;
- `ConsumerRouteProjectionSourceId`,
  `ConsumerRouteProjectionSourceKind`, and
  `ConsumerRouteProjectionSourceIdentity` name every independently replaceable
  configured-instance, prepared-operation, adapter-contribution, or
  active-session observation used by the snapshot;
- `ConsumerRouteProjectionInput<'a>` borrows the exact current
  `ConfiguredProviderInstanceRecord` and `PreparedOperationEvidence`, owns
  their supplied source identities, and borrows only contributions supplied by
  the consumer for linked adapters;
- the sole composer signature is

  ```rust
  pub fn compose_consumer_route_projection(
      input: ConsumerRouteProjectionInput<'_>,
  ) -> Result<ConsumerRouteProjection, ConsumerRouteProjectionFailure>
  ```

- the projection exposes `identity()`, `selection_summary()`,
  `session_start_controls()`, `active_session_state()`, and `sources()`;
  each view exposes one exact-size `rows()` iterator and no mutator;
- `ConsumerRouteProjectionFailure` exposes `kind()` and `diagnostic()`;
  `ConsumerRouteProjectionFailureKind` distinguishes `IdentityInvalid`,
  `LimitExceeded`, `DuplicateSource`, `DuplicateRow`,
  `ApplicabilityDisagreement`, `SnapshotIdentityDisagreement`,
  `MutationAuthorityAbsent`, `ValueDomainInvalid`, and
  `SafeReasonLimitExceeded`;
- one `ConsumerRouteProjectionContribution::new(...)` admission path receives
  exact applicability, source identities, and the three row collections. It
  rejects an invalid contribution before composition. The composer rejects a
  mixed snapshot as a whole.

Portable feature identity is a closed runtime enum over current portable
capability and integration concepts, with a bounded namespaced variant.
Portable control identity is a closed runtime enum, also with a bounded
namespaced variant. A free-form string cannot claim portable standing. Later
package batches may add reviewed enum variants without changing the
namespaced-extension rule.

## Fixed Admission Maxima

The maxima are per exact route projection. They do not use 767 as a snapshot
cap and callers cannot replace them.

| Public constant | Value | Current per-route evidence | Explicit headroom |
| --- | ---: | --- | --- |
| `MAX_CONSUMER_ROUTE_SELECTION_SUMMARY_ROWS` | 32 | 24 rows | 8 rows (33%) |
| `MAX_CONSUMER_ROUTE_SESSION_START_ROWS` | 16 | 11 session-start plus per-turn rows | 5 rows (45%) |
| `MAX_CONSUMER_ROUTE_ACTIVE_SESSION_ROWS` | 8 | 4 rows | 4 rows (100%) |
| `MAX_CONSUMER_ROUTE_ENUMERABLE_VALUES` | 512 | 256 negotiated session options | 256 values (100%) |
| `MAX_CONSUMER_ROUTE_ENUMERABLE_VALUE_BYTES` | 512 | 256-byte negotiated option text | 256 bytes (100%) |
| `MAX_CONSUMER_ROUTE_NAMESPACED_EXTENSIONS` | 16 | 13 controls on the largest control route; 6 route-exclusive semantic ids on the largest such route | 3 controls over the conservative route-control proxy and 10 over route-exclusive identity |
| `MAX_CONSUMER_ROUTE_EXTENSION_TEXT_BYTES` | 128 | 49-byte semantic id and 29-byte route id | at least 79 bytes |
| `MAX_CONSUMER_ROUTE_SOURCE_IDENTITIES` | 16 | four independently replaceable top-level source classes in the accepted first composition | 12 identities |
| `MAX_CONSUMER_ROUTE_SOURCE_ID_BYTES` | 128 | a 29-byte route id plus a 49-byte semantic id covers the longest current composite identity components | at least 49 bytes |
| `MAX_CONSUMER_ROUTE_SAFE_REASON_BYTES` | 256 | 80-byte longest static safe-diagnostic message across core, runtime, Codex, and OpenAI source | 176 bytes (220%) |

All text admission counts UTF-8 bytes, rejects blank or control-bearing
identity text, and rejects rather than truncates. A source may remain
unbounded internally; only a value admitted to the projection must satisfy the
projection maximum. Safe reasons copy only source-supplied safe diagnostics.

## Replacement And Failure Behavior

The composer is pure and receives no prior projection. A changed configured
revision or any changed source identity creates a separately composed
replacement snapshot. It never mutates, refreshes, watches, or patches an
existing value. Equal row content with a different source identity is still a
replacement.

Contribution admission rejects duplicate rows, duplicate source identities,
invalid domains, over-limit collections or text, applicability wider than the
exact contribution binding, a selectable or effective claim without the exact
authority record, and a safe reason not supplied by its named source. Whole
snapshot composition rejects instance, revision, driver, facade, route, model,
operation, access, or source-identity disagreement between the configured
record, prepared evidence, and contributions.

The four Contract 061 points map directly:

| Contract point | Planned behavior |
| --- | --- |
| applicability disagreement | reject the contributed row before publication |
| snapshot identity disagreement | reject the whole composition |
| absent mutation authority | reject selectable, effective, or acknowledged posture; observation-only remains admissible |
| unbounded reason claim | reject the reason; unknown or absent source truth stays unknown or absent |

## First-Vertical Evidence Check

The Codex side is reachable without a new lifecycle abstraction.
`CodexPreparedSession` retains its `CodexPreparedEvidence`, exact plan, and
`OpenSessionRequest`; the plan retains the capability and session-access
policy produced when the consumer admitted the per-turn user-input exchange,
while the request retains the exact session options. The exact prepared
facades can disposition the 36 census rows without claiming provider mutation;
a row backed only by matrix or route-wide posture stays withheld.

The OpenAI Realtime prepared facade retains the requested reasoning mode and
exact plan. During `open_session`, private `expect_updated` code validates the
provider's `session.updated` reasoning acknowledgement. Success is then
returned only as `Box<dyn RealtimeMediaSessionHandle>`, whose public trait has
no acknowledgement accessor. An acknowledgement mismatch becomes only
`RuntimeFailure`.

Extending the generic realtime handle with a provider-specific value would
flatten route truth and widen a shared runtime role. Treating successful open
as effective outside the adapter would discard the exact acknowledgement
record. Publishing only descriptor support would fail the operator-selected
two-route evidence bar. The operator therefore accepted the additive
adapter-owned seam below. It preserves the current generic handle and keeps
acknowledgement truth inside the adapter that parsed it.

## Operator Decision And Exact Route-Local Surface

On 2026-08-31 the operator approved the additive adapter-owned typed open
result. The existing `OpenAiPreparedRealtimeSession::open_session` signature
and behavior remain public and unchanged. One private low-level open lifecycle
serves both methods.

Every listed prepared facade exposes this inherent contribution method:

```rust
pub fn consumer_route_projection_contribution(
    &self,
    source_id: ConsumerRouteProjectionSourceId,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure>
```

For `codex.app-server`, the method is implemented only on
`CodexPreparedCatalogue`, `CodexPreparedSession`,
`CodexPreparedSessionCatalogue`, `CodexPreparedSessionHistory`,
`CodexPreparedSessionImport`, `CodexPreparedSessionReconciliation`,
`CodexPreparedArchive`, `CodexPreparedRestore`, and `CodexPreparedDelete`.
Each facade emits only rows proved by its exact prepared evidence and operation
shape. There is no route-wide aggregate facade or matrix-derived contribution.

For `openai.realtime`, the method is implemented on
`OpenAiPreparedRealtimeSession` for prepared selection and session-start truth.
The exact acknowledgement seam adds these public types and method:

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
    pub fn open_session_with_projection(
        &self,
        active_session_source_id: ConsumerRouteProjectionSourceId,
        services: HostServices,
    ) -> OpenAiRealtimeProjectionOpenFuture;
}
```

`OpenAiRealtimeProjectionOpenOutcome` is returned only after the exact
`session.updated` event. A matching reasoning acknowledgement supplies the
provider-effective contribution. An exact, well-formed different effort may
return `Rejected`; missing, malformed, out-of-order, transport, setup, timeout,
disconnect, or otherwise unknown evidence returns `Runtime` and no rejected
contribution. When reasoning was not requested, successful open does not
invent a reasoning state.

The adapter does not expose its private acknowledgement payload. The new
method adds no callback, handle downcast, generic provider payload,
composer-side execution, runtime enumeration, or mutation authority.

## Portable Fixtures And Validation

Card 022 must include portable testkit fixtures for
all fixed maxima and all nine failure kinds; source replacement with identical
rows; each Contract 061 counterexample; view and lifecycle separation;
unknown/absence without an invented reason; and no raw targets, commands,
credentials, paths, environment, provider payload, or presentation prose.

The two adapter packages must add deterministic, provider-free fixtures for
all 36 Codex rows and 15 Realtime rows. Realtime fixtures must cover exact
requested, pending, matching-effective, mismatched/rejected, transport-failed,
and absent-reasoning states. No live probe belongs to this lane.

The accepting validation tier is:

- `cargo fmt -p swallowtail-runtime -p swallowtail-testkit -p swallowtail-adapter-codex -p swallowtail-adapter-openai -- --check`
- `effigy validate:focused swallowtail-runtime swallowtail-testkit swallowtail-adapter-codex swallowtail-adapter-openai`
- `effigy package:verify-affected swallowtail-runtime swallowtail-testkit swallowtail-adapter-codex swallowtail-adapter-openai`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Readiness Verdict

Posture: `strict-ready`.

Planning verdict: `coherent`. Package ownership, dependency direction,
composer and contribution shape, fixed maxima, failure behavior, replacement,
route-local public signatures, fixtures, validation, and stops are bounded.
The additive Realtime seam preserves exact acknowledgement truth without
widening the generic handle or changing the existing open path.

Card 022 passes the Batch 9.1 readiness rubric as one coherent four-package,
two-route tranche. A runtime/testkit-only card would not prove contribution;
a one-route card would not meet the accepted evidence bar. The card does not
authorize provider contact, the remaining 716 rows, package expansion,
generation closeout, or PR 127 action. It stops after the 51-row proof for an
orchestrator checkpoint.

## Authority

- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
- [g05.009](../roadmaps/g05/009-contract-061-consumer-projection-realization.md)
- [realization-readiness inventory](2026-08-31-contract-061-realization-readiness-inventory.md)
- [consumer projection census](2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [Contract 037](../contracts/037-prepared-consumer-integration.md)
- [Contract 047](../contracts/047-configured-provider-instance-catalogue.md)
