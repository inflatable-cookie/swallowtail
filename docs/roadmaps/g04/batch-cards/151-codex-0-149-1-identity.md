# 151 Codex 0.149.1 Identity

Status: completed
Owner: Tom
Milestone: [g04.054 Codex 0.149.1 Useful Newer](../054-codex-0-149-1-useful-newer.md)
Created: 2026-08-24

## Task

Freeze official npm `@openai/codex@0.149.1` identity evidence. Name segment
shape. Do not edit production claims in this card.

## Method

1. Observe official `0.149.1` (npm `latest`, published 2026-08-24)
2. Extract darwin-arm64 and linux-x64 binaries from official npm platform tarballs
3. Compare exec help output against `0.149.0` frozen corpus
4. Verify selected mapped flags remain
5. Check app-server schema and methods
6. Classify unmapped additions
7. Write identity fixture under `crates/swallowtail-adapter-codex/tests/fixtures/codex-cli-0.149.1/`
8. Write research record 201
9. Name segment shape

No provider prompt. No live session. Host install not changed.

## Expected Shape

Compatible-extension: selected mapped flags present, schema bundles
byte-identical to `0.149.0`, ModelListParams unchanged. Exec help may
differ by unmapped `--thread-source`.

## Acceptance

- Identity fixture written
- Research 201 promoted
- Shape named: compatible-extension / milestone / stop
- No production claim edit
- Auto-continue to claim card 152

## Out Of Scope

- Gemini requalification (deferred)
- Mapping unused surfaces
- Provider work
- Decoder updates
- Editing `docs/roadmaps/README.md` or `docs/roadmaps/g04/README.md`
