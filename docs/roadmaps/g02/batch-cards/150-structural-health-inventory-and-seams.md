# 150 Structural Health Inventory And Seams

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../045-error-level-structural-health-stabilization.md`

## Goal

Freeze the exact error-level structural inventory and divide it into
behavior-preserving implementation batches.

## Scope

1. Confirm Effigy health separately from the structural scan.
2. Count error findings by crate and source kind.
3. Identify the five critical files.
4. Group remaining error files by coupled behavior and focused package gates.
5. Exclude warning-only work from this milestone.

## Acceptance Criteria

- [x] health passes before structural work
- [x] all 33 error findings are assigned once
- [x] critical, source, test, and script counts are explicit
- [x] every implementation batch has focused validation
- [x] public behavior and warning reduction remain out of scope
- [x] stop conditions cover concurrent functional changes

## Validation

- `effigy doctor`
- `cargo check --workspace --all-targets`
- exact scan-report inventory
- docs QA
- `git diff --check`

## Stop Conditions

- Stop if health fails for a non-structural reason.
- Do not edit provider behavior while selecting seams.
- Do not assign warning-only files to this milestone.

## Auto-Continuation

Yes. Continue to card 151 after focused docs validation.

## Evidence

- doctor health passes and structural scan remains the sole red surface
- scan total: 141 findings, 108 warnings, 33 errors
- error kinds: 23 source files, nine test files, one script
- critical files: Anthropic session driver, route-matrix script, Claude Agent
  prepared-facade test, Codex app-server test, OpenCode prepared cases
- concentration: eight OpenCode, six Codex, four Claude Agent, three runtime,
  and twelve findings across the remaining adapters and script
- one compile defect in the concurrent typed user-input example was repaired
  before inventory; focused runtime and workspace health checks pass
