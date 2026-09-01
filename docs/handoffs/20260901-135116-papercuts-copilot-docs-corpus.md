---
title: Copilot CLI docs corpus papercut worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: research
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260901-135116-papercuts-copilot-docs-corpus.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts, research]
---

## Objective

Reconcile the open GitHub Copilot CLI Next.js/Markdown corpus papercut against
Research 188 and the already-correct Research 218. Preserve historical truth,
remove any implication that SPA HTML identifies converted corpus text, and
close the entry if Research 218 already supplies the complete repair.

## State And Scope

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Required ancestor:** `99809930c83dc18949da316ef302060fc94be0c4`
- **Worker branch:** `worker/papercuts-copilot-docs-corpus`
- **Worker worktree:** Paseo-managed from pushed `origin/main`, titled with the
  capitalized `Papercuts` label.
- **Worker class:** mechanical evidence reconciliation.
- **Authority:** the exact PAPERCUTS entry, Research 188, Research 218,
  current official GitHub Copilot CLI `.md` exports, and this handoff.
- **In scope:** verify Research 188 hashes HTML shells; verify Research 218
  hashes `.md` exports as binding docs corpus; add the smallest supersession
  note to Research 188 if needed; close the one entry.
- **Out of scope:** new Copilot capability or compatibility research, package
  artifacts, claims, fixtures, matrices, code, other SPA papercuts, roadmap
  edits, a new research record, or a closeout log.
- **Write partition:** only `PAPERCUTS.md`, Research 188, and Research 218 when
  an exact correction is needed. Do not edit any index or log; the parallel
  Codex currentness lane owns those shared surfaces.
- **Serial edge:** no later papercut starts before this lane merges or stops.

## Acceptance And Completion

Re-fetch the three official `.md` exports named by Research 218. Record exact
URL, retrieval date, body kind, byte count, and SHA-256 in the worker report;
do not rewrite Research 218's historical 2026-08-26 digests merely because
moving documentation drifted after the freeze. HTML is corroboration only.

Do not replace historical Research 188 hashes with current bodies. Prefer a
supersession note when its table is otherwise true. Close only if every
binding docs source is identified by digestable corpus in Research 218 and
Research 188 cannot be misread as current binding corpus. Stop if exports
disappeared, digests cannot reconcile, or a capability conclusion would
change.

Run `effigy qa:docs:links`, `effigy qa:docs:index:research`,
`effigy qa:northstar`, and `git diff --check`. No provider call, credential,
prompt, live operation, package download, or implementation command.

Confirm a clean non-`main` worktree, `HEAD == origin/main`, and this handoff
from `HEAD`. Commit the bounded correction, push, and open one PR. Report exact
head/base, source/digest reconciliation, changed files, disposition,
validation, and PR URL. Do not merge.
