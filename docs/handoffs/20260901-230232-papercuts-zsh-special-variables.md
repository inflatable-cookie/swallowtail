---
title: zsh special-variable papercut worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260901-230232-papercuts-zsh-special-variables.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts, shell]
---

## Objective

Resolve the open papercut “zsh special variables break ordinary shell
snippets.” Find the smallest repo-owned seam that prevents agent-authored
read-only snippets from assigning to zsh special parameters such as `path` and
`status`, or record an evidence-backed ownership stop if none exists.

## State And Scope

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Required ancestor:** `33cde9ed8a1fb27e36c0cfaa0cb0354f5b42f45b`
- **Worker branch:** `worker/papercuts-zsh-special-variables`
- **Worker worktree:** Paseo-managed from pushed `origin/main`, carrying the
  capitalized `Papercuts` workspace label.
- **Worker class:** mechanical. This is bounded shell-friction diagnosis and a
  small instruction/lint/helper repair, stale close, or ownership stop.
- **Authority:** `AGENTS.md`, the exact `PAPERCUTS.md` entry, current repo shell
  scripts/instructions/linters, Effigy routing, and this handoff.
- **In scope:** inventory of repo-owned zsh snippets and agent-facing shell
  instructions; throwaway reproduction; smallest repo-owned preventive rule,
  checker, or helper when it has a real execution/adoption seam;
  `PAPERCUTS.md` disposition.
- **Out of scope:** `.cursor/skills/version-currentness/**`; provider contact;
  production Rust; global installed agent skills; Effigy or Paseo source;
  roadmap, research/log/index surfaces; other papercuts; broad shell-style
  cleanup.
- **Parallel partition:** Claude Code currentness owns Claude-agent and
  currentness research/log/roadmap surfaces. This lane owns only its bounded
  repo instruction/checker/helper surface and `PAPERCUTS.md`.
- **Serial edge:** no later papercut starts before this lane merges or stops.

## Acceptance And Review Oracle

Reproduce both named zsh failures in a throwaway shell: assignment to `path`
must demonstrate its effect on command lookup, and assignment to read-only
`status` must demonstrate the direct error. Check whether the default shell and
repo-owned execution paths actually use zsh; do not generalize a zsh-only rule
onto bash-only scripts.

Inventory existing repo-owned shell guidance and checkers before adding
anything. Prefer one concise, load-bearing instruction at the narrowest
agent-authoring authority surface when the failure is generated text rather
than an executed repo script. Add a checker only if a current, deterministic
repo corpus exists for it to check; do not build a speculative generated-snippet
lint pipeline. Do not merely rename variables in unrelated historical examples.

Falsify a repair with a small negative specimen using `path` or `status`, and a
positive specimen using task-specific names. If no repo-owned seam can prevent
agent-authored ephemeral shell text, leave the checkbox open and name the owner.
Stop if the honest repair belongs in global installed skills, Northstar source,
Effigy/Paseo, or requires changing the active currentness skill.

## Validation And Completion

Confirm a clean non-`main` worktree, `HEAD == origin/main`, required ancestor,
and tracked handoff before edits. Read the Effigy skill. Run validation matched
to the changed surface: exact checker/helper tests if added, relevant docs or
Northstar selectors, papercuts parsing, and `git diff --check`.

Commit, push, and open one PR. Report exact head/base, reproduction, ownership,
repair/disposition, falsification, changed paths, validation, and next open
papercut. Do not add a log while currentness owns logs/index. Do not merge.

