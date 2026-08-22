# 2026-08-22 g04.039 xAI Responses Controls Closeout

Status: review PR open; merge not authorized
Owner: Tom
Milestone: g04.039
Cards: 107-109
Base: `origin/main` at `906e776d957b02b25b0d69fce71b28d6543af40f`
PR: [#38](https://github.com/inflatable-cookie/swallowtail/pull/38)
PR head at opening: `6ad12932`
Branch: `t3code/xai-responses-controls`

This closeout-only follow-up records the PR truth; the final branch head is
reported with the worker handoff because this file is part of that head.

## Result

Research 187 is promoted for the existing `xai.responses-websocket` route and
dated `xai-responses-websocket-2026-04-23` facade. The worker binds only exact
model-qualified dispatch controls:

- `grok-4.5`: reasoning `low`, `medium`, `high`;
- `grok-4.6`: reasoning `low`, `medium`, `high`, `xhigh`;
- both exact models: positive `max_output_tokens` in `1..=2_147_483_647`;
- structured one-response and serial connection-local session profiles.

Aliases, Grok 4.5 `xhigh`, other model ids, and multi-agent effort remain
withheld. Omitted controls preserve the existing request body. Session
selection is fixed through the first turn, serial continuation, failed turn,
and fresh working-state replacement. The implementation claims request
dispatch only, not provider acceptance, effective reasoning depth, or exact
generated text length.

## Evidence

- [Research 187](../research/187-xai-responses-control-evidence.md) records
  the official sources, retrieval date, SHA-256 snapshots, specimens, exact
  dispositions, and facade verdict.
- [Prepared integration guide](../guides/realtime-prepared-integration.md)
  records the route-local dispatch boundary.
- `crates/swallowtail-adapter-xai/src/controls.rs` centralizes the exact
  model/value/range admission and request-plan agreement checks.
- Prepared inputs, immutable plan/evidence, run/session drivers, and the
  WebSocket protocol carry selected controls without changing absent-control
  bytes.
- Deterministic tests cover independent controls, first/later/fresh session
  dispatch, failed turns, unsupported combinations, overflow, and request-plan
  drift before network or credential effects.

Official evidence used by Research 187:

- [xAI WebSocket mode](https://docs.x.ai/developers/advanced-api-usage/websocket-mode)
- [xAI reasoning controls](https://docs.x.ai/developers/model-capabilities/text/reasoning)
- [xAI Grok 4.5](https://docs.x.ai/developers/models/grok-4.5)
- [xAI Grok 4.6](https://docs.x.ai/developers/models/grok-4-6)
- [xAI Responses reference](https://docs.x.ai/developers/rest-api-reference/inference/chat)

## Validation

Passed:

- `cargo fmt -p swallowtail-adapter-xai`
- `effigy validate:focused swallowtail-adapter-xai` — 36 tests passed
- `effigy package:verify-affected swallowtail-adapter-xai`
- `effigy check:examples`
- `effigy qa:routes`
- `effigy qa:northstar`
- research, logs, roadmaps, g04, batch-card index, and next-action gates
- `effigy package:api`
- `git diff --check`

`effigy doctor` remains blocked by the repository's inherited god-file scan
(371 findings, including 45 errors); no new doctor finding was introduced by
this route-local work.

## Deferred Shared-Surface Delta

The worker intentionally did not edit shared surfaces. After review merge, the
orchestrator should apply the exact xAI Responses control truth to:

- `docs/architecture/system-architecture.md`: record exact Grok rows and
  prepared dispatch boundaries without acceptance/effectiveness claims;
- `docs/guides/provider-route-matrix.md`: update the xAI Responses route
  control cells;
- `docs/guides/provider-solution-feature-matrix.csv`: update the xAI route
  reasoning/output-control rows and note exact-model qualification;
- `CHANGELOG.md`: add the unreleased route-local control binding;
- `docs/roadmaps/g04/per-route-feature-completion.md`: mark this control
  family integrated;
- shared roadmap front doors, indexes, and `Next Task`: advance statuses only
  after merge;
- `release-baselines/public-api-0.3.3/packages.txt`: no package-set change;
  this worker added only the allowed unreleased xAI API baseline and package
  entry.

No merge, shared-surface completion, live provider acceptance, account
inspection, release, or publication is claimed from this worker branch.
