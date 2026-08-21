# 068 Codex 0.149.0 Identity

Status: completed
Owner: Tom
Milestone: [g04.025 Codex 0.149.0 Useful Newer](../025-codex-0-149-0-useful-newer.md)
Created: 2026-08-21

## Task

Freeze official npm `@openai/codex@0.149.0` identity evidence. Name segment
shape. Do not edit production claims in this card.

## Method

1. Observe official `0.149.0` (npm `latest`, published 2026-08-20)
2. Extract darwin-arm64 and linux-x64 binaries from GitHub release `rust-v0.149.0`
3. Compare exec help output against `0.148.0` frozen corpus
4. Verify selected mapped flags remain
5. Check app-server schema and methods
6. Classify unmapped additions
7. Write identity fixture under `crates/swallowtail-adapter-codex/tests/fixtures/codex-cli-0.149.0/`
8. Write research record 172
9. Name segment shape

No provider prompt. No live session. Host install not changed.

## Expected Shape

Compatible-extension: exec help is byte-identical to `0.148.0`, selected
flags present, ModelListParams unchanged.

## Acceptance

- Identity fixture written
- Research 172 promoted
- Shape named: compatible-extension / milestone / stop
- No production claim edit
- Passes `effigy validate:focused swallowtail-adapter-codex`
- Passes `effigy qa:northstar`

Auto-continue to claim card 069.

## Out Of Scope

- Gemini requalification (deferred)
- Mapping unused surfaces
- Provider work
- Decoder updates
