# Contract 061 Candidate I Breadth Audit

Status: stopped; the two local-server post-open control rows have no honest
shared representation; candidate I is not promotable on current `main`
Owner: Tom
Date: 2026-09-04
Source: Card 066; handoff
`docs/handoffs/20260904-133530-g05-card066-candidate-i-breadth-audit.md`;
`main` at `bab21839321a1b29da0b14209db32c8323a9d1c2` (== `origin/main`,
worktree `g05-card066-candidate-i-breadth-audit`)

## Purpose

Audit Batch 9.4 candidate I (DeepSeek, DeepSeek Harness) against current
`main` under the Batch 9.4 promotion rubric and return one honest
disposition. This is planning evidence. It authorizes no Rust, no
implementation card, no provider contact, and no coverage claim. The census
CSV is not edited by this lane; every census-source correction below is
recorded here as evidence only.

## Row Reconciliation

The census holds 767 rows. Exactly 47 carry a `route_id` beginning
`deepseek`, split across the two owning packages with no filter or exception
list:

| Route | feature | control | Total | Owning package |
| --- | ---: | ---: | ---: | --- |
| `deepseek.continuation` | 11 | 8 | 19 | `swallowtail-adapter-deepseek` |
| `deepseek-harness.jsonrpc` | 10 | 1 | 11 | `swallowtail-adapter-deepseek-harness` |
| `deepseek-harness.local-server` | 12 | 5 | 17 | `swallowtail-adapter-deepseek-harness` |
| **Total** | **33** | **14** | **47** | |

This matches Card 066, the Batch 9.4 checkpoint's candidate I row
(`deepseek.continuation` 19; `deepseek-harness.jsonrpc` 11;
`deepseek-harness.local-server` 17), and the manifest's 47. Candidate I owns
zero of the nine explicit `audit.no-public-route-specific-selectable-control`
rows; those are assigned to candidates C, G, and H, so this candidate has no
negative-coverage audit rows to preserve. 249 rows are proved by merged
cards; 518 remain unproved, of which these 47 are candidate I's complete
remainder.

## Facade Map And Source Identity

Neither adapter references `consumer_route_projection` or
`AdapterContribution` anywhere (`grep` over both `src/` trees: zero hits), so
no DeepSeek contribution exists on current `main`. Every facade below is
prepared; the only post-open observation surfaces are the activity streams
and the web session catalogue/history/fork/archive operations.

### `deepseek.continuation` (`swallowtail-adapter-deepseek`)

| Facade | Definition | Evidence held | Source identity |
| --- | --- | --- | --- |
| `DeepSeekPreparedIntegration` | `src/prepared.rs:115-120`; entry `prepare_deepseek_direct` `src/prepared.rs:172-183` performs no provider work | instance/access inputs only | caller-supplied `ConfiguredInstanceId` + `ExecutionHostId`; drift gate `validate_execution_binding` `src/prepared.rs:153-168` |
| `DeepSeekPreparedRun` (structured-run) | `src/prepared_profile/run.rs:19-22`; `prepare_run` :80-157; `start_run` :54-62 | `DeepSeekPreparedEvidence` + `StructuredRunRequest` (:19-22) | caller-supplied `RequestId`; adapter mints `deepseek-direct:{request_id}` at start (`driver/run/start.rs:26-34`); `provider_run_ref()` -> `None` (`driver/run/handle.rs:40-42`) |
| `DeepSeekPreparedSession` (interactive-session) | `src/prepared_profile/session.rs:16-19`; `prepare_session` :77-149; `open_session` :47-59 | `DeepSeekPreparedEvidence` + `OpenDirectContinuationSessionRequest` | caller-supplied `RequestId`; adapter-local `RuntimeSessionId` at open (`driver/session.rs:83-90`); `provider_session_ref()`/`resume_binding()` -> `None` (`session.rs:131-136`) |
| `DeepSeekPreparedCatalogue` (model catalogue) | `src/prepared_profile/catalogue.rs:16-19`; `list_models` :47-55 returns `Vec<ModelCatalogEntry>` | `DeepSeekPreparedEvidence` (without activity) + `ModelCatalogRequest` | caller-supplied `RequestId` (`catalogue.rs:82`) |
| `DeepSeekPreparedEvidence` | `src/prepared_profile/plan.rs:11-15`; `from_prepared` :18-30; `from_prepared_with_activity` :32-48 (used by `run.rs:148` and `session.rs:140`) | `PreparedOperationEvidence` + optional `ObservableActivityProfile`, `ReasoningMode`, `DeepSeekThinkingMode` | the exact prepared `PreflightPlan` |

Route capability truth is frozen in `deepseek_v4_requirements`
(`src/selection.rs:146-196`: InteractiveSession, StreamingEvents, ToolCalls,
UsageReporting, OutputTokenLimit, ProviderManagedInferenceCache, reasoning,
Interruption/ActiveTurn) and `deepseek_v4_run_requirements_for_thinking`
(`src/selection.rs:214-258`: StructuredRun, StreamingEvents, UsageReporting,
OutputTokenLimit, ProviderManagedInferenceCache, optional reasoning,
Interruption/StructuredRun). The route emits real post-open activity:
`DeepSeekActivityProjection` (`src/activity.rs:21+`) produces
`swallowtail_runtime::ActivityObservation` values.

### `deepseek-harness.jsonrpc` (`swallowtail-adapter-deepseek-harness`)

| Facade | Definition | Evidence held | Source identity |
| --- | --- | --- | --- |
| `DeepSeekHarnessPreparedIntegration` | `src/prepared.rs:92-100`; `prepare_run` `src/prepared_profile.rs:201-262` | discovery observation + access evidence; no `PreparedOperationEvidence` at integration level | caller-supplied target/instance validated against the pinned static release claim (`src/discovery.rs:40-58`) |
| `DeepSeekHarnessPreparedRun` (structured-run) | `src/prepared_profile.rs:144-147`; `start_run` :175-183 | `DeepSeekHarnessPreparedEvidence` (`:78-82`, `PreparedOperationEvidence` via `from_plan_with_activity` :93-97) + `StructuredRunRequest` | caller-supplied `RequestId`; run id synthesized `deepseek-harness:{request_id}` (`src/driver.rs:181`); `provider_run_ref()` -> `None` (`src/handle.rs:83-85`) |
| `DeepSeekHarnessModelSelection` | `src/prepared_profile.rs:21-25`, consumed only by `prepare_run` :201-262 | exact model route | prepared input + route validation |

The jsonrpc prepared capability set is `capabilities()`
(`src/prepared_profile.rs:265-286`): StructuredRun, StreamingEvents,
ObservableActivity, UsageReporting, Interruption/StructuredRun,
WorkingResource/Read+Filesystem. The descriptor carries only Discovery and
StructuredRun roles (`src/driver.rs:30-32`); catalogue, history, fork, and
archive operations do not exist on this route. Post-open activity is real:
`DeepSeekHarnessActivityProjection` (`src/activity.rs:13-25`) emits
`ActivityObservation` through the run event stream
(`src/protocol.rs:798-802`, `src/handle.rs:87-89`).

### `deepseek-harness.local-server` (`swallowtail-adapter-deepseek-harness`)

| Facade | Definition | Evidence held | Source identity |
| --- | --- | --- | --- |
| `DeepSeekHarnessWebPreparedIntegration` | `src/web_prepared.rs:128-137`; entry `prepare_deepseek_harness_web` :498-521 | environment/endpoint/observation/access/instance | caller-supplied target validated against the pinned web release claim (`web_prepared.rs:524-586`) |
| `DeepSeekHarnessWebPreparedRun` (structured-run) | `src/web_prepared.rs:762-765`; `start_run` :793-801 | `DeepSeekHarnessWebPreparedEvidence` (:690-695, `PreparedOperationEvidence` :694) + request | caller-supplied `RequestId`; run id `deepseek-harness-web:{request_id}` (`web/driver.rs:891-895`); `provider_run_ref()` -> `None` (`web/driver.rs:1584-1586`) |
| `DeepSeekHarnessWebModelSelection` | `src/web_prepared.rs:633`, consumed by `prepare_run` :253-263 and the history binding check :361-367 | exact model route | prepared input + route validation |
| `DeepSeekHarnessWebPreparedSessionCatalogue` | `src/web_prepared.rs:842-846`; `prepare_session_catalogue` :301-354; `list_sessions` :880; `list_page` :889; `search_sessions` :912; `list_models` :930; `prepare_fork` :947 | `PreparedProviderSessionCatalogueEvidence` | prepared catalogue plan (`OperationShape::ProviderSessionCatalogue`, `DriverRole::ProviderSessionCatalogue`, :312-313) |
| `DeepSeekHarnessWebPreparedSessionHistory` | `src/web_prepared.rs:1084-1088`; `prepare_session_history` :357-437; `page_history` :1122 | `PreparedProviderSessionHistoryEvidence`; "exposes no resume handle" (:1083) | prepared history plan (`OperationShape::ProviderSessionHistory`, :390-391); plan deliberately carries no `Capability::Resume` (:1555-1562) |
| `DeepSeekHarnessWebPreparedFork` | `src/web_prepared.rs:996-999`; `execute` :1015-1040 -> `fork_session` (`web/driver.rs:282-316`) | catalogue plan + fork input; harness performs the fork and returns a new `SessionRef` | prepared binding |
| `DeepSeekHarnessWebPreparedArchive` | `src/web_prepared.rs:1185-1189`; `prepare_archive_session` :440-494; `execute` :1217-1225 -> `archive_session` (`web/driver.rs:679-756`) | catalogue plan + management input; result is `ProviderSessionManagementEffect::applied`/`unconfirmed_after_effect`/`failed_before_effect` (:741-753) | prepared binding |

The web prepared capability set is `run_capabilities()`
(`src/web_prepared.rs:1255-1270`): StructuredRun, StreamingEvents,
ObservableActivity, UsageReporting, Interruption/StructuredRun,
WorkingResource/Read. The web structured run emits no runtime activity
observations (`grep RuntimeEventKind::Activity` over `src/web/`: zero hits);
the activity profile is folded into the prepared evidence only
(`with_activity`, :1331-1348).

## Per-Row Ledger

Truth classes on current `main`: **named** (an exact prepared or observed
source exists and the established cards 022-024 contribution pattern can
carry it without new shared vocabulary), **withheld** (matrix or posture
truth with no prepared capability requirement; construction-time withheld
under the card 024 rule, recorded here as negative coverage), or **blocked**
(no honest shared representation exists; see the blocker).

### `deepseek-harness.jsonrpc` (11)

| Census row | Disposition |
| --- | --- |
| 01 `feature.model-catalogue` | withheld. `ModelCatalogEntry` is absent from the crate (`grep`: zero hits); the jsonrpc descriptor has no catalogue role. Matrix posture only. |
| 02 `feature.structured-run` | named. `Capability::StructuredRun` in `capabilities()` (`prepared_profile.rs:267`). |
| 03 `feature.streaming-events` | named. `prepared_profile.rs:268`. |
| 04 `feature.usage-evidence` | named. `Capability::UsageReporting`, `prepared_profile.rs:270`. |
| 05 `feature.cancellation-or-interruption` | named. `Capability::Interruption` with `CancellationScope::StructuredRun`, `prepared_profile.rs:271-276`. |
| 06 `feature.working-resource` | named. `Capability::WorkingResource` Read/Filesystem, `prepared_profile.rs:277-282`. |
| 07 `feature.owned-runtime-lifecycle` | withheld. No capability requirement maps; ownership is carried as `with_ownership_modes` (`prepared_profile.rs:223`), not a projection capability. Matrix posture. |
| 08 `feature.persistent-session-posture` | withheld. No capability requirement; isolation posture is `HarnessIsolation::AmbientHost` (`prepared_profile.rs:246`). Matrix posture. |
| 09 `feature.prepared-facade` | named. Emitted unconditionally from the exact prepared plan per the card 024 pattern (`DeepAgentsPreparedSession` emission, `swallowtail-adapter-deepagents/src/consumer_route_projection.rs:92-99`). |
| 10 `feature.activity-observation` | named. `Capability::ObservableActivity` (`prepared_profile.rs:269`) plus the real jsonrpc activity stream (`activity.rs:13-25`, `protocol.rs:798-802`); descriptor-only, post-open-observation-only, active-session view. |
| 34 `control.model-selection` | named. `DeepSeekHarnessModelSelection` (`prepared_profile.rs:21-25`) admitted only inside `prepare_run` (:201-262); requested+prepared, provider-effective unobserved. |

### `deepseek-harness.local-server` (17)

| Census row | Disposition |
| --- | --- |
| 11 `feature.model-catalogue` | withheld. `ModelCatalogEntry` is absent from the crate; `list_models` is session-scoped (`web_prepared.rs:929-940` takes a `SessionRef` and returns adapter-local `DeepSeekHarnessWebModel`), which is negotiated-session truth, not a route catalogue. Matrix posture. |
| 12-15 `structured-run` / `streaming-events` / `usage-evidence` / `cancellation-or-interruption` | named. `run_capabilities()` (`web_prepared.rs:1256-1270`). |
| 16 `feature.provider-session-catalogue` | named. The prepared catalogue facade exists (`web_prepared.rs:842-846`); the Kimi gate fixes exactly this emission: the prepared `feature.provider-session-catalogue` row and nothing more (`docs/triage/2026-09-01-contract-061-kimi-active-observation-public-baseline-gate.md` lines 176-181). |
| 17 `feature.working-resource` | named. Read-only capability, `web_prepared.rs:1272-1281`. |
| 18 `feature.provider-session-archive` | named. The prepared archive facade exists (`web_prepared.rs:1185-1189`). |
| 19 `feature.owned-runtime-lifecycle` | withheld. Same rule as row 07. |
| 20 `feature.persistent-session-posture` | withheld. No capability requirement; the nearest runtime sources are the offline operation policy (`web_operation_policy`, `web_prepared.rs:1283-1289`) and the durable driver state policy (`web/driver.rs:402-404`), neither of which is a projection capability. Matrix posture. |
| 21 `feature.prepared-facade` | named. Card 024 pattern. |
| 22 `feature.activity-observation` | named. `Capability::ObservableActivity` (`web_prepared.rs:1260`) and the prepared activity profile (:1288-1348); descriptor-only. Recorded nuance: the web run currently emits no runtime activity events, so this row must never carry an observed claim. |
| 35 `control.model-selection` | named. `DeepSeekHarnessWebModelSelection` (`web_prepared.rs:633`), prepared-run and history-binding validated. |
| 44 `control.provider-session-catalogue` | **blocked.** See the blocker section. |
| 45 `control.provider-session-history` | **blocked.** See the blocker section. |
| 46 `control.provider-session-fork` | named. Prepared fork with a validated native ack (`web_prepared.rs:1015-1040`, `web/driver.rs:282-316`); session-start lifecycle action; no portable `ConsumerRouteControlId` variant exists, so identity would be the bounded namespaced extension exactly as card 024 published ZCode's route-local `control.app-server-mode` (`swallowtail-adapter-zcode/src/consumer_route_projection/builder.rs:182`). |
| 47 `control.provider-session-archive` | named. Prepared archive with explicit applied / unconfirmed-after-effect / failed-before-effect outcomes (`web/driver.rs:741-753`); same namespaced-identity path as row 46. |

Census-nuance recorded for rows 46/47: the census marks their state
`requested;prepared;provider-effective-unobserved`, but current `main`
retains exact effective outcomes (applied / unconfirmed / failed-before) as
returned values. An implementation card may honestly emit more than the
census credits; this lane preserves the census lifecycle and records the
correction as evidence only.

### `deepseek.continuation` (19)

| Census row | Disposition |
| --- | --- |
| 23 `feature.model-catalogue` | named. `DeepSeekPreparedCatalogue::list_models -> Vec<ModelCatalogEntry>` (`prepared_profile/catalogue.rs:47-55`) is an exact prepared catalogue operation; the prepared-catalogue-feature emission precedent is the Kimi gate lines 176-181. |
| 24 `feature.structured-run` | named. `selection.rs:220`. |
| 25 `feature.interactive-session` | named. `selection.rs:152`. |
| 26 `feature.streaming-events` | named. `selection.rs:153`, `:221`. |
| 27 `feature.usage-evidence` | named. `selection.rs:155`, `:222`. |
| 28 `feature.output-token-limit` | named. `Capability::OutputTokenLimit` (`selection.rs:156`, `:223`); prepared bound enforced at `prepare_run` (`run.rs:92`). |
| 29 `feature.reasoning-selection` | named. `deepseek_reasoning_requirement` (`selection.rs:158`, `:226-228`); admitted values `low`/`high`/`max` (`selection.rs:30-32`). |
| 30 `feature.consumer-tool-exchange` | named. `Capability::ToolCalls` (`selection.rs:154`); session tools bounded 1..=8 (`session.rs:93-95`); the structured run is tool-free by construction (`input.rs:16`, `driver/run/start.rs:181-183`). |
| 31 `feature.cancellation-or-interruption` | named. Interruption with ActiveTurn (session, `selection.rs:159-164`) and StructuredRun (run, `selection.rs:229-234`) scopes. |
| 32 `feature.prepared-facade` | named. Card 024 pattern. |
| 33 `feature.activity-observation` | named. `from_prepared_with_activity` (`plan.rs:32-48`, used at `run.rs:148` and `session.rs:140`) plus the real `DeepSeekActivityProjection` stream (`activity.rs:21+`); descriptor-only. |
| 36-37 `control.model-selection` (both shapes) | named. `DeepSeekModelSelection` (`prepared_profile/input.rs:128-154`); `ModelRoute` is constructed at `plan.rs:94-108` and pinned to provider `deepseek` (:107); model pinned `deepseek-v4-pro` (`run.rs:129-135`). |
| 38 `control.reasoning-selection` (structured-run) | named. Exactly-one-of reasoning or thinking mode (`run.rs:86-90`); `DeepSeekThinkingMode` is a single-value `disabled` type (`src/thinking.rs:6-21`) that explicitly disclaims provider-effective truth (:3-5). |
| 39 `control.reasoning-selection` (interactive-session) | named. Required `ReasoningMode` (`input.rs:158-164`), validated at `session.rs:90-92`. |
| 40 `control.maximum-output-tokens` | named. `NonZeroU64` with the prepare-time `u32::MAX` bound (`run.rs:92`). |
| 41-42 `control.inference-cache-policy` (both shapes) | named. Pinned to `ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority` (`run.rs:93`; `session.rs:82-88`); the policy is a two-value closed public enum (`swallowtail-core/src/direct_continuation.rs:22-28`). No portable `ConsumerRouteControlId` variant exists; identity follows the namespaced-extension path of rows 46/47. |
| 43 `control.tool-declarations` | named. `DeepSeekSessionProfileInput` tools 1..=8 (`session.rs:93-95`), re-checked by `validate_deepseek_request_plan` (`selection.rs:261-303`). |

Ledger total: 39 named, 6 construction-time withheld, 2 blocked. 39 + 6 + 2
= 47.

## Construction-Time Withholding Rules

1. Matrix and posture rows (01, 07, 08, 11, 19, 20) are withheld at
   construction: no prepared capability requirement or exact operation
   proves them, and `ConsumerRouteEvidenceStrength` has no matrix variant
   (`consumer_route_projection/semantics/authority.rs:23-25`), so a
   matrix-only row cannot enter a projection.
2. Documentation-only evidence never establishes support, availability, or
   applicability; the feature matrices remain QA cross-checks.
3. Unobserved rows stay unobserved: every control row publishes
   requested/prepared truth only; provider-effective and rejected claims
   require an exact acknowledgement source, which no DeepSeek route retains
   (`admission.rs:189-204` rejects unbacked claims).
4. Incompatible-operation rows fail closed through exact applicability; the
   composer rejects any contribution whose applicability differs from the
   anchored prepared record (`compose.rs:96-99`).
5. Withholding is by construction, never by filter: each route's contribution
   is built only from its exact prepared plan and capability requirements
   (`deepagents` pattern, `consumer_route_projection.rs:91-126`), so an
   unsupported row has no construction path at all.

## Blocker: Two Local-Server Post-Open Controls Have No Shared Representation

Census rows 44 (`control.provider-session-catalogue`) and 45
(`control.provider-session-history`) carry lifecycle
`post-open-observation-only` and state `descriptor-only;observed`. Their
sources on current `main` are prepared provider-operation queries, not open
sessions:

- `list_sessions` runs one `POST /api/session.list` under the catalogue plan
  and returns `ProviderSessionCatalogueOutcome::new(...,
  CleanupOutcome::NotApplicable)`; it opens no session, returns no session
  handle, and retains nothing (`web/driver.rs:385-555`, outcome at
  :546-552).
- `page_history` replays `POST /api/session.history` into
  `ProviderSessionHistoryPage` with `CleanupOutcome::NotApplicable`
  (`web/driver.rs:558-676`, page at :673); the prepared facade "exposes no
  resume handle" (`web_prepared.rs:1083`) and its plan deliberately carries
  no `Capability::Resume` (`web_prepared.rs:1555-1562`).

Publishing either row therefore requires all three of the following shared
names to cover a completed provider-operation query, and current
`swallowtail-runtime` defines each one as session-scoped:

| Required | Current public definition | Location |
| --- | --- | --- |
| `ConsumerRouteProjectionSourceKind::ActiveSessionObservation` | "One exact post-open active-session observation." | `consumer_route_projection/identity.rs:16-17` |
| `ConsumerRouteLifecycle::PostOpenObservationOnly` | "Observed only after the session opens." | `consumer_route_projection/semantics/posture.rs:14-15` |
| `ConsumerRouteActiveSessionState` | "Immutable post-open observation and exact negotiated state." | `consumer_route_projection/views.rs:35-36` |

This is the identical conflict the Kimi gate recorded for
`KimiPreparedSessionCatalogue::list_sessions`, verified unchanged on current
`main`, and that gate pre-adjudicates the substitution: prepared evidence
"must not emit `control.provider-session-catalogue` in any state", because
prepared evidence would backdate observed truth to preparation
(`docs/triage/2026-09-01-contract-061-kimi-active-observation-public-baseline-gate.md`
lines 47-81 and 176-181). The same ruling applies to row 45 and to DeepSeek.
No adapter-local arrangement resolves it: emitting under
`PreparedOperation`/`AdapterContribution` would fabricate session-start or
descriptor truth for an observed row, and emitting under
`ActiveSessionObservation` would be false because no session is opened.

Per the fixed boundary this is a stop-and-record gap, not a design task: no
new shared source kind, lifecycle band, view, or contract amendment is
proposed here.

## Rubric Verdicts

1. **Exact census set reconciles without an exception list — pass.** 19 + 11
   + 17 = 47 with no filter; candidate I owns the complete DeepSeek and
   DeepSeek Harness remainders and zero no-control audit rows (all nine live
   in C, G, and H).
2. **Every contributing facade and source-identity kind named; withholding
   rules explicit — fail.** All prepared facades are named above with
   caller-supplied source identity and the `AdapterContribution` kind they
   would use, and the six withheld rows have construction-time rules. It
   fails on rows 44 and 45: their contributing operations have no honest
   source-identity kind to publish under on current `main`.
3. **No new runtime/core public type, fixed maximum, composer failure,
   registry, enumeration, callback, provider payload, or contract amendment
   — fail.** Rows 44/45 require exactly such a shared decision (new source
   kind, lifecycle band, or view widening). Verified as *not* needed for the
   other 45 rows: every census value kind has a `ConsumerRouteValueKind`
   variant (`value.rs:81-114`); route-local control identities for rows
   41/42/46/47 fit the bounded namespaced-extension pattern card 024 already
   used; and every per-route view demand fits the fixed maxima (largest:
   12 selection rows against 32, 8 session-start rows against 16,
   2 active-session rows against 8 — `consumer_route_projection.rs:51-55`).
4. **Deterministic adapter-local ledgers without provider contact — pass for
   the 45 representable rows.** All preparation is provider-free
   (`prepare_deepseek_direct` performs no provider work,
   `prepared.rs:171-183`; harness preparation builds from
   `InstalledExecutableObservation` and pinned release claims), so emitted
   and withheld sets are constructible offline. The two blocked rows cannot
   be proven at all on current `main`.
5. **Focused validation names at most four adapter packages — pass.** Two:
   `swallowtail-adapter-deepseek` and `swallowtail-adapter-deepseek-harness`.
6. **One reviewable tranche, no later-candidate claims — pass.** This audit
   compiles nothing and claims nothing outside candidate I's 47 rows.

Rubric items 2 and 3 fail for the whole package set, and the fixed boundary
forbids exception lists, so candidate I cannot be promoted as one exact
package tranche on current `main`.

## Census-Source Corrections (Evidence Only)

The census CSV is outside this lane's owned paths; these corrections are
recorded for Chatterbox:

- Rows citing `crates/swallowtail-adapter-deepseek-harness/src/lib.rs` as the
  `public_source` of `Capability`, `CapabilityProfile`,
  `CapabilityRequirement`, `PreparedOperationEvidence`, `PreflightPlan`, or
  `ObservableActivityProfile`: the adapter imports those from
  `swallowtail-core`/`swallowtail-runtime` (`prepared_profile.rs:4-17`,
  `validation.rs:1-7`, `web_prepared.rs:14-52`) and exports only
  adapter-local types (`lib.rs:26-54`).
- Rows 10/22/33 cite `ActivityEvent`; no such type exists in
  `swallowtail-core` or `swallowtail-runtime`. The runtime observation type
  is `ActivityObservation`.
- Rows 36/37 cite `ModelRoute` in `prepared_profile/input.rs`; `input.rs`
  imports only `ModelId`, `ModelRouteId`, `ModelRouteRevision`
  (`input.rs:3-5`); `ModelRoute` is constructed at
  `prepared_profile/plan.rs:94-108`.
- Rows 46/47 understate provable state: exact applied/unconfirmed/
  failed-before-effect outcomes are retained as returned values on current
  `main` (`web/driver.rs:741-753`).

## Disposition

**Stop. Candidate I is not promotable on current `main`.** Forty-five of 47
rows sit on established, vocabulary-complete patterns (39 named truth
sources, 6 construction-time withheld matrix postures), but the two
`deepseek-harness.local-server` post-open controls fail rubric items 2 and 3
on the same shared provider-operation observation gap that stopped candidate
F and the Kimi gate. The named blocker is unchanged: the operator's
provider-operation observation public-baseline decision. Per the g05.009
dispatch manifest, that decision reopens only from Card 064's catalogue-route
finding; candidate I joins the same queue and no implementation card should
be compiled from this note. Chatterbox should reconcile the six withheld
rows and the census-source corrections above when the decision lands.
