# 101 v0.4.2 Candidate Preparation

Status: planned; serial after card 100; operator prepare authorization granted 2026-09-05 on the v0.4.1 standing-grant pattern (one transaction per attempt, renewable by Chatterbox on a captured transient failure; a real defect stops the lane)
Owner: Tom
Created: 2026-09-05
Updated: 2026-09-06
Milestone: `../032-v0-4-2-release-readiness.md`
Depends on: the preceding g05.032 card

## Goal

Prepare the `0.4.2` candidate on the g05.030 precedent, compressed: read-only status, one Effigy prepare transaction with the updated per-gate logs, release note authored from the card 100 result and merged tranches, candidate PR with review and exact-SHA CI in parallel, merge. Needs the operator's prepare authorization (the standing-grant pattern from `v0.4.1` is requested).

## Readiness

Chatterbox makes this card ready with its manifest when the preceding card
merges.

## Result

The fourth prepare transaction completed green after the coordinator corrected
the third QA defect. The worker branch was rebased onto the coordinator's
`origin/main` at `d3304577a9a9cb00194000995a345a4d0c3d598e`, which includes the
Card 103 complete status, fixture-only Result, and completed roadmap indexes.
The preserved release note, release index entry, and Card 101 Result remain
scoped to this card.

The fresh read-only status report is retained at
`.effigy/reports/release/status-0.4.2-attempt-4.json` (SHA-256
`877bc5a44916c195b6300a6d432a0757d769503d209b087ad34205a5148e13c4`). It
inferred `0.4.2` as a patch from current `0.4.1`, with 40 locked workspace
packages, 11 configured gates, `ready: true`, and `checked: false`.

Attempt 1 (before the coordinator's shared index correction) ran exactly one
authorized command:

```text
effigy --json release prepare --yes --check-gates --version 0.4.2
```

Its JSON is retained as
`.effigy/reports/release/prepare-0.4.2-attempt-1.json` (the original report is
also retained at `.effigy/reports/release/prepare-0.4.2.json`). `fmt`, `lint`,
`lint:no-features`, and `test` passed; `test` ran 3130 tests across 274
binaries with 24 skipped. The `qa` gate failed after 1.2 seconds because
`docs/logs/README.md` lacked the Markdown index entry for
`2026-09-06-g05-032-card-103-closeout.md`. Its preserved gate log is
`.effigy/reports/release/attempt-1-gates/qa.log`; all first-attempt logs and
the environment are retained under `.effigy/reports/release/attempt-1-gates/`.

Attempt 2 (fresh attempt after that correction) ran exactly the same authorized
command once. Its JSON is retained at
`.effigy/reports/release/prepare-0.4.2-attempt-2.json` (SHA-256
`3ef3df09e4d1e7e32a7f38f887c1242d5f4dcfdde13b8d00aefa0b16ed876c18`). `fmt`,
`lint`, `lint:no-features`, and `test` passed again; `test` again ran 3130 tests
across 274 binaries with 24 skipped. Docs links and all docs indexes passed,
including the corrected logs index. The `qa` gate then failed after 1.3
seconds because roadmap status drift rejects Card 103's combined status value:

```text
docs/roadmaps/g05/batch-cards/103-opencode-cancellation-cleanup-delete-dispatch.md
has unrecognized Status 'review; fixture-only verdict, no `v0.4.2` content'
```

The fresh `qa` log is retained at
`.effigy/reports/release/attempt-2-gates/qa.log` (SHA-256
`b30792c21530d8bab31e7b93420805e5195f6a87bb443979d59ff926c3f3d8ce`); all
fresh per-gate logs and the environment are retained under
`.effigy/reports/release/attempt-2-gates/`. This is a real defect in the
Card 103 roadmap status surface, so the worker performs no further prepare
attempt and does not edit that card.

Attempt 3 (fresh attempt after the Card 103 status correction) ran exactly the
same authorized command once. Its JSON is retained at
`.effigy/reports/release/prepare-0.4.2-attempt-3.json` (SHA-256
`5b34f061afc5eae880af931c4fcef788700a6a3e2b92d971adc11ba5bb59cb71`). `fmt`,
`lint`, `lint:no-features`, and `test` passed again; `test` again ran 3130 tests
across 274 binaries with 24 skipped. Docs links and all docs indexes passed,
including the logs index. The `qa` gate then failed after 1.4 seconds because
roadmap status drift found Card 103 in the wrong index bucket:

```text
batch card 103-opencode-cancellation-cleanup-delete-dispatch.md Status bucket
is 'complete' but index lists it under 'ready'
```

The third-attempt `qa` log is retained at
`.effigy/reports/release/attempt-3-gates/qa.log` (SHA-256
`2dc73f773fa6110e9d8910afcb663e1a6c0761a208a62b609b1a008bb1c204d2`); all
third-attempt per-gate logs and the environment are retained under
`.effigy/reports/release/attempt-3-gates/`. This is a real defect in the
shared Card 103 roadmap index surface, so the worker performs no further
prepare attempt and does not edit that card or index.

Attempt 4 (fresh attempt after the Card 103 index reconciliation) ran exactly
the same authorized command once. Its JSON is retained at
`.effigy/reports/release/prepare-0.4.2-attempt-4.json` (SHA-256
`bb41cdc943c39359a9ba5e7eb3a28de9574f620bc0f9f778c9123ea9124dfe64`). All 11
gates passed: `fmt`, `lint`, `lint:no-features`, `test`, `qa`, `docs`,
`metadata`, `api`, `security`, `floor`, and `source`. The locked test gate ran
3130 tests across 274 binaries with 24 skipped. Effigy wrote
`.release-prepared.json` for `0.4.2`; its source fingerprint records the
prepared worker head before post-prepare release-surface edits.

The fourth-attempt per-gate logs and environment are retained under
`.effigy/reports/release/attempt-4-gates/`: `fmt.log`, `lint.log`,
`lint:no-features.log`, `test.log`, `qa.log`, `docs.log`, `metadata.log`,
`api.log`, `security.log`, `floor.log`, `source.log`, and `environment.json`.
No frozen-tree rerun was performed.

Effigy rolled back its three prepare mutations (`Cargo.toml`, `CHANGELOG.md`,
and `Cargo.lock`) on the first three attempts; the fourth attempt retained the
prepared `Cargo.toml`, `CHANGELOG.md`, and `Cargo.lock`. The generated 0.4.2
baselines contain 40 API package files plus `packages.txt`, 49 production
routes, and 88 internal dependency edges. The four versioned validation
references, consumer front door, and root release posture now point at
`0.4.2` while retaining the `v0.4.1` immutable baseline boundary.
Card 103 remains fixture-only and merged at
`d7b483dd9d850fb0f6f3f04e4297e1bf7662333b`. Card 100's live editing outcome
remains explicitly typed unresolved `ProviderFailed` evidence in the release
note and changelog; Card 102 may consume only an operator-authorized released
`v0.4.2` tag and commit. The Bovine Card 297 constraint is observed: no
Desktop request or untagged-main/`ffc26310` pin, and PR102 remains NO MERGE at
`80af896bbe7cfd213dcf11213c61a497ba637c7c`; its `v0.4.1` pin at
`c3cce750` is not treated as sufficient. `Codex discovery_spawn_failed` is
Desktop-owned and is not Card 101/102 content; remaining `0.153.3` is
`UnverifiedNewer` against the `0.152.1` ceiling.

No 0.4.2 candidate PR/head SHA, workflow-dispatch run ID, merge SHA, or tag
SHA exists yet. Candidate PR creation, independent review, and exact-SHA CI
dispatch are the remaining worker-loop steps. No tag, push-to-tag action,
Desktop request, Card 082 merge, credential/auth/permission change, consumer
pin, feature-code change, or workflow edit was performed.
