# 081 OpenCode HTTP 1.18.20 Identity

Status: completed
Owner: Tom
Milestone: [g04.029 OpenCode HTTP 1.18.20 Useful Newer](../029-opencode-http-1-18-20-useful-newer.md)
Created: 2026-08-21

## Task

Freeze official npm `opencode-ai@1.18.20` identity evidence. Name
segment shape. Do not edit production claims in this card.

## Method

1. Observe official `1.18.20` (npm `latest`, published 2026-08-21)
2. Record published intermediate `1.18.19`
3. Hash GitHub `packages/sdk/openapi.json` at `v1.18.18`, `v1.18.19`,
   and `v1.18.20`
4. Confirm selected mapped routes remain
5. Classify changelog extras as unmapped
6. Write identity fixture under
   `crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.18.20/`
7. Write research record 176
8. Name segment shape

No provider prompt. No live session. Host install not changed.

## Expected Shape

Compatible-extension: OpenAPI byte-identical to `1.18.18`; selected
mapped subset unchanged; keep `surface-19`.

## Acceptance

- Identity fixture written
- Research 176 promoted
- Shape named: compatible-extension / milestone / stop
- No production claim edit
- Focused adapter proof runs with the claim card

Auto-continue to claim card 082.

## Out Of Scope

- Gemini requalification (deferred)
- Mapping unused surfaces
- Provider work
- Decoder updates
- Next Task / generation status edits
