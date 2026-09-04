# Contract 061 Candidate J Breadth Audit

Status: planning evidence; candidate J promotable as one exact two-package
tranche (llama.cpp and Ollama, 35 rows); no blocker and no public-baseline
decision required
Owner: Tom
Date: 2026-09-04
Source: card 067, Batch 9.4 checkpoint, Contract 061, reviewed census, and
`main` at `bab21839321a1b29da0b14209db32c8323a9d1c2`

## Purpose

Audit Batch 9.4 candidate J (llama.cpp, Ollama) against current `main` under
the Batch 9.4 promotion rubric and return one honest disposition. This is
planning-only evidence. It authorizes no Rust, no implementation card, no
provider contact, and no coverage claim; Chatterbox reconciles the note and
promotes at most one implementation card from a passing candidate.

The candidate owns the complete census remainder of two adapter packages:

| Candidate | Adapter packages | Exact route rows | Total |
| --- | --- | --- | ---: |
| J | `swallowtail-adapter-llama-cpp`; `swallowtail-adapter-ollama` | `llama-cpp.attached` 10; `llama-cpp.owned` 6; `ollama.attached` 19 | 35 |

## Facade Map And Source Identities

None of the three routes has an active-observation facade, an acknowledgement
surface, or a post-open seam on current `main`; none of the census rows needs
one (no acknowledgement or provider-session-management row exists in the 35).
The complete projection surface is the prepared-operation family. Neither
adapter crate contains a `consumer_route_projection` module today; every
facade below already retains the exact prepared evidence a contribution needs,
and the established
`consumer_route_projection_contribution(source_id)` shape with a
caller-supplied `ConsumerRouteProjectionSourceId` and
`PreparedOperationRecord` / `AdapterPreparedInput` / `CapabilityProfile`
source classes applies unchanged (precedents: `codex.exec` through
`swallowtail-adapter-codex/src/consumer_route_projection/exec.rs`,
`zcode.app-server` through
`swallowtail-adapter-zcode/src/consumer_route_projection.rs`, and the Cline
`ProjectionBuilder`).

### `llama-cpp.attached` — route id `llama-cpp.attached`

Prepared facades on the attached driver
(`llama_cpp_attached_descriptor`, roles `ModelCatalog` + `StructuredRun`,
shape `StructuredRun`, `src/driver/descriptor.rs`):

| Facade | Prepared operation | Evidence retained | Anchor |
| --- | --- | --- | --- |
| `LlamaCppPreparedCatalogue` | read-only `ModelCatalogRequest` (`list_models` → `Vec<ModelCatalogEntry>`) | `LlamaCppAttachedPreparedEvidence` = `PreparedOperationEvidence` (`PreparedOperationEvidence::from_plan`) | `src/prepared/attached.rs`, `src/prepared/attached/evidence.rs` |
| `LlamaCppPreparedInferenceAttempt` | bounded `StructuredRunRequest` (`start_run` → `RunHandle`) | `LlamaCppAttachedPreparedEvidence` with `ObservableActivityProfile` (`from_plan_with_activity_profile`) | `src/prepared/attached.rs`, `src/prepared/attached/evidence.rs` |

Exact retained request values: `LlamaCppModelSelection`
(`ModelRouteId` + `ModelRouteRevision` + `ModelId`,
`src/prepared/attached/input.rs`); `LlamaCppInferenceProfileInput::new`
requires a positive `NonZeroU64` `maximum_output_tokens`
(`src/prepared/attached/input.rs`). The plan requirements come from
`attached_capabilities(role)` (`src/selection.rs`): `ModelCatalog` for the
catalogue role; `StructuredRun`, `StreamingEvents`, `UsageReporting`,
`OutputTokenLimit` for the structured-run role, plus `ObservableActivity`
added by the activity profile (`src/activity/profile.rs`) on the inference
path. `attached_all_capabilities` (instance profile) contains exactly those
plus `ModelCatalog`. The run handle exposes driver-level cancellation
(`CancellationScope::StructuredRun`, `src/driver/handle.rs`) but **no
attached plan on current `main` requires `Capability::Interruption`**.

### `llama-cpp.owned` — route id `llama-cpp.owned`

Prepared facade on the owned driver
(`llama_cpp_owned_descriptor`, roles `ServingInstanceLifecycle` +
`ModelCatalog` + `StructuredRun`, `src/driver/owned/roles.rs`):

| Facade | Prepared operation | Evidence retained | Anchor |
| --- | --- | --- | --- |
| `LlamaCppPreparedServingStart` | host-owned `StartServingRequest` (`start` → `OwnedServingHandle`) | `LlamaCppOwnedPreparedEvidence` = `PreparedOperationEvidence` + `ModelArtifactBinding` + `Option<LlamaCppContextSize>` + `Option<LlamaCppReasoningSelection>` | `src/prepared/owned.rs`, `src/prepared/owned/evidence.rs` |

The owned serving plan binds `InstanceOwnership::HostOwnedEphemeral` plus a
model route and carries `StreamingEvents` and `Interruption`
(`CancellationScope::OwnedServingInstance`) requirements
(`owned_capabilities`, `src/selection.rs`). Exact retained values:
`LlamaCppOwnedServingSelection::new` requires the `ModelArtifactBinding`;
`with_context_size` bounds `--ctx-size` to the adapter-local
`LlamaCppContextSize` (`MINIMUM = 1`, `MAXIMUM = i32::MAX`,
`src/context_size.rs`); `with_reasoning` admits exactly
`LlamaCppReasoningSelection::Disabled`, dispatching the canonical literal
`off` only — `on` and `auto` are withheld by exact tagged-source evidence
(`src/reasoning.rs`). The owned attach seam is rejected
(`swallowtail.llama_cpp.owned_attach_rejected`,
`src/driver/owned/roles.rs`), and no prepared owned facade carries an
`ObservableActivityProfile` (the crate's only activity profile binds the
attached runtime, `src/activity/profile.rs`).

### `ollama.attached` — route id `ollama.attached`

Prepared facades on the native attached driver
(`ollama_native_descriptor`, roles `ModelCatalog` + `StructuredRun` +
`InteractiveSession`, shapes `StructuredRun` + `InteractiveSession`,
`src/selection.rs`):

| Facade | Prepared operation | Evidence retained | Anchor |
| --- | --- | --- | --- |
| `OllamaPreparedInventory` | read-only `ModelCatalogRequest` (`observe_inventory` → `OllamaInventorySnapshot` of `ModelCatalogEntry`) | `OllamaPreparedEvidence` (`from_prepared`, no activity, no context window) | `src/prepared_profile/inventory.rs` |
| `OllamaPreparedInferenceAttempt` | bounded `StructuredRunRequest` (`start_run` → `RunHandle`) | `OllamaPreparedEvidence` with `ObservableActivityProfile` and `Option<OllamaContextWindow>` | `src/prepared_profile/inference.rs`, `src/prepared_profile/plan.rs` |
| `OllamaPreparedSession` | resource-free `OpenSessionRequest` (`open_session` → `InteractiveSessionHandle`) | same evidence shape as the attempt | `src/prepared_profile/session.rs`, `src/prepared_profile/plan.rs` |

Exact retained request and bound values:
`OllamaModelSelection` (`ModelRouteId` + `ModelRouteRevision` + `ModelId`,
`src/prepared_profile/input.rs`); `OllamaInferenceAttemptInput::new`
requires a `NonZeroU64` `maximum_output_tokens` (route ceiling
`u32::MAX`, `prepare_inference_attempt`); `with_reasoning_mode` is admitted
only when the observed runtime capability table reports
`OllamaModelCapability::Thinking` for the bound model and the mode is one of
`off`/`low`/`medium`/`high` (`validate_reasoning`); `with_structured_output`
is admitted only through `validate_structured_output`;
`with_context_window` bounds `options.num_ctx` to the adapter-local
`OllamaContextWindow` (`MINIMUM = 4`, `MAXIMUM = i32::MAX`,
`src/context_window.rs`). `OllamaSessionProfileInput` carries **no model,
reasoning, or structured-output field** — only a context window — so the
interactive-session shape cannot construct a reasoning selection.
`session_capabilities` requires `InteractiveSession` (24-turn / 1 MiB private
history), `StreamingEvents`, `UsageReporting`, `OutputTokenLimit`, and
`Interruption` (`CancellationScope::ActiveTurn`); the structured-run
capability set (`inference_capabilities`) requires `StructuredRun`,
`StreamingEvents`, `UsageReporting`, `OutputTokenLimit` and adds
`ReasoningSelection` / `StructuredOutput` only when the caller requests them.
The run handle exposes driver-level cancellation
(`CancellationScope::StructuredRun`, `src/driver/handle.rs`), but no
structured-run plan on current `main` requires `Capability::Interruption`.

## Row Reconciliation And Ledgers

All 35 rows reconcile exactly: 10 `llama-cpp.attached`, 6 `llama-cpp.owned`,
and 19 `ollama.attached` distinct tuples from the reviewed census
(`docs/triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv`),
with no filter or exception list and no `route-audit` row on any of the three
routes. Each row below carries its census lifecycle, state support, evidence
strength, value truth, and the current-main anchor that proves or disproves
prepared emission. Dispositions use the ledger vocabulary of the completed
tranches (qoder/zcode/codex): `E` = an exact prepared facade on current
`main` can emit the row; `W` = the row is withheld at construction under the
named withholding rule and counts as explicit negative coverage. Emitted
sets are maximum-profile sets; optional-request rows are emitted only by the
exact maximal prepared attempt, mirroring the conditional reasoning row in
the `codex.exec` proof.

### `llama-cpp.attached` — 10 rows, 9 E / 1 W

| # | Census tuple | Lifecycle / state / strength | Disposition | Current-main anchor and rule |
| --- | --- | --- | --- | --- |
| 1 | feature / model-catalogue / `feature.model-catalogue` | selection-summary / descriptor-only / matrix-cross-check+runtime-type | E | `LlamaCppPreparedCatalogue` plan requires `Capability::ModelCatalog` (`attached_capabilities`, `src/selection.rs`); catalogue observation stays observation, no mutation authority. |
| 2 | feature / structured-run / `feature.structured-run` | selection-summary / descriptor-only / matrix-cross-check+runtime-type | E | `LlamaCppPreparedInferenceAttempt` plan requires `Capability::StructuredRun`. |
| 3 | feature / route-observation / `feature.streaming-events` | selection-summary / descriptor-only / matrix-cross-check+runtime-type | E | plan requires `Capability::StreamingEvents`; the SSE driver emits incremental events. |
| 4 | feature / route-observation / `feature.usage-evidence` | selection-summary / descriptor-only / matrix-cross-check+runtime-type | E | plan requires `Capability::UsageReporting`. |
| 5 | feature / route-capability / `feature.output-token-limit` | selection-summary / descriptor-only / matrix-cross-check+runtime-type | E | plan requires `Capability::OutputTokenLimit`; `LlamaCppInferenceProfileInput::new` requires the positive `NonZeroU64` bound. |
| 6 | feature / route-capability / `feature.cancellation-or-interruption` | selection-summary / descriptor-only / matrix-cross-check+runtime-type | W | docs-only rule. No attached prepared plan or instance profile requires `Capability::Interruption` (`attached_capabilities`, `attached_all_capabilities`, `src/selection.rs`); the run handle's driver-level cancellation (`src/driver/handle.rs`) is post-dispatch and not prepared evidence. The matrix posture cannot be widened into a prepared claim. |
| 7 | feature / route-capability / `feature.prepared-facade` | selection-summary / descriptor-only / matrix-cross-check+runtime-type | E | always emitted from the prepared facade record (`PreparedOperationEvidence`). |
| 8 | feature / route-observation / `feature.activity-observation` | post-open-observation-only / descriptor-only / runtime-export+route-matrix | E | `LlamaCppAttachedPreparedEvidence::new_with_activity` retains the `ObservableActivityProfile` (`src/prepared/attached/evidence.rs`, `src/activity/profile.rs`); post-open lifecycle, observation-only posture, descriptor-only state support — the proved candidate-H pattern. |
| 9 | control / structured-run / `control.model-selection` | selection-summary / requested;prepared;provider-effective-unobserved / runtime-public-type+route-validation | E | `LlamaCppModelSelection` (`ModelRouteId` + `ModelRouteRevision` + `ModelId`, `src/prepared/attached/input.rs`); exact-model-route control; provider-effective stays unobserved. |
| 10 | control / structured-run / `control.maximum-output-tokens` | session-start-only / requested;prepared;effective-unobserved / runtime-public-type+route-validation | E | portable `MaximumOutputTokens` control from the required `NonZeroU64` prepared bound (`LlamaCppInferenceProfileInput::new`, `src/prepared/attached/input.rs`); omission `Required`. |

### `llama-cpp.owned` — 6 rows, 5 E / 1 W

| # | Census tuple | Lifecycle / state / strength | Disposition | Current-main anchor and rule |
| --- | --- | --- | --- | --- |
| 1 | feature / route-capability / `feature.owned-runtime-lifecycle` | selection-summary / descriptor-only / matrix-cross-check+runtime-type | E | namespaced feature row from exact prepared ownership: `InstanceOwnership::HostOwnedEphemeral`, role `ServingInstanceLifecycle`, `OwnedServingHandle` path (`LlamaCppPreparedServingStart`, `src/prepared/owned.rs`; `src/driver/owned/roles.rs`); the zcode owned-runtime precedent (`with_owned_runtime_lifecycle`) applies. |
| 2 | feature / route-capability / `feature.prepared-facade` | selection-summary / descriptor-only / matrix-cross-check+runtime-type | E | always emitted from `LlamaCppOwnedPreparedEvidence` (`PreparedOperationEvidence`). |
| 3 | feature / route-observation / `feature.activity-observation` | post-open-observation-only / descriptor-only / runtime-export+route-matrix | W | absence-proved. `LlamaCppOwnedPreparedEvidence` retains only operation, artifact, context size, and reasoning (`src/prepared/owned/evidence.rs`); the serving-start plan is built with `from_plan` (no activity profile), and the crate's only `ObservableActivityProfile` binds the attached runtime (`src/activity/profile.rs`). No prepared owned evidence can emit the row. |
| 4 | control / session-lifecycle / `control.serving-model-artifact` | session-start-only / requested;prepared;effective-unobserved / runtime-public-type+route-validation | E | namespaced control from the retained `ModelArtifactBinding` + `LlamaCppModelSelection` (`LlamaCppOwnedServingSelection::new`, `src/prepared/owned/input.rs`); omission `Required` for owned server startup. |
| 5 | control / session-lifecycle / `control.serving-context-size` | session-start-only / requested;prepared;effective-unobserved / runtime-public-type+route-validation | E | namespaced bounded-integer control from `with_context_size`; domain `1..=i32::MAX` (`LlamaCppContextSize`, `src/context_size.rs`); omission `PreservesRouteBehavior`. |
| 6 | control / session-lifecycle / `control.serving-reasoning` | session-start-only / requested;prepared;effective-unobserved / runtime-public-type+route-validation | E | namespaced bounded-enum control from `with_reasoning`; exactly one admitted value, canonical literal `off` (`LlamaCppReasoningSelection::Disabled`, `src/reasoning.rs`); omission `PreservesRouteBehavior`; never a portable reasoning capability or `llama-cpp.attached` claim. |

The owned plan also carries `StreamingEvents` and `Interruption`
requirements (`owned_capabilities`), but neither has a `llama-cpp.owned`
census row, so the route-scoped emission mapping must skip both (zcode's
deliberate `feature_for` omission precedent); emitting either would invent a
row the census does not own.

### `ollama.attached` — 19 rows, 18 E / 1 W

| # | Census tuple | Lifecycle / state / strength | Disposition | Current-main anchor and rule |
| --- | --- | --- | --- | --- |
| 1 | feature / model-catalogue / `feature.model-catalogue` | selection-summary / descriptor-only / matrix-cross-check+runtime-type | E | `OllamaPreparedInventory` plan requires `Capability::ModelCatalog` (`src/prepared_profile/inventory.rs`); catalogue observation only. |
| 2 | feature / structured-run / `feature.structured-run` | selection-summary / descriptor-only / matrix-cross-check+runtime-type | E | attempt plan requires `Capability::StructuredRun`. |
| 3 | feature / interactive-session / `feature.interactive-session` | selection-summary / descriptor-only / matrix-cross-check+runtime-type | E | session plan requires `Capability::InteractiveSession` (24-turn bound, `src/prepared_profile/session.rs`). |
| 4 | feature / route-observation / `feature.streaming-events` | selection-summary / descriptor-only / matrix-cross-check+runtime-type | E | session and attempt plans require `Capability::StreamingEvents`. |
| 5 | feature / route-observation / `feature.usage-evidence` | selection-summary / descriptor-only / matrix-cross-check+runtime-type | E | both plans require `Capability::UsageReporting`. |
| 6 | feature / route-capability / `feature.output-token-limit` | selection-summary / descriptor-only / matrix-cross-check+runtime-type | E | both plans require `Capability::OutputTokenLimit` with an `OutputTokenMaximum` constraint; attempt input requires the `NonZeroU64` bound. |
| 7 | feature / route-capability / `feature.reasoning-selection` | selection-summary / descriptor-only / matrix-cross-check+runtime-type | E | exact structured-run evidence only, and only under the maximal attempt profile: `inference_capabilities` adds `Capability::ReasoningSelection` when `with_reasoning_mode` passes `validate_reasoning` (bound model reports `Thinking`, mode in `off`/`low`/`medium`/`high`). The interactive-session shape never carries it (no session reasoning input exists); no portable route-wide claim is made. |
| 8 | feature / route-capability / `feature.structured-output` | selection-summary / descriptor-only / matrix-cross-check+runtime-type | E | exact structured-run evidence only, under the maximal attempt profile with `with_structured_output` past `validate_structured_output` (`Capability::StructuredOutput` with dialect and provider-native enforcement constraints). |
| 9 | feature / route-capability / `feature.cancellation-or-interruption` | selection-summary / descriptor-only / matrix-cross-check+runtime-type | E | exact interactive-session evidence only: `session_capabilities` requires `Capability::Interruption` (`CancellationScope::ActiveTurn`, `src/prepared_profile/session.rs`). The structured-run shape is withheld: no attempt plan requires `Interruption` even though the run handle exposes driver-level cancellation (`src/driver/handle.rs`). The route row is published with the exact interactive-session applicability, never as a structured-run claim. |
| 10 | feature / route-capability / `feature.prepared-facade` | selection-summary / descriptor-only / matrix-cross-check+runtime-type | E | always emitted from the prepared facade record. |
| 11 | feature / route-observation / `feature.activity-observation` | post-open-observation-only / descriptor-only / runtime-export+route-matrix | E | session and attempt evidence retain the `ObservableActivityProfile` (`from_prepared_with_activity`, `src/prepared_profile/plan.rs`; `src/activity/profile.rs`); inventory evidence carries none, so the inventory facade emits no activity row. Post-open lifecycle, observation-only posture, descriptor-only state support. |
| 12 | control / structured-run / `control.model-selection` | selection-summary / requested;prepared;provider-effective-unobserved / runtime-public-type+route-validation | E | exact-model-route control from the integration-bound `OllamaModelSelection` on the attempt plan's model route. |
| 13 | control / interactive-session / `control.model-selection` | selection-summary / requested;prepared;provider-effective-unobserved / runtime-public-type+route-validation | E | same exact model route on the session plan; a distinct census row because the shape is distinct. |
| 14 | control / structured-run / `control.reasoning-selection` | session-start-only / requested;prepared;effective-unobserved / runtime-public-type+route-validation | E | bounded-enumeration control under the maximal attempt profile; route-qualified admitted values from `validate_reasoning`; omission `PreservesRouteBehavior`. |
| 15 | control / interactive-session / `control.reasoning-selection` | session-start-only / descriptor-only / matrix-descriptor-only | W | docs-only rule, absence proved: `OllamaSessionProfileInput` carries no reasoning field (`src/prepared_profile/input.rs`) and `OllamaPreparedSession::prepare_session` accepts no reasoning value; no current public session input exposes reasoning selection. The census retains the matrix descriptor only; publishing a session reasoning control would invent an owner that does not exist. |
| 16 | control / structured-run / `control.maximum-output-tokens` | session-start-only / requested;prepared;effective-unobserved / runtime-public-type+route-validation | E | portable `MaximumOutputTokens` control from the required `NonZeroU64` bound (route ceiling `u32::MAX`); omission `Required`. |
| 17 | control / structured-run / `control.structured-output` | session-start-only / requested;prepared;effective-unobserved / runtime-public-type+route-validation | E | namespaced structured-declaration control under the maximal attempt profile (`with_structured_output`, `validate_structured_output`); omission `SuppliesNothing`. |
| 18 | control / structured-run / `control.context-window` | session-start-only / requested;prepared;effective-unobserved / runtime-public-type+route-validation | E | namespaced bounded-integer control from `with_context_window`; domain `4..=i32::MAX` adapter-local `options.num_ctx` (`OllamaContextWindow`, `src/context_window.rs`); omission `PreservesRouteBehavior`. |
| 19 | control / interactive-session / `control.context-window` | session-start-only / requested;prepared;effective-unobserved / runtime-public-type+route-validation | E | same adapter-local bound on `OllamaSessionProfileInput::with_context_window`; distinct census row for the interactive-session shape; the prepared evidence retains the value (`OllamaPreparedEvidence.context_window`, `src/prepared_profile/plan.rs`). |

Route totals: `llama-cpp.attached` 9 emitted / 1 withheld; `llama-cpp.owned`
5 emitted / 1 withheld; `ollama.attached` 18 emitted / 1 withheld. Ledger
total 32 emitted / 3 withheld across all 35 rows, every row assigned exactly
once, nothing borrowed from another route or candidate.

## Construction-Time Withholding Rules

Documentation-only, catalogue-only, incompatible-operation, and unobserved
rows stay out of the emitted sets without an exception list:

1. **Documentation-only rows** — a census row whose exact truth exists only
   in the provider-solution matrices or the route matrix, with no prepared
   plan requirement, retained value, or runtime type on current `main`, is
   withheld at construction. Applies to `llama-cpp.attached`
   `feature.cancellation-or-interruption` (row 6) and `ollama.attached`
   interactive-session `control.reasoning-selection` (row 15). The matrix
   posture is documentation, not runtime authority; publishing either would
   widen a route claim from QA cross-check evidence.
2. **Absence-proved rows** — a row whose census evidence class requires a
   prepared record that current `main` proves absent is withheld and the
   absence is named. Applies to `llama-cpp.owned`
   `feature.activity-observation` (row 3): the owned prepared evidence
   retains no `ObservableActivityProfile` and no owned activity profile
   exists in the crate.
3. **Catalogue-only and incompatible-operation rows** — no `llama-cpp.owned`
   row family requires the catalogue rule (no catalogue row exists on that
   route), and no census row on any of the three routes belongs to an
   operation shape the route cannot prepare. The attached route's catalogue
   facade and the Ollama inventory facade keep their catalogue rows
   observation-only; no row is borrowed across shapes or routes.
4. **Unobserved rows** — every emitted control carries the census state
   support exactly: `requested; prepared` with provider-effective and
   rejected unobserved. Prepared success is not observation; activity rows
   stay descriptor-only at post-open-observation-only lifecycle. There is no
   acknowledgement, per-turn, negotiated-model-option, or
   provider-session-management row in the 35 to hold at a stronger band.

## Shape-Scope Findings

- **Attached and owned applicability stay distinct.** `llama-cpp.attached`
  (externally managed, `ExternalAttached`) and `llama-cpp.owned`
  (host-owned ephemeral, `HostOwnedEphemeral`) are separate routes with
  separate addable identities, access profiles, runtime windows, and drivers
  (`LLAMA_CPP_ATTACHED_ADDABLE_ROUTE_ID`, `src/addable.rs`;
  `OWNED_ROUTE`, `src/driver/owned.rs`). Serving-only controls
  (`serving-model-artifact`, `serving-context-size`, `serving-reasoning`)
  exist on `llama-cpp.owned` alone; attached inference never acquires an
  artifact, launches a process, or passes `--ctx-size`/`--reasoning`.
  `LlamaCppReasoningSelection` documents itself as owned-serving
  configuration, "not ... an `llama-cpp.attached` request option"
  (`src/reasoning.rs`). No mixed attached/owned assembly can be constructed;
  the existing cross-route mixture rejection applies.
- **Ollama is the first dual-shape route in the tranche series.** Its census
  carries twin rows on one route id — `control.model-selection`,
  `control.context-window`, and `control.reasoning-selection` each appear for
  both `structured-run` and `interactive-session`. Row identity therefore
  cannot be keyed by semantic id alone on this route; the implementation
  card's ledger must key by (operation shape, semantic id) and emit from the
  exact shape's facade, keeping each contribution's applicability shape
  exact. Optional-request rows (reasoning, structured output) exist only in
  the maximal attempt profile and only when route validation admits them.
- **Driver-level cancellation is not prepared evidence.** Both the llama.cpp
  attached run handle and the Ollama run handle expose
  `CancellationControl` at `CancellationScope::StructuredRun`
  (`src/driver/handle.rs` in each crate), and llama.cpp owned plans require
  `Interruption` at `OwnedServingInstance`. Projection publishes only what a
  prepared plan requires or a prepared record retains: attached interruption
  stays withheld, Ollama cancellation publishes on the interactive-session
  shape only, and the owned route's interruption requirement maps to no
  census row and is skipped.
- **No active-observation or acknowledgement surface exists or is needed.**
  None of the three routes retains a provider confirmation, rejected value,
  negotiated option observation, or session catalogue; no row in the 35 is
  post-open beyond the descriptor-only activity rows. No
  `open_session_with_projection`, acknowledgement source identity, or
  observation seam is required, unlike candidates D, F, and G.

## Rubric Verdict

1. **Pass.** The exact census row set reconciles to 35 (10 + 6 + 19) with no
   exception or filter list; every tuple is unique; no `route-audit` row is
   present or invented; the two named packages own no other routes in the
   census.
2. **Pass.** Every contributing facade is named with its retained evidence
   and source class above; prepared and route-observation families stay
   distinct; the three withheld rows each carry an explicit
   construction-time withholding rule, and their absences are proved with
   code references rather than asserted.
3. **Pass.** No new runtime/core public type, fixed maximum, composer
   failure, registry, enumeration, callback, provider payload, or contract
   amendment is needed. All emitted rows fit the existing closed vocabulary:
   portable feature and control identities where they exist
   (`PreparedFacade`, `ModelSelection`, `MaximumOutputTokens`, capability
   feature families); bounded namespaced extensions for the route-local
   serving, context-window, and structured-output controls under the
   established `ConsumerRouteNamespacedExtension::new(route, segment,
   semantic id)` pattern (codex exec, zcode, cline precedents); existing
   value kinds (`CapabilityState`, `Observation`, `ExactModelRoute`,
   `BoundedInteger`, `BoundedEnumeration`, `StructuredDeclarations`) and
   omission semantics (`Required`, `PreservesRouteBehavior`,
   `SuppliesNothing`, `NotSelectable`). Adapter-local bounds
   (`LlamaCppContextSize`, `LlamaCppReasoningSelection`, `OllamaContextWindow`)
   stay adapter-local, exactly as the census documents them. No stop-gap
   recorded.
4. **Pass with a fixture caveat.** Deterministic provider-free ledgers are
   writable for both packages; both crates already carry fixture-server
   support and prepared-facade tests (`tests/prepared_facades.rs`,
   `tests/owned_driver.rs`, `tests/attached_driver.rs`; Ollama
   `tests/prepared_facade.rs`), so no provider contact is needed. The
   implementation card must prove the per-route emitted sets above —
   including the shape-keyed Ollama twin rows and the maximal-profile
   conditional rows — and must assert the three withheld rows never appear,
   mirroring the qoder/zcode ledger fixtures.
5. **Pass.** Exactly two adapter packages (`swallowtail-adapter-llama-cpp`,
   `swallowtail-adapter-ollama`) against the four-package maximum; the card
   adds the semantic API, docs, Northstar, god-file, and diff checks when it
   changes those surfaces.
6. **Pass.** The card stops after one reviewable two-package tranche and
   claims nothing about candidates B, C, E, F, I, K, L, or the 767-row
   all-route audit.

## Disposition

Candidate J is promotable as one exact two-package tranche: card ownership
of `llama-cpp.attached` (10), `llama-cpp.owned` (6), and `ollama.attached`
(19) with the 32 emitted / 3 withheld ledger above, no public-baseline gate,
and no stop. The three withheld rows remain explicit negative coverage. No
operator decision is required by this audit; the route-local emission
details (namespaced segments, per-shape ledger fixtures, maximal-profile
reasoning/structured-output rows) are implementation-card matters under the
existing patterns and fail-closed machinery, not new public surface.

## Sources

- [Batch 9.4 checkpoint](2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
- [Card 067](../roadmaps/g05/batch-cards/067-contract-061-candidate-j-breadth-audit.md)
- [reviewed census](2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- `main` at `bab21839321a1b29da0b14209db32c8323a9d1c2`; adapter sources cited
  inline above
