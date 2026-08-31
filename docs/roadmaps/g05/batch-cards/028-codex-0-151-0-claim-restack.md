# 028 Codex 0.151.0 Claim Restack

Status: completed
Owner: Tom
Created: 2026-08-31
Updated: 2026-08-31
Milestone: `../012-codex-0-151-0-useful-newer.md`
Depends on: completed card 027 with an admitted Contract 029 segment

## Goal

Restack PR 130's Codex `0.151.0` production claim on top of card 027, validate
current main, review the exact head, and merge when clean.

## Scope

1. Apply only the admitted exec and app-server range changes.
2. Preserve exact-pin model verbosity through `0.149.1` and every other
   feature-specific bound not independently proved at `0.151.0`.
3. Keep unpublished gaps out of prepare-based catalogue rows as established by
   PR 130's reviewed repair.
4. Update compatibility fixtures, route and feature matrices, Codex guide,
   changelog, standing lane, and claim log.
5. Run current focused, affected-package, route, docs, Northstar, API, and
   diff gates.
6. Review the exact restacked head and merge under standing operator authority
   only when mergeable and green.

## Out Of Scope

- widening a feature-specific exact set without its own evidence
- another family, provider contact, install, live probe, watcher, skill,
  feature-façade, papercut, release, or broad workspace work

## Acceptance Criteria

- production ranges admit only the proved `0.151.0` segment
- exact feature pins and unpublished gaps remain truthful
- current-main validation passes and the PR is mergeable
- the merged exact head is recorded in standing-lane and log surfaces

## Validation

- `cargo fmt -p swallowtail-adapter-codex -- --check`
- `effigy validate:focused swallowtail-adapter-codex`
- `effigy package:verify-affected swallowtail-adapter-codex`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

No. After merge, run a fresh all-route checkpoint before selecting one next
family.

## Result

The Codex exec and app-server axes qualify official `0.151.0` and published
intermediates `0.150.0` and `0.150.1`. Unpublished gaps remain incompatible.
Model verbosity and every other feature-specific exact set remain bounded
through `0.149.1`. Current-main focused, affected-package, semantic API, route,
docs, Northstar, format, and diff gates pass without provider work.
PR 130 merged exact head `63324b4f` through `3360d497` with five green checks.
