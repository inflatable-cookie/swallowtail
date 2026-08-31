# 2026-08-31 Contract 061 Batch 9.1 Public Baseline Gate

Status: operator API decision required; no ready card
Owner: Tom
Source: Contract 061, accepted realization decisions, and current runtime and
adapter source

## Purpose

Close as much of g05.009 Batch 9.1 as current authority and realized source
permit, then apply the card-readiness rubric. This is planning evidence. It
does not authorize Rust, a public API baseline, a worker, provider contact, or
PR 127.

The review uses main at
`0f84f8fe4d96402c6ece990ee3160fd7bd5e1705` and the reviewed 767-row census.

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
while the request retains the exact session options. An adapter-local accessor
can emit the 36-row contribution without claiming provider mutation.

The OpenAI Realtime side exposes a material public-API fork. The prepared
facade retains the requested reasoning mode and exact plan. During
`open_session`, private `expect_updated` code validates the provider's
`session.updated` reasoning acknowledgement. Success is then returned only as
`Box<dyn RealtimeMediaSessionHandle>`, whose public trait has no acknowledgement
accessor. An acknowledgement mismatch becomes only `RuntimeFailure`. The
requested, pending, provider-effective, and rejected observation therefore
cannot be emitted as an immutable contribution after the exact wire evidence
without selecting a new public return seam.

Extending the generic realtime handle with a provider-specific value would
flatten route truth and widen a shared runtime role. Treating successful open
as effective outside the adapter would discard the exact acknowledgement
record. Publishing only descriptor support would fail the operator-selected
two-route evidence bar.

## Operator API Fork

Recommendation: preserve the existing `open_session` method and add one
adapter-owned prepared-open method whose typed result returns the existing
runtime handle plus an immutable normalized contribution after exact
acknowledgement. Its typed failure may carry a rejected-state contribution
only when the exact `session.updated` evidence proves rejection; transport,
setup, timeout, or unknown failures carry no invented rejected state. The
method delegates to the same low-level lifecycle and adds no callback,
downcast, generic provider payload, composer-side execution, or mutation
authority.

Operator question:

1. Approve that additive adapter-owned typed open-result seam for
   `openai.realtime`; or require a different public route from the exact
   `session.updated` acknowledgement into an immutable contribution. A
   descriptor-only or inferred-effective substitute does not meet the already
   accepted tranche.

Exact route-local Rust names and signatures depend on that answer. The shared
runtime names above do not authorize implementation while this fork remains.

## Portable Fixtures And Validation

The first card, if later made ready, must include portable testkit fixtures for
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

Posture: `strict-paused`.

Planning verdict: `materially ambiguous`. Package ownership, dependency
direction, composer shape, fixed maxima, failure behavior, replacement,
fixtures, validation, and stops are bounded. The Realtime acknowledgement
return seam still governs whether the mandatory 51-row first vertical can
preserve effective and rejected truth. The next batch therefore has an
unresolved public API decision and fails the readiness rubric.

No milestone, card, handoff, worker, implementation, provider contact,
generation closeout, or PR 127 action is ready. The single next route is the
operator API decision above, followed by completion of exact route-local names
and a fresh readiness application.

## Authority

- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
- [g05.009](../roadmaps/g05/009-contract-061-consumer-projection-realization.md)
- [realization-readiness inventory](2026-08-31-contract-061-realization-readiness-inventory.md)
- [consumer projection census](2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [Contract 037](../contracts/037-prepared-consumer-integration.md)
- [Contract 047](../contracts/047-configured-provider-instance-catalogue.md)
