# 187 Command Code Interactive Acceptance

Status: completed
Owner: Tom
Created: 2026-08-09
Milestone: `../060-command-code-interactive-continuity.md`
Depends on: card 186

## Goal

Publish interactive continuity truth and prove one live two-turn plan-mode
session in a single working resource.

## Scope

1. Update guide, feature matrix, route matrix, and activity truth for
   interactive continuity without claiming catalogue or export.
2. Add operator-gated live probe for first turn + exact resume.
3. Run focused, affected-package, guides, and routes validation.

## Acceptance

- [x] matrices distinguish structured `--no-session` from interactive retention
- [x] live two-turn probe passes with exact resume and clean cleanup
- [x] closeout does not claim 017 public load/resume or 046 catalogue/export

## Evidence

- Guide, feature/activity/route matrices, and lifecycle row state private
  exact-id continuity without public load/resume or catalogue/export.
- `effigy probe:command-code-interactive` with
  `SWALLOWTAIL_LIVE_COMMAND_CODE_MODEL=deepseek/deepseek-v4-flash` completed
  two plan-mode turns in one working resource.

## Validation

- `effigy validate:focused swallowtail-adapter-command-code` — 29 passed
- `effigy package:verify-affected swallowtail-adapter-command-code` — passed
- `effigy qa:guides` — passed
- `effigy qa:routes` — passed
- operator-gated interactive live probe — passed

## Stop Conditions

- stop before any version bump, tag, or registry mutation
- stop if live resume requires a different working directory than the first turn

## Auto-Continuation

Continue to g03.061 card 188 for catalogue/export disposition.
