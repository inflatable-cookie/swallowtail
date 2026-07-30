# Codex 0.146 Range Extension

Date: 2026-07-30
Status: completed

## Changed

- moved both Codex guaranteed upper bounds to exact `0.146.0`
- added exact package, exec, app-server, model-list, and lifecycle evidence
- extended continuity, activity, discovery, and prepared fixtures
- retained `0.147.0` as a synthetic later-stable unverified point
- refreshed route, feature-matrix, release, and consumer-handoff truth

## Result

`0.146.0` uses the existing current exec, workspace-root app-server, and
strict descendant hard-delete revisions. Its command-action metadata and
deferred search query are additive activity milestones. No access, sandbox,
workspace, lifecycle, disclosure, cleanup, or public operation changed.

The old baselines, `0.82.0..=0.83.0` and `0.108.0..=0.109.0` gaps, and
prerelease rejection remain intact. Later stable versions remain permitted as
visible unverified newer rather than hard-denied.

## Validation

- 128 focused Codex adapter tests passed
- warnings-denied all-target Codex clippy passed
- provider route, lifecycle, feature, and activity matrix checks passed
- docs QA and `git diff --check` passed

## Next

Execute card 148's OpenCode `1.18.10` range extension.
