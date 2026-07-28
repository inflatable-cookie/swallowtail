# 086 Generation-Control Implementation Tranche

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../026-generation-control-feature-closure.md`
Depends on: card 085

## Objective

Implement the contract-ready generation-control tranche through existing
prepared route identities.

## Scope

1. Add the provider-neutral structured-output enforcement-source constraint
   required by Contract 040.
2. OpenAI:
   - background reasoning selection
   - background provider-native structured output
   - Realtime session output maximum
3. Ollama attached:
   - exact `off`, `low`, `medium`, or `high` reasoning selection
   - provider-native inline JSON Schema output
   - selected-model capability evidence and the full qualified version window
4. OpenCode HTTP:
   - exact model variant reasoning selection
   - harness-validated inline JSON Schema output with zero schema retries
   - provider catalogue evidence for model reasoning and tool capability
5. Keep request, plan, dispatch, matrix, examples, and prepared descriptors in
   agreement.
6. Add malformed, unsupported, drift, unverified-newer, topology, cleanup, and
   redaction coverage without live access.

Do not implement Qwen or Gemini host-scoped configuration, xAI, another route,
or consumer adoption in this card.

## Acceptance Criteria

- [x] every changed matrix cell has a realized prepared operation
- [x] request, plan, dispatch, and observed evidence agree
- [x] malformed and unsupported inputs fail deterministically
- [x] topology, cleanup, and version posture remain unchanged
- [x] package examples compile without live access

## Result

- OpenAI background exposes exact GPT-5.6 reasoning and provider-native inline
  JSON Schema; Realtime exposes an exact 1-4096 session output maximum.
- Ollama attached binds `off`, `low`, `medium`, or `high` reasoning and native
  inline JSON Schema to selected-model capability evidence.
- OpenCode binds exact catalogue variants and tool capability to reasoning and
  zero-retry harness-validated schema output.
- Exact request constraints reach the preflight plan and wire request. Missing
  evidence, unsupported values, and request-plan drift stop before effects.
- Seven matrix cells changed from `No` to `Yes`.

## Auto-Continuation

Satisfied. Continue to card 087.
