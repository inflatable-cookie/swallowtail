# 032 Grok Build Installed And Package Acceptance

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../012-grok-build-0-2-117-range-maintenance.md`
Depends on: card 031

## Goal

Accept the widened Grok Build range against installed exact `0.2.117` and
close the milestone without authentication or a provider prompt.

## Scope

1. Add an explicit ignored installed-version selector using host-approved
   discovery.
2. Prove installed `0.2.117 (f1c06093089f) [stable]` classifies as qualified.
3. Run focused and extracted-package proof.
4. Reconcile architecture, route truth, front doors, roadmap state, and one
   meaningful closeout log.

## Acceptance Criteria

- [x] installed output binds exact `0.2.117` and its task-control behavior
- [x] selector is explicit, bounded, ignored by default, and prompt-free
- [x] no credential, host path, or raw provider output reaches diagnostics
- [x] focused, package, docs, and Northstar checks pass
- [x] no new public task-control, session, or authority claim appears
- [x] roadmap g03.012 closes with one clear next checkpoint

## Validation

- `effigy validate:focused swallowtail-adapter-grok`
- `effigy package:verify-affected swallowtail-adapter-grok`
- explicit Grok installed selector only
- `effigy qa:docs`
- `effigy qa:northstar`
- `cargo fmt --all --check`
- `git diff --check`
- no broad workspace suite

## Auto-Continuation

No. Return to the g03 compatibility-maintenance checkpoint after closeout.

## Evidence

- `effigy probe:grok-installed` classified exact installed `0.2.117` in `0.04s`
- 24 focused Grok tests passed in two seconds
- extracted 45-file package compiled in three seconds
- route, lifecycle, feature, activity, docs, Northstar, formatting, and diff
  checks passed
- no authentication, provider prompt, model request, or durable mutation ran
