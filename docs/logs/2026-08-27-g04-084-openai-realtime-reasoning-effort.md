# 2026-08-27 g04.084 OpenAI Realtime Reasoning Effort

Status: done
Generation: g04
Cards: 236-237
Research: 236
Worker branch: `t3code/openai-realtime-reasoning-effort`
Worker worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-def5c1e9`

## Boundary

One OpenAI-owned delivery lane. Bound only the five exact session-scoped
Realtime values promoted by Research 236 at opaque facade
`openai-realtime-reasoning-2026-08-27` / private behavior
`openai.realtime-manual-pcm-reasoning-v2`. Historical
`openai-realtime-2026-07-22` remains superseded proof.

## Outcome

- Prepared input admits optional `ReasoningMode` for
  `minimal|low|medium|high|xhigh` and rejects every other value before effects.
- Selection agrees across capability, plan, evidence, request, `session.update`,
  and matching `session.updated` acknowledgement.
- Omission keeps historical no-`reasoning` session-update bytes and claims no
  default.
- Fresh restoration re-encodes the immutable selected request.
- Output maximum and reasoning compose independently.
- Per-response override, effective depth, and reasoning-token claims stay out.

## Validation

- `cargo fmt -p swallowtail-adapter-openai`
- `cargo test -p swallowtail-adapter-openai --test realtime_prepared_facade` (22/22)
- `effigy validate:focused swallowtail-adapter-openai`
- `effigy package:verify-affected swallowtail-adapter-openai`
- `effigy check:examples`
- `effigy qa:routes`
- `effigy qa:northstar`
- `effigy package:api`
- `git diff --check`

## Next Move

Open one reviewable PR against current pushed `main`. Do not merge. Orchestrator
owns shared inventory/programme/index/Next Task closeout after merge.
