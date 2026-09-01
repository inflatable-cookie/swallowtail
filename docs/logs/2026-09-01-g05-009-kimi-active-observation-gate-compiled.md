# 2026-09-01 g05.009 Kimi Active-Observation Gate Compiled

Status: complete; card 034 ready
Owner: Tom
Date: 2026-09-01
Contracts: 037, 047, 057, 061

## Result

The operator answered all five `kimi-code.acp` decisions card 033 returned.
`EffectiveReasoningSetup` is unchanged. Exact reasoning and Plan values stay
inside `swallowtail-adapter-kimi` behind one bounded adapter-local
`KimiProviderValue` with its own private 128-byte admission. One additive
`KimiPreparedSession::open_session_with_projection` covers the whole
interactive surface, and one separate
`KimiPreparedSessionCatalogue::list_sessions_with_projection` covers post-open
catalogue observation. `open_session`, `load_session`, `resume_session`,
`list_sessions`, `list_page`, `next_page_request`, and continuation paging keep
their exact behavior.

The gate resolves the two decisions with no Claude Agent or Cline precedent. A
foreign `DeclaredEffort` confirmation is never a publishable rejection; the
preserved path is unchanged. Under requested `"on"` the projected path
publishes the exact provider-confirmed effort while the preserved path keeps
its normalization, so the two cannot drift in failure code or cleanup.

Exact-head review closed five gaps before merge review. Foreign and
unretainable tokens reach the projected path through two disjoint branches, and
the gate now fixes both separately. A concrete request confirmed foreign has
already aborted in `NegotiatedReasoningSetup::confirm`, so no session exists to
close and case 2 returns that preserved `effective_mismatch` unchanged; the new
adapter codes belong only to case 4, reachable solely under `DeclaredEffort`
with requested `"on"`, where the lifecycle succeeded and the projected path
closes the opened session.

`driver.rs` confirms reasoning before Plan, so a maximal request whose
reasoning rejects never observes Plan. Both acknowledgement halves gain
`RequestedNotObserved`, which contributes `pending` state and no domain entry,
so the compound row carries only the observed reasoning rejection without
inventing Plan truth or performing further provider work.

Cross-seam source isolation is applicability, not identifier inequality:
neither seam can observe the other's IDs, so each rejects only its own supplied
pair, and the runtime composer's `snapshot_identity_rejected` is the
cross-operation boundary. `KimiCatalogueProjectionFailure` gains
`SourceIdentity(RuntimeFailure)` because `ConsumerRouteProjectionFailure` has
no public constructor and `ProviderSessionOperationFailure` is the wrong
authority before dispatch. `control.provider-session-catalogue` has exactly one
emitter — a completed `list_sessions_with_projection` — and the prepared
catalogue facade must not emit it in any state.

Candidate F now passes the Batch 9.4 promotion rubric and is ready as card 034.

## Derived Ledgers

The four route ledgers were re-derived from the reviewed census plus current
`main` driver roles, capability profiles, extension namespaces, ownership
modes, and provider-state policies. Card 033's provisional 86/3 reading does
not survive:

| Route | Census | Emitted | Withheld |
| --- | ---: | ---: | ---: |
| `kimi-code.acp` | 25 | 22 | 3 |
| `kimi-code.headless` | 20 | 10 | 10 |
| `kimi-code.local-server` | 31 | 31 | 0 |
| `kimi-platform.chat` | 13 | 12 | 1 |
| **Total** | **89** | **75** | **14** |

Ten of the fourteen withholdings sit on `kimi-code.headless`, whose census
carries a large matrix-only band that no headless capability, role, or public
operation proves. `feature.model-catalogue` is withheld on `kimi-code.acp` and
`kimi-code.headless` but emitted on `kimi-code.local-server` and
`kimi-platform.chat`, where a prepared catalogue facade carries
`DriverRole::ModelCatalog`. `kimi-code.acp`'s single
`feature.persistent-session-posture` row belongs to the
`DurableProviderSessionPreserved` import plan, not the `Prohibited`
interactive-session plan.

## Current State

- g05.009 is `strict-ready`
- card 034 is the sole ready implementation card and sole Next Task
- 249 census rows remain proved; candidate F's 89 rows are not counted until
  implementation review and merge
- 518 rows remain unproved, including candidate F; 429 sit in candidates B, C,
  E, and I-L
- no Rust, manifest, release baseline, contract, architecture, census, or
  provider claim changed
- no provider was contacted and no live probe ran

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Next Move

Implement card 034 as one two-package PR. Stop after its exact 89-row proof for
orchestrator review before reassessing another Batch 9.4 candidate.

## Authority

- [card 034](../roadmaps/g05/batch-cards/034-contract-061-kimi-package-completion.md)
- [public-baseline gate](../triage/2026-09-01-contract-061-kimi-active-observation-public-baseline-gate.md)
- [g05.009](../roadmaps/g05/009-contract-061-consumer-projection-realization.md)
- [Batch 9.4 package expansion](../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
