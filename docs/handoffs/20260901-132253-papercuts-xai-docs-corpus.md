---
title: xAI docs corpus papercut worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: research
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260901-132253-papercuts-xai-docs-corpus.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts, research]
---

## Objective

Reconcile the open xAI Next.js/Markdown corpus papercut against Research 187
and the already-correct Research 227. Preserve historical truth, remove any
implication that SPA HTML identifies converted corpus text, and close the entry
if Research 227 already supplies the complete repair.

## State And Scope

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Required ancestor:** `228422de0bfb75ac25e2cf7506815392c3f01c31`
- **Worker branch:** `worker/papercuts-xai-docs-corpus`
- **Worker worktree:** Paseo-managed from pushed `origin/main`, titled with the
  capitalized `Papercuts` label.
- **Worker class:** mechanical evidence reconciliation.
- **Authority:** the exact PAPERCUTS entry, Research 187, Research 227, current
  official xAI `.md` exports/OpenAPI, and this handoff.
- **In scope:** verify Research 187 hashes HTML shells; verify Research 227
  hashes `.md`/OpenAPI as binding corpus; add the smallest supersession note to
  Research 187 if needed; close the one entry.
- **Out of scope:** new xAI capability/compatibility research, claim, fixture,
  matrix, code, other SPA papercuts, roadmap edits, a new research record, or a
  closeout log.
- **Write partition:** only `PAPERCUTS.md`, Research 187, and Research 227 when
  exact correction is needed. Do not edit any index or log.
- **Serial edge:** no later papercut starts before this lane merges or stops.

## Acceptance And Completion

Record exact URL, retrieval date, body kind, byte count, and SHA-256 for the
binding `.md`/OpenAPI corpus; HTML is corroboration only. Do not replace
historical Research 187 hashes with current bodies. Prefer a supersession note
when its historical table is otherwise true.

Close only if every binding source is identified by digestable corpus in
Research 227 and Research 187 cannot be misread as current binding corpus. Stop
if exports disappeared, digests cannot reconcile, or a capability conclusion
would change.

Run `effigy qa:docs:links`, `effigy qa:docs:index:research`,
`effigy qa:northstar`, and `git diff --check`. No provider call, credential,
prompt, live operation, or implementation command.

Confirm a clean non-`main` worktree, `HEAD == origin/main`, and this handoff
from `HEAD`. Commit the bounded correction, push, and open one PR. Report exact
head/base, source/digest reconciliation, changed files, disposition,
validation, and PR URL. Do not merge.
