# 156 Validation Runtime Inventory And Budgets

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../046-validation-latency-and-proof-routing.md`

## Goal

Freeze the current validation graph, duplicated work, and evidence tiers before
changing Effigy selectors or package scripts.

## Scope

1. Inventory Effigy validation tasks and their underlying commands.
2. Map focused, workspace, package, candidate, and release evidence overlap.
3. Record warm-path timings and known cold package-compilation costs without
   repeatedly running the full suite.
4. Identify safe shared caches and unsafe isolation boundaries.
5. Set budgets and select the smallest implementation tranche.

## Acceptance Criteria

- [x] every active validation task has an evidence tier and owner
- [x] duplicated test, compile, package, and docs work is explicit
- [x] warm and cold cost evidence is sufficient to select changes
- [x] package isolation and release-only gates remain mandatory where required
- [x] implementation seams and budgets are concrete
- [x] no validation behavior changes

## Validation

- Effigy task graph inspection
- script and Cargo topology inspection
- existing timing evidence plus one bounded warm-path measurement
- docs QA
- `git diff --check`

## Stop Conditions

- Do not run the full workspace test suite only to obtain a timing.
- Do not change selectors or scripts in this card.
- Stop if an evidence tier would require product policy.

## Auto-Continuation

Yes. Continue to card 157 when the implementation tranche is contract-ready.

## Evidence

- Effigy task reports provide 84 to 512 observations for common static and
  workspace selectors, plus smaller package and candidate samples.
- Historical successful `test:rust` duration ranges from 75 milliseconds to
  56.3 minutes. `qa` reaches 20.1 minutes; package and candidate selectors
  reach 7.1 to 16.7 minutes.
- The card-155 four-package path took about 10 seconds for focused tests, 9.3
  seconds for focused clippy, and 22.4 seconds for four separately targeted
  extracted-package checks.
- One bounded warm route-matrix measurement passed in 0.13 seconds. No Cargo
  or workspace suite was rerun for this card.
- The inventory assigns every selector to change-author, milestone, package,
  release-operator, or live-evidence ownership.
- Card 157 is bounded to explicit package arguments. It adds no changed-file
  inference and does not alter full QA, package, candidate, consumer, MSRV, or
  live-probe gates.
- Concurrent observable-activity work introduced one Kimi error-level size
  finding after card 155. It does not overlap this inventory or change the
  accepted card-155 baseline.
