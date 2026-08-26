# 190 Qwen Headless 0.22.1 Identity

Status: completed
Owner: Tom
Milestone: [g04.068 Qwen Headless 0.22.1 Useful Newer](../068-qwen-headless-0-22-1-useful-newer.md)
Created: 2026-08-26

## Task

Freeze official npm `@qwen-code/qwen-code` `0.22.1` identity evidence
and published `0.22.0`. Name segment shape. Do not edit production
claims in this card.

## Method

1. Observe official `0.22.1` (npm `latest`, published 2026-08-25) and
   published intermediate `0.22.0`
2. Extract official npm tarballs under `/tmp`
3. Compare selected git blobs against the frozen `0.21.15` corpus
4. Verify selected mapped flags remain
5. Classify unmapped additions
6. Write identity fixture under
   `crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.22.1/`
7. Write research record 215
8. Name segment shape

No provider prompt. No live session. Host install not present and not
changed.

## Expected Shape

Compatible-extension: selected mapped flags, stream types, catalogue
filter, and `set_effort` stay. `0.22.0` types/controller match
`0.21.15`. `0.22.1` extras stay unmapped. Keep `0.21.16` unpublished.

## Acceptance

- Identity fixture written
- Research 215 promoted
- Shape named: compatible-extension
- No production claim edit
- Passes `effigy validate:focused swallowtail-adapter-qwen` at the
  claim card

Auto-continue to claim card 191.

## Out Of Scope

- Gemini requalification (deferred)
- Extending reasoning or budgets past exact `0.21.15`
- Mapping unused surfaces
- Flattening onto Model Studio or ACP
- Provider work
- Decoder updates
- Moving `docs/roadmaps/README.md` or g04 generation status
