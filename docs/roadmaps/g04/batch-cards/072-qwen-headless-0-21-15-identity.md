# 072 Qwen Headless 0.21.15 Identity

Status: completed
Owner: Tom
Milestone: [g04.026 Qwen Headless 0.21.15 Useful Newer](../026-qwen-headless-0-21-15-useful-newer.md)
Created: 2026-08-21

## Task

Freeze official npm `@qwen-code/qwen-code@0.21.15` identity evidence.
Name segment shape. Do not edit production claims in this card.

## Method

1. Observe official `0.21.15` (npm `latest`, published 2026-08-20)
2. Extract the official npm tarball under `/tmp`
3. Compare selected git blobs against the frozen `0.21.14` corpus
4. Verify selected mapped flags remain
5. Classify unmapped additions
6. Write identity fixture under
   `crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.21.15/`
7. Write research record 173
8. Name segment shape

No provider prompt. No live session. Host install not changed.

## Expected Shape

Compatible-extension: `types.ts` and `systemController.ts` are
byte-identical to `0.21.14`. Selected flags remain. `config.ts` changed
only unmapped `--session-id` occupancy.

## Acceptance

- Identity fixture written
- Research 173 promoted
- Shape named: compatible-extension / milestone / stop
- No production claim edit
- Passes `effigy validate:focused swallowtail-adapter-qwen` at the claim
  card

Auto-continue to claim card 071.

## Out Of Scope

- Gemini requalification (deferred)
- Codex
- Mapping unused surfaces
- Provider work
- Decoder updates
