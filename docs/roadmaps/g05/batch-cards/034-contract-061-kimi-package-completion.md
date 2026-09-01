# 034 Contract 061 Kimi And Kimi Platform Package Completion

Status: planned; not ready; blocked by provider-operation observation public-baseline decision
Owner: Tom
Created: 2026-09-01
Updated: 2026-09-01
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: completed card 033; provider-operation observation public-baseline
decision; an accepted compound-acknowledgement representation if candidate F
continues

## Why This Card Is Blocked

The Kimi gate is stopped. Publishing
`control.provider-session-catalogue` needs
`ConsumerRouteProjectionSourceKind::ActiveSessionObservation`,
`ConsumerRouteLifecycle::PostOpenObservationOnly`, and
`ConsumerRouteActiveSessionState`. Current `swallowtail-runtime` defines all
three as post-open **session** semantics, while
`KimiPreparedSessionCatalogue::list_sessions` opens no session. The completed
query therefore cannot be published honestly through those names. Do not
broaden or reinterpret the shared vocabulary here.

The sole open operator decision is whether to compile a shared
provider-operation observation public-baseline gate with honest
source/lifecycle/view vocabulary, or to leave
`control.provider-session-catalogue` withheld and candidate F unpromoted. Both
directions are recorded in the
[stopped gate](../../../triage/2026-09-01-contract-061-kimi-active-observation-public-baseline-gate.md).

This card is retained as planned, not-ready evidence. If candidate F continues,
the compound acknowledgement shape must still be settled in planning before
this card can become ready. It authorizes no Rust,
and none of its 89 rows count toward coverage. Coverage stays at 249 proved and
518 remaining. The compound acknowledgement, attachment-control, and
platform-catalogue corrections below remain route-local evidence requirements;
they do not select a replacement design or create another Next Task.

Do not implement this card. Do not compile a replacement card. Do not choose
the shared runtime direction inside a batch card.

## Intended Goal, Once Unblocked

Complete candidate F's exact 89-row Contract 061 package remainder across
`swallowtail-adapter-kimi` and `swallowtail-adapter-kimi-platform`, exposing
exact `kimi-code.acp` acknowledgement and bounded negotiated model options
through the accepted additive adapter-owned open seam.

## Retained Scope

Every item below is contingent on the provider-operation observation decision.
Items 6 and 7 cannot be written exactly until an acknowledgement representation
is accepted; the notes retain unresolved route-local evidence and do not select
that public shape.

1. Add the established
   `consumer_route_projection_contribution(source_id)` method to
   `KimiPreparedSession`, `KimiPreparedSessionCatalogue`,
   `KimiPreparedSessionImport`, `KimiHeadlessPreparedRun`,
   `KimiLocalServerPreparedCatalogue`, `KimiLocalServerPreparedRun`,
   `KimiLocalServerPreparedSession`, `KimiLocalServerPreparedArchive`,
   `KimiLocalServerPreparedRestore`, `KimiLocalServerPreparedReconciliation`,
   `KimiLocalServerPreparedBindingImport`, `KimiPlatformPreparedCatalogue`,
   and `KimiPlatformPreparedInferenceAttempt`. `KimiPreparedEvidence`,
   `KimiHeadlessPreparedEvidence`, `KimiPlatformPreparedEvidence`,
   `KimiModelSelection`, `KimiPlatformModelSelection`,
   `KimiAcpSessionImportAuthority`, and
   `KimiLocalServerSessionConfiguration` gain no contribution method.
2. Add `KimiProviderValue`, `KimiProjectionOpenFuture`,
   `KimiProjectionOpenOutcome`, and `KimiProjectionOpenFailure` with the exact
   signatures the gate's realizable surface fixes.
   `KimiProviderValue::new` is not public.
3. Add `KimiPreparedSession::open_session_with_projection` with distinct
   prepared and active-session source IDs. Serve it and `open_session` from one
   private open lifecycle that records each half's exact confirmation
   `currentValue` without changing control flow. Keep `open_session`,
   `load_request`, `load_session`, `resume_request`, `resume_session`,
   `prepare_working_state_restoration`, and `into_parts` exactly as they are on
   `main`, including the `DeclaredEffort` requested-`"on"` normalization.
4. **Blocked by the provider-operation observation decision.** No projected
   catalogue seam is authorized.
   `KimiPreparedSessionCatalogue::consumer_route_projection_contribution` emits
   the prepared `feature.provider-session-catalogue` row and must not emit
   `control.provider-session-catalogue` in any state, because that control is
   `post-open-observation-only` with `observed` support and prepared evidence
   would backdate observed truth. `list_sessions`, `list_page`,
   `next_page_request`, continuation paging, cursor semantics, failure stages,
   and cleanup stay unchanged and unprojected.
5. Add one adapter-private `MAXIMUM_KIMI_PROVIDER_VALUE_BYTES = 128` constant
   and one adapter-private admission function. Do not import, re-export, or
   alias `swallowtail-core`'s `ProviderCatalogValue` bound and do not add a
   shared constant. Retain a confirmation token only when it is non-blank,
   trimmed-equal, control-free, and within the bound.
6. Preserve the first-round acknowledgement evidence without selecting a
   public shape. The generic compound row must associate each reasoning/Plan
   half with its exact effective, rejected, or not-observed state without an
   adapter downcast. A requested-but-never-dispatched Plan half must not be
   mapped to `with_pending()` because no acknowledgement was dispatched and
   the reasoning failure is terminal. Under `DeclaredEffort` with requested
   `"on"`, retain the exact provider-confirmed effort rather than `"on"`.
   Keep the two disjoint foreign/unretainable branches the gate fixes separate:
   - **pre-lifecycle (case 2)** — the lifecycle already aborted, so return its
     exact preserved `RuntimeFailure` unchanged with no contribution. No
     session exists to close and the new adapter codes must not appear.
   - **projection-only (case 4)** — reachable only under `DeclaredEffort` with
     requested `"on"` and a non-`"off"` confirmation that is foreign or
     over-bound. Close the opened session and return `Runtime` with
     `swallowtail.kimi.acp.reasoning_value_foreign` or
     `swallowtail.kimi.acp.reasoning_value_unbounded`, publishing no row, while
     `open_session` still succeeds on the identical fixture.
7. Preserve `driver.rs`'s reasoning-then-Plan confirmation order and control
   flow exactly. A maximal request whose reasoning rejects never dispatches the
   Plan request, so any eventual design must represent that Plan half as
   requested but not observed, without extra provider work, without mapping it
   to pending acknowledgement state, and without requiring an adapter downcast
   to read it. `KimiReasoningAcknowledgement::RequestedNotObserved` is
   speculative and unreachable under the fixed order and is not an accepted
   public baseline. Because reasoning is confirmed first, no exposed outcome
   can carry an unobserved reasoning half; every earlier failure takes case 2.
8. Publish `feature.negotiated-model-options-observation` only when
   `parse_model_options` returned `Some`. Change neither `parse_model_options`
   nor `validate_session_configuration`, and introduce no preserved-versus-
   projected model split where current `main` fails both paths identically.
9. Publish the Kimi-only active identities plus the
   `control.provider-session-catalogue` and `control.provider-session-import`
   identities as bounded namespaced extensions qualified by exact route ID and
   exact `protocol_facade_id`. Do not substitute
   `ConsumerRouteControlId::SessionCatalogueBounds`. Scope source-identity
   preflight to the open seam's own supplied pair.
10. Disposition exactly 89 census tuples with no filter or exception list: 25
    `kimi-code.acp`, 20 `kimi-code.headless`, 31 `kimi-code.local-server`, and
    13 `kimi-platform.chat`. Name each tuple once with an emitted or
    construction-time-withheld reason matching the ledgers below.
11. Preserve the corrected interim four-ledger arithmetic as reassessment
    evidence: 89 rows, 74 emitted, 14 withheld, and 1 undecided. These totals
    are not proved coverage or implementation authority while the
    provider-operation observation decision is open.
12. Emit `feature.persistent-session-posture` on `kimi-code.acp` only from
    `KimiPreparedSessionImport`. `KimiPreparedSession` must not emit it; its
    plan is `SessionProviderStatePolicy::Prohibited`.
13. Emit `control.load-session` and `control.resume-session` only from
    attachment-compatible profiles. `load_request` and `resume_request` both
    call `reject_attachment_options`, so a profile that bound reasoning or Plan
    cannot construct either request and must omit both controls.
14. Add deterministic provider-free fixtures and the exact proof set named
    below. Stop after one reviewable two-package PR.

## Exact Four-Route Ledgers

Each row is one `(route_id, operation_shape, semantic_id)` census tuple. The
dispositions are derived from the reviewed census plus current `main` driver
roles, capability profiles, extension namespaces, ownership modes, attachment
option rejection, and provider-state policies — not from the provider feature
matrix and not from card 033's provisional 86/3 split.

These ledgers are retained evidence. They are not authorization, and none of
these rows counts toward Contract 061 coverage while this card is blocked.

### `kimi-code.acp` — 25 Rows, 21 Emitted, 3 Withheld, 1 Undecided

| # | Operation shape | Semantic ID | Disposition |
| ---: | --- | --- | --- |
| 1 | `model-catalogue` | `feature.model-catalogue` | **withheld** — no `DriverRole::ModelCatalog`; no ACP plan requires `Capability::ModelCatalog` |
| 2 | `structured-run` | `feature.structured-run` | **withheld** — the ACP descriptor declares no `StructuredRun` role or shape |
| 3 | `interactive-session` | `feature.interactive-session` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 4 | `route-observation` | `feature.streaming-events` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 5 | `route-capability` | `feature.reasoning-selection` | emitted — `KimiPreparedSession` (maximal); `open_session_with_projection` (maximal) |
| 6 | `route-capability` | `feature.cancellation-or-interruption` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 7 | `session-lifecycle` | `feature.load-session` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 8 | `session-lifecycle` | `feature.resume-session` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 9 | `session-lifecycle` | `feature.provider-session-catalogue` | emitted — `KimiPreparedSessionCatalogue` |
| 10 | `session-lifecycle` | `feature.provider-session-import` | emitted — `KimiPreparedSessionImport` |
| 11 | `route-capability` | `feature.working-resource` | emitted — `KimiPreparedSession`; `open_session_with_projection`; `KimiPreparedSessionCatalogue`; `KimiPreparedSessionImport` |
| 12 | `route-capability` | `feature.bounded-workspace-text-write` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 13 | `session-lifecycle` | `feature.provider-managed-recovery` | **withheld** — no ACP plan requires `Capability::ProviderManagedRecovery`; continuation recovery is load-session authority |
| 14 | `session-lifecycle` | `feature.persistent-session-posture` | emitted — `KimiPreparedSessionImport` only — the session plan is `Prohibited` |
| 15 | `route-capability` | `feature.prepared-facade` | emitted — `KimiPreparedSession`; `KimiPreparedSessionCatalogue`; `KimiPreparedSessionImport`; `open_session_with_projection` |
| 16 | `route-observation` | `feature.activity-observation` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 17 | `interactive-session` | `feature.active-session-reasoning-and-plan-ack` | emitted — `open_session_with_projection` only — **shape unresolved**, see the gate's open route-local item |
| 18 | `interactive-session` | `feature.negotiated-model-options-observation` | emitted — `open_session_with_projection` only |
| 19 | `interactive-session` | `control.model-selection` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 20 | `interactive-session` | `control.reasoning-selection` | emitted — `KimiPreparedSession` (maximal); `open_session_with_projection` (maximal) |
| 21 | `interactive-session` | `control.session-options` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 22 | `session-management` | `control.load-session` | emitted — attachment-compatible profiles only — `load_request` calls `reject_attachment_options`, so maximal reasoning/Plan profiles omit it |
| 23 | `session-management` | `control.resume-session` | emitted — attachment-compatible profiles only — `resume_request` calls `reject_attachment_options`, so maximal reasoning/Plan profiles omit it |
| 24 | `session-management` | `control.provider-session-catalogue` | **undecided** — emitted only if a provider-operation observation baseline is authorized; withheld otherwise |
| 25 | `session-management` | `control.provider-session-import` | emitted — `KimiPreparedSessionImport` |

25 distinct tuples; 21 emitted; 3 withheld; 1 undecided. These dispositions
are evidence only while the gate is stopped.

### `kimi-code.headless` — 20 Rows, 10 Emitted, 10 Withheld

| # | Operation shape | Semantic ID | Disposition |
| ---: | --- | --- | --- |
| 1 | `model-catalogue` | `feature.model-catalogue` | **withheld** — `kimi_headless_descriptor()` carries only `Discovery` and `StructuredRun` |
| 2 | `structured-run` | `feature.structured-run` | emitted — `KimiHeadlessPreparedRun` |
| 3 | `interactive-session` | `feature.interactive-session` | **withheld** — matrix-only; `run_capabilities()` has no `InteractiveSession` |
| 4 | `route-observation` | `feature.streaming-events` | emitted — `KimiHeadlessPreparedRun` |
| 5 | `route-capability` | `feature.reasoning-selection` | **withheld** — matrix-only; the headless run plan requires no `ReasoningSelection` |
| 6 | `route-capability` | `feature.cancellation-or-interruption` | emitted — `KimiHeadlessPreparedRun` |
| 7 | `session-lifecycle` | `feature.load-session` | **withheld** — matrix-only; no `LoadSession` capability and no public load operation |
| 8 | `session-lifecycle` | `feature.resume-session` | **withheld** — matrix-only; no `Resume` capability and no public resume operation |
| 9 | `session-lifecycle` | `feature.provider-session-catalogue` | **withheld** — matrix-only; no catalogue role, capability, or facade |
| 10 | `session-lifecycle` | `feature.provider-session-import` | **withheld** — matrix-only; no import role, capability, or facade |
| 11 | `route-capability` | `feature.working-resource` | emitted — `KimiHeadlessPreparedRun` |
| 12 | `route-capability` | `feature.bounded-workspace-text-write` | **withheld** — matrix-only; `run_capabilities()` has no `WorkingResourceTextWrite` |
| 13 | `session-lifecycle` | `feature.provider-managed-recovery` | emitted — `KimiHeadlessPreparedRun` |
| 14 | `session-lifecycle` | `feature.persistent-session-posture` | emitted — `KimiHeadlessPreparedRun` — `ProviderDurableRetention` |
| 15 | `route-capability` | `feature.prepared-facade` | emitted — `KimiHeadlessPreparedRun` |
| 16 | `route-observation` | `feature.activity-observation` | emitted — `KimiHeadlessPreparedRun` |
| 17 | `structured-run` | `control.model-selection` | emitted — `KimiHeadlessPreparedRun` |
| 18 | `session-management` | `control.load-session` | **withheld** — census `matrix-descriptor-only`; the route exposes no public session input |
| 19 | `session-management` | `control.resume-session` | **withheld** — census `matrix-descriptor-only`; the route exposes no public session input |
| 20 | `structured-run` | `control.provider-managed-recovery` | emitted — `KimiHeadlessPreparedRun``::accept_managed_recovery` |

20 distinct tuples; 10 emitted; 10 withheld.

### `kimi-code.local-server` — 31 Rows, 31 Emitted, 0 Withheld

| # | Operation shape | Semantic ID | Disposition |
| ---: | --- | --- | --- |
| 1 | `model-catalogue` | `feature.model-catalogue` | emitted — `KimiLocalServerPreparedCatalogue` |
| 2 | `structured-run` | `feature.structured-run` | emitted — `KimiLocalServerPreparedRun` |
| 3 | `interactive-session` | `feature.interactive-session` | emitted — `KimiLocalServerPreparedSession` |
| 4 | `route-observation` | `feature.streaming-events` | emitted — `KimiLocalServerPreparedRun`; `KimiLocalServerPreparedSession` |
| 5 | `route-capability` | `feature.reasoning-selection` | emitted — `KimiLocalServerPreparedRun` (maximal); `KimiLocalServerPreparedSession` (maximal) |
| 6 | `route-capability` | `feature.permission-exchange` | emitted — `KimiLocalServerPreparedSession` with manual permission extension namespaces |
| 7 | `route-capability` | `feature.question-exchange` | emitted — `KimiLocalServerPreparedSession` with manual permission extension namespaces |
| 8 | `route-capability` | `feature.cancellation-or-interruption` | emitted — `KimiLocalServerPreparedRun`; `KimiLocalServerPreparedSession` |
| 9 | `session-lifecycle` | `feature.resume-session` | emitted — `KimiLocalServerPreparedSession` |
| 10 | `route-capability` | `feature.working-resource` | emitted — `KimiLocalServerPreparedRun`; `KimiLocalServerPreparedSession` |
| 11 | `session-lifecycle` | `feature.stream-reattachment` | emitted — `KimiLocalServerPreparedRun` with a bounded reattachment policy |
| 12 | `session-lifecycle` | `feature.provider-managed-recovery` | emitted — `KimiLocalServerPreparedRun` |
| 13 | `session-lifecycle` | `feature.provider-session-archive` | emitted — `KimiLocalServerPreparedArchive` |
| 14 | `session-lifecycle` | `feature.provider-session-restore` | emitted — `KimiLocalServerPreparedRestore` |
| 15 | `route-capability` | `feature.owned-runtime-lifecycle` | emitted — the owned `HostOwnedEphemeral` topology only |
| 16 | `session-lifecycle` | `feature.persistent-session-posture` | emitted — `KimiLocalServerPreparedRun`; `KimiLocalServerPreparedSession` |
| 17 | `route-capability` | `feature.prepared-facade` | emitted — every local-server prepared facade |
| 18 | `route-observation` | `feature.activity-observation` | emitted — `KimiLocalServerPreparedRun`; `KimiLocalServerPreparedSession` |
| 19 | `structured-run` | `control.model-selection` | emitted — `KimiLocalServerPreparedRun` |
| 20 | `interactive-session` | `control.model-selection` | emitted — `KimiLocalServerPreparedSession` |
| 21 | `structured-run` | `control.reasoning-selection` | emitted — `KimiLocalServerPreparedRun` (maximal) |
| 22 | `interactive-session` | `control.reasoning-selection` | emitted — `KimiLocalServerPreparedSession` (maximal) |
| 23 | `structured-run` | `control.managed-recovery` | emitted — `KimiLocalServerPreparedRun` |
| 24 | `structured-run` | `control.stream-reattachment` | emitted — `KimiLocalServerPreparedRun` |
| 25 | `structured-run` | `control.permission-mode` | emitted — `KimiLocalServerPreparedRun` |
| 26 | `interactive-session` | `control.permission-mode` | emitted — `KimiLocalServerPreparedSession` |
| 27 | `structured-run` | `control.provider-profile` | emitted — `KimiLocalServerPreparedRun` |
| 28 | `interactive-session` | `control.provider-profile` | emitted — `KimiLocalServerPreparedSession` |
| 29 | `structured-run` | `control.disabled-tools` | emitted — `KimiLocalServerPreparedRun` |
| 30 | `interactive-session` | `control.disabled-tools` | emitted — `KimiLocalServerPreparedSession` |
| 31 | `interactive-session` | `control.active-turn-detachment` | emitted — `KimiLocalServerPreparedSession` with detachment enabled |

31 distinct tuples; 31 emitted; 0 withheld.

### `kimi-platform.chat` — 13 Rows, 12 Emitted, 1 Withheld

| # | Operation shape | Semantic ID | Disposition |
| ---: | --- | --- | --- |
| 1 | `model-catalogue` | `feature.model-catalogue` | emitted — `KimiPlatformPreparedCatalogue` |
| 2 | `structured-run` | `feature.structured-run` | emitted — `KimiPlatformPreparedInferenceAttempt` |
| 3 | `route-observation` | `feature.streaming-events` | emitted — `KimiPlatformPreparedInferenceAttempt` |
| 4 | `route-observation` | `feature.usage-evidence` | emitted — `KimiPlatformPreparedInferenceAttempt` |
| 5 | `route-capability` | `feature.output-token-limit` | emitted — `KimiPlatformPreparedInferenceAttempt` |
| 6 | `route-capability` | `feature.reasoning-selection` | emitted — `KimiPlatformPreparedInferenceAttempt` |
| 7 | `route-capability` | `feature.cancellation-or-interruption` | **withheld** — `inference_capabilities()` requires no `Capability::Interruption` |
| 8 | `route-capability` | `feature.prepared-facade` | emitted — `KimiPlatformPreparedCatalogue`; `KimiPlatformPreparedInferenceAttempt` |
| 9 | `route-observation` | `feature.activity-observation` | emitted — `KimiPlatformPreparedInferenceAttempt` |
| 10 | `structured-run` | `control.model-selection` | emitted — `KimiPlatformPreparedInferenceAttempt` only — `prepare_catalogue` builds its plan with no model route |
| 11 | `structured-run` | `control.reasoning-selection` | emitted — `KimiPlatformPreparedInferenceAttempt` |
| 12 | `structured-run` | `control.maximum-output-tokens` | emitted — `KimiPlatformPreparedInferenceAttempt` |
| 13 | `structured-run` | `control.reasoning-and-output-required` | emitted — `KimiPlatformPreparedInferenceAttempt` |

13 distinct tuples; 12 emitted; 1 withheld.
The four ledgers total 89 distinct tuples: 74 emitted, 14 withheld, and 1
undecided in the corrected reassessment arithmetic. Owning packages split 76
`swallowtail-adapter-kimi` and 13 `swallowtail-adapter-kimi-platform` rows.
`ConsumerRouteEvidenceStrength` has no documentation or QA-matrix variant, so
every withheld row is unprojectable by construction. These totals are evidence
only, not proved coverage or implementation authority. The undecided
`kimi-code.acp` `control.provider-session-catalogue` row remains governed by
the provider-operation observation decision; no implementation may pick a direction.

## Required Proofs

### Prepared Profiles

- minimal and maximal `KimiPreparedSession` profiles: reasoning omitted and
  Plan omitted versus both requested. `feature.reasoning-selection` and
  `control.reasoning-selection` are present only in the maximal profile and
  genuinely absent from the minimal one.
- `control.load-session` and `control.resume-session` are the inverse: present
  only in attachment-compatible profiles. Prove that a profile binding
  reasoning or Plan cannot construct `load_request` or `resume_request` —
  `reject_attachment_options` returns
  `swallowtail.kimi.preparation.attachment_reasoning_unsupported` or
  `…attachment_harness_mode_unsupported` — and that both controls are absent
  from that profile's contribution and from the maximal projected open, while
  the route ledger still names each tuple once.
- minimal and maximal `KimiHeadlessPreparedRun`,
  `KimiLocalServerPreparedRun`, `KimiLocalServerPreparedSession`, and
  `KimiPlatformPreparedInferenceAttempt` profiles. Local-server
  `feature.stream-reattachment`, `feature.permission-exchange`,
  `feature.question-exchange`, `control.stream-reattachment`, and
  `control.active-turn-detachment` are present only in their maximal profiles.
- `feature.owned-runtime-lifecycle` is emitted only from the
  `HostOwnedEphemeral` owned topology and absent from every `ExternalAttached`
  facade.
- `kimi-platform.chat` `control.model-selection` is emitted by
  `KimiPlatformPreparedInferenceAttempt` and **absent** from
  `KimiPlatformPreparedCatalogue`, whose plan is built with no model route.
  Assert the absence directly.

### Reasoning Acknowledgement

- `LegacyReasoning` matching confirmation — effective with the exact value.
- `LegacyReasoning` exact `off` against requested `on` — rejected `"off"`
  preserving `swallowtail.negotiated_reasoning.effective_mismatch`.
- `DeclaredEffort`, requested `"on"`, confirmation `"high"` — projected
  effective `"high"`; preserved `open_session` still succeeds with its
  normalized setup and identical cleanup.
- `DeclaredEffort`, requested `"on"`, confirmation `"off"` — rejected `"off"`
  with the same preserved failure code.
- `DeclaredEffort`, requested `"high"`, confirmation `"medium"` — rejected
  `"medium"`.
- missing, malformed, duplicate, ambiguous, unadvertised, transport, and setup
  confirmations — `Runtime` with no contribution on both paths.
- reasoning omitted — no half, no row, no retained active source.
- no exposed outcome or rejected failure carries an unobserved *reasoning*
  half; assert that every pre-reasoning failure takes case 2 and publishes
  nothing.

### Foreign And Unretainable Tokens — Two Disjoint Branches

These are separate fixtures with separate expected codes. A single blanket
assertion does not satisfy this card.

**Pre-lifecycle, case 2 — the new adapter codes must not appear.**

- `DeclaredEffort`, requested `"high"`, confirmation of a foreign catalogue row
  such as `"ultra"` — `NegotiatedReasoningSetup::confirm` already aborts with
  `swallowtail.negotiated_reasoning.effective_mismatch`. The projected path
  returns that exact preserved `RuntimeFailure` with no contribution, no
  session is closed because none was opened, and cleanup matches
  `open_session` byte for byte. Assert the code is **not**
  `reasoning_value_foreign`.
- `DeclaredEffort`, requested `"high"`, confirmation exceeding 128 bytes or
  carrying a control character or surrounding whitespace — same branch, same
  preserved `effective_mismatch`, and **not** `reasoning_value_unbounded`.
- `LegacyReasoning` with any non-matching confirmation — same branch; the
  `{off, on}` domain makes foreign and over-bound tokens unreachable here.

**Projection-only, case 4 — the sole branch the new codes belong to.**

- `DeclaredEffort`, requested `"on"`, confirmation of a foreign catalogue row —
  `confirm` normalizes to `"on"` and the lifecycle succeeds, so
  `open_session` returns a live session on this fixture. The projected path
  closes that session and returns `Runtime` with
  `swallowtail.kimi.acp.reasoning_value_foreign` and no contribution.
- `DeclaredEffort`, requested `"on"`, confirmation exceeding 128 bytes or
  carrying a control character or surrounding whitespace — same branch with
  `swallowtail.kimi.acp.reasoning_value_unbounded`; `open_session` still
  succeeds on the identical fixture.
- Assert this branch is unreachable elsewhere: no `LegacyReasoning`, concrete
  `DeclaredEffort`, or Plan fixture may produce either new code.

### Plan Acknowledgement

- confirmation `currentValue = "plan"` — effective `"plan"`.
- confirmation exactly `default`, `auto`, or `yolo` against the frozen ordered
  `["default", "plan", "auto", "yolo"]` domain — rejected, preserving
  `swallowtail.kimi.acp.harness_mode_mismatch`.
- any other shape — `Runtime` with no contribution on both paths.
- Plan omitted — no half, no row.
- both halves omitted — no row and no active source in the contribution.

### Maximal Early Stop — Reasoning Rejects While Plan Was Requested

The retained early-stop evidence covers a maximal prepared session requesting
reasoning `"high"` and Plan while the provider confirms `"medium"`.

- the lifecycle aborts at the reasoning `confirm` call and never reaches
  `mode::prepare_plan_mode`; assert no `set_config_option` for `"mode"` was
  dispatched, so no further provider work occurred;
- `open_session` returns `swallowtail.negotiated_reasoning.effective_mismatch`
  with its existing cleanup, unchanged from `main`;
- `open_session_with_projection` returns `Rejected` carrying that same
  `RuntimeFailure` and no session;
- any compound row associates the exact reasoning rejection with the Plan
  half's exact requested-but-not-observed state. It must not map that Plan half
  to `with_pending()`: no acknowledgement was dispatched and the failure is
  terminal;
- a generic Contract 061 consumer can read each half's state without
  downcasting to a Kimi type; and
- no model-option row accompanies the rejection.

Also prove the symmetric termination: reasoning effective with Plan requested
but its option missing or malformed takes case 2 and publishes nothing.

### Model Observation

- exact `model` option with a bounded advertised list — the same
  `NegotiatedSessionModelOptions` snapshot is reachable from
  `session().negotiated_model_options()` and
  `outcome.negotiated_model_options()`, and one observation row is published.
- `model` option with a current value and no `options` array — snapshot `None`,
  no row, open still succeeds on both paths.
- missing or malformed `model` option — `swallowtail.kimi.acp.malformed_response`
  on both paths, unchanged from `main`.
- model observation is present on open, load, and resume handles but published
  only from the projected open.

### Prepared Catalogue

- prepared catalogue evidence emits exactly the prepared
  `feature.provider-session-catalogue` row and **no**
  `control.provider-session-catalogue` row in any state. Assert its absence
  directly.
- `list_sessions`, `list_page`, and `next_page_request` produce results
  identical to `main` and publish no row.

### Sources, Mixtures, And Preservation

- equal prepared and active source IDs supplied to the open seam fail with
  `swallowtail.kimi.projection_source_identity_invalid` before any process,
  connection, or provider work.
- an active source that names no published active row is absent from the
  contribution.
- matching-source cross-route, cross-operation, cross-instance,
  stale-revision, and each exact access-dimension drift fail closed in both
  directions across all four routes.
- the preserved and projected open paths return identical route failure codes,
  handle shapes, and cleanup outcomes for every reasoning and Plan fixture.
- support posture, availability, lifecycle band, omission semantics, actor
  posture, and mutation posture are proved per identity, not per route.

## Out Of Scope

- choosing the provider-operation observation decision, or implementing this
  card
- candidates B, C, E, I-L, Batch 9.5, or generation closeout
- runtime, testkit, core, or any other adapter public API; `ReasoningMode`,
  `NegotiatedReasoningSetup`, `EffectiveReasoningSetup`, contracts,
  architecture, census, compatibility, or route-claim changes
- any projected catalogue seam, or reinterpreting
  `ActiveSessionObservation`, `PostOpenObservationOnly`, or
  `ConsumerRouteActiveSessionState` in adapter documentation
- projecting `load_session`, `resume_session`,
  `prepare_working_state_restoration`, `list_page`, or `next_page_request`
- model mutation, model-catalogue discovery through a session, generic
  active-observation payloads, callbacks, registries, runtime route
  enumeration, or adapter downcasts
- provider contact, live probes, currentness, watcher, skill-discovery,
  papercut, or PAPERCUTS work

## Acceptance Criteria

None may be checked. This card is planned, not ready, and no implementation
acceptance has been earned.

- [ ] the provider-operation observation public-baseline decision is answered
      before any of the following is attempted
- [ ] four independent ledgers reconcile exactly to 25, 20, 31, and 13 rows,
      with each `(route_id, operation_shape, semantic_id)` named once and no
      exception list
- [ ] the corrected interim 89-row arithmetic of 74 emitted, 14 withheld, and
      1 undecided is re-derived as evidence only; it is not counted as proved
      coverage or implementation authority, and minimal and maximal profiles prove every
      optional row is genuinely absent or present
- [ ] every emitted row retains exact source, route, operation, lifecycle,
      value, omission, applicability, access, evidence, support, availability,
      actor posture, and mutation posture
- [ ] prepared `kimi-code.acp` reasoning is requested/prepared/pending and
      never provider-effective or rejected; `control.session-options` names
      only the accepted reasoning and Plan subset
- [ ] the acknowledgement row publishes the exact provider-confirmed effort
      under requested `"on"` and never the normalized `"on"`
- [ ] a maximal request whose reasoning rejects associates the exact reasoning
      rejection with the requested-but-not-observed Plan half, performs no
      further provider work, does not map that terminal state to
      `with_pending()`, preserves `driver.rs`'s confirmation order, and is
      readable generically without an adapter downcast
- [ ] pre-lifecycle foreign or unretainable tokens return the exact preserved
      `RuntimeFailure` with no contribution and without emitting either new
      adapter code
- [ ] projection-only foreign or unretainable tokens — reachable only under
      `DeclaredEffort` with requested `"on"` — close the opened session and
      return the exact new adapter code with no contribution, while
      `open_session` succeeds on the identical fixture
- [ ] `open_session` behavior is byte-identical to `main` on every fixture in
      both branches
- [ ] exact model options survive on both the generic handle and the outcome
      accessor; absent data stays absent; malformed data fails both paths with
      the existing code
- [ ] `control.provider-session-catalogue` is never emitted by the prepared
      catalogue facade in any state
- [ ] `control.load-session` and `control.resume-session` are emitted only from
      attachment-compatible profiles and are absent from maximal reasoning/Plan
      profiles
- [ ] `kimi-platform.chat` `control.model-selection` is emitted only by
      `KimiPlatformPreparedInferenceAttempt` and is absent from
      `KimiPlatformPreparedCatalogue`
- [ ] `KimiReasoningAcknowledgement::RequestedNotObserved` is not introduced as
      an accepted public baseline; fixed reasoning-first order makes it
      speculative and unreachable
- [ ] `feature.persistent-session-posture` on `kimi-code.acp` comes only from
      `KimiPreparedSessionImport`
- [ ] all 14 withheld rows are withheld at construction with their exact
      capability, role, policy, or descriptor-only reason
- [ ] preserved and projected open paths share one private lifecycle and cannot
      drift in setup, failure code, handle, or cleanup
- [ ] only `swallowtail-adapter-kimi` and `swallowtail-adapter-kimi-platform`
      semantic API baselines change; `swallowtail-adapter-kimi` is added to
      `release-baselines/public-api-unreleased/packages.txt` with its own
      unreleased baseline file
- [ ] shared APIs, contracts, census, provider claims, and compatibility claims
      remain unchanged
- [ ] touched source stays below configured god-file thresholds and the scan
      does not exceed the accepted 387-finding repository baseline

## Review Oracle

Invariant: this card is planned and not ready. Nothing in it authorizes
implementation, and none of its 89 rows counts toward coverage.

Counterexamples and required proof:

- treat this card as ready, implement it, or count any of its rows as proved —
  fail; coverage is 249/518
- answer the provider-operation observation decision inside this card, or pick
  a shared runtime direction — fail
- add a projected catalogue seam, or publish
  `control.provider-session-catalogue` through
  `ActiveSessionObservation`, `PostOpenObservationOnly`, or
  `ConsumerRouteActiveSessionState` while those remain session-scoped — fail
- emit `control.provider-session-catalogue` from
  `KimiPreparedSessionCatalogue::consumer_route_projection_contribution`, in
  any state — fail; prepared evidence would backdate observed truth
- publish a foreign or over-bound Kimi token as portable reasoning truth —
  fail; no row publishes on either branch
- answer a pre-lifecycle foreign or over-bound token — `DeclaredEffort`
  requested `"high"` confirmed foreign — with `reasoning_value_foreign` or
  `reasoning_value_unbounded` — fail; the lifecycle already aborted with
  `swallowtail.negotiated_reasoning.effective_mismatch`, no session exists to
  close, and case 2 must return that preserved failure unchanged
- answer a projection-only foreign or over-bound token — `DeclaredEffort`
  requested `"on"` confirmed foreign or over-bound — with the preserved success
  or with `effective_mismatch` — fail; case 4 must close the opened session and
  return the new adapter code
- describe or prove case 4 as a general fallback rather than the
  reasoning-only, requested-`"on"`-only branch — fail
- lose the fact that Plan was requested but not observed when a maximal request's
  reasoning rejected before Plan dispatch, or map that terminal state to
  `with_pending()` — fail
- require an adapter downcast to read either half's state — fail Contract 061's
  facade requirement
- introduce a reachable unobserved *reasoning* half — fail; reasoning is
  confirmed first
- publish `"on"` in place of the exact provider-confirmed effort — fail
- change `EffectiveReasoningSetup`, `NegotiatedReasoningSetup`,
  `ReasoningMode`, or another shared public type — fail
- let the preserved and projected opens differ in setup, failure code, or
  cleanup for the same fixture — fail the shared-lifecycle proof
- give `load_session`, `resume_session`, or continuation recovery projection
  authority from the open-only decision — fail
- emit `control.load-session` or `control.resume-session` from a maximal
  reasoning/Plan profile — fail; `reject_attachment_options` makes those
  requests unconstructible
- attribute `kimi-platform.chat` `control.model-selection` to
  `KimiPlatformPreparedCatalogue` — fail; its plan binds no model route
- treat model options as selection, mutation, acknowledgement, or catalogue
  authority — fail the observation-only posture
- retain a session or a model-option row on a rejected acknowledgement — fail;
  the open did not complete
- reuse one source ID for prepared and active rows on the open seam — fail
  before provider work
- attach a `kimi-code.acp` active row to `kimi-code.headless`,
  `kimi-code.local-server`, or `kimi-platform.chat` — fail route and operation
  applicability
- emit `feature.persistent-session-posture` from the `Prohibited`
  interactive-session plan — fail the operation-shape split
- emit any of the 14 withheld rows from documentation or the provider matrix —
  fail; no evidence-strength variant admits it
- reach 25, 20, 31, or 13 through a filtered superset, exception list,
  duplicated semantic identity, or borrowed route identity — fail ledger
  reconciliation

The ledger source is generated from the reviewed CSV. Observed tuple mapping is
maintained independently of ledger disposition so a wrong route,
operation-shape label, or semantic identity cannot agree with itself.

## Validation

None while planned and not ready. The planning batch that carries this card runs
`effigy qa:docs`, `effigy qa:northstar`, and `git diff --check` only.

Once unblocked, an implementation PR would name:

- `cargo fmt -p swallowtail-adapter-kimi -p swallowtail-adapter-kimi-platform -- --check`
- `effigy validate:focused swallowtail-adapter-kimi swallowtail-adapter-kimi-platform`
- `effigy package:verify-affected swallowtail-adapter-kimi swallowtail-adapter-kimi-platform`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy --json scan god-files`
- `git diff --check`

No live probe or provider contact belongs to this card.

## Auto-Continuation

No. This card is planned but not ready pending the provider-operation
observation public-baseline decision. Do not dispatch an implementation worker
and do not compile a replacement card.

## Stop Conditions

- Stop while the provider-operation observation public-baseline decision is
  open; that is the current state.
- Stop if the card needs a runtime/testkit/core public type, source kind,
  composer rule, fixed maximum, failure kind, or contract amendment beyond
  whatever the provider-operation observation decision authorizes.
- Stop if exact reasoning or Plan rejection requires accepting an unadvertised,
  ambiguous, foreign, or unbounded value, or a raw ACP payload.
- Stop if the preserved `open_session`, `load_session`, `resume_session`,
  `list_sessions`, `list_page`, `next_page_request`, or cleanup behavior cannot
  stay unchanged.
- Stop if the unresolved acknowledgement shape cannot preserve each half's
  exact state generically without an adapter downcast or invented pending
  state. `RequestedNotObserved` must not be mapped to `with_pending()` when no
  acknowledgement was dispatched and the failure is terminal.
- Stop if any 25/20/31/13 ledger needs an exception list, inferred support, or
  truth borrowed from another route or operation.
- Stop if the corrected 74/14/1 reassessment arithmetic cannot be preserved
  from current source; report the exact divergence instead of adjusting the census.
- Stop if scope widens to another candidate, Batch 9.5, shared public API
  beyond the provider-operation observation decision, provider contact, or
  another product track.

## Evidence

- [stopped Kimi active-observation gate](../../../triage/2026-09-01-contract-061-kimi-active-observation-public-baseline-gate.md)
- [Batch 9.4 package expansion](../../../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [completed card 033](033-contract-061-card-032-closeout-and-kimi-reassessment.md)
- [completed card 032](032-contract-061-cline-command-code-copilot-goose-package-completion.md)
- [Contract 061](../../../contracts/061-consumer-route-feature-and-control-projection.md)
- [reviewed census](../../../triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [Batch 9.1 public baseline](../../../triage/2026-08-31-contract-061-batch-9-1-public-baseline-gate.md)
