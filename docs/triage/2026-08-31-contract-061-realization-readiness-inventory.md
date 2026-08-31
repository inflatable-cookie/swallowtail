# 2026-08-31 Contract 061 Realization-Readiness Inventory

Status: operator decision required; no ready card
Owner: Tom
Source: Contract 061 and the reviewed consumer projection census

## Purpose

Test whether Contract 061 has enough realized substrate for a bounded
implementation tranche. This is planning evidence. It does not establish a
Rust API, implementation milestone, card, or worker handoff.

The inventory uses the merged tree at
`ee12e23bc440202ca7155353c101b66d199aad4a`. It does not contact providers or
change Contracts 037, 047, 057, or 061.

## Current Source Ownership

No current package owns the Contract 061 composition. The source truth is
already split along the existing dependency boundary:

| Source class | Current public records | Realized owner | Inventory finding |
| --- | --- | --- | --- |
| provider-neutral identities and applicability vocabulary | `ConfiguredInstanceId`, `InstanceRevision`, `AdapterIdentity`, `OperationShape`, `Capability`, `CapabilityProfile`, `CapabilityRequirement`, `AccessStatus`, `ModelCatalogEntry` | `swallowtail-core` | owns vocabulary, not projection assembly |
| Contract 057 route admission and presentation | `AddableRouteDescriptor`, `AdmittedInstanceRecord`, authenticated-subject and overlay marker records; `AddableRouteCatalog`, connection store, readiness refresh, and model overlay | records in `swallowtail-core`; assembly in `swallowtail-runtime`; adapter-local descriptor constructors where realized | remains pre-session admission and presentation; it cannot become route-feature authority |
| Contract 047 selection snapshot | `ConfiguredProviderInstanceCatalogue`, `ConfiguredProviderInstanceRecord`, `ConfiguredProviderInstanceRoute`, `ConfiguredProviderModelCatalogue` | `swallowtail-runtime` | already binds instance revision, driver, operation shape, capability requirements, access posture, and optional catalogue result |
| Contract 037 prepared truth | `PreflightPlan` and requirements in `swallowtail-core`; `PreparedOperationEvidence` and `PreparedAccessEvidence` in `swallowtail-runtime` | core plus runtime, embedded by adapter-local prepared facades | supplies the exact prepared binding and fail-closed agreement surface |
| common session and negotiated state | `SessionOptions`, `NegotiatedSessionModelOptions`, negotiated reasoning setup/effective records | `swallowtail-runtime` | covers common inputs and some post-open evidence, not every route-specific value or acknowledgement |
| route-specific control truth | model selectors, prepared inputs, configuration records, constructor validation, and acknowledgement parsers | 31 adapter packages | owns accepted domains, omission behavior, version qualification, and provider/wire acknowledgement that cannot be inferred from common records |
| portable conformance | prepared, catalogue, lifecycle, and route fixtures/assertions | `swallowtail-testkit` | already depends on core and runtime; adapters consume it as a development dependency |

The census confirms the split. Its 767 rows cover 48 production route IDs:
555 features, 203 controls, and 9 explicit no-route-specific-control audits.
The largest common class is 499 capability-state rows. Route-specific control
domains and the exact acknowledgement paths remain adapter-owned.

## Dependency Direction

The realized production import edges point inward:

`adapter packages` -> `swallowtail-runtime` -> `swallowtail-core`

`swallowtail-testkit` depends on core and runtime. Adapter test targets depend
on testkit. Core and runtime do not depend on adapter packages.

Vocabulary and admitted evidence flow back outward from core through runtime to
the adapters. The import direction closes several choices:

- runtime composition cannot import adapter-specific control types;
- core cannot compose Contract 047, prepared-operation, session-option, or
  negotiated-session runtime records;
- a runtime adapter registry would reverse the current graph and violate the
  Contract 061 boundary;
- portable assertions may live in testkit while adapter-local fixtures prove
  their own contributions without a production dependency cycle.

The remaining package decision is therefore narrow: either keep the shared
descriptor records and fail-closed composer together in `swallowtail-runtime`,
or split new provider-neutral descriptor vocabulary into core while runtime
owns composition. A new umbrella crate or adapter registry has no supporting
evidence.

Recommendation: keep the first public projection family in
`swallowtail-runtime`. Adapters should construct runtime-owned, normalized
contributions from their prepared facades. Testkit should own portable
conformance assertions. Core should retain its current vocabulary and avoid a
new dependency on projection lifecycle.

## Public API Gap

The current types do not yet provide one admissible input to a composer:

- Contract 047 and `PreparedOperationEvidence` can be compared for instance,
  revision, driver, host, operation shape, access, model route, and interface
  agreement.
- adapter-local prepared values retain the exact route-specific controls and
  validation that a shared package cannot import.
- post-open option snapshots and acknowledgement records are exposed through
  route-local handles; they are not a uniform mutation authority.
- not every source record has a common evidence-identity field. Existing exact
  binding fields can identify much of the snapshot, but content equality alone
  cannot name which otherwise-equal source observation replaced another.

A public baseline therefore needs one normalized adapter contribution boundary
and one source-identity rule before a card can be ready. Consumer assembly of
linked contributions is compatible with the existing Contract 057 pattern;
consumer merging of feature/control semantics is not.

Recommendation: use immutable runtime-owned contribution records accepted by
one pure, fail-closed composer. Adapter-local prepared facades emit those
records. The consumer supplies only the contributions for linked adapters and
the exact current source records. Do not use adapter downcasts, callbacks into
adapter code, runtime enumeration, or a generic provider payload.

Operator API fork:

1. approve the runtime-owned contribution/composer baseline and require an
   explicit typed identity for each replaceable source observation; or
2. require a different source-identity rule or a core/runtime split before API
   planning continues.

The first choice is recommended. Exact Rust names and signatures remain
unplanned.

## Projection Bounds

Inherited source bounds are not sufficient for a public projection baseline.

Realized numeric bounds include 256 configured instances, 64 prepared routes
per instance, 10,000 model entries per instance, and 256 negotiated model
options with 256-byte option text. Other composed inputs do not provide a
uniform numeric boundary: capability profiles and constraints collect
iterators without a count cap; several identity and safe-diagnostic strings
reject blank text but do not cap bytes; Contract 057 catalogs and descriptor
field collections have no shared numeric maximum.

The 767-row census is a repository-wide evidence set, not one projection
snapshot. One exact route has at most 36 census rows today. Neither number is a
portable future bound.

Recommendation: require projection-specific admission bounds before the public
API baseline. Bound at least rows per view, enumerable values per control,
namespaced extension count and text, and copied safe-reason bytes. Prefer fixed
library maxima so identical source evidence cannot be admitted by one consumer
and rejected by another. Do not derive the numbers from the global 767-row
census or widen unbounded source text silently.

Operator bound fork:

1. approve fixed projection admission maxima and authorize a later planning
   step to select exact values from per-route high-water marks plus explicit
   headroom; or
2. accept caller-supplied positive limits, with the resulting non-uniform
   portability, and define a library ceiling.

The first choice is recommended. No numeric values are selected by this
inventory.

## All-Route Coverage

The census is complete enough to define coverage, not to become runtime
authority:

- 48 exact route IDs across 31 adapter packages;
- 6 routes cover all four observed census lifecycle classes, 30 cover three,
  and 12 cover two;
- 9 routes have no route-specific consumer control and must retain that
  explicit absence;
- 9 rows are per-turn;
- 5 rows on 4 routes carry exact
  requested/pending/effective/rejected evidence;
- no row proves mid-turn negotiation.

All-route acceptance must disposition every census row by its exact route,
operation shape, semantic ID, and lifecycle. It must not require production
runtime enumeration. Portable testkit assertions can validate the common
composer; each adapter package can prove its own normalized contributions and
explicit absences. A final repository audit can compare the adapter-local
coverage ledger with the 767-row census.

## First Meaningful Tranche

A common-kernel-only tranche would not prove adapter contribution. A
single-route vertical would also miss a material Contract 061 boundary: no
current route combines the census's per-turn evidence with an exact
requested/pending/effective/rejected acknowledgement path.

The smallest evidence-backed tranche is:

1. runtime-owned immutable projection records, contribution admission, the
   three views, and all four fail-closed Contract 061 points;
2. testkit conformance for identity disagreement, applicability disagreement,
   absent mutation authority, unbounded reasons, and bounded collections;
3. `codex.app-server` as the four-lifecycle route proof, including its per-turn
   exchange without inferred mutation; and
4. `openai.realtime` as the exact acknowledgement proof, keeping requested,
   pending, effective, and rejected state distinct.

These two routes cover 51 census rows. They do not authorize claims for the
remaining 716 rows. Later package-coherent batches would add adapter-local
contributions, followed by one all-route census audit. The 9 no-control routes
remain required negative coverage, not missing work.

Operator tranche and coverage fork:

1. approve the two-route vertical followed by package-coherent expansion and
   one final 767-row audit; or
2. choose selection-summary breadth across all routes first, accepting that it
   defers the route-specific control and acknowledgement boundary.

The first choice is recommended. A one-route implementation card does not meet
the evidence bar.

## Readiness Verdict

Posture: `strict-paused`.

Planning verdict: `materially ambiguous`. Contract 061 has enough realized
substrate for a bounded implementation lane, but not for a ready card. Package
placement, the contribution/source-identity API baseline, projection-bound
policy, and first-tranche/coverage sequence are material operator decisions.

The single next route is an operator intent decision on those four forks. If
the operator accepts the recommended set, the next planning action is roadmap
compilation. If a recommendation is rejected, refocus only that fork before
compiling a roadmap. Execution remains blocked either way until the ordinary
roadmap, card, validation, and handoff gates are satisfied.

No milestone, card, worker handoff, implementation authority, provider contact,
generation closeout, blocked-surface restart, or PR 127 action follows from
this inventory.

## Authority

- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
- [post-Contract-061 reassessment](../logs/2026-08-31-g05-post-contract-061-reassessment.md)
- [consumer projection census synthesis](2026-08-30-consumer-route-feature-and-option-projection.md)
- [consumer projection census](2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [Contract 037](../contracts/037-prepared-consumer-integration.md)
- [Contract 047](../contracts/047-configured-provider-instance-catalogue.md)
- [Contract 057](../contracts/057-route-readiness-and-connection-admission.md)
