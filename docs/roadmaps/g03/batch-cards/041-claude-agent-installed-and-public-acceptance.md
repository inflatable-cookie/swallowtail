# 041 Claude Agent Installed And Public Acceptance

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../015-claude-agent-0-64-standalone-range-maintenance.md`
Depends on: card 040

## Goal

Accept installed `0.63.0` and current exact artifact `0.64.0`, then reconcile
public truth and return to the maintenance checkpoint.

## Acceptance Criteria

- [x] installed `0.63.0` classifies as qualified
- [x] exact `0.64.0` artifact identity is accepted without installation
- [x] public matrices and architecture name the maintained ceiling
- [x] Gemini is deferred rather than paused
- [x] docs, Northstar, routes, package, and diff hygiene pass
- [x] no provider prompt, consumer edit, or publication runs

## Evidence

- `effigy probe:claude-agent-acp-managed` — exact `0.63.0`
- exact signed/npm `0.64.0` artifact accepted from frozen package and source
  digests without installation
- focused validation — 157 passed
- extracted package proof — two packages passed
- route, docs, Northstar, and diff-hygiene checks passed

## Validation

- `effigy validate:focused swallowtail-protocol-acp swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-protocol-acp swallowtail-adapter-claude-agent`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

No. Return to the g03 compatibility-maintenance checkpoint.
