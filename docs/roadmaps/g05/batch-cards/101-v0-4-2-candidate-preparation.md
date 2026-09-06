# 101 v0.4.2 Candidate Preparation

Status: planned; serial after card 100; operator prepare authorization granted 2026-09-05 on the v0.4.1 standing-grant pattern (one transaction per attempt, renewable by Chatterbox on a captured transient failure; a real defect stops the lane)
Owner: Tom
Created: 2026-09-05
Updated: 2026-09-05
Milestone: `../032-v0-4-2-release-readiness.md`
Depends on: the preceding g05.032 card

## Goal

Prepare the `0.4.2` candidate on the g05.030 precedent, compressed: read-only status, one Effigy prepare transaction with the updated per-gate logs, release note authored from the card 100 result and merged tranches, candidate PR with review and exact-SHA CI in parallel, merge. Needs the operator's prepare authorization (the standing-grant pattern from `v0.4.1` is requested).

## Readiness

Chatterbox makes this card ready with its manifest when the preceding card
merges.

## Result

The candidate preparation stopped at the first real release-gate defect. The
read-only status was ready for `0.4.2`: current `0.4.1`, inferred next
`0.4.2`, patch bump, 40 locked workspace packages, and 11 configured gates.

Exactly one authorized command ran:

```text
effigy --json release prepare --yes --check-gates --version 0.4.2
```

The JSON report is retained at
`.effigy/reports/release/prepare-0.4.2.json`. `fmt`, `lint`,
`lint:no-features`, and `test` passed; `test` ran 3130 tests across 274
binaries with 24 skipped. The `qa` gate failed after 1.2 seconds. Its retained
log is `.effigy/reports/release/gates/qa.log`; the defect is the missing
`docs/logs/README.md` Markdown index entry for
`2026-09-06-g05-032-card-103-closeout.md`. The other per-gate logs are in
`.effigy/reports/release/gates/`, with the run environment in
`.effigy/reports/release/gates/environment.json`.

Effigy rolled back its three prepare mutations (`Cargo.toml`, `CHANGELOG.md`,
and `Cargo.lock`); `.release-prepared.json` was not written. Only the authored
`docs/releases/0.4.2.md` and `docs/releases/README.md` remain changed. Card 103
is fixture-only and merged at
`d7b483dd9d850fb0f6f3f04e4297e1bf7662333b`, but its log index entry is a
coordinator-owned shared closeout surface. No retry, baseline generation,
script repoint, candidate PR, review, CI dispatch, merge, or tag action was
performed. The lane stops here pending the shared index correction and a new
coordinator dispatch.
