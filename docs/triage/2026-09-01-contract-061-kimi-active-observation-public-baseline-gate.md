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
    RequestedNotObserved,
    Effective(KimiProviderValue),
    Rejected(KimiProviderValue),
}

impl KimiReasoningAcknowledgement {
    pub const fn observed_value(&self) -> Option<&KimiProviderValue>;
}

pub enum KimiPlanAcknowledgement {
    NotRequested,
    RequestedNotObserved,
    Effective(KimiProviderValue),
    Rejected(KimiProviderValue),
}

impl KimiPlanAcknowledgement {
    pub const fn observed_value(&self) -> Option<&KimiProviderValue>;
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
    SourceIdentity(RuntimeFailure),
    Operation(ProviderSessionOperationFailure),
    Projection(ConsumerRouteProjectionFailure),
}

impl KimiCatalogueProjectionFailure {
    pub const fn source_identity(&self) -> Option<&RuntimeFailure>;
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

Two variants exist because current `main` cannot always produce both halves.

`RequestedNotObserved` is load-bearing, not defensive. `driver.rs` confirms
reasoning first and Plan second, and the gate preserves that order and control
flow exactly. When a maximal request asks for both halves and reasoning
rejects, the lifecycle returns before `mode::prepare_plan_mode`, so the Plan
control was genuinely requested and genuinely never observed.
`NotRequested` would be false, and `Effective` or `Rejected` would be invented
truth. `RequestedNotObserved` states exactly what happened without continuing
provider work or reordering confirmations. It is defined on both halves so a
later order change needs no new type.

`KimiCatalogueProjectionFailure::SourceIdentity` is load-bearing for the same
reason. Source-identity preflight runs before any catalogue dispatch, so it
cannot honestly be an `Operation` failure, and
`ConsumerRouteProjectionFailure` has no public constructor — only
`pub(super) fn failure` inside `swallowtail-runtime` — so the adapter cannot
build one carrying an adapter-specific diagnostic. `RuntimeFailure::new` is
public and is what the adapter's existing `crate::failure::failure` already
uses, so `SourceIdentity(RuntimeFailure)` is the one shape that can truthfully
carry `swallowtail.kimi.projection_source_identity_invalid` before dispatch.
The variants are ordered by when they can occur.

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
unretainable tokens both fail closed on the projected path, through one of the
two disjoint branches fixed under **Which Branch A Foreign Or Unretainable
Token Takes** below. Neither ever becomes `ReasoningMode`,
`NegotiatedReasoningSetup`, `EffectiveReasoningSetup`, or any other portable
value.

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
   or `swallowtail.kimi.acp.harness_mode_mismatch`, and every token it actually
   recorded is retainable and admitted — return `Rejected { failure,
   contribution, reasoning, plan }` carrying the same `RuntimeFailure` the
   preserved path returns, the compound acknowledgement row, and no session. A
   half the lifecycle never reached records no token and blocks nothing here;
   it is `RequestedNotObserved`.
2. Lifecycle failed for any other reason, **or** lifecycle failed with one of
   those two codes but a token it recorded is unretainable or foreign — return
   `Runtime(failure)` with the exact failure the preserved path returns and no
   contribution.
3. Lifecycle succeeded and every recorded token is retainable and admitted —
   return the outcome with the prepared rows plus whichever active rows exist.
4. Lifecycle succeeded but a recorded token is unretainable or foreign — close
   the opened session, return `Runtime` with
   `swallowtail.kimi.acp.reasoning_value_unbounded` or
   `swallowtail.kimi.acp.reasoning_value_foreign`, and publish no contribution.

### Which Branch A Foreign Or Unretainable Token Takes

Foreign and unretainable tokens reach the projected path through two disjoint
branches, and the two adapter codes belong to exactly one of them. Card 034
must prove both separately; neither may be described with blanket wording.

**Pre-lifecycle — case 2.** When the requested mode is concrete, current
`main` already compares it against the confirmation. `LegacyReasoning` and
`DeclaredEffort` with a concrete request both pass `currentValue` straight into
`NegotiatedReasoningSetup::confirm`, which returns
`swallowtail.negotiated_reasoning.effective_mismatch` whenever it differs from
the request. A foreign or over-bound confirmation therefore differs by
construction, the lifecycle has already aborted and cleaned up, and **no
session exists to close**. The projected path returns that exact preserved
`RuntimeFailure` unchanged with no contribution. It must not substitute
`reasoning_value_foreign` or `reasoning_value_unbounded` here, because the new
adapter code would misreport a failure the preserved path already owns and
would break route-code parity.

**Projection-only — case 4.** The lifecycle succeeds while holding a token the
projection cannot publish in exactly one situation: `DeclaredEffort` with
requested `"on"` and any non-`"off"` confirmation. `confirm` normalizes that
confirmation to `"on"`, so `NegotiatedReasoningSetup::confirm` sees requested
equal to effective and the open completes. The recorded token, however, is the
provider's exact effort and may be a foreign catalogue row or exceed the byte
bound. This is the only branch where the new adapter codes are correct: close
the opened session and return `reasoning_value_foreign` or
`reasoning_value_unbounded` with no contribution, while `open_session` still
succeeds on the identical fixture.

Every other combination is unreachable. `LegacyReasoning` confirmations are
constrained to `{off, on}` by `validate_behavior_shape`; concrete
`DeclaredEffort` requests only succeed on an exact match with an
already-admitted short identifier; and `driver/mode.rs` freezes the Plan domain
to `["default", "plan", "auto", "yolo"]`, so no Plan token can be foreign or
over-bound. Case 4 is consequently a reasoning-only, requested-`"on"`-only
branch, and card 034 must name it that precisely rather than as a general
fallback.

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
- reasoning requested but never confirmed because an earlier step aborted the
  lifecycle — `RequestedNotObserved`; no token, no domain entry, no
  effective or rejected state;
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
- Plan requested but never confirmed because reasoning rejected or otherwise
  terminated the lifecycle first — `RequestedNotObserved`;
- `currentValue == "plan"` — `Effective("plan")`; and
- `currentValue` exactly `default`, `auto`, or `yolo` — `Rejected(value)`,
  preserving `swallowtail.kimi.acp.harness_mode_mismatch`.

Every other Plan outcome is ordinary `Runtime` with no contribution.

**Fixed confirmation order and the early stop.** `driver.rs` confirms reasoning
at the `selection.confirm(...)` call and Plan afterwards, and this gate
preserves that order, that control flow, and the `?` propagation exactly. A
maximal request that asks for both halves and rejects on reasoning therefore
returns before `mode::prepare_plan_mode` runs. The Plan half is
`RequestedNotObserved`: no further provider work is performed to discover what
Plan would have done, and no Plan truth is invented. The symmetric case —
reasoning `Effective` and Plan requested but terminated by a malformed or
missing Plan option — is also `RequestedNotObserved`, though it takes case 2
and publishes nothing.

**One row from two halves.** The bounded namespaced
`feature.active-session-reasoning-and-plan-ack` row is published only when at
least one half is **observed**, that is `Effective` or `Rejected`. A
contribution in which every requested half is `RequestedNotObserved` publishes
no row and retains no active source. It carries:

- `ConsumerRouteValueKind::AcknowledgementState`;
- `ConsumerRouteValueDomain::Enumerated` with exactly one entry per **observed**
  half, in fixed order — reasoning first, then Plan — each entry rendered as
  `reasoning=<token>` or `plan=<token>` from the exact retained
  `KimiProviderValue`. A `RequestedNotObserved` half contributes no entry, so
  the domain never asserts that half was effective, rejected, or absent;
- `ConsumerRouteOmissionSemantics::NotSelectable`;
- `ConsumerRouteSourceClass::RouteAcknowledgementEvidence` and
  `ConsumerRouteEvidenceStrength::WireAcknowledgement`;
- `ConsumerRouteLifecycle::PostOpenObservationOnly`;
- `ConsumerRouteActorPosture::ObservationOnly` and
  `ConsumerRouteMutationAuthority::Acknowledged(active_session_source_id)`; and
- state support `with_requested()` if any half is not `NotRequested`, plus
  `with_pending()` if any half is `RequestedNotObserved`, plus
  `with_provider_effective()` if any half is `Effective`, plus
  `with_rejected()` if any half is `Rejected`. This is the census's own
  `requested; pending; effective; rejected` vocabulary, so an unobserved
  requested half is reported as still pending rather than silently dropped.

The exact typed halves stay reachable through `reasoning_acknowledgement()`,
`plan_acknowledgement()`, their `observed_value()` accessors, and the
`Rejected` failure fields, so the rendered string domain is never the only
carrier and a consumer can always distinguish `NotRequested` from
`RequestedNotObserved`. Both halves omitted produces no row and no retained
active source.

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
  must differ from the catalogue-prepared ID supplied on the same call;
- equal IDs on that call return
  `KimiCatalogueProjectionFailure::SourceIdentity` carrying
  `swallowtail.kimi.projection_source_identity_invalid` before any catalogue
  dispatch, so no process, connection, or provider work occurs;
- any `ProviderSessionOperationFailure` — including a cancelled, timed-out,
  dispatch, list, projection, or cleanup stage — returns
  `KimiCatalogueProjectionFailure::Operation` with no contribution; and
- a contribution that cannot be admitted after a completed operation returns
  `KimiCatalogueProjectionFailure::Projection` with no partial row.

The three failure variants are disjoint and ordered by when they can occur:
`SourceIdentity` before dispatch, `Operation` during the preserved catalogue
call, `Projection` only after that call completed. No variant may stand in for
another.

`list_page`, `next_page_request`, and continuation paging stay preserved-only
and gain no projection authority. Prepared catalogue success alone never
publishes the observed half.
`KimiPreparedSessionCatalogue::consumer_route_projection_contribution`
contributes the prepared `feature.provider-session-catalogue` row and nothing
more; it must not emit `control.provider-session-catalogue` in any state. The
census gives that control lifecycle `post-open-observation-only` and state
`descriptor-only; observed`, so emitting it from
`PreparedProviderSessionCatalogueEvidence` — even without the observed bit —
would backdate observed truth to preparation, exactly the substitution card
033 rejected. The tuple has one emitter: a completed
`list_sessions_with_projection`, from exact completed-operation evidence, with
`with_observed()` set. Prepared success and every operation failure leave the
row absent.

### Cross-Seam Source Isolation Is Applicability, Not Identity

The two seams are composed independently and share no state. A caller may
therefore legitimately reuse one bounded source ID across an
`open_session_with_projection` call and a `list_sessions_with_projection`
call, and neither method can observe the other's identifiers. This gate
consequently makes no claim it cannot enforce:

- each seam admits only the two IDs supplied to that call, and rejects their
  equality before provider work; and
- the cross-operation boundary is exact applicability, not identifier
  inequality. Every catalogue row carries the catalogue plan's
  `OperationShape::ProviderSessionCatalogue`,
  `DriverRole::ProviderSessionCatalogue`, and absent model route, so composing
  a catalogue contribution into an interactive-session snapshot fails in the
  runtime composer with
  `swallowtail.consumer_route_projection.snapshot_identity_rejected`, and the
  inverse fails likewise.

Card 034 must prove the boundary that way. A proof that passes only because a
fixture chose different source-ID literals proves nothing and is rejected.

## Projection Semantics

Prepared and active sources are caller-supplied. Each seam admits exactly the
two IDs passed to that call and rejects their equality before any process,
connection, or provider work with
`swallowtail.kimi.projection_source_identity_invalid` — as
`KimiProjectionOpenFailure::Runtime` on the open seam and
`KimiCatalogueProjectionFailure::SourceIdentity` on the catalogue seam.
Neither seam can observe the other's identifiers, so neither asserts anything
about them.

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
  truth — fail; it publishes no row on either branch below
- a **pre-lifecycle** foreign or over-bound confirmation — concretely,
  `DeclaredEffort` requested `"high"` confirmed as a foreign catalogue row —
  answered with `reasoning_value_foreign` or `reasoning_value_unbounded` — fail;
  `NegotiatedReasoningSetup::confirm` already aborted with
  `swallowtail.negotiated_reasoning.effective_mismatch`, no session exists to
  close, and case 2 must return that exact preserved failure with no
  contribution
- a **projection-only** foreign or over-bound confirmation — concretely,
  `DeclaredEffort` requested `"on"` confirmed as a foreign or over-bound effort
  — answered with the preserved success or with `effective_mismatch` — fail;
  case 4 must close the opened session and return
  `reasoning_value_foreign` or `reasoning_value_unbounded` with no
  contribution, while `open_session` still succeeds on the identical fixture
- case 4 described or proved as a general fallback rather than the
  reasoning-only, requested-`"on"`-only branch — fail; every other combination
  is unreachable on current `main`
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
- a maximal request for both halves whose reasoning rejects, reporting Plan as
  `NotRequested`, `Effective`, or `Rejected` — fail; the Plan control was
  requested and never confirmed, so it is `RequestedNotObserved`
- that same early stop performing any further provider work, reordering
  `driver.rs`'s reasoning-then-Plan confirmations, or letting the compound row
  domain carry a `plan=` entry — fail
- a contribution in which every requested half is `RequestedNotObserved`
  publishing a row or retaining an active source — fail; no half was observed
- `load_session`, `resume_session`, or `prepare_working_state_restoration`
  gaining projection authority from the open-only decision — fail
- prepared catalogue success presented as observed catalogue truth — fail; only
  a completed `list_sessions_with_projection` may set `observed`
- a catalogue row carrying `OperationShape::InteractiveSession` applicability,
  or an interactive-session row carrying
  `OperationShape::ProviderSessionCatalogue` — fail on the row's own
  applicability
- a catalogue contribution composed into an interactive-session snapshot, or
  the inverse — fail in the composer with
  `swallowtail.consumer_route_projection.snapshot_identity_rejected`
- a cross-seam isolation proof that passes only because its fixtures chose
  different source-ID literals — fail; neither seam can observe the other's
  IDs, so identifier inequality is not the boundary
- equal prepared and observed catalogue IDs on one
  `list_sessions_with_projection` call reaching catalogue dispatch, or
  surfacing as `Operation` or `Projection` rather than `SourceIdentity` — fail
- `control.provider-session-catalogue` emitted by
  `KimiPreparedSessionCatalogue::consumer_route_projection_contribution`, in
  any state — fail; prepared evidence would backdate observed truth to
  preparation
- `control.provider-session-catalogue` present after an operation failure, or
  absent after a completed `list_sessions_with_projection` — fail; the seam is
  its sole emitter and sets `with_observed()`
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
