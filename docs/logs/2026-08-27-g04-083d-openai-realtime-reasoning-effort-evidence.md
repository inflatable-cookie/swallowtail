# 2026-08-27 g04.083d OpenAI Realtime Reasoning-Effort Evidence

Status: complete
Card: 235
Research: 236

## Boundary

Evidence only. The worker updated this file, card 235, Research 236, and new
OpenAI-local frozen evidence. Shared planning and production code stayed
unchanged.

## Outcome

Research 236 is promoted. Exact model `gpt-realtime-2.1` is statically
reasoning-capable on the Realtime transport. The closed Realtime effort enum
is `minimal|low|medium|high|xhigh`. Session-scoped dispatch belongs on
`session.update.session.reasoning.effort`; per-response override exists in the
reference but is withheld for the current manual PCM route shape.

Current production on facade `openai-realtime-2026-07-22` remains an honest
empty set: `src/realtime.rs` rejects any reasoning selection before access or
connection work, and the dated fixture README still records reasoning as
unsupported.

Future binding may admit all five Realtime values at a new opaque facade point.
Background values `none` and `max`, Responses semantics, catalogue inference,
and reasoning-token usage are not Realtime selected-effort proof.

Frozen evidence:

- `crates/swallowtail-adapter-openai/tests/fixtures/openai-realtime-reasoning-effort-2026-08-27/README.md`
- `crates/swallowtail-adapter-openai/tests/fixtures/openai-realtime-reasoning-effort-2026-08-27/reasoning-effort-session-update.json`
- `crates/swallowtail-adapter-openai/tests/fixtures/openai-realtime-reasoning-effort-2026-08-27/reasoning-effort-session-updated.json`
- `crates/swallowtail-adapter-openai/tests/fixtures/openai-realtime-reasoning-effort-2026-08-27/reasoning-effort-response-create-override.json`

Official specimens fetched 2026-08-27 with digests recorded in Research 236.

## Validation

```sh
effigy validate:focused swallowtail-adapter-openai
effigy qa:northstar
git diff --check
```

All passed on the worker branch before PR open.

## Unresolved

Live provider acceptance, effective reasoning depth, and per-response override
qualification remain for a later binding card. Shared indexes and Next Task
promotion belong to the orchestrator after merge.
