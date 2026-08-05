# 108 Direct And Attached Session Replacement

Status: ready
Owner: Tom
Created: 2026-08-05
Milestone: `../039-provider-wide-session-usability-restoration.md`
Depends on: card 107

## Goal

Map every remaining prepared interactive route to truthful fresh replacement.

## Scope

1. Map Ollama and xAI through the generic prepared replacement constructor.
2. Map Anthropic and DeepSeek through adapter-local direct-continuation wrappers.
3. Map ordinary Alibaba delete-on-close without changing retained recovery.
4. Prove exact interrupted-turn identity, new handle, context loss, and no replay.

## Validation

- `effigy validate:focused swallowtail-adapter-anthropic swallowtail-adapter-deepseek swallowtail-adapter-ollama swallowtail-adapter-xai`
- `effigy validate:focused swallowtail-adapter-alibaba-model-studio swallowtail-runtime`
- affected-package verification for all five adapter packages

## Stop Conditions

- stop if any mapping can expose or reconstruct private continuation state
- stop if ordinary Alibaba inherits retained-session or management authority

## Auto-Continuation

Continue to card 109 when all five mappings pass.
