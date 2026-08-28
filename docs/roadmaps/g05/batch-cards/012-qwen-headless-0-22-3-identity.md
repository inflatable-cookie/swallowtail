# 012 Qwen Headless 0.22.3 Identity

Status: completed
Owner: Tom
Milestone: [g05.004 Qwen Headless 0.22.3 Useful Newer](../004-qwen-headless-0-22-3-useful-newer.md)
Created: 2026-08-28

## Task

Freeze official npm `@qwen-code/qwen-code` `0.22.3` identity evidence
and published `0.22.2`. Name segment shape. Do not edit production
claims in this card.

## Method

1. Observe official `0.22.3` (npm `latest`, published 2026-08-28) and
   published intermediate `0.22.2`
2. Extract official npm tarballs under `/tmp`
3. Compare selected git blobs against the frozen `0.22.1` / `0.22.2`
   corpora
4. Verify selected mapped flags remain
5. Classify unmapped additions
6. Write identity fixture under
   `crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.22.3/`
7. Write research record 258
8. Name segment shape

No provider prompt. No live session. Host install not present and not
changed.

## Expected Shape

Compatible-extension: selected mapped flags, stream types, catalogue
filter, and `set_effort` stay. `0.22.2` types/controller/session match
`0.22.1`. `0.22.3` extras (`tools.eager`, GeminiMd-to-Memory rename,
session comment rename) stay unmapped. Keep `0.21.16` unpublished.

## Acceptance

- Identity fixture written
- Research 258 promoted
- Shape named: compatible-extension
- No production claim edit
- Passes `effigy validate:focused swallowtail-adapter-qwen` at the
  claim card

Auto-continue to claim card 013.

## Out Of Scope

- Gemini requalification (deferred)
- Extending reasoning or budgets past exact `0.21.15`
- Mapping unused surfaces
- Flattening onto Model Studio, ACP, Pi, or Oh My Pi
- Provider work
- Decoder updates
- Moving `docs/roadmaps/README.md` Next Task
- Displacing g05.001-g05.003
