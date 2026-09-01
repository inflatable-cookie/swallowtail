# 034 Contract 061 Kimi And Kimi Platform Package Completion

Status: ready
Owner: Tom
Created: 2026-09-01
Updated: 2026-09-01
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: completed card 033; accepted Kimi active-observation
public-baseline gate

## Goal

Complete candidate F's exact 89-row Contract 061 package remainder and expose
exact `kimi-code.acp` compound reasoning-and-Plan acknowledgement, bounded
negotiated model options, and post-open provider-session catalogue observation
through the two accepted additive adapter-owned seams.

## Scope

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
2. Add `KimiProviderValue`, `KimiReasoningAcknowledgement`,
   `KimiPlanAcknowledgement`, `KimiProjectionOpenFuture`,
   `KimiProjectionOpenOutcome`, `KimiProjectionOpenFailure`,
   `KimiCatalogueProjectionFuture`, `KimiCatalogueProjectionOutcome`, and
   `KimiCatalogueProjectionFailure` with the exact signatures and accessors
   fixed by the
   [public-baseline gate](../../../triage/2026-09-01-contract-061-kimi-active-observation-public-baseline-gate.md).
   `KimiProviderValue::new` is not public.
3. Add `KimiPreparedSession::open_session_with_projection` with distinct
   prepared and active-session source IDs. Serve it and `open_session` from one
   private open lifecycle that records each half's exact confirmation
   `currentValue` without changing control flow. Keep `open_session`,
   `load_request`, `load_session`, `resume_request`, `resume_session`,
   `prepare_working_state_restoration`, and `into_parts` exactly as they are on
   `main`, including the `DeclaredEffort` requested-`"on"` normalization.
4. Add `KimiPreparedSessionCatalogue::list_sessions_with_projection` over the
   preserved `list_sessions` path with its own prepared and observed source
   IDs. Keep `list_sessions`, `list_page`, `next_page_request`, continuation
   paging, candidate projection, cursor semantics, failure stages, and cleanup
   unchanged and unprojected.
5. Add one adapter-private `MAXIMUM_KIMI_PROVIDER_VALUE_BYTES = 128` constant
   and one adapter-private admission function. Do not import, re-export, or
   alias `swallowtail-core`'s `ProviderCatalogValue` bound and do not add a
   shared constant. Retain a confirmation token only when it is non-blank,
   untrimmed-equal, control-free, and within the bound.
6. Publish the compound acknowledgement row only from admitted tokens. Under
   `DeclaredEffort` with requested `"on"`, publish the exact provider-confirmed
   effort, not `"on"`. Return `Rejected` only for
   `swallowtail.negotiated_reasoning.effective_mismatch` and
   `swallowtail.kimi.acp.harness_mode_mismatch` with the retained exact value.
   Foreign or unretainable tokens close the opened session and return `Runtime`
   with `swallowtail.kimi.acp.reasoning_value_foreign` or
   `swallowtail.kimi.acp.reasoning_value_unbounded`, publishing no row.
7. Publish `feature.negotiated-model-options-observation` only when
   `parse_model_options` returned `Some`. Change neither `parse_model_options`
   nor `validate_session_configuration`, and introduce no preserved-versus-
   projected model split where current `main` fails both paths identically.
8. Publish all three Kimi-only active identities plus the prepared
   `control.provider-session-catalogue` and `control.provider-session-import`
   identities as bounded namespaced extensions qualified by exact route ID and
   exact `protocol_facade_id`. Do not substitute
   `ConsumerRouteControlId::SessionCatalogueBounds`.
9. Disposition exactly 89 census tuples with no filter or exception list: 25
   `kimi-code.acp`, 20 `kimi-code.headless`, 31 `kimi-code.local-server`, and
   13 `kimi-platform.chat`. Name each tuple once with an emitted or
   construction-time-withheld reason matching the ledgers below.
10. Prove these maximal ledger totals independently: `kimi-code.acp` 22/3,
    `kimi-code.headless` 10/10, `kimi-code.local-server` 31/0, and
    `kimi-platform.chat` 12/1 emitted/withheld — 75 emitted and 14 withheld
    across four independent ledgers generated from the reviewed CSV.
11. Emit `feature.persistent-session-posture` on `kimi-code.acp` only from
    `KimiPreparedSessionImport`. `KimiPreparedSession` must not emit it; its
    plan is `SessionProviderStatePolicy::Prohibited`.
12. Add deterministic provider-free fixtures and the exact proof set named
    below. Stop after one reviewable two-package PR.

## Exact Four-Route Ledgers

Each row is one `(route_id, operation_shape, semantic_id)` census tuple. The
dispositions are derived from the reviewed census plus current `main` driver
roles, capability profiles, extension namespaces, ownership modes, and
provider-state policies — not from the provider feature matrix and not from
card 033's provisional 86/3 split.

### `kimi-code.acp` — 25 Rows, 22 Emitted, 3 Withheld

| # | Operation shape | Semantic ID | Emitted by / withheld because |
| ---: | --- | --- | --- |
| 1 | `model-catalogue` | `feature.model-catalogue` | **withheld** — no `DriverRole::ModelCatalog`; no ACP plan requires `Capability::ModelCatalog` |
| 2 | `structured-run` | `feature.structured-run` | **withheld** — the ACP descriptor declares no `StructuredRun` role or shape |
| 3 | `interactive-session` | `feature.interactive-session` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 4 | `route-observation` | `feature.streaming-events` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 5 | `route-capability` | `feature.reasoning-selection` | emitted — `KimiPreparedSession` (maximal); `open_session_with_projection` (maximal) |
| 6 | `route-capability` | `feature.cancellation-or-interruption` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 7 | `session-lifecycle` | `feature.load-session` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 8 | `session-lifecycle` | `feature.resume-session` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 9 | `session-lifecycle` | `feature.provider-session-catalogue` | emitted — `KimiPreparedSessionCatalogue`; `list_sessions_with_projection` |
| 10 | `session-lifecycle` | `feature.provider-session-import` | emitted — `KimiPreparedSessionImport` |
| 11 | `route-capability` | `feature.working-resource` | emitted — `KimiPreparedSession`; `open_session_with_projection`; `KimiPreparedSessionCatalogue`; `KimiPreparedSessionImport` |
| 12 | `route-capability` | `feature.bounded-workspace-text-write` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 13 | `session-lifecycle` | `feature.provider-managed-recovery` | **withheld** — no ACP plan requires `Capability::ProviderManagedRecovery`; continuation recovery is load-session authority |
| 14 | `session-lifecycle` | `feature.persistent-session-posture` | emitted — `KimiPreparedSessionImport` only — the session plan is `Prohibited` |
| 15 | `route-capability` | `feature.prepared-facade` | emitted — `KimiPreparedSession`; `KimiPreparedSessionCatalogue`; `KimiPreparedSessionImport`; `open_session_with_projection`; `list_sessions_with_projection` |
| 16 | `route-observation` | `feature.activity-observation` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 17 | `interactive-session` | `feature.active-session-reasoning-and-plan-ack` | emitted — `open_session_with_projection` only |
| 18 | `interactive-session` | `feature.negotiated-model-options-observation` | emitted — `open_session_with_projection` only |
| 19 | `interactive-session` | `control.model-selection` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 20 | `interactive-session` | `control.reasoning-selection` | emitted — `KimiPreparedSession` (maximal); `open_session_with_projection` (maximal) |
| 21 | `interactive-session` | `control.session-options` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 22 | `session-management` | `control.load-session` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 23 | `session-management` | `control.resume-session` | emitted — `KimiPreparedSession`; `open_session_with_projection` |
| 24 | `session-management` | `control.provider-session-catalogue` | emitted — `KimiPreparedSessionCatalogue`; `list_sessions_with_projection` |
| 25 | `session-management` | `control.provider-session-import` | emitted — `KimiPreparedSessionImport` |

25 distinct tuples; 22 emitted; 3 withheld.

### `kimi-code.headless` — 20 Rows, 10 Emitted, 10 Withheld

| # | Operation shape | Semantic ID | Emitted by / withheld because |
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

| # | Operation shape | Semantic ID | Emitted by / withheld because |
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

| # | Operation shape | Semantic ID | Emitted by / withheld because |
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
| 10 | `structured-run` | `control.model-selection` | emitted — `KimiPlatformPreparedCatalogue`; `KimiPlatformPreparedInferenceAttempt` |
| 11 | `structured-run` | `control.reasoning-selection` | emitted — `KimiPlatformPreparedInferenceAttempt` |
| 12 | `structured-run` | `control.maximum-output-tokens` | emitted — `KimiPlatformPreparedInferenceAttempt` |
| 13 | `structured-run` | `control.reasoning-and-output-required` | emitted — `KimiPlatformPreparedInferenceAttempt` |

13 distinct tuples; 12 emitted; 1 withheld.
The four ledgers total 89 distinct tuples, 75 emitted and 14 withheld. Owning
packages split 76 `swallowtail-adapter-kimi` and 13
`swallowtail-adapter-kimi-platform` rows. `ConsumerRouteEvidenceStrength` has
no documentation or QA-matrix variant, so every withheld row is unprojectable
by construction.

## Required Proofs

### Prepared Profiles

- minimal and maximal `KimiPreparedSession` profiles: reasoning omitted and
  Plan omitted versus both requested. `feature.reasoning-selection` and
  `control.reasoning-selection` are present only in the maximal profile and
  genuinely absent from the minimal one.
- minimal and maximal `KimiHeadlessPreparedRun`,
  `KimiLocalServerPreparedRun`, `KimiLocalServerPreparedSession`, and
  `KimiPlatformPreparedInferenceAttempt` profiles. Local-server
  `feature.stream-reattachment`, `feature.permission-exchange`,
  `feature.question-exchange`, `control.stream-reattachment`, and
  `control.active-turn-detachment` are present only in their maximal profiles.
- `feature.owned-runtime-lifecycle` is emitted only from the
  `HostOwnedEphemeral` owned topology and absent from every `ExternalAttached`
  facade.

### Reasoning Acknowledgement

- `LegacyReasoning` matching confirmation — `Effective` with the exact value.
- `LegacyReasoning` exact `off` against requested `on` — `Rejected("off")`
  preserving `swallowtail.negotiated_reasoning.effective_mismatch`.
- `DeclaredEffort`, requested `"on"`, confirmation `"high"` — projected
  `Effective("high")`; preserved `open_session` still succeeds with its
  normalized setup and identical cleanup.
- `DeclaredEffort`, requested `"on"`, confirmation `"off"` — `Rejected("off")`
  with the same preserved failure code.
- `DeclaredEffort`, requested `"high"`, confirmation `"medium"` —
  `Rejected("medium")`.
- `DeclaredEffort`, requested `"high"`, confirmation of a foreign catalogue row
  — projected `Runtime` with
  `swallowtail.kimi.acp.reasoning_value_foreign`, no contribution; preserved
  path unchanged.
- confirmation exceeding 128 bytes, or containing a control character or
  surrounding whitespace — projected `Runtime` with
  `swallowtail.kimi.acp.reasoning_value_unbounded`, no contribution.
- missing, malformed, duplicate, ambiguous, unadvertised, transport, and setup
  confirmations — `Runtime` with no contribution on both paths.
- reasoning omitted — no half, no row, no retained active source.

### Plan Acknowledgement

- confirmation `currentValue = "plan"` — `Effective("plan")`.
- confirmation exactly `default`, `auto`, or `yolo` against the frozen ordered
  `["default", "plan", "auto", "yolo"]` domain — `Rejected(value)` preserving
  `swallowtail.kimi.acp.harness_mode_mismatch`.
- any other shape — `Runtime` with no contribution on both paths.
- Plan omitted — no half, no row.
- reasoning effective with Plan rejected, and reasoning rejected with Plan
  omitted, both produce exactly one compound row with the correct union state
  support.
- both halves omitted — no row and no active source in the contribution.

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

### Catalogue Observation

- successful `list_sessions_with_projection` — the preserved
  `ProviderSessionCatalogueOutcome` candidates, cursor, and cleanup are intact,
  and exactly one `control.provider-session-catalogue` row with
  `ProviderSessionCatalogue` applicability and the observed catalogue source is
  published.
- cancelled, timed-out, dispatch, list, projection, and cleanup failure stages
  — `KimiCatalogueProjectionFailure::Operation` with no contribution.
- prepared catalogue evidence alone emits the prepared
  `feature.provider-session-catalogue` and
  `control.provider-session-catalogue` rows without observed state.
- `list_page` and `next_page_request` continuation produce identical results to
  `main` and publish no row.
- catalogue observation never carries the interactive-session active source ID
  or `OperationShape::InteractiveSession`.

### Sources, Mixtures, And Preservation

- equal prepared and active source IDs fail with
  `swallowtail.kimi.projection_source_identity_invalid` before any process,
  connection, or provider work, on both the open and catalogue seams.
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

- candidates B, C, E, I-L, Batch 9.5, or generation closeout
- runtime, testkit, core, or any other adapter public API; `ReasoningMode`,
  `NegotiatedReasoningSetup`, `EffectiveReasoningSetup`, contracts,
  architecture, census, compatibility, or route-claim changes
- projecting `load_session`, `resume_session`,
  `prepare_working_state_restoration`, `list_page`, or `next_page_request`
- model mutation, model-catalogue discovery through a session, generic
  active-observation payloads, callbacks, registries, runtime route
  enumeration, or adapter downcasts
- provider contact, live probes, currentness, watcher, skill-discovery,
  papercut, or PAPERCUTS work

## Acceptance Criteria

- [ ] four independent ledgers reconcile exactly to 25, 20, 31, and 13 rows,
      with each `(route_id, operation_shape, semantic_id)` named once and no
      exception list
- [ ] maximal dispositions equal 22/3, 10/10, 31/0, and 12/1 emitted/withheld,
      75 emitted and 14 withheld overall; minimal and maximal profiles prove
      every optional row is genuinely absent or present
- [ ] every emitted row retains exact source, route, operation, lifecycle,
      value, omission, applicability, access, evidence, support, availability,
      actor posture, and mutation posture
- [ ] prepared `kimi-code.acp` reasoning is requested/prepared/pending and
      never provider-effective or rejected; `control.session-options` names
      only the accepted reasoning and Plan subset
- [ ] the compound acknowledgement row publishes both halves independently,
      with the exact provider-confirmed effort under requested `"on"` and
      never the normalized `"on"`
- [ ] foreign and unretainable confirmation tokens fail closed on the projected
      path with their exact adapter codes and publish no row, while
      `open_session` behavior is byte-identical to `main`
- [ ] exact model options survive on both the generic handle and the outcome
      accessor; absent data stays absent; malformed data fails both paths with
      the existing code
- [ ] catalogue observation is session-management truth with its own source and
      applicability, and prepared catalogue success never sets observed state
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

Invariant: candidate F publishes 89 exact route tuples, while only an exact
`kimi-code.acp` open observation may add acknowledgement or model-option state
and only an exact `kimi-code.acp` catalogue observation may add observed
catalogue state.

Counterexamples and required proof:

- publish a foreign or over-bound Kimi token as portable reasoning truth —
  fail; return the exact adapter runtime code with no contribution
- publish `"on"` in place of the exact provider-confirmed effort — fail the
  exact-effective proof
- present a static mismatch diagnostic as a rejection without the retained
  exact value — fail as runtime with no contribution
- change `EffectiveReasoningSetup`, `NegotiatedReasoningSetup`,
  `ReasoningMode`, or another shared public type — fail
- let the preserved and projected opens differ in setup, failure code, or
  cleanup for the same fixture — fail the shared-lifecycle proof
- give `load_session`, `resume_session`, or continuation recovery projection
  authority from the open-only decision — fail
- present prepared catalogue success as observed catalogue truth — fail
- give catalogue observation the interactive-session active source or operation
  shape — fail
- publish a catalogue row from `list_page` or a continuation cursor — fail
- treat model options as selection, mutation, acknowledgement, or catalogue
  authority — fail the observation-only posture
- retain a session or a model-option row on a rejected acknowledgement — fail;
  the open did not complete
- reuse one source ID for prepared and active rows on either seam — fail before
  provider work
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
- claim proved coverage for these 89 rows before this card merges — fail

The ledger source is generated from the reviewed CSV. Observed tuple mapping is
maintained independently of ledger disposition so a wrong route,
operation-shape label, or semantic identity cannot agree with itself.

## Validation

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

No. Stop after one reviewable PR. The orchestrator must review the exact 89-row
proof, both adapter API baselines, and the open and catalogue preservation
evidence before another Batch 9.4 candidate is reassessed or promoted.

## Stop Conditions

- Stop if the card needs a runtime/testkit/core public type, source kind,
  composer rule, fixed maximum, failure kind, or contract amendment.
- Stop if exact reasoning or Plan rejection requires accepting an unadvertised,
  ambiguous, foreign, or unbounded value, or a raw ACP payload.
- Stop if the preserved `open_session`, `load_session`, `resume_session`,
  `list_sessions`, `list_page`, `next_page_request`, or cleanup behavior cannot
  stay unchanged.
- Stop if interactive-session acknowledgement, model-option observation, and
  session-management catalogue observation cannot remain three distinct
  sources, lifecycles, and applicabilities.
- Stop if any 25/20/31/13 ledger needs an exception list, inferred support, or
  truth borrowed from another route or operation.
- Stop if the derived 22/3, 10/10, 31/0, or 12/1 disposition cannot be proved
  from current source; report the exact divergence instead of adjusting the
  census.
- Stop if scope widens to another candidate, Batch 9.5, shared public API,
  provider contact, or another product track.

## Evidence

- [Kimi active-observation public-baseline gate](../../../triage/2026-09-01-contract-061-kimi-active-observation-public-baseline-gate.md)
- [Batch 9.4 package expansion](../../../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [completed card 033](033-contract-061-card-032-closeout-and-kimi-reassessment.md)
- [completed card 032](032-contract-061-cline-command-code-copilot-goose-package-completion.md)
- [Contract 061](../../../contracts/061-consumer-route-feature-and-control-projection.md)
- [reviewed census](../../../triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [Batch 9.1 public baseline](../../../triage/2026-08-31-contract-061-batch-9-1-public-baseline-gate.md)
