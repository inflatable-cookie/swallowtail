# 2026-08-23 g04.049 OpenAI Background Service Tier Closeout

Status: complete; PR updated after review
Owner: Tom
Milestone: g04.049
Cards: 136-138 complete
Branch: `t3code/read-background-service-tier-handoff`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-12466bfd`
Base: `8d49f7049e4372fc304580a5f75ce7d77983ca45` (`origin/main` at dispatch)
PR: [#48](https://github.com/inflatable-cookie/swallowtail/pull/48)
Head: `efa1e5eb`
Review: orchestrator asked to reclassify explicit `default`; that subset is now bound
Merge: none; worker must not merge

The launcher-provided worktree and branch differ from the handoff
placeholders. They were used as supplied.

## Outcome

Research 196 now admits one deliver-now row: explicit Responses
`service_tier: "default"` as dispatch-only on ordinary attached runs and one
in-process reattachment. Official docs distinguish `default` from `auto`.
There is no documented enrollment gate. Contract 040 allows qualified dispatch
without claiming acceptance or effective value.

Cards 137-138 bind that subset through `OpenAiBackgroundServiceTier::standard()`,
optional prepared input, evidence, driver, and create encoding. Omission keeps
the prior fixture bytes. Detachment plus `default` rejects before effects.
Reconciliation does not restore a selected tier. Returned `service_tier` is
ignored. `auto`, `flex`, `priority`, `fast`, `ultrafast`, and `scale` are not
constructible.

Current executable facade is
`openai-responses-background-2026-08-23-service-tier` with private behavior
`openai.responses-background-v3`. Claim id stays
`openai.responses-background-window-1` as one Maintained exact segment. The
2026-08-23 / v2 point is superseded proof and is not executable.

## Route-Local Surfaces

Changed:

- `crates/swallowtail-adapter-openai/**`: typed standard-tier selection,
  preparation rejection with detachment, create encoding, facade/behavior
  mint, fixtures, and tests
- `docs/guides/openai-background-prepared-integration.md`
- `docs/research/196-openai-background-service-tier-evidence.md`
- `docs/roadmaps/g04/049-openai-background-service-tier.md`
- cards 136-138
- `release-baselines/public-api-unreleased/packages.txt` and
  `swallowtail-adapter-openai.txt`
- this closeout log

## Shared-Surface Closeout

Recorded here for orchestrator merge closeout; not applied on this branch:

- architecture and route/feature matrices should name dispatch-only explicit
  `default` on ordinary attached + one in-process reattachment, withheld
  detachment/reconciliation, and facade
  `openai-responses-background-2026-08-23-service-tier`
- `crates/swallowtail-testkit/tests/fixtures/direct-activity-applicability.json`
  still lists qualified `openai-responses-background-2026-08-23`
- `CHANGELOG.md` should record the additive dispatch-only selection and the
  new opaque facade point
- programme/front doors and the sole Next Task stay on the dispatch text
  until orchestrator merge closeout
- batch-card, research, and log indexes still describe card 136 as ready /
  Research 196 as reserved until orchestrator updates them
- Contract 029 currentness remains in its standing lane

## Validation

Passed:

- `cargo fmt -p swallowtail-adapter-openai`
- `effigy validate:focused swallowtail-adapter-openai` — 62 tests passed
- `effigy package:verify-affected swallowtail-adapter-openai`
- `effigy check:examples`
- `effigy qa:routes`
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `effigy package:api` — 40 package APIs passed
- `git diff --check`

`effigy doctor` reproduces the inherited baseline: 374 god-file findings
(329 warnings, 45 errors) and one generated-in-src warning. This lane added
no `PAPERCUTS.md` entry.

## Unresolved

Withheld until later exact evidence: `auto`, `flex`, `priority`, `fast`,
`ultrafast`, `scale`, returned-tier observation, and selected-tier retention
across detachment or restart reconciliation. No live provider proof was
attempted. Contract 029 currentness was not changed.
