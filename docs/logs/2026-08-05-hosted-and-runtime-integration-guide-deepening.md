# 2026-08-05 Hosted And Runtime Integration Guide Deepening

Roadmap: `../roadmaps/g03/042-complete-integration-guide-system.md`
Card: `../roadmaps/g03/batch-cards/121-hosted-local-and-realtime-guide-deepening.md`

## Changed

- normalized Anthropic Messages, Kimi Platform, DeepSeek, Alibaba, OpenAI
  background, Anthropic Managed Agents, Bedrock, realtime, Ollama, and
  llama.cpp guidance against Contract 052
- corrected Anthropic direct's stale text-only claim and documented its
  bounded PNG and provider web-search inputs
- documented OpenAI background reasoning, provider-native JSON Schema,
  terminal response deletion, detachment, and reconciliation truth
- documented Ollama selected-model reasoning, structured output, and bounded
  clean-turn transcript replay
- preserved branch-specific catalogue, inference, conversation, retained-run,
  media, attached-runtime, and owned-serving authority
- moved the remaining 14 route rows from partial to complete; all 33
  production route rows are now complete

## Validation

- `effigy check:examples` — passed
- `effigy qa:docs` — passed
- `effigy qa:routes` — passed
- `cargo fmt --all -- --check` — passed
- `git diff --check` — passed

No live or authenticated provider work ran.

## Next Move

Execute card 122: add cross-cutting consumer and operator feature runbooks for
every feature-family row in the integration guide map.
