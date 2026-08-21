# 079 Claude Code 2.1.238 Identity

Status: completed
Owner: Tom
Milestone: [g04.028 Claude Code 2.1.238 Useful Newer](../028-claude-code-2-1-238-useful-newer.md)
Created: 2026-08-21

## Task

Freeze official npm `@anthropic-ai/claude-code` `2.1.238` identity
evidence. Name segment shape. Do not edit production claims in this card.

## Method

1. Observe official `2.1.238` (npm `latest`, published 2026-08-20) and
   published intermediates `2.1.236` and `2.1.237`
2. Compare extracted official tarball wrapper files and changelog
   against the frozen `2.1.235` corpus
3. Classify unmapped additions
4. Write identity fixture under
   `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.238/`
5. Write research record 175
6. Name segment shape

No provider prompt. No live session. Host install not present and not
changed.

## Expected Shape

Compatible-extension: installer wrapper files match `2.1.235` except the
version pin. Selected mapped stream-JSON flags stay. Changelog extras
stay unmapped.

## Acceptance

- Identity fixture written
- Research 175 promoted
- Shape named: compatible-extension / milestone / stop
- No production claim edit
- Passes `effigy validate:focused swallowtail-adapter-claude-agent` at
  the claim card

Auto-continue to claim card 080.

## Out Of Scope

- Gemini requalification (deferred)
- Codex
- Qwen
- Ollama
- Mapping unused surfaces
- Flattening onto Claude Agent ACP
- Provider work
- Decoder updates
- Next Task changes, Kimi Platform implementation, architecture, or contract
  edits
