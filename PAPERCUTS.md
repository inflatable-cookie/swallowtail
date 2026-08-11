# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

### [ ] Roadmap docs policy resolves child links and next pointers incorrectly — 2026-08-11
- Friction: `effigy qa:docs` reports existing `g01/README.md`, `g02/README.md`,
  `g03/README.md`, and `backlog/README.md` links as missing even though they
  exist under `docs/roadmaps/`.
- Impact: the broad docs selector cannot certify an otherwise indexable docs
  change; its next-action gate also requires `## Next Task` in every generation
  and backlog index, contrary to this repo's single front-door pointer rule.
- Possible fix: resolve policy-index child links relative to the indexed file's
  directory and scope the roadmaps next-action policy to
  `docs/roadmaps/README.md`.
- Surface: Effigy roadmap index and next-action docs policies.

## Closed

### [x] Release prepare omits coordinated workspace dependency versions — 2026-08-08
- Friction: Effigy updated `workspace.package.version` before gates but left
  versioned path entries under `workspace.dependencies` at the previous release.
- Impact: the first Cargo-backed gate could not resolve the newly versioned local
  packages, so an otherwise valid coordinated release preparation failed.
- Fix: Effigy v0.11 release planning now updates exact-version path dependencies
  for workspace members inheriting `workspace.package.version`. The focused
  fixture covers plan previews, guarded lock sync, exclusions, and rollback.
- Surface: Effigy Cargo-workspace release preparation.

### [x] Bootstrap papercuts before an exact-SHA release lane — 2026-08-06
- Friction: Northstar first required this file after the release candidate had already passed exact-commit CI.
- Impact: Adding the repository hygiene file during tag closeout would invalidate the clean-tree release check or move the tag beyond the green SHA.
- Fix: Northstar adopt/upgrade and release-posture guidance now seed
  `PAPERCUTS.md` before exact-SHA / clean-tree release prep (skill repo-contract,
  normalize-docs, bundle-docs/papercuts.md, template-bundle).
- Surface: Northstar adoption and tagged-release preparation.
