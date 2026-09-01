# 033 Contract 061 Card 032 Closeout And Kimi Reassessment

Status: complete; evidence stop; candidate F not promoted; no Rust change
Owner: Tom
Created: 2026-09-01
Updated: 2026-09-01
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: completed card 032; Batch 9.4 lifecycle-priority sequence

## Goal

Reconcile candidate G's reviewed 48-row completion, then reassess candidate F's
exact 89-row Kimi package remainder against current `main`. Promote at most one
implementation card if the complete candidate passes the existing Batch 9.4
rubric without a new operator decision; otherwise record the exact stop and
name the narrow next gate.

## Scope

1. Bind card 032 to PR 144's exact reviewed head and merge commit. Reconcile
   the five exact route ledgers, emitted/withheld totals, public-baseline
   changes, failure preservation, validation, and no-contact boundary.
2. Update Contract 061 coverage from 201 to 249 proved rows. Reconcile the
   exact 518-row remainder across candidates B, C, E, F, and I-L with no filter
   or exception list.
3. Reconcile candidate F's complete 89-row remainder: 25
   `kimi-code.acp`, 20 `kimi-code.headless`, 31 `kimi-code.local-server`, and
   13 `kimi-platform.chat` rows across the Kimi and Kimi Platform packages.
4. Name every prepared facade, active-observation facade, source identity, and
   lifecycle transition F needs. Trace exact provider evidence for compound
   reasoning-and-plan acknowledgement, negotiated model options, and post-open
   provider-session catalogue observation.
5. Prove whether current `main` retains exact effective and rejected values,
   bounded model-option evidence, and catalogue observation strongly enough to
   form route-local contributions. Documentation, prepared success, session
   existence, or static failure codes are not active evidence.
6. Identify exact construction-time withholding for catalogue-only,
   incompatible-operation, documentation-only, and unobserved rows. Do not
   narrow the adapter-package remainder around a blocker.
7. Apply the Batch 9.4 promotion rubric: exact ledgers, fail-closed
   source/applicability assembly, lifecycle and authority distinctions,
   package extraction, public API stability, and the four-package focused
   validation maximum.
8. If F passes without a new product or public-baseline choice, compile one
   numbered implementation card with exact facades, sources, rows,
   counterexamples, validation, stops, and one-PR scope. If it does not, keep
   F unpromoted and name the smallest Kimi-only public-baseline gate plus every
   operator decision it needs. Do not answer those decisions in this card.
9. Reconcile the Batch 9.4 checkpoint, milestone, g05 front door, generation
   index, batch-card index, closeout log, and sole Next Task. Stop for
   orchestrator exact-head review.

## Out Of Scope

- Rust, manifests, release baselines, contracts, architecture, or census edits
- implementing candidate F or any route-local Kimi public-baseline repair
- deciding whether rejected Kimi reasoning stays adapter-local or changes the
  runtime `EffectiveReasoningSetup` surface
- inventing acknowledgement, negotiated-model-option, or catalogue semantics
- candidates B, C, E, I-L, Batch 9.5, or generation closeout
- provider contact, live probes, compatibility/currentness, watcher, skill
  inventory, or papercut work

## Acceptance Criteria

- [x] card 032's exact 11/8/11/9/9 route ledgers and 9/2, 7/1, 10/1, 6/3,
      6/3 emitted/withheld totals bind to the reviewed PR 144 head and merge
- [x] coverage reconciles exactly to 249 proved and 518 remaining rows, with
      candidate F's 89 rows and the other seven candidates assigned once
- [x] all four F route shapes and both package remainders reconcile without a
      filter, exception list, partial package, or borrowed identity
- [x] every required prepared facade, active-observation facade, source, exact
      transition, and absence on current `main` is named
- [x] compound acknowledgement, negotiated model options, and post-open
      catalogue observation remain distinct lifecycle and authority families
- [x] prepared success, documentation, session existence, and static mismatch
      diagnostics cannot masquerade as retained active-session truth
- [x] construction-time withholding and negative coverage are explicit for
      every affected route
- [x] F has a concrete deterministic ledger and mixed-assembly proof plan that
      preserves lifecycle, authority, source, route, operation, instance,
      access, acknowledgement, model-option, and catalogue truth
- [x] exactly one F implementation card is ready without a new decision, or
      the checkpoint proves why none can be promoted and names the exact
      Kimi-only operator gate
- [x] shared planning surfaces and the sole Next Task agree; no other
      candidate gains execution authority

## Review Oracle

- count card 032 as complete without binding its exact five route ledgers,
  failure preservation, and reviewed merge — fail
- reach 249/518 through a filtered census remainder, duplicated row, omitted
  candidate, or stale pre-card-032 count — fail
- promote F because Kimi documentation advertises a feature while current
  adapter evidence is discarded or unbounded — fail
- flatten reasoning acknowledgement, Plan acknowledgement, model-option
  observation, and provider-session catalogue into one supported boolean —
  fail
- emit an active row from prepared success, session existence, a static
  diagnostic, or another Kimi route's source — fail
- narrow F around `kimi-code.acp` blockers or omit a package/route to fit the
  tranche — fail
- select F despite needing an unresolved adapter/runtime public-baseline or
  product decision — stop and name the gate; do not compile implementation
- promote B, C, E, or I-L while performing the F checkpoint — fail

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

No Rust validation, live probe, or provider contact belongs to this planning
card.

## Auto-Continuation

No. Return one reviewable planning PR. The orchestrator reviews the exact
coverage closeout, candidate F disposition, and any compiled implementation
card before another worker is dispatched.

## Stop Conditions

- Stop if card 032's reviewed result cannot reconcile to the census and merged
  source without changing Rust or the census.
- Stop if F needs a runtime/testkit/core public type, composer rule, fixed
  maximum, callback, registry, generic provider payload, or contract amendment.
- Stop if current Kimi evidence cannot represent an exact rejected value,
  bounded model options, or post-open catalogue observation without a new
  operator decision.
- Stop if scope widens to another candidate, implementation, provider contact,
  Batch 9.5, or another product track.

## Closeout

Evidence stop. Candidate F is not promoted, no implementation card is
compiled, and no Rust, manifest, baseline, contract, architecture, or census
file changed. No provider was contacted and no live probe ran.

### Card 032 Binding

PR 144 merged exact reviewed head
`e50e3f4619451e066df3b2b6b37d045be28e370e` through
`18a6907e75e55a6b181632a1da35a2fefd0824fe`. The squash merge preserves the
reviewed head exactly: both commits resolve to tree
`5c4774f2f5db4c6ee58c2b2d4ea66a8088ab2f5e` and `git diff` between them is
empty. The five ledgers reconcile against the merged source, not the card
prose:

| Route | Census | Emitted | Withheld | Merged ledger source |
| --- | ---: | ---: | ---: | --- |
| `cline.acp` | 11 | 9 | 2 | `swallowtail-adapter-cline/tests/consumer_route_projection/ledger.rs` |
| `cline.headless` | 8 | 7 | 1 | same 19-tuple `LEDGER` |
| `command-code.headless` | 11 | 10 | 1 | `swallowtail-adapter-command-code/tests/consumer_route_projection.rs` |
| `copilot-cli.acp` | 9 | 6 | 3 | `swallowtail-adapter-copilot-cli/tests/consumer_route_projection.rs` |
| `goose.acp` | 9 | 6 | 3 | `swallowtail-adapter-goose/tests/consumer_route_projection.rs` |
| **Total** | **48** | **38** | **10** | |

Only the four candidate G adapter semantic API baselines and the package index
changed under `release-baselines/public-api-unreleased/`. No runtime, testkit,
core, or other adapter baseline moved. Cline's preserved and projected opens
share one private lifecycle and one Plan failure code. The card carried no
provider contact or live probe.

### Coverage Reconciliation

The census holds 767 rows across 48 route IDs. Repartitioning it independently
of the Batch 9.4 tables, every route ID is assigned exactly once with no
duplicate and no omission:

| State | Cards / candidates | Routes | Rows |
| --- | --- | ---: | ---: |
| proved | 022, A/023, H/024, D/031, G/032 | 16 | 249 |
| remaining | B, C, E, F, I, J, K, L | 32 | 518 |
| **Total** | | **48** | **767** |

Proved decomposes as 51 + 59 + 38 + 53 + 48. Remaining decomposes as B 76,
C 94, E 56, F 89, I 47, J 35, K 52, L 69. No filter, exception list, or
duplicated row reaches either total.

### Candidate F Exact Remainder

F owns the complete census remainder of `swallowtail-adapter-kimi` and
`swallowtail-adapter-kimi-platform`: 25 `kimi-code.acp`, 20
`kimi-code.headless`, 31 `kimi-code.local-server`, and 13
`kimi-platform.chat` rows. All 89 `(route_id, operation_shape, semantic_id)`
tuples are distinct. Owning packages split 76 / 13. The remainder holds no
`audit.no-public-route-specific-selectable-control` row and no per-turn row,
so F adds neither negative-coverage family nor consumer-mediated authority.

Lifecycle partition of the 89 rows: 60 selection-summary, 22
session-start-only, and 7 post-open-observation-only. Four of the seven
post-open rows are `feature.activity-observation`, one per route, and stay
descriptor-only under the pattern cards 023, 024, 031, and 032 already proved.
The remaining three sit on `kimi-code.acp` alone and are the whole blocker:

| Row | Operation shape | Census state support |
| --- | --- | --- |
| `feature.active-session-reasoning-and-plan-ack` | interactive-session | requested; pending; effective; rejected |
| `feature.negotiated-model-options-observation` | interactive-session | descriptor-only; current-and-advertised |
| `control.provider-session-catalogue` | session-management | descriptor-only; observed |

86 of 89 rows therefore rest on already-proved prepared patterns; exactly 3
rows on one route decide the candidate.

### Named Facades And Sources

Prepared facades exist for every route and need no new decision:

- `kimi-code.acp` — `KimiPreparedSession`, `KimiPreparedEvidence`,
  `KimiPreparedSessionCatalogue`, `KimiPreparedSessionImport`,
  `KimiModelSelection`, `KimiAcpSessionImportAuthority`
- `kimi-code.headless` — `KimiHeadlessPreparedRun`,
  `KimiHeadlessPreparedEvidence`
- `kimi-code.local-server` — `KimiLocalServerPreparedRun`,
  `KimiLocalServerPreparedSession`, `KimiLocalServerPreparedCatalogue`,
  `KimiLocalServerPreparedArchive`, `KimiLocalServerPreparedRestore`,
  `KimiLocalServerPreparedReconciliation`,
  `KimiLocalServerPreparedBindingImport`, `KimiLocalServerSessionConfiguration`
- `kimi-platform.chat` — `KimiPlatformPreparedCatalogue`,
  `KimiPlatformPreparedInferenceAttempt`, `KimiPlatformPreparedEvidence`,
  `KimiPlatformModelSelection`

Active-observation facades: none exist. Neither adapter defines a
`consumer_route_projection` module, a projected-open outcome, or an
`ActiveSessionObservation` source. Only `AdapterContribution` prepared sources
are reachable today.

### Current-Main Evidence For The Three Coupled Families

Traced read-only through the retained source, not through the matrices.

**1. Compound reasoning-and-plan acknowledgement — not retained, and the
reasoning half is additionally unrepresentable.**

`driver.rs` obtains both confirmations inside `open_session` and drops both:
`let _ = selection.confirm(&confirmation, selected.behavior())?;` and
`mode::confirm_plan_mode(&confirmation)?`. `KimiSessionHandle` has no
acknowledgement field, and `KimiPreparedSession::open_session` returns
`Box<dyn InteractiveSessionHandle>`.

The two halves fail differently, and card 030 did not separate them:

- Plan. `driver/mode.rs` freezes the provider domain to exactly
  `["default", "plan", "auto", "yolo"]` in listed order and rejects any other
  shape as malformed. The exact rejected value is therefore bounded and
  present at the confirmation site, then discarded into the static
  `swallowtail.kimi.acp.harness_mode_mismatch` failure. This half is the exact
  analogue of the proved `cline.acp` Plan case.
- Reasoning. `driver/reasoning.rs` `validate_behavior_shape` deliberately
  admits foreign catalogue rows under `KimiAcpBehavior::DeclaredEffort`:
  "unknown rows do not make the whole option malformed". The advertised value
  domain is open, so a confirmation may carry an identifier outside the
  admitted set `{off, on, low, medium, high, xhigh, max}`. Unlike Cline's
  frozen `["plan", "act"]`, no bounded rejected reasoning domain exists on
  current `main` to publish.
- Reasoning, second loss. Under `DeclaredEffort` with requested `"on"`,
  `confirm` normalizes any non-`"off"` current value to `"on"` before calling
  `NegotiatedReasoningSetup::confirm`. The exact provider-effective effort is
  discarded even when the confirmation succeeds. Kimi therefore lacks exact
  *effective* truth as well as exact rejected truth; candidates D and G each
  lacked only the rejected half.

`EffectiveReasoningSetup` cannot absorb any of this. Its only constructor is
`NegotiatedReasoningSetup::confirm`, which returns
`swallowtail.negotiated_reasoning.effective_mismatch` whenever
`requested != effective`, so the struct can hold nothing but equal requested
and effective modes.

**2. Negotiated model options — retained, but not publishable.**

This is the material change from card 030's reading. `driver/validation.rs`
`parse_model_options` extracts the exact `model` config option — one current
value plus the bounded advertised list with optional display names — and
`driver.rs` threads it into `attachment.take_session(...)` on open, load, and
resume. `KimiSessionHandle::negotiated_model_options()` returns
`Option<&NegotiatedSessionModelOptions>` through the generic runtime accessor.
Kimi already holds what the Cline gate had to authorize adding.

The remaining blocker is publication, not retention: no adapter-owned seam
carries a route- and revision-qualified `ActiveSessionObservation`
contribution out of the open, so the row cannot be formed without the same
class of additive public surface the Cline gate fixed.

**3. Post-open provider-session catalogue — a third seam shape, not a second.**

`driver/session_catalogue.rs` implements `ProviderSessionCatalogueDriver` and
`ProviderSessionImportDriver` for `KimiAcpDriver`, and
`KimiPreparedSessionCatalogue::list_sessions` returns
`ProviderSessionCatalogueOutcome`. That outcome exposes only `candidates()`,
`next_cursor()`, and `cleanup()`; the adapter retains nothing after the call
and the outcome carries no contribution. The observation is real but lives on
a separate prepared operation, not on an open interactive session, so the
`open_session_with_projection` shape proved for `openai.realtime`,
`claude-agent.acp`, and `cline.acp` does not reach it.

Emitting this row from `PreparedProviderSessionCatalogueEvidence` instead
would publish the census `observed` half from prepared success, which this
card's Review Oracle and Contract 061's absent-mutation-authority point both
reject.

### Construction-Time Withholding

Withholding rules derivable from current `main` without a new decision:

- Documentation-only rows. `feature.model-catalogue` has no
  `DriverRole::ModelCatalog` on `kimi-code.acp` or `kimi-code.headless` and is
  withheld there. `kimi-code.local-server` and `kimi-platform.chat` do carry
  that role, so their catalogue rows are a maximal-ledger determination for
  the implementation card, not an automatic withholding.
- Unobserved rows. All three `kimi-code.acp` post-open families above are
  withheld at construction while no active-observation facade exists.
- Incompatible-operation rows. `kimi-code.acp` carries two distinct provider
  state policies by operation shape: `prepared_profile/plan.rs` sets
  `SessionProviderStatePolicy::Prohibited` on the interactive-session plan,
  while `prepared_profile/provider_session_catalogue.rs` sets
  `DurableProviderSessionPreserved` on the `ProviderSessionCatalogue` and
  `ProviderSessionImport` requirements. Card 032's route-scoped persistence
  withholding must therefore become operation-shape-scoped on this route; the
  `session-lifecycle` `feature.persistent-session-posture` row belongs to the
  durable family, not the prohibited one.
- Cross-route rows. `kimi-code.headless`, `kimi-code.local-server`, and
  `kimi-platform.chat` have no active-session evidence at all; any ACP
  observation reaching them fails route and operation applicability.

### Rubric Result

| Item | Result |
| --- | --- |
| 1 exact set reconciles without exception list | pass — 89 distinct tuples, two complete package remainders |
| 2 every contributing facade and source named | **fail** — no active-observation facade exists for the three `kimi-code.acp` post-open rows |
| 3 no new public decision required | **fail** — each of the three families needs an additive adapter-owned public surface |
| 4 deterministic provider-free ledgers | pass in shape — four independent ledgers, no provider contact needed |
| 5 focused validation names at most four packages | pass — two packages |
| 6 one reviewable tranche, no later claim | pass |

F cannot be narrowed around the blocker. The fixed Batch 9.4 boundary requires
the complete package remainder and forbids exception lists, and the three
blocked rows define the acknowledgement, observation, and catalogue bands.

### Ledger And Mixed-Assembly Proof Plan

The implementation card that follows an accepted gate must carry four
independent ledgers of 25 / 20 / 31 / 13 rows generated from the reviewed CSV,
with observed tuple mapping maintained separately from disposition. Beyond the
patterns cards 031 and 032 proved, it must additionally prove: reasoning and
Plan acknowledgement as independently sourced halves of one compound row;
omitted-reasoning, omitted-Plan, and both-omitted opens; model observation
present on open, load, and resume but published only where the census admits
it; catalogue observation never borrowed from `kimi-code.local-server`'s own
catalogue family; and the operation-shape-scoped persistence split above.
Cross-instance, stale-revision, cross-route, cross-operation, and each exact
access drift must fail closed in both directions.

### Smallest Kimi-Only Gate

One Batch 9.1-class public-baseline gate scoped to `kimi-code.acp` alone. It
is not compiled here. It needs these operator decisions, none of which this
card answers:

1. Whether `swallowtail-adapter-kimi` retains exact provider-effective and
   exact rejected reasoning and Plan values adapter-locally, as Claude Agent
   and Cline do, or whether `EffectiveReasoningSetup`'s missing rejected state
   becomes a `swallowtail-runtime` public decision.
2. What exact bounded reasoning value domain `kimi-code.acp` may publish,
   given `KimiAcpBehavior::DeclaredEffort` admits foreign catalogue rows — and
   consequently whether a foreign confirmation is an ordinary `Runtime`
   failure or a publishable rejection.
3. Whether exact provider-effective effort replaces the current
   requested-`"on"` normalization on a projected path while the preserved
   `open_session` keeps its present behavior and failure codes.
4. Whether one additive adapter-owned `open_session_with_projection` seam on
   `KimiPreparedSession` is the whole interactive surface, preserving
   `open_session`, `load_session`, and `resume_session` unchanged, given the
   census carries exactly one interactive-session observation row per family.
5. Whether post-open `control.provider-session-catalogue` gets a second
   additive adapter-owned projection seam over
   `KimiPreparedSessionCatalogue::list_sessions`, or stays withheld at
   construction as unobserved.

Decisions 2, 3, and 5 have no precedent in the Claude Agent or Cline gates.
Neither of those gates grants Kimi authority.

### Gate Outcome

The operator answered all five decisions, and the
[compiled gate](../../../triage/2026-09-01-contract-061-kimi-active-observation-public-baseline-gate.md)
stopped on the fifth: no shared runtime source kind, lifecycle band, or view
represents a completed provider-session catalogue query. Candidate F is still
not promoted and [card 034](034-contract-061-kimi-package-completion.md) is
`blocked`. The gate's re-derivation from current source does replace this
card's provisional 86/3 reading with four exact ledgers totalling 74 emitted,
14 withheld, and 1 undecided row.

### Validation

`effigy qa:docs`, `effigy qa:northstar`, and `git diff --check` pass. No Rust
validation, live probe, or provider contact belongs to this card.

## Evidence

- [completed card 032](032-contract-061-cline-command-code-copilot-goose-package-completion.md)
- [Batch 9.4 package expansion](../../../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [Contract 061](../../../contracts/061-consumer-route-feature-and-control-projection.md)
- [reviewed census](../../../triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [stopped Kimi active-observation gate](../../../triage/2026-09-01-contract-061-kimi-active-observation-public-baseline-gate.md)
- [blocked card 034](034-contract-061-kimi-package-completion.md)
- [card 030 acknowledgement reassessment](030-contract-061-acknowledgement-candidate-reassessment.md)
- [card 031 closeout log](../../../logs/2026-09-01-g05-009-card-031-closeout-and-lifecycle-reassessment.md)
