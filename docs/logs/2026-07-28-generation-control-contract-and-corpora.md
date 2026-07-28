# 2026-07-28 Generation-Control Contract And Corpora

## Changed

- promoted Contract 040
- separated requested, planned, dispatched, accepted, effective, and observed
  control states
- separated provider-native schema enforcement from harness validation
- bound reasoning and schema support to exact model and version evidence
- froze OpenAI background and Realtime, Ollama, and OpenCode requests
- fixed OpenCode schema retries at zero

## Current State

Card 085 is complete. Card 086 is ready for seven cells:

- OpenAI background reasoning and structured output
- OpenAI Realtime output maximum
- Ollama reasoning and structured output
- OpenCode reasoning and structured output

Cards 086-087 remain in bounds. Qwen and Gemini host-scoped configuration,
other direct providers, xAI, and later feature families remain outside this
batch.

## Validation

- four focused generation-control corpus tests passed
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy qa:routes`
- `effigy format:check`
- `git diff --check`

No live credential, provider request, container, or model server was used.

## Next

Execute card 086. Implement the seven exact prepared paths, then update matrix
cells only after their public operations and conformance evidence exist.
