# 2026-08-22 g04.039 xAI Responses Controls Closeout

Status: merged; shared closeout complete
Owner: Tom
Milestone: g04.039
Cards: 107-109
Base: `origin/main` at `906e776d957b02b25b0d69fce71b28d6543af40f`
PR: [#38](https://github.com/inflatable-cookie/swallowtail/pull/38)
PR head at opening: `6ad12932`
Accepted and merged head: `e9ae1a49a90a32c9242eaec0b64d80c3050d2e40`
Merge: fast-forward-only onto `main`, 2026-08-22
Branch: `t3code/xai-responses-controls`

The exact accepted head passed all five CI jobs before `main` advanced. No merge
commit or red-head bypass was used.

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
- `CARGO_BUILD_JOBS=1 NEXTEST_TEST_THREADS=1 effigy validate:focused swallowtail-adapter-xai` — 40 tests passed
- `effigy package:verify-affected swallowtail-adapter-xai`
- `effigy check:examples`
- `effigy qa:routes`
- `effigy qa:northstar`
- research, logs, roadmaps, g04, batch-card index, and next-action gates
- xAI package API baseline diff
- `git diff --check`

`effigy doctor` remains blocked by the repository's inherited god-file scan
(371 findings: 326 warnings and 45 errors); the post-fix count matches the
`origin/main` baseline and no new doctor finding was introduced by this
route-local work. The doctor report also retains its existing graph-index and
generated-in-source warnings. The full workspace `effigy package:api` gate was
attempted in both the repository and an isolated target, but the host returned
`EMFILE` (`Too many open files in system`) while documenting unrelated
workspace/ACP crates. The xAI package baseline comparison passed exactly.

During orchestrator review the full workspace `effigy package:api` gate passed
at the exact accepted head. The earlier `EMFILE` result was host-transient, not
a route or public-API failure.

## Shared-Surface Closeout

The orchestrator applied the worker's deferred delta after merge:

- `docs/architecture/system-architecture.md`: record exact Grok rows and
  prepared dispatch boundaries without acceptance/effectiveness claims;
- `docs/guides/provider-route-matrix.md`: update the xAI Responses route
  control cells;
- `docs/guides/provider-solution-feature-matrix.csv`: update the xAI route
  reasoning/output-control rows and note exact-model qualification;
- `CHANGELOG.md`: add the unreleased route-local control binding;
- `docs/roadmaps/g04/per-route-feature-completion.md`: mark this control
  family integrated;
- shared roadmap front doors, indexes, and `Next Task`: g04.039 is complete and
  the next action is a one-family selection and g04.040 compilation pass;
- `release-baselines/public-api-0.3.3/packages.txt`: no package-set change;
  this worker added only the allowed unreleased xAI API baseline and package
  entry.

No live provider acceptance, account inspection, release, or publication is
claimed.
