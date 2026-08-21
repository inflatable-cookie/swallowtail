# 074 Ollama 0.32.15 Identity

Status: completed
Owner: Tom
Milestone: [g04.027 Ollama 0.32.15 Useful Newer](../027-ollama-0-32-15-useful-newer.md)
Created: 2026-08-21

## Task

Freeze official GitHub `ollama/ollama` `v0.32.15` identity evidence.
Name segment shape. Do not edit production claims in this card.

## Method

1. Observe official `v0.32.15` (GitHub latest stable, published 2026-08-19)
2. Compare selected tagged `api/types.go` structs and `server/routes.go`
   registrations against the frozen `0.32.14` corpus
3. Classify unmapped additions
4. Write identity fixture under
   `crates/swallowtail-adapter-ollama/tests/fixtures/ollama-0.32.15/`
5. Write research record 174
6. Name segment shape

No provider prompt. No live session. Host install not changed.

## Expected Shape

Compatible-extension: `api/types.go` is byte-identical to `v0.32.14`.
Selected structs and the five native routes remain. `routes.go` changed
only unselected scheduler cache and parser-error cancel.

## Acceptance

- Identity fixture written
- Research 174 promoted
- Shape named: compatible-extension / milestone / stop
- No production claim edit
- Passes `effigy validate:focused swallowtail-adapter-ollama` at the claim
  card

Auto-continue to claim card 075.

## Out Of Scope

- Gemini requalification (deferred)
- Codex
- Qwen
- Mapping unused surfaces
- Flattening llama.cpp onto Ollama
- Provider work
- Decoder updates
