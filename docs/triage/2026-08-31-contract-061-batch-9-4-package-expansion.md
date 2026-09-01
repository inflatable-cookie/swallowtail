# Contract 061 Batch 9.4 Package Expansion

Status: active planning evidence; candidates A, D, G, and H complete through
cards 023, 024, 031, and 032; 249 rows proved and 518 remaining; candidate F
promoted as ready card 034 behind the accepted `kimi-code.acp`
active-observation gate
Owner: Tom
Date: 2026-08-31

## Trigger

PR 131 merged the exact 36-row `codex.app-server` and 15-row
`openai.realtime` vertical at `fdd2b018`. Its ledgers intentionally claim
nothing for the other 716 rows in the reviewed Contract 061 census. Batch 9.4
must partition that remainder before another implementation tranche starts.

## Fixed Boundary

This checkpoint does not change Contract 061 or the Batch 9.1 public runtime
baseline. Every candidate must:

- own all remaining census rows for each adapter package it names;
- disposition rows by exact route, operation shape, semantic ID, lifecycle,
  value domain, omission, applicability, evidence source, and evidence
  strength;
- emit only truth admitted by exact prepared or active-observation evidence
  and withhold documentation-only or incompatible-operation rows at
  construction;
- retain separate source identities, exact access dimensions,
  consumer-mediated per-turn authority, and exact acknowledgement truth;
- keep explicit no-control route audits as negative coverage;
- use the existing consumer-supplied contribution/composer boundary, with no
  registry, runtime adapter enumeration, callback, downcast, or provider
  payload; and
- stop if `swallowtail-runtime`, `swallowtail-testkit`, `swallowtail-core`, or
  Contracts 037, 047, 057, or 061 need a new public decision.

Provider contact, live probes, currentness work, watcher work, PR 127, PR 130,
and generation closeout remain outside Batch 9.4.

## Exact Remainder

The CSV has 767 rows. Removing only the 36 `codex.app-server` and 15
`openai.realtime` rows leaves exactly 716 rows across all 31 adapter packages
and 46 route IDs. The following partition assigns every remaining row once.
Each candidate contains one to four complete adapter-package remainders and
35 to 94 rows, so normal focused validation can name its adapter packages
explicitly.

| Candidate | Complete adapter-package remainder | Exact route rows | Total | Sensitive truth retained | State |
| --- | --- | --- | ---: | --- | --- |
| A | Codex; OpenAI | `codex.exec` 35; `openai.background` 24 | 59 | prepared-only controls; background recovery descriptors | complete through card 023 / PR 133 |
| B | Alibaba Model Studio; Anthropic; xAI | `alibaba.conversations` 19; `anthropic.managed-agent` 17; `anthropic.messages` 23; `xai.responses-websocket` 17 | 76 | one consumer-mediated per-turn exchange | candidate only |
| C | Antigravity; Bedrock; Cursor | `antigravity.catalogue` 14; `antigravity.headless` 18; `bedrock.catalogue` 9; `bedrock.runtime` 10; `cursor-agent.acp` 13; `cursor-agent.catalogue` 13; `cursor-agent.headless` 17 | 94 | four explicit no-control route audits | candidate only |
| D | Claude Agent | `claude-agent.acp` 30; `claude-code.headless` 12; `claude-code.response-only` 11 | 53 | exact active-session acknowledgement | complete through card 031 / PR 141 |
| E | Gemini; Grok | `gemini-cli.acp` 14; `gemini-cli.headless` 13; `gemini.live` 16; `grok-build.acp` 13 | 56 | ACP, headless, and live applicability remain distinct | candidate only |
| F | Kimi; Kimi Platform | `kimi-code.acp` 25; `kimi-code.headless` 20; `kimi-code.local-server` 31; `kimi-platform.chat` 13 | 89 | exact active-session acknowledgement | candidate only |
| G | Cline; Command Code; Copilot CLI; Goose | `cline.acp` 11; `cline.headless` 8; `command-code.headless` 11; `copilot-cli.acp` 9; `goose.acp` 9 | 48 | exact active-session acknowledgement; two no-control audits | complete through card 032 / PR 144 |
| H | Deep Agents; Kiro; Qoder; Zcode | `deepagents.acp` 9; `kiro.acp` 9; `qoder.headless` 8; `zcode.app-server` 12 | 38 | three explicit no-control route audits | complete through card 024 / PR 138 |
| I | DeepSeek; DeepSeek Harness | `deepseek.continuation` 19; `deepseek-harness.jsonrpc` 11; `deepseek-harness.local-server` 17 | 47 | continuation and local-server lifecycle remain distinct | candidate only |
| J | llama.cpp; Ollama | `llama-cpp.attached` 10; `llama-cpp.owned` 6; `ollama.attached` 19 | 35 | attached and owned applicability remain distinct | candidate only |
| K | Mistral Vibe; Muse; Oh My Pi; Qwen | `mistral-vibe.headless` 8; `muse-code.headless` 10; `oh-my-pi.rpc` 18; `qwen.headless` 16 | 52 | one consumer-mediated per-turn attachment row | candidate only |
| L | OpenCode; Pi | `opencode.http` 35; `pi.rpc` 15; `pi.sdk-sidecar` 19 | 69 | six per-turn rows, including observed callbacks | candidate only |
| **Total** | **31 packages** | **46 route IDs** | **716** | **all remaining census truth** | |

The nine no-control audits are assigned exactly once: four in C
(`antigravity.catalogue`, `bedrock.catalogue`, `cursor-agent.catalogue`, and
`cursor-agent.acp`), two in G (`copilot-cli.acp` and `goose.acp`), and three in
H (`deepagents.acp`, `kiro.acp`, and `qoder.headless`). The three remaining
exact active-session acknowledgement rows stay isolated in D, F, and G. The
eight remaining per-turn rows stay in B, K, and L; none may be relabelled as
session-start authority.

After candidates A, D, G, and H, 518 rows across 32 route IDs and 22 adapter
packages remain unproved in candidates B, C, E, F, and I-L.

## Promotion Rubric

A candidate becomes a numbered ready card only when all of the following are
closed for its whole package set:

1. the exact census row set and explicit no-control rows reconcile to the
   candidate total without an exception or filter list;
2. every contributing prepared or active-observation façade and its source
   identity kind are named, and documentation-only rows have an explicit
   construction-time withholding rule;
3. the work needs no new runtime/core public type, fixed maximum, composer
   failure, registry, enumeration, callback, provider payload, or contract
   amendment;
4. deterministic adapter-local ledgers prove exact emitted and withheld sets,
   source/applicability disagreement, lifecycle and authority distinctions,
   and negative coverage without contacting a provider;
5. focused validation names no more than four exact adapter packages, with
   semantic API, docs, Northstar, god-file, and diff checks added when the card
   changes or relies on those surfaces; and
6. the card stops after one reviewable package tranche and does not claim later
   candidates or the 767-row all-route audit.

Before reassessment, candidates B-L were planning rows without execution
authority. Promote one only after its adapter-local façade and ledger audit
passes this rubric against the then-current main branch.

## Candidate A Completion

Candidate A passed the rubric and was promoted as card 023:

- its exact set is the 35 `codex.exec` plus 24 `openai.background` rows;
- `CodexPreparedExec` and `OpenAiPreparedBackgroundRun` are the existing
  `PreparedOperationEvidence`-backed façades that may emit exact
  operation-owned truth through the already established
  `consumer_route_projection_contribution(source_id)` shape and
  `AdapterContribution` source kind;
- catalogue, route-wide, post-open, lifecycle, or other rows not proved by
  those exact prepared values are withheld at construction rather than
  widened from matrices or another route;
- the separate OpenAI Models and background-reconciliation prepared families
  are not treated as `openai.background` structured-run evidence and gain no
  new composer or observation seam;
- no new runtime/testkit/core public surface, open-result type, observer,
  acknowledgement path, or provider operation is required; and
- two exact adapter packages plus their deterministic 59-row ledgers fit one
  bounded provider-free validation tranche.

PR 133 merged exact reviewed head `fbb4b118` through `58be7122`. The proof
keeps the 35-row and 24-row ledgers independent, rejects mixed access evidence,
keeps prepared activity descriptors descriptor-only, and leaves unsupported
rows withheld at construction. No shared public or contract surface changed.

The `codex.exec` output-token row is now corrected as descriptor-only negative
coverage: the generic runtime request can carry a positive value, the prepared
facade cannot construct it, and route validation rejects every present value.
Card 023's construction-time withholding remains exact.

## Post-Card-023 Reassessment

The current-main audit selected candidate H and no other candidate. Its four
contributing facades are `DeepAgentsPreparedSession`, `KiroPreparedSession`,
`QoderHeadlessPreparedRun`, and `ZcodePreparedRun`. Each already retains exact
prepared-operation evidence and activity posture. ZCode additionally retains
the exact model route and app-server mode needed for its two route-specific
controls. All contributions can use the established `AdapterContribution`
source kind and caller-supplied source ID.

Candidate H passes the promotion rubric:

- its 38 rows reconcile exactly as 9 `deepagents.acp`, 9 `kiro.acp`, 8
  `qoder.headless`, and 12 `zcode.app-server` rows;
- the first three routes have explicit no-public-route-specific-control audit
  rows, not hidden composer controls;
- catalogue-only and audit rows can be withheld at construction, while
  activity remains descriptor-only;
- no active-observation, acknowledgement, per-turn mutation, provider contact,
  new runtime/core/testkit public type, or contract change is needed; and
- four exact adapter packages fit the normal focused-validation maximum.

The other candidates remain unpromoted:

| Candidate | Current-main audit disposition |
| --- | --- |
| B | Hold. Its consumer-mediated per-turn exchange needs a dedicated authority and lifecycle proof. |
| C | Viable later, but its 94 rows and seven route shapes are a larger negative-coverage tranche than H. |
| D | Hold. Exact active-session acknowledgement needs separate active-observation source and state proof. |
| E | Viable later, but ACP, headless, and live applicability need a larger three-family proof. |
| F | Hold. Exact active-session acknowledgement and four route shapes remain coupled. |
| G | Hold. Exact acknowledgement, two no-control audits, ACP, and headless evidence remain coupled. |
| H | Promote as card 024. The complete four-package 38-row proof is closed. |
| I | Viable later, but continuation, JSON-RPC, and local-server lifecycle need a distinct multi-facade audit. |
| J | Viable later. It has fewer rows than H but more route-specific controls and attached/owned prepared families. |
| K | Hold. The consumer-mediated per-turn attachment row needs its own authority proof. |
| L | Hold. Six per-turn rows and observed callbacks need a dedicated lifecycle and callback boundary audit. |

Candidates B-G and I-L remain planning rows without card numbers or execution
authority. Card 024 was the sole ready implementation tranche at that
checkpoint; its review had to close before another candidate could be
promoted.

## Candidate H Completion

PR 138 merged exact reviewed head `c796ad7f` through `8b295e6b`. Four
independent ledgers prove all 9 `deepagents.acp`, 9 `kiro.acp`, 8
`qoder.headless`, and 12 `zcode.app-server` rows. The three no-control audits
remain negative coverage; activity remains descriptor-only; ZCode model and
mode controls come from exact prepared bindings. Matching-source cross-route
and cross-access mixtures fail closed. No shared public type, contract,
provider contact, or live probe was added.

Card 030 was the post-card-024 planning checkpoint. It audited D, F, and G
against current `main` and promoted none of them.

## Post-Card-024 Acknowledgement Reassessment

Card 030 audited candidates D, F, and G against current `main` and promoted
none. Every total reconciles exactly with no filter or exception list, and each
candidate owns the complete census remainder of its adapter packages:

| Candidate | Adapter packages | Exact route rows | Total |
| --- | --- | --- | ---: |
| D | `swallowtail-adapter-claude-agent` | `claude-agent.acp` 30; `claude-code.headless` 12; `claude-code.response-only` 11 | 53 |
| F | `swallowtail-adapter-kimi`; `swallowtail-adapter-kimi-platform` | `kimi-code.acp` 25; `kimi-code.headless` 20; `kimi-code.local-server` 31; `kimi-platform.chat` 13 | 89 |
| G | `swallowtail-adapter-cline`; `swallowtail-adapter-command-code`; `swallowtail-adapter-copilot-cli`; `swallowtail-adapter-goose` | `cline.acp` 11; `cline.headless` 8; `command-code.headless` 11; `copilot-cli.acp` 9; `goose.acp` 9 | 48 |

### Shared Blocker

Rubric item 2 fails identically on all three. Each of the three exact
active-session acknowledgement routes validates its provider confirmation on
current `main` and then discards it. No adapter retains an acknowledgement
value, none can carry an exact rejected value, and none has an
active-observation facade to name:

| Route | Acknowledgement site | Retained on current `main` |
| --- | --- | --- |
| `claude-agent.acp` | `driver/config.rs` `confirm_reasoning` via `confirm_value` | nothing; `Result<(), RuntimeFailure>` dropped in `driver/access.rs`; mismatch is a static failure with no rejected value |
| `kimi-code.acp` | `driver/reasoning.rs` `KimiReasoningSelection::confirm`; `driver/mode.rs` `confirm_plan_mode` | nothing; `driver.rs` writes `let _ = selection.confirm(...)?`; plan half returns `Result<(), _>` |
| `cline.acp` | `driver/mode.rs` `confirm_plan_mode` | nothing; `Result<(), _>` dropped in `driver.rs` |

`EffectiveReasoningSetup` cannot help. It encodes only requested == effective;
a mismatch becomes `swallowtail.negotiated_reasoning.effective_mismatch`, so
the census `rejected` state is unrepresentable through it without a new runtime
public type, which the fixed boundary forbids.

`openai.realtime` is the exact contrast, not a precedent that transfers. Card
022 could prove it only because that card added `RealtimeAcknowledgement` and
`RealtimeOpenRejection::rejected_effort` plus the additive
`open_session_with_projection` seam, under the operator decision recorded in
the Batch 9.1 public-baseline gate on 2026-08-31. That gate closed the
route-local acknowledgement surface for `openai.realtime` alone. No gate closes
it for `claude-agent.acp`, `kimi-code.acp`, or `cline.acp`.

### Current-Main Dispositions

| Candidate | Disposition |
| --- | --- |
| D | Promote after gate acceptance. One adapter package, one acknowledgement row, no other non-descriptor post-open row. The prepared facades already exist; the accepted route-local baseline adds only exact acknowledgement retention and the additive open result. Card 031 owns the complete 53-row proof. |
| F | Promoted as card 034 after the accepted `kimi-code.acp` gate. Previously stopped as the largest and most coupled candidate. 89 rows, two packages, four route shapes, a compound reasoning-and-plan acknowledgement, plus two further unproved post-open families on `kimi-code.acp`: `feature.negotiated-model-options-observation` and post-open `control.provider-session-catalogue`. Three observation seams, not one. |
| G | Stop. 48 rows but all four adapter packages, so rubric item 5 has zero headroom. Its two `audit.no-public-route-specific-selectable-control` rows on `copilot-cli.acp` and `goose.acp` are fine — card 024 proved that negative-coverage pattern. `cline.acp` adds one further post-open family, `feature.negotiated-model-options-observation`. |

No candidate can be narrowed around the blocker. The fixed boundary requires
the complete package remainder and forbids exception lists, and withholding the
one acknowledgement row that defines the band would weaken a blocker to force a
selection.

### Next Planning Move

One Batch 9.1-class public-baseline gate, not an implementation card, scoped to
`claude-agent.acp` alone. That is candidate D's only acknowledgement route and,
by this audit's own evidence, the smallest useful unblock: one adapter package,
one acknowledgement row, and no second post-open observation family. The gate
must close, with an operator decision:

- adapter-local retention of the exact provider-effective and rejected values
  on `claude-agent.acp`; and
- one additive adapter-owned open-with-projection outcome and failure that
  preserves the existing `ClaudeAgentPreparedSession::open_session` signature
  and behavior.

`kimi-code.acp` and `cline.acp` are deliberately excluded. Their route-local
gates come later and separately, and they carry questions this gate must not
answer: whether `EffectiveReasoningSetup`'s missing rejected state stays an
adapter-local concern or becomes a runtime public decision, and whether
`feature.negotiated-model-options-observation` and post-open
`control.provider-session-catalogue` need their own observation seams before F
or G can be promoted. Coupling all three routes into one gate would cut across
the one-candidate-at-a-time runway and delay the smallest unblock.

Candidates B, C, E-G, and I-L still hold no card number or execution
authority. The per-turn band B/K/L and the breadth band C/E/I/J are unchanged
by card 030.

## Claude Agent Gate Acceptance And Candidate D Promotion

The operator accepted the two route-local decisions Card 030 named:

- retain exact provider-effective and rejected reasoning values inside
  `swallowtail-adapter-claude-agent`; and
- add one adapter-owned open-with-projection outcome/failure while preserving
  `ClaudeAgentPreparedSession::open_session`.

The
[Claude Agent public-baseline gate](2026-08-31-contract-061-claude-agent-acknowledgement-public-baseline-gate.md)
fixes the exact public signatures, source split, state transitions, unknown
failure boundary, shared-open lifecycle, and provider-free review oracle. It
adds no runtime/core public decision and grants no authority to Kimi or Cline.

At that gate, candidate D passed the promotion rubric and became card 031.
Its one adapter package owns the exact 30 `claude-agent.acp`, 12
`claude-code.headless`, and 11 `claude-code.response-only` tuples. The card
combines the adapter-local public-baseline repair with the full 53-row package
proof. The following completion records the reviewed result.

## Candidate D Completion

PR 141 merged exact reviewed head `1edc7e73` through `5d1f173a`. Three
independent ledgers prove all 30 `claude-agent.acp`, 12
`claude-code.headless`, and 11 `claude-code.response-only` tuples: 29/1,
11/1, and 9/2 emitted/withheld. Exact ACP reasoning acknowledgement uses the
accepted adapter-local open-with-projection surface. Prepared and active
sources remain distinct, malformed or unqualified confirmation publishes no
state, and the preserved and additive open methods share one private
lifecycle.

Only the Claude Agent adapter semantic API baseline changed, with 27 additive
lines. Focused validation passed 188 tests, package and semantic API checks
passed, the god-file scan improved from 391 to 387, and all five CI jobs were
green. No provider contact or live probe occurred.

The proved total is now 201: 51 from card 022, 59 from card 023, 38 from card
024, and 53 from card 031. The exact remainder is 566 rows across 37 route IDs
and 24 adapter packages.

## Post-Card-031 Lifecycle Reassessment

The lifecycle-priority sequence returns to F and G on `main` at `153e3e43`.

F remains the larger coupled stop. Its 89 rows cover two packages and four
route shapes. `kimi-code.acp` still needs three distinct post-open families:
compound reasoning-and-plan acknowledgement, negotiated model options, and
provider-session catalogue observation. Card 031 changed only the Claude
Agent adapter and grants Kimi no public-baseline authority.

G is the narrower route-local tranche. Its 48 rows cover four complete
adapter-package remainders. The Copilot CLI and Goose no-control audits already
have a proved negative-coverage pattern. The operator has now approved Cline
adapter-local retention of exact effective/rejected Plan acknowledgement plus
exact bounded negotiated model options, exposed through one additive
adapter-owned projected-open seam while preserving
`ClinePreparedSession::open_session`.

The
[Cline active-observation gate](2026-09-01-contract-061-cline-active-observation-public-baseline-gate.md)
fixes the exact adapter-owned signatures, typed state, prepared and active
source identities, lifecycle, failure preservation, construction-time
withholding, and provider-free oracles. Candidate G now passes all six
promotion criteria and is ready as card 032. This route-local decision grants
no authority to Kimi or another candidate.

Candidates B, C, E, I-L stay in their existing later bands. Batch 9.5 remains
uncompiled.

## Candidate G Completion

PR 144 merged exact reviewed head `e50e3f46` through `18a6907e`. The squash
merge preserves the reviewed head exactly: both commits resolve to tree
`5c4774f2`. Five independent ledgers in the merged source prove 11
`cline.acp`, 8 `cline.headless`, 11 `command-code.headless`, 9
`copilot-cli.acp`, and 9 `goose.acp` rows with 9/2, 7/1, 10/1, 6/3, and 6/3
emitted/withheld, totalling 48 rows and 38 emitted. Cline's preserved and
projected opens share one private lifecycle and one Plan failure code. Only
the four candidate G adapter semantic API baselines and the package index
changed. No provider contact or live probe occurred.

The proved total is now 249: 51 from card 022, 59 from card 023, 38 from card
024, 53 from card 031, and 48 from card 032. The exact remainder is 518 rows
across 32 route IDs and 22 adapter packages.

## Post-Card-032 Candidate F Reassessment

Card 033 audited candidate F against current `main` and did not promote it.
The census repartitions independently of the tables above: all 48 route IDs
are assigned exactly once, proved 249 (16 routes) plus remaining 518 (32
routes) equals 767, and the remaining candidates decompose as B 76, C 94, E
56, F 89, I 47, J 35, K 52, L 69.

F owns 89 distinct tuples across two complete package remainders: 25
`kimi-code.acp`, 20 `kimi-code.headless`, 31 `kimi-code.local-server`, and 13
`kimi-platform.chat`. It holds no no-control audit row and no per-turn row.
Its lifecycle partition is 60 selection-summary, 22 session-start-only, and 7
post-open-observation-only rows. Four post-open rows are per-route
`feature.activity-observation` and stay descriptor-only under the proved
pattern. The other three sit on `kimi-code.acp` alone and block the candidate.

### Current-Main Truth For The Three Coupled Families

Card 030 called all three unproved. The current-main trace separates them:

| Family | Retained on current `main` | Exact blocker |
| --- | --- | --- |
| `feature.active-session-reasoning-and-plan-ack` | nothing | both confirmations discarded; the Plan half has a frozen domain, the reasoning half does not |
| `feature.negotiated-model-options-observation` | **yes** — exact current value and bounded advertised list on `KimiSessionHandle::negotiated_model_options()` | publication only; no adapter-owned route-qualified active source |
| post-open `control.provider-session-catalogue` | nothing after the call | observation lives on a separate prepared operation, not on an open session |

Acknowledgement detail. `driver.rs` writes
`let _ = selection.confirm(&confirmation, selected.behavior())?` and
`mode::confirm_plan_mode(&confirmation)?` inside `open_session` and keeps
neither. `driver/mode.rs` freezes the Plan domain to exactly
`["default", "plan", "auto", "yolo"]`, so its rejected value is bounded and
present at the site — the exact `cline.acp` analogue. `driver/reasoning.rs`
does not: `validate_behavior_shape` deliberately admits foreign catalogue rows
under `KimiAcpBehavior::DeclaredEffort`, so a confirmation may carry an
identifier outside `{off, on, low, medium, high, xhigh, max}` and no bounded
rejected reasoning domain exists to publish. That branch also normalizes any
non-`"off"` current value to `"on"` when `"on"` was requested, so Kimi
additionally loses exact *effective* effort — a loss candidates D and G never
had. `EffectiveReasoningSetup` cannot absorb either loss: its only constructor
rejects `requested != effective` with
`swallowtail.negotiated_reasoning.effective_mismatch`.

Model-option detail. `driver/validation.rs` `parse_model_options` extracts one
current value plus the bounded advertised list with optional display names,
and `driver.rs` threads it into the handle on open, load, and resume. Kimi
already holds what the Cline gate authorized adding; only the publishing seam
is missing.

Catalogue detail. `driver/session_catalogue.rs` implements
`ProviderSessionCatalogueDriver` for `KimiAcpDriver`, and
`KimiPreparedSessionCatalogue::list_sessions` returns a
`ProviderSessionCatalogueOutcome` exposing only `candidates()`,
`next_cursor()`, and `cleanup()`. The adapter retains nothing and the outcome
carries no contribution. This is a third seam *shape*, not a third instance of
the projected-open shape. Emitting the row from
`PreparedProviderSessionCatalogueEvidence` would publish the census `observed`
half from prepared success, which Contract 061's absent-mutation-authority
point rejects.

### Rubric Result

Items 1, 4, 5, and 6 pass: 89 distinct tuples with no exception list, four
provider-free ledgers, two packages against a four-package maximum, and one
reviewable tranche. Items 2 and 3 fail — no active-observation facade exists
for the three `kimi-code.acp` post-open rows, and each family needs an
additive adapter-owned public surface. F cannot be narrowed around the
blocker; the fixed boundary requires the complete package remainder.

### Withholding Correction Carried Forward

`kimi-code.acp` carries two provider state policies by operation shape:
`prepared_profile/plan.rs` sets `SessionProviderStatePolicy::Prohibited` on
the interactive-session plan while
`prepared_profile/provider_session_catalogue.rs` sets
`DurableProviderSessionPreserved` on the `ProviderSessionCatalogue` and
`ProviderSessionImport` requirements. Card 032's route-scoped persistence
withholding must become operation-shape-scoped here, so the `session-lifecycle`
`feature.persistent-session-posture` row belongs to the durable family.
`feature.model-catalogue` is withheld on `kimi-code.acp` and
`kimi-code.headless`, which carry no `DriverRole::ModelCatalog`;
`kimi-code.local-server` and `kimi-platform.chat` do carry it, so their
catalogue rows stay a maximal-ledger determination for the implementation
card.

### Next Planning Move

One Batch 9.1-class public-baseline gate scoped to `kimi-code.acp` alone, not
an implementation card. Card 033 names it and does not compile it. Its
operator decisions are:

1. adapter-local retention of exact provider-effective and exact rejected
   reasoning and Plan values, or a `swallowtail-runtime` decision on
   `EffectiveReasoningSetup`'s missing rejected state;
2. the exact bounded reasoning value domain publishable under
   `DeclaredEffort`'s open catalogue, and what a foreign confirmation does;
3. whether exact provider-effective effort replaces the requested-`"on"`
   normalization on a projected path while the preserved `open_session` keeps
   its behavior and failure codes;
4. whether one additive `KimiPreparedSession::open_session_with_projection`
   seam is the whole interactive surface, leaving `open_session`,
   `load_session`, and `resume_session` unchanged; and
5. whether post-open `control.provider-session-catalogue` gets a second
   additive adapter-owned seam over
   `KimiPreparedSessionCatalogue::list_sessions`, or stays withheld as
   unobserved.

Decisions 2, 3, and 5 have no precedent in the Claude Agent or Cline gates.
Neither gate grants Kimi authority.

### Accepted Kimi Gate

The operator answered all five. The
[Kimi active-observation public-baseline gate](2026-09-01-contract-061-kimi-active-observation-public-baseline-gate.md)
keeps `EffectiveReasoningSetup` unchanged, retains exact reasoning and Plan
values adapter-locally behind one bounded 128-byte `KimiProviderValue`, adds
only `KimiPreparedSession::open_session_with_projection` plus a separate
`KimiPreparedSessionCatalogue::list_sessions_with_projection`, and keeps
prepared and active source IDs distinct. Under `DeclaredEffort` with requested
`"on"` the projected path publishes the exact provider-confirmed effort;
foreign or over-bound confirmations are ordinary `Runtime` failures that
publish no row while the preserved path stays unchanged.

The gate re-derives candidate F's four route ledgers from current source
rather than inheriting card 033's provisional 86/3 reading:

| Route | Census | Emitted | Withheld |
| --- | ---: | ---: | ---: |
| `kimi-code.acp` | 25 | 22 | 3 |
| `kimi-code.headless` | 20 | 10 | 10 |
| `kimi-code.local-server` | 31 | 31 | 0 |
| `kimi-platform.chat` | 13 | 12 | 1 |
| **Total** | **89** | **75** | **14** |

Candidate F therefore passes rubric items 2 and 3 and is ready as card 034.
Its 89 rows are not counted as proved before that card merges.

Candidates B, C, E, and I-L keep no card number or execution authority. The
per-turn band B/K/L and the breadth band C/E/I/J are unchanged by card 033 and
the Kimi gate. Batch 9.5 remains uncompiled.

## Lifecycle-Priority Sequence

The four-track reframe keeps lifecycle value ahead of row count. After card
031 completion, reassess the remaining candidates in these bands:

1. D, F, and G: exact active-session acknowledgements and post-open state;
   D is complete through card 031, G is complete through card 032, and F is
   ready as card 034 behind the accepted `kimi-code.acp` gate.
2. B, K, and L: turn-start and mid-turn consumer-mediated truth, including
   observed callbacks.
3. C, E, I, and J: remaining breadth and negative applicability coverage.

Promote only one candidate at a time against current `main`. A lifecycle-rich
candidate still must pass the existing rubric; this order grants no execution
authority and does not compile Batch 9.5 early.

## All-Route Gate

Batch 9.5 remains uncompiled. It may become ready only after candidates A-L
are completed and one audit can reconcile 767 rows, 48 route IDs, 31 adapter
packages, nine no-control routes, nine per-turn rows, and five exact
acknowledgement rows without weakening any Contract 061 boundary.

## Sources

- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
- [g05.009](../roadmaps/g05/009-contract-061-consumer-projection-realization.md)
- [Batch 9.1 public baseline gate](2026-08-31-contract-061-batch-9-1-public-baseline-gate.md)
- [reviewed census](2026-08-30-consumer-route-feature-and-option-projection-census.csv)
