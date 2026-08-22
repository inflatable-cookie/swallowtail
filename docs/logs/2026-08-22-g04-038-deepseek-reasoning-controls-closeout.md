# 2026-08-22 g04.038 DeepSeek Reasoning Controls Closeout

Status: complete on review PR; not merged
Owner: Tom
PR: https://github.com/inflatable-cookie/swallowtail/pull/36
Evidence head: `e66d6f2e`

Card 106 replaces the reservation with route-local evidence. The orchestrator
records merge and shared-surface closeout separately.

## Result

Research 186 promotes exact `low`, `high`, and `max` `ReasoningMode` values for
`deepseek-v4-pro` on the existing `deepseek-openai-chat-2026-07-22` facade.
The adapter sends the selected value byte-for-byte as `reasoning_effort` on
structured runs and every admitted continuation attempt. `thinking.type` is
fixed to `enabled`; upstream `disabled` is documented but withheld because no
qualified typed control or continuation-safe private-replay proof exists.

Prepared input, capability constraint, immutable plan, prepared evidence,
configured driver, request policy, and wire body agree on one exact selection.
The selected value remains fixed across initial, tool-result, later-turn, and
fresh-restoration paths. DeepSeek `reasoning_content` remains bounded,
adapter-held replay state and is never consumer output or durable session
material. `medium`, `xhigh`, provider aliases, alternate models, mismatches,
and unaccepted cache posture fail before provider work.

This evidence proves local dispatch and acceptance boundaries only. It does not
claim provider acceptance, effective reasoning depth, account entitlement,
balance, or live-model behavior.

## Evidence

- Research: [186 DeepSeek Reasoning Control Evidence](../research/186-deepseek-reasoning-control-evidence.md)
- Guide: [DeepSeek Prepared Integration](../guides/deepseek-prepared-integration.md)
- Cards: [104 evidence](../roadmaps/g04/batch-cards/104-deepseek-reasoning-evidence.md), [105 binding](../roadmaps/g04/batch-cards/105-deepseek-reasoning-binding.md), [106 acceptance](../roadmaps/g04/batch-cards/106-deepseek-reasoning-acceptance.md)
- Official API: <https://api-docs.deepseek.com/api/create-chat-completion/> (`452902008200767f318c8353cc225fca241777d8cd3f0b764fb94ffa7a612dea`)
- Official Thinking Mode: <https://api-docs.deepseek.com/guides/thinking_mode/> (`d9c7bf018583b542431aa91c995ee64a8c7aa3df32286c10634a04d2e1661982`)
- Official Tool Calls: <https://api-docs.deepseek.com/guides/tool_calls/> (`d50d330bc0e1f30b84ee804d77f1ad3f7073e5d2557a6a80545d2d2696ea2471`)
- Official Models and Pricing: <https://api-docs.deepseek.com/quick_start/pricing/> (`c6db3039404a4d108cf3c4e9b6c1891e4f446a6e8e8a3cfdf3c1eff739e194e6`)

## Shared-surface delta for orchestrator

The worker intentionally leaves the following shared surfaces unchanged. Apply
these deltas only after review and merge, in the fixed Ollama → Anthropic →
DeepSeek integration order:

- `docs/architecture/system-architecture.md`: extend the realized DeepSeek
  direct-inference bullet to name exact V4 Pro `low`/`high`/`max` reasoning
  selection, fixed enabled thinking, and private continuation replay without
  claiming effective depth or exposing reasoning bytes. No new contract,
  facade revision, route, or dependency direction is required.
- `docs/guides/provider-route-matrix.md`: update the
  `deepseek.continuation` row to describe typed exact `low`/`high`/`max`
  dispatch, fixed `thinking=enabled`, pre-network alias rejection, and the
  existing context-losing consumer-owned continuation boundary.
- `docs/guides/provider-solution-feature-matrix.csv`: update the DeepSeek
  Open Platform continuation notes for the same exact reasoning-selection and
  thinking posture. Keep the existing `reasoning_selection=Yes` cell and do
  not promote V4 Flash or effective-depth evidence.
- `CHANGELOG.md`: add an Unreleased DeepSeek bullet for typed exact
  `low`/`high`/`max` reasoning dispatch on the existing V4 Pro facade, fixed
  enabled thinking, private replay preservation, and the absence of provider
  acceptance/effective-depth claims; reference Research 186 and g04.038.
- `docs/roadmaps/g04/per-route-feature-completion.md`: mark the DeepSeek
  reasoning-control family integrated after merge and retain xAI as the next
  route family.
- `docs/roadmaps/g04/README.md`, `docs/roadmaps/README.md`, shared research,
  log, roadmap, and batch-card indexes, and the sole `## Next Task` pointer:
  update statuses and current handoff only after the fixed integration order
  is reconciled. No worker-side index edit is needed; pre-indexed Research 186
  and this closeout path remain valid.
- `release-baselines/public-api-0.3.3/packages.txt`: no package-set delta.
  `release-baselines/public-api-unreleased/swallowtail-adapter-deepseek.txt`
  records the additive `DeepSeekPreparedEvidence::reasoning_mode` accessor so
  the exact prepared selection remains inspectable. `effigy package:api`
  passes with that package-local API addition.

## Validation

Passed for the review fix:

- `cargo fmt -p swallowtail-adapter-deepseek`
- `cargo test -p swallowtail-adapter-deepseek --locked -- --test-threads=1`
- `effigy validate:focused swallowtail-adapter-deepseek`
- `effigy package:verify-affected swallowtail-adapter-deepseek`
- `effigy check:examples`
- `effigy qa:routes`
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `effigy package:api`
- `git diff --check`

No live DeepSeek request, account inspection, or provider-state mutation is
part of this closeout.
