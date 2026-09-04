# Contract 061 Candidate C Breadth Audit

Status: active planning evidence; Candidate C audit complete; recommended disposition: promotable as one exact package tranche
Owner: Tom
Date: 2026-09-04
Source: Card 064, Contract 061, Batch 9.4 package expansion, and `main` at `bab21839321a1b29da0b14209db32c8323a9d1c2`

## Purpose

Audit Batch 9.4 candidate C (`swallowtail-adapter-antigravity`,
`swallowtail-adapter-bedrock`, `swallowtail-adapter-cursor`) against current
`main` under the promotion rubric and return one honest disposition. This is
planning evidence: it authorizes no Rust changes, no provider contact or
credentials, and no direct coverage claims.

## Candidate Summary

Candidate C owns three complete adapter-package remainders across seven route
shapes and exactly 94 census rows:

| Route ID | Owning package | Census rows | Route shape | Audit role |
| --- | --- | ---: | --- | --- |
| `antigravity.catalogue` | `swallowtail-adapter-antigravity` | 14 | Model catalogue | Explicit no-control audit |
| `antigravity.headless` | `swallowtail-adapter-antigravity` | 18 | Stream-JSON structured run | 5 prepared controls |
| `bedrock.catalogue` | `swallowtail-adapter-bedrock` | 9 | Control-plane catalogue | Explicit no-control audit |
| `bedrock.runtime` | `swallowtail-adapter-bedrock` | 10 | Direct runtime inference | 2 prepared controls |
| `cursor-agent.acp` | `swallowtail-adapter-cursor` | 13 | Interactive ACP session | Explicit no-control audit |
| `cursor-agent.catalogue` | `swallowtail-adapter-cursor` | 13 | Model catalogue | Explicit no-control audit |
| `cursor-agent.headless` | `swallowtail-adapter-cursor` | 17 | Stream-JSON structured run | 5 prepared controls |
| **Total** | **3 packages** | **94** | **7 routes** | **4 no-control audits; 12 controls** |

The four explicit no-control route audits (`antigravity.catalogue`,
`bedrock.catalogue`, `cursor-agent.acp`, `cursor-agent.catalogue`) carry
`audit.no-public-route-specific-selectable-control` as negative coverage.
No route in Candidate C carries mid-turn, per-turn, or active-session
observation rows.

## Census Reconciliation (94 Rows)

The 94 rows reconcile exactly from the reviewed Contract 061 census CSV with
zero filters, exemptions, or duplicated rows.

### Row Breakdown By Route And Kind

| Route | Features | Controls | Route Audits | Total |
| --- | ---: | ---: | ---: | ---: |
| `antigravity.catalogue` | 13 | 0 | 1 | 14 |
| `antigravity.headless` | 13 | 5 | 0 | 18 |
| `bedrock.catalogue` | 8 | 0 | 1 | 9 |
| `bedrock.runtime` | 8 | 2 | 0 | 10 |
| `cursor-agent.acp` | 12 | 0 | 1 | 13 |
| `cursor-agent.catalogue` | 12 | 0 | 1 | 13 |
| `cursor-agent.headless` | 12 | 5 | 0 | 17 |
| **Total** | **78** | **12** | **4** | **94** |

### Complete Census Ledger

| Route ID | Operation Shape | Semantic ID | Kind | Lifecycle | State Support |
| --- | --- | --- | --- | --- | --- |
| `antigravity.catalogue` | `model-catalogue` | `feature.model-catalogue` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.catalogue` | `structured-run` | `feature.structured-run` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.catalogue` | `interactive-session` | `feature.interactive-session` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.catalogue` | `route-observation` | `feature.streaming-events` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.catalogue` | `route-observation` | `feature.usage-evidence` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.catalogue` | `route-capability` | `feature.reasoning-selection` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.catalogue` | `route-capability` | `feature.structured-output` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.catalogue` | `route-capability` | `feature.cancellation-or-interruption` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.catalogue` | `route-capability` | `feature.working-resource` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.catalogue` | `route-capability` | `feature.bounded-workspace-text-write` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.catalogue` | `session-lifecycle` | `feature.persistent-session-posture` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.catalogue` | `route-capability` | `feature.prepared-facade` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.catalogue` | `route-observation` | `feature.activity-observation` | feature | `post-open-observation-only` | `descriptor-only` |
| `antigravity.catalogue` | `route-selection` | `audit.no-public-route-specific-selectable-control` | route-audit | `selection-summary` | `descriptor-only` |
| `antigravity.headless` | `model-catalogue` | `feature.model-catalogue` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.headless` | `structured-run` | `feature.structured-run` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.headless` | `interactive-session` | `feature.interactive-session` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.headless` | `route-observation` | `feature.streaming-events` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.headless` | `route-observation` | `feature.usage-evidence` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.headless` | `route-capability` | `feature.reasoning-selection` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.headless` | `route-capability` | `feature.structured-output` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.headless` | `route-capability` | `feature.cancellation-or-interruption` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.headless` | `route-capability` | `feature.working-resource` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.headless` | `route-capability` | `feature.bounded-workspace-text-write` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.headless` | `session-lifecycle` | `feature.persistent-session-posture` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.headless` | `route-capability` | `feature.prepared-facade` | feature | `selection-summary` | `descriptor-only` |
| `antigravity.headless` | `route-observation` | `feature.activity-observation` | feature | `post-open-observation-only` | `descriptor-only` |
| `antigravity.headless` | `structured-run` | `control.model-selection` | control | `selection-summary` | `requested;prepared;provider-effective-unobserved` |
| `antigravity.headless` | `structured-run` | `control.reasoning-selection` | control | `session-start-only` | `requested;prepared;effective-unobserved` |
| `antigravity.headless` | `structured-run` | `control.structured-output` | control | `session-start-only` | `requested;prepared;effective-unobserved` |
| `antigravity.headless` | `structured-run` | `control.resource-access` | control | `session-start-only` | `requested;prepared;effective-unobserved` |
| `antigravity.headless` | `structured-run` | `control.isolation` | control | `session-start-only` | `requested;prepared;effective-unobserved` |
| `bedrock.catalogue` | `model-catalogue` | `feature.model-catalogue` | feature | `selection-summary` | `descriptor-only` |
| `bedrock.catalogue` | `structured-run` | `feature.structured-run` | feature | `selection-summary` | `descriptor-only` |
| `bedrock.catalogue` | `route-observation` | `feature.streaming-events` | feature | `selection-summary` | `descriptor-only` |
| `bedrock.catalogue` | `route-observation` | `feature.usage-evidence` | feature | `selection-summary` | `descriptor-only` |
| `bedrock.catalogue` | `route-capability` | `feature.output-token-limit` | feature | `selection-summary` | `descriptor-only` |
| `bedrock.catalogue` | `route-capability` | `feature.cancellation-or-interruption` | feature | `selection-summary` | `descriptor-only` |
| `bedrock.catalogue` | `route-capability` | `feature.prepared-facade` | feature | `selection-summary` | `descriptor-only` |
| `bedrock.catalogue` | `route-observation` | `feature.activity-observation` | feature | `post-open-observation-only` | `descriptor-only` |
| `bedrock.catalogue` | `route-selection` | `audit.no-public-route-specific-selectable-control` | route-audit | `selection-summary` | `descriptor-only` |
| `bedrock.runtime` | `model-catalogue` | `feature.model-catalogue` | feature | `selection-summary` | `descriptor-only` |
| `bedrock.runtime` | `structured-run` | `feature.structured-run` | feature | `selection-summary` | `descriptor-only` |
| `bedrock.runtime` | `route-observation` | `feature.streaming-events` | feature | `selection-summary` | `descriptor-only` |
| `bedrock.runtime` | `route-observation` | `feature.usage-evidence` | feature | `selection-summary` | `descriptor-only` |
| `bedrock.runtime` | `route-capability` | `feature.output-token-limit` | feature | `selection-summary` | `descriptor-only` |
| `bedrock.runtime` | `route-capability` | `feature.cancellation-or-interruption` | feature | `selection-summary` | `descriptor-only` |
| `bedrock.runtime` | `route-capability` | `feature.prepared-facade` | feature | `selection-summary` | `descriptor-only` |
| `bedrock.runtime` | `route-observation` | `feature.activity-observation` | feature | `post-open-observation-only` | `descriptor-only` |
| `bedrock.runtime` | `structured-run` | `control.model-selection` | control | `selection-summary` | `requested;prepared;provider-effective-unobserved` |
| `bedrock.runtime` | `structured-run` | `control.maximum-output-tokens` | control | `session-start-only` | `requested;prepared;effective-unobserved` |
| `cursor-agent.acp` | `model-catalogue` | `feature.model-catalogue` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.acp` | `structured-run` | `feature.structured-run` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.acp` | `interactive-session` | `feature.interactive-session` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.acp` | `route-observation` | `feature.streaming-events` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.acp` | `route-observation` | `feature.usage-evidence` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.acp` | `route-capability` | `feature.reasoning-selection` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.acp` | `route-capability` | `feature.cancellation-or-interruption` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.acp` | `route-capability` | `feature.working-resource` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.acp` | `route-capability` | `feature.bounded-workspace-text-write` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.acp` | `session-lifecycle` | `feature.persistent-session-posture` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.acp` | `route-capability` | `feature.prepared-facade` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.acp` | `route-observation` | `feature.activity-observation` | feature | `post-open-observation-only` | `descriptor-only` |
| `cursor-agent.acp` | `route-selection` | `audit.no-public-route-specific-selectable-control` | route-audit | `selection-summary` | `descriptor-only` |
| `cursor-agent.catalogue` | `model-catalogue` | `feature.model-catalogue` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.catalogue` | `structured-run` | `feature.structured-run` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.catalogue` | `interactive-session` | `feature.interactive-session` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.catalogue` | `route-observation` | `feature.streaming-events` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.catalogue` | `route-observation` | `feature.usage-evidence` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.catalogue` | `route-capability` | `feature.reasoning-selection` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.catalogue` | `route-capability` | `feature.cancellation-or-interruption` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.catalogue` | `route-capability` | `feature.working-resource` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.catalogue` | `route-capability` | `feature.bounded-workspace-text-write` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.catalogue` | `session-lifecycle` | `feature.persistent-session-posture` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.catalogue` | `route-capability` | `feature.prepared-facade` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.catalogue` | `route-observation` | `feature.activity-observation` | feature | `post-open-observation-only` | `descriptor-only` |
| `cursor-agent.catalogue` | `route-selection` | `audit.no-public-route-specific-selectable-control` | route-audit | `selection-summary` | `descriptor-only` |
| `cursor-agent.headless` | `model-catalogue` | `feature.model-catalogue` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.headless` | `structured-run` | `feature.structured-run` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.headless` | `interactive-session` | `feature.interactive-session` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.headless` | `route-observation` | `feature.streaming-events` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.headless` | `route-observation` | `feature.usage-evidence` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.headless` | `route-capability` | `feature.reasoning-selection` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.headless` | `route-capability` | `feature.cancellation-or-interruption` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.headless` | `route-capability` | `feature.working-resource` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.headless` | `route-capability` | `feature.bounded-workspace-text-write` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.headless` | `session-lifecycle` | `feature.persistent-session-posture` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.headless` | `route-capability` | `feature.prepared-facade` | feature | `selection-summary` | `descriptor-only` |
| `cursor-agent.headless` | `route-observation` | `feature.activity-observation` | feature | `post-open-observation-only` | `descriptor-only` |
| `cursor-agent.headless` | `structured-run` | `control.model-selection` | control | `selection-summary` | `requested;prepared;provider-effective-unobserved` |
| `cursor-agent.headless` | `structured-run` | `control.fast` | control | `session-start-only` | `requested;prepared;effective-unobserved` |
| `cursor-agent.headless` | `structured-run` | `control.context-window` | control | `session-start-only` | `requested;prepared;effective-unobserved` |
| `cursor-agent.headless` | `structured-run` | `control.reasoning-effort` | control | `session-start-only` | `requested;prepared;effective-unobserved` |
| `cursor-agent.headless` | `structured-run` | `control.read-mode` | control | `session-start-only` | `requested;prepared;effective-unobserved` |

## Facade And Source Identity Map On Current `main`

Every prepared facade exists on current `main`. Active observation facades do
not exist and are not required by Candidate C.

### Prepared Facades

| Route | Prepared Facade | Source Kind | Current `main` Code Reference |
| --- | --- | --- | --- |
| `antigravity.catalogue` | `AntigravityPreparedCatalogue` | `AdapterContribution` | `crates/swallowtail-adapter-antigravity/src/prepared/catalogue.rs:37-43` |
| `antigravity.headless` | `AntigravityPreparedHeadlessRun` | `AdapterContribution` | `crates/swallowtail-adapter-antigravity/src/prepared/run.rs:98-104` |
| `bedrock.catalogue` | `BedrockPreparedCatalogue` | `AdapterContribution` | `crates/swallowtail-adapter-bedrock/src/prepared/catalogue.rs:186-192` |
| `bedrock.runtime` | `BedrockPreparedInferenceAttempt` | `AdapterContribution` | `crates/swallowtail-adapter-bedrock/src/prepared/runtime.rs:234-240` |
| `cursor-agent.acp` | `CursorPreparedAcpSession` | `AdapterContribution` | `crates/swallowtail-adapter-cursor/src/prepared/acp.rs:33-38` |
| `cursor-agent.catalogue` | `CursorPreparedCatalogue` | `AdapterContribution` | `crates/swallowtail-adapter-cursor/src/prepared/catalogue.rs:38-44` |
| `cursor-agent.headless` | `CursorPreparedHeadlessRun` | `AdapterContribution` | `crates/swallowtail-adapter-cursor/src/prepared/headless.rs:144-151` |

All seven prepared facades use caller-supplied
`ConsumerRouteProjectionSourceKind::AdapterContribution`.

### Active-Observation Facades: Exact Absence Proof

Active-observation facades on current `main` are absent on all seven routes:

- `antigravity.catalogue`: executes unary installed CLI discovery via
  `AntigravityCatalogueDriver::list_models` (`prepared/catalogue.rs:110-119`).
  It returns a list of models, opens no session, and carries no post-open
  observation method or state.
- `antigravity.headless`: executes one-shot stream-JSON structured execution via
  `StructuredRunDriver::start_run` (`prepared/run.rs:194-203`). It returns a
  `RunHandle`, not an interactive session handle. No active-observation facade
  exists.
- `bedrock.catalogue`: executes control-plane `ListFoundationModels` via
  `BedrockCatalogueDriver::list_models` (`prepared/catalogue.rs:220-229`). It
  opens no session and carries no active observation facade.
- `bedrock.runtime`: executes direct SDK inference via
  `BedrockDirectDriver::start_run` (`prepared/runtime.rs:268-276`). It returns
  a `RunHandle` without an active-session observation outcome.
- `cursor-agent.acp`: opens an interactive session via
  `CursorAcpDriver::open_session` returning a standard
  `InteractiveSessionHandle` (`prepared/acp.rs:135-144`). It retains no
  provider-effective or rejected confirmation state; no adapter-owned
  projected-open outcome or active-observation facade exists.
- `cursor-agent.catalogue`: executes installed CLI discovery via
  `CursorCatalogueDriver::list_models` (`prepared/catalogue.rs:114-122`). No
  active observation facade exists.
- `cursor-agent.headless`: executes one-shot stream-JSON structured execution via
  `StructuredRunDriver::start_run` (`prepared/headless.rs:234-243`). No active
  observation facade exists.

Absence is verified and compliant: no census row in Candidate C requires an
active observation facade. All 12 controls are prepared-only controls
(`requested;prepared;provider-effective-unobserved` or
`requested;prepared;effective-unobserved`), and all 78 features and 4 route
audits are `descriptor-only`.

## Construction-Time Withholding Rules

To preserve pure fail-closed composition without post-hoc filtering:

1. **Incompatible Operation Shape Withholding**:
   - Catalogue operations (`OperationShape::StructuredRun` with
     `DriverRole::ModelCatalog`) withhold `feature.structured-run` (consumer
     execution), `feature.interactive-session`, and all execution controls at
     construction.
   - Headless structured runs withhold `feature.model-catalogue` and
     `feature.interactive-session` at construction.
   - ACP interactive sessions withhold `feature.model-catalogue` and
     `feature.structured-run` at construction.

2. **Documentation-Only Matrix Feature Withholding**:
   - Rows documented in `docs/guides/provider-solution-feature-matrix.csv` but
     not preflight-bound on the prepared operation (such as
     `feature.streaming-events` on Bedrock catalogue, `feature.usage-evidence`
     on Cursor ACP, `feature.cancellation-or-interruption` on Bedrock
     catalogue and Bedrock runtime, and `feature.bounded-workspace-text-write`
     on Cursor ACP/headless and Antigravity catalogue/headless) are withheld
     at construction.

3. **Negative-Coverage Route-Audit Withholding**:
   - `audit.no-public-route-specific-selectable-control` on the four no-control
     routes (`antigravity.catalogue`, `bedrock.catalogue`, `cursor-agent.acp`,
     `cursor-agent.catalogue`) records the explicit absence of route-specific
     controls. It must be withheld at construction from public control
     descriptors. Emitting a selectable control from any of these routes would
     falsify the audit.

4. **Activity Observation Withholding On Catalogue Routes**:
   - `feature.activity-observation` is descriptor-only with lifecycle
     `post-open-observation-only`. It is emitted from operations carrying an
     observable activity profile (`antigravity.headless`, `bedrock.runtime`,
     `cursor-agent.acp`, `cursor-agent.headless`).
   - On catalogue operations (`antigravity.catalogue`, `bedrock.catalogue`,
     `cursor-agent.catalogue`), the preflight plan carries no
     `ObservableActivityProfile`; `feature.activity-observation` is withheld
     at construction.

5. **Conditional Feature Withholding**:
   - `feature.reasoning-selection` on `antigravity.headless` and
     `cursor-agent.headless` is emitted only when reasoning effort is
     explicitly configured in the preparation input; otherwise withheld.
   - `feature.structured-output` on `antigravity.headless` is emitted only
     when a schema is supplied; otherwise withheld.

## Section 6a: Catalogue-Route Provider-Operation Observation Audit (Kimi Reopen Trigger)

Card 064 explicitly asks:
> Report explicitly whether any row on `antigravity.catalogue`,
> `bedrock.catalogue`, or `cursor-agent.catalogue` needs a provider-operation
> observation source kind, lifecycle band, or view that current
> `swallowtail-runtime` defines only as post-open session semantics. This
> finding is the reopen trigger for the deferred Kimi decision recorded in the
> Kimi active-observation gate note; state it in its own section.

### Finding: NO Catalogue Route Needs Provider-Operation Observation

A complete audit of all rows on `antigravity.catalogue` (14 rows),
`bedrock.catalogue` (9 rows), and `cursor-agent.catalogue` (13 rows) confirms:

1. **No `control.provider-session-catalogue` Row**: None of the three
   catalogue routes has a `control.provider-session-catalogue` row. In the
   global 767-row census, that row appears only on `opencode.http`,
   `deepseek-harness.local-server`, and `kimi-code.acp`.
2. **Model Catalogue Is Discovery, Not Session State**: The catalogue feature
   on these routes is `feature.model-catalogue`, which carries lifecycle
   `selection-summary` and state `descriptor-only`. It represents foundation
   model enumeration via `ModelCatalogDriver`, not post-operation observation
   of provider session state.
3. **No Active Observation Posture**: Every feature row on these three
   catalogue routes is `descriptor-only`. None carries `state_support: observed`
   or post-open session state.
4. **Activity Observation Is Descriptor-Only**: `feature.activity-observation`
   on these catalogue routes has lifecycle `post-open-observation-only` but state
   `descriptor-only`; moreover, because none of the three catalogue preflight
   plans attaches an activity profile, the row is withheld at construction.

### Decision Consequence

The reopen condition recorded in
`docs/logs/2026-09-04-contract-061-observation-deferral-and-breadth-audits.md`
is **not** triggered:
- Candidate C's catalogue routes do not need a provider-operation observation
  source kind, lifecycle band, or view.
- The deferred provider-operation observation decision remains deferred.
- Candidate F remains unpromoted.
- Card 034 remains planned and not ready.

## Promotion Rubric Evaluation

### Rubric Item 1: Exact Census Reconciliation And No-Control Negative Coverage

- Exact census rows: 94 rows across 7 routes (14 + 18 + 9 + 10 + 13 + 13 + 17).
- Four explicit no-control route audits (`antigravity.catalogue`,
  `bedrock.catalogue`, `cursor-agent.acp`, `cursor-agent.catalogue`) carry
  `audit.no-public-route-specific-selectable-control` as negative coverage.
- Reconciles without an exception or filter list.
- **Verdict: PASS**.

### Rubric Item 2: Facade Map, Source Identity, And Construction Withholding

- Every contributing prepared facade is named on current `main`:
  `AntigravityPreparedCatalogue`, `AntigravityPreparedHeadlessRun`,
  `BedrockPreparedCatalogue`, `BedrockPreparedInferenceAttempt`,
  `CursorPreparedAcpSession`, `CursorPreparedCatalogue`, and
  `CursorPreparedHeadlessRun`.
- Source identity kind: `ConsumerRouteProjectionSourceKind::AdapterContribution`
  on all seven routes.
- Active-observation facades: proven absent on current `main`; none of the 94
  rows claims or requires active post-open provider observation.
- Explicit construction-time withholding rules are defined for
  incompatible-operation, documentation-only, catalogue-only, unobserved, and
  negative-coverage rows.
- **Verdict: PASS**.

### Rubric Item 3: Public Baseline Stability

- No new `swallowtail-runtime` or `swallowtail-core` public types are needed.
- Standard controls (`control.model-selection`, `control.reasoning-selection`,
  `control.maximum-output-tokens`) map directly to `ConsumerRouteControlId`
  enum variants.
- Route-specific controls (`control.structured-output`,
  `control.resource-access`, `control.isolation`, `control.fast`,
  `control.context-window`, `control.reasoning-effort`, `control.read-mode`) map
  to `ConsumerRouteControlId::Namespaced(ConsumerRouteNamespacedExtension)`.
- Fixed library maxima are respected:
  - Selection-summary rows per route: maximum 13 (limit 32).
  - Session-start controls per route: maximum 4 (limit 16).
  - Active-session rows per route: 0 (limit 8).
  - Namespaced extensions per route: maximum 4 (limit 16).
- Pure fail-closed composer rules, registry absence, and Contract 061
  boundaries remain intact. No contract amendment is needed.
- **Verdict: PASS**.

### Rubric Item 4: Deterministic Adapter-Local Ledgers

- Deterministic ledgers for all seven routes prove exact emitted and withheld
  sets without contacting a provider.
- Negative coverage is explicitly asserted for the four no-control routes.
- Lifecycle and authority distinctions are preserved: prepared success does
  not create active observation.
- **Verdict: PASS**.

### Rubric Item 5: Package Boundary And Focused Validation

- Candidate C encompasses exactly three adapter packages:
  - `swallowtail-adapter-antigravity`
  - `swallowtail-adapter-bedrock`
  - `swallowtail-adapter-cursor`
- Three packages is strictly within the four-package maximum for focused
  validation (`effigy validate:focused ...`, `effigy package:verify-affected ...`).
- **Verdict: PASS**.

### Rubric Item 6: Reviewable Tranche Scope

- Candidate C forms one cohesive, self-contained breadth tranche.
- The audit claims nothing about remaining candidates (B, E, F, I, J, K, L)
  or the 767-row Batch 9.5 audit.
- Stops after this triage note and Card 064 Result update.
- **Verdict: PASS**.

## Recommended Disposition

**Promotable as one exact package tranche**.

Candidate C passes all six promotion rubric criteria against current `main`
at `bab21839321a1b29da0b14209db32c8323a9d1c2`:
- 94 rows across 3 adapter packages and 7 route shapes.
- 4 explicit no-control negative-coverage audits.
- 0 active-observation dependencies; 0 public baseline gaps.
- Does not trigger reopening the deferred Kimi decision.

Chatterbox may promote Candidate C to an implementation card upon operator
review.
