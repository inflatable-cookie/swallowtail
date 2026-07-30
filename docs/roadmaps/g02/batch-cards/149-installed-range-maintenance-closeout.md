# 149 Installed Range Maintenance Closeout

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../044-installed-harness-range-maintenance.md`

## Goal

Close the Codex and OpenCode range tranche through cross-host, package, and
public integration truth.

## Scope

1. Run both adapters across local and remote-authoritative fixture hosts.
2. Assemble and compile affected package archives.
3. Refresh route, release-note, front-door, roadmap, and log truth.
4. Confirm later stable execution remains unverified rather than denied.
5. Leave one explicit stabilization checkpoint.

## Acceptance Criteria

- [x] both range extensions pass cross-host conformance
- [x] affected package archives assemble and compile
- [x] public route and release truth match exact claims
- [x] no provider prompt, consumer edit, or publication ran
- [x] roadmap g02.044 is honestly closed
- [x] one clear next task remains

## Validation

- focused adapter and shared compatibility suites
- affected extracted-package compile
- package metadata and public-API gates
- docs QA
- doctor delta review
- `git diff --check`

## Stop Conditions

- Stop if cross-host behavior differs.
- Do not replace the retained release candidate or publish.
- Do not widen any third provider range.

## Auto-Continuation

No. Return to the g02 stabilization checkpoint.

## Evidence

- focused Codex and OpenCode prepared-profile tests execute both authoritative
  host topologies
- final focused adapter evidence remains 128 Codex tests and 82 OpenCode tests
- the 11-test shared harness-activity suite passes with 14 harness routes, 20
  prepared activity profiles, and 51 OpenCode release points
- the 165,540-byte Codex archive has SHA-256
  `9c275e78664c431f2a71b441e831799e0fccdbc22c7b2f69dd9dfeb597a654a0`
- the 121,288-byte OpenCode archive has SHA-256
  `76e6fc4a84d4ac86f54acafbe0b08804a657ab1e8271ad78213fbeedcecfec28`
- both extracted archives pass all-target check and test compilation against
  local unpublished dependencies
- workspace all-target check, package metadata, public-API, route matrices,
  docs QA, and `git diff --check` pass
- doctor health passes; its known structural scan reports 141 findings:
  108 warnings and 33 errors
- no provider prompt, attached-server mutation, consumer edit, retained-
  candidate replacement, or publication ran
