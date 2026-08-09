# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

### [ ] npm shebang harness probes fail when PATH is stripped — 2026-08-09
- Friction: Approving a Node shebang launcher (`#!/usr/bin/env node`) as a
  native executable fails once the local process host clears ambient `PATH`.
- Impact: Installed discovery returns `Failed` for otherwise exact payloads
  (Command Code `command-code` bin → `dist/index.mjs`).
- Possible fix: Default npm-harness live hosts and guides to
  `approve_installed_executable_launch` with exact `node` plus the resolved
  script prefix, matching Pi / Oh My Pi.
- Surface: `swallowtail-host-local` installed launches; npm adapter live probes.

### [ ] Release prepare omits coordinated workspace dependency versions — 2026-08-08
- Friction: Effigy updates `workspace.package.version` before gates but leaves
  versioned path entries under `workspace.dependencies` at the previous release.
- Impact: the first Cargo-backed gate cannot resolve the newly versioned local
  packages, so an otherwise valid coordinated release preparation fails.
- Possible fix: teach release preparation to update matching internal workspace
  dependency requirements and synchronize only workspace package lock entries.
- Surface: Effigy Cargo-workspace release preparation.

### [ ] Bootstrap papercuts before an exact-SHA release lane — 2026-08-06
- Friction: Northstar first required this file after the release candidate had already passed exact-commit CI.
- Impact: Adding the repository hygiene file during tag closeout would invalidate the clean-tree release check or move the tag beyond the green SHA.
- Possible fix: Install `PAPERCUTS.md` when adopting or upgrading Northstar, before release-candidate preparation.
- Surface: Northstar adoption and tagged-release preparation.
