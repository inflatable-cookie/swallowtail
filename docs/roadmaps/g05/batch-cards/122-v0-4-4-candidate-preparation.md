# 122 v0.4.4 Candidate Preparation

Status: planned; candidate prepared; hosted CI and independent review pending
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-07
Milestone: `../036-v0-4-4-release-readiness.md`
Depends on: ready on card 124's merge

## Goal

Prepare the `0.4.4` candidate on the simplified lane from `docs/guides/release-playbook.md`: cheap local gates, one prepare transaction, the qualifying hosted `workflow_dispatch` run as heavy evidence, candidate PR with review in parallel, merge, candidate SHA and run id to Chatterbox at once. Record the wall clock.

## Readiness

Ready on the condition in the status line; the manifest in the milestone
roadmap carries the full scope, owned paths, validation, and stop rules.

## Earlier stopped attempt (historical)

The clean canonical base was `02003ecb7dd18f96d2e66dd98c77604cdb970117`,
matching `origin/main`; the worktree was clean and the feature freeze was in
force. Read-only release status inferred `0.4.4` as a patch from `0.4.3`, with
all seven cheap gates green and no blockers. Status JSON SHA-256 is
`4896ada60aaf57886b761d1dc28b594523c4ddc11fbe9acf2577807ba2f7c54e`.

Exactly one authorized command ran, dispatched at approximately
`2026-09-07T15:08:52+0100`:

```text
effigy --json release prepare --yes --check-gates --version 0.4.4
```

`fmt` passed. `qa` stopped after `15.802s` because the prepared `0.4.4`
consumer front door still pins `v0.4.3` in `README.md`. This is a real
release-surface defect, so the prepare authorization is consumed and no retry
is allowed. Effigy rolled back its temporary version and lock mutations; no
`0.4.4` baselines or `.release-prepared.json` were written.

The failed JSON capture is retained at
`.effigy/reports/release/prepare-0.4.4.json`; executed gate evidence is under
`.effigy/reports/release/prepare-0.4.4-executed-gates/`. No candidate PR,
candidate head, qualifying hosted run, independent review, merge, tag, or
provider/live/auth/Desktop action exists.

## Final renewed attempt

The candidate was safely rebased onto exact `origin/main`
`9281d12a89ecca4efbfa0035617c7d44d5ee73ae`; the only main delta since the
previously observed `d988c67f` was the release-playbook baseline-preseed fix.
Before prepare, 314 files at `<=0.4.3` were hashed with aggregate
`05492770e3f03be79b8def8196abf46d141859fd6202f33a596564af3f77bc8c`, and the
snapshot is retained at `.effigy/reports/release/baselines-before-0.4.4.json`.
The 0.4.4 public-API, route, and internal-dependency sets were then materialized
as the only new baseline sets; their validated shapes are 40 package files
plus `packages.txt`, 49 routes, and 88 dependency edges. The route matrix
check passed, and the older-baseline aggregate remained unchanged.

The final renewed prepare dispatched at `2026-09-07T15:27:04+0100` after the
first stop at `2026-09-07T15:08:52+0100` and the second stop at
`2026-09-07T15:17:28+0100`. The exact command ran once and all seven cheap gates
passed. The successful JSON capture is retained at
`.effigy/reports/release/prepare-0.4.4-success.json` with SHA-256
`da037b70f979f072b062d624c011ab859b370c03fd96f7d2a68c8e6e874ce69c`;
per-gate evidence is under `.effigy/reports/release/gates/`. Candidate PR, hosted
`workflow_dispatch`, and same-workspace independent review remain pending;
there is no merge or tag authority.
