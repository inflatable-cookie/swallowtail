# 2026-08-22 g04.041 Qwen Headless Reasoning Effort Closeout

Status: worker PR pending review
Owner: Tom
Milestone: g04.041
Cards: 113-115

The exact `0.21.15` evidence gate admitted two DashScope model rows and all five
canonical reasoning values. The route-local implementation and deterministic
acceptance coverage are complete in the selected worker worktree.

## Worker state

- Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-b71d0882`
- Branch: `t3code/review-qwen-reasoning-handoff`
- Implementation commit: `b20c5e84dd77c61df0521ad8ccb6e5bcc931267c`
  (pushed)
- Base/head before worker changes: `a4440b4f917aa065bcc1e946deabecad87841acf`
- Closeout metadata is pushed on the same PR branch.
- PR: https://github.com/inflatable-cookie/swallowtail/pull/40
- Reviewed head: pending orchestrator review
- Merge: not performed

## Changed route-local surfaces

- `crates/swallowtail-adapter-qwen/**`: exact preparation binding, private
  stream-json `initialize`/`set_effort` control handshake, repeated first and
  resumed-turn setup, absent-path preservation, fixtures, and tests
- `release-baselines/public-api-unreleased/swallowtail-adapter-qwen.txt`
- `release-baselines/public-api-unreleased/packages.txt`: one
  operator-authorized Qwen registration required by the API gate
- `docs/research/189-qwen-headless-reasoning-effort-evidence.md`
- `docs/guides/qwen-headless-prepared-integration.md`
- g04.041 and cards 113-115

## Validation

Passed:

- `cargo fmt -p swallowtail-adapter-qwen`
- `effigy validate:focused swallowtail-adapter-qwen` — 45 tests passed
- `effigy package:verify-affected swallowtail-adapter-qwen`
- `effigy check:examples`
- `effigy qa:routes`
- `effigy qa:northstar`
- research, logs, roadmaps, g04, batch-card, and next-action index gates
- `effigy package:api`
- `git diff --check`

The operator authorized the one-line Qwen registration in the unreleased API
package list, resolving the handoff scope conflict. The package-specific
baseline is selected and `effigy package:api` passes.

Do not claim merge or edit shared architecture, matrices, changelog, programme,
front doors, indexes, matrix assertions, or package manifests on the worker
branch.
