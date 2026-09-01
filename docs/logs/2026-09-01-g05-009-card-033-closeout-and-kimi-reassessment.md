# 2026-09-01 g05.009 Card 033 Closeout And Kimi Reassessment

Status: complete; evidence stop; candidate F not promoted; no Rust change
Owner: Tom
Date: 2026-09-01
Contracts: 037, 047, 057, 061

## Outcome

Card 033 is complete as a planning-only evidence stop. Candidate F is not
promoted, no implementation card is compiled, and no Rust, manifest, release
baseline, contract, architecture, or census file changed. No provider was
contacted and no live probe ran.

## Card 032 Binding

PR 144 merged exact reviewed head
`e50e3f4619451e066df3b2b6b37d045be28e370e` through
`18a6907e75e55a6b181632a1da35a2fefd0824fe`. The squash merge preserves the
reviewed head exactly: both commits resolve to tree `5c4774f2` and `git diff`
between them is empty. The five ledgers were re-derived from the merged source
rather than from card prose:

| Route | Census | Emitted | Withheld |
| --- | ---: | ---: | ---: |
| `cline.acp` | 11 | 9 | 2 |
| `cline.headless` | 8 | 7 | 1 |
| `command-code.headless` | 11 | 10 | 1 |
| `copilot-cli.acp` | 9 | 6 | 3 |
| `goose.acp` | 9 | 6 | 3 |
| **Total** | **48** | **38** | **10** |

Only the four candidate G adapter semantic API baselines and the package index
changed under `release-baselines/public-api-unreleased/`.

## Census State

Repartitioning the CSV independently of the Batch 9.4 tables assigns all 48
route IDs exactly once with no duplicate and no omission. Cards 022-024, 031,
and 032 prove 249 of 767 rows across 16 routes: 51 + 59 + 38 + 53 + 48. The
exact remainder is 518 rows across 32 route IDs and 22 adapter packages in
candidates B 76, C 94, E 56, F 89, I 47, J 35, K 52, and L 69. Batch 9.5
remains uncompiled.

## Candidate F Reassessment

F owns 89 distinct tuples across two complete package remainders: 25
`kimi-code.acp`, 20 `kimi-code.headless`, 31 `kimi-code.local-server`, and 13
`kimi-platform.chat`. It carries no no-control audit row and no per-turn row.
Its lifecycle partition is 60 selection-summary, 22 session-start-only, and 7
post-open. Four post-open rows are per-route `feature.activity-observation`
and stay descriptor-only under the proved pattern. 86 of 89 rows therefore
rest on prepared facades that already exist. Exactly three rows on
`kimi-code.acp` decide the candidate.

Card 030 called all three unproved. The current-main trace separates them:

| Family | Retained on current `main` | Exact blocker |
| --- | --- | --- |
| `feature.active-session-reasoning-and-plan-ack` | nothing | both confirmations discarded; the Plan half has a frozen domain, the reasoning half does not |
| `feature.negotiated-model-options-observation` | **yes** — exact current value and bounded advertised list on `KimiSessionHandle::negotiated_model_options()` | publication only; no adapter-owned route-qualified active source |
| post-open `control.provider-session-catalogue` | nothing after the call | the observation lives on a separate prepared operation, not on an open session |

`driver.rs` writes `let _ = selection.confirm(&confirmation, …)?` and
`mode::confirm_plan_mode(&confirmation)?` inside `open_session` and keeps
neither. `driver/mode.rs` freezes the Plan domain to exactly
`["default", "plan", "auto", "yolo"]`, so its rejected value is bounded — the
exact `cline.acp` analogue. `driver/reasoning.rs` does not: under
`KimiAcpBehavior::DeclaredEffort` it deliberately admits foreign catalogue
rows, so a confirmation may carry an identifier outside
`{off, on, low, medium, high, xhigh, max}` and no bounded rejected reasoning
domain exists. That branch also normalizes any non-`"off"` current value to
`"on"` when `"on"` was requested, so Kimi loses exact *effective* effort too —
a loss candidates D and G never had. `EffectiveReasoningSetup` cannot absorb
either: its only constructor rejects `requested != effective`.

Rubric items 1, 4, 5, and 6 pass. Items 2 and 3 fail. F cannot be narrowed
around the blocker; the fixed Batch 9.4 boundary requires the complete package
remainder.

## Withholding Correction

`kimi-code.acp` carries two provider state policies by operation shape:
`SessionProviderStatePolicy::Prohibited` on the interactive-session plan and
`DurableProviderSessionPreserved` on the `ProviderSessionCatalogue` and
`ProviderSessionImport` requirements. Card 032's route-scoped persistence
withholding must become operation-shape-scoped here.
`feature.model-catalogue` is withheld on `kimi-code.acp` and
`kimi-code.headless`, which carry no `DriverRole::ModelCatalog`;
`kimi-code.local-server` and `kimi-platform.chat` do carry it, so their
catalogue rows stay a maximal-ledger determination for the implementation
card.

## Current State

- g05.009 is `strict-paused`
- no card is ready
- the sole Next Task is the `kimi-code.acp` active-observation operator
  decision
- acceptance would authorize a later exact API gate and one candidate F
  implementation card, not implementation by itself
- candidates B, C, E, and I-L are unchanged and hold no execution authority
- Batch 9.5 remains uncompiled
- watcher, skill, currentness, papercut, and provider lanes were not touched

## Next Move

Decide one `kimi-code.acp`-only public baseline. Five operator decisions, none
answered by card 033:

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

## Authority

- [completed card 033](../roadmaps/g05/batch-cards/033-contract-061-card-032-closeout-and-kimi-reassessment.md)
- [completed card 032](../roadmaps/g05/batch-cards/032-contract-061-cline-command-code-copilot-goose-package-completion.md)
- [g05.009](../roadmaps/g05/009-contract-061-consumer-projection-realization.md)
- [Batch 9.4 checkpoint](../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [card 030 acknowledgement reassessment](../roadmaps/g05/batch-cards/030-contract-061-acknowledgement-candidate-reassessment.md)
- [Cline active-observation gate](../triage/2026-09-01-contract-061-cline-active-observation-public-baseline-gate.md)
- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
- [reviewed census](../triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv)
