# Contract 061 Kimi Active-Observation Public-Baseline Gate

Status: complete; strict-ready; card 034 ready
Owner: Tom
Date: 2026-09-01
Source: operator decision, Card 033 evidence stop, Contract 061, and `main` at
`10fab5eac15f16199f8da51484f94a8a4755efeb`

## Purpose

Close candidate F's three `kimi-code.acp` post-open blockers without changing a
shared runtime type or transferring Kimi authority to another route. This is
planning evidence. It fixes the adapter-owned surface, retention rules, failure
boundary, and proof oracle for card 034; it does not implement Rust, contact a
provider, or authorize another candidate.

## Operator Decision

The operator answered all five decisions card 033 returned:

1. `swallowtail-runtime::EffectiveReasoningSetup` is unchanged. Exact Kimi
   reasoning and Plan effective and rejected values stay inside
   `swallowtail-adapter-kimi`.
2. Kimi uses a bounded adapter-local provider-value representation. Exact
   admitted or foreign confirmation tokens are retained only when they satisfy
   the adapter's exact non-blank byte bound. Foreign or over-bound tokens fail
   closed on the projected path and publish no active row. `open_session`
   keeps its current normalization, failures, handle, and cleanup behavior.
3. Only `KimiPreparedSession::open_session_with_projection` is added.
   `open_session`, `load_session`, and `resume_session` keep their exact public
   behavior. Both open methods share one private lifecycle.
4. A separate projected catalogue seam is added over
   `KimiPreparedSessionCatalogue::list_sessions`. `list_sessions`,
   `list_page`, `next_page_request`, and continuation paging are unchanged.
   Catalogue observation is exact session-management truth and never
   interactive-session truth.
5. Prepared and active-observation source IDs are distinct and
   caller-supplied. No runtime, testkit, core, contract, registry, callback,
   generic provider payload, or cross-route decision is added.

The decision is route-local to `kimi-code.acp`. It grants no authority to
`kimi-code.headless`, `kimi-code.local-server`, `kimi-platform.chat`, or any
other candidate.

## Exact Public Surface

`KimiPreparedSession::open_session`, `load_request`, `load_session`,
`resume_request`, `resume_session`, `prepare_working_state_restoration`, and
`into_parts` keep their current signatures, handles, failure codes, and cleanup
behavior. `KimiPreparedSessionCatalogue::list_sessions`, `list_page`,
`next_page_request`, `evidence`, `plan`, `request`, and `low_level_driver` keep
theirs. Card 034 adds exactly this public Kimi family:

```rust
pub struct KimiProviderValue { /* private fields */ }

impl KimiProviderValue {
    pub fn as_str(&self) -> &str;
}

pub enum KimiReasoningAcknowledgement {
    NotRequested,
    Effective(KimiProviderValue),
    Rejected(KimiProviderValue),
}

pub enum KimiPlanAcknowledgement {
    NotRequested,
    Effective(KimiProviderValue),
    Rejected(KimiProviderValue),
}

pub type KimiProjectionOpenFuture = BoxFuture<
    'static,
    Result<KimiProjectionOpenOutcome, KimiProjectionOpenFailure>,
>;

pub struct KimiProjectionOpenOutcome { /* private fields */ }

impl KimiProjectionOpenOutcome {
    pub fn session(&self) -> &dyn InteractiveSessionHandle;
    pub const fn contribution(&self) -> &ConsumerRouteProjectionContribution;
    pub const fn reasoning_acknowledgement(&self) -> &KimiReasoningAcknowledgement;
    pub const fn plan_acknowledgement(&self) -> &KimiPlanAcknowledgement;
    pub fn negotiated_model_options(&self) -> Option<&NegotiatedSessionModelOptions>;
    pub fn into_parts(
        self,
    ) -> (
        Box<dyn InteractiveSessionHandle>,
        ConsumerRouteProjectionContribution,
    );
}

pub enum KimiProjectionOpenFailure {
    Runtime(RuntimeFailure),
    Rejected {
        failure: RuntimeFailure,
        contribution: ConsumerRouteProjectionContribution,
        reasoning: KimiReasoningAcknowledgement,
        plan: KimiPlanAcknowledgement,
    },
}

impl KimiProjectionOpenFailure {
    pub const fn failure(&self) -> &RuntimeFailure;
    pub const fn rejected_contribution(
        &self,
    ) -> Option<&ConsumerRouteProjectionContribution>;
    pub fn into_parts(
        self,
    ) -> (RuntimeFailure, Option<ConsumerRouteProjectionContribution>);
}

impl KimiPreparedSession {
    pub fn open_session_with_projection(
        &self,
        prepared_source_id: ConsumerRouteProjectionSourceId,
        active_session_source_id: ConsumerRouteProjectionSourceId,
        services: HostServices,
    ) -> KimiProjectionOpenFuture;
}

pub type KimiCatalogueProjectionFuture = BoxFuture<
    'static,
    Result<KimiCatalogueProjectionOutcome, KimiCatalogueProjectionFailure>,
>;

pub struct KimiCatalogueProjectionOutcome { /* private fields */ }

impl KimiCatalogueProjectionOutcome {
    pub const fn outcome(&self) -> &ProviderSessionCatalogueOutcome;
    pub const fn contribution(&self) -> &ConsumerRouteProjectionContribution;
    pub fn into_parts(
        self,
    ) -> (
        ProviderSessionCatalogueOutcome,
        ConsumerRouteProjectionContribution,
    );
}

pub enum KimiCatalogueProjectionFailure {
    Operation(ProviderSessionOperationFailure),
    Projection(ConsumerRouteProjectionFailure),
}

impl KimiCatalogueProjectionFailure {
    pub const fn operation(&self) -> Option<&ProviderSessionOperationFailure>;
    pub const fn projection(&self) -> Option<&ConsumerRouteProjectionFailure>;
}

impl KimiPreparedSessionCatalogue {
    pub fn list_sessions_with_projection(
        &self,
        prepared_source_id: ConsumerRouteProjectionSourceId,
        observed_catalogue_source_id: ConsumerRouteProjectionSourceId,
        services: HostServices,
    ) -> KimiCatalogueProjectionFuture;
}
```

`KimiProviderValue::new` is not public. The type is constructed only by the
adapter's private open lifecycle, so no caller can manufacture an
acknowledgement value. The projected open never exposes the ACP response,
config-option object, command, path, credential, or any other provider payload.

The exact prepared contribution method established by cards 022-024 is added to
the candidate F prepared facades with no callback and no provider payload:

```rust
pub fn consumer_route_projection_contribution(
    &self,
    source_id: ConsumerRouteProjectionSourceId,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure>;
```

The contributing facades are:

- `kimi-code.acp` — `KimiPreparedSession`, `KimiPreparedSessionCatalogue`,
  `KimiPreparedSessionImport`
- `kimi-code.headless` — `KimiHeadlessPreparedRun`
- `kimi-code.local-server` — `KimiLocalServerPreparedCatalogue`,
  `KimiLocalServerPreparedRun`, `KimiLocalServerPreparedSession`,
  `KimiLocalServerPreparedArchive`, `KimiLocalServerPreparedRestore`,
  `KimiLocalServerPreparedReconciliation`,
  `KimiLocalServerPreparedBindingImport`
- `kimi-platform.chat` — `KimiPlatformPreparedCatalogue`,
  `KimiPlatformPreparedInferenceAttempt`

`KimiPreparedEvidence`, `KimiHeadlessPreparedEvidence`,
`KimiPlatformPreparedEvidence`, `KimiModelSelection`,
`KimiPlatformModelSelection`, `KimiAcpSessionImportAuthority`, and
`KimiLocalServerSessionConfiguration` remain inputs and evidence. They gain no
contribution method and prove no row by themselves. Only
`swallowtail-adapter-kimi` exports the projected-open and projected-catalogue
families.

## Bounded Adapter-Local Provider Value

`swallowtail-adapter-kimi` owns one private constant and one admission
function. Both live in the adapter; neither is shared, re-exported, or derived
from a runtime or core constant:

```rust
const MAXIMUM_KIMI_PROVIDER_VALUE_BYTES: usize = 128;
```

`128` is the same width `swallowtail-core`'s
`crates/swallowtail-core/src/model_catalog/value.rs` already uses for one
bounded provider-defined catalogue value. The gate reuses the number as
planning precedent only. Card 034 must declare its own adapter-local constant
and must not import, re-export, or alias
`ProviderCatalogValue`'s `MAX_PROVIDER_VALUE_BYTES`.

A confirmation token is retainable only when all of these hold:

- it is non-blank after trimming;
- `value.trim() == value`;
- it contains no control characters; and
- `value.len() <= MAXIMUM_KIMI_PROVIDER_VALUE_BYTES`.

Retention is not publication. A retained token publishes an active row only
when it is additionally *admitted* for the exact prepared
`KimiAcpBehavior`:

- `LegacyReasoning` — `off` or `on`;
- `DeclaredEffort` — `off`, `on`, `low`, `medium`, `high`, `xhigh`, or `max`;
- harness mode — `default`, `plan`, `auto`, or `yolo`, which
  `driver/mode.rs` already freezes as one exact ordered domain.

A token outside its half's admitted set is *foreign*. Foreign tokens and
unretainable tokens both fail closed on the projected path. Neither ever
becomes `ReasoningMode`, `NegotiatedReasoningSetup`, `EffectiveReasoningSetup`,
or any other portable value.

## Preserved Versus Projected Behavior

One private Kimi ACP open lifecycle serves both public open methods. It runs
exactly today's sequence — plan agreement, plan validation, negotiated
reasoning preparation, request validation, attachment start, initialize
validation, `session/new`, model validation, reasoning `set_config_option` and
`confirm`, Plan `set_config_option` and `confirm_plan_mode`, resume binding,
`take_session` — and additionally records, per half, the exact `currentValue`
string that half's confirmation carried, or nothing when the half was not
requested. Recording never changes control flow.

`open_session` maps the lifecycle result to today's public result and discards
both records. The byte bound and the admitted-set check never run on this path.
Its success, failure codes, handle type, and cleanup outcome are byte-identical
to current `main`, including the `DeclaredEffort` requested-`"on"`
normalization to `ReasoningMode::new("on")`.

`open_session_with_projection` maps the same lifecycle result in exactly this
order:

1. Lifecycle failed with `swallowtail.negotiated_reasoning.effective_mismatch`
   or `swallowtail.kimi.acp.harness_mode_mismatch`, and every recorded token is
   retainable and admitted — return `Rejected { failure, contribution,
   reasoning, plan }` carrying the same `RuntimeFailure` the preserved path
   returns, the compound acknowledgement row, and no session.
2. Lifecycle failed for any other reason, or a recorded token is unretainable
   or foreign — return `Runtime(failure)` with the exact failure the preserved
   path returns and no contribution.
3. Lifecycle succeeded and every recorded token is retainable and admitted —
   return the outcome with the prepared rows plus whichever active rows exist.
4. Lifecycle succeeded but a recorded token is unretainable or foreign — close
   the opened session, return `Runtime` with
   `swallowtail.kimi.acp.reasoning_value_unbounded` or
   `swallowtail.kimi.acp.reasoning_value_foreign`, and publish no contribution.

Case 4 is the only place the projected path may differ from a successful
preserved open, and it fails closed. Cases 1 and 2 keep exact route-code and
cleanup parity with `open_session` for every fixture.

## Compound Reasoning-And-Plan Acknowledgement

The two halves are sourced independently and never substitute for each other.

**Reasoning half.** Under a requested `ReasoningMode` `R`, prepared behavior
`B`, and confirmation `currentValue` `C` that `parse_option` and
`validate_behavior_shape` already accepted:

- reasoning not requested — `NotRequested`; no half, no row contribution, no
  active source;
- `B = LegacyReasoning`, `C == R` — `Effective(C)`;
- `B = LegacyReasoning`, `C != R` — `Rejected(C)`, preserving
  `swallowtail.negotiated_reasoning.effective_mismatch`;
- `B = DeclaredEffort`, `R == "on"`, `C == "off"` — `Rejected("off")`,
  preserving the same mismatch failure;
- `B = DeclaredEffort`, `R == "on"`, `C != "off"` — `Effective(C)` carrying the
  **exact provider token**, not the normalized `"on"`. The preserved path still
  succeeds with its normalized `EffectiveReasoningSetup`; the projected path
  publishes the exact effort the provider confirmed;
- `B = DeclaredEffort`, `R` concrete, `C == R` — `Effective(C)`;
- `B = DeclaredEffort`, `R` concrete, `C != R` — `Rejected(C)`, preserving the
  same mismatch failure; and
- missing, malformed, duplicated, ambiguous, unadvertised, transport, setup,
  unretainable, or foreign confirmation — no half; ordinary `Runtime` with no
  contribution.

**Plan half.** `driver/mode.rs` freezes `["default", "plan", "auto", "yolo"]`
in listed order and rejects any other shape as malformed, so the Plan token is
always admitted and always within the byte bound:

- Plan not requested — `NotRequested`;
- `currentValue == "plan"` — `Effective("plan")`; and
- `currentValue` exactly `default`, `auto`, or `yolo` — `Rejected(value)`,
  preserving `swallowtail.kimi.acp.harness_mode_mismatch`.

Every other Plan outcome is ordinary `Runtime` with no contribution.

**One row from two halves.** The bounded namespaced
`feature.active-session-reasoning-and-plan-ack` row is published when at least
one half is not `NotRequested`. It carries:

- `ConsumerRouteValueKind::AcknowledgementState`;
- `ConsumerRouteValueDomain::Enumerated` with exactly one entry per
  non-`NotRequested` half, in fixed order — reasoning first, then Plan — each
  entry rendered as `reasoning=<token>` or `plan=<token>` from the exact
  retained `KimiProviderValue`;
- `ConsumerRouteOmissionSemantics::NotSelectable`;
- `ConsumerRouteSourceClass::RouteAcknowledgementEvidence` and
  `ConsumerRouteEvidenceStrength::WireAcknowledgement`;
- `ConsumerRouteLifecycle::PostOpenObservationOnly`;
- `ConsumerRouteActorPosture::ObservationOnly` and
  `ConsumerRouteMutationAuthority::Acknowledged(active_session_source_id)`; and
- state support `with_requested()`, plus `with_provider_effective()` if any
  half is `Effective`, plus `with_rejected()` if any half is `Rejected`.

The exact typed halves stay reachable through
`reasoning_acknowledgement()`, `plan_acknowledgement()`, and the `Rejected`
failure fields, so the rendered string domain is never the only carrier. Both
halves omitted produces no row and no retained active source.

## Negotiated Model-Option Observation

`driver/validation.rs` `parse_model_options` already extracts one exact `model`
config option — one current value plus the optional bounded advertised list
with optional display names — and `driver.rs` threads it into
`attachment.take_session(...)` on open, load, and resume.
`KimiSessionHandle::negotiated_model_options()` already returns
`Option<&NegotiatedSessionModelOptions>`.

That behavior does not change. Unlike `cline.acp`, Kimi already fails
`swallowtail.kimi.acp.malformed_response` for a missing or malformed `model`
option on both paths, so there is no preserved-versus-projected model split to
create and card 034 must not create one.

The bounded namespaced `feature.negotiated-model-options-observation` row is
published only when the snapshot is `Some`. It carries
`ConsumerRouteValueKind::Observation`, an unenumerated descriptor domain,
`NotSelectable`, `with_observed()` state, wire-acknowledgement evidence,
`ObservationOnly` posture, and no mutation authority.
`NegotiatedSessionModelOptions::new` remains the count, text, uniqueness, and
current-membership authority; the projection never flattens that typed
snapshot into an ambiguous string.

## Projected Provider-Session Catalogue Observation

Catalogue observation is a third seam shape, not a second copy of the open
seam. It is bound to `KimiPreparedSessionCatalogue`, whose plan carries
`OperationShape::ProviderSessionCatalogue`,
`DriverRole::ProviderSessionCatalogue`, `Capability::ProviderSessionCatalogue`,
and no model route.

`list_sessions_with_projection` calls the preserved `list_sessions` path
verbatim, then projects only the fact of the completed bounded query:

- the preserved outcome's `candidates()`, `next_cursor()`, and `cleanup()`
  behavior is unchanged and is returned intact through `outcome()`;
- the bounded namespaced `control.provider-session-catalogue` row carries
  `ConsumerRouteValueKind::Observation`, a bounded-query descriptor domain,
  optional-observation omission semantics, `with_observed()` state,
  `RouteAcknowledgementEvidence`, `WireAcknowledgement`,
  `PostOpenObservationOnly` lifecycle, `ObservationOnly` posture, and no
  mutation authority;
- the row's applicability comes from the catalogue plan, so its operation shape
  is `ProviderSessionCatalogue` and never `InteractiveSession`;
- its source is `ActiveSessionObservation(observed_catalogue_source_id)`, which
  must differ from both catalogue-prepared and interactive-session source IDs;
- any `ProviderSessionOperationFailure` — including a cancelled, timed-out,
  dispatch, projection, or cleanup stage — returns
  `KimiCatalogueProjectionFailure::Operation` with no contribution; and
- a contribution that cannot be admitted returns
  `KimiCatalogueProjectionFailure::Projection` with no partial row.

`list_page`, `next_page_request`, and continuation paging stay preserved-only
and gain no projection authority. Prepared catalogue success alone never
publishes the observed half:
`PreparedProviderSessionCatalogueEvidence` contributes the prepared
`feature.provider-session-catalogue` row and nothing more.

## Projection Semantics

Prepared and active sources are caller-supplied and must differ. Equal IDs fail
before any process, connection, or provider work with
`swallowtail.kimi.projection_source_identity_invalid`.

Prepared selection and session-start rows use
`AdapterContribution(prepared_source_id)`. Post-open acknowledgement and model
observation use `ActiveSessionObservation(active_session_source_id)`. Post-open
catalogue observation uses
`ActiveSessionObservation(observed_catalogue_source_id)`. An active source that
names no published active row is omitted from the contribution.

All three Kimi-only active identities are bounded namespaced extensions
qualified by exact route ID and exact `protocol_facade_id`. The prepared
`control.provider-session-catalogue` and `control.provider-session-import`
identities are also bounded namespaced extensions; the closed
`ConsumerRouteControlId::SessionCatalogueBounds` names a different census row
and must not be substituted.

The `kimi-code.acp` prepared `control.reasoning-selection` row is requested,
prepared, and pending at session start. It never carries provider-effective or
rejected state. `control.session-options` names exactly the accepted Kimi
subset — portable reasoning mode and Plan harness mode — and never projects the
developer-instruction or tool fields `validate_options` rejects. Omission
creates no row and no Swallowtail default.

## Operation-Shape-Scoped Persistence

`kimi-code.acp` carries two distinct provider-state policies by operation
shape, and the census carries exactly one
`feature.persistent-session-posture` row for the route:

- `prepared_profile/plan.rs` sets `SessionProviderStatePolicy::Prohibited` on
  the interactive-session plan, so `KimiPreparedSession` must never emit the
  persistence row; and
- `prepared_profile/provider_session_catalogue.rs` sets
  `DurableProviderSessionPreserved` and requires
  `Capability::ProviderDurableRetention` on the session-import plan, so
  `KimiPreparedSessionImport` is the sole emitter.

Card 032's route-scoped persistence withholding therefore becomes
operation-shape-scoped on this route. Documentation cannot override either
policy.

## Candidate F Readiness

The gate closes Batch 9.4 rubric items 2 and 3 for the complete two-package
candidate. Card 034 owns 89 exact census tuples across four routes. Every
disposition below is derived from the reviewed census plus current `main`
capability profiles, driver roles, and prepared policies — not from the matrix
and not from card 033's provisional 86/3 split:

| Route | Census | Emitted | Withheld |
| --- | ---: | ---: | ---: |
| `kimi-code.acp` | 25 | 22 | 3 |
| `kimi-code.headless` | 20 | 10 | 10 |
| `kimi-code.local-server` | 31 | 31 | 0 |
| `kimi-platform.chat` | 13 | 12 | 1 |
| **Total** | **89** | **75** | **14** |

The 14 construction-time withholdings, each with its exact reason:

| Route | Row | Withheld because |
| --- | --- | --- |
| `kimi-code.acp` | `feature.model-catalogue` | `kimi_acp_descriptor()` carries no `DriverRole::ModelCatalog` and no ACP prepared plan requires `Capability::ModelCatalog` |
| `kimi-code.acp` | `feature.structured-run` | the ACP descriptor declares no `StructuredRun` role or shape; `KimiCodePreparedIntegration` selects a route and proves no run |
| `kimi-code.acp` | `feature.provider-managed-recovery` | no ACP prepared plan requires `Capability::ProviderManagedRecovery`; `PreparedWorkingStateRestoration` is load-session continuation authority |
| `kimi-code.headless` | `feature.model-catalogue` | `kimi_headless_descriptor()` carries only `Discovery` and `StructuredRun` |
| `kimi-code.headless` | `feature.interactive-session` | matrix-only; `run_capabilities()` has no `InteractiveSession` |
| `kimi-code.headless` | `feature.reasoning-selection` | matrix-only; the headless run plan requires no `ReasoningSelection` |
| `kimi-code.headless` | `feature.load-session` | matrix-only; no `LoadSession` capability and no public load operation |
| `kimi-code.headless` | `feature.resume-session` | matrix-only; no `Resume` capability and no public resume operation |
| `kimi-code.headless` | `feature.provider-session-catalogue` | matrix-only; no catalogue role, capability, or facade |
| `kimi-code.headless` | `feature.provider-session-import` | matrix-only; no import role, capability, or facade |
| `kimi-code.headless` | `feature.bounded-workspace-text-write` | matrix-only; `run_capabilities()` has no `WorkingResourceTextWrite` |
| `kimi-code.headless` | `control.load-session` | census `matrix-descriptor-only`; the route exposes no public session input |
| `kimi-code.headless` | `control.resume-session` | census `matrix-descriptor-only`; the route exposes no public session input |
| `kimi-platform.chat` | `feature.cancellation-or-interruption` | `inference_capabilities()` requires no `Capability::Interruption` |

`ConsumerRouteEvidenceStrength` has no documentation or QA-matrix variant, so
every one of these rows is unprojectable on current `main` by construction, not
by preference.

Nothing else is withheld. In particular `feature.model-catalogue` is emitted on
`kimi-code.local-server` and `kimi-platform.chat`, where
`KimiLocalServerPreparedCatalogue` and `KimiPlatformPreparedCatalogue` each
carry `DriverRole::ModelCatalog` and `Capability::ModelCatalog`. The route
carries no `audit.no-public-route-specific-selectable-control` row and no
per-turn row, so candidate F adds neither a negative-coverage family nor
consumer-mediated authority.

Six rows are maximal-only and are absent from a minimal prepared profile:
`kimi-code.acp` `feature.reasoning-selection` and `control.reasoning-selection`;
`kimi-code.local-server` `feature.reasoning-selection`,
`feature.stream-reattachment`, `feature.permission-exchange`, and
`feature.question-exchange`. `kimi-code.local-server`
`feature.owned-runtime-lifecycle` is emitted only from the
`HostOwnedEphemeral` owned topology, never from the `ExternalAttached`
interactive session.

No shared public decision, provider contact, live probe, callback, registry,
runtime enumeration, generic provider payload, or contract amendment is needed.
Two exact packages fit the normal focused-validation maximum. Candidate F
therefore passes the promotion rubric as card 034.

## Semantic API Boundary

Only `swallowtail-adapter-kimi` and `swallowtail-adapter-kimi-platform` change
their semantic API baselines. `swallowtail-adapter-kimi-platform` already sits
in `release-baselines/public-api-unreleased/packages.txt`;
`swallowtail-adapter-kimi` does not and must be added there with its own
unreleased baseline file, exactly as card 032 added Command Code, Copilot CLI,
and Goose. No `swallowtail-runtime`, `swallowtail-core`, `swallowtail-testkit`,
`swallowtail-host-local`, or other adapter baseline may move. Contracts,
architecture, the census, provider claims, and compatibility claims may not
change.

## Review Oracle

Invariant: only exact prepared evidence, one exact `kimi-code.acp` open
observation, or one exact `kimi-code.acp` catalogue observation may publish a
candidate F row. Session existence, prepared success, documentation, a static
diagnostic, or another route cannot substitute.

Counterexamples and required proof:

- a foreign or over-bound confirmation token published as portable reasoning
  truth — fail; the projected path returns `Runtime` with
  `reasoning_value_foreign` or `reasoning_value_unbounded`, no contribution,
  and the preserved path is unchanged
- requested `"on"` under `DeclaredEffort` publishing `"on"` instead of the
  exact provider-confirmed effort — fail the exact-effective proof
- a rejection published from `swallowtail.negotiated_reasoning.effective_mismatch`
  or `swallowtail.kimi.acp.harness_mode_mismatch` without the retained exact
  confirmation value — fail as `Runtime` with no contribution
- `EffectiveReasoningSetup`, `NegotiatedReasoningSetup`, `ReasoningMode`, or
  any other shared public type changed or widened — fail
- the preserved and projected open paths differing in setup order, route
  failure code, handle shape, or cleanup outcome for the same fixture — fail
  the shared-lifecycle proof
- `load_session`, `resume_session`, or `prepare_working_state_restoration`
  gaining projection authority from the open-only decision — fail
- prepared catalogue success presented as observed catalogue truth — fail; only
  a completed `list_sessions_with_projection` may set `observed`
- catalogue observation carrying the interactive-session active source ID or
  `OperationShape::InteractiveSession` applicability — fail
- `list_page`, `next_page_request`, or a continuation cursor publishing a
  catalogue row — fail; the seam covers `list_sessions` only
- model options inferred from session existence, or presented as selectable,
  mutable, acknowledged, or catalogue authority — fail the observation-only
  posture
- a preserved-versus-projected model-option split introduced where current
  `main` fails both paths identically — fail; `open_session` behavior is fixed
- a rejected acknowledgement contribution retaining a session or a model-option
  row — fail; the open did not complete
- omitted reasoning and omitted Plan producing an acknowledgement row or an
  unused active-observation source — fail
- equal prepared and active source IDs reaching process, connection, or
  provider work — fail before dispatch
- `feature.persistent-session-posture` emitted from the `Prohibited`
  interactive-session plan, or withheld despite the
  `DurableProviderSessionPreserved` import plan — fail the operation-shape
  split
- a `kimi-code.acp` active row reaching `kimi-code.headless`,
  `kimi-code.local-server`, or `kimi-platform.chat` — fail route and operation
  applicability
- a matrix-only row emitted on any of the 14 withheld tuples — fail; no
  evidence-strength variant admits documentation
- any route ledger reaching 25, 20, 31, or 13 through a filter, exception list,
  duplicate tuple, or borrowed identity — fail exact tuple reconciliation
- matching-source cross-route, cross-operation, cross-instance, stale-revision,
  or cross-access mixture accepted — fail closed in both directions

## Validation Boundary

Card 034 names exactly these packages:

- `swallowtail-adapter-kimi`
- `swallowtail-adapter-kimi-platform`

It adds package-scoped formatting, focused validation, extracted-package,
semantic API, route, docs, Northstar, god-file, and diff checks. No provider
contact or live probe belongs to the card.

## Authority

- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
- [Batch 9.4 package expansion](2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [completed card 033](../roadmaps/g05/batch-cards/033-contract-061-card-032-closeout-and-kimi-reassessment.md)
- [reviewed census](2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [Batch 9.1 public baseline](2026-08-31-contract-061-batch-9-1-public-baseline-gate.md)
- [Claude Agent acknowledgement gate](2026-08-31-contract-061-claude-agent-acknowledgement-public-baseline-gate.md)
- [Cline active-observation gate](2026-09-01-contract-061-cline-active-observation-public-baseline-gate.md)
