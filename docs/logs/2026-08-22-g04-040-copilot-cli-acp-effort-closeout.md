# 2026-08-22 g04.040 Copilot CLI ACP Effort Closeout

Status: worker-complete; awaiting review
Owner: Tom
Milestone: g04.040
Cards: 110 complete; 111-112 blocked
Branch: `t3code/copilot-cli-acp-effort`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-4f3ec348`
PR: [#39](https://github.com/inflatable-cookie/swallowtail/pull/39)
Implementation head: `b40637ec`

Card 110 froze official ACP-server documentation and exact `@github/copilot`
`1.0.80` package evidence. Research 188 admits no deliver-now portable
reasoning row. Cards 111-112 were not executed. Production claims, the Copilot
guide, and the package API baseline are unchanged. No login, install, account
inspection, or provider prompt was used.

## Evidence Stop

Exact `1.0.80` `app.js` registers `--effort` / `--reasoning-effort` as one
commander option whose choices come from native `reasoningEffortLevels()`.
ACP `session/new` still does not carry reasoning; it stores the CLI value as
session-initial effort. Entitled application is model-gated. Unsupported
values are rejected or replaced with the current model's default. Gemini
tables also name `minimal`, which is not in the official CLI flag table.

`copilot-cli.acp` has no selected model. Contract 040 therefore cannot map
any of `low`, `medium`, `high`, `xhigh`, or `max` without model inference,
clamping, or default substitution.

The one-child/one-session topology could keep spawn argv fixed. That does not
remove the model-capability gap.

## Changed Route-Local Surfaces

- `docs/research/188-copilot-cli-acp-effort-evidence.md`: promoted stop;
  official and package digests; syntax/value/lifetime table; no-model-route
  Contract 040 decision
- `docs/roadmaps/g04/040-copilot-cli-acp-session-effort.md`: stopped after
  card 110
- `docs/roadmaps/g04/batch-cards/110-copilot-cli-acp-effort-evidence.md`:
  complete
- `docs/roadmaps/g04/batch-cards/111-copilot-cli-acp-effort-binding.md`:
  blocked
- `docs/roadmaps/g04/batch-cards/112-copilot-cli-acp-effort-acceptance.md`:
  blocked

Unchanged: adapter, fixtures, Copilot CLI ACP guide, unreleased API baseline.

## Shared-Surface Delta For Orchestrator Closeout

Do not present this family as a shipped reasoning control. Suggested
orchestrator-only edits after merge:

- `docs/architecture/system-architecture.md`: record that Copilot CLI ACP
  server-start effort was evidenced on `1.0.80` and withheld because entitled
  values are model-gated and the route has no selected model.
- `docs/guides/provider-route-matrix.md` and
  `docs/guides/provider-solution-feature-matrix.csv`: keep
  `copilot-cli.acp` `reasoning_selection` as `No`; add the exact incompatible
  reason if the cell currently lacks it.
- `CHANGELOG.md`: no Unreleased feature entry. Optional research-stop note
  only if the orchestrator wants the withheld finding visible.
- `docs/roadmaps/g04/per-route-feature-completion.md`: mark g04.040 stopped
  / withheld, not delivered.
- `docs/triage/2026-08-21-advanced-route-features.md`: Copilot CLI ACP effort
  is no longer a deliver-now candidate on this package without a model route.
- `docs/roadmaps/README.md` and `docs/roadmaps/g04/README.md`: move Next Task
  off cards 110-112; pick the next remaining promoted family.
- `docs/roadmaps/g04/batch-cards/README.md`, `docs/research/README.md`, and
  `docs/logs/README.md`: refresh status text only. Research 188 and this log
  were pre-indexed.
- `release-baselines/public-api-0.3.3/packages.txt` and
  `release-baselines/public-api-unreleased/packages.txt`: no change.
- matrix-assertion tests: no change expected while the cell stays `No`.

## Validation

Passed:

- `effigy validate:focused swallowtail-adapter-copilot-cli` (28 tests)
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `git diff --check`

Inherited doctor baseline (371 god-file findings, generated-in-src) is unchanged.
No package API or example delta. Cards 111-112 gates were not run because there
is no binding.

## Unresolved

A later lane may reopen Copilot effort only with an exact selected-model
route or an upstream interface that accepts one value without model
entitlement. Tool filters, TCP, permissions, login, and model selection
remain out of scope. Merge is not claimed.
