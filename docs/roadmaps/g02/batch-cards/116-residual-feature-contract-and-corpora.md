# 116 Residual Feature Contract And Corpora

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../034-residual-feature-matrix-truth-and-programme-checkpoint.md`
Depends on: card 115

## Objective

Settle contract fit for Qwen headless and Ollama attached interactive
sessions, then freeze exact offline corpora before implementation.

## Scope

1. Preserve interface, route, operation, evidence, host, version, and support
   authority.
2. Keep Qwen harness-retained resume across owned child processes separate
   from Ollama consumer-owned bounded transcript replay over attached HTTP.
3. Define exact admission, identity, history, failure, cancellation, deadline,
   terminal, and cleanup behavior.
4. Freeze Qwen `0.19.11` and every maintained Ollama
   `0.14.0..=0.32.1` segment, including the `0.32.2` exclusion.
5. Prove no hidden provider-session management, server ownership, model
   selection, credential, write, or sandbox authority.
6. Add no generic session, media, pricing, or forward-compatibility fallback.

## Acceptance Criteria

- [x] every selected cell has a settled contract path
- [x] every selected version segment has deterministic evidence
- [x] usage cannot mint billed cost
- [x] streaming cannot mint media or interactive continuity
- [x] Qwen and Ollama continuity ownership remains visibly different
- [x] implementation scope is bounded and fixture-first

## Result

- Contract 043 separates restarted harness continuation from consumer-owned
  transactional transcript replay over turn-scoped provider work.
- Research 062 freezes exact source evidence and implementation boundaries.
- Qwen `0.19.11` fixtures cover first-turn identity, exact private resume,
  mismatch rejection, failure invalidation, and child-before-next-turn join.
- Ollama fixtures cover all four existing qualification points, exact
  `0.32.2` exclusion, ordered replay, successful pair commit, unchanged
  history on failure, and attached-runtime preservation.
- Eight focused corpus tests pass without a live provider effect.

Card 117 is contract-ready.

## Auto-Continuation

Continue only when every selected route is contract-ready.
