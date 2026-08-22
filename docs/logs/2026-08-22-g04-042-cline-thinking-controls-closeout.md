# 2026-08-22 g04.042 Cline Thinking Controls Closeout

Status: worker complete; evidence stop; awaiting orchestrator review
Owner: Tom
Milestone: g04.042
Cards: 116 complete; 117-118 blocked
Branch: `t3code/review-thinking-controls-handoff`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-a6424c6d`
Base: `019b2c94c362bdda3675afad5ae579a8ef5add2c` (`origin/main` at dispatch)
PR: [#41](https://github.com/inflatable-cookie/swallowtail/pull/41)
Worker implementation head: `c3293090`
Review: changes requested on `4d2ca7ec`; headless acknowledgement truth corrected
Merge: none; the worker did not merge

Card 116 froze the current official Cline pages and exact `cline@3.0.55`
package evidence. Research 190 admits no deliver-now portable reasoning row on
either transport. Cards 117-118 were not executed. The Cline adapter, its
fixtures, both route guides, and the unreleased package API baseline are
unchanged. No install, login, credential or account inspection, config
mutation, live catalogue, provider prompt, or Cline process was used.

## Evidence Stop

The published `cline@3.0.55` tarball is the 6-file Node wrapper Research 146
and 147 already froze (`7eec2ad8…`). Its `README.md` is byte-identical to
`apps/cli/README.md` at GitHub commit `ad442cbb6a81d21773ceabc1398ea5eb58170718`
(`94c3c1b2…`), and `apps/cli/package.json` there is `@cline/cli@3.0.55`. That
binds the tagged source to the published package point without extracting or
executing a platform binary.

Three current official descriptions of omission disagree. The packaged README
says thinking is off when the flag is omitted; the CLI reference says
`(default medium)`; the source's own commander description says omission leaves
the provider default. Exact source settles it: omission consults persisted
provider reasoning and otherwise leaves both fields undefined, so a model or
provider default applies. `medium` comes only from pre-parse argv rewriting of
a bare `--thinking`, not from a commander default. `--reasoning-effort` is an
upstream alias rewritten to `--thinking`. The same commit's own e2e test
expects help text `--thinking [level]` while `program.ts` declares
`--thinking <level>`, so help spelling is not usable evidence at this point.

`cline.acp` parses and validates the flag, then discards it. `main.ts` calls
`runAcpMode({ autoApproveTools })` and returns before any thinking-bearing
config exists, and `AcpAgent.buildConfig` hard-codes `thinking: false` and
never sets `reasoningEffort`. No ACP request, session config option, or
documented ACP environment variable carries a level, and the current official
ACP page contains no thinking text. A valid level is accepted and silently
ignored; only an invalid level changes behavior, by exiting 1.

`cline.headless` does apply the selection to its one run config, and an
explicit CLI level beats persisted provider settings. That is where the
invariance ends. The route passes neither `-m` nor `-P`, so the model comes
from `args.model ?? persisted ?? knownModelIds[0] ?? "anthropic/claude-sonnet-4.6"`
against a resolved catalogue, and the provider defaults to `cline`. The
resolved value is then clamped to the nearest advertised tier
(`normalizeReasoningEffort`), substituted for models with no advertised
controls (`xhigh` and `max` become `high`), removed when the model advertises an
empty control list, or converted into a derived token budget when the model
advertises `budget_tokens`. An explicit off survives only when the model
supports off, and is dropped outright for Cline-provider Claude Fable models
because reasoning is mandatory there.

The selected headless argv returns no acknowledgement of the value at all.
`run_agent.ts` calls `printModelProviderInfo` — the sole emitter of the
`run_start` envelope and its `thinking` field — only inside `if (config.verbose)`,
and `cline.headless` passes no `-v/--verbose`. `--verbose` is an unselected
surface, and selecting it would not help: `run_start.thinking` is a boolean
that cannot distinguish `low` from `medium`, `high`, or `xhigh`. Dispatch
therefore stops at argv, with no acceptance tier available.

Upstream `none` is therefore not exact portable `off` on either route.
`cline.acp` fails at dispatch; `cline.headless` fails at model qualification.
Sharing `cline@3.0.55` did not share a claim.

No behavior revision is needed. `cline.acp.stdio-v1`,
`cline.headless.stdio-json-v1`, the `cline.package` `3.0.55` qualified point,
and the compatibility claim ceiling are unchanged. Contract 029 currentness
stays in its standing lane.

## Changed Route-Local Surfaces

- `docs/research/190-cline-thinking-control-evidence.md`: promoted stop;
  official and exact-package digests; resolved omission contradiction;
  independent ACP/headless parse, application, precedence, and lifetime;
  route/value disposition table; Contract 040 decision
- `docs/roadmaps/g04/042-cline-thinking-controls.md`: stopped after card 116
- `docs/roadmaps/g04/batch-cards/116-cline-thinking-control-evidence.md`:
  complete
- `docs/roadmaps/g04/batch-cards/117-cline-thinking-control-binding.md`:
  blocked
- `docs/roadmaps/g04/batch-cards/118-cline-thinking-control-acceptance.md`:
  blocked
- this closeout log

Unchanged: `crates/swallowtail-adapter-cline/**`, both Cline route fixtures,
`docs/guides/cline-acp-prepared-integration.md`,
`docs/guides/cline-headless-prepared-integration.md`, and the unreleased
public-API baseline. The guides are silent on thinking today and stay accurate
without an edit; card 118, which owns them, did not execute.

## Shared-Surface Closeout Delta

Deferred to the orchestrator after merge. None of these were edited on the
worker branch:

- `docs/architecture/system-architecture.md`: record that Cline thinking was
  evidenced on exact `3.0.55` and withheld on both transports — ACP discards
  the flag, headless is model-entitled with no selected model.
- `docs/guides/provider-route-matrix.md` and
  `docs/guides/provider-solution-feature-matrix.csv`: keep `cline.acp` and
  `cline.headless` `reasoning_selection` as `No`, with separate reasons per
  route.
- `CHANGELOG.md`: unchanged; no feature shipped.
- `docs/roadmaps/g04/per-route-feature-completion.md`: mark g04.042 stopped /
  withheld, not delivered.
- `docs/triage/2026-08-21-advanced-route-features.md`: record that Cline
  thinking is no longer a deliver-now candidate on this package point.
- `docs/roadmaps/README.md` and `docs/roadmaps/g04/README.md`: move the sole
  Next Task off g04.042 to the next selected route family.
- `docs/roadmaps/g04/batch-cards/README.md`, `docs/research/README.md`, and
  `docs/logs/README.md`: refresh status text only. Research 190 and this log
  remain pre-indexed.
- `release-baselines/public-api-*/packages.txt`: no change.
- matrix-assertion tests: no change expected while both cells stay `No`.

## Validation

Passed:

- `cargo fmt -p swallowtail-adapter-cline`
- `effigy validate:focused swallowtail-adapter-cline`
- `effigy package:verify-affected swallowtail-adapter-cline`
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

`effigy doctor` reproduces the inherited baseline unchanged: 371 god-file
findings (326 warnings, 45 errors) plus one generated-in-src warning. This lane
created no new finding and no `PAPERCUTS.md` entry.

Card 117 and 118 binding-only gates had no subject: there is no typed input,
plan constraint, prepared evidence, driver binding, or argv delta to exercise.
There is no package API or example delta.

## Unresolved

A later lane may reopen `cline.headless` only with an exact selected-model
route, or an upstream point that applies a named tier without model entitlement
and reports the applied tier. `cline.acp` may reopen only if a later package
point carries a thinking selection into the ACP child. Cline model, provider,
plan mode, compaction, retries, timeout, permissions, tools, teams, hub,
worktree, and session load/resume remain out of scope. Contract 029 currentness
for `cline.package` stays in its standing lane and was not touched.
