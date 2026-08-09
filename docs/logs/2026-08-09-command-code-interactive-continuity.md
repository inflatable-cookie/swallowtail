# Command Code Interactive Continuity And Catalogue Disposition

Date: 2026-08-09
Roadmap: g03.060, g03.061
Card: 187, 188

## Outcome

`command-code.headless` now owns Contract 043 interactive continuity beside
structured `--no-session` runs. First turn retains a project-scoped session;
later turns privately pass only exact `--resume <sessionId>`. Ambient
`--continue` and `--fork-session` stay rejected. No public 017 load/resume
binding is minted.

Catalogue, import, and provider export remain unsupported on npm `1.15.1`.
TTY `/export` / `/sessions` and `~/.commandcode/projects` scanning are not
Swallowtail surfaces. Promotion waits for a non-interactive machine
list/export. Provider API stays deferred by operator choice.

## Live Finding

Operator-gated `effigy probe:command-code-interactive` with
`SWALLOWTAIL_LIVE_COMMAND_CODE_MODEL=deepseek/deepseek-v4-flash` completed two
plan-mode turns in one working resource with clean turn and session cleanup.

## Validation

- `effigy validate:focused swallowtail-adapter-command-code` — 29 passed
- `effigy package:verify-affected swallowtail-adapter-command-code` — passed
- `effigy qa:guides` — passed
- `effigy qa:routes` — 69 operations, 43 available, 35 production routes
- `effigy probe:command-code-interactive` — passed

No version bump, tag, GitHub Release, or registry mutation ran.

## Next

Return to the operator for the next lane. Catalogue/export reopen only on a
qualified non-TTY list/export surface.
