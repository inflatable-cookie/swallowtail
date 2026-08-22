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
- Review-fix implementation commit: `09cbd50b` (pushed)
- Base/head before worker changes: `a4440b4f917aa065bcc1e946deabecad87841acf`
- Closeout metadata is pushed on the same PR branch.
- PR: https://github.com/inflatable-cookie/swallowtail/pull/40
- Reviewed head: pending orchestrator review
- Merge: not performed

## Changed route-local surfaces

- `crates/swallowtail-adapter-qwen/**`: exact preparation binding, private
  stream-json `initialize`/`set_effort` control handshake, repeated first and
  resumed-turn setup, fresh-replacement proof, absent-path preservation,
  cumulative control bounds, fixtures, and tests
- `release-baselines/public-api-unreleased/swallowtail-adapter-qwen.txt`
- `release-baselines/public-api-unreleased/packages.txt`: one
  operator-authorized Qwen registration required by the API gate
- `docs/research/189-qwen-headless-reasoning-effort-evidence.md`
- `docs/guides/qwen-headless-prepared-integration.md`
- g04.041 and cards 113-115

## Validation

Passed:

- `cargo fmt -p swallowtail-adapter-qwen`
- `effigy validate:focused swallowtail-adapter-qwen` — 48 tests passed
- `effigy package:verify-affected swallowtail-adapter-qwen`
- `effigy check:examples`
- `effigy qa:routes`
- `effigy qa:northstar`
- research, logs, roadmaps, g04, batch-card, and next-action index gates
- `effigy package:api`
- `effigy doctor` — inherited baseline 371 findings (326 warnings, 45 errors);
  no new Qwen god-file finding
- `git diff --check`

The operator authorized the one-line Qwen registration in the unreleased API
package list, resolving the handoff scope conflict. The package-specific
baseline is selected and `effigy package:api` passes.

## Review-fix truth

Contract 029 now keeps
`qwen-code.headless.v0.21.0-catalogue-filter` through `0.21.14` and binds the
exact `qwen-code.headless.v0.21.15-reasoning-control` revision at `0.21.15`.
The reasoning mapping is not claimed retroactively for `0.21.0..=0.21.14`.

The control prelude rejects cumulative output over `2 MiB`, more than `4096`
records, and unexpected control-response request IDs. Plan/evidence/driver
mismatches fail before process start. Ambient override and control-substitution
failures are observed after child startup and rejected before the user
message/provider prompt; they are not claimed to be pre-process failures.
Fresh-session restoration is exercised end to end: the replacement opens,
starts a turn, repeats the exact control handshake, and sends the user message
without a resume selector.

The review-fix implementation restores the inherited Effigy doctor baseline:
371 findings (326 warnings, 45 errors). No new papercut or threshold exception
was recorded.

Do not claim merge or edit shared architecture, matrices, changelog, programme,
front doors, indexes, matrix assertions, or package manifests on the worker
branch.
