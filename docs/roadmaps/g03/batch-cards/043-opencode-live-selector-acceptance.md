# 043 OpenCode Live Selector Acceptance

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../016-attached-harness-probe-compatibility-truth.md`
Depends on: card 042

## Goal

Accept the repaired gated target and return to compatibility maintenance
without requiring an operator-started server.

## Acceptance Criteria

- [x] the feature-gated test target compiles and deterministic cases pass
- [x] focused OpenCode validation passes
- [x] the Effigy live selector remains explicit and separately gated
- [x] roadmap, card, research, and log indexes reconcile
- [x] docs, Northstar, and diff hygiene pass
- [x] no prompt, authentication, session, server-start, or consumer effect runs

## Validation

- `cargo test -p swallowtail-adapter-opencode --features live-probes --test installed_probe`
- `effigy validate:focused swallowtail-adapter-opencode`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- do not run `effigy probe:opencode-installed` without an explicit
  operator-started endpoint

## Auto-Continuation

No. Return to the g03 compatibility-maintenance checkpoint.

## Evidence

- feature-gated target: four passed, one network test ignored
- focused OpenCode validation: 82 passed
- docs, Northstar, and diff-hygiene checks passed
- no live endpoint or provider effect ran
