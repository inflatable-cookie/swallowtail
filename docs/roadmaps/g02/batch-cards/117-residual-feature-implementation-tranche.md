# 117 Residual Feature Implementation Tranche

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../034-residual-feature-matrix-truth-and-programme-checkpoint.md`
Depends on: card 116

## Objective

Implement only the contract-ready residual feature routes selected by cards
115-116.

## Scope

1. Add separate Qwen headless and Ollama attached prepared interactive
   profiles.
2. Bind Contract 043 mode, interface, route, operation, host, topology,
   version, bounds, state, and cleanup.
3. Implement one exact Qwen owned child or Ollama HTTP request per turn.
4. Reject identity, lifecycle, media, cost, or version drift.
5. Preserve provider completion and cleanup as independent outcomes.
6. Change matrix cells only after focused deterministic conformance passes.

## Acceptance Criteria

- [x] every converted cell has a public prepared path
- [x] forward admission remains visible and unguaranteed
- [x] media and session roles remain distinct
- [x] billed cost remains provider-authoritative
- [x] Qwen private resume does not claim public load or resume
- [x] Ollama transcript mutation is transactional and bounded
- [x] focused exact-range conformance passes offline

## Result

- `QwenPreparedSession` opens one turn-scoped runtime handle over the exact
  prepared model, host, working resource, access, and `0.19.11` binding.
- The first Qwen turn has no selector. Later turns use only the privately
  observed exact `--resume` ID. Every child joins before reuse. Mismatch or
  failed terminal truth invalidates the handle.
- `OllamaPreparedSession` opens one resource-free attached-runtime handle over
  the exact runtime version, model tag, and digest.
- Ollama replays only committed user/assistant pairs. Provider failure leaves
  history unchanged; session close clears history and preserves the attached
  runtime.
- The shared attached-runtime preflight now admits exact interactive direct
  inference as well as structured direct inference. Evidence requirements are
  unchanged.
- The two matrix cells changed from `No` to `Yes`. Public load, resume,
  provider session references, media, and billed cost remain absent.
- Focused core, runtime, testkit, Qwen, and Ollama validation passed.

## Auto-Continuation

Continue to card 118 only after every selected cell has deterministic
production evidence.
