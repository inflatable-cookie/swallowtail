# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

### [ ] Bootstrap papercuts before an exact-SHA release lane — 2026-08-06
- Friction: Northstar first required this file after the release candidate had already passed exact-commit CI.
- Impact: Adding the repository hygiene file during tag closeout would invalidate the clean-tree release check or move the tag beyond the green SHA.
- Possible fix: Install `PAPERCUTS.md` when adopting or upgrading Northstar, before release-candidate preparation.
- Surface: Northstar adoption and tagged-release preparation.
