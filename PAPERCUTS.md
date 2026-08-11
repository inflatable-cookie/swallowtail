# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

### [ ] Release prepare omits coordinated workspace dependency versions — 2026-08-08
- Friction: Effigy updates `workspace.package.version` before gates but leaves
  versioned path entries under `workspace.dependencies` at the previous release.
- Impact: the first Cargo-backed gate cannot resolve the newly versioned local
  packages, so an otherwise valid coordinated release preparation fails.
- Possible fix: teach release preparation to update matching internal workspace
  dependency requirements and synchronize only workspace package lock entries.
- Surface: Effigy Cargo-workspace release preparation.

## Closed

### [x] Bootstrap papercuts before an exact-SHA release lane — 2026-08-06
- Friction: Northstar first required this file after the release candidate had already passed exact-commit CI.
- Impact: Adding the repository hygiene file during tag closeout would invalidate the clean-tree release check or move the tag beyond the green SHA.
- Fix: Northstar adopt/upgrade and release-posture guidance now seed
  `PAPERCUTS.md` before exact-SHA / clean-tree release prep (skill repo-contract,
  normalize-docs, bundle-docs/papercuts.md, template-bundle).
- Surface: Northstar adoption and tagged-release preparation.
