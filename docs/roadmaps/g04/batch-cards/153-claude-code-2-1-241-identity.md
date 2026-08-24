# 153 Claude Code 2.1.241 Identity

Status: completed
Owner: Tom
Milestone: [g04.055 Claude Code 2.1.241 Useful Newer](../055-claude-code-2-1-241-useful-newer.md)
Created: 2026-08-24

## Task

Freeze official npm `@anthropic-ai/claude-code` `2.1.241` identity
evidence. Name segment shape. Do not edit production claims in this card.

## Method

1. Observe official `2.1.241` (npm `latest`, published 2026-08-22) and
   published intermediates `2.1.239` and `2.1.240`
2. Compare extracted official wrapper files, linux-x64 `--help` /
   `--version`, and changelog against the frozen `2.1.238` corpus
3. Classify unmapped additions
4. Write identity fixture under
   `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.241/`
5. Write research record 202
6. Name segment shape

No provider prompt. No live session. Host install not present and not
changed.

## Expected Shape

Compatible-extension: installer wrapper files match `2.1.238` except the
version pin. Official extracted help is byte-identical to `2.1.238`.
Selected mapped stream-JSON flags stay. Changelog extras stay unmapped.

## Acceptance

- Identity fixture written
- Research 202 promoted
- Shape named: compatible-extension / milestone / stop
- No production claim edit
- Passes `effigy validate:focused swallowtail-adapter-claude-agent` at
  the claim card

Auto-continue to claim card 154.

## Out Of Scope

- Gemini requalification (deferred)
- Codex
- Qwen
- Ollama
- Mapping unused surfaces
- Flattening onto Claude Agent ACP
- Provider work
- Decoder updates
- Editing `docs/roadmaps/README.md` or `docs/roadmaps/g04/README.md`
