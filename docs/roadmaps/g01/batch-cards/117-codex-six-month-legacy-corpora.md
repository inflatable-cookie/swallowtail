# 117 Codex Six-Month Legacy Corpora

Status: completed
Owner: Tom
Updated: 2026-07-23
Milestone: `../039-installed-harness-compatibility-range-audit.md`

## Objective

Freeze exact January-to-April Codex exec and app-server behavior without
changing production claims.

## Scope

- exec checkpoints `0.80.0`, `0.81.0`, `0.84.0`, `0.94.0`, `0.98.0`,
  `0.99.0`, `0.100.0`, `0.110.0`, and `0.121.0`
- app-server checkpoints `0.80.0`, `0.81.0`, `0.84.0`, `0.94.0`, `0.99.0`,
  `0.100.0`, and `0.107.0`
- current `0.122.0` exec and `0.110.0` app-server boundary neighbors
- exact exclusion of unpublished `0.82.0`, `0.83.0`, `0.108.0`, and `0.109.0`
- source-generated historical schema fixtures record exact source commits and
  generation commands
- stable read-only app-server subset only
- ambient versus suppressed config and retained versus ephemeral exec cases
- no production descriptor or driver change

## Acceptance Criteria

- [x] every published boundary and behavior milestone has frozen evidence
- [x] generated and upstream-published schema artifacts remain distinguishable
- [x] v2 selected methods are proven at `0.80.0`
- [x] default and explicit stdio invocation remain separate
- [x] every exec argument set matches its exact behavior segment
- [x] unpublished and unknown points remain rejected
- [x] fixture diagnostics contain no private payload

## Validation

- eight focused current and legacy Codex corpus tests pass
- `effigy qa:docs` passes
- `git diff --check` passes
- `effigy doctor` retains the inherited seven errors and twelve warnings

## Auto-Continuation

Completed. Continue to card 118.
