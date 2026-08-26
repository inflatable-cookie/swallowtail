# 2026-08-26 g04.073 Cline Headless Plan Mode Closeout

Status: complete
Owner: Tom
Milestone: g04.073
Cards: 201-203
Research: 220

## Outcome

Delivered portable `HarnessMode::Plan` on route `cline.headless` for exact
`3.0.55`. No behavior revision: `cline.headless.stdio-json-v1` is unchanged.
Selection is optional on `ClineHeadlessRunProfileInput`. Canonical argv places
`--plan` after `--auto-approve false` and before `-c <cwd> <prompt>`. Omission
keeps `--json --auto-approve false -c <cwd> <prompt>` and is not implicit Plan.

Provider `act|yolo|zen` stay unselected. ACP `--plan` stays out. The JSON child
does not register `switch_to_act_mode`. Plan is prompt, tool-preset, and
`run_commands` blacklist behavior, not filesystem, network, shell, process,
sandbox, or descendant containment. Isolation stays `AmbientHost`. `--auto-approve
false` and read-only working-resource policy stay independent. Effective JSON
mode observation is withheld.

## Evidence

- Research 220 promoted one deliver-now row on exact package `3.0.55`, tag
  `cli-v3.0.55`, commit `ad442cbb6a81d21773ceabc1398ea5eb58170718`
- command, driver, and prepared-facade fixtures cover omit argv, canonical
  `--plan` placement, Plan-without-capability rejection before spawn, instance
  advertisement, input/plan/policy/argv agreement, and prompt secrecy
- Cline headless guide, route matrix, feature-matrix notes, architecture, and
  triage distinguish Plan dispatch from isolation and auto-approve

## Validation

Focused Cline adapter validation, affected-package verification, examples,
unreleased API, Northstar, research/logs/roadmap indexes, next-action, and
`git diff --check` passed. Additive Cline public API is recorded in
`public-api-unreleased`; `public-api-0.3.3` stays immutable.

Doctor remains an inherited error: `scan.god-files` plus generated-in-src.
This run also scanned untracked local `.evidence/cline-3.0.55` sources, which
inflates the finding count and is not part of the PR. This lane does not
repair god-files.

## Generation Boundary

g04.073 closes only this route-local family. g04 remains open for the next
per-route inventory reassessment unless the operator supplies a different
direction. Contract 029 currentness stays standing. Do not merge from the
worker thread.
