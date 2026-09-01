# Contract 061 Kimi Active-Observation Public-Baseline Gate

Status: stopped; evidence stop; gate incomplete; candidate F not promoted
Owner: Tom
Date: 2026-09-01
Source: operator decision, Card 033 evidence stop, Contract 061, and `main` at
`10fab5eac15f16199f8da51484f94a8a4755efeb`

## Purpose

Compile the operator's five `kimi-code.acp` answers into one exact
public-baseline gate for candidate F.

The gate could not be completed. Four of the five answers are realizable
route-locally against current `main`. The fifth is not: no shared runtime
source kind, lifecycle band, or projection view can represent a completed
provider-session catalogue query, and reinterpreting the existing ones in
adapter documentation would silently widen shared public semantics. That is the
stop condition the handoff fixed, so this document records the exact conflict
and returns one further operator decision instead of weakening the baseline.

This is planning evidence. It authorizes no Rust, no implementation card, no
provider contact, and no coverage claim.

## Operator Decision As Given

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
   `KimiPreparedSessionCatalogue::list_sessions`. **This answer cannot be
   realized without a shared runtime decision the operator has not made.** See
   the blocker below.
5. Prepared and active-observation source IDs are distinct and
   caller-supplied. No runtime, testkit, core, contract, registry, callback,
   generic provider payload, or cross-route decision is added.

Decisions 1, 2, 3, and 5 remain route-local to `kimi-code.acp` and grant no
authority to any other route or candidate.

## Blocker: Provider-Operation Observation Has No Shared Representation

`KimiPreparedSessionCatalogue::list_sessions` runs a
`ProviderSessionCatalogue` operation. Its plan carries
`OperationShape::ProviderSessionCatalogue`,
`DriverRole::ProviderSessionCatalogue`, and no model route. It opens no
interactive session and returns no session handle.

The census gives `control.provider-session-catalogue` lifecycle
`post-open-observation-only` and state `descriptor-only; observed`. Publishing
it therefore requires all three of the following, and current
`swallowtail-runtime` defines each one as session-scoped:

| Required | Current public definition | Location |
| --- | --- | --- |
| `ConsumerRouteProjectionSourceKind::ActiveSessionObservation` | "One exact post-open active-session observation." | `consumer_route_projection/identity.rs:16-17` |
| `ConsumerRouteLifecycle::PostOpenObservationOnly` | "Observed only after the session opens." | `consumer_route_projection/semantics/posture.rs:14-15` |
| `ConsumerRouteActiveSessionState` | "Immutable post-open observation and exact negotiated state." | `consumer_route_projection/views.rs:35-36` |

A completed catalogue query satisfies none of them. Exact
`ProviderSessionCatalogue` applicability does prevent cross-operation mixing in
the composer, but applicability is a separate axis: it does not make the source
kind, lifecycle band, or view true.

An earlier draft of this gate asserted that it did. That was wrong. Restating
"post-open active-session observation" to mean "post-operation observation" in
an adapter-owned document changes what those three shared public names mean for
every route that already uses them, without a runtime baseline change, a
contract amendment, or an operator decision. Contract 061 and the handoff both
forbid that.

The blocker is a missing shared vocabulary, not a missing adapter seam. No
arrangement of adapter-local types resolves it, so `KimiCatalogueProjection*`
and `list_sessions_with_projection` are withdrawn from this gate's fixed
surface. They are not part of any authorized baseline.

## Realizable Route-Local Surface

The following is fixed evidence for whichever direction the operator chooses.
It is **not** authorization to implement. `KimiPreparedSession::open_session`,
`load_request`, `load_session`, `resume_request`, `resume_session`,
`prepare_working_state_restoration`, and `into_parts`, and
`KimiPreparedSessionCatalogue::list_sessions`, `list_page`,
`next_page_request`, `evidence`, `plan`, `request`, and `low_level_driver`,
all keep their current signatures, handles, failure codes, and cleanup
behavior.

```rust
pub struct KimiProviderValue { /* private fields */ }

impl KimiProviderValue {
    pub fn as_str(&self) -> &str;
}

pub type KimiProjectionOpenFuture = BoxFuture<
    'static,
    Result<KimiProjectionOpenOutcome, KimiProjectionOpenFailure>,
>;

pub struct KimiProjectionOpenOutcome { /* private fields */ }

impl KimiProjectionOpenOutcome {
    pub fn session(&self) -> &dyn InteractiveSessionHandle;
    pub const fn contribution(&self) -> &ConsumerRouteProjectionContribution;
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
```

The acknowledgement accessors are deliberately absent. Their exact shape is an
open route-local item; see **Unresolved Route-Local Public-Baseline Item**.

`KimiProviderValue::new` is not public. The projected open never exposes the
ACP response, config-option object, command, path, credential, or any other
provider payload.

The prepared contribution method established by cards 022-024 applies
unchanged to the candidate F prepared facades:

```rust
pub fn consumer_route_projection_contribution(
    &self,
    source_id: ConsumerRouteProjectionSourceId,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure>;
```

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

`KimiPreparedSessionCatalogue::consumer_route_projection_contribution` emits
the prepared `feature.provider-session-catalogue` row and nothing more. It must
not emit `control.provider-session-catalogue` in any state: that control is
`post-open-observation-only` with `observed` support, so prepared evidence
would backdate observed truth to preparation, exactly the substitution card 033
rejected.

## Bounded Adapter-Local Provider Value

`swallowtail-adapter-kimi` owns one private constant and one admission
function. Neither is shared, re-exported, or derived from a runtime or core
constant:

```rust
const MAXIMUM_KIMI_PROVIDER_VALUE_BYTES: usize = 128;
```

`128` is the width `swallowtail-core`'s
`crates/swallowtail-core/src/model_catalog/value.rs` already uses for one
bounded provider-defined catalogue value. That is planning precedent only; an
implementation must declare its own adapter-local constant and must not import,
re-export, or alias `ProviderCatalogValue`'s `MAX_PROVIDER_VALUE_BYTES`.

A confirmation token is retainable only when it is non-blank after trimming,
`value.trim() == value`, control-free, and within the bound.

Retention is not publication. A retained token publishes an active row only
when it is additionally *admitted* for the exact prepared `KimiAcpBehavior`:

- `LegacyReasoning` — `off` or `on`;
- `DeclaredEffort` — `off`, `on`, `low`, `medium`, `high`, `xhigh`, or `max`;
- harness mode — `default`, `plan`, `auto`, or `yolo`, which
  `driver/mode.rs` already freezes as one exact ordered domain.

A token outside its half's admitted set is *foreign*. Foreign and unretainable
tokens both fail closed on the projected path, through one of the two disjoint
branches below. Neither ever becomes `ReasoningMode`,
`NegotiatedReasoningSetup`, `EffectiveReasoningSetup`, or any other portable
value.

## Preserved Versus Projected Behavior

One private Kimi ACP open lifecycle serves both public open methods. It runs
exactly today's sequence and additionally records, per half, the exact
`currentValue` string that half's confirmation carried, or nothing when the
half was not requested. Recording never changes control flow.

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
   contribution }` carrying the same `RuntimeFailure` the preserved path
   returns, the acknowledgement row, and no session.
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
branches, and the two adapter codes belong to exactly one of them. Neither may
be described with blanket wording.

**Pre-lifecycle — case 2.** When the requested mode is concrete, current `main`
already compares it against the confirmation. `LegacyReasoning` and
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
branch and must be named that precisely rather than as a general fallback.

Case 4 is the only place the projected path may differ from a successful
preserved open, and it fails closed. Cases 1 and 2 keep exact route-code and
cleanup parity with `open_session` for every fixture.

## Compound Acknowledgement Half States

`driver.rs` confirms reasoning first and Plan second. This gate preserves that
order, that control flow, and the `?` propagation exactly; no design may
perform extra provider work to discover an unobserved half.

**Reasoning half.** Under a requested `ReasoningMode` `R`, prepared behavior
`B`, and confirmation `currentValue` `C`:

- reasoning not requested — no half, no row contribution, no active source;
- `B = LegacyReasoning`, `C == R` — effective, exact `C`;
- `B = LegacyReasoning`, `C != R` — rejected, exact `C`, preserving
  `swallowtail.negotiated_reasoning.effective_mismatch`;
- `B = DeclaredEffort`, `R == "on"`, `C == "off"` — rejected `"off"`,
  preserving the same mismatch failure;
- `B = DeclaredEffort`, `R == "on"`, `C != "off"` — effective, carrying the
  **exact provider token**, not the normalized `"on"`. The preserved path still
  succeeds with its normalized `EffectiveReasoningSetup`;
- `B = DeclaredEffort`, `R` concrete, `C == R` — effective, exact `C`;
- `B = DeclaredEffort`, `R` concrete, `C != R` — rejected, exact `C`; and
- missing, malformed, duplicated, ambiguous, unadvertised, transport, setup,
  unretainable, or foreign confirmation — no half; ordinary `Runtime` with no
  contribution.

Because reasoning is confirmed first, no exposed outcome or rejected failure
can carry a "reasoning requested but not observed" state. Every earlier failure
takes case 2 and publishes nothing. A `RequestedNotObserved` variant on the
reasoning half would be unreachable and is explicitly rejected.

**Plan half.** `driver/mode.rs` freezes `["default", "plan", "auto", "yolo"]`
in listed order, so the Plan token is always admitted and always within the
byte bound:

- Plan not requested — no half;
- `currentValue == "plan"` — effective `"plan"`;
- `currentValue` exactly `default`, `auto`, or `yolo` — rejected, exact value,
  preserving `swallowtail.kimi.acp.harness_mode_mismatch`; and
- Plan requested but never confirmed because reasoning rejected first — a real,
  reachable state with no observed value. This is the case 1 early stop, and it
  is exactly what the unresolved item below must represent.

Every other Plan outcome is ordinary `Runtime` with no contribution.

## Unresolved Route-Local Public-Baseline Item

The compound `feature.active-session-reasoning-and-plan-ack` row has no fixed
shape. An earlier draft encoded the halves as `reasoning=<token>` and
`plan=<token>` domain entries with row-level union state flags. That does not
work, for two independent reasons, and both must be settled before any
implementation card:

1. **Half-to-state association is lost.** With effective reasoning and rejected
   Plan, the row carries two domain entries and both `provider_effective` and
   `rejected` flags at row level. A generic Contract 061 consumer cannot tell
   which token is effective and which is rejected. Adapter-specific typed
   accessors do not repair this: Contract 061's facade requirement is that a
   consumer reads projection rows without downcasting to adapter types.
2. **Pending was invented.** The earlier draft set `with_pending()` for a Plan
   half that reasoning had terminally rejected. `ConsumerRouteStateSupport`
   documents that flag as "Adds proven pending acknowledgement state"
   (`semantics/authority.rs`). After a terminal reasoning rejection no Plan
   request was ever dispatched, so nothing is pending; the acknowledgement will
   never arrive.

An eventual exact design must preserve each half's state generically, without
downcasting and without inventing pending truth, and must represent "requested,
never dispatched, will not arrive" honestly. Whether that is achievable with
two separate namespaced rows inside the one census tuple, or needs its own
operator decision, is open. This gate does not choose.

## Negotiated Model-Option Observation

`driver/validation.rs` `parse_model_options` already extracts one exact `model`
config option and `driver.rs` threads it into `attachment.take_session(...)` on
open, load, and resume.
`KimiSessionHandle::negotiated_model_options()` already returns
`Option<&NegotiatedSessionModelOptions>`. That behavior does not change.

Unlike `cline.acp`, Kimi already fails
`swallowtail.kimi.acp.malformed_response` for a missing or malformed `model`
option on both paths, so there is no preserved-versus-projected model split to
create and none may be created.

The bounded namespaced `feature.negotiated-model-options-observation` row is
published only when the snapshot is `Some`. It carries
`ConsumerRouteValueKind::Observation`, an unenumerated descriptor domain,
`NotSelectable`, `with_observed()` state, wire-acknowledgement evidence,
`ObservationOnly` posture, and no mutation authority. This family is genuinely
post-open on an interactive session, so it does not hit the blocker above.

## Projection Semantics

Prepared and active sources are caller-supplied. The open seam admits exactly
the two IDs passed to that call and rejects their equality before any process,
connection, or provider work with
`swallowtail.kimi.projection_source_identity_invalid` as
`KimiProjectionOpenFailure::Runtime`.

Prepared selection and session-start rows use
`AdapterContribution(prepared_source_id)`. Post-open acknowledgement and model
observation use `ActiveSessionObservation(active_session_source_id)` — both are
genuinely post-open on an opened interactive session. An active source that
names no published active row is omitted from the contribution.

The Kimi-only active identities and the prepared
`control.provider-session-catalogue` and `control.provider-session-import`
identities are bounded namespaced extensions qualified by exact route ID and
exact `protocol_facade_id`. The closed
`ConsumerRouteControlId::SessionCatalogueBounds` names a different census row
and must not be substituted.

The `kimi-code.acp` prepared `control.reasoning-selection` row is requested,
prepared, and pending at session start. It never carries provider-effective or
rejected state. `control.session-options` names exactly the accepted Kimi
subset — portable reasoning mode and Plan harness mode — and never projects the
developer-instruction or tool fields `validate_options` rejects.

## Operation-Shape-Scoped Persistence

`kimi-code.acp` carries two provider-state policies by operation shape, and the
census carries exactly one `feature.persistent-session-posture` row:

- `prepared_profile/plan.rs` sets `SessionProviderStatePolicy::Prohibited` on
  the interactive-session plan, so `KimiPreparedSession` must never emit the
  persistence row; and
- `prepared_profile/provider_session_catalogue.rs` sets
  `DurableProviderSessionPreserved` and requires
  `Capability::ProviderDurableRetention` on the session-import plan, so
  `KimiPreparedSessionImport` is the sole emitter.

Documentation cannot override either policy.

## Profile-Conditional Attachment Controls

`KimiPreparedSession::load_request` and `resume_request` both call
`reject_attachment_options`, which rejects any bound reasoning mode or harness
mode. A prepared session that requested either therefore cannot produce a load
or resume request at all.

`control.load-session` and `control.resume-session` are consequently
**profile-conditional**, not unconditional session rows. They are emitted from
attachment-compatible profiles — those that bound neither reasoning nor Plan —
and must be omitted from maximal reasoning/Plan profiles and from the maximal
projected open. The route ledger still carries each tuple once.

## Candidate F Is Not Promoted

Candidate F fails Batch 9.4 rubric item 3: it needs a public decision that is
not closed. Coverage stays at 249 proved and 518 remaining rows. Its 89 rows
remain unproved.

The four route ledgers below are re-derived from the reviewed census plus
current `main` and are retained as evidence for whichever direction the
operator chooses. They are not authorization.

| Route | Census | Emitted | Withheld | Undecided |
| --- | ---: | ---: | ---: | ---: |
| `kimi-code.acp` | 25 | 21 | 3 | 1 |
| `kimi-code.headless` | 20 | 10 | 10 | 0 |
| `kimi-code.local-server` | 31 | 31 | 0 | 0 |
| `kimi-platform.chat` | 13 | 12 | 1 | 0 |
| **Total** | **89** | **74** | **14** | **1** |

The single undecided row is `kimi-code.acp`
`session-management` / `control.provider-session-catalogue`. If the operator
authorizes a provider-operation observation baseline it becomes emitted and the
totals are 75/14; if the row stays withheld they are 74/15. Card 033's
provisional 86/3 reading does not survive either way.

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
every one of these rows is unprojectable on current `main` by construction.

`feature.model-catalogue` is emitted on `kimi-code.local-server` and
`kimi-platform.chat`, where `KimiLocalServerPreparedCatalogue` and
`KimiPlatformPreparedCatalogue` each carry `DriverRole::ModelCatalog` and
`Capability::ModelCatalog`. On `kimi-platform.chat`,
`control.model-selection` comes from `KimiPlatformPreparedInferenceAttempt`
alone: `prepare_catalogue` builds its plan with no model route, so the prepared
catalogue proves no model selection.

Six rows are maximal-only: `kimi-code.acp` `feature.reasoning-selection` and
`control.reasoning-selection`; `kimi-code.local-server`
`feature.reasoning-selection`, `feature.stream-reattachment`,
`feature.permission-exchange`, and `feature.question-exchange`.
`kimi-code.local-server` `feature.owned-runtime-lifecycle` is emitted only from
the `HostOwnedEphemeral` owned topology. `kimi-code.acp`
`control.load-session` and `control.resume-session` are the inverse: emitted
only from attachment-compatible profiles.

## Operator Decisions Required

Two decisions are open. Neither is answered here.

1. **Provider-operation observation baseline.** Either broaden the shared
   Contract 061 vocabulary so a completed provider-operation query can be
   projected honestly — a new source kind, lifecycle band, and view, or an
   explicit widening of the existing three — or leave
   `kimi-code.acp` `control.provider-session-catalogue` withheld as
   unrepresentable. The first is a `swallowtail-runtime` public decision with
   cross-route reach and would touch Contract 061; the second keeps this gate
   route-local but leaves one census row permanently unproved on a route that
   demonstrably performs the observation.
2. **Compound acknowledgement representation.** Fix a shape that preserves each
   half's state generically, without adapter downcasts and without inventing
   pending state, and that represents a requested-but-never-dispatched Plan
   half honestly.

Decision 1 gates candidate F as a whole. Decision 2 gates the
`feature.active-session-reasoning-and-plan-ack` row and therefore also gates
any implementation card, because the census carries that row on
`kimi-code.acp`.

## Review Oracle

Invariant: this document is planning evidence for a stopped gate. It authorizes
no implementation.

- treat this gate as complete, strict-ready, or as promoting candidate F — fail
- count any of the 89 rows as proved — fail; coverage is 249/518
- publish `control.provider-session-catalogue` through
  `ActiveSessionObservation`, `PostOpenObservationOnly`, or
  `ConsumerRouteActiveSessionState` while those three remain session-scoped —
  fail; that reinterprets shared public semantics in adapter documentation
- resolve the blocker with adapter-local types alone — fail; the missing
  vocabulary is shared
- emit `control.provider-session-catalogue` from
  `KimiPreparedSessionCatalogue::consumer_route_projection_contribution`, in
  any state — fail; prepared evidence would backdate observed truth
- fix the compound acknowledgement row's shape without settling half-to-state
  association generically — fail
- set `with_pending()` for a half that was never dispatched — fail; the flag
  means proven pending acknowledgement state
- introduce a reachable `RequestedNotObserved` on the reasoning half — fail;
  reasoning is confirmed first, so no exposed outcome can carry it
- answer a pre-lifecycle foreign or over-bound token with
  `reasoning_value_foreign` or `reasoning_value_unbounded` — fail; case 2
  returns the preserved `effective_mismatch`
- answer a projection-only foreign or over-bound token — `DeclaredEffort`
  requested `"on"` — with the preserved success or with `effective_mismatch` —
  fail; case 4 closes the opened session and returns the new adapter code
- describe case 4 as a general fallback rather than the reasoning-only,
  requested-`"on"`-only branch — fail
- emit `control.load-session` or `control.resume-session` from a maximal
  reasoning/Plan profile — fail; `reject_attachment_options` makes those
  requests unconstructible
- attribute `kimi-platform.chat` `control.model-selection` to
  `KimiPlatformPreparedCatalogue` — fail; its plan binds no model route
- emit `feature.persistent-session-posture` on `kimi-code.acp` from the
  `Prohibited` interactive-session plan — fail
- emit any of the 14 withheld rows from documentation or the provider matrix —
  fail; no evidence-strength variant admits it
- reach 25, 20, 31, or 13 through a filter, exception list, duplicate tuple, or
  borrowed identity — fail

## Validation Boundary

None. This document changes no Rust and authorizes no validation tier beyond
the planning batch that carries it: `effigy qa:docs`, `effigy qa:northstar`,
and `git diff --check`.

## Authority

- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
- [Batch 9.4 package expansion](2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [completed card 033](../roadmaps/g05/batch-cards/033-contract-061-card-032-closeout-and-kimi-reassessment.md)
- [blocked card 034](../roadmaps/g05/batch-cards/034-contract-061-kimi-package-completion.md)
- [reviewed census](2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [Batch 9.1 public baseline](2026-08-31-contract-061-batch-9-1-public-baseline-gate.md)
- [Claude Agent acknowledgement gate](2026-08-31-contract-061-claude-agent-acknowledgement-public-baseline-gate.md)
- [Cline active-observation gate](2026-09-01-contract-061-cline-active-observation-public-baseline-gate.md)
