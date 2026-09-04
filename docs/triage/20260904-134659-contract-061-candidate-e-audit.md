# Contract 061 Candidate E Breadth Audit

Status: active planning evidence; Candidate E audited against current `main`
Owner: Tom
Date: 2026-09-04
Source: Contract 061, Batch 9.4 checkpoint, Card 065, and `main` at `bab21839321a1b29da0b14209db32c8323a9d1c2`

## Trigger

Card 065 audits Batch 9.4 candidate E (`swallowtail-adapter-gemini` and
`swallowtail-adapter-grok`) against current `main` under the promotion rubric.
The lane is planning-only evidence work: no Rust changes, no provider contact,
and no claim modification.

## Exact Census Reconciliation

Candidate E covers 56 rows across 4 route IDs and 2 adapter packages in
`docs/triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv`:

| Route ID | Owning Package | Total Census Rows | Sensitive Truth Retained |
| --- | --- | ---: | --- |
| `gemini-cli.acp` | `swallowtail-adapter-gemini` | 14 | ACP interactive session, Plan harness mode, model options observation |
| `gemini-cli.headless` | `swallowtail-adapter-gemini` | 13 | Headless structured run, usage reporting, model selection |
| `gemini.live` | `swallowtail-adapter-gemini` | 16 | Hosted realtime media session, output limit, reasoning, media config, rollover |
| `grok-build.acp` | `swallowtail-adapter-grok` | 13 | ACP dual-shape (interactive session / structured run), model selection, empty options |
| **Total** | **2 packages** | **56** | **All candidate E rows assigned once with no filter or exception list** |

Candidate E contains zero `audit.no-public-route-specific-selectable-control`
rows (negative coverage is 0 rows, unlike candidates C, G, and H).

## Facade Map And Source Identities

### Contributing Prepared Facades On Current `main`

Every prepared facade exists on current `main` and contributes under
`ConsumerRouteProjectionSourceKind::AdapterContribution`:

1. `gemini-cli.acp` (interactive-session):
   `GeminiPreparedSession` in `crates/swallowtail-adapter-gemini/src/prepared_profile/session.rs:19`.
   Preflight plan and capability profile derived via `session_capabilities()` in
   `crates/swallowtail-adapter-gemini/src/prepared/instance.rs:40`.
2. `gemini-cli.headless` (structured-run):
   `GeminiHeadlessPreparedRun` in `crates/swallowtail-adapter-gemini/src/prepared_headless/profile.rs:192`.
   Preflight plan and capability profile derived via `run_capabilities()` in
   `crates/swallowtail-adapter-gemini/src/prepared_headless/instance.rs:40`.
3. `gemini.live` (realtime-media-session):
   `GeminiPreparedLiveSession` in `crates/swallowtail-adapter-gemini/src/prepared_live_profile/session.rs:20`.
   Preflight plan, media config, and rollover policy derived via `gemini_live_base_capabilities()`
   in `crates/swallowtail-adapter-gemini/src/live_selection.rs:222`.
4. `grok-build.acp` (interactive-session):
   `GrokPreparedSession` in `crates/swallowtail-adapter-grok/src/prepared_profile.rs:132`.
   Preflight plan derived via `session_capabilities()` in
   `crates/swallowtail-adapter-grok/src/prepared/instance.rs:39`.
5. `grok-build.acp` (structured-run):
   `GrokPreparedRun` in `crates/swallowtail-adapter-grok/src/prepared_profile/run.rs:33`.
   Preflight plan derived via `run_capabilities()` in
   `crates/swallowtail-adapter-grok/src/prepared/instance.rs:82`.

### Active-Observation Facades On Current `main` (Absence Proof)

On current `main`, neither adapter exports an active-observation facade,
`ActiveSessionObservation` source, or projected-open seam:

- `GeminiPreparedSession::open_session` in `crates/swallowtail-adapter-gemini/src/prepared_profile/session.rs:50`
  returns `GeminiPreparedSessionFuture` yielding `Box<dyn InteractiveSessionHandle>`.
  `GeminiInteractiveSession` in `crates/swallowtail-adapter-gemini/src/driver/session.rs:13,37`
  holds `model_options: Option<NegotiatedSessionModelOptions>` and exposes it via
  `InteractiveSessionHandle::negotiated_model_options`, but no `open_session_with_projection`
  or projection outcome exists in the crate.
- `GeminiPreparedLiveSession::open_session` in `crates/swallowtail-adapter-gemini/src/prepared_live_profile/session.rs:56`
  returns `BoxFuture<'static, Result<Box<dyn RealtimeMediaSessionHandle>, RuntimeFailure>>`.
  No projected-open seam exists.
- `GrokPreparedSession::open_session` in `crates/swallowtail-adapter-grok/src/prepared_profile.rs:167`
  returns `GrokPreparedSessionFuture` yielding `Box<dyn InteractiveSessionHandle>`.
  `GrokInteractiveSession` in `crates/swallowtail-adapter-grok/src/driver/session.rs:14,42`
  holds `model_options: NegotiatedSessionModelOptions` and exposes it via
  `InteractiveSessionHandle::negotiated_model_options`, but no `open_session_with_projection`
  or projection outcome exists in the crate.
- `GrokPreparedRun::start_run` in `crates/swallowtail-adapter-grok/src/prepared_profile/run.rs:68`
  starts the structured run and returns `GrokPreparedRunFuture` yielding `Result<Box<dyn RunHandle>, RuntimeFailure>`.
  No projection outcome exists.

## Applicability Truth And Separation

Batch 9.4 classed candidate E viable later because ACP, headless, and live
applicability need a three-family proof that keeps each applicability distinct.
The code inspection shows that `gemini.live` and `gemini-cli.acp` actually **share**
`OperationShape::InteractiveSession` (`crates/swallowtail-adapter-gemini/src/live_selection.rs:190`
and `src/live.rs:209`), because `OperationShape` in `swallowtail-core` has no realtime-media
variant.

However, three-family separation remains absolute and fully proved on the specific
dimensions that actually distinguish the routes:

| Dimension | `gemini-cli.acp` | `gemini-cli.headless` | `gemini.live` | `grok-build.acp` |
| --- | --- | --- | --- | --- |
| Route ID | `gemini-cli.acp` | `gemini-cli.headless` | `gemini.live` | `grok-build.acp` |
| Owning Adapter | `swallowtail-adapter-gemini` | `swallowtail-adapter-gemini` | `swallowtail-adapter-gemini` | `swallowtail-adapter-grok` |
| Operation Shape | `InteractiveSession` | `StructuredRun` | `InteractiveSession` | `InteractiveSession` (session) / `StructuredRun` (run) |
| Driver Role | `DriverRole::InteractiveSession` | `DriverRole::StructuredRun` | `DriverRole::RealtimeMediaSession` | `DriverRole::InteractiveSession` (session) / `DriverRole::StructuredRun` (run) |
| Protocol Facade ID | `acp-v1` | `gemini-headless-stream-json-v1` | `google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent.thinking-output-max-context-compression-2026-08-23` | `acp-v1` |
| Instance Ownership | `HostOwnedEphemeral` | `HostOwnedEphemeral` | `ExternalAttached` | `HostOwnedEphemeral` |
| Access Profile ID | Caller input (no fixed adapter-owned identity) | Caller input (no fixed adapter-owned identity) | `gemini.authorization-api-key.project` | `grok-build.subscription.delegated-oauth` |
| Instance Policy ID | `gemini-prepared-ambient` | `gemini-headless-ambient-plan` | `gemini-live-preview-authorization-key-manual-audio` | `grok-build-prepared-ambient` |
| Endpoint Audience | `gemini-developer-api` | `gemini-developer-api` | `generativelanguage.googleapis.com` | `grok-build.subscription` |
| Resource Access | `ResourceAccess::Read` (filesystem) | `ResourceAccess::Read` (filesystem) | Resource-free (`SessionAccessPolicy::resource_free()`) | `ResourceAccess::ReadWrite` (filesystem) |
| Mode / Harness Controls | Plan mode admitted | None | None | None |
| Media / Rollover Policy | None | None | Asymmetric PCM audio; 1 planned rollover | None |
| Model Selection | None (ambient/session) | `GeminiHeadlessModelSelection` | `GEMINI_LIVE_MODEL_ID` | `GrokModelSelection` |

Separation holds firmly without overlap:
- `gemini.live` separates from `gemini-cli.acp` on `DriverRole::RealtimeMediaSession` (vs `DriverRole::InteractiveSession`), `InstanceOwnership::ExternalAttached` (vs `HostOwnedEphemeral`), distinct protocol facade revision, fixed project API-key access profile (`gemini.authorization-api-key.project`) vs caller-input access profile, instance policy ID (`gemini-live-preview-authorization-key-manual-audio` vs `gemini-prepared-ambient`), endpoint audience (`generativelanguage.googleapis.com` vs `gemini-developer-api`), and realtime media requirements.
- `gemini-cli.headless` separates on `OperationShape::StructuredRun`, `DriverRole::StructuredRun`, and instance policy ID (`gemini-headless-ambient-plan`).
- `grok-build.acp` separates on distinct adapter crate (`swallowtail-adapter-grok`), separate provider family (`grok`), `ResourceAccess::ReadWrite`, and its own subscription access profile, audience, and instance policy ID.

No route borrows another's preflight plan, driver role, facade ID, access profile, or instance ownership.
Cross-route assembly between any of these fails closed under Contract 061.

## Construction-Time Withholding Rules

Every row that cannot be proved from the exact prepared facade's operation
record or plan requirements must be withheld at construction, not filtered
downstream:

1. **Catalogue-only withholding:**
   `feature.model-catalogue` is withheld at construction across all 4 routes.
   `gemini-cli.acp`, `gemini-cli.headless`, `gemini.live`, and `grok-build.acp`
   lack `DriverRole::ModelCatalog`. Catalogue observation for Gemini belongs to
   the separate hosted `gemini.models` route (`prepare_gemini_models`).
2. **Incompatible-operation shape withholding:**
   - `gemini-cli.acp`: `feature.structured-run` is withheld because the route
     prepares only `OperationShape::InteractiveSession`.
   - `gemini-cli.headless`: `feature.interactive-session` is withheld because
     the route prepares only `OperationShape::StructuredRun`.
   - `grok-build.acp`: On `GrokPreparedSession`, `feature.structured-run` and
     `control.model-selection` (structured-run) are withheld. On `GrokPreparedRun`,
     `feature.interactive-session`, `control.model-selection` (interactive-session),
     and `control.session-options` are withheld.
3. **Documentation-only withholding:**
   Rows documented in `provider-solution-feature-matrix.csv` without backing
   capability requirements or prepared evidence are withheld at construction:
   - `gemini-cli.acp`: `feature.usage-evidence` (absent from `session_capabilities()`),
     `feature.bounded-workspace-text-write`, `feature.owned-remote-resource-cleanup`.
   - `gemini-cli.headless`: `feature.bounded-workspace-text-write`,
     `feature.owned-remote-resource-cleanup`.
   - All routes: `feature.persistent-session-posture` is lifecycle evidence,
     not a consumer default or composer control.
4. **Unobserved post-open withholding at prepared time:**
   `feature.negotiated-model-options-observation` on `gemini-cli.acp` and
   `grok-build.acp` is `post-open-observation-only`. It is withheld at
   construction from prepared `AdapterContribution` snapshots because
   the session is not open and wire model options have not been observed.

## Deterministic Route Ledgers

### 1. `gemini-cli.acp` Ledger (14 rows)

Census lines: 251-262, 553, 661.

| # | Operation Shape | Semantic ID | Prepared Status | Lifecycle | Evidence Source / Withholding Reason |
| --- | --- | --- | --- | --- | --- |
| 1 | `model-catalogue` | `feature.model-catalogue` | Withheld | `selection-summary` | Catalogue-only; separate `gemini.models` route; lacks `DriverRole::ModelCatalog` |
| 2 | `structured-run` | `feature.structured-run` | Withheld | `selection-summary` | Incompatible operation shape; route prepares interactive session only |
| 3 | `interactive-session` | `feature.interactive-session` | Emitted | `selection-summary` | `Capability::InteractiveSession` in `session_capabilities()` |
| 4 | `route-observation` | `feature.streaming-events` | Emitted | `selection-summary` | `Capability::StreamingEvents` in `session_capabilities()` |
| 5 | `route-observation` | `feature.usage-evidence` | Withheld | `selection-summary` | Documentation-only; `UsageReporting` absent from `session_capabilities()` |
| 6 | `route-capability` | `feature.cancellation-or-interruption` | Emitted | `selection-summary` | `Capability::Interruption` with `ActiveTurn` in `session_capabilities()` |
| 7 | `route-capability` | `feature.working-resource` | Emitted | `selection-summary` | `Capability::WorkingResource` with `Filesystem` and `Read` in `session_capabilities()` |
| 8 | `route-capability` | `feature.bounded-workspace-text-write` | Withheld | `selection-summary` | Documentation-only; unobserved on prepared ACP session |
| 9 | `route-capability` | `feature.owned-remote-resource-cleanup` | Withheld | `selection-summary` | Documentation-only; unobserved on prepared ACP session |
| 10 | `session-lifecycle` | `feature.persistent-session-posture` | Withheld | `selection-summary` | Lifecycle evidence only; ephemeral host instance exposes no management control |
| 11 | `route-capability` | `feature.prepared-facade` | Emitted | `selection-summary` | `GeminiPreparedSession` prepared operation record |
| 12 | `route-observation` | `feature.activity-observation` | Emitted | `post-open-observation-only` | `ObservableActivityProfile` from `acp_activity`; descriptor-only |
| 13 | `interactive-session` | `feature.negotiated-model-options-observation` | Withheld (prepared) | `post-open-observation-only` | Unobserved at prepared time; retained on handle post-open but no open-with-projection seam exists on `main` |
| 14 | `interactive-session` | `control.harness-mode` | Emitted | `session-start-only` | `GeminiSessionProfileInput::new; SessionOptions::with_harness_mode`; admitted `["plan"]` under read access |

- Prepared facade emission: 7 emitted, 7 withheld (total 14).
- Active-session observation: 1 post-open row (`feature.negotiated-model-options-observation`) is publishable only with an additive `open_session_with_projection` seam.

### 2. `gemini-cli.headless` Ledger (13 rows)

Census lines: 263-274, 580.

| # | Operation Shape | Semantic ID | Prepared Status | Lifecycle | Evidence Source / Withholding Reason |
| --- | --- | --- | --- | --- | --- |
| 1 | `model-catalogue` | `feature.model-catalogue` | Withheld | `selection-summary` | Catalogue-only; separate `gemini.models` route; lacks `DriverRole::ModelCatalog` |
| 2 | `structured-run` | `feature.structured-run` | Emitted | `selection-summary` | `Capability::StructuredRun` in `run_capabilities()` |
| 3 | `interactive-session` | `feature.interactive-session` | Withheld | `selection-summary` | Incompatible operation shape; route executes structured runs only |
| 4 | `route-observation` | `feature.streaming-events` | Emitted | `selection-summary` | `Capability::StreamingEvents` in `run_capabilities()` |
| 5 | `route-observation` | `feature.usage-evidence` | Emitted | `selection-summary` | `Capability::UsageReporting` in `run_capabilities()` |
| 6 | `route-capability` | `feature.cancellation-or-interruption` | Emitted | `selection-summary` | `Capability::Interruption` with `StructuredRun` in `run_capabilities()` |
| 7 | `route-capability` | `feature.working-resource` | Emitted | `selection-summary` | `Capability::WorkingResource` with `Filesystem` and `Read` in `run_capabilities()` |
| 8 | `route-capability` | `feature.bounded-workspace-text-write` | Withheld | `selection-summary` | Documentation-only; headless is read-only |
| 9 | `route-capability` | `feature.owned-remote-resource-cleanup` | Withheld | `selection-summary` | Documentation-only; unobserved on prepared headless run |
| 10 | `session-lifecycle` | `feature.persistent-session-posture` | Withheld | `selection-summary` | Lifecycle evidence only; transcript retention is not a consumer management control |
| 11 | `route-capability` | `feature.prepared-facade` | Emitted | `selection-summary` | `GeminiHeadlessPreparedRun` prepared operation record |
| 12 | `route-observation` | `feature.activity-observation` | Emitted | `post-open-observation-only` | `ObservableActivityProfile` from `headless_activity`; descriptor-only |
| 13 | `structured-run` | `control.model-selection` | Emitted | `selection-summary` | `GeminiHeadlessModelSelection` bound to preflight plan `ModelRoute` and `ModelId` |

- Prepared facade emission: 8 emitted, 5 withheld (total 13).

### 3. `gemini.live` Ledger (16 rows)

Census lines: 275-285, 614, 641, 755, 756, 757.

| # | Operation Shape | Semantic ID | Prepared Status | Lifecycle | Evidence Source / Withholding Reason |
| --- | --- | --- | --- | --- | --- |
| 1 | `model-catalogue` | `feature.model-catalogue` | Withheld | `selection-summary` | Catalogue-only; separate `gemini.models` route; lacks `DriverRole::ModelCatalog` |
| 2 | `realtime-media-session` | `feature.realtime-media-session` | Emitted | `selection-summary` | Realtime media session operation shape in preflight plan |
| 3 | `route-observation` | `feature.streaming-events` | Emitted | `selection-summary` | `Capability::StreamingEvents` in `gemini_live_base_capabilities()` |
| 4 | `route-observation` | `feature.usage-evidence` | Emitted | `selection-summary` | `Capability::UsageReporting` in `gemini_live_base_capabilities()` |
| 5 | `route-capability` | `feature.output-token-limit` | Emitted | `selection-summary` | `Capability::OutputTokenLimit` when `maximum_output_tokens` configured |
| 6 | `route-capability` | `feature.reasoning-selection` | Emitted | `selection-summary` | `Capability::ReasoningSelection` when `reasoning_mode` configured |
| 7 | `route-capability` | `feature.cancellation-or-interruption` | Emitted | `selection-summary` | `Capability::Interruption` with `ActiveResponse` in `gemini_live_base_capabilities()` |
| 8 | `session-lifecycle` | `feature.planned-connection-rollover` | Emitted | `selection-summary` | `Capability::PlannedConnectionRollover` bound 1 in `gemini_live_base_capabilities()` |
| 9 | `session-lifecycle` | `feature.persistent-session-posture` | Withheld | `selection-summary` | Lifecycle evidence only; external attached connection is ephemeral |
| 10 | `route-capability` | `feature.prepared-facade` | Emitted | `selection-summary` | `GeminiPreparedLiveSession` prepared operation record |
| 11 | `route-observation` | `feature.activity-observation` | Emitted | `post-open-observation-only` | `ObservableActivityProfile` from live descriptor; descriptor-only |
| 12 | `realtime-media-session` | `control.reasoning-selection` | Emitted | `session-start-only` | `GeminiLiveSessionProfileInput::with_reasoning_mode`; validated thinking levels |
| 13 | `realtime-media-session` | `control.maximum-output-tokens` | Emitted | `session-start-only` | `GeminiLiveSessionProfileInput::with_maximum_output_tokens`; bound 1..=65536 |
| 14 | `realtime-media-session` | `control.realtime-media-config` | Emitted | `session-start-only` | `GeminiLiveSessionProfileInput::new`; validated against `gemini_live_media_config()` |
| 15 | `realtime-media-session` | `control.context-window-compression` | Emitted | `session-start-only` | `GeminiLiveSessionProfileInput::with_context_window_compression`; sliding-window policy |
| 16 | `realtime-media-session` | `control.planned-connection-rollover` | Emitted | `session-start-only` | `GeminiLiveSessionProfileInput::new`; validated against `gemini_live_rollover_policy()` |

- Prepared facade emission: 14 emitted, 2 withheld (total 16).

### 4. `grok-build.acp` Ledger (13 rows)

Census lines: 518-526, 554, 581, 582, 665.

| # | Operation Shape | Semantic ID | Session Status | Run Status | Lifecycle | Evidence Source / Withholding Reason |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | `model-catalogue` | `feature.model-catalogue` | Withheld | Withheld | `selection-summary` | Catalogue-only; separate route; lacks `DriverRole::ModelCatalog` |
| 2 | `structured-run` | `feature.structured-run` | Withheld | Emitted | `selection-summary` | `Capability::StructuredRun` in `run_capabilities()`; withheld on session |
| 3 | `interactive-session` | `feature.interactive-session` | Emitted | Withheld | `selection-summary` | `Capability::InteractiveSession` in `session_capabilities()`; withheld on run |
| 4 | `route-observation` | `feature.streaming-events` | Emitted | Emitted | `selection-summary` | `Capability::StreamingEvents` in both capability profiles |
| 5 | `route-capability` | `feature.cancellation-or-interruption` | Emitted | Emitted | `selection-summary` | `Capability::Interruption` in both capability profiles |
| 6 | `route-capability` | `feature.working-resource` | Emitted | Emitted | `selection-summary` | `Capability::WorkingResource` with `ReadWrite` in both profiles |
| 7 | `session-lifecycle` | `feature.persistent-session-posture` | Withheld | Withheld | `selection-summary` | Lifecycle evidence only; durable retention is not a consumer management control |
| 8 | `route-capability` | `feature.prepared-facade` | Emitted | Emitted | `selection-summary` | `GrokPreparedSession` / `GrokPreparedRun` prepared operation records |
| 9 | `route-observation` | `feature.activity-observation` | Emitted | Emitted | `post-open-observation-only` | `Capability::ObservableActivity` in both profiles; descriptor-only |
| 10 | `interactive-session` | `feature.negotiated-model-options-observation` | Withheld (prepared) | Withheld | `post-open-observation-only` | Unobserved at prepared time; retained on handle post-open but no open-with-projection seam exists on `main` |
| 11 | `structured-run` | `control.model-selection` | Withheld | Emitted | `selection-summary` | `GrokModelSelection` bound to preflight plan for structured run; withheld on session |
| 12 | `interactive-session` | `control.model-selection` | Emitted | Withheld | `selection-summary` | `GrokModelSelection` bound to preflight plan for interactive session; withheld on run |
| 13 | `interactive-session` | `control.session-options` | Emitted | Withheld | `session-start-only` | `GrokSessionProfileInput::new`; empty structured options required; withheld on run |

- `GrokPreparedSession` emission: 8 emitted, 5 withheld (total 13).
- `GrokPreparedRun` emission: 7 emitted, 6 withheld (total 13).
- Across both prepared facades, all 13 route census rows reconcile deterministically.
- Active-session observation: 1 post-open row (`feature.negotiated-model-options-observation`) is publishable only with an additive `open_session_with_projection` seam.

### Ledger Summary

- Total census rows: 56.
- Prepared facade emitted rows:
  - `gemini-cli.acp`: 7 emitted, 7 withheld (total 14)
  - `gemini-cli.headless`: 8 emitted, 5 withheld (total 13)
  - `gemini.live`: 14 emitted, 2 withheld (total 16)
  - `grok-build.acp`: session emitted = rows {3, 4, 5, 6, 8, 9, 12, 13} (8 emitted, 5 withheld); run emitted = rows {2, 4, 5, 6, 8, 9, 11} (7 emitted, 6 withheld); union across both prepared facades = {2, 3, 4, 5, 6, 8, 9, 11, 12, 13} = **10 unique emitted rows**. Rows withheld across both facades = {1, 7, 10} = 3. 10 + 3 = 13.
- Total unique rows emitted across candidate E at prepared construction: 7 + 8 + 14 + 10 = **39**.
- Total unique rows withheld across candidate E at prepared construction: 7 + 5 + 2 + 3 = **17**.
- Sum: 39 + 17 = 56.

## Promotion Rubric Evaluation

### Rubric Item 1: Census Row Set Reconciliation
**Verdict: PASS.**
The exact census row set reconciles to 56 rows (14 `gemini-cli.acp` + 13 `gemini-cli.headless` +
16 `gemini.live` + 13 `grok-build.acp`) without an exception or filter list. Candidate E contains
zero explicit `audit.no-public-route-specific-selectable-control` rows.

### Rubric Item 2: Facades, Source Identities, And Withholding Rules
**Verdict: PASS FOR PREPARED; BLOCKED ON PUBLICATION FOR ACTIVE OBSERVATION.**
- Every contributing prepared facade is named with exact code references on current `main`:
  `GeminiPreparedSession`, `GeminiHeadlessPreparedRun`, `GeminiPreparedLiveSession`,
  `GrokPreparedSession`, and `GrokPreparedRun`. All contribute under `AdapterContribution`.
- Documentation-only, catalogue-only, and incompatible-operation rows have explicit construction-time
  withholding rules.
- Active-observation facades are proved absent on current `main`: neither adapter exports
  `open_session_with_projection`, an `ActiveSessionObservation` source, or a projected-open outcome.
- For post-open observation, both `GeminiInteractiveSession` and `GrokInteractiveSession`
  already retain `NegotiatedSessionModelOptions` on current `main`. However, publishing
  `feature.negotiated-model-options-observation` requires an additive adapter-owned
  projected-open seam (`open_session_with_projection`), following the pattern proved for
  `cline.acp` in Card 032. Without that additive public seam, post-open model options must be
  withheld at construction as unobserved.

### Rubric Item 3: Public API, Bounds, Composer, And Contract Stability
**Verdict: PASS.**
- No new runtime or core public type is required. All portable feature and control identities
  (`ConsumerRouteFeatureId`, `ConsumerRouteControlId`), value kinds, lifecycles, and source kinds
  already exist in `swallowtail-runtime`.
- No new fixed maximum is required. Candidate E row counts fit comfortably within the fixed per-view limits (`consumer_route_projection.rs:51,53,55`):
  - Selection-summary view: maximum 12 rows across routes (`gemini-cli.headless` 12, `gemini-cli.acp` 11, `gemini.live` 10, `grok-build.acp` 10) vs limit of 32 (`MAX_CONSUMER_ROUTE_SELECTION_SUMMARY_ROWS`).
  - Session-start view: maximum 5 controls on `gemini.live` (1 on `gemini-cli.acp`, 0 on `gemini-cli.headless`, 1 on `grok-build.acp`) vs limit of 16 (`MAX_CONSUMER_ROUTE_SESSION_START_ROWS`).
  - Active-session view: maximum 2 rows (`feature.activity-observation` and `feature.negotiated-model-options-observation`) vs limit of 8 (`MAX_CONSUMER_ROUTE_ACTIVE_SESSION_ROWS`).
- Existing composer failure rules (Contract 061) and replacement semantics apply cleanly.
- No registry, runtime route enumeration, callback, or provider payload is introduced.
- Contracts 037, 047, 057, and 061 remain unchanged.

### Rubric Item 4: Deterministic Adapter-Local Ledgers
**Verdict: PASS.**
Four deterministic route ledgers account for all 56 rows with explicit emitted/withheld statuses,
lifecycle views, and evidence sources. All tests can run provider-free using preflight plans
and existing adapter test fixtures.

### Rubric Item 5: Focused Validation Maximum
**Verdict: PASS.**
The candidate spans exactly two adapter packages (`swallowtail-adapter-gemini` and
`swallowtail-adapter-grok`), comfortably within the four-package maximum:
`effigy validate:focused swallowtail-adapter-gemini swallowtail-adapter-grok` and
`effigy package:verify-affected swallowtail-adapter-gemini swallowtail-adapter-grok`.

### Rubric Item 6: Single Reviewable Tranche
**Verdict: PASS.**
Candidate E is strictly bounded to the 56 census rows for Gemini and Grok. It claims no
coverage for candidates B, C, F, I, J, K, L, or the 767-row Batch 9.5 census audit.

## Recommended Disposition

**Promotable as one exact package tranche with adapter-owned projected-open seams (qualified on publication of active session observation).**

### Evidence Basis

1. **All 5 prepared facades already exist and validate cleanly on current `main`.**
   The candidate requires zero new shared runtime/core public types, zero contract amendments,
   and zero new composer failure rules.
2. **Three-family applicability separation is realized and proven.**
   `gemini.live` and `gemini-cli.acp` share `OperationShape::InteractiveSession`, but separate
   cleanly on `DriverRole::RealtimeMediaSession` vs `DriverRole::InteractiveSession`, external attached ownership
   vs host-owned ephemeral instance, protocol facade revision, project API-key access profile vs caller-input
   access profile, instance policy ID, endpoint audience, and realtime media requirements.
   `gemini-cli.headless` separates on `OperationShape::StructuredRun`, `DriverRole::StructuredRun`,
   and instance policy ID. Grok ACP separates across interactive session and structured run under
   its own subscription access profile and instance policy ID.
3. **Model options retention is already implemented on current `main`.**
   Unlike Claude Agent and Cline before their respective gates (which discarded provider
   confirmations), both `GeminiInteractiveSession` and `GrokInteractiveSession` already parse,
   validate, and store `NegotiatedSessionModelOptions` on `self.model_options` during session open.
   No confirmation is discarded.
4. **Publication path follows the proved Cline precedent (Card 032).**
   To publish `feature.negotiated-model-options-observation`, the implementation card can add
   adapter-local additive `open_session_with_projection` seams to `GeminiPreparedSession` and
   `GrokPreparedSession` while preserving the existing `open_session` methods. Alternatively,
   if the operator prefers a prepared-only implementation tranche (like Cards 023 and 024),
   post-open model options can simply remain withheld at construction as unobserved.

### Operator / Chatterbox Decision

Chatterbox reconciles this note and decides whether:
- **Path A (Full 56-row proof with active model-option observation):**
  Promote candidate E as one implementation card that implements `consumer_route_projection_contribution`
  on all 5 prepared facades and adds the additive adapter-owned `open_session_with_projection`
  seam on `GeminiPreparedSession` and `GrokPreparedSession` (following Card 032).
- **Path B (Prepared-only proof):**
  Promote candidate E as one implementation card that implements prepared projection only,
  withholding `feature.negotiated-model-options-observation` at construction as unobserved.
- **Path C (Pre-implementation gate):**
  If operator policy requires a formal public-baseline gate before adding adapter-local
  `open_session_with_projection` to Gemini and Grok, stop for that gate. (Unlike Kimi, no
  runtime type gaps or open-enum reasoning issues exist here).
