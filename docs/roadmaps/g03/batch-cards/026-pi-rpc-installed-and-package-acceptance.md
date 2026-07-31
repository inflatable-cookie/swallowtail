# 026 Pi RPC Installed And Package Acceptance

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../010-pi-rpc-installed-range-closure.md`
Depends on: card 025

## Goal

Accept the widened Pi range against the installed exact executable and close
the milestone without a provider prompt.

## Scope

1. Add or repair an explicit ignored installed-version selector.
2. Prove installed `0.83.0` output classifies as qualified.
3. Run focused and extracted-package proof.
4. Reconcile architecture, front doors, roadmap state, research index, and one
   meaningful log.

## Acceptance Criteria

- [x] installed output binds exact `0.83.0`
- [x] the selector is explicit, bounded, ignored by default, and prompt-free
- [x] no credential, host path, or raw provider output reaches diagnostics
- [x] focused, package, docs, and Northstar checks pass
- [x] Pi load/resume remains visibly blocked and unchanged
- [x] roadmap g03.010 closes with one clear next checkpoint

## Validation

- `effigy validate:focused swallowtail-adapter-pi`
- `effigy package:verify-affected swallowtail-adapter-pi`
- explicit Pi installed selector only
- `effigy qa:docs`
- `effigy qa:northstar`
- `cargo fmt --all --check`
- `git diff --check`
- no broad workspace suite

## Auto-Continuation

No. Return to the g03 provider-maintenance checkpoint after closeout.

## Evidence

- the bounded ignored selector classified installed exact `0.83.0`
- 41 focused Pi tests passed
- the extracted 85-file package compiled
- local-host discovery safely failed on the npm launcher's `/usr/bin/env node`
  dependency; Research 084 owns that separate portability gap
- no provider prompt, credential mutation, or workspace effect ran
