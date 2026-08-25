# 2026-08-25 g04.063 Kimi Code Headless Reasoning Effort Closeout

Status: complete; evidence stop
Owner: Tom
Milestone: g04.063
Cards: 176-178

## Result

Research 210 admits an empty deliver-now set. Card 176 completed the evidence
gate. Cards 177-178 stay blocked. No headless reasoning-effort feature ships.

Exact `@moonshot-ai/kimi-code@0.38.0` documents process-local
`KIMI_MODEL_THINKING_EFFORT` after config resolution, with Kimi-protocol
normalization and fallback to `default_effort` on unsupported configured values.
Headless `-p --output-format stream-json` exposes no confirmation transport
comparable to Kimi ACP `session/set_config_option` (Research 207) or Qwen
`set_effort` (Research 189). Ambient `thinking.enabled = false` disables the env
override entirely. The route's Ambient harness posture does not let the adapter
read or override user `config.toml`, and headless provides no session-open
catalogue snapshot to freeze per-model `support_efforts` at preparation.

## Validation

- `effigy validate:focused swallowtail-adapter-kimi` — passed
- `effigy qa:northstar` — passed
- relevant docs index and next-action gates — passed
- `git diff --check` — passed
- inherited `effigy doctor` baseline unchanged

## Continuation

g04 remains open. Orchestrator should reassess the remaining per-route feature
inventory after this evidence stop. Do not close or roll over the generation
without explicit operator direction.
