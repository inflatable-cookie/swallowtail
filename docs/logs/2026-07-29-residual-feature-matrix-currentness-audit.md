# 2026-07-29 Residual Feature Matrix Currentness Audit

Roadmap: `../roadmaps/g02/034-residual-feature-matrix-truth-and-programme-checkpoint.md`
Card: 115

## Changed

- Research 061 classifies all 61 starting cells.
- Ten unverified-newer cells without a runtime-ordered interface axis become
  `Not applicable`.
- Three ordinary-interactive cells and sixteen realtime-media cells become
  `Not applicable` because the operation shape does not fit.
- Five local-compute or subscription-harness billed-cost cells become
  `Not applicable`.
- Twenty-seven exact gaps remain `No`: nine contract or corpus candidates,
  three separate realtime routes, twelve selected-surface cost absences, and
  three non-authoritative harness cost fields.
- The route-matrix gate now preserves every disposition and has no unaudited
  fallback cell.
- Qwen headless and Ollama attached interactive sessions are selected for the
  first proof pair.

## Current State

- card 115 is complete
- card 116 is ready
- the full matrix has 236 `No` and 216 `Not applicable` cells
- no capability became `Yes` during this audit
- billed cost still requires an exact provider-declared per-attempt charge
- Alibaba, Bedrock, and xAI realtime media remain separate-route work
- llama.cpp forward admission remains contract and corpus gated
- no provider, credential, process, model, container, or paid effect occurred

## Validation

- `bash -n scripts/check-provider-route-matrix.sh`: passed
- `bash scripts/check-provider-route-matrix.sh`: passed
- `effigy qa:docs`: passed
- `effigy qa:routes`: passed
- `git diff --check`: passed

## Remaining Risks

- Qwen continuation uses harness-retained provider state across child
  processes; Ollama continuation uses consumer-owned bounded replay. Their
  implementation must not share an implicit persistence claim.
- llama.cpp exposes ordered build tags without semver or a stable channel.
  Its exact build and commit axes need a separate compatibility decision.
- provider cost and billing surfaces can change independently of inference
  response schemas; retained `No` values require currentness review when
  those schemas change.
- the three realtime candidates have distinct endpoint, credential, model,
  and lifecycle contracts despite sharing provider families with current
  routes.

## Next

Card 116 settles Qwen `0.19.11` and Ollama `0.14.0..=0.32.1`
interactive-session contract fit and freezes exact offline corpora. Cards
117-118 remain in bounds.
