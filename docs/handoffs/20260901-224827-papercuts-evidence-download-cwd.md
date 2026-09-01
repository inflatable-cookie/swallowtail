---
title: Evidence-download cwd papercut worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260901-224827-papercuts-evidence-download-cwd.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts, tooling]
---

## Objective

Resolve the open papercut “evidence-download cwd steals later repo commands.”
Determine whether current repository authority already closes it; otherwise
make the smallest repo-owned repair that prevents or clearly fails the cwd
leak. Close only with falsifiable evidence.

## State And Scope

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Required ancestor:** `e2e7ba0360bc3a78c6a1790340a3f3ddf47feeda`
- **Worker branch:** `worker/papercuts-evidence-download-cwd`
- **Worker worktree:** Paseo-managed from pushed `origin/main`, carrying the
  capitalized `Papercuts` workspace label.
- **Worker class:** mechanical. This is a bounded workflow-friction diagnosis
  and small repair or evidence-backed stale close/ownership stop.
- **Authority:** `AGENTS.md`, the exact `PAPERCUTS.md` entry, current repo-owned
  evidence helpers/instructions, Effigy routing, and this handoff.
- **In scope:** read-only inventory of repo-owned evidence download commands,
  scripts, handoffs, and instruction surfaces; throwaway reproduction outside
  user worktrees; the smallest repo-owned script/instruction correction if an
  active path still leaks cwd; `PAPERCUTS.md` disposition.
- **Out of scope:** `.cursor/skills/version-currentness/**` and its `.agents`
  symlink while the Claude Code currentness worker is active; provider contact,
  prompts, login, installs, host mutation, Cargo production behavior, roadmap,
  research/log/index surfaces, other papercuts, and unrelated shell cleanup.
- **Parallel partition:** the Claude Code worker owns Claude-agent currentness
  code, fixtures, research/log/roadmap, and the version-currentness skill as
  read authority. This lane owns only its bounded tooling/instruction repair and
  `PAPERCUTS.md`.
- **Serial edge:** no later papercut starts before this lane merges or stops.

## Acceptance And Review Oracle

Inventory every current repo-owned path that deliberately changes into a
temporary evidence directory and later runs `git`, `cargo`, or `effigy`.
Distinguish a persistent interactive-shell cwd leak from a subshell or command
with an explicit `workdir`. `AGENTS.md` already requires returning to the repo
root for currentness work; test whether that and current helpers make the entry
stale rather than assuming so.

Use only throwaway directories/repositories for reproduction and delete them
afterward. Falsify the original failure by showing a leaking shape makes the
next repo command fail, then show the current or repaired repo-owned path keeps
the caller cwd stable or explicitly restores/rebinds it. A prose reminder alone
does not close an active script defect. Conversely, do not add a wrapper or
lint when no current repo-owned execution seam exists.

Allowed dispositions:

- **closed/stale:** current authority or helpers already prevent the failure,
  with the exact load-bearing evidence named;
- **closed/repaired:** smallest active repo-owned seam fixed and tested;
- **open/ownership stop:** the only remaining seam is agent/terminal behavior
  outside Swallowtail, with no fail-closed repo interception.

Stop if the honest repair requires editing the active version-currentness skill,
Effigy itself, Paseo/launcher behavior, global agent instructions, or a broader
shell-policy decision. Leave the checkbox open in that case.

## Validation And Completion

Confirm a clean non-`main` worktree, `HEAD == origin/main`, required ancestor,
and tracked handoff before edits. Read the Effigy skill. Run validation matched
to changed files: the exact helper/script test if repaired, relevant docs or
Northstar checks, god-files scan if code size changes, and `git diff --check`.

Commit, push, and open one PR. Report exact head/base, reproduction, ownership,
disposition, changed paths, validation, and the next open papercut. Do not add a
log while the currentness worker owns logs/index surfaces. Do not merge.

