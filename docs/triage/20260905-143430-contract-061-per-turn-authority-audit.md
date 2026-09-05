# Contract 061 Per-Turn Authority Audit

Status: promoted 2026-09-05 into ready cards 097 (L), 098 (B), and 099 (K);
retained as the ledger evidence those cards own; pruned when the last closes
Owner: Tom
Date: 2026-09-05
Card: `docs/roadmaps/g05/batch-cards/096-contract-061-per-turn-authority-audit.md`

## Trigger

The Batch 9.4 reassessment held candidates B, K, and L because each carries
consumer-mediated per-turn truth that no earlier tranche proved. These are the
last 197 of the 767 census rows. Card 096 settles the authority and lifecycle
question once, without Rust and without promoting a candidate.

## Row Reconciliation

Counted directly from the reviewed census with no filter or exception list.

| Candidate | Adapter packages | Exact route rows | Total |
| --- | --- | --- | ---: |
| B | `swallowtail-adapter-alibaba-model-studio`; `swallowtail-adapter-anthropic`; `swallowtail-adapter-xai` | `alibaba.conversations` 19; `anthropic.managed-agent` 17; `anthropic.messages` 23; `xai.responses-websocket` 17 | 76 |
| K | `swallowtail-adapter-mistral-vibe`; `swallowtail-adapter-muse`; `swallowtail-adapter-oh-my-pi`; `swallowtail-adapter-qwen` | `mistral-vibe.headless` 8; `muse-code.headless` 10; `oh-my-pi.rpc` 18; `qwen.headless` 16 | 52 |
| L | `swallowtail-adapter-opencode`; `swallowtail-adapter-pi` | `opencode.http` 35; `pi.rpc` 15; `pi.sdk-sidecar` 19 | 69 |
| **Total** | **9 packages** | **11 route IDs** | **197** |

Lifecycle bands across all 197 rows: 136 selection-summary, 41
session-start-only, 12 post-open-observation-only, 8 per-turn. The 8 per-turn
rows are exactly the eight the Batch 9.4 checkpoint reserved to B, K, and L:
one in B, one in K, six in L.

Four of the 197 rows carry `matrix-descriptor-only` evidence strength and
therefore cannot enter any projection:
`alibaba.conversations control.provider-state-policy`,
`alibaba.conversations control.resume-session`,
`opencode.http control.reasoning-selection`, and
`opencode.http control.provider-turn-reference`. Each is withheld at
construction as negative coverage.

## Classified Rows

### Per-Turn Rows

| # | Candidate | Route | Row | Retained evidence and code anchor | Disposition |
| ---: | --- | --- | --- | --- | --- |
| 1 | B | `anthropic.managed-agent` | `control.consumer-tool-exchange` | `AnthropicManagedAgentRunInput` retains `tools` at `crates/swallowtail-adapter-anthropic/src/prepared_managed_profile/input.rs:43`, bounds them to eight at `prepared_managed_profile/run.rs:110`, and binds them into the public `StructuredRunRequest` at `run.rs:125`, readable through `AnthropicPreparedManagedAgentRun::request()` (`run.rs:35`) and `swallowtail-runtime/src/roles/api/basic.rs:127`. The plan independently carries `Capability::ToolCalls` with exact dialect, 16 KiB schema, and count-8 bounds at `crates/swallowtail-adapter-anthropic/src/managed_selection.rs:160`. | Emit as `PerTurn` + `ConsumerMediatedPerTurn`. Both the route's admitted exchange bounds and the consumer's supplied declarations are retained; neither is inferred from a prepared plan alone nor from a successful call. |
| 2 | K | `oh-my-pi.rpc` | `control.attachments` (interactive-session) | `OhMyPiSessionProfileInput::with_image_attachments` at `crates/swallowtail-adapter-oh-my-pi/src/prepared_profile/input.rs:110` flows into `session_capabilities(image_attachments)` at `prepared_profile/session.rs:92`, which pushes the exact `Capability::Attachments` requirement — `image/png`, 1 MiB, count 1 — at `prepared/instance.rs:94`. | Emit as `PerTurn` + `ConsumerMediatedPerTurn`. The bounded per-turn media capability is on the immutable plan and is absent when the consumer did not mediate it. |
| 3 | L | `pi.rpc` | `control.attachments` (interactive-session) | `PiSessionProfileInput::with_image_attachments` at `crates/swallowtail-adapter-pi/src/prepared_profile/input.rs:110` → `session_capabilities(image_attachments)` at `prepared_profile/session.rs:93` → `image_attachment_capability()` at `prepared/instance.rs:93`. | Emit as `PerTurn` + `ConsumerMediatedPerTurn`. |
| 4 | L | `pi.sdk-sidecar` | `control.attachments` (interactive-session) | `PiSdkSidecarSessionPreparation::with_image_attachments` at `crates/swallowtail-adapter-pi/src/sidecar/prepared.rs:83` → conditional `Capability::Attachments` at `sidecar/prepared/build.rs:62`. The sidecar driver already reads the truth back from the plan at `sidecar/driver.rs:111-114`. | Emit as `PerTurn` + `ConsumerMediatedPerTurn`. `sidecar/driver.rs:111` is the existing production readback proof that this evidence survives preparation. |
| 5 | L | `opencode.http` | `control.attachments` (interactive-session) | `OpenCodeSessionProfileInput::with_image_attachments` at `crates/swallowtail-adapter-opencode/src/prepared_profile/input.rs:368`; `Capability::Attachments` is filtered *out* of the session profile when the flag is false at `prepared_profile/operations/integration.rs:66`, and the bounded requirement is at `prepared/instance.rs:101`. `HostServiceKind::Attachment` is added to the requirements at `prepared_profile/plan.rs:104`. | Emit as `PerTurn` + `ConsumerMediatedPerTurn`. |
| 6 | L | `opencode.http` | `control.provider-callbacks` (structured-run) | `OpenCodeRunProfileInput::with_provider_callbacks` at `crates/swallowtail-adapter-opencode/src/prepared_profile/input.rs:477` → `run_requirements(..., provider_callbacks)` at `prepared_profile/plan.rs:187`, which attaches the exact `opencode/permission` and `opencode/question` extension namespaces at `prepared_profile/plan.rs:219`, defined at `driver/callback.rs:22` and `driver/callback.rs:26`. | Emit as `PerTurn` + `ConsumerMediatedPerTurn`. The retained evidence is the exact namespace pair on the plan's requirements, not the builder flag. |
| 7 | L | `opencode.http` | `control.provider-callbacks` (interactive-session) | `OpenCodeSessionProfileInput::with_provider_callbacks` at `prepared_profile/input.rs:375` → `SessionAccessPolicy::ambient_harness_with_consumer_mediated_requests(ResourceAccess::ReadWrite, namespaces)` at `prepared_profile/plan.rs:138`, bound to the requirements at `prepared_profile/plan.rs:147`, and defined at `crates/swallowtail-core/src/session_access.rs:243`. Callbacks and active-turn detachment are mutually exclusive at `prepared_profile/operations/integration.rs:43`. | Emit as `PerTurn` + `ConsumerMediatedPerTurn`. This is the strongest per-turn evidence in the corpus: an explicit consumer-mediated-request session access policy plus the exact namespaces. |
| 8 | L | `opencode.http` | `control.provider-turn-reference` | No runtime-public per-turn owner exists. The only public turn-reference member, `OpenCodeSessionReconciliationInput::with_provider_turn_ref` at `prepared_profile/input.rs:109`, belongs to session reconciliation, which rejects any turn reference at `prepared_profile/provider_sessions/reconciliation.rs:158` with `swallowtail.opencode.preparation.session_reconciliation_turn_ref_unsupported`. | **Withhold at construction.** Matrix-descriptor-only. Retain as negative coverage; do not relabel as session-start authority. |

### Attachment Rows

Thirteen rows carry attachment identity across the three candidates. Rows 2-5
above are the per-turn subset and are not repeated.

| Candidate | Route | Row | Lifecycle | Retained evidence and code anchor | Disposition |
| --- | --- | --- | --- | --- | --- |
| B | `anthropic.messages` | `feature.attachments` | selection-summary | Capability-state only. `Capability::Attachments` with `image/png`, 1 MiB, count 1 is pushed conditionally at `crates/swallowtail-adapter-anthropic/src/prepared_profile/inference.rs:187`. The route descriptor declares no attachment capability. | Emit as descriptor-only capability state **only from a facade whose capability profile actually carries the requirement**; withhold otherwise. Never emit from the feature matrix. |
| B | `anthropic.messages` | `control.attachments` | session-start-only | `AnthropicInferenceAttemptInput::with_attachments` at `prepared_profile/input.rs:125`; bound capability at `prepared_profile/inference.rs:187`; driver-side bound check at `driver/run.rs:210`. | Emit as `SessionStartOnly` + `PreparedSessionStart`. |
| K | `oh-my-pi.rpc` | `feature.attachments` | selection-summary | `image_attachment_capability()` at `crates/swallowtail-adapter-oh-my-pi/src/prepared/instance.rs:94`, admitted conditionally through `session_capabilities`/`run_capabilities` at `prepared/instance.rs:44` and `prepared/instance.rs:69`. | Emit as descriptor-only capability state, conditional on the facade's profile; withhold otherwise. |
| K | `oh-my-pi.rpc` | `control.attachments` (structured-run) | session-start-only | `OhMyPiRunProfileInput::with_attachments` at `prepared_profile/input.rs:171`; `image_attachments = !attachments.is_empty()` at `prepared_profile/run.rs:78`; capability at `prepared/instance.rs:94`. | Emit as `SessionStartOnly` + `PreparedSessionStart`. |
| L | `pi.rpc` | `feature.attachments` | selection-summary | `image_attachment_capability()` at `crates/swallowtail-adapter-pi/src/prepared/instance.rs:93`. | Emit conditional on the facade's profile; withhold otherwise. |
| L | `pi.rpc` | `control.attachments` (structured-run) | session-start-only | `PiRunProfileInput::with_attachments` at `prepared_profile/input.rs:169`; `run_capabilities(image_attachments)` at `prepared_profile/run.rs:80`; driver bound checks at `driver/validation/attachments.rs:63`. | Emit as `SessionStartOnly` + `PreparedSessionStart`. |
| L | `pi.sdk-sidecar` | `feature.attachments` | selection-summary | Conditional `Capability::Attachments` at `sidecar/prepared/build.rs:62`; readback at `sidecar/driver.rs:114`. | Emit conditional on the facade's profile; withhold otherwise. |
| L | `opencode.http` | `feature.attachments` | selection-summary | `image_attachment_capability()` at `crates/swallowtail-adapter-opencode/src/prepared/instance.rs:101`; explicitly filtered out of the profile when unrequested at `prepared_profile/operations/support.rs:8` and `operations/integration.rs:66`. | Emit conditional on the facade's profile; withhold otherwise. |
| L | `opencode.http` | `control.attachments` (structured-run) | session-start-only | `OpenCodeRunProfileInput::with_attachments` at `prepared_profile/input.rs:467`; route validation rejecting count > 1 and non-PNG media at `prepared_profile/operations/support.rs:33-46`; `image_attachments = !attachments.is_empty()` at `operations/integration.rs:132`. | Emit as `SessionStartOnly` + `PreparedSessionStart`. |

`anthropic.managed-agent feature.stream-reattachment` and
`anthropic.managed-agent control.stream-reattachment` are lexically similar but
are not attachment rows. They are stream reattachment policy, retained on
`AnthropicManagedAgentRunInput` at `prepared_managed_profile/input.rs:46` and
constrained to exactly one bounded reattachment at
`prepared_managed_profile/run.rs:101`. Both stay `SessionStartOnly` +
`PreparedSessionStart` with no provider-effective claim.

### Remaining Rows By Band

The other 180 rows carry no per-turn or attachment identity and classify
mechanically. Every one of the nine adapter packages exposes a
`PreparedOperationEvidence`-backed facade with public `evidence()`, `plan()`,
and `request()` accessors, so the source class is `PreparedOperationRecord`,
`CapabilityProfile`, `RuntimeRequestType`, or `AdapterPreparedInput` in every
case.

| Band | Rows | Classification | Authority |
| --- | ---: | --- | --- |
| selection-summary, `matrix-cross-check+runtime-type` | 119 | Descriptor-only capability or facade state. Provable only where the exact prepared capability profile carries the requirement; the feature matrix alone never admits a row (`ConsumerRouteEvidenceStrength` has no documentation variant — `crates/swallowtail-runtime/src/consumer_route_projection/semantics/authority.rs:26`). | `Absent`, `ObservationOnly` posture |
| selection-summary, `runtime-public-type+route-validation` | 17 | Exact model-route selection rows on each `prepare_run` / `prepare_session` facade. | `PreparedSessionStart` |
| session-start-only, `runtime-public-type+route-validation` | 38 | Prepared-input controls: reasoning, thinking mode, output-token maxima, harness mode, agentic-turn maxima, structured output, provider-state and retention/recovery policy, load/resume bindings, detachment. | `PreparedSessionStart` |
| session-start-only, `matrix-descriptor-only` | 3 | `alibaba.conversations control.provider-state-policy`, `alibaba.conversations control.resume-session`, `opencode.http control.reasoning-selection`. No public owner exists on current `main`. | Withheld at construction |
| post-open-observation-only | 12 | 11 `feature.activity-observation` rows plus `opencode.http control.provider-session-catalogue`. Descriptor-only activity and bounded query. | `Absent`, `ObservationOnly` posture |

11 + 119 = 130 selection-summary feature rows against 136 total; the remaining
6 selection-summary rows are the `feature.attachments` and
`control.model-selection` rows already itemised above. Bands sum to
119 + 17 + 38 + 3 + 12 + 8 (per-turn) = 197.

## Ruling: Existing Vocabulary Suffices

**No additive shared baseline is needed. No new runtime, core, or testkit
public type, no composer failure, and no Contract 061 amendment.** Every one of
the 197 rows, including all 8 per-turn rows, is representable today.

Four findings carry the ruling.

1. **The authority variant already exists and already fails closed.**
   `ConsumerRouteMutationAuthority::ConsumerMediatedPerTurn` is defined at
   `crates/swallowtail-runtime/src/consumer_route_projection/semantics/authority.rs:169`.
   `admit_lifecycle_authority` at
   `consumer_route_projection/admission.rs:211-236` rejects a `PerTurn` row
   that claims `PreparedSessionStart`, claims `Acknowledged`, claims
   `provider_effective` or `rejected` state, or is `ConsumerSelectable` without
   `ConsumerMediatedPerTurn`; it symmetrically rejects any non-per-turn row that
   claims `ConsumerMediatedPerTurn`. This is exactly the card's review-oracle
   counterexample — a per-turn row whose authority is inferred from a prepared
   plan — and it is already a composer failure, not a convention.

2. **Every honest per-turn row has retained plan-borne evidence.** Rows 2-7
   above resolve to either a bounded `Capability::Attachments` requirement or a
   `SessionAccessPolicy::ambient_harness_with_consumer_mediated_requests` plus
   the exact `opencode/permission` and `opencode/question` extension
   namespaces, all on the immutable `PreflightPlan`. Row 1 additionally retains
   the consumer's exact tool declarations on the public `StructuredRunRequest`.
   None of these is inferred from a successful local call, and each is absent
   from the plan when the consumer did not mediate it — proved by the explicit
   filters at `swallowtail-adapter-opencode/src/prepared_profile/operations/support.rs:8`
   and `operations/integration.rs:66`, and by the conditional pushes at
   `swallowtail-adapter-pi/src/prepared/instance.rs:88` and
   `swallowtail-adapter-oh-my-pi/src/prepared/instance.rs:88`.

3. **The projection vocabulary already spells the row identities.**
   `ConsumerRouteControlId::UserInputExchange` and
   `ConsumerRouteFeatureId::ConsumerToolExchange` / `QuestionExchange` exist at
   `consumer_route_projection/semantics/identity.rs:84`, `:27`, and `:29`.
   `ConsumerRouteValueKind::ExchangeCallback` exists at
   `consumer_route_projection/value.rs:111`. Attachment and provider-callback
   control rows use the bounded `Namespaced(ConsumerRouteNamespacedExtension)`
   identity, following the merged precedent at
   `crates/swallowtail-adapter-codex/src/consumer_route_projection/exec.rs:53`,
   which already publishes `control.attachments` that way.

4. **The one row that cannot be honest is already withheld by production
   code.** `opencode.http control.provider-turn-reference` has no per-turn
   owner, and the reconciliation path rejects every turn reference at
   `swallowtail-adapter-opencode/src/prepared_profile/provider_sessions/reconciliation.rs:158`.
   It stays negative coverage. No shared design is required to hold it.

### Named Precedent Check

The provider-operation-observation and compound-acknowledgement precedents both
opened because a *completed provider outcome* had no shared representation.
Neither applies here: no per-turn row in B, K, or L claims a provider outcome,
a provider acknowledgement, or provider-effective state. Per-turn truth in
these 197 rows is entirely consumer-side mediation admitted by the exact route,
which is precisely what `ConsumerMediatedPerTurn` was added for in card 022.

### Two Conditions For The Implementing Cards

Recorded, not designed. Neither requires a shared change.

- `ConsumerRouteFeatureId` has no `PermissionExchange` variant.
  `opencode.http feature.permission-exchange` must use a bounded `Namespaced`
  extension over the `opencode/permission` namespace. It must **not** be
  folded into `QuestionExchange`, which is a separate census row on the same
  route with its own namespace.
- `feature.attachments` selection-summary rows are conditional on the prepared
  capability profile, because no route descriptor declares
  `Capability::Attachments`. An implementing card must not emit them from a
  facade prepared without attachments, and must not widen them from the
  feature matrix.

## Rubric Verdicts

Applied against `dda64268` for each candidate's whole package set.

| Item | B | K | L |
| --- | --- | --- | --- |
| 1 rows reconcile without filters | Pass — 19+17+23+17 = 76 | Pass — 8+10+18+16 = 52 | Pass — 35+15+19 = 69 |
| 2 every facade and source identity named; documentation-only rows have a withholding rule | Pass — 13 prepared facades across three packages, all exposing `evidence()`/`plan()`/`request()`; two `alibaba.conversations` matrix-descriptor rows withheld | Pass — 9 prepared facades across four packages; no documentation-only rows | Pass — 15 prepared facades across two packages; two `opencode.http` matrix-descriptor rows withheld |
| 3 no new shared public type, bound, composer failure, registry, callback, provider payload, or contract amendment | Pass | Pass | Pass |
| 4 deterministic provider-free adapter-local ledgers prove emitted/withheld, disagreement, lifecycle and authority distinctions, negative coverage | Pass | Pass | Pass |
| 5 focused validation names at most four adapter packages | Pass — 3 packages | Pass — **exactly 4**, at the maximum | Pass — 2 packages |
| 6 one reviewable tranche, no claim on later candidates or the 767-row audit | Pass | Pass | Pass |
| **Verdict** | **Promotable** | **Promotable** | **Promotable** |

All three pass. The audit found no evidence stop and no candidate that needs a
gate. This is a different outcome from cards 030 and 033, and the reason is
narrow: acknowledgement candidates failed because their routes discarded a
provider confirmation, whereas per-turn candidates retain consumer-side
mediation on the plan, which nothing discards.

## Recommended First Promotion

**Promote L first.** This matches Chatterbox's expectation, on these grounds:

- L holds six of the eight remaining per-turn rows, so one card retires the
  per-turn question for the whole census rather than spreading it across three
  tranches.
- L is the only candidate with the full per-turn spectrum — bounded attachment
  media on three routes, consumer-mediated permission/question callbacks on two
  operation shapes, and one withheld descriptor-only row — so its ledgers are
  the strongest falsification surface for the `ConsumerMediatedPerTurn` posture.
- L is two adapter packages, the smallest validation scope of the three, and
  well inside the focused-validation maximum.
- `opencode.http` already exercises the sharpest fail-closed edges on current
  `main`: callbacks excluded under active-turn detachment
  (`operations/integration.rs:43`) and turn-scoped reconciliation rejected
  (`provider_sessions/reconciliation.rs:158`).

Suggested order after L: **B** second (76 rows, three packages, one per-turn
row whose evidence is the richest single case, plus two descriptor withholds),
then **K** last (52 rows but four packages, the least per-turn value, and the
only candidate sitting exactly at the four-package validation ceiling).

Expected ledger shape if promoted, stated as a target and not as proof:
`opencode.http` 33 emitted / 2 withheld, `pi.rpc` 15 / 0, `pi.sdk-sidecar`
19 / 0 — 67 emitted and 2 withheld across 69 rows.

## Stop

This note is evidence, not authority. No candidate is promoted, no shared
vocabulary is changed, no census or contract surface is edited, and no Rust
changed. Chatterbox owns promotion of the ruling and of at most one
implementation card per passing candidate.
