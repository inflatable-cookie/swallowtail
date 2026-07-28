# 086 Generation-Control Implementation Tranche

Status: ready
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

- [ ] every changed matrix cell has a realized prepared operation
- [ ] request, plan, dispatch, and observed evidence agree
- [ ] malformed and unsupported inputs fail deterministically
- [ ] topology, cleanup, and version posture remain unchanged
- [ ] package examples compile without live access

## Auto-Continuation

Continue to card 087 only after all seven cells have public prepared paths and
focused plus workspace validation passes.

## Auto-Continuation

No while planned. Card 085 must make the implementation envelope exact.
