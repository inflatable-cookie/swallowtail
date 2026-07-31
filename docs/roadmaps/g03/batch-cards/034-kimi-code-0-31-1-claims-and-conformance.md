# 034 Kimi Code 0.31.1 Claims And Conformance

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../013-kimi-code-0-31-1-range-maintenance.md`
Depends on: card 033

## Goal

Extend all maintained Kimi Code route claims through `0.31.1` using the exact
corpus and one new private local-server behavior milestone.

## Scope

1. Extend ACP and headless ceilings without changing their behavior revisions.
2. Add exact local-server `0.31.1` with refresh-stable behavior.
3. Bind discovery, preflight, run, activity, lifecycle, and corroborated server
   evidence to the planned exact version.
4. Cover boundaries, interiors, prereleases, mismatches, and later stable
   unverified versions.

## Acceptance Criteria

- [x] all exact maintained points map to their recorded behavior revisions
- [x] exact `0.31.1` is qualified on ACP, headless, and local-server routes
- [x] later stable versions remain permitted but unverified
- [x] unsupported and malformed versions fail before provider work
- [x] optional interruption detail cannot widen portable activity authority
- [x] focused and extracted-package validation pass
- [x] card 035 becomes sole ready and next

## Validation

- `effigy validate:focused swallowtail-adapter-kimi`
- `effigy package:verify-affected swallowtail-adapter-kimi`
- `git diff --check`
- no broad workspace suite

## Auto-Continuation

Yes. Continue to card 035 after focused and extracted-package proof.

## Evidence

- ACP claim `window-4` retains the declared-effort behavior through `0.31.1`
- headless claim `window-3` retains stream-JSON v1 through `0.31.1`
- local-server claim `window-4` adds exact refresh-stable `0.31.1`
- optional interruption detail decodes to the existing terminal reason only
- 89 focused tests passed in three seconds
- the independently extracted 212-file package compiled in five seconds
