# 068 Contract 061 llama.cpp And Ollama Package Completion

Status: ready
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: completed card 067; the candidate J audit note; Contract 061; completed cards 022-024

## Goal

Complete candidate J's Contract 061 census dispositions: all 10
`llama-cpp.attached`, 6 `llama-cpp.owned`, and 19 `ollama.attached` rows,
as the exact 32 emitted / 3 withheld ledger fixed by the
[candidate J audit](../../../triage/20260904-134914-contract-061-candidate-j-audit.md).

## Scope

1. Add the established `consumer_route_projection_contribution(source_id)`
   shape to `LlamaCppPreparedCatalogue`, `LlamaCppPreparedInferenceAttempt`,
   `LlamaCppPreparedServingStart`, `OllamaPreparedInventory`,
   `OllamaPreparedInferenceAttempt`, and `OllamaPreparedSession`, following
   the `codex.exec`, `zcode.app-server`, and Cline precedents.
2. Use only each facade's exact `PreparedOperationEvidence`, activity
   profile, request input, and adapter-local prepared evidence. Preserve the
   caller-supplied source identity.
3. Disposition exactly 35 census tuples with the audit's per-row anchors:
   9 E / 1 W on `llama-cpp.attached`, 5 E / 1 W on `llama-cpp.owned`,
   18 E / 1 W on `ollama.attached`.
4. Withhold at construction, never through an emitted-row filter:
   `llama-cpp.attached` `feature.cancellation-or-interruption`
   (documentation-only; no attached plan requires `Interruption`);
   `llama-cpp.owned` `feature.activity-observation` (absence proved; owned
   evidence retains no activity profile); `ollama.attached`
   interactive-session `control.reasoning-selection` (documentation-only;
   the session input has no reasoning field).
5. Key the Ollama ledger by `(operation shape, semantic id)`. The route
   carries twin `model-selection`, `context-window`, and
   `reasoning-selection` rows across `structured-run` and
   `interactive-session`; emit each from the exact shape's facade with exact
   applicability. Emit `feature.cancellation-or-interruption` on the
   interactive-session shape only.
6. Emit optional-request rows (`feature.reasoning-selection`,
   `feature.structured-output`, `control.reasoning-selection`,
   `control.structured-output`) only from the exact maximal attempt profile
   after route validation admits them, mirroring the `codex.exec`
   conditional reasoning row.
7. Publish the route-local serving, context-window/size, and
   structured-output controls as bounded namespaced extensions with the
   adapter-local domains the audit records (`LlamaCppContextSize`
   `1..=i32::MAX`, `LlamaCppReasoningSelection` exactly `off`,
   `OllamaContextWindow` `4..=i32::MAX`) and the census omission semantics.
   Portable `MaximumOutputTokens` and exact-model-route controls use the
   existing portable identities.
8. Skip the `llama-cpp.owned` `StreamingEvents` and `Interruption` plan
   requirements: neither has a census row; emitting either invents a row.
9. Keep attached and owned applicability distinct; the existing cross-route
   mixture rejection applies. Keep activity descriptor-only.
10. Add one deterministic adapter-local ledger per route, asserting every
    exact tuple once with an emitted or withheld reason and no exception
    list, plus per-shape Ollama fixtures.
11. Keep the runtime/testkit/core public baseline and Contracts 037, 047,
    057, and 061 unchanged. Stop after one reviewable two-package PR.

## Out Of Scope

- any census row outside candidate J; candidates B, C, E, F, I, K, L
- a catalogue contribution seam, active-observation seam, acknowledgement
  result, per-turn mutation path, callback, or provider payload
- a new runtime, testkit, or core public type, maximum, failure, composer
  rule, registry, or downcast
- changes to Contracts 037, 047, 057, or 061
- provider contact, live probes, currentness, watcher or skill work,
  papercut fixes, or generation closeout

## Acceptance Criteria

- [ ] three independent ledgers reconcile exactly to 10, 6, and 19 rows with
      32 emitted and 3 withheld in total
- [ ] every emitted row comes from its exact prepared operation and retains
      source, route, operation shape, lifecycle, value, omission,
      applicability, and evidence truth
- [ ] the three withheld rows never appear, and each fixture asserts why
- [ ] Ollama twin rows are keyed by shape and never cross shapes
- [ ] optional-request rows appear only under the maximal validated profile
- [ ] owned-only serving controls never appear on `llama-cpp.attached`
- [ ] activity observation stays descriptor-only where emitted
- [ ] no runtime/testkit/core public API, contract, active observation,
      acknowledgement, execution, or mutation authority is added
- [ ] touched source stays below god-file thresholds and the repository
      scan does not exceed its accepted baseline

## Review Oracle

Invariant: candidate J publishes only truth already bound by one exact
prepared operation on one exact route and shape.

Counterexamples, each a fail:

- `llama-cpp.attached` cancellation emitted from the run handle's
  driver-level control rather than a prepared plan requirement
- `llama-cpp.owned` activity emitted without a retained activity profile
- an Ollama session reasoning control emitted when no session input exists
- an Ollama row emitted from the wrong shape's facade, or twin rows collapsed
  into one
- a serving control emitted on the attached route, or a portable reasoning
  capability claimed for `llama-cpp.owned`
- an optional-request row emitted from a minimal profile
- ledgers reaching 35 through an exception list or borrowed identity

## Validation

- `cargo fmt -p swallowtail-adapter-llama-cpp -p swallowtail-adapter-ollama -- --check`
- `effigy validate:focused swallowtail-adapter-llama-cpp swallowtail-adapter-ollama`
- `effigy package:verify-affected swallowtail-adapter-llama-cpp swallowtail-adapter-ollama`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy --json scan god-files`
- `git diff --check`

No live probe or provider contact belongs to this card.

## Auto-Continuation

No. Stop after one reviewable PR for exact-head review.

## Stop Conditions

- any row needs a new runtime/testkit/core public type, source kind,
  authority posture, composer rule, fixed maximum, or contract amendment
- the exact 10/6/19 ledgers need an exception list, inferred support, or
  truth borrowed from another route or shape
- activity cannot remain descriptor-only, or a withheld row would need a
  filter to stay out
- scope widens beyond the two named adapter packages or contacts a provider

## Evidence

- [candidate J audit](../../../triage/20260904-134914-contract-061-candidate-j-audit.md)
- [Batch 9.4 package expansion](../../../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [Contract 061](../../../contracts/061-consumer-route-feature-and-control-projection.md)
- [reviewed census](../../../triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [completed card 024](024-contract-061-deepagents-kiro-qoder-zcode-package-completion.md)

## Result

Implemented the exact 35-row candidate J tranche. Three independent ledgers
reconcile to 10 `llama-cpp.attached`, 6 `llama-cpp.owned`, and 19
`ollama.attached` with 32 emitted and 3 construction-time withheld.
`llama-cpp.attached` cancellation, `llama-cpp.owned` activity, and Ollama
interactive-session reasoning stay out of the emitted sets. Ollama twins are
keyed by operation shape. Optional reasoning and structured-output rows
appear only on the maximal inference attempt. Owned serving controls stay
off the attached route. Current `v0.4.0` adapter API snapshots record only
the six additive contribution methods.
